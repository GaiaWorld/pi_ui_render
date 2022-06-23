use std::io::Result;

use nalgebra::Orthographic3;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_atom::Atom;
use pi_ecs::{prelude::{Join, Write, ResMut, OrDefault, Query, Res, ParamSet, res::WriteRes}, monitor::{Event, EventType}, storage::Offset, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_null::Null;
use pi_render::{
	graph::graph::RenderGraph, 
	rhi::{device::RenderDevice, asset::RenderRes, bind_group::BindGroup, buffer::Buffer}, components::view::target_alloc::ShareTargetView
};
use pi_share::Share;
use pi_spatialtree::quad_helper::intersects;
use wgpu::{IndexFormat, DepthStencilState, TextureFormat, CompareFunction, StencilState, DepthBiasState};

use crate::{
	components::{
		calc::{NodeId, ContentBox, DrawList, Visibility, WorldMatrix, Quad, InPassId, Pass2DId, TransformWillChangeMatrix, OverflowAabb}, 
		pass_2d::{Camera, DirtyRectState, GraphId, Draw2DList, ParentPassId, Pass2D, DirtyRect, DrawIndex, PostProcessList, LastDirtyRect, ViewMatrix}, 
		user::{Matrix4, Node, CgColor, TransformWillChange, Aabb2, Point2}, 
		draw_obj::{DrawState, DrawObject, VSDefines, FSDefines}
	}, 
	utils::{
		tools::{intersect, calc_aabb, calc_bound_box}, 
		shader_helper::{DEPTH_GROUP, VIEW_GROUP, PROJECT_GROUP}
	}, 
	resource::{
		draw_obj::{ShareLayout, UnitQuadBuffer, Shaders, PipelineMap, ShaderInfoMap, ShaderCatch, VertexBufferLayoutMap, StateMap, DynFboClearColorBindGroup, ClearColorBindGroup, PipelineState}, 
		ClearColor, ClearDrawObj
	}, 
	system::{
		node::background_color::{create_rgba_bind_group}, 
		draw_obj::{pipeline::CalcPipeline, world_marix::modify_world_matrix}, shader_utils::{create_camera_bind_group, StaticIndex, create_depth_group, color::{ColorStaticIndex, COLOR_GROUP}}
	}
};

use super::pass_graph_node::{Pass2DNode};

pub struct CalcRender;

/// 需要为清屏颜色创建DrawObj，依赖CalcBackground的初始化，请在初始化本功能前先初始化CalcBackground
#[setup]
impl CalcRender{
	// 创建清屏的drawobj
	#[init]
	pub fn init(
		mut pipeline_map: ResMut<PipelineMap>,
		mut shader_map: ResMut<ShaderInfoMap>,
		mut depth_cache: ResMut<DepthCache>,

		unit_quad_buffer: Res<UnitQuadBuffer>,
		static_index: Res<ColorStaticIndex>,
		shader_statics: Res<Shaders>,
		device: Res<RenderDevice>,
		shader_catch: Res<ShaderCatch>,
		vertex_buffer_layout_map: Res<VertexBufferLayoutMap>,
		mut state_map: ResMut<StateMap>,
		share_layout: Res<ShareLayout>,

		buffer_assets: Res<Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<Share<AssetMgr<RenderRes<BindGroup>>>>,
		
		mut dyn_fbo_clear_color_bind_group: WriteRes<DynFboClearColorBindGroup>,
		mut clear_draw_obj: WriteRes<ClearDrawObj>,
	) {
		let pipeline_state = create_clear_pipeline_state();
		let pipeline_state = state_map.insert(pipeline_state);
		let static_index = StaticIndex { 
			shader: static_index.shader,
			pipeline_state: pipeline_state, 
			vertex_buffer_index: static_index.vertex_buffer_index, 
			name: "clear screen" 
		};

		let color_group_layout = match shader_statics.get(static_index.shader) {
			Some(r) => r.bind_group.get(COLOR_GROUP).unwrap(),
			None => return,
		};
		
		// 设置清屏颜色的vb、ib
		let mut draw_state = DrawState::default();
		draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
		draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
		
		// 设置清屏颜色的pipeline
		let (vs_defines, fs_defines) = (VSDefines::default(), FSDefines::default());
		let pipeline = CalcPipeline::calc_pipeline(
			&vs_defines,
			&fs_defines,
			&static_index,

			&shader_statics,
			&device,
			&vertex_buffer_layout_map,
			&state_map,
			&shader_catch,

			&mut pipeline_map,
			&mut shader_map,
			&share_layout,
		);
		// 设置pipeline
		draw_state.pipeline = Some(pipeline);

		// 设置清屏颜色的世界矩阵

		// 设置清屏颜色的世界矩阵、投影矩阵、视图矩阵
		// 视图矩阵和投影矩阵都设置为单位阵
		let view = WorldMatrix::default().0; 
		let project = WorldMatrix::default().0;
		let view_bind_group = create_camera_bind_group(
			&view, 
			&share_layout.view, 
			&device, 
			&buffer_assets,
			&bind_group_assets,);
		let project_bind_group = create_camera_bind_group(
			&project, 
			&share_layout.project, 
			&device, 
			&buffer_assets,
			&bind_group_assets,);
		draw_state.bind_groups.insert(VIEW_GROUP, view_bind_group);
		draw_state.bind_groups.insert(PROJECT_GROUP, project_bind_group);

		// 世界矩阵
		let view = Matrix4::new(
			2.0, 0.0, 0.0, -1.0,
			0.0, 2.0, 0.0, -1.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		);
		modify_world_matrix(
			&WorldMatrix(view, false),
			&mut draw_state,
			&device,
			&share_layout.matrix,
			&buffer_assets,
			&bind_group_assets,
		);

		// 深度设置为-1(最远)
		let depth_bind_group = create_depth_group(
			0,
			&buffer_assets, 
			&bind_group_assets, 
			&mut depth_cache,
			&device,
			&share_layout);
		draw_state.bind_groups.insert(DEPTH_GROUP, depth_bind_group);

		dyn_fbo_clear_color_bind_group.write(
			DynFboClearColorBindGroup(create_rgba_bind_group(
				&CgColor::new(1.0, 1.0, 1.0, 0.0),
				&device,
				color_group_layout,
				&buffer_assets,
				&bind_group_assets,
			))
		);
		clear_draw_obj.write(ClearDrawObj(draw_state));
	}

	
	#[system]
	pub fn calc_render<'a>(
		parent_pass_id: Query<Pass2D, Option<&ParentPassId>>,
		mut query_draw2d_list: ParamSet<(
			Query<Pass2D, &'static mut Draw2DList>, 
			Query<Pass2D, (
				&'static Draw2DList, 
				Option<&'static PostProcessList>)>)>,
		mut query_pass: ParamSet<(
			Query<Pass2D, (
				Write<Camera>, 
				Write<ViewMatrix>, 
				Write<LastDirtyRect>,
				Option<&'static OverflowAabb>,
				Join<NodeId, Node, (
					&'static ContentBox, 
					&'static Quad,
					Option<&'static TransformWillChangeMatrix>, 
					Option<&'static TransformWillChange>
				)>
			)>,
			Query<Node, (&'static InPassId, Option<&'static Pass2DId>, Option<&'static DrawList>, &'static Quad, OrDefault<Visibility>, Join<InPassId, Pass2D, &'static LastDirtyRect>)>,
		)>,
		mut draw_state: Query<DrawObject, &mut DrawState>,
		share_layout: Res<'a, ShareLayout>,
		device: Res<'a, RenderDevice>,
		global_dirty_rect: Res<'a, DirtyRect>,

		buffer_assets: Res<'a, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets:  Res<'a, Share<AssetMgr<RenderRes<BindGroup>>>>,
		mut depth_cache: ResMut<'a, DepthCache>,
	) -> Result<()> {
		// log::info!("calc_render=================");
		// 不脏，不需要组织渲染图， 也不需要渲染
		if global_dirty_rect.state == DirtyRectState::UnInit {
			return Ok(());
		}
	
		for (mut camera, view_matrix, mut last_dirty, overflow_aabb, (context_box, quad, willchange_matrix, will_change)) in query_pass.p0_mut().iter_mut() {
			// 存在脏区域，与现有脏区域相交，得到最终脏区域
			let mut c;

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
							None => Aabb2::new(
								Point2::new(0.0, 0.0), 
								Point2::new(0.0, 0.0))
						};
					}
				}
				&c
			} else {
				&aabb
			};

			let aabb = if let Some(aabb) = intersect(&global_dirty_rect.value, context_box) {
				// 如果存在transformwillchange，则需要算上脏区域
				// no_will_change用于包围盒剔除渲染对象（渲染对象使用quad来剔除，quad是没有willchange_matrix的参与的）
				let no_will_change = if let Some(r) = willchange_matrix {
					calc_aabb(&aabb, &r.will_change_invert)
				} else {
					aabb
				};

				last_dirty.write(LastDirtyRect{
					last: aabb.clone(),
					no_will_change,
				});

				if let Some(overflow) = overflow_aabb {
					// 存在裁剪区，并且旋转，
					if let (Some(overflow), Some(r)) = (&overflow.aabb, &overflow.matrix) {
						let r = calc_bound_box(&aabb, r);
						intersect(&overflow, &r).unwrap_or(
							Aabb2::new(
								Point2::new(0.0, 0.0), 
								Point2::new(0.0, 0.0)))
					} else{
						aabb
					}
				} else {
					aabb
				}
			} else {
				continue;
			};

			let project = create_project(
				aabb.mins.x,
				aabb.maxs.x,
				aabb.mins.y,
				aabb.maxs.y,
			);
			let view = WorldMatrix::default().0;
			
			let view = if let Some(overflow) = overflow_aabb {
				// 存在裁剪区，并且未旋转，则直接与视口相交
				if let (Some(_aabb), Some(mtrix)) = (&overflow.aabb, &overflow.matrix) {
					mtrix
				} else {
					&view
				}
			}else if let (Some(willchange_matrix), Some(_)) = (willchange_matrix, will_change) {
				&willchange_matrix.will_change
			} else {
				&view
			};

			let project_bind_group = create_camera_bind_group(
				&project, 
				&share_layout.project, 
				&device,
				&buffer_assets,
				&bind_group_assets,
			);
			let view_bind_group = create_camera_bind_group(
				&view, 
				&share_layout.view, 
				&device,
				&buffer_assets,
				&bind_group_assets,
			);

			let aabb = Aabb2::new(
				Point2::new(aabb.mins.x.floor(), aabb.mins.y.floor() ), 
				Point2::new(aabb.maxs.x.ceil(), aabb.maxs.y.ceil() ));
			
			camera.write(Camera {
				// view: match willchange_matrix {
				// 	Some(r) => r.0.clone(),
				// 	Non
				// }, 
				view_bind_group: Some(view_bind_group),
				project_bind_group: Some(project_bind_group),
				view_port: aabb,
			});

			
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
		for (in_pass_id, pass_id, draw_list, quad, visibility, context_dirty) in query_pass.p1().iter() {
			// global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
			if let Some(draw_list) = draw_list {
				if **visibility && intersects(quad, &context_dirty.no_will_change) {
					for draw_id in draw_list.iter() {
						p0.get_unchecked_mut(**in_pass_id).all_list.push(DrawIndex::DrawObj(*draw_id));
					}
				}
			}
			
			if let Some(pass_id) = pass_id {
				if let Some(parent) = parent_pass_id.get_unchecked(pass_id.0) {
					if let Some(mut p) = p0.get_mut(parent.0) {
						p.all_list.push(DrawIndex::Pass2D(pass_id.0));
					}
				}
			}
		}

		// 遍历所有的pass，设置不透明渲染列表和候命渲染列表
		for mut list in p0.iter_mut() {
			list.opaque.clear();
			list.transparent.clear();
			if list.all_list.len() == 0 {
				continue;
			}

			// TODO
			// list.all_list.sort_by(|a, b| {

			// });
			
			for i in 0..list.all_list.len() {
				let entity = list.all_list[i];
				// 暂时放入不透明列表，TODO
				list.opaque.push(entity);
			}

			list.all_list.clear();
		}

		let p1 = query_draw2d_list.p1();
		for (list, post) in p1.iter() {
			// 不存在后处理，不主动分配depth（需要pass2d分配）
			// 如果post不为none，但长度大于0，表示根节点，也需要自己分配depth
			if let None = post {
				continue; 
			}

			alloc_depth(&device, p1, list, &share_layout, &mut draw_state, &buffer_assets, 
				&bind_group_assets, &mut depth_cache, &mut 0);
		}
		Ok(())
	}
	
	/// 创建渲染图节点
	/// 插入Draw2DList
	#[listen(entity=(Pass2D, Create))]
	pub fn create_graph_node(
		e: Event,
		mut query: Query<Pass2D, (Write<GraphId>, Write<Draw2DList>)>,
		mut rg: ResMut<RenderGraph<Option<ShareTargetView>>> ,
	) {
		// log::info!("create_graph_node================={:?}", e.id);
		let node = Pass2DNode::new(unsafe { Id::new(e.id.local()) });
		let graph_id = rg.add_node(format!("Pass2D {:?}", e.id.local().offset()), node);
		let (mut graph_id_item, mut list_item) = query.get_unchecked_mut_by_entity(e.id);
		graph_id_item.write(GraphId(graph_id));
		list_item.write(Draw2DList::default());
	}
	
	// 移除渲染图节点
	#[listen(entity=(Pass2D, Delete))]
	pub fn delete_graph_node(
		e: Event,
		query: Query<Pass2D, &GraphId>,
		// rg: Res<RenderGraph>,
	) {
		// log::info!("delete_graph_node================={:?}", e.id);
		if let Some(_graph_id) = query.get_by_entity(e.id) {
			// (*rg).remove_node(*graph_id); // TODO
		}
	}
	
	#[listen(component=(Pass2D, ParentPassId, Create))]
	pub fn depend_graph_node(
		e: Event,
		query: Query<Pass2D, (&ParentPassId, &GraphId)>,
		query_graph: Query<Pass2D, &GraphId>,
		mut rg: ResMut<RenderGraph<Option<ShareTargetView>>>,
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
			if let Err(e) = rg.set_depend(**graph_id, **parent_graph_id) {
				log::error!("{:?}", e);
			}
		}
	}

	#[listen(resource=(ClearColor, (Modify, Create, Delete)))]
	pub fn clear_change(
		e: Event,
		color: Option<Res<ClearColor>>,

		mut bind_group: ResMut<ClearColorBindGroup>,
		device: Res<RenderDevice>,
		buffer_assets: Res<Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<Share<AssetMgr<RenderRes<BindGroup>>>>,
		static_index: Res<ColorStaticIndex>,
		shader_statics: Res<Shaders>,

	) {
		match e.ty {
			EventType::Create | EventType::Modify => {
				let color_group_layout = match shader_statics.get(static_index.shader) {
					Some(r) => r.bind_group.get(COLOR_GROUP).unwrap(),
					None => return,
				};
				bind_group.0 = Some(create_rgba_bind_group(
					&color.unwrap(),
					&device,
					color_group_layout,
					&buffer_assets,
					&bind_group_assets,
				));
			},
			EventType::Delete => {},
		};
		
		
		// // log::info!("create_graph_node================={:?}", e.id);
		// let node = Pass2DNode::new(unsafe { Id::new(e.id.local()) });
		// let graph_id = rg.add_node(format!("Pass2D {:?}", e.id.local().offset()), node);
		// let (mut graph_id_item, mut list_item) = query.get_unchecked_mut_by_entity(e.id);
		// graph_id_item.write(GraphId(graph_id));
		// list_item.write(Draw2DList::default());
	}
}



