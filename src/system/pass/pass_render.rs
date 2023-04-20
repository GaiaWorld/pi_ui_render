//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use bevy::{ecs::{
    prelude::Entity,
    system::{ParamSet, Query, Res, ResMut},
}, prelude::DetectChangesMut};
use nalgebra::Orthographic3;
use pi_assets::{mgr::AssetMgr};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::{
    prelude::{Layer, OrDefault},
    system_param::res::{OrInitRes, OrInitResMut},
};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_null::Null;
use pi_render::{
	rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer, device::RenderDevice, shader::{BindLayout}}, renderer::draw_obj::DrawBindGroup,
};
use pi_share::Share;
use pi_sparialtree::quad_helper::intersects;

use crate::{
    components::{
        calc::{
            DrawInfo, DrawList, EntityKey, InPassId, IsShow, View, Quad, RootDirtyRect, TransformWillChangeMatrix, WorldMatrix, ZRange,
        },
        draw_obj::DrawState,
        pass_2d::{Camera, DirtyRectState, Draw2DList, DrawIndex, LastDirtyRect, ParentPassId, PostProcessList, ViewMatrix},
        user::{Aabb2, Matrix4, Point2, RenderDirty, Viewport, Vector2},
    },
    resource::draw_obj::{CameraGroup, GroupAlloterCenter, ShareGroupAlloter, ShareLayout, DepthCache},
    shader::{
        camera::{ProjectUniform, ViewUniform},
        depth::DepthBind,
    },
    utils::tools::{calc_aabb, intersect}, system::utils::rotatequad_quad_intersection,
};

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera_depth_and_renderlist(
    mut query_draw2d_list: ParamSet<(
        Query<&'static mut Draw2DList>,
        Query<(&'static Draw2DList, Entity, &'static ParentPassId)>,
    )>,
    mut query_pass: ParamSet<(
        Query<(
            Entity,
            &mut Camera,
            &mut ViewMatrix,
            &mut LastDirtyRect,
            &mut PostProcessList,
            &View,
            &TransformWillChangeMatrix,
            &Layer,
        )>,
        (
            Query<(&InPassId, &DrawList, &Quad, &ZRange, &IsShow, Entity)>,
            Query<(Option<&ParentPassId>, &LastDirtyRect)>,
        ),
        (Query<(&Camera, &View)>, Query<&'static mut PostProcessList>),
    )>,
    mut query_root: ParamSet<(Query<(&RootDirtyRect, OrDefault<RenderDirty>, &Viewport)>, Query<&mut RenderDirty>)>,
    mut draw_state: Query<&'static mut DrawState>,
    draw_info: Query<&DrawInfo>,

    res: (
        Res<ShareLayout>,
        Res<PiRenderDevice>,
        Res<PiRenderQueue>,
        Res<ShareAssetMgr<RenderRes<Buffer>>>,
        Res<ShareAssetMgr<RenderRes<BindGroup>>>,
        Res<GroupAlloterCenter>,
    ),
    mut depth_cache: OrInitResMut<DepthCache>,
    camera_material_alloter: OrInitRes<ShareGroupAlloter<CameraGroup>>,

	mut post_resource: ResMut<PostprocessResource>,
    // mut geometrys: ResMut<PiPostProcessGeometryManager>,
    // mut postprocess_pipelines: ResMut<PiPostProcessMaterialMgr>,
) {
    let (share_layout, device, queue, buffer_assets, bind_group_assets, group_alloc_center) = res;
	let p0 = query_root.p0();
    for (
        entity,
        mut camera,
        _view_matrix,
        mut last_dirty,
        mut postprocess_list,
        overflow_aabb,
        willchange_matrix,
        layer,
    ) in query_pass.p0().iter_mut()
    {
        camera.is_active = false;

        let (global_dirty_rect, render_dirty_mark, viewport) = match p0.get(layer.root()) {
            Ok(r) => r,
            _ => continue,
        };

        // 不脏，不需要组织渲染图， 也不需要渲染脏
        if global_dirty_rect.state == DirtyRectState::UnInit && !render_dirty_mark.0 {
            continue;
        }

        // 如果render_dirty_mark.0, 表示全屏zz
        let mut dirty_rect = global_dirty_rect.value.clone();
        if render_dirty_mark.0 {
            dirty_rect = viewport.0;
        }

        // log::warn!("pass_id1========={:?}, {:?}", dirty_rect, willchange_matrix);
		// 计算视图区域（坐标系为本节点的非旋转坐标系）
		let view_aabb = if let Some(oveflow_rotate) = &overflow_aabb.matrix {
			// let mins = oveflow_rotate.rotate_matrix_invert * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
			// 脏区域变化到当前上下文的非旋转坐标系，与当前上下文的视图aabb相交，得到最终视口区域
			let rr = rotatequad_quad_intersection(&(
				Vector2::new(dirty_rect.mins.x, dirty_rect.mins.y),
				Vector2::new(dirty_rect.mins.x, dirty_rect.maxs.y),
				Vector2::new(dirty_rect.maxs.x, dirty_rect.maxs.y),
				Vector2::new(dirty_rect.maxs.x, dirty_rect.mins.y)
			), &oveflow_rotate.world_rotate_invert, &overflow_aabb.view_box.aabb);

			// let r = calc_bound_box(&aabb, &oveflow_rotate.rotate_matrix_invert);
			// let rr = intersect(&overflow, &r).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), 
			// Point2::new(0.0, 0.0)));
			// log::warn!("rr=====id: {:?} \nrotate_matrix_invert: {:?}, \ndirty_rect: {:?}, \nview_box.aabb: {:?}, \n rr: {:?}, ", entity, &oveflow_rotate.world_rotate_invert, dirty_rect, overflow_aabb.view_box.aabb, rr);
			rr
		} else {
			intersect(&overflow_aabb.view_box.aabb, &dirty_rect).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), 
			Point2::new(0.0, 0.0)))
		};

		
		if view_aabb.mins.x >= view_aabb.maxs.x || view_aabb.mins.y >= view_aabb.maxs.y {
			continue;
		}

		// 计算视图区域（世界坐标系）
		let aabb_temp;
		let view_world_aabb = match &overflow_aabb.matrix {
			Some(r) => {
				aabb_temp = calc_aabb(&view_aabb, &r.world_rotate);
				&aabb_temp
			}
			None => &view_aabb
		};

		// 计算用于剔除的aabb
		// 如果存在transformwillchange，则需要算上脏区域
		// no_will_change用于包围盒剔除渲染对象（渲染对象使用quad来剔除，quad是没有willchange_matrix的参与的）
		let cull_aabb = if let Some(r) = &willchange_matrix.0 {
			if postprocess_list.has_effect() {
				calc_aabb(&view_world_aabb, &r.will_change_invert)
			} else {
				view_aabb.clone()
			}
			
		} else {
			view_aabb.clone()
		};
		*last_dirty = LastDirtyRect {
			// last: view_aabb.clone(),
			no_will_change: cull_aabb,
		};

		// 计算投影矩阵（投影矩阵将view_aabb范围内的对象投影到-1~1， 注意view_aabb所在坐标系为当前节点的非旋转坐标系）
        let project_matrix = create_project(view_aabb.mins.x, view_aabb.maxs.x, view_aabb.mins.y, view_aabb.maxs.y);
		
		// 计算视图矩阵
		let view_temp;
		// 将willchange应用到视图矩阵中
        let mut view_matrix = match &willchange_matrix.0 {
			Some(r) => &r.will_change.0,
			None => {
				view_temp = WorldMatrix::default().0;
				&view_temp
			}
		};
		let view_temp1;
		if let Some(r) = &overflow_aabb.matrix {
			// 由于投影矩阵是将非旋转坐标系中的一个区域投影到-1~1，因此需要将渲染物体变换到非旋转坐标系中
			view_temp1 = r.world_rotate_invert * view_matrix;
			view_matrix = &view_temp1;
		}

        // log::warn!("pass_id2=========\nentity: {:?}, \nproject: {:?}, \nview: {}, \naabb:{:?}, \noverflow_aabb: {:?}", entity, project, view, aabb, overflow_aabb);

        let mut camera_group = camera_material_alloter.alloc();
        camera_group.set_uniform(&ProjectUniform(project_matrix.as_slice()));
        camera_group.set_uniform(&ViewUniform(view_matrix.as_slice()));

        let aabb = Aabb2::new(
            Point2::new(view_aabb.mins.x.floor(), view_aabb.mins.y.floor()),
            Point2::new(view_aabb.maxs.x.ceil(), view_aabb.maxs.y.ceil()),
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
        *camera = Camera {
            view: view_matrix.clone(),
            project: project_matrix,
            bind_group: Some(DrawBindGroup::Offset(camera_group)),
            view_port: aabb,
            world_matrix: world_matrix.clone(),
            is_active: true,
        };

        if postprocess_list.has_effect() {
            postprocess_list.calc(
                16,
                &device,
				&queue,
				&mut post_resource.vballocator,
                // &mut postprocess_pipelines,
                // &mut geometrys,
                // &[Some(wgpu::ColorTargetState {
                //     format: wgpu::TextureFormat::pi_render_default(),
                //     blend: Some(wgpu::BlendState {
                //         color: wgpu::BlendComponent {
                //             operation: wgpu::BlendOperation::Add,
                //             src_factor: wgpu::BlendFactor::SrcAlpha,
                //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                //         },
                //         alpha: wgpu::BlendComponent {
                //             operation: wgpu::BlendOperation::Add,
                //             src_factor: wgpu::BlendFactor::One,
                //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                //         },
                //     }),
                //     write_mask: wgpu::ColorWrites::ALL,
                // })],
                // Some(wgpu::DepthStencilState {
                //     format: TextureFormat::Depth32Float,
                //     depth_write_enabled: true,
                //     depth_compare: CompareFunction::GreaterEqual,
                //     stencil: StencilState::default(),
                //     bias: DepthBiasState::default(),
                // }),
            );
            postprocess_list.view_port = aabb;
            postprocess_list.matrix = WorldMatrix(world_matrix, false);
            // 存在旋转，需要旋转回父上下文
            if let Some(matrix) = &overflow_aabb.matrix {
                postprocess_list.matrix = WorldMatrix(&matrix.from_context_rotate * &postprocess_list.matrix.0, true);
            }
        }
    }

    let mut p0 = query_draw2d_list.p0();
    // 组织渲染列表
    // 用脏区域，查询到脏区域内的渲染节点，对其进行遍历，放入对应的pass中（TODO，aabb查询四叉树）
    let p1 = query_pass.p1();
    for (in_pass_id, draw_list, quad, z_range, is_show, entity) in p1.0.iter() {
        // log::warn!("draw_list1==================entity: {:?}, draw_list: {:?}, {}, {:?}", entity, draw_list, is_show.get_visibility(), quad, );
        let (parent_pass_id, context_dirty) = match p1.1.get(***in_pass_id) {
            Ok(r) => r,
            _ => continue,
        };
		// log::warn!("draw_list2==================context_dirty: {:?}", context_dirty);
        // global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
        if draw_list.len() > 0 {
            if is_show.get_visibility() && intersects(quad, &context_dirty.no_will_change) {
                let mut list = p0.get_mut(***in_pass_id).unwrap();
                let list = &mut list;
                for (draw_id, _) in draw_list.iter() {
                    list.all_list.push((
						DrawIndex::DrawObj(EntityKey(*draw_id)),
						z_range.clone(),
						*draw_info.get(*draw_id).unwrap(),
					));
                }
            } else {
				// log::warn!("cull======{:?}, {:?}, is_show: {:?}", quad, &context_dirty.no_will_change, is_show.get_visibility());
			}
        }

        if let Some(parent) = parent_pass_id {
            if let Ok(mut p) = p0.get_mut(*parent.0) {
                p.all_list
                    .push((DrawIndex::Pass2D(EntityKey(entity)), z_range.clone(), DrawInfo::new(10, false)));
            }
        }
    }

    // 遍历所有的pass，设置不透明渲染列表和候命渲染列表
    for mut list in query_draw2d_list.p0().iter_mut() {
        let list = &mut list;
        list.opaque.clear();
        list.transparent.clear();
        if list.all_list.len() == 0 {
            continue;
        }

		// 按深度从小到大排序
        list.all_list.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
            if a_z_depth.start < b_z_depth.start {
                std::cmp::Ordering::Less
            } else if a_z_depth.start > b_z_depth.start {
                std::cmp::Ordering::Greater
            } else {
                if a_sort.order() < b_sort.order() {
                    std::cmp::Ordering::Less
                } else if a_sort.order() > b_sort.order() {
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
    let p2 = query_pass.p2();
    let camera_query = p2.0;
    let mut postprocess_lists = p2.1;
    for (list, pass_id, parent_pass_id) in p1.iter() {
        // 不存在后处理，不主动分配depth（需要pass2d分配）
        // 如果post不为none，但长度大于0，表示根节点，也需要自己分配depth
        match postprocess_lists.get(pass_id) {
            Ok(r) if r.has_effect() => {
                if !parent_pass_id.is_null() {
                    continue;
                }
            }
            _ => (),
        };


        alloc_depth(
            &device,
            &p1,
            &mut postprocess_lists,
            &camera_query,
            list,
            &share_layout,
            &mut draw_state,
            &buffer_assets,
            &bind_group_assets,
            &mut depth_cache,
            &mut 0,
            // &mut geometrys,
            // &mut postprocess_pipelines,
        );
    }

    // 清理列表
    for mut list in query_draw2d_list.p0().iter_mut() {
        list.all_list.clear();
    }

    group_alloc_center.write_buffer(&device, &queue);

	// 重置渲染脏
	for mut i in query_root.p1().iter_mut() {
		**i = false;
	}
}

pub fn create_project(left: f32, right: f32, top: f32, bottom: f32) -> Matrix4 {
    let ortho = Orthographic3::new(left, right, bottom, top, -1.0, 1.0);
    Matrix4::from(ortho)
}

fn alloc_depth<'a>(
    device: &'a RenderDevice,
    pass2d: &'a Query<(&'static Draw2DList, Entity, &'static ParentPassId)>,
    post_process_list: &mut Query<&'static mut PostProcessList>,
    camera_query: &'a Query<(&'static Camera, &'static View)>,
    list: &'a Draw2DList,
    share_layout: &'a ShareLayout,
    draw_state: &'a mut Query<&'static mut DrawState>,
    buffer_assets: &'a Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
    depth_cache: &'a mut DepthCache,
    cur_depth: &'a mut usize,
    // geometrys: &mut PostProcessGeometryManager,
    // postprocess_pipelines: &mut PostProcessMaterialMgr,
) {
    for index in list.all_list.iter() {
        match &index.0 {
            // 如果绘制索引是一个DrawObj，则设置该DrawObj的depth group
            DrawIndex::DrawObj(draw_key) => {
				depth_cache.or_create_depth(*cur_depth, device, bind_group_assets);
                alloc_depth_one(**draw_key, draw_state, cur_depth, depth_cache);
            }
            // 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
            DrawIndex::Pass2D(pass2d_id) => {
                let list = if let Ok((list, pass_id, _parent_pass_id)) = pass2d.get(**pass2d_id) {
                    let mut post = post_process_list.get_mut(pass_id).unwrap();
                    match post.has_effect() {
                        true => {
                            post.depth = *cur_depth as f32;
							depth_cache.or_create_depth(*cur_depth, device, bind_group_assets);
                            *cur_depth += 1;
                            continue;
                        }
                        false => list,
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
                    // geometrys,
                    // postprocess_pipelines,
                );
            }
        }
    }
}

fn alloc_depth_one<'a>(draw_key: Entity, draw_state: &'a mut Query<&'static mut DrawState>, cur_depth: &'a mut usize, depth_cache: &'a DepthCache) {
    let mut draw_state = match draw_state.get_mut(draw_key) {
        Ok(r) => r,
        _ => return,
    };
    draw_state.bypass_change_detection().bindgroups.insert_group(DepthBind::set(), DrawBindGroup::Independ(depth_cache.list[*cur_depth].clone()));

    *cur_depth += 1;
}
