use std::{borrow::BorrowMut, mem::transmute};

use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_ecs::{
    monitor::Event,
    prelude::{FromWorld, QueryState, Res, World, Query, ResMut}, query::{Write, Join, OrDefault},
};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Layer;
use pi_futures::BoxFuture;
use pi_hash::XHashMap;
use pi_null::Null;
use pi_postprocess::{
    postprocess_geometry::PostProcessGeometryManager, postprocess_pipeline::PostProcessMaterialMgr, temprory_render_target::EPostprocessTarget,
};
use pi_render::{
    components::view::target_alloc::{
        GetTargetView, SafeAtlasAllocator, ShareTargetView, TargetDescriptor, TargetView, TextureDescriptor,
    },
    graph::{
        node::{Node, ParamUsage, NodeId as GraphNodeId},
        param::InParamCollector,
        // param::P
        RenderContext,
    },
    rhi::{
        asset::RenderRes, bind_group::BindGroup, bind_group_layout::BindGroupLayout, device::RenderDevice, dyn_uniform_buffer::Group,
        texture::{ScreenTexture, PiRenderDefault}, CommandEncoder, RenderQueue,
    },
};
use pi_share::{Share, ShareRefCell};
use pi_slotmap::DefaultKey;
use pi_style::style::CgColor;
use render_derive::NodeParam;
use smallvec::SmallVec;
use wgpu::{RenderPass, Sampler};

use crate::{
    components::{
        draw_obj::{DrawObject, DrawState, ClearColorBindGroup, DynTargetType, CopyFboToScreen},
        pass_2d::{Camera, Draw2DList, DrawIndex, GraphId, ParentPassId, Pass2D, Pass2DKey, PostProcessList, RenderTarget, ScreenTarget},
        user::{Aabb2, Point2, Viewport, ClearColor, RenderTargetType}, calc::{NodeId, Quad},
    },
    resource::draw_obj::{CommonSampler, DynBindGroups, DynFboClearColorBindGroup, ClearDrawObj, MaxViewSize},
    shaders::{color::{CameraMatrixGroup, ColorMaterialGroup}, image::SampTex2DGroup},
    utils::tools::calc_hash,
};


