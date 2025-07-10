
use std::{mem::transmute, ops::Range};

use pi_world::{prelude::{Entity, OrDefault, ParamSet, Query, SingleRes, SystemParam}, query::QueryError, single_res::SingleResMut};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes, Layer};
use pi_bevy_render_plugin::{asimage_url::RenderTarget as RenderTarget1};

use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{
    node::Node, PiRenderDevice, PiRenderQueue, PiSafeAtlasAllocator, PiScreenTexture, RenderContext, SimpleInOut
};
use pi_futures::BoxFuture;
use pi_null::Null;
use pi_render::components::view::target_alloc::{GetTargetView, SafeAtlasAllocator, SafeTargetView};
use pi_postprocess::prelude::PostprocessTexture;
use pi_render::{
    components::view::target_alloc::ShareTargetView,
    renderer::texture::texture_view::ETextureViewUsage,
    rhi::{
        asset::RenderRes,
        pipeline::RenderPipeline,
        shader::BindLayout,
        texture::{PiRenderDefault, ScreenTexture},
        CommandEncoder,
    },
};

use pi_share::{ShareRefCell, Share};
use pi_style::style::AsImage as AsImage1;
use wgpu::RenderPass;

use crate::{
    components::{
        calc::{EntityKey, WorldMatrix}, draw_obj::{DynTargetType, FboInfo, InstanceIndex}, pass_2d::{CacheTarget, Camera, Draw2DList, DrawElement, ParentPassId, PostProcess, PostProcessInfo, RenderTarget, RenderTargetCache, ScreenTarget, StrongTarget}, user::{Aabb2, AsImage, RenderTargetType, Viewport}
    }, resource::{
        draw_obj::{InstanceContext, RenderState, TargetCacheMgr}, RenderContextMarkType
    }, shader1::batch_meterial::{CameraBind, LayoutUniform, RenderFlagType, TyMeterial, UvUniform}
};
use crate::components::pass_2d::IsSteady;


/// Pass2D 渲染图节点
// #[derive(Clone)]
pub struct Pass2DNode {
    pub pass2d_id: Entity,
	// pub output_target: Option<ShareTargetView>, // 握住一个ShareTargetView， 该view肯呢个占用了分配空间， 当它释放时，空间可能被释放
	pub render_target: Option<ShareTargetView>, // 握住一个ShareTargetView， 该view肯呢个占用了分配空间， 当它释放时，空间可能被释放
}

#[derive(SystemParam)]
pub struct BuildParam<'w> {
	query: ParamSet<'w, (
		
		Query<
			'static,
			(
				&'static Layer,
				&'static mut Camera,
				&'static ParentPassId,
				&'static RenderTarget,
				Option<&'static AsImage>,
				&'static IsSteady,
				&'static mut PostProcess, 
				&'static PostProcessInfo, 
				&'static InstanceIndex,
				&'static Draw2DList,
				&'static mut FboInfo,
				&'static mut RenderTarget1,
				// &'static IsShow, 
				// Entity,
			),
		>,
		
		Query<'static, (&'static InstanceIndex, &'static mut FboInfo, &'static mut RenderTarget1)>,
	)>,
	query_pass_node: Query<
        'w,
        (
            &'static DynTargetType,
            OrDefault<RenderTargetType>,
        ),
    >,
	query_out: Query<'w, &'static mut SimpleInOut>,
	post_resource: SingleRes<'w, PostprocessResource>,
    pipline_assets: SingleRes<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,
	atlas_allocator: SingleRes<'w, PiSafeAtlasAllocator>,
	device: SingleRes<'w, PiRenderDevice>,
	queue: SingleRes<'w, PiRenderQueue>,
	surface: SingleRes<'w, PiScreenTexture>,
	cache_target: SingleRes<'w, TargetCacheMgr>,
	as_image_mark_type: OrInitSingleRes<'w, RenderContextMarkType<AsImage>>,
	instance_draw: OrInitSingleResMut<'w, InstanceContext>,	
}

#[derive(SystemParam)]
pub struct Param<'w> {
	fbo_query: Query<
		'w,
		(&'static FboInfo,
		&'static RenderTarget1),
	>,
	root_query1: Query<'w, &'static Viewport>,
	
    pass2d_query: Query<'w,(&'static Camera, &'static RenderTarget, &'static FboInfo)>,
	post_query: Query<'w, &'static PostProcess>,
    // graph_id_query: Query<'w, &'static GraphId>,
    screen: SingleRes<'w, ScreenTarget>,
    surface: SingleRes<'w, PiScreenTexture>,
	instance_draw: OrInitSingleRes<'w, InstanceContext>,
	query_parent: Query<'w, &'static ParentPassId>,
	render_cross_query: Query<'w, &'static pi_bevy_render_plugin::render_cross::DrawList>,
	
}

impl Pass2DNode {
    pub fn new(pass2d_id: Entity) -> Self {
        Self {
            pass2d_id,
            // last_post_key: EntityKey::default(),
            // rt: None,
			// post_draw: None,
			// output_target: None,
			render_target: None,
            // param,
        }
    }
}

// (, Handle<RenderRes<BindGroup>>)


impl Node for Pass2DNode {

	type BuildParam = BuildParam<'static>;
    type RunParam = Param<'static>;

	type ResetParam = Query<'static, &'static mut SimpleInOut>;
	
