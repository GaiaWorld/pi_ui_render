//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use std::{io::Result};

use nalgebra::Orthographic3;
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_atom::Atom;
use pi_ecs::{
    entity::Id,
    monitor::Event,
    prelude::{res::WriteRes, Join, OrDefault, ParamSet, Query, Res, ResMut, Write},
    storage::Offset, query::ChangeTrackers,
};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Layer;
use pi_null::Null;
use pi_postprocess::{postprocess_geometry::PostProcessGeometryManager, postprocess_pipeline::PostProcessMaterialMgr};
use pi_render::{
    graph::graph::RenderGraph,
    rhi::{
        asset::RenderRes,
        bind_group::BindGroup,
        buffer::Buffer,
        device::RenderDevice,
        dyn_uniform_buffer::{Bind, Group},
        RenderQueue, texture::PiRenderDefault,
    },
};
use pi_share::Share;
use pi_spatialtree::quad_helper::intersects;
use smallvec::smallvec;
use wgpu::{CompareFunction, DepthBiasState, IndexFormat, StencilState, TextureFormat};

use crate::{
    components::{
        calc::{
            ContentBox, DrawInfo, DrawList, InPassId, NodeId, OverflowAabb, Pass2DId, Quad, TransformWillChangeMatrix, Visibility, WorldMatrix,
            ZRange
        },
        draw_obj::{DrawGroup, DrawKey, DrawObject, DrawState, DynDrawGroup, ClearColorBindGroup},
        pass_2d::{
            Camera, DirtyRect, DirtyRectState, Draw2DList, DrawIndex, GraphId, LastDirtyRect, ParentPassId, Pass2D, PostProcessList, ViewMatrix,
        },
        user::{Aabb2, Matrix4, Node, Point2, TransformWillChange, Viewport, ClearColor, RenderDirty, Canvas},
    },
    resource::draw_obj::{ClearDrawObj, ColorStaticIndex, DynBindGroupIndex, DynBindGroups, DynFboClearColorBindGroup, DynUniformBuffer, PipelineState,
		ShareLayout, StateMap, StaticIndex, UnitQuadBuffer,
    },
    shaders::color::{
        CameraMatrixBind, CameraMatrixGroup, ColorMaterialBind, ColorMaterialGroup, ColorUniform, DepthUniform, ProjectUniform, ViewUniform,
        WorldUniform,
    },
    utils::tools::{calc_aabb, calc_bound_box, intersect},
};

use super::pass_graph_node::Pass2DNode;

pub struct CalcRender;

