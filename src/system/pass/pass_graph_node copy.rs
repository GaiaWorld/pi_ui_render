// use std::borrow::BorrowMut;

// use bevy_ecs::{
//     prelude::Entity,
//     query::With,
//     system::{Query, Res, SystemParam, SystemState},
//     world::World,
// };
// use pi_assets::asset::Handle;
// use pi_bevy_asset::ShareAssetMgr;
// use pi_bevy_ecs_extend::{
//     prelude::{Layer, OrDefault},
//     system_param::res::OrInitRes,
// };
// use pi_bevy_post_process::PostprocessResource;
// use pi_bevy_render_plugin::{
//     node::{Node, NodeId as GraphNodeId, ParamUsage},
//     param::InParamCollector,
//     PiSafeAtlasAllocator,
//     PiScreenTexture,
//     // param::P
//     RenderContext,
//     SimpleInOut, PiRenderDevice, PiRenderQueue,
// };
// use pi_futures::BoxFuture;
// use pi_hash::XHashMap;
// use pi_null::Null;
// use pi_render::{rhi::shader::Input, components::view::target_alloc::{SafeTargetView, SafeAtlasAllocator}};
// // use pi_postprocess::
// use pi_postprocess::{prelude::PostprocessTexture, image_effect::PostProcessDraw};
// use pi_render::{
//     components::view::target_alloc::ShareTargetView,
//     renderer::{draw_obj::DrawObj, texture::texture_view::ETextureViewUsage},
//     rhi::{
//         asset::RenderRes,
//         bind_group::BindGroup,
//         buffer::Buffer,
//         device::RenderDevice,
//         pipeline::RenderPipeline,
//         shader::BindLayout,
//         texture::{PiRenderDefault, ScreenTexture},
//         CommandEncoder, RenderQueue,
//     },
// };
// use pi_share::{ShareRefCell, Share};
// use pi_style::style::AsImage as AsImage1;
// use wgpu::{RenderPass, Sampler};

// use crate::{
//     components::{
//         calc::{EntityKey, NodeId, Quad},
//         draw_obj::{ClearColorBindGroup, DrawState, DynTargetType},
//         pass_2d::{Camera, Draw2DList, DrawIndex, GraphId, ParentPassId, PostProcess, PostProcessInfo, RenderTarget, ScreenTarget, StrongTarget, CacheTarget, RenderTargetCache},
//         user::{Aabb2, AsImage, Point2, RenderTargetType},
//     },
//     resource::{
//         draw_obj::{ClearDrawObj, CommonSampler, DepthCache, DynFboClearColorBindGroup, PostBindGroupLayout, TargetCacheMgr},
//         RenderContextMarkType, PassGraphMap,
//     },
//     shader::{
//         camera::CameraBind,
//         depth::DepthBind,
//         image::{SampBind, UvVert},
//         ui_meterial::UiMaterialBind,
//     },
// };


// /// Pass2D 渲染图节点
// // #[derive(Clone)]
// pub struct Pass2DNode {
//     // // 输入描述
//     // input: Vec<SlotInfo>,
//     // // 输出描述
//     // output: Vec<SlotInfo>,
//     pub pass2d_id: Entity,
//     // pub output_target: Option<ShareTargetView>,
//     // pub last_post_key: DefaultKey,
//     pub rt: Option<RenderPassTarget>,
// 	pub post_draw: Option<(Vec<PostProcessDraw>, ShareTargetView)>,
//     // node_id_query: QueryState<&'static NodeId, With<Camera>>,
// }

// #[derive(SystemParam)]
// pub struct BuildParam<'w, 's> {
// 	pass2d_query: Query<
// 		'w,
// 		's,
// 		(
// 			&'static Layer,
// 			&'static Camera,
// 			&'static Draw2DList,
// 			&'static ParentPassId,
// 			Option<&'static ClearColorBindGroup>,
// 			&'static RenderTarget,
// 			Option<&'static AsImage>,
// 			&'static mut PostProcess, 
// 			&'static PostProcessInfo, 
// 		),
// 	>,
// 	query_pass_node: Query<
//         'w,
//         's,
//         (
//             &'static DynTargetType,
//             OrDefault<RenderTargetType>,
//         ),
//     >,
// 	post_resource: Res<'w, PostprocessResource>,
//     pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,
// 	atlas_allocator: Res<'w, PiSafeAtlasAllocator>,
// 	device: Res<'w, PiRenderDevice>,
// 	queue: Res<'w, PiRenderQueue>,
// 	surface: Res<'w, PiScreenTexture>,
// 	pass_graph_map: Res<'w, PassGraphMap>,
// 	render_targets: Query<'w, 's, &'static RenderTarget>,
// 	cache_target: Res<'w, TargetCacheMgr>,
// 	as_image_mark_type: OrInitRes<'w, RenderContextMarkType<AsImage>>,
// }

// #[derive(SystemParam)]
// pub struct QueryParam<'w, 's> {
//     query_pass_node: Query<
//         'w,
//         's,
//         (
//             &'static DynTargetType,
//             // Option<&'static ClearColorBindGroup>,
//             // &'static RenderTarget,
//             OrDefault<RenderTargetType>,
//             // Option<&'static CopyFboToScreen>,
//         ),
//     >,
//     pass2d_query: (
//         Query<'w, 's, &'static Layer, With<Camera>>,
//         Query<
//             'w,
//             's,
//             (
//                 &'static Camera,
//                 &'static Draw2DList,
//                 &'static ParentPassId,
//                 Option<&'static ClearColorBindGroup>,
//                 &'static RenderTarget,
//             ),
//         >,
//         Query<'w, 's, (&'static PostProcess, &'static PostProcessInfo, &'static GraphId, Option<&'static AsImage>)>,
//         Query<'w, 's, (&'static PostProcess, &'static GraphId, &'static NodeId)>,
//         Query<'w, 's, &'static PostProcessInfo>,
//     ),
//     draw_query: Query<'w, 's, (&'static DrawState, &'static NodeId, Option<&'static GraphId>)>,
// 	graph_node_query: Query<'w, 's, &'static GraphId>,
//     node_query: Query<'w, 's, &'static Quad>,
//     // graph_id_query: Query<'w, 's, &'static GraphId>,
//     screen: Res<'w, ScreenTarget>,
//     surface: Res<'w, PiScreenTexture>,
//     atlas_allocator: Res<'w, PiSafeAtlasAllocator>,
//     // bind_group_assets: Res<'w, ShareAssetMgr<RenderRes<BindGroup>>>,
//     post_bind_group_layout: OrInitRes<'w, PostBindGroupLayout>,
//     // postprocess_pipelines: Res<'w, My PiPostProcessMaterialMgr>,
//     post_resource: Res<'w, PostprocessResource>,
//     pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,

//     // 清屏相关参数
//     fbo_clear_color: Res<'w, DynFboClearColorBindGroup>,
//     clear_draw: Res<'w, ClearDrawObj>,
//     common_sampler: Res<'w, CommonSampler>,

//     depth_cache: OrInitRes<'w, DepthCache>,
// 	#[cfg(debug_assertions)]
// 	debug_entity: OrInitRes<'w, crate::resource::DebugEntity>,
// }

// // vballocator: &mut VertexBufferAllocator,
// // safeatlas: &SafeAtlasAllocator,
// // resources: &SingleImageEffectResource,
// // pipelines: &Share<AssetMgr<RenderRes<RenderPipeline>>>,


// pub struct Param<'w, 's> {
//     pass2d_query: Query<
//         'w,
//         's,
//         (
//             &'static Camera,
//             &'static Draw2DList,
//             &'static ParentPassId,
//             Option<&'static ClearColorBindGroup>,
//             &'static RenderTarget,
//         ),
//     >,

