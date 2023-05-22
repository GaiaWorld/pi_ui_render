use std::{borrow::BorrowMut, mem::transmute};

use bevy::ecs::{
    prelude::Entity,
    query::With,
    system::{Query, Res, SystemParam, SystemState},
    world::World,
};
use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::{
    prelude::{Layer, OrDefault},
    system_param::res::OrInitRes,
};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{
    node::{Node, NodeId as GraphNodeId, ParamUsage},
    param::InParamCollector,
    PiSafeAtlasAllocator,
    PiScreenTexture,
    // param::P
    RenderContext,
    SimpleInOut,
};
use pi_futures::BoxFuture;
use pi_hash::XHashMap;
use pi_null::Null;
use pi_render::rhi::shader::Input;
// use pi_postprocess::
use pi_postprocess::temprory_render_target::PostprocessTexture;
use pi_render::{
    components::view::target_alloc::ShareTargetView,
    renderer::{draw_obj::DrawObj, texture::texture_view::ETextureViewUsage},
    rhi::{
        asset::RenderRes,
        bind_group::BindGroup,
        buffer::Buffer,
        device::RenderDevice,
        pipeline::RenderPipeline,
        shader::BindLayout,
        texture::{PiRenderDefault, ScreenTexture},
        CommandEncoder, RenderQueue,
    },
};
use pi_share::ShareRefCell;
use wgpu::{RenderPass, Sampler};

use crate::{
    components::{
        calc::{EntityKey, NodeId, Quad},
        draw_obj::{ClearColorBindGroup, CopyFboToScreen, DrawState, DynTargetType},
        pass_2d::{Camera, Draw2DList, DrawIndex, GraphId, ParentPassId, PostProcessList, RenderTarget, ScreenTarget},
        user::{Aabb2, Point2, RenderTargetType},
    },
    resource::draw_obj::{ClearDrawObj, CommonSampler, DynFboClearColorBindGroup, PostBindGroupLayout},
    shader::{
        camera::CameraBind,
        depth::DepthBind,
        image::{SampBind, UvVert},
        ui_meterial::UiMaterialBind,
    },
    utils::tools::calc_hash,
};


/// Pass2D 渲染图节点
// #[derive(Clone)]
pub struct Pass2DNode {
    // // 输入描述
    // input: Vec<SlotInfo>,
    // // 输出描述
    // output: Vec<SlotInfo>,
    pub pass2d_id: Entity,
    pub output_target: Option<ShareTargetView>,
    // pub last_post_key: DefaultKey,
    pub out: Option<ShareTargetView>,
    // node_id_query: QueryState<&'static NodeId, With<Camera>>,
}