/// 需要为清屏颜色创建DrawObj，依赖CalcBackground的初始化，请在初始化本功能前先初始化CalcBackground
#[setup]
impl CalcRender {
    // 创建清屏的drawobj
    #[init]
    pub fn init(
        unit_quad_buffer: Res<UnitQuadBuffer>,
        static_index: Res<ColorStaticIndex>,
        mut state_map: ResMut<StateMap>,

        mut dyn_fbo_clear_color_bind_group: WriteRes<DynFboClearColorBindGroup>,
        mut clear_draw_obj: WriteRes<ClearDrawObj>,

        color_material_bind_group: Res<'static, DynBindGroupIndex<ColorMaterialGroup>>,
        camera_bind_group: Res<'static, DynBindGroupIndex<CameraMatrixGroup>>,

        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
    ) {
        let pipeline_state = create_clear_pipeline_state();
        let pipeline_state = state_map.insert(pipeline_state);
        // 清屏使用的渲染状态不同
        let static_index = StaticIndex {
            shader: static_index.shader,
            pipeline_state: pipeline_state,
            vertex_buffer_index: static_index.vertex_buffer_index,
            name: static_index.name,
        };

        // 设置清屏颜色的vb、ib
        let mut draw_state = DrawState::default();
        draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
        draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));

        // 暂时在pipeline system中创建pipeline， 考虑ecs新增只运行一次的system，将该逻辑放入这类system中（创建pipeline为异步操作， 当前方法为同步方法，而pipeline system每帧都会运行， 此pipeline最适合放入到一个只运行一次的system中）
        // // 设置清屏颜色的pipeline
        // let (vs_defines, fs_defines) = (VSDefines::default(), FSDefines::default());
        // let pipeline = CalcPipeline::calc_pipeline(
        // 	&vs_defines,
        // 	&fs_defines,
        // 	&static_index,

        // 	&shader_statics,
        // 	&device,
        // 	&vertex_buffer_layout_map,
        // 	&state_map,
        // 	&shader_catch,

        // 	&mut pipeline_map,
        // 	&mut shader_map,
        // 	&share_layout,
        // );
        // // 设置pipeline
        // draw_state.pipeline = Some(pipeline);

        // 设置清屏颜色的世界矩阵

        // 设置清屏颜色的世界矩阵、投影矩阵、视图矩阵
        // 视图矩阵和投影矩阵都设置为单位阵
        let view_project = WorldMatrix::default().0;
        let camera_dyn_offset = dyn_uniform_buffer.alloc_binding::<CameraMatrixBind>();
        dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ProjectUniform(view_project.as_slice()));
        dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ViewUniform(view_project.as_slice()));

        draw_state.bind_groups.insert_group(
            CameraMatrixGroup::id(),
            DrawGroup::Dyn(DynDrawGroup::new(**camera_bind_group, smallvec![camera_dyn_offset])),
        );

        let color_dyn_offset = dyn_uniform_buffer.alloc_binding::<ColorMaterialBind>();
        // 世界矩阵
        let world = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        dyn_uniform_buffer.set_uniform(&color_dyn_offset, &WorldUniform(world.as_slice()));
        // 深度设置为-1(最远)
        dyn_uniform_buffer.set_uniform(&color_dyn_offset, &DepthUniform(&[0.0]));
        dyn_uniform_buffer.set_uniform(&color_dyn_offset, &ColorUniform(&[0.0, 0.0, 0.0, 0.0]));

        let group = DrawGroup::Dyn(DynDrawGroup::new(**color_material_bind_group, smallvec![color_dyn_offset]));
        dyn_fbo_clear_color_bind_group.write(DynFboClearColorBindGroup(group));

        clear_draw_obj.write(ClearDrawObj(draw_state, static_index));
    }


    #[system]
    pub fn calc_render<'a>(
        parent_pass_id: Query<Pass2D, Option<&ParentPassId>>,
        mut postprocess_lists: Query<Pass2D, Option<&mut PostProcessList>>,
        mut query_draw2d_list: ParamSet<(Query<Pass2D, &'static mut Draw2DList>, Query<Pass2D, (&'static Draw2DList, Id<Pass2D>, &'static ParentPassId)>)>,
        mut query_pass: ParamSet<(
            Query<
                Pass2D,
                (
                    Write<Camera>,
                    Write<ViewMatrix>,
                    Write<LastDirtyRect>,
                    Option<&'static OverflowAabb>,
                    Join<
                        NodeId,
                        Node,
                        (
                            &'static ContentBox,
                            &'static Quad,
                            Option<&'static TransformWillChangeMatrix>,
                            Option<&'static TransformWillChange>,
							&'static Layer<Node>,
                        ),
                    >,
                    Option<&'static mut PostProcessList>,
                ),
            >,
            Query<
                Node,
                (
                    &'static InPassId,
                    Option<&'static Pass2DId>,
                    Option<&'static DrawList>,
                    &'static Quad,
                    &'static ZRange,
                    OrDefault<Visibility>,
                    Join<InPassId, Pass2D, &'static LastDirtyRect>,
                ),
            >,
            Query<Pass2D, (&'static Camera, Option<&'static OverflowAabb>)>,
        )>,
		query_node: Query<Node, (&DirtyRect, OrDefault<RenderDirty>, &Viewport)>,
        mut draw_state: Query<DrawObject, &mut DrawState>,
        draw_info: Query<DrawObject, OrDefault<DrawInfo>>,
        // mut z_query1&: Query<Pass2D, Join<NodeId, Node, &mut &ZRange>>,
        share_layout: Res<'a, ShareLayout>,
        device: Res<'a, RenderDevice>,
        queue: Res<'a, RenderQueue>,
        // global_dirty_rect: Res<'a, DirtyRect>,
		// render_dirty_mark: Res<'a, RenderDirty>,

        buffer_assets: Res<'a, Share<AssetMgr<RenderRes<Buffer>>>>,
        bind_group_assets: Res<'a, Share<AssetMgr<RenderRes<BindGroup>>>>,
        mut depth_cache: ResMut<'a, DepthCache>,
        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
        camera_bind_group: Res<DynBindGroupIndex<CameraMatrixGroup>>,
        mut dyn_bind_groups: ResMut<'static, DynBindGroups>,

        mut geometrys: ResMut<'static, PostProcessGeometryManager>,
        mut postprocess_pipelines: ResMut<'static, PostProcessMaterialMgr>,
		mut render_graph: ResMut<RenderGraph>,
		canvas_query: Query<Node, (&'static Canvas, ChangeTrackers<Canvas>, Join<InPassId, Pass2D, &'static GraphId>)>,
    ) -> Result<()> {
		
		// canvas的图节点id由外部系统设置，设置时，InPassId可能并未创建，因此需要延迟到帧推时处理
		for (canvas, canvas_ticker, graph_id) in canvas_query.iter() {
			if canvas_ticker.is_changed() {
				if let Err(e) = render_graph.add_depend(canvas.0, graph_id.0) {
					log::error!("{:?}", e);
				}
			}
		}

        for (mut camera, _view_matrix, mut last_dirty, overflow_aabb, (context_box, quad, willchange_matrix, will_change, layer), postprocess_list) in
            query_pass.p0_mut().iter_mut()
        {

			let (global_dirty_rect, render_dirty_mark, viewport) = match query_node.get(layer.root()) {
				Some(r) => r, 
				None => continue
			};
		
			// 不脏，不需要组织渲染图， 也不需要渲染脏
			if global_dirty_rect.state == DirtyRectState::UnInit && !render_dirty_mark.0{
				continue;
			}
			// 如果render_dirty_mark.0, 表示全屏zz
			let mut dirty_rect = global_dirty_rect.value.clone();
			if render_dirty_mark.0 {
				dirty_rect = viewport.0;
			}

            // 存在脏区域，与现有脏区域相交，得到最终脏区域
            let mut c;
			// log::warn!("pass_id=========pass_id: {:?}, context_box: {:?}, overflow_aabb: {:?}, overflow_aabb {:?}, viewpor {:?}", pass_id, context_box, overflow_aabb, overflow_aabb, *viewport);

            let aabb = if let Some(_overflow) = overflow_aabb {
                // 存在overflow
                &**quad
            } else {
                // 否则， 该上下文最大的渲染区域不超过context_box
                &context_box.0
            };

            let context_box = if let Some(r) = willchange_matrix {
                c = calc_aabb(aabb, &r.will_change);
                if let Some(overflow) = overflow_aabb {
                    if let (Some(overflow), None) = (&overflow.aabb, &overflow.matrix) {
                        // 不存在旋转时，该上下文最大的渲染区域不超过quad
                        c = match intersect(&c, overflow) {
                            Some(r) => r,
                            None => Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
                        };
                    }
                }
                &c
            } else {
                &aabb
            };

            let context_box = match intersect(&context_box, &viewport.0) {
                Some(r) => r,
                None => Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            };

			// log::warn!("pass_id1========={:?}, {:?}, {:?}, {:?}", pass_id, context_box, dirty_rect, intersect(&context_box, &dirty_rect));

            let aabb = if let Some(aabb) = intersect(&dirty_rect, &context_box) {
                // 如果存在transformwillchange，则需要算上脏区域
                // no_will_change用于包围盒剔除渲染对象（渲染对象使用quad来剔除，quad是没有willchange_matrix的参与的）
                let no_will_change = if let Some(r) = willchange_matrix {
                    calc_aabb(&aabb, &r.will_change_invert)
                } else {
                    aabb
                };

                last_dirty.write(LastDirtyRect {
                    last: aabb.clone(),
                    no_will_change,
                });

                if let Some(overflow) = overflow_aabb {
                    // 存在裁剪区，并且旋转，
                    if let (Some(overflow), Some(r)) = (&overflow.aabb, &overflow.matrix) {
                        let r = calc_bound_box(&aabb, &r.rotate_matrix_invert);
                        intersect(&overflow, &r).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
                    } else {
                        aabb
                    }
                } else {
                    aabb
                }
            } else {
                continue;
            };

            let project = create_project(aabb.mins.x, aabb.maxs.x, aabb.mins.y, aabb.maxs.y);
            let view = WorldMatrix::default().0;

            let view = if let Some(overflow) = overflow_aabb {
                // 存在裁剪区，并且未旋转，则直接与视口相交
                if let (Some(_aabb), Some(matrix)) = (&overflow.aabb, &overflow.matrix) {
                    &matrix.rotate_matrix_invert
                } else {
                    &view
                }
            } else if let (Some(willchange_matrix), Some(_)) = (willchange_matrix, will_change) {
                &willchange_matrix.will_change
            } else {
                &view
            };

            let camera_dyn_offset = dyn_uniform_buffer.alloc_binding::<CameraMatrixBind>();
            dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ProjectUniform(project.as_slice()));
            dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ViewUniform(view.as_slice()));

            let aabb = Aabb2::new(
                Point2::new(aabb.mins.x.floor(), aabb.mins.y.floor()),
                Point2::new(aabb.maxs.x.ceil(), aabb.maxs.y.ceil()),
            );

            let scale_x = (aabb.maxs.x - aabb.mins.x) / 2.0;
            let scale_y = (aabb.maxs.y - aabb.mins.y) / 2.0;
            // 后处理效果与gui坐标系使用不一致，所以缩放为-scale_y
            let world_matrix = Matrix4::new(
                scale_x,
                0.0,
                0.0,
                aabb.mins.x + scale_x,
                0.0,
                -scale_y,
                0.0,
                aabb.mins.y + scale_y,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            );
            camera.write(Camera {
                // view: match willchange_matrix {
                // 	Some(r) => r.0.clone(),
                // 	Non
                // },
                view: view.clone(),
                project,
                bind_group: Some(DrawGroup::Dyn(DynDrawGroup::new(**camera_bind_group, smallvec![camera_dyn_offset]))),
                view_port: aabb,
                world_matrix: world_matrix.clone(),
				is_active: true,
            });

            if let Some(mut postprocess) = postprocess_list {
                postprocess.calc(
                    16,
                    &device,
                    &mut postprocess_pipelines,
                    &mut geometrys,
                    &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::pi_render_default(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                operation: wgpu::BlendOperation::Add,
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            },
                            alpha: wgpu::BlendComponent {
                                operation: wgpu::BlendOperation::Add,
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    Some(wgpu::DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::GreaterEqual,
                        stencil: StencilState::default(),
                        bias: DepthBiasState::default(),
                    }),
                );
                postprocess.view_port = aabb;
                postprocess.matrix = WorldMatrix(world_matrix, false);
                if let Some(overflow) = overflow_aabb {
                    // 存在裁剪区，并且未旋转，则直接与视口相交
                    if let Some(matrix) = &overflow.matrix {
                        postprocess.matrix = WorldMatrix(&matrix.rotate_matrix * &postprocess.matrix.0, true);
                    }
                }
            }

            // if let (Some(willchange_matrix), Some(_)) = (willchange_matrix, will_change) {
            // 	let view_bind_group = create_camera_bind_group(
            // 		&willchange_matrix.will_change,
            // 		&share_layout.view,
            // 		&device,
            // 		&buffer_assets,
            // 		&bind_group_assets,
            // 	);
            // 	view_matrix.write(ViewMatrix {
            // 		bind_group: Some(view_bind_group),
            // 		// value: willchange_matrix.will_change.clone(),
            // 	});
            // }
        }

        let p0 = query_draw2d_list.p0_mut();
        // 组织渲染列表
        // 用脏区域，查询到脏区域内的渲染节点，对其进行遍历，放入对应的pass中（TODO，aabb查询四叉树）
        for (in_pass_id, pass_id, draw_list, quad, z_range, visibility, context_dirty) in query_pass.p1().iter() {
            // global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
            if let Some(draw_list) = draw_list {
				// log::warn!("draw_list==================pass_id: {:?}, {:?}", draw_list.0, pass_id);
                if **visibility && intersects(quad, &context_dirty.no_will_change) {
					let mut list  = p0.get_unchecked_mut(**in_pass_id);
                    for draw_id in draw_list.iter() {
                        list.all_list.push((
                            DrawIndex::DrawObj(*draw_id),
                            z_range.clone(),
                            *draw_info.get_unchecked(*draw_id),
                        ));
                    }
                }
            }

            if let Some(pass_id) = pass_id {
                if let Some(parent) = parent_pass_id.get_unchecked(pass_id.0) {
                    if let Some(mut p) = p0.get_mut(parent.0) {
                        p.all_list.push((DrawIndex::Pass2D(pass_id.0), z_range.clone(), DrawInfo::new(10, false)));
                    }
                }
            }
        }

        // 遍历所有的pass，设置不透明渲染列表和候命渲染列表
        for mut list in query_draw2d_list.p0_mut().iter_mut() {
            list.opaque.clear();
            list.transparent.clear();
            if list.all_list.len() == 0 {
                continue;
            }

            list.all_list.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
                if a_z_depth.start < b_z_depth.start {
                    std::cmp::Ordering::Less
                } else if a_z_depth.start > b_z_depth.start {
                    std::cmp::Ordering::Greater
                } else {
                    if a_sort < b_sort {
                        std::cmp::Ordering::Less
                    } else if a_sort > b_sort {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                // 用渲染管线排序，TODO
                // draw_state.get(a)
            });

            for i in 0..list.all_list.len() {
                let (entity, _, draw_info) = list.all_list[i];
                // 暂时放入不透明列表，TODO
                if draw_info.is_opacity() {
                    list.opaque.push(entity);
                } else {
                    list.transparent.push(entity);
                }
            }
        }

        let p1 = query_draw2d_list.p1();
        let camera_query = query_pass.p2();
        for (list, pass_id, parent_pass_id) in p1.iter() {
            // 不存在后处理，不主动分配depth（需要pass2d分配）
            // 如果post不为none，但长度大于0，表示根节点，也需要自己分配depth
            if let None = postprocess_lists.get_unchecked(pass_id) {
				if !parent_pass_id.is_null() {
					continue;
				}
            }

            alloc_depth(
                &device,
                p1,
                &mut postprocess_lists,
                camera_query,
                list,
                &share_layout,
                &mut draw_state,
                &buffer_assets,
                &bind_group_assets,
                &mut depth_cache,
                &mut 0,
                &mut dyn_uniform_buffer,
                &mut geometrys,
                &mut postprocess_pipelines,
            );
        }

        // 清理列表
        for mut list in query_draw2d_list.p0_mut().iter_mut() {
            list.all_list.clear();
        }

		update_uniform_buffer(&device, &queue, &mut dyn_uniform_buffer, &mut dyn_bind_groups);

        Ok(())
    }

    /// 创建渲染图节点
    /// 插入Draw2DList
    #[listen(entity=(Pass2D, Create))]
    pub fn create_graph_node(
        e: Event,
        mut query: Query<Pass2D, (Write<GraphId>, Write<Draw2DList>)>,
        mut rg: ResMut<RenderGraph>,
    ) {
		log::trace!("create_graph_node=================={:?}", e.id);
        let node = Pass2DNode::new(unsafe { Id::new(e.id.local()) });
        // rg.reset();
        let graph_id = match rg.add_node(format!("Pass2D {:?}", e.id.local().offset()), node) {
			Ok(r) => r,
			Err(e) => {
				log::error!("{:?}", e);
				return;
			}
		};
        let (mut graph_id_item, mut list_item) = query.get_unchecked_mut_by_entity(e.id);
        graph_id_item.write(GraphId(graph_id));
        list_item.write(Draw2DList::default());
    }

    // 移除渲染图节点
    #[listen(entity=(Pass2D, Delete))]
    pub fn delete_graph_node(e: Event, query: Query<Pass2D, &GraphId>, mut rg: ResMut<RenderGraph>) {
        // log::info!("delete_graph_node================={:?}", e.id);
        if let Some(graph_id) = query.get_by_entity(e.id) {
            rg.remove_node(**graph_id, format!("Pass2D {:?}", e.id.local().offset()));
        }
    }

    #[listen(component=(Pass2D, ParentPassId, (Create, Modify)))]
    pub fn depend_graph_node(
        e: Event,
        query: Query<Pass2D, (&ParentPassId, &GraphId)>,
        query_graph: Query<Pass2D, &GraphId>,
        mut rg: ResMut<RenderGraph>,
    ) {
        // log::info!("depend_graph_node================={:?}", e.id);
        let (parent_id, graph_id) = query.get_unchecked_by_entity(e.id);
        if parent_id.is_null() {
            if let Err(e) = rg.set_finish(**graph_id, true) {
                log::error!("{:?}", e);
            }
        } else {
            // rg.set_node_finish(graph_id, false);
            let parent_graph_id = query_graph.get_unchecked(**parent_id);
            // 建立父子依赖关系，使得子pass先渲染
            if let Err(e) = rg.add_depend(**graph_id, **parent_graph_id) {
                log::error!("{:?}", e);
            }
        }
    }

	// 清屏颜色修改后，重新创建bindgroup
    #[listen(component=(Node, ClearColor, (Modify, Create, Delete)))]
    pub fn clear_change(
        e: Event,
        query: Query<Node, (Option<&ClearColor>, Write<ClearColorBindGroup>)>,

        // mut color_bind_group: ResMut<ClearColorBindGroup>,
        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
        color_material_bind_group: Res<'static, DynBindGroupIndex<ColorMaterialGroup>>,
    ) {
		if let Some((color, mut color_bind_group)) = query.get_mut_by_entity(e.id) {
			if let Some(color) = color {
				let color_bind_group = match color_bind_group.get_mut() {
					Some(r) => r,
					None => {
						color_bind_group.insert_no_notify(ClearColorBindGroup(None));
						color_bind_group.get_mut().unwrap()
					}
				};
				let color_bind_group = match &mut color_bind_group.0 {
                    Some(r) => r,
                    None => {
                        let color_dyn_offset = dyn_uniform_buffer.alloc_binding::<ColorMaterialBind>();
						let world = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
						dyn_uniform_buffer.set_uniform(&color_dyn_offset, &WorldUniform(world.as_slice()));

                        color_bind_group.0 = Some(DrawGroup::Dyn(DynDrawGroup::new(
                            **color_material_bind_group,
                            smallvec![color_dyn_offset],
                        )));

                        color_bind_group.0.as_mut().unwrap()
                    }
                };
                let offset = color_bind_group.get_offset(ColorMaterialBind::index()).unwrap();
                dyn_uniform_buffer.set_uniform(offset, &ColorUniform(&[color.0.x, color.0.y, color.0.z, color.0.w]));
			} else {

			}
		}

        // // log::info!("create_graph_node================={:?}", e.id);
        // let node = Pass2DNode::new(unsafe { Id::new(e.id.local()) });
        // let graph_id = rg.add_node(format!("Pass2D {:?}", e.id.local().offset()), node);
        // let (mut graph_id_item, mut list_item) = query.get_unchecked_mut_by_entity(e.id);
        // graph_id_item.write(GraphId(graph_id));
        // list_item.write(Draw2DList::default());
    }
}