//     draw_query: Query<'w, 's, (&'static DrawState, &'static NodeId, Option<&'static GraphId>)>,
// 	graph_node_query: Query<'w, 's, &'static GraphId>,
//     node_query: Query<'w, 's, &'static Quad>,
//     // graph_id_query: Query<'w, 's, &'static GraphId>,
//     post_query: Query<'w, 's, (&'static PostProcess, &'static PostProcessInfo, &'static GraphId, Option<&'static AsImage>)>,
//     draw_post_query: Query<'w, 's, (&'static PostProcess, &'static GraphId, &'static NodeId)>,
//     draw_post_info: Query<'w, 's, &'static PostProcessInfo>,
//     screen: Res<'s, ScreenTarget>,
//     atlas_allocator: Res<'s, PiSafeAtlasAllocator>,
//     // bind_group_assets: Res<'s, ShareAssetMgr<RenderRes<BindGroup>>>,
//     post_bind_group_layout: OrInitRes<'s, PostBindGroupLayout>,
//     // postprocess_pipelines: Res<'s, PiPostProcessMaterialMgr>,
//     // geometrys: Res<'s, PiPostProcessGeometryManager>,
//     post_resource: Res<'w, PostprocessResource>,
//     pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,

//     // 清屏相关参数
//     fbo_clear_color: Res<'s, DynFboClearColorBindGroup>,
//     clear_draw: Res<'s, ClearDrawObj>,
//     common_sampler: Res<'s, CommonSampler>,

//     // last_rt: &'s RenderTarget,
//     last_rt_type: RenderTargetType,
//     t_type: &'s DynTargetType,
//     // copy_fbo: Option<&'s CopyFboToScreen>,
//     device: &'s RenderDevice,
//     queue: &'s RenderQueue,
//     // clear_color_group: Option<&'s ClearColorBindGroup>,
//     surface: &'s ScreenTexture,

//     depth_cache: &'s DepthCache,
// 	#[cfg(debug_assertions)]
// 	debug_entity: OrInitRes<'s, crate::resource::DebugEntity>,
// }

// // last_rt_type: RenderTargetType,
// // clear_color: ClearColor,

// impl Pass2DNode {
//     pub fn new(pass2d_id: Entity) -> Self {
//         Self {
//             pass2d_id,
//             // last_post_key: EntityKey::default(),
//             rt: None,
// 			post_draw: None
//             // param,
//         }
//     }
// }

// // (, Handle<RenderRes<BindGroup>>)


// impl Node for Pass2DNode {
//     type Input = InParamCollector<SimpleInOut>;
//     type Output = SimpleInOut;

// 	type BuildParam = BuildParam<'static, 'static>;
//     type RunParam = QueryParam<'static, 'static>;

// 	fn build<'a>(
// 		&'a mut self,
// 		world: &'a mut bevy_ecs::world::World,
// 		query_param_state: &'a mut bevy_ecs::system::SystemState<Self::BuildParam>,
// 		_context: pi_bevy_render_plugin::RenderContext,
// 		input: &'a Self::Input,
// 		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
// 		_id: GraphNodeId,
// 		_from: &'a [GraphNodeId],
// 		to: &'a [GraphNodeId],
// 	) -> Result<Self::Output, String> {
// 		let pass2d_id = self.pass2d_id;
// 		let mut param = query_param_state.get_mut(world);
// 		let mut out = SimpleInOut {
// 			target: None,
// 			valid_rect: None,
// 		};
// 		log::trace!(pass = format!("{:?}", pass2d_id).as_str(); "run graph node");
// 		// log::warn!("run1======{:?}", pass2d_id);
// 		let (layer, 
// 			camera,
// 			list,
// 			parent_pass2d_id,
// 			_clear_group, 
// 			render_target, 
// 			as_image,
// 			mut post_process, 
// 			post_process_info) = match param.pass2d_query.get_mut(pass2d_id) {
// 			Ok(r) if r.0.layer() > 0 => r,
// 			_ => return Ok(out),
// 		};
// 		// log::warn!("run2======{:?}", pass2d_id);

// 		match &**param.surface {
// 			Some(r) => r,
// 			_ => return Ok(out),
// 		};

// 		// log::warn!("run3======{:?}", pass2d_id);
// 		let (t_type, last_rt_type) = {
// 			match param.query_pass_node.get(layer.root()) {
// 				Ok(r) => (
// 					r.0.clone(),
// 					r.1.clone()
// 				),
// 				_ => {
// 					return Ok(out)
// 				}
// 			}
// 		};
// 		// SAFE: 保证渲染图并行时不会访问同时访问同一个实体的renderTarget，这里的转换是安全的
// 		let render_target = unsafe { &mut *(render_target as *const RenderTarget as usize as *mut RenderTarget) };
// 		// log::warn!("run5======{:?}, {:?}, {:?}, {:?}", pass2d_id, list.transparent, list.opaque, &render_target.bound_box);
// 		// log::warn!("run graph4==============, pass2d_id: {:?}, input count: {}, opaque: {}, transparent: {}, is_active: {:?}, is_changed: {:?}, opaque_list: {:?}, transparent_list: {:?}, view_port: {:?}, render_target: {:?}", pass2d_id, input.0.len(), list.opaque.len(), list.transparent.len(), camera.is_active, camera.is_change, &list.opaque, &list.transparent, &camera.view_port, &render_target.target);
// 		// log::trace!(pass = format!("{:?}", pass2d_id).as_str();"run graph node1, pass2d_id: {pass2d_id:?}, \nparent_pass2d_id: {:?}, \ninput count: {}, \ninput: {:?} \nopaque: {}, \ntransparent: {}, \nis_active: {:?}, \nis_changed: {:?}, \nopaque_list: {:?}, \ntransparent_list: {:?}, \nview_port: {:?}, \nlast_rt_type: {:?}, \nfrom: {from:?}, \nto: {to:?}", parent_pass2d_id, input.0.len(), input.0.iter().map(|r| {(r.0.clone(), r.1.target.is_some(), &r.1.valid_rect)}).collect::<Vec<_>>(), list.opaque.len(), list.transparent.len(), camera.is_active, camera.is_change, &list.opaque, &list.transparent, &camera.view_port, param.last_rt_type);
// 		if camera.is_active || parent_pass2d_id.is_null() {
// 			let mut render_to_fbo = false;
// 			let (offsetx, offsety) = (
// 				render_target.bound_box.mins.x - camera.view_port.mins.x,
// 				render_target.bound_box.mins.y - camera.view_port.mins.y,
// 			);
// 			let (view_port_w, view_port_h) = (
// 				camera.view_port.maxs.x - camera.view_port.mins.x,
// 				camera.view_port.maxs.y - camera.view_port.mins.y,
// 			);
// 			if list.opaque.len() > 0 || list.transparent.len() > 0 {
// 				let rt = if parent_pass2d_id.is_null() && !post_process_info.has_effect() && RenderTargetType::Screen == last_rt_type {
// 					// 如果是根节点，并且不存在effect， 直接渲染到屏幕
// 					// 根节点应该有个组件，表明是否渲染到屏幕， 如果不渲染到屏幕，则渲染到临时fbo并输出（TODO）
// 					// (RenderPassTarget::Screen(&param.surface, &param.screen.depth), &param.fbo_clear_color.0)
// 					RenderPassTarget::Screen
// 				} else {

// 					// 排除to节点中分配的Target
// 					let mut next_target = Vec::with_capacity(1);
// 					for next in to.iter() {
// 						if let Some(pass_id) = param.pass_graph_map.get(next) {
// 							if let Ok(render_target) = param.render_targets.get(*pass_id) {
// 								let r: Share<SafeTargetView> = match &render_target.target {
// 									StrongTarget::Asset(r) => r.0.clone(),
// 									StrongTarget::Raw(r) => r.0.clone(),
// 									_ => continue,
// 								};
// 								next_target.push(SimpleInOut {
// 									target: Some(r),
// 									valid_rect: None,
// 								});
// 							}
// 						}
						
// 					}
// 					// log::warn!("fbo============{:?}", );
// 					// 否则渲染到临时fbo上
// 					match render_target.get_or_create(
// 						&param.atlas_allocator,
// 						as_image,
// 						&param.cache_target,
// 						&param.as_image_mark_type,
// 						post_process_info,
// 						&t_type,
// 						16 * 1024 * 1024, // 默认最多缓存16M的target，可配置？TODO
// 						input.0.values().chain(next_target.iter()),
// 						parent_pass2d_id.is_null(),
// 					) {
// 						Some(r) => {
// 							render_to_fbo = true;
// 							out.target = Some(r.clone());
// 							RenderPassTarget::Fbo(r)
// 						}
// 						None => {
// 							// log::trace!("none==============={:?}", pass2d_id);
// 							// 不进行渲染（可能由父节点对它进行渲染）
// 							return Ok(out);
// 						}
// 					}
// 				};
// 				self.rt = Some(rt);
				
