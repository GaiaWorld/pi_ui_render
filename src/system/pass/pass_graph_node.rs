use std::borrow::BorrowMut;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs_macros::{listen, setup};
use pi_map::hashmap::HashMap;
use pi_null::Null;
use pi_postprocess::{postprocess_geometry::PostProcessGeometryManager, postprocess_pipeline::PostProcessMaterialMgr, material::{blend::EBlend}, temprory_render_target::{EPostprocessTarget, PostprocessShareTarget}};
use pi_render::{
    components::view::target_alloc::{ShareTargetView, SafeAtlasAllocator, TargetType, TargetDescriptor, TextureDescriptor},
    graph::{
        node::{Node, NodeRunError},
        RenderContext,
    },
    rhi::{CommandEncoder, bind_group_layout::BindGroupLayout, device::RenderDevice, asset::RenderRes, bind_group::BindGroup, buffer::Buffer, texture::ScreenTexture, RenderQueue, dyn_uniform_buffer::Group},
};
use futures::{future::BoxFuture, FutureExt};
use pi_ecs::{prelude::{QueryState, FromWorld, World, Res, res::WriteRes}, monitor::Event, storage::Offset};
use pi_share::{ShareRefCell, Share,};
use pi_slotmap::DefaultKey;
use smallvec::SmallVec;
use wgpu::RenderPass;

use crate::{components::{draw_obj::{DrawObject, DrawState}, pass_2d::{Camera, Draw2DList, Pass2DKey, Pass2D, PostProcessList, RenderTarget, DrawIndex, PostTemp, ViewMatrix, ScreenTarget, ParentPassId}, user::{Aabb2, Matrix4, Point2}, calc::NodeId}, resource::{Viewport, draw_obj::{DynFboClearColorBindGroup, ClearColorBindGroup, CommonSampler, ShareLayout, CopyFboToScreen, DynBindGroups}, ClearDrawObj}, utils::{tools::{calc_hash, calc_float_hash}, shader_helper::{WORLD_MATRIX_GROUP, PROJECT_GROUP, VIEW_GROUP}}, system::{ draw_obj::world_marix::create_world_matrix_bind}, shaders::{color::{ColorShader, ColorMaterialGroup, CameraMatrixGroup}}};


/// Pass2D 渲染图节点
#[derive(Clone, Default)]
pub struct Pass2DNode{
	// // 输入描述
	// input: Vec<SlotInfo>,
	// // 输出描述
	// output: Vec<SlotInfo>,
	pub pass2d_id: Pass2DKey,
	pub output_target: Option<ShareTargetView>,
	pub last_post_key: DefaultKey,
	pub out: Option<ShareTargetView>,

	// pub param: ParamState,
}

pub struct Param<'s> {
	pass2d_query: QueryState<Pass2D,(&'static Camera, Option<&'static ViewMatrix>, &'static Draw2DList, Option<&'static NodeId>, &'static ParentPassId)>,
	draw_query: QueryState<DrawObject, &'static DrawState>,
	post_query: QueryState<Pass2D, &'static PostProcessList>,
	last_rt: &'s RenderTarget,
	screen: &'s ScreenTarget,
	surface: &'s ScreenTexture,
	atlas_allocator: &'s SafeAtlasAllocator,
	t_type: &'s DynTargetType,
	buffer_assets: &'s Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'s Share<AssetMgr<RenderRes<BindGroup>>>,
	device: &'s RenderDevice,
	queue: &'s RenderQueue,
	post_bind_group_layout: &'s PostBindGroupLayout,
	share_layout: &'s ShareLayout,
	dyn_bind_groups: &'s DynBindGroups,
	postprocess_pipelines: &'s PostProcessMaterialMgr,
    geometrys: &'s PostProcessGeometryManager,

	// 清屏相关参数
	fbo_clear_color: &'s DynFboClearColorBindGroup,
	clear_color: &'s ClearColorBindGroup,
	clear_draw: &'s ClearDrawObj,
	common_sampler: &'s CommonSampler,

	copy_fbo: Option<&'s CopyFboToScreen>,
}