	fn reset<'a>(
		&'a mut self,
		// world: &'a mut World,
		param: &'a mut Self::ResetParam,
		_context: RenderContext,
			// input: &'a Self::Input,
		// usage: &'a ParamUsage,
			id: Entity,
			// from: &'a [Entity],
			// to: &'a [Entity],
	) {
		if let Ok(mut r) = param.get_mut(id) {
			if let Some(t) = &mut r.target {
				*t = Share::new(t.downgrade());
			}
		}
	}

	/// 用于给pass2d分配fbo
	fn build<'a>(
		&'a mut self,
		// world: &'a mut pi_world::world::World,
		param: &'a mut Self::BuildParam,
		_context: pi_bevy_render_plugin::RenderContext,
		// input: &'a Self::Input,
		// _usage: &'a pi_bevy_render_plugin::node::ParamUsage,
		id: Entity,
		from: &'a [Entity],
		to: &'a [Entity],
	) -> Result<(), String> {
		let pass2d_id = self.pass2d_id;
		let mut out = param.query_out.get_mut(id).unwrap();
		let out = &mut *out;
		let out = unsafe {transmute::<_, &'static mut SimpleInOut>(out)}; // 非安全防止转换生命周期， 使得query_out可再次使用
		out.target = None; // 删除旧的
		// let mut out = SimpleInOut {
		// 	target: None,
		// 	valid_rect: None,
		// };
		log::debug!("build======{:?}", (pass2d_id, id, from, to));
		// let t1 = std::time::Instant::now();
		// let mut param = query_param_state.get_mut(world);
		// pass2d_id为null， 表示一个空节点， 空节点在全局只会有一个， 用于将所有根节点渲染到屏幕
		
		// let t2 = std::time::Instant::now();
		log::trace!(pass = format!("{:?}", pass2d_id).as_str(); "build graph node, pass: {:?}", pass2d_id);
		// log::warn!("run1======{:?}", pass2d_id);
		let p0 = param.query.p0();
		let (layer, 
			mut camera,
			parent_pass2d_id,
			render_target, 
			as_image,
			is_steady,
			mut post_process, 
			post_process_info,
			instance_index,
			list0,
			mut fbo_info, mut out_target) = match p0.get_mut(pass2d_id) {
			Ok(r) if r.0.layer() > 0 => r,
			_ => return Ok(()),
		};

		// 非fbo节点， 不build
		if !parent_pass2d_id.0.is_null() && !post_process_info.has_effect() {
			return Ok(());
		}
		
		log::trace!(pass = format!("{:?}", pass2d_id).as_str(); "build graph node, instance_index: {:?}, has_effect: {:?},pass2d_id: {:?}", instance_index, post_process_info.has_effect(), pass2d_id);

		match &**param.surface {
			Some(r) => r,
			_ => return Ok(()),
		};

		let (t_type, last_rt_type) = {
			match param.query_pass_node.get(layer.root()) {
				Ok(r) => (
					r.0.clone(),
					r.1.clone()
				),
				_ => {
					return Ok(())
				}
			}
		}; 

		// SAFE: 保证渲染图并行时不会访问同时访问同一个实体的renderTarget，这里的转换是安全的
		let render_target = unsafe { &mut *(render_target as *const RenderTarget as *mut RenderTarget) };
		// let t3 = std::time::Instant::now();
		// log::warn!("graph build======{:?}, {:?}, {:?}, {:?}", pass2d_id, list.transparent, list.opaque, &render_target.bound_box);
		// log::warn!("run graph4==============, pass2d_id: {:?}, input count: {}, opaque: {}, transparent: {}, is_active: {:?}, is_changed: {:?}, opaque_list: {:?}, transparent_list: {:?}, view_port: {:?}, render_target: {:?}", pass2d_id, input.0.len(), list.opaque.len(), list.transparent.len(), camera.is_active, camera.is_change, &list.opaque, &list.transparent, &camera.view_port, &render_target.target);
		
		// if content_box.layout.width() >= 700.0 && content_box.layout.height() >= 910.0 {
		// 	println!("pass1, {:?}", (pass2d_id, camera.is_active, parent_pass2d_id.is_null(), list0.instance_range.len(),  content_box.layout.width(), content_box.layout.height()));
		// }
		let is_not_only_as_image = post_process_info.is_not_only_as_image(&param.as_image_mark_type);
		log::trace!(pass = format!("{:?}", pass2d_id).as_str();"build graph node1, pass2d_id: {pass2d_id:?}, \nparent_pass2d_id: {:?}, \nis_active: {:?}, \nis_changed: {:?}, \nview_port: {:?}, \nfrom: {from:?}, \nto: {to:?}", parent_pass2d_id, camera.is_render_own, camera.draw_changed, &camera.view_port);

		let mut render_to_fbo = false;
		let (offsetx, offsety) = (
			render_target.bound_box.mins.x - camera.view_port.mins.x,
			render_target.bound_box.mins.y - camera.view_port.mins.y,
		);
		let (view_port_w, view_port_h) = (
			camera.view_port.maxs.x - camera.view_port.mins.x,
			camera.view_port.maxs.y - camera.view_port.mins.y,
		);
		// let (fbo_width, fbo_height) = if parent_pass2d_id.0.is_null() {
		// 	// 根节点， fbo为视口大小
		// 	let view_port = param.query_view_port.get(pass2d_id).unwrap();
		// 	(
		// 		view_port.maxs.x - view_port.mins.x,
		// 		view_port.maxs.y - view_port.mins.y,
		// 	)
		// } else{
		// 	(view_port_w, view_port_h)
		// };
		// let next_target = &mut param.temp_next_target.0;

		// if list.opaque.len() > 0 || list.transparent.len() > 0 {
		let catch_target = match &render_target.target {
			StrongTarget::Asset(r) => Some(r.clone()),
			// StrongTarget::Raw(r) => Some(r.0.clone()),
			_ => None,
		};

		let mut r_target = None;

		if let Some(catch_target) = catch_target {
			out.target = Some(catch_target.clone()); // 缓存fbo
			fbo_info.post_draw = None; // 不进行后处理， 因为渲染上下文未改变， 并且渲染结果已经缓存
			out.valid_rect = Some((offsetx as u32, offsety as u32, view_port_w as u32, view_port_h as u32));
			// 根节点， 不允许设置后处理，其catch_target就是r_target
			// 根节点有在原有fbo上渲染部分的需求， 因此将catch_target赋值给r_target
			if parent_pass2d_id.0.is_null() {
				r_target = Some(catch_target); 
			} 
		} else if camera.is_render_own || parent_pass2d_id.0.is_null() {
			if parent_pass2d_id.0.is_null() && !post_process_info.has_effect() && RenderTargetType::Screen == last_rt_type {

			} else if let Some(input_fbo) = is_only_one_pass(from, &mut param.query_out, &param.instance_draw, &list0.instance_range, (render_target.bound_box.maxs.x - render_target.bound_box.mins.x) as u32, (render_target.bound_box.maxs.x - render_target.bound_box.mins.x) as u32) {
				log::debug!("is_only_one_pass================={:?}", pass2d_id);
				// 如果只有一个输入，并且draw2dList中也只存在一个渲染对象(该渲染对象一定是将输入fb拷贝到目标上)
				// 此时， 可直接将输入作为输出
				r_target = input_fbo.target.clone();
				camera.is_render_own = false; // 自身不渲染（渲染结果跟输入完全一样， 直接使用了输入fbo的结果）
				render_to_fbo = true;
			} else {
				// log::warn!("get_or_create================={:?}", (pass2d_id, _id, _from, ));
				// 否则渲染到临时fbo上
				match render_target.get_or_create(
					&param.atlas_allocator,
					as_image,
					&param.cache_target,
					&param.as_image_mark_type,
					post_process_info,
					&t_type,
					16 * 1024 * 1024, // 默认最多缓存16M的target，可配置？TODO
					param.query_out.iter(),
					!is_not_only_as_image,
					is_steady.0,
				) {
					
					Some(r) => {
						
						// for i in input.0.values() {
						// 	if let Some(t) = &i.target {
						// 		log::debug!("alloc input =============={:?}", (pass2d_id, r.index, &t.target().colors[0].1));
						// 	} else {
						// 		log::debug!("alloc input =============={:?}", (pass2d_id, false));
						// 	}
							
						// }
						// next_target.clear();
						render_to_fbo = true;
						log::debug!("alloc rendertarget========{:?}", (self.pass2d_id, id, from, r.target().colors[0].0.id, r.rect()));
						// log::debug!("build========{:?}", (pass2d_id, &r.target().colors[0].0));
						r_target = Some(r.clone());
						

						RenderPassTarget::Fbo(r)
					}
					None => {
						// next_target.clear();
						// log::debug!("none==============={:?}", pass2d_id);
						// 不进行渲染（可能由父节点对它进行渲染, 也可能渲染目标尺寸为0）
						return Ok(());
					}
				};

				
			};
			// let t4 = std::time::Instant::now();

			out.valid_rect = Some((offsetx as u32, offsety as u32, view_port_w as u32, view_port_h as u32));
			if let (Some(rt), true) = (&mut r_target, render_to_fbo) {
				if is_not_only_as_image {
					let mut target = PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default());
					let rect: guillotiere::euclid::Box2D<i32, guillotiere::euclid::UnknownUnit> = rt.rect().clone();

					let dst_size = if parent_pass2d_id.0.is_null() {
						// 根节点必须整个target做后处理
						target.use_x = rect.min.x as u32; // TODO(浮点误差？)
						target.use_y = rect.min.y as u32;
						(rect.max.x as u32 - rect.min.x as u32, rect.max.y as u32 - rect.min.y as u32)
					} else {
						// 其他节点只对脏区域做后处理
						target.use_x = rect.min.x as u32 + offsetx as u32; // TODO(浮点误差？)
						target.use_y = rect.min.y as u32 + offsety as u32;
						(view_port_w as u32, view_port_h as u32)
					};

					// log::warn!("dst_size============{:?}, {:?}", dst_size, &post_info.effect_mark);

					target.use_w = dst_size.0;
					target.use_h = dst_size.1;
					
					
					// 渲染后处理
					if let Ok(post_draw) = post_process.calc(
						16, 
						&param.device, 
						&param.queue, 
						// target,
						PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default()),
						dst_size,
						&param.atlas_allocator,
						&param.post_resource.resources,
						&param.pipline_assets,
						t_type.no_depth,
						wgpu::TextureFormat::pi_render_default(),
					) {
						if let ETextureViewUsage::SRT(post_target) = post_draw.1.view {
							let target_size = ((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32);

							let target = PostprocessTexture::from_share_target(post_target.clone(), wgpu::TextureFormat::pi_render_default());
							let final_draw = if let Some(draw_obj) = post_process.draw_final(
								&param.device,
								&param.queue,
								WorldMatrix::default().as_slice(),
								0.0,
								&param.atlas_allocator,
								&target,
								target_size,
								&param.post_resource.resources,
								&param.pipline_assets,
								wgpu::ColorTargetState {
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
								},
								Some(pi_render::renderer::pipeline::DepthStencilState {
									format: wgpu::TextureFormat::Depth32Float,
									depth_write_enabled: true,
									depth_compare: wgpu::CompareFunction::GreaterEqual,
									stencil: wgpu::StencilState::default(),
									bias: pi_render::renderer::pipeline::DepthBiasState::default(),
								}),
								t_type.no_depth,
								wgpu::TextureFormat::pi_render_default(),
							) {
								Some(draw_obj)
							} else {
								None
							};
							self.render_target = Some(rt.clone());


							out.valid_rect = None;


							let final_target = render_target.get_or_create(
								&param.atlas_allocator,
								as_image,
								&param.cache_target,
								&param.as_image_mark_type,
								post_process_info,
								&t_type,
								16 * 1024 * 1024, // 默认最多缓存16M的target，可配置？TODO
								[post_target.clone()].iter(),
								true,
								is_steady.0,
							).unwrap();

							log::debug!("alloc outputtarget========{:?}", (self.pass2d_id, final_target.target().colors[0].0.id, final_target.rect()));
							// let final_target = param.atlas_allocator.allocate( 
							// 	target_size.0, 
							// 	target_size.1, 
							// 	t_type.has_depth, 
							// 	[post_target.clone()].iter()
							// );
							// log::warn!("build1========{:?}", (pass2d_id, &final_target.target().colors[0].0));

							if Share::ptr_eq(&final_target.target().colors[0].0, &post_target.target().colors[0].0) {
								panic!("pass!!! ========={:?}", pass2d_id);
							}

							out.target = Some(final_target);

							log::trace!("post1111============={:?}, {:?}", pass2d_id, final_draw.is_some());
							match final_draw {
								Some(r) => {
									fbo_info.post_draw = Some((post_draw.0, r));
								},
								None => fbo_info.post_draw = None,
							}
						}
					};
				} else {
					log::trace!("post222============={:?}", pass2d_id);
					out.target = r_target.clone();
				}
				// let t5 = std::time::Instant::now();
				// println!("build1============{:?}", (t2 - t1, t3 - t2, t4 - t3, t5 - t4));
			}
		}
		
		// let t6 = std::time::Instant::now();
		if let Some(as_image) = as_image {
			if as_image.level != pi_style::style::AsImage::Force {
				log::trace!("as_image=============== {:?}", (pass2d_id, as_image.level));
				// 每帧都清理掉render_target.target， 避免握住无法释放
				render_target.target = StrongTarget::None;
			}
		}

		if let (true, Some(target)) = (!list0.clear_instance.is_null(), &r_target) {
			// 旧的fbo与新的fbo不同， 或区域不同， 需要重新设置清屏实例数据
			let mut is_set_clear = false;
			if let Some(fbo) = &fbo_info.fbo {
				let rect1 = fbo.rect_with_border();
				let rect2 = target.rect_with_border();
				if rect1 != rect2 || !Share::ptr_eq(&fbo.target().colors[0].0 , &target.target().colors[0].0) || camera.view_port != camera.old_view_port{
					is_set_clear = true;
				}
			} else {
				is_set_clear = true;
			}
			if is_set_clear {
				// 重新设置清屏范围
				let rect = target.rect_with_border();
				let (x, x1, y, y1) = (
					if offsetx == 0.0 {rect.min.x as f32}else {rect.min.x as f32 + 1.0 - offsetx},
					if render_target.bound_box.maxs.x == camera.view_port.maxs.x {rect.max.x as f32}else {rect.max.x as f32 - 1.0 - (render_target.bound_box.maxs.x - camera.view_port.maxs.x)},
					if render_target.bound_box.maxs.y == camera.view_port.maxs.y {rect.max.y as f32}else {rect.max.y as f32 - 1.0 - (render_target.bound_box.maxs.y - camera.view_port.maxs.y)},
					if offsety == 0.0 {rect.min.y as f32}else {rect.min.y as f32 + 1.0 - offsety},
				);
				let (xmin, xmax, ymin, ymax) = (
					x/target.target().width as f32 * 2.0 - 1.0,
					x1/target.target().width as f32 * 2.0 - 1.0,
					-(y/target.target().height as f32 * 2.0 - 1.0),
					-(y1/target.target().height as f32 * 2.0 - 1.0), // y轴需要翻转
				);

				// if parent_pass2d_id.0.is_null() {
				// 	log::debug!("clear_rect=============== {:?}", (pass2d_id, list0.clear_instance / 224, rect, (target.target().width, target.target().height), xmin, ymin, xmax, ymax, &camera.view_port, &render_target.bound_box, &rect));
				// }
				
				// set_matrix(
				// 	&WorldMatrix::default(), 
				// 	&Aabb2::new(Point2::new(xmin, ymin), 
				// 	Point2::new(xmax, ymax)), 
				// 	&mut param.instance_draw.instance_data.instance_data_mut(list0.clear_instance)
				// );

				param.instance_draw.instance_data.instance_data_mut(list0.clear_instance)
					.set_data(&LayoutUniform(&[xmin, ymin, xmax - xmin, ymax - ymin]));
				// param.instance_draw.instance_data.instance_data_mut(list0.clear_instance).set_data(&QuadUniform(&[
				// 	xmin, ymin,
				// 	xmin, ymax,			
				// 	xmax, ymax,
				// 	xmax, ymin,
				// ]));
			}
		}
		
		// 旧的fbo输出与新的不同， 需要重新设置uv
		let has_custom_post = match as_image {
			Some(r) => !r.old_pass2d_graph_id.is_null(),
			None => false,
		};
		// 如果存在自定义后处理， 会在copy节点中比较
		if !has_custom_post {
			compare_target( &out.target, &mut out_target, Some(render_target), instance_index, &mut param.instance_draw);
			log::trace!("out.target======{:?}", (pass2d_id, r_target.is_some(), out.target.is_some()));
			out_target.0 = out.target.as_ref().map(|r| {Share::new(r.downgrade())}); // 设置到组件上， 后续批处理需要用到
		} 

		

		// if instance_index.start == 125 * 240 {
		// 	println!("visibility=============== {:?}", (pass2d_id, instance_index.start, visibility,  out.target.is_none(), list0.instance_range.len() > 0));
		// }
		fbo_info.fbo = r_target.as_ref().map(|r| {Share::new(r.downgrade())});
		// log::debug!("target==============={:?}", (pass2d_id, fbo_info.fbo.is_some(), camera.view_port));
		
		// if content_box.layout.width() >= 700.0 && content_box.layout.height() >= 910.0 {
		// 	println!("pass2, {:?}", (pass2d_id, fbo_info.out.is_some()));
		// }
		// let t7 = std::time::Instant::now();
		// use pi_slotmap::Key;
		// if pass2d_id.index() == 4 {
		// 	log::error!("id============{:?}", pass2d_id);
		// }
		
		Ok(())
	}

    fn run<'a>(
        &'a mut self,
        // world: &'a World,
        param: &'a Self::RunParam,
        _context: RenderContext,
        commands: ShareRefCell<CommandEncoder>,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<(), String>> {
        let pass2d_id = self.pass2d_id;
		// let rt = self.rt.take();
		// let post_draw = self.post_draw.take();
		log::debug!("draw1==={:?}", (pass2d_id));
        Box::pin(async move {
			log::debug!("run0======{:?}", pass2d_id);
            // let query_param = query_param_state.get(world);
            // log::trace!(pass = format!("{:?}", pass2d_id).as_str(); "run graph node, ", param.surface.is_some());
			

			let draw_range = if EntityKey(pass2d_id).is_null() {
				param.instance_draw.draw_screen_range.clone()
			} else {
				let (camera, _render_target, _fbo_info) = match param.pass2d_query.get(pass2d_id) {
					Ok(r) => r,
					Err(_) => return Ok(()),
				};
				camera.draw_range.clone()
			};
			if draw_range.len() == 0 {
				return Ok(());
			}
			let surface = match &**param.surface {
				Some(r) => r,
				_ => {
					return Ok(())
				}
			};
			
			// log::warn!("draw=========================");

			log::debug!("draw_elements======{:?}", (pass2d_id, &draw_range));
			let mut commands = commands.borrow_mut();

			let (mut rt, mut rp, mut pre_fbo_pass_id, mut fbo_view_port, mut fbo_camera_viewport);
			
			let mut i = draw_range.start;
			loop {
				if i >=  draw_range.end { // 超出渲染范围， 返回
					return Ok(());
				}
				let mut pass_query = Err(QueryError::NoMatchArchetype);
				let element = &param.instance_draw.draw_list[i];
				rt = if EntityKey(element.1).is_null() {
					
					RPTarget::Screen(param.surface.as_ref().unwrap(), &None)
				} else {
					pass_query = param.pass2d_query.get(element.1);
					let fbo1 = match param.pass2d_query.get(element.1) {
						Ok(r) => if !r.0.is_render_own {
							// log::warn!("is_render_own false====================={:?}", element.1);
							// 自身不渲染， 则跳过
							i += 1;
							continue;
						} else {
							r.2
							// log::warn!("is_render_own true====================={:?}", element.1);
						}
						_ => continue,
					};
					match fbo1.fbo.as_ref() {
						Some(r) => {
							// log::warn!("create_rp0============={:?}", &r.target().colors[0].1);
							RPTarget::Fbo(r)
						},
						None => {
							// log::warn!("screen============={:?}", element.1);
							RPTarget::Screen(&surface, &param.screen.depth)
						}
					}
				};
				
				log::debug!("create_rp1============={:?}", (pass2d_id, &element.1, param.query_parent.get(element.1), &rt));
				rp = create_rp(
					&rt,
					&mut commands,
					None,
				);
				pre_fbo_pass_id = element.1;
				(fbo_view_port, fbo_camera_viewport) = if let Ok((camera, render_target, _)) = pass_query {
					(calc_view_port(&rt, &camera.view_port, &render_target.bound_box), &camera.view_port)
				} else {
					(
						(param.screen.aabb.mins.x, param.screen.aabb.mins.y, param.screen.aabb.maxs.x - param.screen.aabb.mins.x, param.screen.aabb.maxs.y - param.screen.aabb.mins.y),
						&param.screen.aabb,
					)
				};
				break;
			}
			

			let mut pre_pass = EntityKey::null();
			let mut render_state = RenderState {
				reset: true,
				pipeline: param.instance_draw.common_pipeline.clone(),
				texture: param.instance_draw.batch_texture.default_texture_group.clone(),
			};
			

			// 本帧渲染是否设置过相机
			let mut camera_is_set = false;

			
			// let mut set_camera = false;
			
			// log::warn!("draw_list============={:?}", param.instance_draw.draw_list.len());
			// log::warn!("draw_list============={:?}", (param.instance_draw.draw_list.len(), &param.instance_draw.draw_list));
			// let mut ii = 0;
			for i in i..draw_range.end {
				let element = &param.instance_draw.draw_list[i];
				// log::warn!("element============={:?}, {:?}", &element.1, &pre_fbo_pass_id);
				let mut pass_query = Err(QueryError::NoMatchArchetype);
				if let DrawElement::DrawPost(_post_range) = &element.0 {
					// 如果是后处理， 不需要在此处创建rp， 后处理本身会创建rp， 并且不能用element.1判断相机是否渲染自身， 因为一个DrawPost包含前面的多个pass的后处理
				} else if pre_fbo_pass_id != element.1 {
					// ii += 1;
					let t: RPTarget<'_> = if EntityKey(element.1).is_null() {
						// log::warn!("Screen=============depth none");
						RPTarget::Screen(&surface, &None)
					} else {
						pass_query = param.pass2d_query.get(element.1);
						let fbo1 = match pass_query {
							Ok(r) => if !r.0.is_render_own {
								// log::warn!("is_render_own false1====================={:?}", element.1);
								// 自身不渲染， 则跳过
								continue;
							} else {
								r.2
								// log::warn!("is_render_own true1====================={:?}", element.1);
							},
							_ => continue,
						};
						match fbo1.fbo.as_ref() {
							Some(r) => {
								// log::warn!("create_rp1============={:?}", (element.1, &r.target().colors[0].1));
								RPTarget::Fbo(r)
							},
							None => {
								// log::warn!("screen1============={:?}", element.1);
								RPTarget::Screen(&surface, &param.screen.depth)
							}
						}
					};

					if !t.eq(&rt) {
						log::debug!("create_rp2============={:?}", (pass2d_id, &element.1, param.query_parent.get(element.1), &t));
						{let _a = rp;} // 释放
						rp = create_rp(
							&t,
							&mut commands,
							None,
						);
						render_state.reset = true;

						// log::debug!("create_rp1============={:?}", element.1);
					}
					rt = t;

					if let Ok((camera, render_target, _)) = pass_query {
						(fbo_view_port, fbo_camera_viewport) = (calc_view_port(&rt, &camera.view_port, &render_target.bound_box), &camera.view_port);
						// log::warn!("fbo_view_port============={:?}", (&element.1, fbo_view_port, &render_target.bound_box, &camera.view_port));
					};
					pre_fbo_pass_id = element.1;
					
				}
				
				match &element.0 {
					DrawElement::Clear { draw_state, is_active } => {
						// log::warn!("clear======={:?}, {:?}, {:?}", element.1, is_active, draw_state.instance_data_range.start / 224);
						if !*is_active {
							// log::trace!("is_active======{:?}", pass);
							continue; // 没有激活的fbo， 不清屏
						}
						param.instance_draw.set_pipeline(&mut rp, draw_state, &mut render_state);
						// log::warn!("clear======={:?}, {:?}, {:?}, {:?}, {:?}", pass, element.1, draw_state.instance_data_range.start/224..draw_state.instance_data_range.end/224, draw_state.instance_data_range.start..draw_state.instance_data_range.end, param.instance_draw.instance_data.data.len());
						// 批量清屏， 每个清屏实例的布局数据描述了清理区域， 视口设置为整张fbo
						if let RPTarget::Fbo(rt) = rt {
							// log::warn!("clear view port: {:?}", (element.1, rt.rect_with_border()));
							// 清屏视口
							rp.set_viewport(0.0, 0.0, rt.target().width as f32, rt.target().height as f32, 0.0, 1.0);
						}
						let group = param.instance_draw.default_camera.get_group();
						rp.set_bind_group(CameraBind::set(), &**group.bind_group, group.offsets);

						param.instance_draw.draw(&mut rp, draw_state, &mut render_state);
					},
					DrawElement::DrawInstance { draw_state, pass, .. } => {
						// use pi_slotmap::Key;
						// if pass.index() == 67 {
							// log::debug!("draw_elements1======{:?}", (pass2d_id, element.1, draw_state.instance_data_range.start/224..draw_state.instance_data_range.end/224));
						// }
						// log::debug!("DrawInstance======={:?}, {:?}, {:?}, {:?}", pass, &draw_state.texture_bind_group, element.1, ( draw_state.instance_data_range.start/224..draw_state.instance_data_range.end/224));
						// log::debug!("DrawInstance======={:?}, {:?}, {:?}", pass, element.1, draw_state.instance_data_range.start/224..draw_state.instance_data_range.end/224);
						// 设置相机
						if EntityKey(element.1).is_null() {
							// 将根的内容拷贝到屏幕上
							if let Ok(view_port) = param.root_query1.get(*pass) {
								param.instance_draw.set_pipeline(&mut rp, draw_state, &mut render_state);
								// log::warn!("root view port: {:?}", (pass, element.1, &view_port));
								rp.set_viewport(view_port.mins.x, view_port.mins.y, view_port.maxs.x - view_port.mins.x, view_port.maxs.y - view_port.mins.y, 0.0, 1.0);
								// 如果没有设置相机， 则随便设置一个（这里仅仅是将根节点的内容拷贝到屏幕， 实际上不会用到相机， 但是为了统一pipeline， 需要设置一个）
								if !camera_is_set {
									let group = param.instance_draw.default_camera.get_group();
									rp.set_bind_group(CameraBind::set(), &**group.bind_group, group.offsets);
									// set_camera = true;
									camera_is_set = true;
								}
								param.instance_draw.draw(&mut rp, draw_state, &mut render_state);
							} else {
								unreachable!();
							}
						} else {
							if let Ok((camera, _render_target, _)) = param.pass2d_query.get(*pass) {
								if !camera.is_render_own {
									// log::debug!("is not active DrawInstance======={:?}, {:?}", pass, element.1);
									continue;
								}
								param.instance_draw.set_pipeline(&mut rp, draw_state, &mut render_state);
								if pre_pass != EntityKey(*pass) {
									log::trace!("change pass======{:?}", pass);
									if let Some(c) = &camera.bind_group {
										c.set(&mut rp, CameraBind::set());
										// set_camera = true;
									}
									let view_port = (
										(fbo_view_port.0 as f32 - fbo_camera_viewport.mins.x) + camera.view_port.mins.x,
										(fbo_view_port.1 as f32 - fbo_camera_viewport.mins.y) + camera.view_port.mins.y,
										camera.view_port.maxs.x - camera.view_port.mins.x,
										camera.view_port.maxs.y - camera.view_port.mins.y,
									);
	
									// let view_port = calc_view_port(&rt, &camera.view_port, &render_target.bound_box);
									// log::warn!("DrawInstance view port: {:?}", (pass, element.1, view_port, camera.view_port, &fbo_camera.view_port, &fbo_view_port));
									rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
									pre_pass = EntityKey(*pass);
								}
								// if pass.index() == 67 {
									// log::debug!("draw_elements2======{:?}", (pass2d_id, element.1, draw_state.instance_data_range.start/224..draw_state.instance_data_range.end/224));
								// }
								param.instance_draw.draw(&mut rp, draw_state, &mut render_state);
								
							} else {
								unreachable!();
							}
							
							
						}
						
						// log::trace!("draw_state========={:?}", draw_state);
						// if !set_camera{
						// 	log::warn!("DrawInstance!============{:?}", (pass, pre_pass, render_state.reset));
						// }
						
						
					},
					DrawElement::DrawPost(post_range) => {
						log::trace!("post1============={:?}", post_range);
						// log::warn!("DrawPost======{:?}", element.1);
						// 处理后处理
						for post_pass_id in param.instance_draw.posts[post_range.clone()].iter() {
							if let Ok((camera, _render_target, _)) = param.pass2d_query.get(*post_pass_id) {
								// 如果目标fbo对应的相机未激活, 不需要渲染
								if !camera.is_render_own {
									continue;
								}
							}
							let (fbo, out_target) = param.fbo_query.get(*post_pass_id).unwrap(); 
							log::trace!("post============={:?}", (post_pass_id, fbo.post_draw.is_some(), out_target.0.is_some()));
							if let (Some((front_draw, final_draw)), Some(final_target)) = (&fbo.post_draw, &out_target.0) {
								log::trace!("post0============={:?}", post_pass_id);
								let post_process = if let Ok(post_process) = param.post_query.get(*post_pass_id) {
									post_process
								} else {
									log::trace!("post1============={:?}", post_pass_id);
									continue;
								};

								
								// log::warn!("post============={:?}", post_pass_id);
								{
									// log::warn!("create_rp post!!!!!!!=============");
									let _a = rp;
								}
								// log::warn!("front_draw===={:?}, {:?}", final_draw, post_pass_id);
								// front_draw
								post_process.draw_front(
									&mut commands,
									&front_draw,
								);

								// final_draw
								let render_target_rect = final_target.rect();
								rp = create_rp_for_fbo1(final_target, &mut commands, None);
								log::debug!("create_rp post============={:?}", (post_pass_id, &final_target.target().colors[0].1));
								let view_port = (
									render_target_rect.min.x as f32, 
									render_target_rect.min.y as f32,
									render_target_rect.max.x as f32 - render_target_rect.min.x as f32, 
									render_target_rect.max.y as f32 - render_target_rect.min.y as f32
								);
								// log::warn!("post view port: {:?}", (element.1, &view_port));
								rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
								final_draw.draw(&mut rp);

								rt = RPTarget::Fbo(final_target);
								// log::warn!("final_draw===={:?}, {:?}", final_draw, view_port);
							}
						}
						render_state.reset = true;
					},
					DrawElement::GraphDrawList {id, .. } => {
						if let Ok(r) = param.render_cross_query.get(id.0) {
							// 需要重新创建rp， 清理深度
							{let _a = rp;}
							rp = create_rp(
								&rt,
								&mut commands,
								None,
							);
							pi_render::renderer::draw_obj_list::DrawList::render(r.draw_list.list.as_slice(), &mut rp);
							// 结束后， 重置rp状态， 后续渲染也需要清理深度
							pre_fbo_pass_id = Null::null();
							rt = RPTarget::None;
						}
					},
					
				}
			}

            Ok(())
        })
    }
}