#[derive(SystemParam)]
pub struct QueryParam<'w, 's> {
    query_pass_node: Query<
        'w,
        's,
        (
            &'static DynTargetType,
            Option<&'static ClearColorBindGroup>,
            &'static RenderTarget,
            OrDefault<RenderTargetType>,
            Option<&'static CopyFboToScreen>,
        ),
    >,
    pass2d_query: (
        Query<'w, 's, &'static Layer, With<Camera>>,
        Query<'w, 's, (&'static Camera, &'static Draw2DList, &'static ParentPassId)>,
        Query<'w, 's, (&'static PostProcessList, &'static GraphId)>,
    ),
    draw_query: Query<'w, 's, (&'static DrawState, &'static NodeId, Option<&'static GraphId>)>,
    node_query: Query<'w, 's, &'static Quad>,
    // graph_id_query: Query<'w, 's, &'static GraphId>,
    screen: Res<'w, ScreenTarget>,
    surface: Res<'w, PiScreenTexture>,
    atlas_allocator: Res<'w, PiSafeAtlasAllocator>,
    bind_group_assets: Res<'w, ShareAssetMgr<RenderRes<BindGroup>>>,
    post_bind_group_layout: OrInitRes<'w, PostBindGroupLayout>,
    // postprocess_pipelines: Res<'w, My PiPostProcessMaterialMgr>,
    post_resource: Res<'w, PostprocessResource>,
    pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,

    // 清屏相关参数
    fbo_clear_color: Res<'w, DynFboClearColorBindGroup>,
    clear_draw: Res<'w, ClearDrawObj>,
    common_sampler: Res<'w, CommonSampler>,
}

// vballocator: &mut VertexBufferAllocator,
// safeatlas: &SafeAtlasAllocator,
// resources: &SingleImageEffectResource,
// pipelines: &Share<AssetMgr<RenderRes<RenderPipeline>>>,


pub struct Param<'w, 's> {
    pass2d_query: Query<'w, 's, (&'static Camera, &'static Draw2DList, &'static ParentPassId)>,
    draw_query: Query<'w, 's, (&'static DrawState, &'static NodeId, Option<&'static GraphId>)>,
    node_query: Query<'w, 's, &'static Quad>,
    // graph_id_query: Query<'w, 's, &'static GraphId>,
    post_query: Query<'w, 's, (&'static PostProcessList, &'static GraphId)>,
    screen: Res<'s, ScreenTarget>,
    atlas_allocator: Res<'s, PiSafeAtlasAllocator>,
    bind_group_assets: Res<'s, ShareAssetMgr<RenderRes<BindGroup>>>,
    post_bind_group_layout: OrInitRes<'s, PostBindGroupLayout>,
    // postprocess_pipelines: Res<'s, PiPostProcessMaterialMgr>,
    // geometrys: Res<'s, PiPostProcessGeometryManager>,
    post_resource: Res<'w, PostprocessResource>,
    pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,

    // 清屏相关参数
    fbo_clear_color: Res<'s, DynFboClearColorBindGroup>,
    clear_draw: Res<'s, ClearDrawObj>,
    common_sampler: Res<'s, CommonSampler>,

    last_rt: &'s RenderTarget,
    last_rt_type: RenderTargetType,
    t_type: &'s DynTargetType,
    copy_fbo: Option<&'s CopyFboToScreen>,
    device: &'s RenderDevice,
    queue: &'s RenderQueue,
    clear_color_group: Option<&'s ClearColorBindGroup>,
    surface: &'s ScreenTexture,
}

// last_rt_type: RenderTargetType,
// clear_color: ClearColor,

impl Pass2DNode {
    pub fn new(pass2d_id: Entity) -> Self {
        Self {
            pass2d_id,
            output_target: None,
            // last_post_key: EntityKey::default(),
            out: None,
            // param,
        }
    }
}

// (, Handle<RenderRes<BindGroup>>)


impl Node for Pass2DNode {
    type Input = InParamCollector<SimpleInOut>;
    type Output = SimpleInOut;

    type Param = QueryParam<'static, 'static>;


    fn run<'a>(
        &'a mut self,
        world: &'a World,
        query_param_state: &'a mut SystemState<Self::Param>,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        input: &'a Self::Input,
        _usage: &'a ParamUsage,
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let RenderContext { device, queue } = context;

        let pass2d_id = self.pass2d_id;


        Box::pin(async move {
            let query_param = query_param_state.get(world);
            // log::warn!("run1======{:?}", pass2d_id);
            let layer = match query_param.pass2d_query.0.get(pass2d_id) {
                Ok(r) if r.layer() > 0 => r.clone(),
                _ => return Ok(SimpleInOut { target: None }),
            };
            // log::warn!("run2======{:?}", pass2d_id);

            let surface = match &**query_param.surface {
                Some(r) => r,
                _ => return Ok(SimpleInOut { target: None }),
            };


            // log::warn!("run3======{:?}", pass2d_id);
            let (t_type, clear_color_group, last_rt, last_rt_type, copy_fbo) = {
                match query_param.query_pass_node.get(layer.root()) {
                    Ok(r) => (
                        r.0.clone(),
                        unsafe { transmute(r.1) },
                        unsafe { transmute(r.2) },
                        r.3.clone(),
                        unsafe { transmute(r.4) },
                        // r.5.map_or(ClearColor(CgColor::new(0.0, 0.0, 0.0, 1.0), false), |r| r.clone()),
                    ),
                    _ => return Ok(SimpleInOut { target: None }),
                }
            };
            // log::warn!("run4======{:?}", pass2d_id);

            let param = Param {
                pass2d_query: query_param.pass2d_query.1,
                draw_query: query_param.draw_query,
                post_query: query_param.pass2d_query.2,
                node_query: query_param.node_query,
                // graph_id_query: query_param.graph_id_query,
                last_rt: last_rt,
                last_rt_type,
                copy_fbo,
                screen: query_param.screen,
                surface: surface,
                atlas_allocator: query_param.atlas_allocator,
                t_type: &t_type,
                bind_group_assets: query_param.bind_group_assets,
                post_bind_group_layout: query_param.post_bind_group_layout,
                // postprocess_pipelines: query_param.postprocess_pipelines,
                // geometrys: query_param.geometrys,
                post_resource: query_param.post_resource,
                pipline_assets: query_param.pipline_assets,

                device: &device,
                queue: &queue,
                fbo_clear_color: query_param.fbo_clear_color,
                clear_color_group,
                clear_draw: query_param.clear_draw,
                common_sampler: query_param.common_sampler,
            };

            let post_list = param.post_query.get(pass2d_id);
            let mut out = None;


            if let Ok((camera, list, parent_pass2d_id)) = param.pass2d_query.get(pass2d_id) {
                // log::warn!("run5======{:?}, {:?}, {:?}", pass2d_id, list.transparent, list.opaque);
                // log::warn!("run graph4==============, input count: {}, opaque: {}, transparent: {}, is_active: {:?}, opaque_list: {:?}, transparent_list: {:?}, view_port: {:?}", input.0.len(), list.opaque.len(), list.transparent.len(), camera.is_active, &list.opaque, &list.transparent, &camera.view_port);
                if camera.is_active && (list.opaque.len() > 0 || list.transparent.len() > 0) {
                    let (rt, clear_color) = match post_list {
                        // 渲染类型为新建渲染目标对其进行渲染，则从纹理分配器中分配一个fbo矩形区
                        Ok(r) => {
                            let has_effect = r.0.has_effect();
                            if has_effect {
                                (
                                    // has_effect,
                                    Some(param.atlas_allocator.allocate(
                                        (camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
                                        (camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
                                        param.t_type.has_depth,
                                        input.0.values(),
                                    )),
                                    // Some((
                                    // 	(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
                                    // 	(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
                                    // 	param.t_type.has_depth,
                                    // )),
                                    Some(&param.fbo_clear_color.0),
                                )
                            } else {
                                if parent_pass2d_id.is_null() {
                                    out = param.last_rt.0.clone();
                                    // 如果没有设置任何一个后处理效果，且不存在父节点，渲染到最终目标上(返回渲染目标为None)，并且清屏色为用户设置的清屏色
                                    (
                                        None,
                                        // has_effect,
                                        match param.clear_color_group {
                                            Some(r) => r.0.as_ref(),
                                            None => None,
                                        },
                                    )
                                } else {
                                    // 如果后处理为None， 并且存在父节点，不进行渲染（可能由父节点对它进行渲染）
                                    return Ok(SimpleInOut { target: None });
                                }
                            }
                        }
                        _ => {
                            // 应该不会进入该分支
                            return Ok(SimpleInOut { target: None });
                        }
                    };

                    {
                        let input_groups = Vec::with_capacity(input.0.len());
                        let post_draw = Vec::with_capacity(input.0.len());
                        // 创建一个渲染Pass
                        let (mut rp, view_port) = create_rp(
                            rt.as_ref(),
                            commands.borrow_mut(),
                            &camera.view_port,
                            &param.last_rt,
                            None,
                            param.surface,
                            None,
                        );

                        // 设置视口
                        rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
                        // 清屏
                        if let Some(clear_color) = clear_color {
                            clear_color.0.set(&mut rp, UiMaterialBind::set());
                            clear_color.1.set(&mut rp, DepthBind::set());
                            param.clear_draw.0.draw(&mut rp);
                            // 相机在drawObj中已经描述
                        }

                        Self::draw_list(
                            &input.0,
                            &post_draw,
                            &input_groups,
                            &mut rp,
                            (view_port.2 as u32, view_port.3 as u32),
                            &world,
                            list,
                            &param,
                            camera,
                            camera,
                            &view_port,
                            &view_port,
                        );
                    }

                    // 			vballocator: &mut VertexBufferAllocator,
                    // safeatlas: &SafeAtlasAllocator,
                    // resources: &SingleImageEffectResource,
                    // pipelines: &Share<AssetMgr<RenderRes<RenderPipeline>>>,
                    // target_type: TargetType,

                    if let Ok((post_process, _graph_id)) = post_list {
                        if let Some(rt) = rt {
                            let rect = rt.rect().clone();
                            let (w, h) = ((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32);
                            // 渲染后处理
                            if let Ok(r) = post_process.draw_front(
                                param.device,
                                param.queue,
                                commands.borrow_mut(),
                                PostprocessTexture::from_share_target(rt, wgpu::TextureFormat::pi_render_default()),
                                (w, h),
                                &param.atlas_allocator,
                                &param.post_resource.resources,
                                &param.pipline_assets,
                                param.t_type.no_depth,
                            ) {
                                if let ETextureViewUsage::SRT(r) = r.view {
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
                }

                // 处理根节点
                if parent_pass2d_id.is_null() {
                    if let (Some(copy_fbo), RenderTargetType::Screen) = (param.copy_fbo, param.last_rt_type) {
                        if let Some(copy_fbo) = &copy_fbo.0 {
                            let t = param.last_rt.0.as_ref().unwrap();
                            let rect = t.rect();
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
                                None,
                            );

                            // 设置视口
                            rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

                            copy_fbo.draw(&mut rp);
                        }
                    }
                }
            }

            Ok(SimpleInOut { target: out })
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
        pass2d_id: EntityKey,
        input: &'a XHashMap<GraphNodeId, SimpleInOut>,
        post_draw: &'a Vec<DrawObj>,
        input_groups: &'a Vec<(Handle<RenderRes<BindGroup>>, Buffer)>,
        rp: &mut RenderPass<'a>,
        target_size: (u32, u32),
        world: &'a World,
        param: &'a Param<'a, 'a>,
        last_camera: &'a Camera,
        cur_camera: &'a Camera,
        last_view_port: &(f32, f32, f32, f32),
        cur_view_port: &(f32, f32, f32, f32),
    ) {
        match param.post_query.get(*pass2d_id) {
            Ok((r, graph_id)) if r.has_effect() => {
                let src = match input.get(&graph_id.0) {
                    Some(r) => match &r.target {
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
                // unsafe { &mut *(post_draw as *const Vec<DrawObj> as usize as *mut Vec<DrawObj>) }
                //     .push(Self::create_post_process_data(src, &param, &param.common_sampler.pointer));
                // let index = post_draw.len() - 1;

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

                // log::warn!("pass2d_id======={:?}, {:?}", pass2d_id, src.rect());
                // &input_groups[index],
                if let Some(draw_obj) = r.draw_final(
                    param.device,
                    param.queue,
                    matrix.as_slice(),
                    r.depth as f32 / 60000.0,
                    &param.atlas_allocator,
                    &PostprocessTexture::from_share_target(src.clone(), wgpu::TextureFormat::pi_render_default()),
                    target_size,
                    &param.post_resource.resources,
                    &param.pipline_assets,
                    wgpu::ColorTargetState {
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
                    },
                    Some(pi_render::renderer::pipeline::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::GreaterEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: pi_render::renderer::pipeline::DepthBiasState::default(),
                    }),
                    param.t_type.has_depth,
                ) {
                    // 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
                    unsafe { &mut *(post_draw as *const Vec<DrawObj> as usize as *mut Vec<DrawObj>) }.push(draw_obj);
                    let index = post_draw.len() - 1;

                    post_draw[index].draw(rp);
                    // log::error!("draw_final fail, {:?} ", e);
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
            _ => {
                // 如果不存在后处理，则将pass2d中的所有渲染对象渲染到rp上
                if let Ok((
                    camera_new,
                    // rt_key,
                    list,
                    _pass2d_id,
                )) = param.pass2d_query.get(*pass2d_id)
                {
                    let v = (
                        (last_view_port.0 as f32 - last_camera.view_port.mins.x) + camera_new.view_port.mins.x,
                        (last_view_port.1 as f32 - last_camera.view_port.mins.y) + camera_new.view_port.mins.y,
                        camera_new.view_port.maxs.x - camera_new.view_port.mins.x,
                        camera_new.view_port.maxs.y - camera_new.view_port.mins.y,
                    );

                    if v.2 <= 0.0 || v.3 <= 0.0 {
                        return;
                    }

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

                    Self::draw_list(
                        input,
                        post_draw,
                        input_groups,
                        rp,
                        target_size,
                        world,
                        list,
                        param,
                        last_camera,
                        camera_new,
                        last_view_port,
                        &v,
                    );

                    rp.set_viewport(cur_view_port.0, cur_view_port.1, cur_view_port.2, cur_view_port.3, 0.0, 1.0);
                    if let Some(camera) = &cur_camera.bind_group {
                        camera.set(rp, CameraBind::set());
                    }
                }
            }
        }
    }

    fn draw_list<'a, 'w>(
        input: &'a XHashMap<GraphNodeId, SimpleInOut>,
        post_draw: &'a Vec<DrawObj>,
        input_groups: &'a Vec<(Handle<RenderRes<BindGroup>>, Buffer)>,
        rp: &'w mut RenderPass<'a>,
        target_size: (u32, u32),
        world: &'a World,
        list: &Draw2DList,

        param: &'a Param<'a, 'a>,
        last_camera: &'a Camera,
        cur_camera: &'a Camera,
        last_view_port: &(f32, f32, f32, f32),
        cur_view_port: &(f32, f32, f32, f32),
    ) {
        if let Some(camera) = &cur_camera.bind_group {
            camera.set(rp, CameraBind::set());
        }

        // log::warn!("draw============================={:?}, {:?}, {:?}, {:?}", list.opaque.len(), list.transparent.len(), list.opaque, list.transparent);

        for e in list.opaque.iter().chain(list.transparent.iter()) {
            match e {
                DrawIndex::DrawObj(e) => {
                    if let Ok((state, node_id, graph_id)) = param.draw_query.get(**e) {
                        let quad = match param.node_query.get(***node_id) {
                            Ok(r) => r,
                            _ => continue,
                        };
                        // 如果存在graph_id，表示该渲染对象将输入的一个ShareTargetView作为纹理，渲染到gui上
                        if let Some(graph_id) = graph_id {
                            let src = match input.get(&**graph_id) {
                                Some(r) => match &r.target {
                                    Some(r) => r,
                                    None => continue,
                                },
                                None => continue,
                            };
                            let rect = src.rect();
                            // 根据纹理大小和渲染目标大小，来确定过滤方式
                            // 如果大小近似相等，则使用点过滤，否则使用双线性过滤
                            let s = if ((quad.maxs.x - quad.mins.x) as i32 - rect.width()).abs() <= 1
                                && ((quad.maxs.y - quad.mins.y) as i32 - rect.height()).abs() <= 1
                            {
                                &param.common_sampler.pointer
                            } else {
                                &param.common_sampler.default
                            };
                            // 这里使用非安全的方式将不可变引用转为可变引用的前提是，Vec在创建时容量足够，使得push时不需要扩容，同时使用Vec的地方不能多线程
                            unsafe {
                                &mut *(input_groups as *const Vec<(Handle<RenderRes<BindGroup>>, Buffer)> as usize
                                    as *mut Vec<(Handle<RenderRes<BindGroup>>, Buffer)>)
                            }
                            .push(Self::create_post_process_data(src, &param, s));
                            let index = input_groups.len() - 1;
                            rp.set_bind_group(SampBind::set(), &input_groups[index].0, &[]);
                            rp.set_vertex_buffer(UvVert::location() as u32, *input_groups[index].1.slice(..));
                        }


                        if state.bindgroups.get_group(CameraBind::set()).is_none() {
                            if let Some(r) = &cur_camera.bind_group {
                                r.set(rp, CameraBind::set());
                            }
                        }
                        state.draw(rp);
                    }
                }
                DrawIndex::Pass2D(e) => {
                    Self::render_pass_2d(
                        *e,
                        input,
                        post_draw,
                        input_groups,
                        rp,
                        target_size,
                        world,
                        param,
                        last_camera,
                        cur_camera,
                        last_view_port,
                        cur_view_port,
                    );
                }
            }
        }
    }

    // 创建后处理数据（bindgroup和uv buffer）
    fn create_post_process_data<'s>(
        texture: &ShareTargetView,
        param: &'s Param<'s, 's>,
        sampler: &'s Sampler,
    ) -> (Handle<RenderRes<BindGroup>>, Buffer) {
        let uv = texture.uv();
        let group_key = calc_hash(&(texture.ty_index(), texture.target_index()), calc_hash(&"render target", 0)); // TODO
        (
            match param.bind_group_assets.get(&group_key) {
                Some(r) => r,
                None => {
                    let group = param.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &param.post_bind_group_layout,
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
            },
            // 实时创建uvbuffer， 因为该buffer动态性很高，可能不应该创建为资源？
            // 这里应该与脏区域相交，渲染脏区域， TODO
            param.device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some("post process uv Buffer"),
                contents: bytemuck::cast_slice(&uv),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        )
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
            let mut r = last_rt.0.as_ref().unwrap();
            if let Some(t) = rt {
                r = t;
            }
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
                if rt.is_some() {
                    (
                        rect.min.x as f32,
                        rect.min.y as f32,
                        view_port.maxs.x - view_port.mins.x,
                        view_port.maxs.y - view_port.mins.y,
                    )
                } else {
                    (
                        rect.min.x as f32 + view_port.mins.x,
                        rect.min.y as f32 + view_port.mins.y,
                        view_port.maxs.x - view_port.mins.x,
                        view_port.maxs.y - view_port.mins.y,
                    )
                },
            )
        }
    }

    // match (screen, last_rt.1) {
    // 	(Some(t), None, _) | (None, None, RenderTargetType::OffScreen) | (None, None, RenderTargetType::Screen) => {

    // 	}
    // 	(_, Some(screen), _) => {

    // 	}
    // }
}