impl Pass2DNode {
	pub fn new(pass2d_id: Pass2DKey) -> Self {
		Self {
			pass2d_id,
			output_target: None,
			last_post_key: DefaultKey::default(),
			out: None,
			// param,
		}
	}
}

impl Node for Pass2DNode {
	type Output = Option<ShareTargetView>;

    fn run<'a>(
        &'a self,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<Self::Output, NodeRunError>> {
        let RenderContext { mut world, device, queue,.. } = context;
		
		let pass2d_id = self.pass2d_id;
        async move {
			let mut param = Param {
				pass2d_query: QueryState::<Pass2D,(&'static Camera, Option<&'static ViewMatrix>, &'static Draw2DList, Option<&'static NodeId>, &'static ParentPassId)>::new(&mut world),
				draw_query: QueryState::<DrawObject, &'static DrawState>::new(&mut world),
				post_query: QueryState::<Pass2D, &'static PostProcessList>::new(&mut world),
				last_rt: world.get_resource::<RenderTarget>().unwrap(),
				screen: world.get_resource::<ScreenTarget>().unwrap(),
				surface: world.get_resource::<ScreenTexture>().unwrap(),
				atlas_allocator: world.get_resource::<SafeAtlasAllocator>().unwrap(),
				t_type:world.get_resource::<DynTargetType>().unwrap(),
				buffer_assets: world.get_resource::<Share<AssetMgr<RenderRes<Buffer>>>>().unwrap(),
				bind_group_assets: world.get_resource::<Share<AssetMgr<RenderRes<BindGroup>>>>().unwrap(),
				post_bind_group_layout: world.get_resource::<PostBindGroupLayout>().unwrap(),
				share_layout: world.get_resource::<ShareLayout>().unwrap(),
				dyn_bind_groups: world.get_resource::<DynBindGroups>().unwrap(),
				postprocess_pipelines: world.get_resource::<PostProcessMaterialMgr>().unwrap(),
    			geometrys: world.get_resource::<PostProcessGeometryManager>().unwrap(),

				device: &device,
				queue: &queue,
				fbo_clear_color: world.get_resource::<DynFboClearColorBindGroup>().unwrap(),
				clear_color: world.get_resource::<ClearColorBindGroup>().unwrap(),
				clear_draw: world.get_resource::<ClearDrawObj>().unwrap(),
				common_sampler: world.get_resource::<CommonSampler>().unwrap(),

				copy_fbo: world.get_resource::<CopyFboToScreen>(),
			};

			let post_list = param.post_query.get(&world, self.pass2d_id);
			let mut out = None;

			if let Some((
				camera, 
				_view_matrix,
				list,
				node_id,
				parent_pass2d_id)) = param.pass2d_query.get(&world, pass2d_id) {
				
				let (rt, clear_color) = match post_list{
					None => if !parent_pass2d_id.is_null() {
							// 渲染目标类型为None， 并且存在父节点，不进行渲染（可能由父节点对它进行渲染）
							return Ok(None)
						} else {
							// 如果渲染目标类型类型为None，且不存在父节点，渲染到最终目标上，并且后处理列表长度为0，则不创建离屏的fbo
							(None, Some(&param.fbo_clear_color.0))
						},
					// 渲染类型为新建渲染目标对其进行渲染，则从纹理分配器中分配一个fbo矩形区
					Some(_) => (Some(param.atlas_allocator.allocate(
						(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
						(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
						param.t_type.has_depth,
						inputs.iter()
					)), Some(&param.fbo_clear_color.0)),
				};
				
				{
					// 创建一个渲染Pass
					let (mut rp, view_port) = self.create_rp(rt.as_ref(),
					commands.borrow_mut(),
					&camera.view_port,
					&param.last_rt,
					&param.screen,
					&param.surface,
					None
					);

					// 设置视口
					rp.set_viewport(
						view_port.0,
						view_port.1,
						view_port.2,
						view_port.3,
						0.0,
						1.0
					);
					// 清屏
					if let Some(clear_color) = clear_color {
						clear_color.draw(&mut rp, &param.dyn_bind_groups, ColorMaterialGroup::id());
						param.clear_draw.0.draw(&mut rp, &param.dyn_bind_groups); // 相机在drawObj中已经描述
					}
					
					
					// println!("pass_node1==========================opaque: {}, transparent:{}", list.opaque.len(), list.transparent.len());
					self.draw_list(&mut rp, &world, list, &mut param, camera, camera, &view_port, &view_port);
				}
				

				if let Some(post_process) = post_list {
					if let Some(r) = rt {
						let rect = r.rect().clone();
						// 渲染后处理
						if let Ok(r) = post_process.draw_front(
							
							param.device, 
							param.queue, 
							commands.borrow_mut(), 
							param.atlas_allocator, 
							param.postprocess_pipelines, 
							param.geometrys,
							EPostprocessTarget::from_share_target(r, wgpu::TextureFormat::Bgra8Unorm), 
							((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32)) {

							if let EPostprocessTarget::ShareTarget(r) = r {
								let r = r.view;
								let post_process_mut = unsafe {&mut *( post_process as *const PostProcessList as usize as *mut PostProcessList)};
								let data = Self::create_post_process_data(&r, &param, &camera.view_port, node_id);
								post_process_mut.cur_result = Some((r.clone(), data));
								out = Some(r);
							}
						};
						// let r = self.post_process(
						// 	commands.borrow_mut(),
						// 	r,
						// 	post_process,
						// 	param.t_type.no_depth,
						// 	camera,
						// 	&world,
						// 	&mut param,
						// 	);
						// 设置本次后处理结果，放入最后一个后处理中
						// 如果后处理长度为0，则无法放入（也不需要放入，长度为0表示根节点）
						// if post_process.0.len() > 0 {
							// 只会在本节点才会修改该post_process，除非存在两个相同pass2d_id的节点（应用逻辑应该保证不会重复）
							// let post_process_mut = unsafe {&mut *( post_process as *const PostProcessList as usize as *mut PostProcessList)};
							// let data = Self::create_post_process_data(&r.0, &param, &camera.view_port, node_id);
							// post_process_mut.curResult = ()
							// post_process_mut.0[r.1].result = Some(PostTemp {
							// 	target: r.0.clone(),
							// 	texture_group: data.0,
							// 	uv: data.1,
							// });
							// post_process_mut.1 = r.1;
						// } 
					}
				}

				// 处理根节点
				if parent_pass2d_id.is_null() {
					if let (Some(copy_fbo), RenderTarget::OffScreen(last_rt)) = (param.copy_fbo, param.last_rt) {
						let rect = last_rt.rect();
						// 将最终渲染目标渲染到屏幕上
						// 创建一个渲染Pass
						let (mut rp, view_port) = self.create_rp(
							None,
							commands.borrow_mut(),
							&Aabb2::new(
								Point2::new(rect.min.x as f32, rect.min.y as f32),
								Point2::new(rect.max.x as f32, rect.max.y as f32),
							),
							&RenderTarget::Screen,
							&param.screen,
							&param.surface,
							Some(
								wgpu::Operations {
									load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
									// load: wgpu::LoadOp::Load,
									store: true,
								}
							)
						);

						// 设置视口
						rp.set_viewport(
							view_port.0,
							view_port.1,
							view_port.2,
							view_port.3,
							0.0,
							1.0
						);

						copy_fbo.0.draw(&mut rp, &param.dyn_bind_groups);
					}
				}
			}

			Ok(out)
        }
        .boxed()
    }

    fn prepare<'a>(
        &'a self,
        _context: RenderContext,
    ) -> Option<BoxFuture<'a, Result<(), NodeRunError>>> {
        None
    }

    fn finish<'a>(
        &'a self,
        _context: RenderContext,
        _inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<(), NodeRunError>> {
        async { Ok(()) }.boxed()
    }
}

impl Pass2DNode {
	/// 渲染pass_2d(渲染列表中的一个渲染索引，如果是一个Pass2d， 则走该分支)
	/// * last_view_port-当前渲染目标的视口范围（）
	/// * last_camera-当前渲染目标的根相机（渲染过程是一个递归过程，每遇到一个Pass2d，当前相机会发生变化，当last_camera在递归过程保持不变）
	/// * cur_view_port-当前设置的视口
	/// * cur_camera-当前设置的相机
	pub fn render_pass_2d<'a>(
		&self,
		pass2d_id: Pass2DKey,