#[derive(Clone)]
pub enum RenderPassTarget {
    Fbo(ShareTargetView),
    // Screen(&'a ScreenTexture, &'a Option<Handle<RenderRes<wgpu::TextureView>>>),
	Screen
}

#[derive(Clone, Debug)]
pub enum RPTarget<'a> {
    Fbo(&'a ShareTargetView),
    Screen(&'a ScreenTexture, &'a Option<Handle<RenderRes<wgpu::TextureView>>>),
	None,
}

impl<'a> RPTarget<'a>{
	fn eq(&self, other: &RPTarget<'a>) -> bool {
		match (self, other) {
		    (RPTarget::Fbo(a), RPTarget::Fbo(b)) => Share::ptr_eq(&a.target().colors[0].0, &b.target().colors[0].0),
			(RPTarget::Screen(_, None), RPTarget::Screen(_, None)) | (RPTarget::Screen(_, Some(_)), RPTarget::Screen(_, Some(_)))  => true,
			_ => false
		}
	}
}

// 返回renderpass， view_port， clear_port
pub fn create_rp<'a>(
    rt: &RPTarget<'a>,
    commands: &'a mut CommandEncoder,
    ops: Option<wgpu::Operations<wgpu::Color>>,
) -> RenderPass<'a> {
    match rt {
        RPTarget::Screen(surface, depth) => {
            create_screen_rp(surface, depth, commands, ops)
        }
        RPTarget::Fbo(rt) => {
            // 渲染到临时的fbo上
            // let mut r = last_rt.target.as_ref().unwrap();
            // if let Some(t) = rt {
            //     r = t;
            // }
			// fbo永远不清屏
            create_rp_for_fbo1(rt, commands, None)
        }
		_ => unreachable!()
    }
}
// 返回renderpass， view_port， clear_port
pub fn create_screen_rp<'a>(
    surface: &'a ScreenTexture,
	depth: &'a Option<Handle<RenderRes<wgpu::TextureView>>>,
    commands: &'a mut CommandEncoder,
    // target_view_port: &Aabb2, // 渲染目标对应的view_port;
    // last_rt: &'a RenderTarget,
    // surface: &'a ScreenTexture,
    ops: Option<wgpu::Operations<wgpu::Color>>,
) -> RenderPass<'a> {
	log::trace!("create_screen_rp===={:?}", depth.is_some());
	// 渲染到屏幕上
	let ops = match ops {
		Some(r) => r,
		None => wgpu::Operations {
			// load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
			load: wgpu::LoadOp::Load,
			store: wgpu::StoreOp::Store,
		},
	};
	commands.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: None,
		color_attachments: &[Some(wgpu::RenderPassColorAttachment {
			resolve_target: None,
			ops,
			view: surface.view.as_ref().unwrap(),
		})],
		depth_stencil_attachment:  match depth {
			Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
				stencil_ops: None,
				// 渲染到屏幕，不需要清理深度，也不需要写深度
				depth_ops: Some(wgpu::Operations {
					load: wgpu::LoadOp::Clear(0.0),
					store: wgpu::StoreOp::Discard,
				}),
				view: r,
			}),
			None => None,
		},
		timestamp_writes: None,
		occlusion_query_set: None,
	})
}

