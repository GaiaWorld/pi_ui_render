use std::{
    mem::transmute,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::ecs::{
    prelude::Entity,
    query::{ChangeTrackers, Changed, Or, With},
    system::{Commands, ParamSet, Query, Res, ResMut},
};
use pi_assets::asset::Handle;
use pi_async::{
    prelude::AsyncVariableNonBlocking,
    prelude::{AsyncRuntime, AsyncRuntimeExt},
};
use pi_bevy_assert::ShareAssetMgr;
use pi_bevy_ecs_extend::{prelude::OrDefault, system_param::res::{OrInitRes, OrInitResMut}};
use pi_bevy_render_plugin::{PiRenderDevice, PiSafeAtlasAllocator};
use pi_hal::runtime::RENDER_RUNTIME;
use pi_render::{
    components::view::target_alloc::{SafeAtlasAllocator, ShareTargetView, TargetDescriptor, TextureDescriptor},
    rhi::{asset::RenderRes, bind_group::BindGroup, pipeline::RenderPipeline, shader::BindLayout, texture::PiRenderDefault}, renderer::{draw_obj::DrawBindGroup, vertices::{RenderVertices, EVerticesBufferUsage, RenderIndices}},
};
use pi_share::Share;
use smallvec::SmallVec;
use wgpu::IndexFormat;

use crate::{
    components::{
        calc::WorldMatrix,
        draw_obj::{CopyFboToScreen, DrawState, DynTargetType, PipelineMeta},
        pass_2d::RenderTarget,
        user::{Matrix4, RenderTargetType, Viewport},
    },
    resource::draw_obj::{
        CameraGroup, CommonSampler, MaxViewSize, PosUv1VertexLayout, PostBindGroupLayout, Program, ProgramMetaRes, ShaderInfoCache,
        ShareGroupAlloter, UiMaterialGroup, UnitQuadBuffer, DepthCache,
    },
    shader::{
        camera::{ProjectUniform, ViewUniform},
        image::{ProgramMeta, SampBind},
        ui_meterial::{WorldUniform},
		depth::DepthBind,
    },
    system::draw_obj::pipeline::calc_pipeline,
    utils::tools::calc_hash,
};

/// 处理视口改变
pub fn view_port_change(
    res: (
        Res<ShareAssetMgr<RenderRes<RenderPipeline>>>,
        Res<ShareAssetMgr<RenderRes<Program>>>,
        Res<UnitQuadBuffer>,
        OrInitRes<ProgramMetaRes<ProgramMeta>>,
        OrInitRes<PosUv1VertexLayout>,
		OrInitResMut<DepthCache>,
        Res<PiRenderDevice>,
        // Res<ShaderCatch>,
    ),
    // state_map: Res< StateMap>,
    bind_group_assets: Res<ShareAssetMgr<RenderRes<BindGroup>>>,

    post_bind_group_layout: OrInitRes<PostBindGroupLayout>,
    shader_info_cache: OrInitRes<ShaderInfoCache>,
    common_sampler: Res<CommonSampler>,
    // common_state: Res<CommonPipelineState>,
    camera_group_alloter: OrInitRes<ShareGroupAlloter<CameraGroup>>,
    ui_meterial_group_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
    allocator: Res<PiSafeAtlasAllocator>,

    mut query: ParamSet<(
        Query<
            (
                Entity,
                &'static mut CopyFboToScreen,
                &'static mut RenderTarget,
                OrDefault<RenderTargetType>,
                &'static Viewport,
                ChangeTrackers<Viewport>,
                &'static DynTargetType,
            ),
            (With<Viewport>, Or<(Changed<Viewport>, Changed<RenderTargetType>)>),
        >,
        Query<&'static mut CopyFboToScreen>,
    )>,
) {
    let pipeline_map: Res<'static, ShareAssetMgr<RenderRes<RenderPipeline>>> = unsafe { transmute(res.0) };
    let shader_map: Res<'static, ShareAssetMgr<RenderRes<Program>>> = unsafe { transmute(res.1) };

    let unit_quad_buffer: Res<'static, UnitQuadBuffer> = unsafe { transmute(res.2) };
    let image_shader_meta: OrInitRes<'static, ProgramMetaRes<ProgramMeta>> = unsafe { transmute(res.3) };
    let vert_layout: OrInitRes<'static, PosUv1VertexLayout> = unsafe { transmute(res.4) };
	let depth_cache: OrInitResMut<'static, DepthCache> = unsafe { transmute(res.5) };
    let device: Res<'static, PiRenderDevice> = unsafe { transmute(res.6) };
    let bind_group_assets: Res<'static, ShareAssetMgr<RenderRes<BindGroup>>> = unsafe { transmute(bind_group_assets) };

    // mut copy_draw_obj: WriteRes<'static, CopyFboToScreen>,
    let post_bind_group_layout: OrInitRes<'static, PostBindGroupLayout> = unsafe { transmute(post_bind_group_layout) };
    let common_sampler: Res<'static, CommonSampler> = unsafe { transmute(common_sampler) };
    let shader_info_cache: OrInitRes<'static, ShaderInfoCache> = unsafe { transmute(shader_info_cache) };

    // render_target: Res<'static, RenderTarget>,
    // let camera_bind_group: Res<'static, DynBindGroupIndex<CameraMatrixGroup>> = unsafe { transmute(camera_bind_group)};
    // let post_bind_group: Res<'static, DynBindGroupIndex<UiMaterialGroup>> = unsafe { transmute(post_bind_group)};
    // let common_state: Res<'static, CommonPipelineState> = unsafe { transmute(common_state)};

    let camera_group_alloter: OrInitRes<'static, ShareGroupAlloter<CameraGroup>> = unsafe { transmute(camera_group_alloter) };
    let ui_meterial_group_alloter: OrInitRes<'static, ShareGroupAlloter<UiMaterialGroup>> = unsafe { transmute(ui_meterial_group_alloter) };
    let allocator: Res<'static, PiSafeAtlasAllocator> = unsafe { transmute(allocator) };

    let query0: Query<
        'static,
        'static,
        (
            Entity,
            &'static mut CopyFboToScreen,
            &'static mut RenderTarget,
            OrDefault<RenderTargetType>,
            &'static Viewport,
            ChangeTrackers<Viewport>,
            &'static DynTargetType,
        ),
        (With<Viewport>, Or<(Changed<Viewport>, Changed<RenderTargetType>)>),
    > = unsafe { transmute(query.p0()) };

    let value = AsyncVariableNonBlocking::<Vec<(Entity, Handle<RenderRes<RenderPipeline>>)>>::new();
    let count = Share::new(AtomicUsize::new(0));
    let mut task_count = 0;

    render_change_async(
		depth_cache,
        value.clone(),
        count.clone(),
        &mut task_count,
        pipeline_map,
        shader_map,
        unit_quad_buffer,
        image_shader_meta,
        vert_layout,
        shader_info_cache,
        device,
        bind_group_assets,
        post_bind_group_layout,
        common_sampler,
        camera_group_alloter,
        ui_meterial_group_alloter,
        allocator,
        query0,
    );
    let query1: Query<'static, 'static, &'static mut CopyFboToScreen> = unsafe { transmute(query.p1()) };
    if task_count > 0 {
        RENDER_RUNTIME
            .block_on(async move {
                let mut result = value.await;
                set_result(query1, &mut result);
            })
            .unwrap();
    }

    // 死循环，等待渲染管线创建完成
}

#[allow(unused_must_use)]
fn render_change_async(
	mut depth_cache: OrInitResMut<'static, DepthCache>,
    value: AsyncVariableNonBlocking<Vec<(Entity, Handle<RenderRes<RenderPipeline>>)>>,
    count: Share<AtomicUsize>,
    task_count: &mut usize,
    pipeline_map: Res<'static, ShareAssetMgr<RenderRes<RenderPipeline>>>,
    shader_map: Res<'static, ShareAssetMgr<RenderRes<Program>>>,

    unit_quad_buffer: Res<'static, UnitQuadBuffer>,
    image_program_meta: OrInitRes<'static, ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitRes<'static, PosUv1VertexLayout>,
    shader_info_catch: OrInitRes<'static, ShaderInfoCache>,
    device: Res<'static, PiRenderDevice>,
    bind_group_assets: Res<'static, ShareAssetMgr<RenderRes<BindGroup>>>,

    // mut copy_draw_obj: WriteRes<'static, CopyFboToScreen>,
    post_bind_group_layout: OrInitRes<'static, PostBindGroupLayout>,
    common_sampler: Res<'static, CommonSampler>,

    // render_target: Res<'static, RenderTarget>,
    // camera_bind_group: Res<'static, DynBindGroupIndex<CameraMatrixGroup>>,
    // post_bind_group: Res<'static, DynBindGroupIndex<UiMaterialGroup>>,
    // common_state: Res<'static, CommonPipelineState>,
    camera_group_alloter: OrInitRes<ShareGroupAlloter<CameraGroup>>,
    ui_meterial_group_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
    allocator: Res<'static, PiSafeAtlasAllocator>,

    mut query: Query<
        'static,
        'static,
        (
            Entity,
            &'static mut CopyFboToScreen,
            &'static mut RenderTarget,
            OrDefault<RenderTargetType>,
            &'static Viewport,
            ChangeTrackers<Viewport>,
            &'static DynTargetType,
        ),
        (With<Viewport>, Or<(Changed<Viewport>, Changed<RenderTargetType>)>),
    >,
) {
    // let queue = Share::new(SegQueue::new());
    let pipeline_map: &'static Res<'static, ShareAssetMgr<RenderRes<RenderPipeline>>> = unsafe { transmute(&pipeline_map) };
    let shader_map: &'static Res<'static, ShareAssetMgr<RenderRes<Program>>> = unsafe { transmute(&shader_map) };
    let device: &'static Res<'static, PiRenderDevice> = unsafe { transmute(&device) };

    for (entity, mut copy_draw_obj, mut render_target, render_target_type, view_port, view_port_ticker, dyn_target_type) in query.iter_mut() {
        if view_port_ticker.is_changed() {
            let last_target = allocator.allocate::<&ShareTargetView, _>(
                (view_port.maxs.x - view_port.mins.x).ceil() as u32,
                (view_port.maxs.y - view_port.mins.y).ceil() as u32,
                dyn_target_type.has_depth,
                [].iter(),
            );
            *render_target = RenderTarget(Some(last_target));
        }

        // 如果是离屏渲染，则不需要创建CopyFboToScreen
        if let RenderTargetType::OffScreen = render_target_type {
            continue;
        }
        let target = render_target.0.as_ref().unwrap();


        // let render_target = if let Some(r) = render_target.get() {
        // 	if view_port_ticker.is_changed()
        // } else {
        // 	// 需要单独的一个target类型
        // 	let last_target = allocator.allocate::<&ShareTargetView, _>(
        // 		(view_port.maxs.x - view_port.mins.x).ceil() as u32,
        // 		(view_port.maxs.y - view_port.mins.y).ceil() as u32,
        // 		dyn_target_type.has_depth,
        // 		[].iter(),
        // 	);
        // 	render_target.write(RenderTarget::Screen(last_target));
        // 	render_target.get().unwrap()
        // };

        // let target = match &*render_target {
        // 	RenderTarget::OffScreen(target) | RenderTarget::Screen(target) => target,
        // };

        // 如果渲染目标不是一个离屏Target，则需要创建一个离屏fbo， 将gui渲染到离屏fbo上，再将fbo渲染到最终目标上
        // 原因是，gui的渲染机制为局部脏更机制，需要保留上一帧的画面，如果不用离屏fbo，在多缓冲模式下，不能保留原有画面
        // 此逻辑创建一个drawobj，用于将离屏的fbo渲染到最终目标上
        let mut draw_state = DrawState::default();
		draw_state.vertex = 0..4;
		draw_state.insert_vertices(RenderVertices { slot: 0, buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.vertex.clone()), buffer_range: None, size_per_value: 8 });
		draw_state.insert_vertices(RenderVertices { slot: 1, buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.uv.clone()), buffer_range: None, size_per_value: 8 });
    	draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.index.clone()), buffer_range: None, format: IndexFormat::Uint16 } );


        let image_static_index = PipelineMeta {
            program: image_program_meta.clone(),
            state: shader_info_catch.premultiply.clone(),
            vert_layout: vert_layout.clone(),
            defines: Default::default(),
        };

        let mut camera_group = camera_group_alloter.alloc();
        let camera_matrix = WorldMatrix::default();
        camera_group.set_uniform(&ProjectUniform(camera_matrix.as_slice()));
        camera_group.set_uniform(&ViewUniform(camera_matrix.as_slice()));
        draw_state
            .bindgroups
            .insert_group(camera_group_alloter.group_index, DrawBindGroup::Offset(camera_group));

        // 世界矩阵
        let world_matrix = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut post_group = ui_meterial_group_alloter.alloc();
        post_group.set_uniform(&WorldUniform(world_matrix.as_slice()));
        draw_state
            .bindgroups
            .insert_group(ui_meterial_group_alloter.group_index, DrawBindGroup::Offset(post_group));
		
		// 深度
		depth_cache.or_create_depth(0, device, &bind_group_assets);
		draw_state.bindgroups.insert_group(DepthBind::set(), DrawBindGroup::Independ(depth_cache.list[0].clone()));

        let group_key = calc_hash(&("bind", target.target().colors[0].0.key()), 0);
        let texture_bind = match bind_group_assets.get(&group_key) {
            Some(r) => r,
            None => {
                let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &post_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&target.target().colors[0].0),
                        },
                    ],
                    label: Some("post process texture bind group create"),
                });
                bind_group_assets.insert(group_key, RenderRes::new(group, 5)).unwrap()
            }
        };
        draw_state.bindgroups.insert_group(SampBind::set(), DrawBindGroup::Independ(texture_bind));
        *copy_draw_obj = CopyFboToScreen(Some(draw_state));

        let value_copy = value.clone();
        let count_copy = count.clone();
        count_copy.fetch_add(1, Ordering::Relaxed);
        *task_count += 1;
        // log::warn!("zzzzzzzzzzzzzzzzzzzzzzzzzzz====={:p}", &shader_statics.0);
        RENDER_RUNTIME
            .spawn(RENDER_RUNTIME.alloc(), async move {
                match calc_pipeline(&image_static_index, device, pipeline_map, shader_map).await {
                    Ok(r) => {
                        let mut locked = value_copy.lock().unwrap();
                        if let &None = &*locked {
                            *locked = Some(Vec::new());
                        }
                        let value = locked.as_mut().unwrap();
                        value.push((entity, r));

                        if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                            locked.finish();
                        }
                    }
                    Err(e) => {
                        let locked = value_copy.lock().unwrap();
                        if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                            locked.finish();
                        }
                        panic!("create CopyFboToScreen pipeline fail, {:?}", e)
                    }
                };
            })
            .unwrap();
    }
}