		rp: &mut RenderPass<'a>,
		world: &'a World,
		param: &'a Param<'a>,
		last_camera: &'a Camera,
		cur_camera: &'a Camera,
		last_view_port: &(f32, f32, f32, f32),
		cur_view_port: &(f32, f32, f32, f32),
	) {
		match param.post_query.get(world, pass2d_id) {
			Some(r) => {
				let src = r.cur_result.as_ref().unwrap();

				// let offset_x = last_view_port.0 as f32 - last_camera.view_port.mins.x;
				// let offset_y = last_view_port.1 as f32 - last_camera.view_port.mins.y;
				// let x = offset_x + r.view_port.mins.x.max(cur_camera.view_port.mins.x);
				// let y = offset_y + r.view_port.mins.y.max(cur_camera.view_port.mins.y);
				// // 设置视口到当前后处理在渲染目标上的位置(渲染目标本身相对其所在纹理也可能有一定的偏移位置，这里已将其考虑进去, 同时，需要与父视口求交)
				// rp.set_viewport(
				// 	x,
				// 	y,
				// 	(offset_x + r.view_port.maxs.x.min(cur_camera.view_port.maxs.x)) - x,
				// 	(offset_y + r.view_port.maxs.y.min(cur_camera.view_port.maxs.y)) - y, 
				// 	0.0, 
				// 	1.0);
				let matrix = &cur_camera.project * &r.matrix.0;

				if let Err(e) = r.draw_final(
					param.device, 
					param.queue, 
					param.postprocess_pipelines , 
					param.geometrys, 
				rp, 
				EPostprocessTarget::from_share_target(src.0.clone(), wgpu::TextureFormat::Bgra8Unorm),
				&src.1,
				&[wgpu::ColorTargetState {
					format: wgpu::TextureFormat::Bgra8Unorm,
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
				}], 
				&Some(wgpu::DepthStencilState {
					format: wgpu::TextureFormat::Depth32Float,
					depth_write_enabled: true,
					depth_compare: wgpu::CompareFunction::GreaterEqual,
					stencil: wgpu::StencilState::default(),
					bias: wgpu::DepthBiasState::default(),
				}), 
				matrix.as_slice(), 
				r.depth as f32/60000.0) {
					log::error!("draw_final fail, {:?} ", e);
				} 
				// // 还原视口
				// rp.set_viewport(
				// 	cur_view_port.0, 
				// 	cur_view_port.1, 
				// 	cur_view_port.2, 
				// 	cur_view_port.3, 
				// 	0.0,
				// 	1.0);
				// 如果存在后处理，则直接将后处理结果渲染出来(后处理结果)
				// if let Some(post) = r.0.get(r.1) {
				// 	if let Some(state) = param.draw_query.get(world, post.draw_obj_key) {
				// 		Self::draw_one_post_process(rp, state, post, last_camera, param);
				// 		// 释放握住的上次的渲染结果
				// 		let post_process_mut = unsafe {&mut *( post as *const PostProcess as usize as *mut PostProcess)};
				// 		post_process_mut.result = None;
				// 	}
				// }
			},
			None => {
				// 如果不存在后处理，则将pass2d中的所有渲染对象渲染到rp上
				if let Some((
					camera_new, 
					view_matrix,
					// rt_key, 
					list,
					node, 
					pass2d_id)) = param.pass2d_query.get(world, pass2d_id) {
					
					let v = (
						(last_view_port.0 as f32 - last_camera.view_port.mins.x) + camera_new.view_port.mins.x,
						(last_view_port.1 as f32 - last_camera.view_port.mins.y) + camera_new.view_port.mins.y,
						camera_new.view_port.maxs.x - camera_new.view_port.mins.x,
						camera_new.view_port.maxs.y - camera_new.view_port.mins.y,
					);


					rp.set_viewport(
						v.0, 
						v.1, 
						v.2, 
						v.3, 
						0.0, 
						1.0);

					// if let Some(view_matrix) = &camera_new.view_bind_group {
					// 	rp.set_bind_group(VIEW_GROUP as u32, view_matrix, &[])
					// }
					// if let Some(project_matrix) = &camera_new.project_bind_group {
					// 	rp.set_bind_group(PROJECT_GROUP as u32, project_matrix, &[])
					// }
					// // camera.vie
					// // 设置视图矩阵
					// if let Some(view_matrix) = view_matrix {
					// 	rp.set_bind_group(VIEW_GROUP as u32, view_matrix.bind_group.as_ref().unwrap(), &[]);
					// }

					self.draw_list(rp, world, list, param, last_camera, camera_new, last_view_port, cur_view_port);

					rp.set_viewport(
						cur_view_port.0, 
						cur_view_port.1, 
						cur_view_port.2, 
						cur_view_port.3, 
						0.0,
						1.0);
					if let Some(camera) = &cur_camera.bind_group {
						camera.draw(rp, param.dyn_bind_groups, CameraMatrixGroup::id());
					}
				}
			},
		}
	}
	// /// 对除最后一个后处理以外的其他后处理进行渲染, 返回倒数第二个后处理的结果(ShareTargetView), 如果后处理列表的长度是0，则返回输入(ShareTargetView)
	// pub fn post_process<'a>(
	// 	&self,
	// 	commands: &'a mut CommandEncoder,
	// 	input: ShareTargetView,
	// 	post_process: &PostProcessList,
	// 	t_type: TargetType,
	// 	camera: &'a Camera,
	// 	world: &'a World,