#[inline]
pub fn calc_view_port<'a>(
	rt: &RPTarget,
	view_port: &Aabb2,
    target_view_port: &Aabb2,
) -> (f32, f32, f32, f32) {
	match rt {
        RPTarget::Screen(_surface, _depth) => {
            (
				view_port.mins.x,
				view_port.mins.y,
				view_port.maxs.x - view_port.mins.x,
				view_port.maxs.y - view_port.mins.y,
			)
        }
        RPTarget::Fbo(rt) => calc_fbo_view_port(rt, view_port, target_view_port),
		_ => unreachable!()
    }
}

pub fn calc_fbo_view_port<'a>(
	rt: &'a ShareTargetView,
	view_port: &Aabb2,
    target_view_port: &Aabb2,
) -> (f32, f32, f32, f32) {
	// fbo永远不清屏
	let rect = rt.rect();
	let (offsetx, offsety) = (view_port.mins.x - target_view_port.mins.x, view_port.mins.y - target_view_port.mins.y);
	(
		rect.min.x as f32 + offsetx,
		rect.min.y as f32 + offsety,
		view_port.maxs.x - view_port.mins.x,
		view_port.maxs.y - view_port.mins.y,
	)
}

pub fn create_rp_for_fbo1<'a>(
	r: &'a ShareTargetView,
	commands: &'a mut CommandEncoder,
	ops: Option<wgpu::Operations<wgpu::Color>>,) -> RenderPass<'a> {
	let ops = match ops {
		Some(r) => r,
		None => wgpu::Operations {
			// load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.0}),
			load: wgpu::LoadOp::Load,
			store: wgpu::StoreOp::Store,
		},
	};

	log::trace!("create_rp_for_fbo1===={:?}", r.target().depth.is_some());
	commands.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: None,
		color_attachments: r
			.target()
			.colors
			.iter()
			.map(|view| {
				Some(wgpu::RenderPassColorAttachment {
					resolve_target: None,
					ops,
					view: &view.0,
				})
			})
			.collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
			.as_slice(),
		depth_stencil_attachment: match &r.target().depth {
			Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
				stencil_ops: None,
				depth_ops: Some(wgpu::Operations {
					load: wgpu::LoadOp::Clear(0.0),
					store: wgpu::StoreOp::Store,
				}),
				view: &r.0,
			}),
			None => None,
		},
		timestamp_writes: None,
		occlusion_query_set: None,
	})
}