// 			} else  {
// 				match &render_target.target {
// 					StrongTarget::Asset(r) => out.target = Some(r.0.clone()),
// 					StrongTarget::Raw(r) => out.target = Some(r.0.clone()),
// 					StrongTarget::None => ()
// 				};
// 				if let Some(target) = &out.target {
// 					self.rt = Some(RenderPassTarget::Fbo(target.clone()));
// 				}
// 				render_to_fbo = true;
// 			}

// 			out.valid_rect = Some((offsetx as u32, offsety as u32, view_port_w as u32, view_port_h as u32));
// 			if let (Some(rt), true) = (&mut out.target, render_to_fbo) {

// 				if post_process_info.has_effect() {
// 					let mut target = PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default());
// 					let rect: guillotiere::euclid::Box2D<i32, guillotiere::euclid::UnknownUnit> = rt.rect().clone();

// 					let dst_size = if parent_pass2d_id.is_null() {
// 						// 根节点必须整个target做后处理
// 						target.use_x = rect.min.x as u32; // TODO(浮点误差？)
// 						target.use_y = rect.min.y as u32;
// 						(rect.max.x as u32 - rect.min.x as u32, rect.max.y as u32 - rect.min.y as u32)
// 					} else {
// 						// 其他节点只对脏区域做后处理
// 						target.use_x = rect.min.x as u32 + offsetx as u32; // TODO(浮点误差？)
// 						target.use_y = rect.min.y as u32 + offsety as u32;
// 						(view_port_w as u32, view_port_h as u32)
// 					};

// 					// log::warn!("dst_size============{:?}, {:?}", dst_size, &post_info.effect_mark);

// 					target.use_w = dst_size.0;
// 					target.use_h = dst_size.1;
					
					
// 					// 渲染后处理
// 					if let Ok(r) = post_process.calc(
// 						16, 
// 						&param.device, 
// 						&param.queue, 
// 						PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default()),
// 						dst_size,
// 						&param.atlas_allocator,
// 						&param.post_resource.resources,
// 						&param.pipline_assets,
// 						t_type.no_depth,
// 						wgpu::TextureFormat::pi_render_default(),
// 					) {
// 						if let ETextureViewUsage::SRT(post_target) = r.1.view {
// 							out.valid_rect = None;
// 							*rt = post_target.clone();
// 							self.post_draw = Some((r.0, post_target));
// 						}
// 					};
// 				}
// 			}
// 		}

// 		if let Some(as_image) = as_image {
// 			if as_image.level != pi_style::style::AsImage::Force {
// 				// 每帧都清理掉render_target.target， 避免握住无法释放
// 				render_target.target = StrongTarget::None;
// 			}
// 		}
// 		// log::warn!("out1=========={:?}, {:?}", pass2d_id, out.target);
// 		Ok(out)
// 	}

//     fn run<'a>(
//         &'a mut self,
//         world: &'a World,
//         query_param_state: &'a mut SystemState<Self::RunParam>,
//         context: RenderContext,
//         mut commands: ShareRefCell<CommandEncoder>,
//         input: &'a Self::Input,
//         _usage: &'a ParamUsage,
//         _id: GraphNodeId,
//         _from: &'a [GraphNodeId],
//         _to: &'a [GraphNodeId],
//         // context: RenderContext,
//         // mut commands: ShareRefCell<CommandEncoder>,
//         // inputs: &'a [Self::Output],
//     ) -> BoxFuture<'a, Result<(), String>> {
//         let RenderContext { device, queue, .. } = context;
		
//         let pass2d_id = self.pass2d_id;
// 		let rt = self.rt.take();
// 		let post_draw = self.post_draw.take();
// 		// log::warn!("draw1==={:?}", pass2d_id);
//         Box::pin(async move {
// 			// log::warn!("run0======{:?}", pass2d_id);
// 			let rt = match rt {
// 				Some(r) => r,
// 				None => return Ok(()),
// 			};
//             let query_param = query_param_state.get(world);
//             log::trace!(pass = format!("{:?}", pass2d_id).as_str(); "run graph node");
//             // log::warn!("run1======{:?}", pass2d_id);
//             let layer = match query_param.pass2d_query.0.get(pass2d_id) {
//                 Ok(r) if r.layer() > 0 => r.clone(),
//                 _ => {
//                     return Ok(())
//                 }
//             };
//             // log::warn!("run2======{:?}", pass2d_id);

//             let surface = match &**query_param.surface {
//                 Some(r) => r,
//                 _ => {
//                     return Ok(())
//                 }
//             };


//             // log::warn!("run3======{:?}", pass2d_id);
//             let (t_type, last_rt_type) = {
//                 match query_param.query_pass_node.get(layer.root()) {
//                     Ok(r) => (
//                         r.0.clone(),
// 						r.1.clone()
//                         // unsafe { transmute(r.1) },
//                         // unsafe { transmute(r.2) },
//                         // r.3.clone(),
//                         // unsafe { transmute(r.4) },
//                         // r.5.map_or(ClearColor(CgColor::new(0.0, 0.0, 0.0, 1.0), false), |r| r.clone()),
//                     ),
//                     _ => {
//                         return Ok(())
//                     }
//                 }
//             };
//             // log::warn!("run4======{:?}", pass2d_id);

//             let param = Param {
// 				graph_node_query: query_param.graph_node_query,
//                 pass2d_query: query_param.pass2d_query.1,
//                 draw_query: query_param.draw_query,
//                 post_query: query_param.pass2d_query.2,
//                 node_query: query_param.node_query,
//                 draw_post_query: query_param.pass2d_query.3,
//                 draw_post_info: query_param.pass2d_query.4,
//                 // graph_id_query: query_param.graph_id_query,
//                 // last_rt: last_rt,
//                 last_rt_type,
//                 // copy_fbo,
//                 screen: query_param.screen,
//                 surface: surface,
//                 atlas_allocator: query_param.atlas_allocator,
//                 t_type: &t_type,
//                 // bind_group_assets: query_param.bind_group_assets,
//                 post_bind_group_layout: query_param.post_bind_group_layout,
//                 // postprocess_pipelines: query_param.postprocess_pipelines,
//                 // geometrys: query_param.geometrys,
//                 post_resource: query_param.post_resource,
//                 pipline_assets: query_param.pipline_assets,

//                 device: &device,
//                 queue: &queue,
//                 fbo_clear_color: query_param.fbo_clear_color,
//                 // clear_color_group,
//                 clear_draw: query_param.clear_draw,
//                 common_sampler: query_param.common_sampler,
//                 depth_cache: &query_param.depth_cache,
// 				#[cfg(debug_assertions)]
// 				debug_entity: query_param.debug_entity,
//             };

//             let post_list = param.post_query.get(pass2d_id);