	// 	param: &'a Param<'a>,
	// 	// atlas_allocator: &SafeAtlasAllocator,
	// 	// last_rt: &RenderTarget,
	// 	// draw_query: &QueryState<DrawObject, &DrawState>,
	// 	// world: &World,
	// 	// surface: &TextureView,
	// ) -> (ShareTargetView, DefaultKey) {
	// 	let len = post_process.0.len();
	// 	let mut i = 0;
	// 	let mut cur_rt = input;
	// 	for (k, v) in post_process.0.iter() {
	// 		i += 1;

	// 		// 最后一个后处理不执行，交给下一个节点渲染
	// 		if i == len {
	// 			return (cur_rt, k);
	// 		}

	// 		// 分配一个rendertarget，用于渲染后处理内容
	// 		let target = param.atlas_allocator.allocate(
	// 			(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
	// 			(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
	// 			t_type,
	// 			[cur_rt.clone()].iter()
	// 		);

	// 		// 渲染后处理到target上
	// 		self.render_post_poss(commands, v, Some(&target), camera, world, param);

	// 		// 设置当前rt为当前后处理的处理结果
	// 		cur_rt = target;
	// 	}
	// 	(cur_rt, DefaultKey::default())

	// }

	// /// 渲染后处理
	// pub fn render_post_poss<'a>(
	// 	&self,
	// 	commands: &'a mut CommandEncoder,
	// 	post_process: &PostProcess,
	// 	rt: Option<&'a ShareTargetView>, // 渲染目标，如果渲染目标不存在，则渲染到最终目标上
	// 	camera: &'a Camera,
	// 	world: &'a World,