fn update_uniform_buffer(
	device: &RenderDevice,
	queue: &RenderQueue,
	dyn_uniform_buffer: &mut DynUniformBuffer, 
	dyn_bind_groups: &mut DynBindGroups
) {
	let r = dyn_uniform_buffer.write_buffer(&device, &queue);
	// println!("time================={:?}, {:?}", std::time::Instant::now() - time, dyn_uniform_buffer.capacity());
	// let time = std::time::Instant::now();
	if r {
		let buffer = dyn_uniform_buffer.buffer().unwrap();
		// 返回true表示buffer已修改，需要重新创建bindgroup
		for (group, layout, create_fn) in dyn_bind_groups.iter_mut() {
			*group = Some(create_fn(&device, layout, buffer));
		}
		// println!("create group================={:?}, {:?}", std::time::Instant::now() - time, dyn_bind_groups.len());
	}
}


pub fn create_project(left: f32, right: f32, top: f32, bottom: f32) -> Matrix4 {
    let ortho = Orthographic3::new(left, right, bottom, top, -1.0, 1.0);
    Matrix4::from(ortho)
}

fn alloc_depth<'a>(
    device: &'a RenderDevice,
    pass2d: &'a Query<Pass2D, (&Draw2DList, Id<Pass2D>, &ParentPassId)>,
    post_process_list: &mut Query<Pass2D, Option<&mut PostProcessList>>,
    camera_query: &'a Query<Pass2D, (&Camera, Option<&OverflowAabb>)>,
    list: &'a Draw2DList,
    share_layout: &'a ShareLayout,
    draw_state: &'a mut Query<DrawObject, &mut DrawState>,
    buffer_assets: &'a Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
    depth_cache: &'a mut Vec<Handle<RenderRes<BindGroup>>>,
    cur_depth: &'a mut usize,
    dyn_uniform_buffer: &'a mut DynUniformBuffer,
    geometrys: &mut PostProcessGeometryManager,
    postprocess_pipelines: &mut PostProcessMaterialMgr,
) {
    for index in list.all_list.iter() {
        match &index.0 {
            // 如果绘制索引是一个DrawObj，则设置该DrawObj的depth group
            DrawIndex::DrawObj(draw_key) => {
                alloc_depth_one(*draw_key, draw_state, cur_depth, dyn_uniform_buffer);
            }
            // 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
            DrawIndex::Pass2D(pass2d_id) => {
                let list = if let Some((list, pass_id, _parent_pass_id)) = pass2d.get(pass2d_id.clone()) {
                    let post = post_process_list.get_unchecked_mut(pass_id);
                    match post {
                        Some(mut r) => {
                            r.depth = *cur_depth as f32;
                            *cur_depth += 1;
                            continue;
                        }
                        None => list,
                    }
                } else {
                    continue;
                };
                alloc_depth(
                    device,
                    pass2d,
                    post_process_list,
                    camera_query,
                    list,
                    share_layout,
                    draw_state,
                    buffer_assets,
                    bind_group_assets,
                    depth_cache,
                    cur_depth,
                    dyn_uniform_buffer,
                    geometrys,
                    postprocess_pipelines,
                );
            }
        }
    }
}

fn alloc_depth_one<'a>(
    draw_key: DrawKey,
    draw_state: &'a mut Query<DrawObject, &mut DrawState>,
    cur_depth: &'a mut usize,
    dyn_uniform_buffer: &'a mut DynUniformBuffer,
) {
    let draw_state_item = match draw_state.get_mut(draw_key) {
        Some(r) => r,
        None => return,
    };
    let color_dyn_offset = draw_state_item
        .bind_groups
        .get_group(ColorMaterialGroup::id())
        .unwrap()
        .get_offset(ColorMaterialBind::index())
        .unwrap();
    dyn_uniform_buffer.set_uniform(color_dyn_offset, &DepthUniform(&[*cur_depth as f32]));

    *cur_depth += 1;
}

lazy_static! {
    pub static ref DEPTH: Atom = Atom::from("depth");
}

/// depth BindGroup缓存
#[derive(Deref, DerefMut, Default)]
pub struct DepthCache(Vec<Handle<RenderRes<BindGroup>>>);


pub fn create_clear_pipeline_state() -> PipelineState {
    PipelineState {
        targets: vec![Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::pi_render_default(),
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                },
                alpha: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                },
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })],
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    }
}
