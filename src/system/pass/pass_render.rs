use nalgebra::Orthographic3;
use pi_ecs::{prelude::{Join, Write, ResMut, OrDefault, Query, Res}, monitor::Event, storage::Offset, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_null::Null;
use pi_render::{graph::graph::RenderGraph, rhi::device::RenderDevice};
use pi_share::Share;
use pi_spatialtree::quad_helper::intersects;

use crate::{components::{calc::{NodeId, ContentBox, DrawList, Visibility, WorldMatrix, Quad, InPassId}, pass_2d::{Camera, DirtyRectState, GraphId, Draw2DList, ParentPassId, Pass2D, DirtyRect}, user::{Matrix4, Node}, draw_obj::{DrawState, DrawObject}}, utils::{tools::intersect, shader_helper::DEPTH_GROUP}, resource::draw_obj::ShareLayout};

use super::pass_graph_node::Pass2DNode;

pub struct CalcRender;

#[setup]
impl CalcRender{
	#[system]
	pub fn calc_render(
		mut query_pass: Query<Pass2D, (Write<Camera>, Join<NodeId, Node, &ContentBox>)>,
		query_node: Query<Node, (&InPassId, &DrawList, &Quad, OrDefault<Visibility>)>,
		mut query_draw2d_list: Query<Pass2D, &mut Draw2DList>,
		mut draw_state: Query<DrawObject, &mut DrawState>,
		share_layout: Res<ShareLayout>,
		device: Res<RenderDevice>,
		global_dirty_rect: Res<DirtyRect>,
	) {
		// log::info!("calc_render=================");
		// 不脏，不需要组织渲染图， 也不需要渲染
		if global_dirty_rect.state == DirtyRectState::UnInit {
			return;
		}
	
		for (mut camera, context_box) in query_pass.iter_mut() {
			// TODO， 还应该判断TransformWillChange
			// 求全局脏区域和自身脏区域的ContextBox的交集
			let aabb = intersect(&global_dirty_rect.value, &context_box.0);
			match aabb {
				Some(aabb) => {
					let view = WorldMatrix::default().0; 
					let project = create_project(
						aabb.mins.x,
						aabb.maxs.x,
						aabb.mins.y,
						aabb.maxs.y,
					);
					let mut vec = Vec::new();
					vec.extend_from_slice(project.as_slice());
					vec.extend_from_slice(view.as_slice());
					let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("camera buffer init"),
						contents: bytemuck::cast_slice(vec.as_slice()),
						usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
					});

					let bind_group = device.create_bind_group(
						&wgpu::BindGroupDescriptor {
							layout: &share_layout.camera,
							entries: &[
								wgpu::BindGroupEntry {
									binding: 0,
									resource: buf.as_entire_binding(),
								},
							],
							label: Some("camera create"),
						}
					);
					camera.write(Camera {
						view, project, bind_group: Some(bind_group),
						view_port: aabb,
					});
				},
				_ => (),
			}
		}
	
		// 组织渲染列表
		// 用脏区域，查询到脏区域内的渲染节点，对其进行遍历，放入对应的pass中（TODO，aabb查询四叉树）
		for (pass_id, draw_list, quad, visibility) in query_node.iter() {
			// global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
			if **visibility && intersects(quad, &global_dirty_rect.value) {
				for draw_id in draw_list.iter() {
					query_draw2d_list.get_unchecked_mut(**pass_id).all_list.push(*draw_id);
				}
			}
		}

		// 遍历所有的pass，设置不透明渲染列表和候命渲染列表
		for mut list in query_draw2d_list.iter_mut() {
			if list.all_list.len() == 0 {
				continue;
			}

			// TODO
			// list.all_list.sort_by(|a, b| {

			// });
			
			for i in 0..list.all_list.len() {
				let entity = list.all_list[i];

				let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
					label: Some("depth buffer init"),
					contents: bytemuck::cast_slice(&[i as f32 / 600000.0]),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
				});
				let bind_group = Share::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
					layout: &share_layout.depth,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: uniform_buf.as_entire_binding(),
						},
					],
					label: Some("depth group create"),
				}));

				draw_state.get_unchecked_mut(entity).bind_groups.insert(
					DEPTH_GROUP, 
					bind_group
				);
				let entity = list.all_list[i];
				// 暂时放入不透明列表，TODO
				list.opaque.push(entity);
			}
		}
	}
	
	/// 创建渲染图节点
	/// 插入Draw2DList
	#[listen(entity=(Pass2D, Create))]
	pub fn create_graph_node(
		e: Event,
		mut query: Query<Pass2D, (Write<GraphId>, Write<Draw2DList>)>,
		mut rg: ResMut<RenderGraph>,
	) {
		// log::info!("create_graph_node================={:?}", e.id);
		let node = Pass2DNode{
			pass2d_id: unsafe { Id::new(e.id.local()) },
		};
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
		mut rg: ResMut<RenderGraph>,
	) {
		// log::info!("depend_graph_node================={:?}", e.id);
		let (parent_id, graph_id) = query.get_unchecked_by_entity(e.id);
		if parent_id.is_null() {
			if let Err(e) = rg.set_node_finish(**graph_id, true) {
				log::error!("{:?}", e);
			}
		} else {
			// rg.set_node_finish(graph_id, false);
			let parent_graph_id = query_graph.get_unchecked(**parent_id);
			// 建立父子依赖关系，使得子pass先渲染
			if let Err(e) = rg.add_node_edge(**graph_id, **parent_graph_id) {
				log::error!("{:?}", e);
			}
		}
	}
}



pub fn create_project(left: f32, right: f32, top: f32, bottom: f32) -> Matrix4 {
	let ortho = Orthographic3::new(left, right, top, bottom, -1.0, 1.0);
	Matrix4::from(ortho)
}