pub fn create_project(left: f32, right: f32, top: f32, bottom: f32) -> Matrix4 {
	let ortho = Orthographic3::new(left, right, bottom, top, -1.0, 1.0);
	Matrix4::from(ortho)
}

fn alloc_depth<'a>(
	device: &'a RenderDevice,
	pass2d: &'a Query<Pass2D, (&Draw2DList, Option<&PostProcessList>)>,
	list: &'a Draw2DList,
	share_layout: &'a ShareLayout,
	draw_state: &'a mut Query<DrawObject, &mut DrawState>,
	buffer_assets: &'a Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
	depth_cache: &'a mut Vec<Handle<RenderRes<BindGroup>>>,
	cur_depth: &'a mut usize,
) {
	for index in list.all_list.iter() {
		match index {
			// 如果绘制索引是一个DrawObj，则设置该DrawObj的depth group
			DrawIndex::DrawObj(draw_key) => {
				let mut draw_state_item = match draw_state.get_mut(*draw_key) {
					Some(r) => r,
					None => continue,
				};
	
				let bind_group = create_depth_group(
					*cur_depth, 
					buffer_assets, 
					bind_group_assets, 
					depth_cache,
					device,
					share_layout);
				
				draw_state_item.bind_groups.insert(
					DEPTH_GROUP, 
					bind_group
				);
				*cur_depth += 1;
			},
			// 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
			DrawIndex::Pass2D(pass2d_id) => {
				let list = if let Some(r) = pass2d.get(pass2d_id.clone()) {
					r.0
				} else {
					continue;
				};
				alloc_depth(device, pass2d, list, share_layout, draw_state, buffer_assets, bind_group_assets, depth_cache, cur_depth)
			}
		}
	}
}

lazy_static! {
    pub static ref DEPTH: Atom = Atom::from("depth");
}

/// depth BindGroup缓存
#[derive(Deref, DerefMut, Default)]
pub struct DepthCache(Vec<Handle<RenderRes<BindGroup>>>);


pub fn create_clear_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![wgpu::ColorTargetState {
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			blend: Some(wgpu::BlendState {
				color: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
			}),
			write_mask: wgpu::ColorWrites::ALL,
		}],
		primitive: wgpu::PrimitiveState {
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			..Default::default()
		},
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth32Float,
			depth_write_enabled: true,
			depth_compare: CompareFunction::Never,
			stencil: StencilState::default(),
			bias: DepthBiasState::default(),
		}),
		multisample: wgpu::MultisampleState::default(),
		multiview: None,
	}
}