//             if let Ok((camera, list, parent_pass2d_id, _clear_group, render_target)) = param.pass2d_query.get(pass2d_id) {
// 				// log::warn!("run5==={:?}", pass2d_id);
// 				// SAFE: 保证渲染图并行时不会访问同时访问同一个实体的renderTarget，这里的转换是安全的
// 				// let render_target = unsafe { &mut *(render_target as *const RenderTarget as usize as *mut RenderTarget) };
// 				// log::warn!("run5======{:?}, {:?}, {:?}, {:?}", pass2d_id, list.transparent, list.opaque, &render_target.bound_box);
// 				// log::warn!("run graph4==============, pass2d_id: {:?}, input count: {}, opaque: {}, transparent: {}, is_active: {:?}, is_changed: {:?}, opaque_list: {:?}, transparent_list: {:?}, view_port: {:?}, render_target: {:?}", pass2d_id, input.0.len(), list.opaque.len(), list.transparent.len(), camera.is_active, camera.is_change, &list.opaque, &list.transparent, &camera.view_port, &render_target.target);
// 				// log::trace!(pass = format!("{:?}", pass2d_id).as_str();"run graph node1, pass2d_id: {pass2d_id:?}, \nparent_pass2d_id: {:?}, \ninput count: {}, \ninput: {:?} \nopaque: {}, \ntransparent: {}, \nis_active: {:?}, \nis_changed: {:?}, \nopaque_list: {:?}, \ntransparent_list: {:?}, \nview_port: {:?}, \nlast_rt_type: {:?}, \nfrom: {from:?}, \nto: {to:?}", parent_pass2d_id, input.0.len(), input.0.iter().map(|r| {(r.0.clone(), r.1.target.is_some(), &r.1.valid_rect)}).collect::<Vec<_>>(), list.opaque.len(), list.transparent.len(), camera.is_active, camera.is_change, &list.opaque, &list.transparent, &camera.view_port, param.last_rt_type);
// 				// let (offsetx, offsety) = (
// 				// 	render_target.bound_box.mins.x - camera.view_port.mins.x,
// 				// 	render_target.bound_box.mins.y - camera.view_port.mins.y,
// 				// );
// 				// let (view_port_w, view_port_h) = (
// 				// 	camera.view_port.maxs.x - camera.view_port.mins.x,
// 				// 	camera.view_port.maxs.y - camera.view_port.mins.y,
// 				// );
// 				let clear_color = &param.fbo_clear_color.0;
// 				if list.transparent.len() > 0 || list.opaque.len() > 0 {
// 					let (_offset, _view_port) = {
// 						let mut input_groups = LinkNode { value: None, next: None };
// 						let mut post_draw = LinkNode { value: None, next: None };
						
// 						let rp_rt = match &rt {
// 							RenderPassTarget::Fbo(r) => RPTarget::Fbo(r),
// 							RenderPassTarget::Screen => RPTarget::Screen(&param.surface, &param.screen.depth),
// 						};
// 						// 创建一个渲染Pass
// 						let (mut rp, view_port, clear_port, offset) = create_rp(
// 							rp_rt,
// 							commands.borrow_mut(),
// 							&camera.view_port,
// 							&render_target.bound_box,
// 							None,
// 						);

// 						// log::warn!("set_viewport clear_port 1============={:?}, clear_port: {:?}, view_port: {:?}, render_target_view: {:?}", pass2d_id, clear_port, &camera.view_port, render_target.bound_box);
// 						// 清屏
// 						// if let Some(clear_color) = clear_color {
// 						// fbo总是需要使用draw的方式清屏，如果是根节点，直接绘制到屏幕，就不需要使用这种方式清屏
// 						// if !parent_pass2d_id.is_null() {
// 						// 设置视口
// 						rp.set_viewport(clear_port.0, clear_port.1, clear_port.2, clear_port.3, 0.0, 1.0);
// 						clear_color.set(&mut rp, UiMaterialBind::set());
// 						param.depth_cache.list[0].set(&mut rp, DepthBind::set()); // 清屏所用深度总用0
// 						param.clear_draw.0.draw(&mut rp);
// 						// 相机在drawObj中已经描述
// 						// }
// 						// log::warn!("set_viewport2============={:?}", view_port);
// 						// 设置视口
// 						rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

// 						Self::draw_list(
// 							&input.0,
// 							&mut post_draw.value,
// 							&mut post_draw.next,
// 							&mut input_groups.value,
// 							&mut input_groups.next,
// 							&mut rp,
// 							(view_port.2 as u32, view_port.3 as u32),
// 							&world,
// 							list,
// 							&param,
// 							camera,
// 							camera,
// 							&view_port,
// 							&view_port,
// 						);
// 						// log::warn!("draw=========={:?}, {:?}, {:?}", pass2d_id, list.transparent.len(), list.opaque.len());
// 						(offset, view_port)
// 					};
// 				}

// 				// out.valid_rect = Some((offsetx as u32, offsety as u32, view_port_w as u32, view_port_h as u32));
				
// 				if let Ok((post_process, post_info, _graph_id, _as_image)) = post_list {
// 					// log::warn!("run6==={:?}", pass2d_id);
// 					if let Some(post_draw) = post_draw {
// 						// log::warn!("run7==={:?}", pass2d_id);
// 						// 渲染后处理
// 						post_process.draw_front(
// 							commands.borrow_mut(),
// 							&post_draw.0,
// 						);

// 						// 处理根节点
// 						if parent_pass2d_id.is_null() {
// 							if let RenderTargetType::Screen = param.last_rt_type {
// 								// let rect = rt.rect();
// 								// let view_port = Aabb2::new(
// 								//     Point2::new(rect.min.x as f32 + render_target.bound_box.mins.x, rect.min.y as f32 + render_target.bound_box.mins.y),
// 								//     Point2::new(rect.max.x as f32 + render_target.bound_box.mins.x, rect.max.y as f32 + render_target.bound_box.mins.y),
// 								// );
// 								let post_draw1;
// 								let view_port = Aabb2::new(
// 									Point2::new(render_target.bound_box.mins.x as f32, render_target.bound_box.mins.y as f32),
// 									Point2::new(render_target.bound_box.maxs.x as f32 - render_target.bound_box.mins.x as f32, render_target.bound_box.maxs.y as f32 - render_target.bound_box.mins.y as f32),
// 								);
// 								// log::warn!("set view_port============{:?}, {:?}", view_port, &render_target.bound_box);
// 								// 将最终渲染目标渲染到屏幕上
// 								// 创建一个渲染Pass
// 								let (mut rp, view_port, _clear_port, _) = create_rp(
// 									RPTarget::Screen(&param.surface, &None),
// 									commands.borrow_mut(),
// 									&view_port,
// 									&view_port,
// 									None,
// 								);
// 								// 设置视口
// 								rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
// 								let matrix = &camera.project * &post_info.matrix.0; // post_info.matrix?TODO
// 								if let Some(draw_obj) = post_process.draw_final(
// 									param.device,
// 									param.queue,
// 									matrix.as_slice(),
// 									0.0,
// 									&param.atlas_allocator,
// 									&PostprocessTexture::from_share_target(post_draw.1.clone(), wgpu::TextureFormat::pi_render_default()),
// 									(view_port.2 as u32, view_port.3 as u32),
// 									&param.post_resource.resources,
// 									&param.pipline_assets,
// 									wgpu::ColorTargetState {
// 										format: wgpu::TextureFormat::pi_render_default(),
// 										blend: Some(wgpu::BlendState {
// 											color: wgpu::BlendComponent {
// 												operation: wgpu::BlendOperation::Add,
// 												src_factor: wgpu::BlendFactor::One,
// 												dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
// 											},
// 											alpha: wgpu::BlendComponent {
// 												operation: wgpu::BlendOperation::Add,
// 												src_factor: wgpu::BlendFactor::One,
// 												dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
// 											},
// 										}),
// 										write_mask: wgpu::ColorWrites::ALL,
// 									},
// 									None,
// 									param.t_type.no_depth,
// 									wgpu::TextureFormat::pi_render_default(),
// 								) {
// 									post_draw1 = draw_obj;
// 									post_draw1.draw(&mut rp);
// 									// log::error!("draw_final fail, {:?} ", e);
// 								}

// 								// log::warn!("draw screen=========={:?}", pass2d_id);
// 							}
// 						}
// 					}
// 				}
//             }
//             // log::warn!("out1=========={:?}, {:?}", pass2d_id, out.target.is_some());
//             Ok(())
//         })
//     }
// }