pub fn create_rp_for_fbo<'a>(
    r: &'a ShareTargetView,
    commands: &'a mut CommandEncoder,
    view_port: &Aabb2,
    target_view_port: &Aabb2,
    ops: Option<wgpu::Operations<wgpu::Color>>,
) -> (RenderPass<'a>, (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32)) {
    let ops = match ops {
        Some(r) => r,
        None => wgpu::Operations {
            // load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.0}),
            load: wgpu::LoadOp::Load,
            store: wgpu::StoreOp::Store,
        },
    };
    let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: r
            .target()
            .colors
            .iter()
            .map(|view| {
                Some(wgpu::RenderPassColorAttachment {
                    resolve_target: None,
                    ops,
                    view: &view.0,
                })
            })
            .collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
            .as_slice(),
        depth_stencil_attachment: match &r.target().depth {
            Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
                stencil_ops: None,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.0),
                    store: wgpu::StoreOp::Store,
                }),
                view: &r.0,
            }),
            None => None,
        },
		timestamp_writes: None,
        occlusion_query_set: None,
    });
    let rect = r.rect();
    let (offsetx, offsety) = (view_port.mins.x - target_view_port.mins.x, view_port.mins.y - target_view_port.mins.y);
    let view_port_ = (
        rect.min.x as f32 + offsetx,
        rect.min.y as f32 + offsety,
        view_port.maxs.x - view_port.mins.x,
        view_port.maxs.y - view_port.mins.y,
    );
    // 如果
    let scissor = if target_view_port.mins.x == view_port.mins.x
        && target_view_port.maxs.x == view_port.maxs.x
        && target_view_port.mins.y == view_port.mins.y
        && target_view_port.maxs.y == view_port.maxs.y
    {
        // 如果target对应的视口区域跟当前需要渲染的视口区域一样，则设置裁剪口为border区域（因为这很可能是第一次渲染该target，分配出来的fbo中的数据是随机的，如果不清理边框区域，边缘可能会有黑线）
       let rect_border: &guillotiere::euclid::Box2D<i32, guillotiere::euclid::UnknownUnit> = r.rect_with_border();
	//    log::warn!("rect_with_border========{:?}, {:?}", rect, rect_border);
	   (
        rect_border.min.x as f32,
        rect_border.min.y as f32,
        (rect_border.max.x - rect_border.min.x) as f32,
        (rect_border.max.y - rect_border.min.y) as f32,
    )
    } else {
		// log::warn!("rect_with_border1========{:?}, {:?}, {:?}", rect, target_view_port, view_port);
        // 否则为视口区域
        view_port_
    };

    (rp, view_port_, scissor, (offsetx, offsety))
}