	// 	param: &'a Param<'a>,
	// ) {
	// 	if let Some(state) = param.draw_query.get(&world, post_process.draw_obj_key) {
	// 		{
	// 			let (mut rp, view_port) = self.create_rp(rt, commands, &camera.view_port, param.last_rt, param.screen, param.surface, None);
	// 			rp.set_viewport(
	// 				view_port.0,
	// 				view_port.1,
	// 				view_port.2,
	// 				view_port.3,
	// 				0.0,
	// 				1.0
	// 			);
	// 			// 清屏
	// 			param.fbo_clear_color.0.draw(&mut rp, &param.dyn_bind_groups, ColorMaterialGroup::id());
	// 			param.clear_draw.0.draw(&mut rp, &param.dyn_bind_groups); // 相机在drawObj中已经描述

	// 			Self::draw_one_post_process(&mut rp, state, post_process, camera, param);
	// 		}
			
	// 		// 释放握住的上次的渲染结果
	// 		let post_process_mut = unsafe {&mut *( post_process as *const PostProcess as usize as *mut PostProcess)};
	// 		post_process_mut.result = None;
	// 	}
	// }

	pub fn create_rp<'a>(
		&self,
		rt: Option<&'a ShareTargetView>,
		commands: &'a mut CommandEncoder,
		view_port: &Aabb2,
		last_rt: &'a RenderTarget,
		screen: &'a ScreenTarget,
		surface: &'a ScreenTexture,
		ops: Option<wgpu::Operations<wgpu::Color>>,
	) -> (RenderPass<'a>, (f32, f32, f32, f32)) {
		let ops = match ops {
			Some(r) => r,
			None => wgpu::Operations {
				// load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
				load: wgpu::LoadOp::Load,
				store: true,
			}
		};
		match (rt, last_rt) {
			(Some(r), _) | (None, RenderTarget::OffScreen(r)) => {
				let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: r.target().colors
						.iter()
						.map(|view| {
							wgpu::RenderPassColorAttachment {
								resolve_target: None,
								ops,
								view: &view.0,
							}
						})
						.collect::<Vec<wgpu::RenderPassColorAttachment>>().as_slice(),
					depth_stencil_attachment: match &r.target().depth {
						Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
							stencil_ops: None,
							depth_ops: Some(wgpu::Operations {
								load: wgpu::LoadOp::Load,
								store: true,
							}),
							view: &r.0,
						}),
						None => None,
					},
				});
				let rect = r.rect();
				(rp, (
					rect.min.x as f32,
					rect.min.y as f32,
					view_port.maxs.x - view_port.mins.x,
					view_port.maxs.y - view_port.mins.y,
				))
			},
			(None, RenderTarget::Screen) => {
				let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: &[wgpu::RenderPassColorAttachment {
						resolve_target: None,
						ops,
						view: surface.view.as_ref().unwrap(),
					}],
					depth_stencil_attachment: match &screen.depth {
						Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
							stencil_ops: None,
							depth_ops: Some(wgpu::Operations {
								load: wgpu::LoadOp::Load,
								store: true,
							}),
							view: r,
						}),
						None => None,
					},
				});
				(rp, (
					view_port.mins.x,
					view_port.mins.y,
					view_port.maxs.x - view_port.mins.x,
					view_port.maxs.y - view_port.mins.y,
				))
			},
		}
	}

	fn draw_list<'a, 'w>(
		&self,
		rp: &'w mut RenderPass<'a>,
		world: &'a World,
		list: &Draw2DList,

		param: &'a Param<'a>,
		last_camera: &'a Camera,
		cur_camera: &'a Camera,
		last_view_port: &(f32, f32, f32, f32),
		cur_view_port: &(f32, f32, f32, f32),
	) {
		if let Some(camera) = &cur_camera.bind_group {
			camera.draw(rp, &param.dyn_bind_groups, CameraMatrixGroup::id());
		}
		
		for e in list.opaque.iter().chain(list.transparent.iter()) {
			match e {
				DrawIndex::DrawObj(e) => {// 设置相机
					if let Some(state) = param.draw_query.get(world, *e) {
						if state.bind_groups.get_group(CameraMatrixGroup::id()).is_none() {
							if let Some(r) = &cur_camera.bind_group {
								r.draw(rp, &param.dyn_bind_groups, CameraMatrixGroup::id());
							}
						}
						state.draw(rp, &param.dyn_bind_groups );
					}
				},
				DrawIndex::Pass2D(e) => {
					self.render_pass_2d(*e, rp, world, param, last_camera, cur_camera, last_view_port, cur_view_port);
				},
			}
		}
	}

	// fn draw_one_post_process<'a>(
	// 	rp: &mut RenderPass<'a>,
	// 	state: &'a DrawState,
	// 	post_process: &'a PostProcess,
	// 	camera: &'a Camera, // TODO 可能不是相机， 需要考虑TransformWillChange
	// 	param: &'a Param<'a>,
	// ) {
	// 	// 后处理的投影矩阵设置， TODO
	// 	if let Some(PostTemp{texture_group, uv, ..}) = &post_process.result {
	// 		// rp.set_bind_group(WORLD_MATRIX_GROUP as u32, matrix, &[]);
	// 		rp.set_bind_group(post_process.texture_bind_index as u32, texture_group, &[]);
	// 		rp.set_vertex_buffer(post_process.uv_vb_index as u32, (*****uv).slice(..));
	// 		state.draw(rp, &param.dyn_bind_groups);
	// 	}
		
	// }

	// 创建后处理数据（bindgroup和uv buffer）
	fn create_post_process_data<'s>(
		texture: &ShareTargetView,
		param: &'s Param<'s>,
		render_rect: &Aabb2,
		node: Option<&NodeId>,
	) -> Handle<RenderRes<BindGroup>> {
		// let uv = texture.uv();
		let group_key = calc_hash(&(texture.ty_index(), texture.target_index()), calc_hash(&"render target", 0)); // TODO
		// let buffer_key = calc_float_hash(&uv, calc_hash(&"vert", 0));
		// let matrix = Matrix4::new(
		// 	render_rect.maxs.x-render_rect.mins.x,0.0,0.0, render_rect.mins.x,
		// 	0.0,render_rect.maxs.y-render_rect.mins.y,0.0, render_rect.mins.y,
		// 	0.0,0.0,1.0, node.map_or_else(|| {0}, |node| {node.offset()}) as f32,
		// 	0.0,0.0,0.0,1.0,
		// );
		match param.bind_group_assets.get(&group_key) {
			Some(r) => r,
			None => {
				let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
					layout: param.post_bind_group_layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::Sampler(&param.common_sampler.pointer),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0),
						},
					],
					label: Some("post process texture bind group create"),
				});
				param.bind_group_assets.insert(group_key, RenderRes::new(group.clone(), 5)).unwrap()
			},
		}
	}

	// // 创建后处理数据（bindgroup和uv buffer）
	// fn create_post_process_data<'s>(
	// 	texture: &ShareTargetView,
	// 	param: &'s Param<'s>,
	// 	render_rect: &Aabb2,
	// 	node: Option<&NodeId>,
	// ) -> (Handle<RenderRes<BindGroup>>, Handle<RenderRes<Buffer>>) {
	// 	let uv = texture.uv();
	// 	let group_key = calc_hash(&(texture.ty_index(), texture.target_index()), calc_hash(&"render target", 0)); // TODO
	// 	let buffer_key = calc_float_hash(&uv, calc_hash(&"vert", 0));
	// 	// let matrix = Matrix4::new(
	// 	// 	render_rect.maxs.x-render_rect.mins.x,0.0,0.0, render_rect.mins.x,
	// 	// 	0.0,render_rect.maxs.y-render_rect.mins.y,0.0, render_rect.mins.y,
	// 	// 	0.0,0.0,1.0, node.map_or_else(|| {0}, |node| {node.offset()}) as f32,
	// 	// 	0.0,0.0,0.0,1.0,
	// 	// );
	// 	(
	// 		match param.bind_group_assets.get(&group_key) {
	// 			Some(r) => r,
	// 			None => {
	// 				let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
	// 					layout: param.post_bind_group_layout,
	// 					entries: &[
	// 						wgpu::BindGroupEntry {
	// 							binding: 0,
	// 							resource: wgpu::BindingResource::Sampler(&param.common_sampler.pointer),
	// 						},
	// 						wgpu::BindGroupEntry {
	// 							binding: 1,
	// 							resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0),
	// 						},
	// 					],
	// 					label: Some("post process texture bind group create"),
	// 				});
	// 				param.bind_group_assets.insert(group_key, RenderRes::new(group.clone(), 5)).unwrap()
	// 			},
	// 		},
	// 		match param.buffer_assets.get(&buffer_key) {
	// 			Some(r) => r,
	// 			None => {
	// 				let uv_buf = param.device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
	// 					label: Some("post process uv Buffer"),
	// 					contents: bytemuck::cast_slice(&uv),
	// 					usage: wgpu::BufferUsages::VERTEX,
	// 				});
	// 				param.buffer_assets.insert(buffer_key, RenderRes::new(uv_buf, 32)).unwrap()
	// 			}
	// 		},
	// 		// create_world_matrix_bind(param.device, param.queue, &param.share_layout.matrix, param.buffer_assets, param.bind_group_assets)
	// 	)
	// }
	
}