// impl Pass2DNode {
//     /// 渲染pass_2d(渲染列表中的一个渲染索引，如果是一个Pass2d， 则走该分支)
//     /// * last_view_port-当前渲染目标的视口范围（）
//     /// * last_camera-当前渲染目标的根相机（渲染过程是一个递归过程，每遇到一个Pass2d，当前相机会发生变化，当last_camera在递归过程保持不变）
//     /// * cur_view_port-当前设置的视口
//     /// * cur_camera-当前设置的相机
// 	#[inline]
//     pub fn render_pass_2d<'a, 'b>(
//         pass2d_id: EntityKey,
//         input: &'a XHashMap<GraphNodeId, SimpleInOut>,
// 		mut post_draw: &'a mut Option<DrawObj>,
// 		mut post_draw_next: &'a mut Option<Box<LinkNode<DrawObj>>>,
//         mut input_groups: &'a mut Option<(BindGroup, Buffer)>,
// 		mut input_groups_next: &'a mut Option<Box<LinkNode<(BindGroup, Buffer)>>>,
//         rp: &mut RenderPass<'a>,
//         target_size: (u32, u32),
//         world: &'a World,
//         param: &'a Param<'a, 'a>,
//         last_camera: &'a Camera,
//         cur_camera: &'a Camera,
//         last_view_port: &(f32, f32, f32, f32),
//         cur_view_port: &(f32, f32, f32, f32),
//         depth: usize,
//     ) -> (&'a mut Option<DrawObj>, &'a mut Option<Box<LinkNode<DrawObj>>>, &'a mut Option<(BindGroup, Buffer)>, &'a mut Option<Box<LinkNode<(BindGroup, Buffer)>>>) {
//         match param.post_query.get(*pass2d_id) {
//             Ok((r, post_info, graph_id, as_image)) if post_info.has_effect() => {
// 				let graph_id = match as_image {
// 					Some(r) => {
// 						if r.post_process.is_null() {
// 							graph_id.clone()
// 						} else if let Ok(r) = param.graph_node_query.get(*r.post_process) {
// 							r.clone()
// 						} else {
// 							GraphId::default()
// 						}
						
// 					},
// 					None => graph_id.clone(),
// 				};
//                 // log::warn!("draw_final0==========={:?}", graph_id.0);
//                 let (src, valid_rect) = match input.get(&graph_id) {
//                     Some(r) => (
//                         match &r.target {
//                             Some(r) => r,
//                             None => return (post_draw, post_draw_next, input_groups, input_groups_next),
//                         },
//                         &r.valid_rect,
//                     ),
//                     None => {
//                         // 这种情况有可能出现，后处理对象可能为空
//                         log::debug!(
//                             "prepare render post process, but input is none, pass2d_id={:?}, graph_id={:?}",
//                             pass2d_id,
//                             graph_id
//                         );
//                         return (post_draw, post_draw_next, input_groups, input_groups_next);
//                     }
//                 };

//                 let matrix = &cur_camera.project * &post_info.matrix.0;

//                 // let blend_state = if !r.src_preimultiplied {
//                 //     wgpu::BlendState {
//                 //         color: wgpu::BlendComponent {
//                 //             operation: wgpu::BlendOperation::Add,
//                 //             src_factor: wgpu::BlendFactor::SrcAlpha,
//                 //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                 //         },
//                 //         alpha: wgpu::BlendComponent {
//                 //             operation: wgpu::BlendOperation::Add,
//                 //             src_factor: wgpu::BlendFactor::One,
//                 //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                 //         },
//                 //     }
//                 // } else {
//                 //     wgpu::BlendState {
//                 //         color: wgpu::BlendComponent {
//                 //             operation: wgpu::BlendOperation::Add,
//                 //             src_factor: wgpu::BlendFactor::One,
//                 //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                 //         },
//                 //         alpha: wgpu::BlendComponent {
//                 //             operation: wgpu::BlendOperation::Add,
//                 //             src_factor: wgpu::BlendFactor::One,
//                 //             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                 //         },
//                 //     }
//                 // };
//                 // log::warn!("node_id1======{:?}, {:?}", pass2d_id, post_info.matrix.0);
//                 // log::warn!("draw_final==========={:?}", r)

//                 let mut target = PostprocessTexture::from_share_target(src.clone(), wgpu::TextureFormat::pi_render_default());
//                 if let Some(r) = valid_rect {
//                     target.use_x = target.use_x + r.0;
//                     target.use_y = target.use_y + r.1;
//                     target.use_w = r.2;
//                     target.use_h = r.3;
//                 }
//                 if let Some(draw_obj) = r.draw_final(
//                     param.device,
//                     param.queue,
//                     matrix.as_slice(),
//                     depth as f32 / 60000.0,
//                     &param.atlas_allocator,
//                     &target,
//                     target_size,
//                     &param.post_resource.resources,
//                     &param.pipline_assets,
//                     wgpu::ColorTargetState {
//                         format: wgpu::TextureFormat::pi_render_default(),
//                         blend: Some(wgpu::BlendState {
//                             color: wgpu::BlendComponent {
//                                 operation: wgpu::BlendOperation::Add,
//                                 src_factor: wgpu::BlendFactor::One,
//                                 dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                             },
//                             alpha: wgpu::BlendComponent {
//                                 operation: wgpu::BlendOperation::Add,
//                                 src_factor: wgpu::BlendFactor::One,
//                                 dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                             },
//                         }),
//                         write_mask: wgpu::ColorWrites::ALL,
//                     },
//                     Some(pi_render::renderer::pipeline::DepthStencilState {
//                         format: wgpu::TextureFormat::Depth32Float,
//                         depth_write_enabled: true,
//                         depth_compare: wgpu::CompareFunction::GreaterEqual,
//                         stencil: wgpu::StencilState::default(),
//                         bias: pi_render::renderer::pipeline::DepthBiasState::default(),
//                     }),
//                     param.t_type.has_depth,
//                     wgpu::TextureFormat::pi_render_default(),
//                 ) {
// 					*post_draw = Some(draw_obj);
// 					post_draw.as_ref().unwrap().draw(rp);

// 					*post_draw_next = Some(Box::new(LinkNode{ value: None, next: None}));
// 					let LinkNode { value, next } = &mut **post_draw_next.as_mut().unwrap();
// 					return (value, next, input_groups, input_groups_next);
//                     // // 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
// 					// let rr = unsafe { &mut *(post_draw as *const Vec<Vec<Option<DrawObj>>> as usize as *mut Vec<Vec<Option<DrawObj>>>) };
// 					// let capacity = rr.last_mut().unwrap().capacity();
// 					// if capacity == *draw_i {
// 					// 	let mut v: Vec<Option<DrawObj>> = Vec::with_capacity(10);
// 					// 	for _i in 0..10 {
// 					// 		v.push(None);
// 					// 	}
// 					// 	rr.push(v);
// 					// 	*draw_i = *draw_i - capacity;
// 					// }
// 					// let last = rr.last_mut().unwrap();
// 					// last[*draw_i] = Some(draw_obj);
// 					// last[*draw_i].as_ref().unwrap().draw(rp);
// 					// *draw_i += 1;

//                     // // log::warn!("zzzz=========={:p}", post_draw);
// 					// let rr = unsafe { &mut *(post_draw as *const Vec<DrawObj> as usize as *mut Vec<DrawObj>) };
// 					// if rr.capacity() == post_draw.len() {
// 					// 	panic!("xxxxx");
// 					// }
// 					// rr.push(draw_obj);
//                     // let index = rr.len() - 1;
// 					// // log::warn!("bbb=========={:?}, {:?}, index= {}", rr.len(), rr.capacity(), rr.get(index).is_some());
// 					// if let Some(rr) = rr.get(index) { // 似乎编译器存在bug？ rr[index].draw(rp);调用在release版本下会崩溃
// 					// 	rr.draw(rp);
// 					// } else {
// 					// 	unreachable!();
// 					// }
//                     // rr[index].draw(rp);
//                     // log::error!("draw_final fail, {:?} ", e);
//                 }
//             }
//             _ => {
				
//                 // 如果不存在后处理，则将pass2d中的所有渲染对象渲染到rp上
//                 if let Ok((
//                     camera_new,
//                     // rt_key,
//                     list,
//                     _pass2d_id,
//                     _,
//                     _,
//                 )) = param.pass2d_query.get(*pass2d_id)
//                 {

// 					log::trace!("run pass, pass_id: {:?}, opaque={:?}, transparent={:?}", pass2d_id, &list.opaque,  &list.transparent);
//                     let v = (
//                         (last_view_port.0 as f32 - last_camera.view_port.mins.x) + camera_new.view_port.mins.x,
//                         (last_view_port.1 as f32 - last_camera.view_port.mins.y) + camera_new.view_port.mins.y,
//                         camera_new.view_port.maxs.x - camera_new.view_port.mins.x,
//                         camera_new.view_port.maxs.y - camera_new.view_port.mins.y,
//                     );

