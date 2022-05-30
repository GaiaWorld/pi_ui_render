use std::io::Result;

use futures::{future::BoxFuture, FutureExt};
use nalgebra::Orthographic3;
use pi_assets::{mgr::{LoadResult, AssetMgr}, asset::Handle};
use pi_ecs::{prelude::{Join, Write, ResMut, OrDefault, Query, Res, ParamSet, FromWorld, world::WorldRead, DataState}, monitor::Event, storage::Offset, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_null::Null;
use pi_render::{
	graph::graph::RenderGraph, 
	rhi::{device::RenderDevice, asset::RenderRes, bind_group::BindGroup, buffer::Buffer}, components::view::target_alloc::ShareTargetView
};
use pi_share::Share;
use pi_spatialtree::quad_helper::intersects;

use crate::{
	components::{
		calc::{NodeId, ContentBox, DrawList, Visibility, WorldMatrix, Quad, InPassId}, 
		pass_2d::{Camera, DirtyRectState, GraphId, Draw2DList, ParentPassId, Pass2D, DirtyRect, Pass2DKey, DrawIndex, PostProcessList}, 
		user::{Matrix4, Node}, 
		draw_obj::{DrawState, DrawObject}
	}, 
	utils::{
		tools::{intersect, calc_hash}, 
		shader_helper::DEPTH_GROUP
	}, 
	resource::draw_obj::ShareLayout
};

use super::pass_graph_node::{Pass2DNode};

pub struct CalcRender;

#[setup]
impl CalcRender{
	#[system]
	pub async fn calc_render<'a>(
		mut query_pass: Query<Pass2D, (Write<Camera>, Join<NodeId, Node, &ContentBox>)>,
		mut query_draw2d_list: ParamSet<(Query<Pass2D, &'static mut Draw2DList>, Query<Pass2D, (&'static Draw2DList, Option<& 'static PostProcessList>)>)> ,
		query_node: Query<Node, (&InPassId, &DrawList, &Quad, OrDefault<Visibility>)>,
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
		
		let p0 = query_draw2d_list.p0_mut();
		// 组织渲染列表
		// 用脏区域，查询到脏区域内的渲染节点，对其进行遍历，放入对应的pass中（TODO，aabb查询四叉树）
		for (pass_id, draw_list, quad, visibility) in query_node.iter() {
			// global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
			if **visibility && intersects(quad, &global_dirty_rect.value) {
				for draw_id in draw_list.iter() {
					p0.get_unchecked_mut(**pass_id).all_list.push(DrawIndex::DrawObj(*draw_id));
				}
			}
		}

		// 遍历所有的pass，设置不透明渲染列表和候命渲染列表
		for mut list in p0.iter_mut() {
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
		}

		let p1 = query_draw2d_list.p1();
		for (list, post) in p1.iter() {
			// 不存在后处理，不主动分配depth（需要pass2d分配）
			// 如果post不为none，但长度大于0，表示根节点，也需要自己分配depth
			if let None = post {
				continue; 
			}

			alloc_depth(&device, p1, list, &share_layout, &mut draw_state, &buffer_assets, 
				&bind_group_assets, &mut depth_cache, &mut 0).await;
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
		mut rg: ResMut<RenderGraph<Pass2DKey>>,
	) {
		// log::info!("depend_graph_node================={:?}", e.id);
		let (parent_id, graph_id) = query.get_unchecked_by_entity(e.id);
		if parent_id.is_null() {
			println!("finish========");
			if let Err(e) = rg.set_finish(**graph_id, true) {
				log::error!("{:?}", e);
			}
		} else {
			// rg.set_node_finish(graph_id, false);
			let parent_graph_id = query_graph.get_unchecked(**parent_id);
			// 建立父子依赖关系，使得子pass先渲染
			println!("set_depend========");
			if let Err(e) = rg.set_depend(**graph_id, **parent_graph_id) {
				log::error!("{:?}", e);
			}
		}
	}
}



pub fn create_project(left: f32, right: f32, top: f32, bottom: f32) -> Matrix4 {
	let ortho = Orthographic3::new(left, right, top, bottom, -1.0, 1.0);
	Matrix4::from(ortho)
}

fn alloc_depth<'a>(
	device: &'a RenderDevice,
	pass2d: &'a Query<Pass2D, (&Draw2DList, Option<&PostProcessList>)>,
	list: &'a Draw2DList,
	share_layout: &'a ShareLayout,
	mut draw_state: &'a mut Query<DrawObject, &mut DrawState>,
	buffer_assets: &'a Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
	depth_cache: &'a mut Vec<Handle<RenderRes<BindGroup>>>,
	cur_depth: &'a mut usize,
) -> BoxFuture<'a, ()> {
	async move {
		for index in list.all_list.iter() {
			match index {
				// 如果绘制索引是一个DrawObj，则设置该DrawObj的depth group
				DrawIndex::DrawObj(draw_key) => {
					let mut draw_state_item = match draw_state.get_mut(*draw_key) {
						Some(r) => r,
						None => continue,
					};
		
					let bind_group = match depth_cache.get(*cur_depth) {
						Some(r) => r.clone(),
						None => {
							let value = *cur_depth as f32 / 600000.0;
							let key = calc_hash(cur_depth); // TODO
							let uniform_buf = match AssetMgr::load(buffer_assets, &key) {
								LoadResult::Ok(r) => r,
								LoadResult::Wait(f) => f.await.unwrap(),
								LoadResult::Receiver(recv) => {
									let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
										label: Some("depth buffer init"),
										contents: bytemuck::cast_slice(&[value]),
										usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
									});
									recv.receive(key, Ok(RenderRes::new(uniform_buf, 5))).await.unwrap()
								},
							};
							let d = match AssetMgr::load(bind_group_assets, &key) {
								LoadResult::Ok(r) => r,
								LoadResult::Wait(f) => f.await.unwrap(),
								LoadResult::Receiver(recv) => {
									let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
										layout: &share_layout.depth,
										entries: &[
											wgpu::BindGroupEntry {
												binding: 0,
												resource: uniform_buf.as_entire_binding(),
											},
										],
										label: Some("depth group create"),
									});
									recv.receive(key, Ok(RenderRes::new(group, 5))).await.unwrap()
								},
							};
							depth_cache.push(d.clone());
							d
						}
					};
					
					draw_state_item.bind_groups.insert(
						DEPTH_GROUP, 
						bind_group
					);
				},
				// 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
				DrawIndex::Pass2D(pass2d_id) => {
					alloc_depth(device, pass2d, list, share_layout, draw_state, buffer_assets, bind_group_assets, depth_cache, cur_depth).await
				}
			}
			*cur_depth += 1;
		}
	}.boxed()
}

/// depth BindGroup缓存
#[derive(Deref, DerefMut, Default)]
pub struct DepthCache(Vec<Handle<RenderRes<BindGroup>>>);