#[derive(Deref)]
pub struct PostBindGroupLayout(pub BindGroupLayout);

impl FromWorld for PostBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
		let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("post_process_texture_layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Texture {
						multisampled: false,
                        sample_type: wgpu::TextureSampleType::default(),
                        view_dimension: wgpu::TextureViewDimension::D2,
					},
					count: None,
				},
			],
		});
		Self(layout)
	}
}


/// 渲染目标类型（有深度缓冲区和无深度缓冲区两种，rgba格式）
/// 后处理通常使用无深度缓冲区的渲染目标
/// 普通节点渲染使用有深度缓冲器
pub struct DynTargetType {
	pub has_depth: TargetType,
	pub no_depth: TargetType,
}

/// 创建图节点所需要的数据
/// 如： DynTargetType (需要根据视口变化及时调整)
pub struct InitGraphData;
// use crate::components::user::Node;
#[setup]
impl InitGraphData{

	#[listen(resource=(Viewport, (Modify, Create)))]
	pub fn calc_dyn_target_type(
		_e: Event,
		atlas_allocator: Res<SafeAtlasAllocator>,
		view_port: Res<Viewport>,
		mut dyn_target_type: WriteRes<DynTargetType>,
	) {
		let ty = Self::create_dyn_target_type(&atlas_allocator, &view_port);
		dyn_target_type.write(ty);

	}