// fn calc_scissor(r: & ShareTargetView, view_port: &Aabb2, target_view_port: &Aabb2) -> (f32, f32, f32, f32) {
// 	if target_view_port.mins.x == view_port.mins.x
// 	&& target_view_port.maxs.x == view_port.maxs.x
// 	&& target_view_port.mins.y == view_port.mins.y
// 	&& target_view_port.maxs.y == view_port.maxs.y
// 	{
// 		// 如果target对应的视口区域跟当前需要渲染的视口区域一样，则设置裁剪口为border区域（因为这很可能是第一次渲染该target，分配出来的fbo中的数据是随机的，如果不清理边框区域，边缘可能会有黑线）
// 		let rect_border: &guillotiere::euclid::Box2D<i32, guillotiere::euclid::UnknownUnit> = r.rect_with_border();
// 		//    log::warn!("rect_with_border========{:?}, {:?}", rect, rect_border);
// 		(
// 		rect_border.min.x as f32,
// 		rect_border.min.y as f32,
// 		(rect_border.max.x - rect_border.min.x) as f32,
// 		(rect_border.max.y - rect_border.min.y) as f32,
// 		)
// 	} else {
// 		// log::warn!("rect_with_border1========{:?}, {:?}, {:?}", rect, target_view_port, view_port);
// 		// 否则为视口区域
// 		calc_fbo_view_port(r, view_port, target_view_port)
// 	}
// }


