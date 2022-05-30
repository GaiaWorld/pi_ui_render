use std::{sync::Arc, borrow::BorrowMut};

use pi_assets::{mgr::{AssetMgr, LoadResult}, asset::Handle};
use pi_ecs_macros::{listen, setup};
use pi_render::{
    components::view::{target::{RenderTargets, TextureViews}, target_alloc::{ShareTargetView, SafeAtlasAllocator, TargetType, TargetDescriptor, GetTargetView, TargetView, TextureDescriptor}},
    graph::{
        node::{Node, NodeRunError},
        RenderContext,
    },
    rhi::{CommandEncoder, bind_group_layout::BindGroupLayout, device::RenderDevice, asset::RenderRes, bind_group::BindGroup, buffer::Buffer, texture::ScreenTexture},
};
use futures::{future::BoxFuture, FutureExt};
use pi_ecs::{prelude::{QueryState, FromWorld, World, ResMut, Res, res::{WriteRes, ResState}, Query, SystemState, SystemParamState, DataState}, monitor::Event};
use pi_share::{ShareRefCell, Share, cell::TrustCell};
use pi_slotmap::{DefaultKey, SlotMap};
use smallvec::SmallVec;
use wgpu::{RenderPass, TextureView};

use crate::{components::{draw_obj::{DrawObject, DrawState}, pass_2d::{Camera, Draw2DList, Pass2DKey, Pass2D, PostProcessList, RenderTargetType, PostProcess, RenderTarget, DrawIndex}}, resource::{draw_obj::RenderInfo, Viewport}, utils::tools::{calc_hash, calc_float_hash}};


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
	pass2d_query: QueryState<Pass2D,(&'static Camera, &'static Draw2DList, &'static RenderTargetType)>,
	draw_query: QueryState<DrawObject, &'static DrawState>,
	post_query: QueryState<Pass2D, &'static PostProcessList>,
	last_rt: &'s RenderTarget,
	surface: &'s ScreenTexture,
	atlas_allocator: &'s SafeAtlasAllocator,
	t_type: &'s DynTargetType,
	buffer_assets: &'s Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'s Share<AssetMgr<RenderRes<BindGroup>>>,
	device: &'s RenderDevice,
	post_bind_group_layout: &'s PostBindGroupLayout,
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
		println!("pass_node==========================");
        let RenderContext { mut world, device,.. } = context;

        // let pass_query = QueryState::<Pass2D,(&'static Camera,&'static Draw2DList,&'static RenderTargetType),
        // >::new(&mut world);

        // let draw_query = QueryState::<DrawObject, &'static DrawState>::new(&mut world);
		// let post_query = QueryState::<Pass2D, &'static PostProcessList>::new(&mut world);
		
		let pass2d_id = self.pass2d_id;
        async move {
			let mut param = Param {
				pass2d_query: QueryState::<Pass2D,(&'static Camera,&'static Draw2DList,&'static RenderTargetType)>::new(&mut world),
				draw_query: QueryState::<DrawObject, &'static DrawState>::new(&mut world),
				post_query: QueryState::<Pass2D, &'static PostProcessList>::new(&mut world),
				last_rt: world.get_resource::<RenderTarget>().unwrap(),
				surface: world.get_resource::<ScreenTexture>().unwrap(),
				atlas_allocator: world.get_resource::<SafeAtlasAllocator>().unwrap(),
				t_type:world.get_resource::<DynTargetType>().unwrap(),
				buffer_assets: world.get_resource::<Share<AssetMgr<RenderRes<Buffer>>>>().unwrap(),
				bind_group_assets: world.get_resource::<Share<AssetMgr<RenderRes<BindGroup>>>>().unwrap(),
				post_bind_group_layout: world.get_resource::<PostBindGroupLayout>().unwrap(),
				device: &device,
			};

			let post_list = param.post_query.get(&world, self.pass2d_id);
			let post_list_len = match post_list {
				Some(r) => r.0.len(),
				None => 0
			};

			let mut out = None;
			
			if let Some((
				camera, 
				// rt_key, 
				list,
				render_target_ty)) = param.pass2d_query.get(&world, pass2d_id) {
				
				let mut out = None;
				
				let mut rt = match *render_target_ty {
					// 如果渲染目标类型类型为，渲染到最终目标上，并且后处理列表长度为0，则不创建离屏的fbo
					RenderTargetType::Last if post_list_len == 0 => None,
					// 渲染目标类型为None，表示不进行渲染（可能由父节点对它进行渲染）
					RenderTargetType::None => return Ok(None), 
					// RenderTargetType::New || post_list_len > 0
					// 渲染类型为新建渲染目标对其进行渲染，则从纹理分配器中分配一个fbo矩形区
					_ => Some(param.atlas_allocator.allocate(
						(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
						(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
						param.t_type.has_depth,
						inputs.iter()
					)),
				};
				
				{
						// 创建一个渲染Pass
					let (mut rp, view_port) = self.create_rp(rt.as_ref(),
					commands.borrow_mut(),
					camera,
					&param.last_rt,
					&param.surface,
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

					// println!("pass_node1==========================opaque: {}, transparent:{}", list.opaque.len(), list.transparent.len());
					self.draw_list(pass2d_id, &mut rp, &world, camera, list, &mut param);
				}
				

				if let (Some(r), Some(post_process)) = (rt, post_list) {
					// 渲染后处理
					let r = self.post_process(
						commands.borrow_mut(),
						r,
						post_process,
						param.t_type.no_depth,
						camera,
						&world,
						&mut param,
						);
					// 设置本次后处理结果，放入最后一个后处理中
					// 如果后处理长度为0，则无法放入（也不需要放入，长度为0表示根节点）
					if post_process.0.len() > 0 {
						// 只会在本节点才会修改该post_process，除非存在两个相同pass2d_id的节点（应用逻辑应该保证不会重复）
						let post_process_mut = unsafe {&mut *( post_process as *const PostProcessList as usize as *mut PostProcessList)};
						let data = Self::create_post_process_data(&r.0, &param);
						post_process_mut.0[r.1].target = Some((r.0.clone(), data.0, data.1));
						post_process_mut.1 = r.1;
					}
					

					out = Some(r.0);
				}
			}

			Ok(out)
        }
        .boxed()
    }
}

impl Pass2DNode {
	/// 渲染pass_2d(渲染列表中的一个渲染索引，如果是一个Pass2d， 则走该分支)
	pub fn render_pass_2d<'a>(
		&self,
		pass2d_id: Pass2DKey,
		rp: &mut RenderPass<'a>,
		world: &'a World,
		camera: &'a Camera,
		param: &'a Param<'a>,
	) {
		match param.post_query.get(world, pass2d_id) {
			Some(r) => {
				// 如果存在后处理，则直接将后处理结果渲染出来(后处理结果)
				if let Some(post) = r.0.get(r.1) {
					if let Some(state) = param.draw_query.get(world, post.draw_obj_key) {
						Self::draw_one_post_process(rp, state, post, camera, param);
					}
				}
			},
			None => {
				// 如果不存在后处理，则将pass2d中的所有渲染对象渲染到rp上
				if let Some((
					camera, 
					// rt_key, 
					list,
					render_target_ty)) = param.pass2d_query.get(&world, pass2d_id) {
					// todo， 重新计算相机
					self.draw_list(pass2d_id, rp, world, camera, list, param);
				}
			},
		}
	}
	/// 对除最后一个后处理以外的其他后处理进行渲染, 返回倒数第二个后处理的结果(ShareTargetView), 如果后处理列表的长度是0，则返回输入(ShareTargetView)
	pub fn post_process<'a>(
		&self,
		commands: &'a mut CommandEncoder,
		input: ShareTargetView,
		post_process: &PostProcessList,
		t_type: TargetType,
		camera: &'a Camera,
		world: &'a World,

		param: &'a Param<'a>,
		// atlas_allocator: &SafeAtlasAllocator,
		// last_rt: &RenderTarget,
		// draw_query: &QueryState<DrawObject, &DrawState>,
		// world: &World,
		// surface: &TextureView,
	) -> (ShareTargetView, DefaultKey) {
		let len = post_process.0.len();
		let mut i = 0;
		let mut cur_rt = input;
		for (k, v) in post_process.0.iter() {
			i += 1;

			// 最后一个后处理不执行，交给下一个节点渲染
			if i == len {
				return (cur_rt, k);
			}

			// 分配一个rendertarget，用于渲染后处理内容
			let target = param.atlas_allocator.allocate(
				v.width,
				v.height,
				t_type,
				[cur_rt.clone()].iter()
			);

			// 渲染后处理到target上
			self.render_post_poss(commands, v, Some(&target), camera, world, param);

			// 设置当前rt为当前后处理的处理结果
			cur_rt = target;
		}
		(cur_rt, DefaultKey::default())

	}

	/// 渲染后处理
	pub fn render_post_poss<'a>(
		&self,
		mut commands: &'a mut CommandEncoder,
		post_process: &PostProcess,
		rt: Option<&'a ShareTargetView>, // 渲染目标，如果渲染目标不存在，则渲染到最终目标上
		camera: &'a Camera,
		world: &'a World,

		param: &'a Param<'a>,
	) {
		if let Some(state) = param.draw_query.get(&world, post_process.draw_obj_key) {
			let (mut rp, view_port) = self.create_rp(rt, commands, camera, param.last_rt, param.surface);
			rp.set_viewport(
				view_port.0,
				view_port.1,
				view_port.2,
				view_port.3,
				0.0,
				1.0
			);

			Self::draw_one_post_process(&mut rp, state, post_process, camera, param);
		}
	}

	pub fn create_rp<'a>(
		&self,
		rt: Option<&'a ShareTargetView>,
		mut commands: &'a mut CommandEncoder,
		camera: &Camera,
		last_rt: &'a RenderTarget,
		surface: &'a ScreenTexture,
	) -> (RenderPass<'a>, (f32, f32, f32, f32)) {
		match (rt, last_rt) {
			(Some(r), _) | (None, RenderTarget::OffScreen(r)) => {
				let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: r.target().colors
						.iter()
						.map(|view| {
							wgpu::RenderPassColorAttachment {
								resolve_target: None,
								ops: wgpu::Operations {
									load: wgpu::LoadOp::Load,
									store: true,
								},
								view: view,
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
							view: r,
						}),
						None => None,
					},
				});
				let rect = r.rect();
				(rp, (
					rect.min.x as f32,
					rect.min.y as f32,
					camera.view_port.maxs.x - camera.view_port.mins.x,
					camera.view_port.maxs.y - camera.view_port.mins.y,
				))
			},
			(None, RenderTarget::Screen{depth, aabb}) => {
				let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: &[wgpu::RenderPassColorAttachment {
						resolve_target: None,
						ops: wgpu::Operations {
							load: wgpu::LoadOp::Load,
							store: true,
						},
						view: surface.view.as_ref().unwrap(),
					}],
					depth_stencil_attachment: match depth {
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
					camera.view_port.mins.x,
					camera.view_port.mins.y,
					camera.view_port.maxs.x - camera.view_port.mins.x,
					camera.view_port.maxs.y - camera.view_port.mins.y,
				))
			},
		}
	}

	fn draw_list<'a, 'w>(
		&self,
		pass2d_id: Pass2DKey,
		rp: &'w mut RenderPass<'a>,
		world: &'a World,
		camera: &'a Camera,
		list: &Draw2DList,

		param: &'a Param<'a>,
	) {

		for e in list.opaque.iter().chain(list.transparent.iter()) {
			match e {
				DrawIndex::DrawObj(e) => {
					if let Some(state) = param.draw_query.get(world, *e) {
						println!("draw=====================");
						state.draw(rp, camera);
					}
				},
				DrawIndex::Pass2D(e) => self.render_pass_2d(*e, rp, world, camera, param),
			}
		}
	}

	fn draw_one_post_process<'a>(
		rp: &mut RenderPass<'a>,
		state: &'a DrawState,
		post_process: &'a PostProcess,
		camera: &'a Camera, // TODO 可能不是相机， 需要考虑TransformWillChange
		param: &'a Param<'a>,
	) {
		if let Some((target, bind_group, buffer)) = &post_process.target {
			// let r = Self::create_post_process_data(target, param, post_cache);
			// TODO
			rp.set_bind_group(post_process.texture_bind_index as u32, bind_group, &[]);
			rp.set_vertex_buffer(post_process.uv_vb_index as u32, (*****buffer).slice(..));
			state.draw(rp, camera);
		}
		
	}

	// 创建后处理数据（bindgroup和uv buffer）
	fn create_post_process_data<'s>(
		texture: &ShareTargetView,
		param: &'s Param<'s>,
	) -> (Handle<RenderRes<BindGroup>>, Handle<RenderRes<Buffer>>) {
		let uv = texture.uv();
		let group_key = calc_hash(&(texture.ty_index(), texture.target_index())); // TODO
		let buffer_key = calc_float_hash(&uv);
		(
			match param.bind_group_assets.get(&group_key) {
				Some(r) => r,
				None => {
					let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
						layout: param.post_bind_group_layout,
						entries: &[
							wgpu::BindGroupEntry {
								binding: 0,
								resource: wgpu::BindingResource::TextureView(&texture.target().colors[0]),
							},
						],
						label: Some("post process texture bind group create"),
					});
					param.bind_group_assets.cache(group_key, RenderRes::new(group, 5));
					param.bind_group_assets.get(&group_key).unwrap()
				},
			},
			match param.buffer_assets.get(&buffer_key) {
				Some(r) => r,
				None => {
					let uv_buf = param.device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("post process uv Buffer"),
						contents: bytemuck::cast_slice(&uv),
						usage: wgpu::BufferUsages::VERTEX,
					});
					param.buffer_assets.cache(buffer_key, RenderRes::new(uv_buf, 32));
					param.buffer_assets.get(&buffer_key).unwrap()
				},
			}
		)
	}
	
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
		e: Event,
		atlas_allocator: Res<SafeAtlasAllocator>,
		view_port: Res<Viewport>,
		mut dyn_target_type: WriteRes<DynTargetType>,
	) {
		let ty = DynTargetType{
			has_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
				texture_descriptor: SmallVec::from_slice(&[TextureDescriptor {
					mip_level_count: 1,
					sample_count: 1,
					dimension: wgpu::TextureDimension::D2,
					format: wgpu::TextureFormat::Rgba8Unorm,
					usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
					base_mip_level: 1,
					base_array_layer: 1,
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
					format: wgpu::TextureFormat::Rgba8Unorm,
					usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
					base_mip_level: 1,
					base_array_layer: 1,
					array_layer_count: None,
					view_dimension: None,
				}]),
				need_depth: false,
				default_width: (view_port.maxs.x - view_port.mins.x).ceil() as u32,
				default_height: (view_port.maxs.y - view_port.mins.y).ceil() as u32,
			})
		};
		dyn_target_type.write(ty);

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