	pub fn create_dyn_target_type(
		atlas_allocator: &SafeAtlasAllocator,
		view_port: &Viewport,
	) -> DynTargetType {
		DynTargetType{
			has_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
				texture_descriptor: SmallVec::from_slice(&[TextureDescriptor {
					mip_level_count: 1,
					sample_count: 1,
					dimension: wgpu::TextureDimension::D2,
					format: wgpu::TextureFormat::Bgra8Unorm,
					usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
					base_mip_level: 0,
					base_array_layer: 0,
					array_layer_count: None,
					view_dimension: None,
				}]),
				need_depth: true,
				default_width: (view_port.maxs.x - view_port.mins.x).ceil() as u32,
				default_height: (view_port.maxs.y - view_port.mins.y).ceil() as u32,
			}),
			no_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
				texture_descriptor: SmallVec::from_slice(&[TextureDescriptor {
					mip_level_count: 1,
					sample_count: 1,
					dimension: wgpu::TextureDimension::D2,
					format: wgpu::TextureFormat::Bgra8Unorm,
					usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
					base_mip_level: 0,
					base_array_layer: 0,
					array_layer_count: None,
					view_dimension: None,
				}]),
				need_depth: false,
				default_width: (view_port.maxs.x - view_port.mins.x).ceil() as u32,
				default_height: (view_port.maxs.y - view_port.mins.y).ceil() as u32,
			})
		}
	}
}

// device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
// 	label: Some("color_layout"),
// 	entries: &[
// 		wgpu::BindGroupLayoutEntry {
// 			binding: 0,
// 			visibility: wgpu::ShaderStages::FRAGMENT,
// 			ty: wgpu::BindingType::Buffer {
// 				ty: wgpu::BufferBindingType::Uniform,
// 				has_dynamic_offset: false,
// 				min_binding_size: wgpu::BufferSize::new(16), // rgba四个通道，每个通道为一个f32, 大小为 4 * 4（每个通道一个u8， todo）
// 			},
// 			count: None,
// 		},
// 	],
// })