impl RenderTarget {
    // 返回(渲染目标, 是否使用了新的渲染目标)
    // 如果未分配新的渲染目标，渲染时应该做脏更
    pub fn get_or_create<'s, G: GetTargetView, T: Iterator<Item=G>>(
        &'s mut self,
        atlas_allocator: &SafeAtlasAllocator,
        as_image: Option<&AsImage>,
        assets: &TargetCacheMgr,
        as_image_mark_type: &RenderContextMarkType<AsImage>,
        post_info: &PostProcessInfo,
        t_type: &DynTargetType,
        max_cache: usize,
        exclude: T,
        by_catch: bool,
		is_steady: bool,
    ) -> Option<Share<SafeTargetView>> {
        if by_catch {
			match &self.target {
				StrongTarget::Asset(r) => {
					return Some(r.clone())
				},
				// StrongTarget::Raw(r) => return Some(r.0.clone()),
				StrongTarget::None => {
					// 从缓存中取到
					match &self.cache {
						RenderTargetCache::None => (),
						RenderTargetCache::Strong(droper) => return Some(droper.clone()),
						RenderTargetCache::Weak(weak) => {
							if let Some(droper) = weak.upgrade() {
								self.target = StrongTarget::Asset(droper.clone());
								return Some(droper.clone());
							}
						},
					};
					let width = (self.bound_box.maxs.x - self.bound_box.mins.x).ceil() as u32;
					let height = (self.bound_box.maxs.y - self.bound_box.mins.y).ceil() as u32;

					if width == 0 || height == 0 {
						return None;
					}

					let as_image = match as_image {
						Some(r) => r.level.clone(),
						None => pi_style::style::AsImage::None,
					};

					// println!("get_width======: {:?}",( width, height, assets.assets.size(), max_cache));

					let capacity_overflow = assets.0.size() as u32 + width * height * 4 > max_cache as u32;
					// 如果设置节点为建议缓存，在显存已经超出max_cache的情况下， 不为其分配target， 该相机下的物体直接渲染到父target上
					if AsImage1::Advise == as_image && post_info.is_not_only_as_image(as_image_mark_type) && capacity_overflow
					{
						return None;
					};

					// 分配渲染目标
					let t = CacheTarget(atlas_allocator.allocate(width, height, t_type.has_depth, exclude));
					match (as_image, is_steady) {
						(AsImage1::None, false) => {
							
							
							return Some(t.0);
							// // 放入资产管理器，由资产管理器管理
							// if capacity_overflow {
							// 	// self.target = StrongTarget::Raw(t.clone());
							// 	return Some(t.0);
							// } else {
							// 	let t = assets.push(t.clone());
							// 	// self.target = StrongTarget::Asset(t.clone());
							// 	return Some(t.0.clone());
							// }
							
						},
						r => {
							assets.0.push(t.clone());
							match r {
								(AsImage1::Advise, _) | (_, true) => {
									self.target = StrongTarget::Asset(t.0.clone());
									self.cache = RenderTargetCache::Weak(Share::downgrade(&t.0))
								},
								(AsImage1::Force, _) => {
									self.target = StrongTarget::Asset(t.0.clone());
									self.cache = RenderTargetCache::Strong(t.0.clone())
								},
								_ => (),
							};
							// self.target = StrongTarget::Asset(t.clone());
							return Some(t.0);
						}
					};
					
					
				}
			}
		} else {
			let width = (self.bound_box.maxs.x - self.bound_box.mins.x).ceil() as u32;
			let height = (self.bound_box.maxs.y - self.bound_box.mins.y).ceil() as u32;

			if width == 0 || height == 0 {
				return None;
			}
			Some(atlas_allocator.allocate(width, height, t_type.has_depth, exclude))
		}
    }
}


pub fn is_only_one_pass<'a>(from: &[Entity], query: &'a Query<'static, &'static mut SimpleInOut>, instance_draw: &InstanceContext, instance_range: &Range<usize>, view_port_w: u32, view_port_h: u32) -> Option<&'a SimpleInOut> {
	if from.len() == 1 && instance_range.len() == instance_draw.instance_data.alignment {
		// 如果只有一个输入，并且draw2dList中也只存在一个渲染对象(该渲染对象一定是将输入fb拷贝到目标上)
		// 此时， 可直接将输入作为输出
		if let Ok(input_fbo) = query.get(from[0]) {
			if let Some(r) = &input_fbo.target {
				// let (w, h) = match input_fbo.valid_rect {
				// 	Some(rect) => (rect.2, rect.3),
				// 	None => (r.target().width, r.target().height),
				// };
				let (w, h) = (r.target().width, r.target().height);
				if w == view_port_w && h == view_port_h {
					return Some(input_fbo);
				}
			}
		}
		
	}
	None
}