//                     if v.2 <= 0.0 || v.3 <= 0.0 {
//                         return (post_draw, post_draw_next, input_groups, input_groups_next);
//                     }
//                     rp.set_viewport(v.0, v.1, v.2, v.3, 0.0, 1.0);
//                     let r = Self::draw_list(
//                         input,
//                         post_draw,
// 						post_draw_next,
//                         input_groups,
// 						input_groups_next,
//                         rp,
//                         target_size,
//                         world,
//                         list,
//                         param,
//                         last_camera,
//                         camera_new,
//                         last_view_port,
//                         &v,
//                     );
// 					post_draw = r.0;
// 					post_draw_next = r.1;
// 					input_groups = r.2;
// 					input_groups_next = r.3;
// 					// log::warn!("set_viewport5============={:?}", cur_view_port);
//                     rp.set_viewport(cur_view_port.0, cur_view_port.1, cur_view_port.2, cur_view_port.3, 0.0, 1.0);
//                     if let Some(camera) = &cur_camera.bind_group {
//                         camera.set(rp, CameraBind::set());
//                     }
//                 }
//             }
//         }
// 		return (post_draw, post_draw_next, input_groups, input_groups_next);
//     }

//     // 将单个DrawObj的后处理结果渲染到目标上
//     #[inline]
// 	pub fn render_draw_obj_post<'a, 'b>(
//         draw_obj_id: EntityKey,
//         input: &'a XHashMap<GraphNodeId, SimpleInOut>,
//         post_draw: &'a mut Option<DrawObj>,
// 		post_draw_next: &'a mut Option<Box<LinkNode<DrawObj>>>,
//         rp: &mut RenderPass<'a>,
//         target_size: (u32, u32),
//         param: &'a Param<'a, 'a>,
//         cur_camera: &'a Camera,
//         depth: usize,
//     ) -> (&'a mut Option<DrawObj>, &'a mut Option<Box<LinkNode<DrawObj>>>) {
//         if let Ok((r, graph_id, node_id)) = param.draw_post_query.get(*draw_obj_id) {
//             let src = match input.get(&graph_id.0) {
//                 Some(r) => match &r.target {
//                     Some(r) => r,
//                     None => return (post_draw, post_draw_next),
//                 },
//                 None => {
//                     // 这种情况有可能出现，后处理对象可能为空
//                     // log::error!("prepare render post process, but pre result is none");
//                     return (post_draw, post_draw_next);
//                 }
//             };

//             let post_info = match param.draw_post_info.get(***node_id) {
//                 Ok(r) => r,
//                 Err(_) => return (post_draw, post_draw_next),
//             };
//             // log::warn!("node_id======{:?}, {:?}", node_id, post_info.matrix);
//             let matrix = &cur_camera.project * &post_info.matrix.0;
//             if let Some(draw_obj) = r.draw_final(
//                 param.device,
//                 param.queue,
//                 matrix.as_slice(),
//                 depth as f32 / 60000.0,
//                 &param.atlas_allocator,
//                 &PostprocessTexture::from_share_target(src.clone(), wgpu::TextureFormat::pi_render_default()),
//                 target_size,
//                 &param.post_resource.resources,
//                 &param.pipline_assets,
//                 wgpu::ColorTargetState {
//                     format: wgpu::TextureFormat::pi_render_default(),
//                     blend: Some(wgpu::BlendState {
//                         color: wgpu::BlendComponent {
//                             operation: wgpu::BlendOperation::Add,
//                             src_factor: wgpu::BlendFactor::SrcAlpha,
//                             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                         },
//                         alpha: wgpu::BlendComponent {
//                             operation: wgpu::BlendOperation::Add,
//                             src_factor: wgpu::BlendFactor::One,
//                             dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
//                         },
//                     }),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 },
//                 Some(pi_render::renderer::pipeline::DepthStencilState {
//                     format: wgpu::TextureFormat::Depth32Float,
//                     depth_write_enabled: true,
//                     depth_compare: wgpu::CompareFunction::GreaterEqual,
//                     stencil: wgpu::StencilState::default(),
//                     bias: pi_render::renderer::pipeline::DepthBiasState::default(),
//                 }),
//                 param.t_type.has_depth,
//                 wgpu::TextureFormat::pi_render_default(),
//             ) {
// 				*post_draw = Some(draw_obj);
// 				post_draw.as_ref().unwrap().draw(rp);

// 				*post_draw_next = Some(Box::new(LinkNode{ value: None, next: None}));
// 				let LinkNode { value, next } = &mut **post_draw_next.as_mut().unwrap();
// 				return (value, next);

// 				// 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
// 				// let rr = unsafe { &mut *(post_draw as *const Vec<Vec<Option<DrawObj>>> as usize as *mut Vec<Vec<Option<DrawObj>>>) };
// 				// let capacity = rr.last_mut().unwrap().capacity();
// 				// if capacity == *draw_i {
// 				// 	let mut v: Vec<Option<DrawObj>> = Vec::with_capacity(10);
// 				// 	for _i in 0..10 {
// 				// 		v.push(None);
// 				// 	}
// 				// 	rr.push(v);
// 				// 	*draw_i = *draw_i - capacity;
// 				// }
// 				// let last = rr.last_mut().unwrap();
// 				// last[*draw_i] = Some(draw_obj);
// 				// last[*draw_i].as_ref().unwrap().draw(rp);
// 				// *draw_i += 1;
// 				// rr.push(draw_obj);
// 				// let index = rr.len() - 1;
// 				// log::warn!("bbb=========={:?}, {:?}, index= {}", rr.len(), rr.capacity(), rr.get(index).is_some());
// 				// if let Some(rr) = rr.get(draw_i) { // 似乎编译器存在bug？ rr[index].draw(rp);调用在release版本下会崩溃
// 				// 	rr.draw(rp);
// 				// } else {
// 				// 	unreachable!();
// 				// }
//                 // log::error!("draw_final fail, {:?} ", e);
//             }
//         }

// 		(post_draw, post_draw_next)
//     }

//     fn draw_list<'a, 'w, 'b>(
//         input: &'a XHashMap<GraphNodeId, SimpleInOut>,
//         mut post_draw: &'a mut Option<DrawObj>,
// 		mut post_draw_next: &'a mut Option<Box<LinkNode<DrawObj>>>,
//         mut input_groups: &'a mut Option<(BindGroup, Buffer)>,
// 		mut input_groups_next: &'a mut Option<Box<LinkNode<(BindGroup, Buffer)>>>,
//         rp: &'w mut RenderPass<'a>,
//         target_size: (u32, u32),
//         world: &'a World,
//         list: &Draw2DList,

//         param: &'a Param<'a, 'a>,
//         last_camera: &'a Camera,
//         cur_camera: &'a Camera,
//         last_view_port: &(f32, f32, f32, f32),
//         cur_view_port: &(f32, f32, f32, f32),
//     ) -> (&'a mut Option<DrawObj>, &'a mut Option<Box<LinkNode<DrawObj>>>, &'a mut Option<(BindGroup, Buffer)>, &'a mut Option<Box<LinkNode<(BindGroup, Buffer)>>>) {

//         if let Some(camera) = &cur_camera.bind_group {
//             camera.set(rp, CameraBind::set());
//         }

//         // log::warn!("draw============================={:?}, {:?}, {:?}, {:?}", list.opaque.len(), list.transparent.len(), list.opaque, list.transparent);