fn set_result(mut query: Query<&'static mut CopyFboToScreen>, result: &mut Vec<(Entity, Handle<RenderRes<RenderPipeline>>)>) {
    while let Some((id, pipeline)) = result.pop() {
        if let Ok(mut copy_draw_obj) = query.get_mut(id) {
            if let Some(r) = &mut copy_draw_obj.0 {
                r.pipeline = Some(pipeline);
            }
        }
    }
}

/// 创建图节点所需要的数据
/// 如： DynTargetType (需要根据视口变化及时调整)
pub fn calc_dyn_target_type(
    mut query: Query<(&Viewport, Option<&mut DynTargetType>, Entity), Changed<Viewport>>,

    atlas_allocator: Res<PiSafeAtlasAllocator>,
    mut max_view_size: ResMut<MaxViewSize>,

    mut commands: Commands,
) {
    for (view_port, dyn_target_type, entity) in query.iter_mut() {
        max_view_size.width = max_view_size.width.min((view_port.maxs.x - view_port.mins.x).ceil() as u32);
        max_view_size.height = max_view_size.height.min((view_port.maxs.y - view_port.mins.y).ceil() as u32);
        let ty = create_dyn_target_type(&atlas_allocator, max_view_size.width, max_view_size.height);
        match dyn_target_type {
            Some(mut r) => *r = ty,
            None => {
                commands.entity(entity).insert(ty);
            }
        };
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