/// Pass2D 渲染图节点
#[derive(Clone, Default)]
pub struct Pass2DNode {
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
    pass2d_query: QueryState<Pass2D, (&'static Camera, &'static Draw2DList, &'static ParentPassId)>,
    draw_query: QueryState<DrawObject, (&'static DrawState, Option<&'static GraphNodeId>, Join<NodeId, crate::components::user::Node, &'static Quad>)>,
    post_query: QueryState<Pass2D, (&'static PostProcessList, &'static GraphId)>,
    last_rt: &'s RenderTarget,
	last_rt_type: RenderTargetType,
	copy_fbo: Option<&'s CopyFboToScreen>,
    screen: &'s ScreenTarget,
    surface: &'s ScreenTexture,
    atlas_allocator: &'s SafeAtlasAllocator,
    t_type: &'s DynTargetType,
    bind_group_assets: &'s Share<AssetMgr<RenderRes<BindGroup>>>,
    device: &'s RenderDevice,
    queue: &'s RenderQueue,
    post_bind_group_layout: &'s PostBindGroupLayout,
    dyn_bind_groups: &'s DynBindGroups,
    postprocess_pipelines: &'s PostProcessMaterialMgr,
    geometrys: &'s PostProcessGeometryManager,

    // 清屏相关参数
    fbo_clear_color: &'s DynFboClearColorBindGroup,
    clear_color_group: Option<&'s ClearColorBindGroup>,
	clear_color: ClearColor,
    clear_draw: &'s ClearDrawObj,
    common_sampler: &'s CommonSampler,
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

// (, Handle<RenderRes<BindGroup>>)

#[derive(Default, NodeParam, Clone)]
pub struct RenderResult {
    result: Option<ShareTargetView>,
}
impl GetTargetView for RenderResult {
    fn get_target_view(&self) -> Option<&TargetView> { return self.result.as_ref().map(|r| &**r); }
}

// /// 清屏节点
// #[derive(Clone, Default)]
// pub struct ClearNode {
// 	root_id: Id<Pass2D>,
// }

// impl Node for ClearNode {
// 	type Input = ();
//     type Output = ();
// 	fn run<'a>(
//         &'a self,
//         context: RenderContext,
//         mut commands: ShareRefCell<CommandEncoder>,
//         input: &'a Self::Input,
//         _usage: &'a ParamUsage,
//         // context: RenderContext,
//         // mut commands: ShareRefCell<CommandEncoder>,
//         // inputs: &'a [Self::Output],
//     ) -> BoxFuture<'a, Result<Self::Output, String>> {
// 		let RenderContext { mut world, device, queue } = context;
// 		let root_id = self.root_id;

// 		Box::pin(async move {
// 			let query = QueryState::<Pass2D, (&'static Camera, &'static Draw2DList, &'static ParentPassId)>::new(&mut world);
// 			let last_rt = world.get_resource::<RenderTarget>().unwrap();

// 			if let RenderTarget::OffScreen(last_rt) = last_rt {
// 				let rect = last_rt.rect();
// 				// 将最终渲染目标渲染到屏幕上
// 				// 创建一个渲染Pass
// 				let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
// 					label: None,
// 					color_attachments: last_rt
// 						.target()
// 						.colors
// 						.iter()
// 						.map(|view| Some(wgpu::RenderPassColorAttachment {
// 							resolve_target: None,
// 							ops,
// 							view: &view.0,
// 						}))
// 						.collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
// 						.as_slice(),
// 					depth_stencil_attachment: match &r.target().depth {
// 						Some(r) => Some(wgpu::RenderPassDepthStencilAttachment {
// 							stencil_ops: None,
// 							depth_ops: Some(wgpu::Operations {
// 								load: wgpu::LoadOp::Load,
// 								store: true,
// 							}),
// 							view: &r.0,
// 						}),
// 						None => None,
// 					},
// 				});

// 				// 设置视口
// 				rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

// 				copy_fbo.0.draw(&mut rp, &param.dyn_bind_groups);
// 			}
// 			Ok(())
// 		})
// 	}
// }

impl Node for Pass2DNode {
    type Input = InParamCollector<RenderResult>;
    type Output = RenderResult;

    fn run<'a>(
        &'a self,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        input: &'a Self::Input,
        _usage: &'a ParamUsage,
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        // log::warn!("run graph==============, input count: {}", input.0.len());
        let RenderContext { mut world, device, queue } = context;

        let pass2d_id = self.pass2d_id;
		
        Box::pin(async move {
			let layer_query = QueryState::<Pass2D, Join<NodeId, crate::components::user::Node, &Layer<crate::components::user::Node>>>::new(&mut world);
			let (t_type, clear_color_group, last_rt, last_rt_type, copy_fbo, clear_color) = match layer_query.get(&world,  pass2d_id) {
				Some(r) => {
					let r = r.clone();
					let dyn_target_query = QueryState::<crate::components::user::Node,( &'static DynTargetType, Option<&'static ClearColorBindGroup>, &'static RenderTarget, OrDefault<RenderTargetType>,  Option<&'static CopyFboToScreen>, Option<&'static ClearColor>)>::new(&mut world);
					match dyn_target_query.get(&world, r.root()) {
						Some(r) => (r.0.clone(), unsafe { transmute(r.1) },  unsafe { transmute(r.2) }, r.3.clone(), unsafe { transmute(r.4) }, r.5.map_or(ClearColor(CgColor::new(0.0, 0.0, 0.0, 1.0), false), |r| {r.clone()}) ),
						None => return Ok(RenderResult { result: None }),
					}
				},
				None => return Ok(RenderResult { result: None }),
			};

            let mut param = Param {
                pass2d_query: QueryState::<Pass2D, (&'static Camera, &'static Draw2DList, &'static ParentPassId)>::new(&mut world),
                draw_query: QueryState::<DrawObject, (&'static DrawState, Option<&'static GraphNodeId>, Join<NodeId, crate::components::user::Node, &'static Quad>)>::new(&mut world),
                post_query: QueryState::<Pass2D, (&'static PostProcessList, &'static GraphId)>::new(&mut world),
                last_rt: last_rt,
				last_rt_type,
				copy_fbo,
                screen: world.get_resource::<ScreenTarget>().unwrap(),
                surface: world.get_resource::<ScreenTexture>().unwrap(),
                atlas_allocator: world.get_resource::<SafeAtlasAllocator>().unwrap(),
                t_type: &t_type,
                bind_group_assets: world.get_resource::<Share<AssetMgr<RenderRes<BindGroup>>>>().unwrap(),
                post_bind_group_layout: world.get_resource::<PostBindGroupLayout>().unwrap(),
                dyn_bind_groups: world.get_resource::<DynBindGroups>().unwrap(),
                postprocess_pipelines: world.get_resource::<PostProcessMaterialMgr>().unwrap(),
                geometrys: world.get_resource::<PostProcessGeometryManager>().unwrap(),

                device: &device,
                queue: &queue,
                fbo_clear_color: world.get_resource::<DynFboClearColorBindGroup>().unwrap(),
                clear_color_group,
				clear_color,
                clear_draw: world.get_resource::<ClearDrawObj>().unwrap(),
                common_sampler: world.get_resource::<CommonSampler>().unwrap(),
            };

            let post_list = param.post_query.get(&world, self.pass2d_id);
            let mut out = None;

            if let Some((camera, list, parent_pass2d_id)) = param.pass2d_query.get(&world, pass2d_id) {
				if camera.is_active && (list.opaque.len() > 0 || list.transparent.len() > 0 ) {
					let (rt, clear_color) = match post_list {
						None => {
							if !parent_pass2d_id.is_null() {
								// 如果后处理为None， 并且存在父节点，不进行渲染（可能由父节点对它进行渲染）
								return Ok(RenderResult { result: None });
							} else {
								out = Some(param.last_rt.0.clone());
								// 如果后处理为None，且不存在父节点，渲染到最终目标上(返回渲染目标为None)，并且清屏色为用户这只的清屏色
								(None, match param.clear_color_group {
									Some(r) => r.0.as_ref(),
									None => None
								})
							}
						}
						// 渲染类型为新建渲染目标对其进行渲染，则从纹理分配器中分配一个fbo矩形区
						Some(_) => (
							Some(param.atlas_allocator.allocate(
								(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
								(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
								param.t_type.has_depth,
								input.0.values(),
							)),
							Some(&param.fbo_clear_color.0),
						),
					};

					{
						let input_groups = Vec::with_capacity(input.0.len());
						// 创建一个渲染Pass
						let (mut rp, view_port) = create_rp(
							rt.as_ref(),
							commands.borrow_mut(),
							&camera.view_port,
							&param.last_rt,
							None,
							&param.surface,
							None,
						);

						// 设置视口
						rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
						// 清屏
						if let Some(clear_color) = clear_color {
							clear_color.draw(&mut rp, &param.dyn_bind_groups, ColorMaterialGroup::id());
							param.clear_draw.0.draw(&mut rp, &param.dyn_bind_groups);
							// 相机在drawObj中已经描述
						}


						// log::warn!("pass_node1==========================id:{:?}, view_port: {:?}, opaque: {}, transparent:{}", pass2d_id, view_port, list.opaque.len(), list.transparent.len());
						self.draw_list(&input.0, &input_groups, &mut rp, &world, list, &mut param, camera, camera, &view_port, &view_port);
					}


					if let Some((post_process, _graph_id)) = post_list {
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
								EPostprocessTarget::from_share_target(r, wgpu::TextureFormat::pi_render_default()),
								((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32),
							) {
								if let EPostprocessTarget::ShareTarget(r) = r {
									out = Some(r.view);
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
				}

                // 处理根节点
                if parent_pass2d_id.is_null() {
                    if let (Some(copy_fbo), RenderTargetType::Screen) = (param.copy_fbo, param.last_rt_type) {
                        let rect = param.last_rt.0.rect();
                        // 将最终渲染目标渲染到屏幕上
                        // 创建一个渲染Pass
                        let (mut rp, view_port) = create_rp(
                            None,
                            commands.borrow_mut(),
                            &Aabb2::new(
                                Point2::new(rect.min.x as f32, rect.min.y as f32),
                                Point2::new(rect.max.x as f32, rect.max.y as f32),
                            ),
                            &param.last_rt,
                            Some(&param.screen),
                            &param.surface,
                            if param.clear_color.1 {
								Some(wgpu::Operations {
									load: wgpu::LoadOp::Clear(wgpu::Color {
										r: param.clear_color.0.x as f64,
										g: param.clear_color.0.y as f64,
										b: param.clear_color.0.z as f64,
										a: param.clear_color.0.w as f64,
									}),
									// load: wgpu::LoadOp::Load,
									store: true,
								})
							} else {
								None
							},
                        );

                        // 设置视口
                        rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

                        copy_fbo.0.draw(&mut rp, &param.dyn_bind_groups);
                    }
                }
            }

            Ok(RenderResult { result: out })
        })
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
        input: &'a XHashMap<GraphNodeId, RenderResult>,
		input_groups: &'a Vec<Handle<RenderRes<BindGroup>>>,
        rp: &mut RenderPass<'a>,
        world: &'a World,
        param: &'a Param<'a>,
        last_camera: &'a Camera,
        cur_camera: &'a Camera,
        last_view_port: &(f32, f32, f32, f32),
        cur_view_port: &(f32, f32, f32, f32),
    ) {
        match param.post_query.get(world, pass2d_id) {
            Some((r, graph_id)) => {
                let src = match input.get(&graph_id.0) {
                    Some(r) => match &r.result {
						Some(r) => r,
						None => return,
					},
                    None => {
                        // 这种情况有可能出现，后处理对象可能为空
                        // log::error!("prepare render post process, but pre result is none");
                        return;
                    }
                };
				// 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
				unsafe {&mut *(input_groups as *const Vec<Handle<RenderRes<BindGroup>>> as usize as *mut Vec<Handle<RenderRes<BindGroup>>>)}.push(Self::create_post_process_data(src, &param, &param.common_sampler.pointer));
				let index = input_groups.len() - 1;

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

				// let post_process_mut = unsafe { &mut *(post_process as *const PostProcessList as usize as *mut PostProcessList) };
				// post_process_mut.cur_result = Some((r.clone(), data));
				
                if let Err(e) = r.draw_final(
                    param.device,
                    param.queue,
                    param.postprocess_pipelines,
                    param.geometrys,
                    rp,
                    EPostprocessTarget::from_share_target(src.clone(), wgpu::TextureFormat::pi_render_default()),
                    &input_groups[index],
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
                    &Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::GreaterEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    matrix.as_slice(),
                    r.depth as f32 / 60000.0,
                ) {
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
            }
            None => {
                // 如果不存在后处理，则将pass2d中的所有渲染对象渲染到rp上
                if let Some((
                    camera_new,
                    // rt_key,
                    list,
                    _pass2d_id,
                )) = param.pass2d_query.get(world, pass2d_id)
                {
                    let v = (
                        (last_view_port.0 as f32 - last_camera.view_port.mins.x) + camera_new.view_port.mins.x,
                        (last_view_port.1 as f32 - last_camera.view_port.mins.y) + camera_new.view_port.mins.y,
                        camera_new.view_port.maxs.x - camera_new.view_port.mins.x,
                        camera_new.view_port.maxs.y - camera_new.view_port.mins.y,
                    );


                    rp.set_viewport(v.0, v.1, v.2, v.3, 0.0, 1.0);

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

                    self.draw_list(input, input_groups, rp, world, list, param, last_camera, camera_new, last_view_port, cur_view_port);

                    rp.set_viewport(cur_view_port.0, cur_view_port.1, cur_view_port.2, cur_view_port.3, 0.0, 1.0);
                    if let Some(camera) = &cur_camera.bind_group {
                        camera.draw(rp, param.dyn_bind_groups, CameraMatrixGroup::id());
                    }
                }
            }
        }
    }

    fn draw_list<'a, 'w>(
        &self,
        input: &'a XHashMap<GraphNodeId, RenderResult>,
		input_groups: &'a Vec<Handle<RenderRes<BindGroup>>>,
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

		// log::warn!("draw============================={:?}, {:?}, {:?}, {:?}", list.opaque.len(), list.transparent.len(), list.opaque, list.transparent);

        for e in list.opaque.iter().chain(list.transparent.iter()) {
            match e {
                DrawIndex::DrawObj(e) => {
                    // 设置相机
                    if let Some((state, graph_id, quad)) = param.draw_query.get(world, *e) {
						// 如果存在graph_id，表示该渲染对象将输入的一个ShareTargetView作为纹理，渲染到gui上
						if let Some(graph_id) = graph_id {
							let src = match input.get(graph_id) {
								Some(r) => match &r.result {
									Some(r) => r, 
									None => continue,
								},
								None => continue
							};
							let rect = src.rect();
							// 根据纹理大小和渲染目标大小，来确定过滤方式
							// 如果大小近似相等，则使用点过滤，否则使用双线性过滤
							let s = if ((quad.maxs.x - quad.mins.x) as i32 - rect.width()).abs() <= 1 &&
											((quad.maxs.y - quad.mins.y) as i32 - rect.height()).abs() <= 1 {
												&param.common_sampler.pointer
											} else {
												&param.common_sampler.default
											};
							// 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
							unsafe {&mut *(input_groups as *const Vec<Handle<RenderRes<BindGroup>>> as usize as *mut Vec<Handle<RenderRes<BindGroup>>>)}.push(Self::create_post_process_data(src, &param, s));
							let index = input_groups.len() - 1;
							rp.set_bind_group(SampTex2DGroup::id(), &input_groups[index], &[])
						}
						

                        if state.bind_groups.get_group(CameraMatrixGroup::id()).is_none() {
                            if let Some(r) = &cur_camera.bind_group {
                                r.draw(rp, &param.dyn_bind_groups, CameraMatrixGroup::id());
                            }
                        }
                        state.draw(rp, &param.dyn_bind_groups);
                    }
                }
                DrawIndex::Pass2D(e) => {
                    self.render_pass_2d(*e, input, input_groups, rp, world, param, last_camera, cur_camera, last_view_port, cur_view_port);
                }
            }
        }
    }

    // 创建后处理数据（bindgroup和uv buffer）
    fn create_post_process_data<'s>(texture: &ShareTargetView, param: &'s Param<'s>, sampler: &'s Sampler) -> Handle<RenderRes<BindGroup>> {
        // let uv = texture.uv();
        let group_key = calc_hash(&(texture.ty_index(), texture.target_index()), calc_hash(&"render target", 0)); // TODO
        match param.bind_group_assets.get(&group_key) {
            Some(r) => r,
            None => {
                let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: param.post_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0),
                        },
                    ],
                    label: Some("post process texture bind group create"),
                });
                param.bind_group_assets.insert(group_key, RenderRes::new(group.clone(), 5)).unwrap()
            }
        }
    }
}

pub fn create_rp<'a>(
	rt: Option<&'a ShareTargetView>,
	commands: &'a mut CommandEncoder,
	view_port: &Aabb2,
	last_rt: &'a RenderTarget,
	screen: Option<&'a ScreenTarget>,
	surface: &'a ScreenTexture,
	ops: Option<wgpu::Operations<wgpu::Color>>,
) -> (RenderPass<'a>, (f32, f32, f32, f32)) {
	let ops = match ops {
		Some(r) => r,
		None => wgpu::Operations {
			// load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}),
			load: wgpu::LoadOp::Load,
			store: true,
		},
	};
	match screen { 
		Some(screen) => {
			let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					resolve_target: None,
					ops,
					view: surface.view.as_ref().unwrap(),
				})],
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
			(
				rp,
				(
					view_port.mins.x,
					view_port.mins.y,
					view_port.maxs.x - view_port.mins.x,
					view_port.maxs.y - view_port.mins.y,
				),
			)
		}
		None => {
			let mut r = &last_rt.0;
			if let Some(t) = rt {
				r = t;
			}
			let rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: r
					.target()
					.colors
					.iter()
					.map(|view| Some(wgpu::RenderPassColorAttachment {
						resolve_target: None,
						ops,
						view: &view.0,
					}))
					.collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
					.as_slice(),
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
			(
				rp,
				(
					rect.min.x as f32,
					rect.min.y as f32,
					view_port.maxs.x - view_port.mins.x,
					view_port.maxs.y - view_port.mins.y,
				),
			)
		},
		
	}

	// match (screen, last_rt.1) {
	// 	(Some(t), None, _) | (None, None, RenderTargetType::OffScreen) | (None, None, RenderTargetType::Screen) => {
			
	// 	}
	// 	(_, Some(screen), _) => {
			
	// 	}
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

/// 创建图节点所需要的数据
/// 如： DynTargetType (需要根据视口变化及时调整)
pub struct InitGraphData;
// use crate::components::user::Node;
#[setup]
impl InitGraphData {
    #[listen(component=(crate::components::user::Node, Viewport, (Modify, Create)))]
    pub fn calc_dyn_target_type(
        e: Event,
		query: Query<crate::components::user::Node, (&Viewport, Write<DynTargetType>)>,

        atlas_allocator: Res<SafeAtlasAllocator>,
		mut max_view_size: ResMut<MaxViewSize>,
    ) {
		if let Some((view_port, mut dyn_target_type)) = query.get_by_entity(e.id) {
			max_view_size.width = max_view_size.width.min((view_port.maxs.x - view_port.mins.x).ceil() as u32);
			max_view_size.height = max_view_size.height.min((view_port.maxs.y - view_port.mins.y).ceil() as u32);
			let ty = Self::create_dyn_target_type(&atlas_allocator, max_view_size.width, max_view_size.height);
			dyn_target_type.write(ty);
		}
    }

    pub fn create_dyn_target_type(atlas_allocator: &SafeAtlasAllocator, width: u32, height: u32) -> DynTargetType {
        DynTargetType {
            has_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
                texture_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::pi_render_default(),
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    base_mip_level: 0,
                    base_array_layer: 0,
                    array_layer_count: None,
                    view_dimension: None,
                }]),
                need_depth: true,
                default_width: width,
                default_height: height,
            }),
            no_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
                texture_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::pi_render_default(),
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    base_mip_level: 0,
                    base_array_layer: 0,
                    array_layer_count: None,
                    view_dimension: None,
                }]),
                need_depth: false,
                default_width: width,
                default_height: height,
            }),
        }
    }
}