//         for (draw_index, depth) in list.opaque.iter().chain(list.transparent.iter()) {
//             match draw_index {
//                 DrawIndex::DrawObj(e) => {
//                     if let Ok((state, node_id, graph_id)) = param.draw_query.get(**e) {
// 						// log::warn!("draw======{node_id:?}, {graph_id:?}");
//                         let quad = match param.node_query.get(***node_id) {
//                             Ok(r) => r,
//                             _ => continue,
//                         };
// 						// log::warn!("draw1======{node_id:?}, {graph_id:?}");
//                         // 如果存在graph_id，表示该渲染对象将输入的一个ShareTargetView作为纹理，渲染到gui上
//                         if let Some(graph_id) = graph_id {
// 							log::trace!("draw canvas========={graph_id:?}");
// 							// log::warn!("xxxx========={graph_id:?}");
//                             let src = match input.get(&**graph_id) {
//                                 Some(r) => match &r.target {
//                                     Some(r) => r,
//                                     None => continue,
//                                 },
//                                 None => continue,
//                             };
// 							// log::warn!("xxxx1========={graph_id:?}");
//                             let rect = src.rect();
//                             // 根据纹理大小和渲染目标大小，来确定过滤方式
//                             // 如果大小近似相等，则使用点过滤，否则使用双线性过滤
//                             let s = if ((quad.maxs.x - quad.mins.x) as i32 - rect.width()).abs() <= 1
//                                 && ((quad.maxs.y - quad.mins.y) as i32 - rect.height()).abs() <= 1
//                             {
//                                 &param.common_sampler.pointer
//                             } else {
//                                 &param.common_sampler.default
//                             };

// 							let group = Self::create_post_process_data(src, &param, s);
// 							*input_groups = Some(group);
//                             // 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
// 							// let rr = unsafe {
//                             //     &mut *(input_groups as *const Vec<Vec<Option<(BindGroup, Buffer)>>> as usize
//                             //         as *mut Vec<Vec<Option<(BindGroup, Buffer)>>>)
//                             // };
// 							// let capacity = rr.last_mut().unwrap().capacity();
// 							// if capacity == *groups_i {
// 							// 	let mut v = Vec::with_capacity(10);
// 							// 	for _i in 0..10 {
// 							// 		v.push(None);
// 							// 	}
// 							// 	rr.push(v);
// 							// 	*groups_i = *groups_i - capacity;
// 							// }
// 							// let last = rr.last_mut().unwrap();
//                             // last[*groups_i] = Some(Self::create_post_process_data(src, &param, s));
							
// 							// let input_groups = unsafe {transmute::<_, &'a mut Option<(BindGroup, Buffer)>>(input_groups)};
//                             rp.set_bind_group(SampBind::set(), &input_groups.as_ref().unwrap().0, &[]);
//                             rp.set_vertex_buffer(UvVert::location() as u32, *input_groups.as_ref().unwrap().1.slice(..));

// 							*input_groups_next = Some(Box::new(LinkNode { value: None, next: None }));
// 							let LinkNode{value, next} = &mut **input_groups_next.as_mut().unwrap();
// 							input_groups = value;
// 							input_groups_next = next;
// 							// *groups_i += 1;
//                         }


//                         if state.bindgroups.get_group(CameraBind::set()).is_none() {
//                             if let Some(r) = &cur_camera.bind_group {
//                                 r.set(rp, CameraBind::set());
//                             }
//                         }
//                         if let Some(depth) = param.depth_cache.list.get(*depth) {
//                             // 设置深度bind
//                             depth.set(rp, DepthBind::set());
//                         }

						
// 						#[cfg(debug_assertions)]
// 						{
// 							use crate::shader::ui_meterial::WorldUniform;
// 							use pi_render::renderer::draw_obj::DrawBindGroup;
// 							let o = &state.bindgroups.groups()[<WorldUniform as pi_render::rhi::shader::Uniform>::Binding::set() as usize];
// 							if let DrawBindGroup::Offset(r) = o {
// 								let group = r.get_group();
// 								if ***node_id == param.debug_entity.0.0 {
// 									log::debug!("draw group====={:?}, {:?}", group.bind_group, &group.offsets);
// 								}
// 							}
// 						}
					
//                         state.draw(rp);
//                     }
//                 }
//                 DrawIndex::DrawObjPost(e) => {

//                     let r = Self::render_draw_obj_post(*e, input, post_draw, post_draw_next, rp, target_size, param, cur_camera, *depth);
// 					post_draw = r.0;
// 					post_draw_next = r.1;
//                 }
//                 DrawIndex::Pass2D(e) => {

//                     let r = Self::render_pass_2d(
//                         *e,
//                         input,
//                         post_draw,
// 						post_draw_next,
//                         input_groups,
// 						input_groups_next,
//                         rp,
//                         target_size,
//                         world,
//                         param,
//                         last_camera,
//                         cur_camera,
//                         last_view_port,
//                         cur_view_port,
//                         *depth,
//                     );
// 					post_draw = r.0;
// 					post_draw_next = r.1;
// 					input_groups = r.2;
// 					input_groups_next = r.3;
//                 }
//             }
//         }

// 		return (post_draw, post_draw_next,input_groups, input_groups_next);
//     }

//     // 创建后处理数据（bindgroup和uv buffer）
//     fn create_post_process_data<'s>(
//         texture: &ShareTargetView,
//         param: &'s Param<'s, 's>,
//         sampler: &'s Sampler,
//     ) -> (BindGroup, Buffer) {
//         let uv = texture.uv();
//         // let group_key = calc_hash(&(texture.ty_index(), texture.target_index()), calc_hash(&"render target", 0)); // TODO
//         // (
//         //     match param.bind_group_assets.get(&group_key) {
//         //         Some(r) => r,
//         //         None => {
// 		// 			log::warn!("zzzz===============, group_key{:?}, {:?}", texture.ty_index(), texture.target_index());
//         //             let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
//         //                 layout: &param.post_bind_group_layout,
//         //                 entries: &[
//         //                     wgpu::BindGroupEntry {
//         //                         binding: 0,
//         //                         resource: wgpu::BindingResource::Sampler(sampler),
//         //                     },
//         //                     wgpu::BindGroupEntry {
//         //                         binding: 1,
//         //                         resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0),
//         //                     },
//         //                 ],
//         //                 label: Some("post process texture bind group create"),
//         //             });
//         //             param.bind_group_assets.insert(group_key, RenderRes::new(group.clone(), 5)).unwrap()
//         //         }
//         //     },
// 		(
// 			param.device.create_bind_group(&wgpu::BindGroupDescriptor {
// 				layout: &param.post_bind_group_layout,
// 				entries: &[
// 					wgpu::BindGroupEntry {
// 						binding: 0,
// 						resource: wgpu::BindingResource::Sampler(sampler),
// 					},
// 					wgpu::BindGroupEntry {
// 						binding: 1,
// 						resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0),
// 					},
// 				],
// 				label: Some("post process texture bind group create"),
// 			}),
			
//             // 实时创建uvbuffer， 因为该buffer动态性很高，可能不应该创建为资源？
//             // 这里应该与脏区域相交，渲染脏区域， TODO
//             param.device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                 label: Some("post process uv Buffer"),
//                 contents: bytemuck::cast_slice(&uv),
//                 usage: wgpu::BufferUsages::VERTEX,
//             }),
//         )
//     }
// }

// #[derive(Clone)]
// pub enum RenderPassTarget {
//     Fbo(ShareTargetView),
//     // Screen(&'a ScreenTexture, &'a Option<Handle<RenderRes<wgpu::TextureView>>>),
// 	Screen
// }

// #[derive(Clone)]
// pub enum RPTarget<'a> {
//     Fbo(&'a ShareTargetView),
//     Screen(&'a ScreenTexture, &'a Option<Handle<RenderRes<wgpu::TextureView>>>),
// }

// // 返回renderpass， view_port， clear_port
// pub fn create_rp<'a>(
//     rt: RPTarget<'a>,
//     commands: &'a mut CommandEncoder,
//     view_port: &Aabb2,
//     target_view_port: &Aabb2, // 渲染目标对应的view_port;
//     // last_rt: &'a RenderTarget,
//     // surface: &'a ScreenTexture,
//     ops: Option<wgpu::Operations<wgpu::Color>>,
// ) -> (RenderPass<'a>, (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32)) {
//     match rt {
//         RPTarget::Screen(surface, depth) => {
//             // 渲染到屏幕上
//             let ops = match ops {
//                 Some(r) => r,
//                 None => wgpu::Operations {
//                     // load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
//                     load: wgpu::LoadOp::Load,
//                     store: true,
//                 },
//             };
//             let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: None,
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     resolve_target: None,
//                     ops,
//                     view: surface.view.as_ref().unwrap(),
//                 })],
//                 depth_stencil_attachment: match depth {
//                     Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
//                         stencil_ops: None,
//                         depth_ops: Some(wgpu::Operations {
//                             load: wgpu::LoadOp::Load,
//                             store: true,
//                         }),
//                         view: r,
//                     }),
//                     None => None,
//                 },
//             });
//             (
//                 rp,
//                 (
//                     view_port.mins.x,
//                     view_port.mins.y,
//                     view_port.maxs.x - view_port.mins.x,
//                     view_port.maxs.y - view_port.mins.y,
//                 ),
//                 (
//                     view_port.mins.x,
//                     view_port.mins.y,
//                     view_port.maxs.x - view_port.mins.x,
//                     view_port.maxs.y - view_port.mins.y,
//                 ),
//                 (0.0, 0.0),
//             )
//         }
//         RPTarget::Fbo(rt) => {
//             // 渲染到临时的fbo上
//             // let mut r = last_rt.target.as_ref().unwrap();
//             // if let Some(t) = rt {
//             //     r = t;
//             // }
// 			// fbo永远不清屏
//             create_rp_for_fbo(rt, commands, view_port, target_view_port, None)
//         }
//     }
// }