// 用于将外部系统的图节点输出的fbo， 拷贝到RenderTarget1中， 后续才能正常渲染
pub struct CustomCopyNode(pub Entity/*做自定义后处理效果的实体id*/);
impl CustomCopyNode {
	pub fn new(id: Entity) -> Self {
		Self(id)
	}
}

impl Node for CustomCopyNode {
    // type Input = SimpleInOut;
    // type Output = SimpleInOut;

	type BuildParam = (
		Query<'static, (&'static mut RenderTarget1, Option<&'static RenderTarget>, &'static InstanceIndex)>, 
		SingleResMut<'static, InstanceContext>,
		Query<'static, &'static mut SimpleInOut>, 
	);
    type RunParam = ();

	type ResetParam = Query<'static, &'static mut SimpleInOut>;
	
	fn reset<'a>(
		&'a mut self,
		// world: &'a mut World,
		param: &'a mut Self::ResetParam,
		_context: RenderContext,
			// input: &'a Self::Input,
		// usage: &'a ParamUsage,
			id: Entity,
			// from: &'a [Entity],
			// to: &'a [Entity],
	) {
		if let Ok(mut r) = param.get_mut(id) {
			if let Some(t) = &mut r.target {
				*t = Share::new(t.downgrade())
			}
		}
	}

	/// 用于给pass2d分配fbo
	fn build<'a>(
		&'a mut self,
		// world: &'a mut pi_world::world::World,
		param: &'a mut Self::BuildParam,
		_context: pi_bevy_render_plugin::RenderContext,
		_id: Entity,
		_from: &'a [Entity],
		_to: &'a [Entity],
	) -> Result<(), String> {
		if _from.len() == 1 {
			if let Ok(input) = param.2.get_mut(_from[0]) {
				match (&input.target, param.0.get_mut(self.0)) {
					(r, Ok((mut out_target, render_target, instance_index))) => {
						// 比较target是否发生改变， 如果发生改变， 需要重新批处理
						compare_target(r, out_target.bypass_change_detection(), render_target, instance_index, &mut param.1);
						// log::error!("out_target.0================={:?}", (&self.0, &out_target.0));
						out_target.0 = r.as_ref().map(|r| {Share::new(r.downgrade())});
					}
					_ => (),
				}
				return Ok(());
			}
			
		}
		Ok(())
    }

    fn run<'a>(
        &'a mut self,
        // world: &'a World,
        _param: &'a Self::RunParam,
        _context: RenderContext,
        _commands: ShareRefCell<CommandEncoder>,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<(), String>> {
        unreachable!()
    }
	
	
}

fn compare_target(
	target: &Option<ShareTargetView>, 
	out_target: &mut RenderTarget1, 
	render_target: Option<&RenderTarget>, 
	instance_index: &InstanceIndex, 
	instance_context: &mut InstanceContext,
	// node: GraphNodeId,
) {
	// if !node.is_null() {
	// 	log::warn!("input============{:?}", (node, target.is_some()));
	// }
	let index = if instance_index.transparent.is_null() {
		&instance_index.opacity
	} else {
		&instance_index.transparent
	};
	let has_instance = !index.start.is_null() && index.end > index.start;
	if let Some(target) = &target {
		// 旧的fbo输出与新的不同， 需要重新设置uv
		// 旧的fbo输出与新的不同， 需要重新设置uv
		let mut is_set_uv = false;
		if let Some(fbo) = &out_target.0 {
			if !Share::ptr_eq(&fbo.target().colors[0].0 , &target.target().colors[0].0) {
				instance_context.rebatch = true; // 设置rebatch为true， 使得后续重新进行批处理
				// is_set_uv = true; // TODO ， 现阶段有些情况， 设置过uv， 但实例被释放，之后被其他实例占用， 但out_target没有被清理， 导致uv比较可能相等， 后续不能重置uv， 需要优化（设置uv要更新数据到显卡， 性能不好）
			}
			let rect1 = fbo.uv_box();
			let rect2 = target.uv_box();
			if rect1 != rect2 {
				is_set_uv = true;
			}
			// if !node.is_null() {
			// 	log::warn!("input1============{:?}", (node, has_instance, rect1, rect2, is_set_uv,
			// 		!Share::ptr_eq(&fbo.target().colors[0].0 , &target.target().colors[0].0)));
			// }
		} else {
			instance_context.rebatch = true; // 设置rebatch为true， 使得后续重新进行批处理
			is_set_uv = true;
		}
		if is_set_uv {
			if has_instance {
				// uv变化，设置uv
				let mut uv_box = target.uv_box();
				let rect = target.rect().size();
				let (t_w, t_h) = (target.target().width, target.target().height);
				let (w, h) = (rect.width as f32 / t_w as f32, rect.height as f32 / t_h as f32);
				// 修正uv， 渲染目标宽高一定是整数， 但真实的渲染区域尺寸不一定， 修正到精确的渲染区域
				if let Some(render_target) = render_target {
					let mins = &render_target.accurate_bound_box.mins; 
					let maxs = &render_target.accurate_bound_box.maxs; 
					uv_box[0] += mins.x * w;
					uv_box[1] += mins.y * h;
					uv_box[2] += maxs.x * w;
					uv_box[3] += maxs.y * h;
				}
				instance_context.instance_data.instance_data_mut(index.start).set_data(&UvUniform(uv_box.as_slice()));
				// if !node.is_null() {
				// 	log::error!("set uv: {:?}, {:?}, {:?}", uv_box, target.rect(), (node, target.target().colors[0].0.id));
				// }
								
				
			} else {
				instance_context.rebatch = true; // 设置rebatch为true， 使得后续重新进行批处理
			}
		}
	} else if out_target.0.is_some() {
		// 旧的fbo存在， 新的fbo不存在，设置rebatch为true， 使得后续重新进行批处理
		instance_context.rebatch = true;
	}

	
	// 设置实例是否可见
	if has_instance {
		let mut ty = instance_context.instance_data.instance_data_mut(index.start).get_render_ty();
		// 没有分配fbo，设置将渲染无效
		let invaild = target.is_none();
		let invaild = (unsafe {transmute::<_, u8>(invaild)} as usize) << (RenderFlagType::Invalid as usize);
		// if !node.is_null() {
		// 	log::error!("!!!!!!!!!!!!========================={:?}", (
		// 		node, 
		// 		target.is_none(), ty & (1 << RenderFlagType::Invalid as usize), invaild, 
		// 		ty & !(1 << RenderFlagType::Invalid as usize) | invaild,
		// 	));
		// }
		if ty & (1 << RenderFlagType::Invalid as usize) != invaild {
			
			ty = ty & !(1 << RenderFlagType::Invalid as usize) | invaild;
			
			// 根据canvas是否有对应的fbo，决定该节点是否显示
			instance_context.instance_data.instance_data_mut(index.start).set_data(&TyMeterial(&[ty as f32]));
		}
	}
}