// pub fn create_rp_for_fbo<'a>(
//     r: &'a ShareTargetView,
//     commands: &'a mut CommandEncoder,
//     view_port: &Aabb2,
//     target_view_port: &Aabb2,
//     ops: Option<wgpu::Operations<wgpu::Color>>,
// ) -> (RenderPass<'a>, (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32)) {
//     let ops = match ops {
//         Some(r) => r,
//         None => wgpu::Operations {
//             // load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
//             load: wgpu::LoadOp::Load,
//             store: true,
//         },
//     };
//     let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
//         label: None,
//         color_attachments: r
//             .target()
//             .colors
//             .iter()
//             .map(|view| {
//                 Some(wgpu::RenderPassColorAttachment {
//                     resolve_target: None,
//                     ops,
//                     view: &view.0,
//                 })
//             })
//             .collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
//             .as_slice(),
//         depth_stencil_attachment: match &r.target().depth {
//             Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
//                 stencil_ops: None,
//                 depth_ops: Some(wgpu::Operations {
//                     load: wgpu::LoadOp::Load,
//                     store: true,
//                 }),
//                 view: &r.0,
//             }),
//             None => None,
//         },
//     });
//     let rect = r.rect();
//     let (offsetx, offsety) = (view_port.mins.x - target_view_port.mins.x, view_port.mins.y - target_view_port.mins.y);
//     let view_port_ = (
//         rect.min.x as f32 + offsetx,
//         rect.min.y as f32 + offsety,
//         view_port.maxs.x - view_port.mins.x,
//         view_port.maxs.y - view_port.mins.y,
//     );
//     // 如果
//     let scissor = if target_view_port.mins.x == view_port.mins.x
//         && target_view_port.maxs.x == view_port.maxs.x
//         && target_view_port.mins.y == view_port.mins.y
//         && target_view_port.maxs.y == view_port.maxs.y
//     {
//         // 如果target对应的视口区域跟当前需要渲染的视口区域一样，则设置裁剪口为border区域（因为这很可能是第一次渲染该target，分配出来的fbo中的数据是随机的，如果不清理边框区域，边缘可能会有黑线）
//        let rect_border: &guillotiere::euclid::Box2D<i32, guillotiere::euclid::UnknownUnit> = r.rect_with_border();
// 	//    log::warn!("rect_with_border========{:?}, {:?}", rect, rect_border);
// 	   (
//         rect_border.min.x as f32,
//         rect_border.min.y as f32,
//         (rect_border.max.x - rect_border.min.x) as f32,
//         (rect_border.max.y - rect_border.min.y) as f32,
//     )
//     } else {
// 		// log::warn!("rect_with_border1========{:?}, {:?}, {:?}", rect, target_view_port, view_port);
//         // 否则为视口区域
//         view_port_
//     };
// 	// log::warn!("!!!!=========={:?}, {:?}, {:?}", target_view_port, view_port, view_port_);
//     // let scissor = (
//     //     scissor_rect.min.x as f32 + offsetx,
//     //     scissor_rect.min.y as f32 + offsety,
//     //     (scissor_rect.max.x - scissor_rect.min.x) as f32,
//     //     (scissor_rect.max.y - scissor_rect.min.y) as f32,
//     // );

//     // log::warn!(
//     //     "offsetx==========={}, {}, {:?}, {:?}, {:?}, {:?}",
//     //     offsetx,
//     //     offsety,
//     //     view_port_,
//     //     scissor,
//     //     target_view_port,
//     //     view_port
//     // );

//     (rp, view_port_, scissor, (offsetx, offsety))
// }


// impl RenderTarget {
//     // 返回(渲染目标, 是否使用了新的渲染目标)
//     // 如果未分配新的渲染目标，渲染时应该做脏更
//     pub fn get_or_create<'s, T: Iterator<Item = &'s SimpleInOut>>(
//         &'s mut self,
//         atlas_allocator: &SafeAtlasAllocator,
//         as_image: Option<&AsImage>,
//         assets: &TargetCacheMgr,
//         as_image_mark_type: &RenderContextMarkType<AsImage>,
//         post_info: &PostProcessInfo,
//         t_type: &DynTargetType,
//         max_cache: usize,
//         exclude: T,
//         is_force_alloc: bool,
//     ) -> Option<Share<SafeTargetView>> {
//         if is_force_alloc || post_info.has_effect() {
//             match &self.target {
//                 StrongTarget::Asset(r) => return Some(r.0.clone()),
// 				StrongTarget::Raw(r) => return Some(r.0.clone()),
//                 StrongTarget::None => {
//                     let width = (self.bound_box.maxs.x - self.bound_box.mins.x).ceil() as u32;
//                     let height = (self.bound_box.maxs.y - self.bound_box.mins.y).ceil() as u32;

//                     let as_image = match as_image {
//                         Some(r) => r.level.clone(),
//                         None => pi_style::style::AsImage::None,
//                     };

// 					let capacity_overflow = assets.assets.size() as u32 + width * height * 4 > max_cache as u32;
//                     // 如果设置节点为建议缓存，在显存已经超出max_cache的情况下， 不为其分配target， 该相机下的物体直接渲染到父target上
//                     if AsImage1::Advise == as_image && post_info.is_only_as_image(as_image_mark_type) && capacity_overflow
//                     {
//                         return None;
//                     };

//                     // 分配渲染目标
//                     let t = CacheTarget(atlas_allocator.allocate_not_hold(width, height, t_type.has_depth, exclude));
// 					// log::warn!("alloc======={:?}, {:?}, {:?}", width, height, t);

//                     match as_image {
//                         AsImage1::None => {
// 							return Some(t.0);
// 							// // 放入资产管理器，由资产管理器管理
// 							// if capacity_overflow {
// 							// 	// self.target = StrongTarget::Raw(t.clone());
// 							// 	return Some(t.0);
// 							// } else {
// 							// 	let t = assets.push(t.clone());
// 							// 	// self.target = StrongTarget::Asset(t.clone());
// 							// 	return Some(t.0.clone());
// 							// }
							
// 						},
// 						r => {
// 							let t = assets.push(t.clone());
// 							match r {
// 								AsImage1::Advise => {
// 									self.target = StrongTarget::Asset(t.clone());
// 									self.cache = RenderTargetCache::Weak(Share::downgrade(&t))
// 								},
// 								AsImage1::Force => {
// 									self.target = StrongTarget::Asset(t.clone());
// 									self.cache = RenderTargetCache::Strong(t.clone())
// 								},
// 								_ => (),
// 							};
// 							// self.target = StrongTarget::Asset(t.clone());
// 							return Some(t.0.clone());
// 						}
//                     };
                    
                    
//                 }
//             }
//         // // if let None = target {
//         // // 如果后处理效果不只包含as_image，则
//         // if post_info.is_only_as_image(as_image_mark_type) {
//         // 	// || assets.size() as u32 + width * height * 4 <= max_cache as u32

//         // 	return (Some(t.0), true)
//         // }
//         // }
//         } else {
//             None
//         }
//     }
// }


// pub struct LinkNode<T> {
// 	value: Option<T>,
// 	next: Option<Box<LinkNode<T>>>,
// }