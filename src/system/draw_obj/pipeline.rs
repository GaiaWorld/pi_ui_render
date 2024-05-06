use std::{
    mem::transmute,
    sync::atomic::{AtomicUsize, Ordering},
};

use pi_world::prelude::{Changed, SingleRes, Query, Entity};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_assets::{
    asset::{GarbageEmpty, Handle},
    mgr::{AssetMgr, LoadResult, Receiver},
};
use pi_async_rt::prelude::{AsyncRuntime, AsyncRuntimeExt, AsyncVariableNonBlocking};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_hal::runtime::RENDER_RUNTIME;
use pi_render::rhi::{asset::RenderRes, device::RenderDevice, pipeline::RenderPipeline};
use pi_share::Share;

use crate::utils::tools::calc_hash;
use crate::{
    components::draw_obj::{DrawState, PipelineMeta},
    resource::draw_obj::Program,
};

use super::calc_text::IsRun;

/// 计算DrawObj的pipeline
pub fn calc_node_pipeline(
    query_draw: Query<(Entity, &'static PipelineMeta), Changed<PipelineMeta>>,
    draw_state: Query<&'static mut DrawState>,
    device: SingleRes<PiRenderDevice>,
    // state_map: SingleRes<StateMap>,

    // shader_catch: SingleRes<ShaderCatch>,
    pipeline_map: SingleRes<ShareAssetMgr<RenderRes<RenderPipeline>>>,
    shader_map: SingleRes<ShareAssetMgr<RenderRes<Program>>>,
    // mut pipeline_map: SingleResMut<PipelineMap>,
    // mut shader_map: SingleResMut<ShaderInfoMap>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let query_draw: Query<'static, (Entity, &'static PipelineMeta), Changed<PipelineMeta>> = unsafe { transmute(query_draw) };
    let draw_state: Query<'static, &'static mut DrawState> = unsafe { transmute(draw_state) };
    let device: SingleRes<'static, PiRenderDevice> = unsafe { transmute(device) };
    // let shader_statics: SingleRes<'static,Shaders> = unsafe { transmute(shader_statics)};
    // let state_map: SingleRes<'static,StateMap> = unsafe { transmute(state_map)};
    // let shader_catch: SingleRes<'static,ShaderCatch> = unsafe { transmute(shader_catch)};

    let pipeline_map: SingleRes<'static, ShareAssetMgr<RenderRes<RenderPipeline>>> = unsafe { transmute(pipeline_map) };
    let shader_map: SingleRes<'static, ShareAssetMgr<RenderRes<Program>>> = unsafe { transmute(shader_map) };

    RENDER_RUNTIME
        .block_on(async move {
            calc_node_pipeline1(query_draw, draw_state, device, pipeline_map, shader_map).await;
        })
        .unwrap();
}

/// 计算DrawObj的pipeline
pub async fn calc_node_pipeline1(
    query_draw: Query<'static, (Entity, &'static PipelineMeta), Changed<PipelineMeta>>,
    mut draw_state_query: Query<'static, &'static mut DrawState>,
    device: SingleRes<'static, PiRenderDevice>,
    pipeline_map: SingleRes<'static, ShareAssetMgr<RenderRes<RenderPipeline>>>,
    shader_map: SingleRes<'static, ShareAssetMgr<RenderRes<Program>>>,
) {
    let value = AsyncVariableNonBlocking::<(
        Vec<(Entity, Handle<RenderRes<RenderPipeline>>)>,
        usize,
        Option<Handle<RenderRes<RenderPipeline>>>,
    )>::new();
    let count = Share::new(AtomicUsize::new(0));
    let mut task_count = 0;
    // log::warn!("shader_statics==============={:p}", &shader_statics.0);
    // log::warn!("shader_map==============={:p}, {:p}", &shader_map.0, &*shader_map.0, );

    let device = unsafe { transmute::<_, &'static RenderDevice>(&*device) };
    // log::warn!("shader_statics1==============={:p}", &shader_statics.0);
    for (id, pipeline_meta) in query_draw.iter() {
        let mut draw_state = match draw_state_query.get_mut(id) {
            Ok(r) => r,
            _ => continue,
        };

        let hash = calc_hash(pipeline_meta, 0);

        let load = AssetMgr::load(&pipeline_map, &hash);
        let pipeline_receiver = match load {
            LoadResult::Ok(pipeline) => {
                draw_state.pipeline = Some(pipeline);
                continue;
            }
            LoadResult::Wait(r) => {
                let value_copy = value.clone();
                let count_copy = count.clone();
                count_copy.fetch_add(1, Ordering::Relaxed);
                task_count += 1;
                RENDER_RUNTIME
                    .spawn(async move {
                        match r.await {
                            Ok(r) => {
                                let mut locked = value_copy.lock().unwrap();
                                if let &None = &*locked {
                                    *locked = Some((Vec::new(), 0, None));
                                }
                                let value = locked.as_mut().unwrap();
                                value.0.push((id, r));

                                if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                                    locked.finish();
                                }
                            }
                            Err(e) => {
                                let locked = value_copy.lock().unwrap();
                                if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                                    locked.finish();
                                }
                                log::error!("{:?}", e);
                            }
                        };
                    })
                    .unwrap();
                continue;
            }
            LoadResult::Receiver(r) => r,
        };

        let (shader_program, shader_map) = (pipeline_meta.clone(), shader_map.clone());
        let value_copy = value.clone();
        let count_copy = count.clone();
        count_copy.fetch_add(1, Ordering::Relaxed);
        task_count += 1;
        RENDER_RUNTIME
            .spawn(async move {
                match async_calc_pipeline(&shader_program, &device, &shader_map, pipeline_receiver, hash).await {
                    Ok(r) => {
                        let mut locked = value_copy.lock().unwrap();
                        if let &None = &*locked {
                            *locked = Some((Vec::new(), 0, None));
                        }
                        let value = locked.as_mut().unwrap();
                        value.0.push((id, r));
                        if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                            locked.finish();
                        }
                    }
                    Err(e) => {
                        let locked = value_copy.lock().unwrap();
                        if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
                            locked.finish();
                        }
                        log::error!("{:?}", e);
                    }
                }
            })
            .unwrap();
    }

    // // let clear_pipeline = Share::new(ShareMutex::new(None));
    // if let None = clear_color_obj.0.pipeline {
    //     let pipeline_meta = &clear_color_obj.1;
    //     let hash = calc_hash(pipeline_meta, 0);

    //     let load = AssetMgr::load(&pipeline_map, &hash);
    //     match load {
    //         LoadResult::Ok(pipeline) => {
    //             clear_color_obj.0.pipeline = Some(pipeline);
    //         }
    //         LoadResult::Wait(r) => {
    //             let value_copy = value.clone();
    //             let count_copy = count.clone();
    //             count_copy.fetch_add(1, Ordering::Relaxed);
    //             task_count += 1;
    //             RENDER_RUNTIME
    //                 .spawn(async move {
    //                     match r.await {
    //                         Ok(r) => {
    //                             let mut locked = value_copy.lock().unwrap();
    //                             if let &None = &*locked {
    //                                 *locked = Some((Vec::new(), 0, None));
    //                             }
    //                             let value = locked.as_mut().unwrap();
    //                             value.2 = Some(r);
    //                             if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
    //                                 locked.finish();
    //                             }
    //                         }
    //                         Err(e) => {
    //                             let locked = value_copy.lock().unwrap();
    //                             if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
    //                                 locked.finish();
    //                             }
    //                             log::error!("{:?}", e);
    //                         }
    //                     };
    //                     ()
    //                 })
    //                 .unwrap();
    //         }
    //         LoadResult::Receiver(r) => {
    //             let value_copy = value.clone();
    //             let count_copy = count.clone();
    //             count_copy.fetch_add(1, Ordering::Relaxed);
    //             task_count += 1;
    //             let (shader_meta, shader_map) = (pipeline_meta.clone(), shader_map.clone());
    //             RENDER_RUNTIME
    //                 .spawn(async move {
    //                     match async_calc_pipeline(&shader_meta, &device, &shader_map, r, hash).await {
    //                         Ok(r) => {
    //                             let mut locked = value_copy.lock().unwrap();
    //                             if let &None = &*locked {
    //                                 *locked = Some((Vec::new(), 0, None));
    //                             }
    //                             let value = locked.as_mut().unwrap();
    //                             value.2 = Some(r);
    //                             if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
    //                                 locked.finish();
    //                             }
    //                         }
    //                         Err(e) => {
    //                             let locked = value_copy.lock().unwrap();
    //                             if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
    //                                 locked.finish();
    //                             }
    //                             log::error!("{:?}", e);
    //                         }
    //                     }
    //                     ()
    //                 })
    //                 .unwrap();
    //         }
    //     };
    // }

    // 没有任务，返回
    if task_count > 0 {
        let mut result = value.await;
        set_result(&mut draw_state_query, &mut result);
    }
}

fn set_result(
    query_draw: &mut Query<&'static mut DrawState>,
    result: &mut (
        Vec<(Entity, Handle<RenderRes<RenderPipeline>>)>,
        usize,
        Option<Handle<RenderRes<RenderPipeline>>>,
    ),
) {
    while let Some((id, pipeline)) = result.0.pop() {
        if let Ok(mut draw_state) = query_draw.get_mut(id) {
            draw_state.pipeline = Some(pipeline);
        }
    }

    // if let Some(r) = &result.2 {
    //     clear_color_obj.0.pipeline = Some(r.clone());
    // }
}

async fn async_calc_pipeline(
    pipeline_meta: &PipelineMeta,

    device: &RenderDevice,
    // state_map: &StateMap,
    // shader_catch: &ShaderCatch,

    // pipeline_map: &PipelineMap,
    // shader_map: &ShaderInfoMap,
    shader_map: &Share<AssetMgr<RenderRes<Program>>>,

    pipeline_receiver: Receiver<RenderRes<RenderPipeline>, GarbageEmpty>,
    hash: u64,
) -> Result<Handle<RenderRes<RenderPipeline>>, std::io::Error> {
    // println!("====={:?}, {:?}", &vs_defines.0, &fs_defines.0);
    // let time = std::time::Instant::now();

    let shader_hash = calc_hash(pipeline_meta, 0);
    let shader_info = match AssetMgr::load(&shader_map, &shader_hash) {
        LoadResult::Ok(r) => Ok(r),
        LoadResult::Wait(r) => r.await,
        LoadResult::Receiver(shader_receiver) => {
            // 创建编译后的shader
            let shader_info = pipeline_meta.program.create_program(&pipeline_meta.defines, &device);
            shader_receiver.receive(shader_hash, Ok(RenderRes::new(shader_info, 5))).await
        }
    }?;

    let vertex_buffer_layout: Vec<wgpu::VertexBufferLayout> = pipeline_meta
        .vert_layout
        .iter()
        .map(|r| wgpu::VertexBufferLayout {
            array_stride: r.array_stride,
            step_mode: r.step_mode,
            attributes: &r.attributes,
        })
        .collect();
	
	let mut name = pipeline_meta.program.shader_meta.name.clone();
	for r in pipeline_meta.defines.iter() {
		name += "-";
		name += r.as_str();
	};
	
    // 创建pipline
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&name),
        layout: Some(&shader_info.pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_info.vs_shader,
            entry_point: "main",
            buffers: vertex_buffer_layout.as_slice(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_info.fs_shader,
            entry_point: "main",
            targets: pipeline_meta.state.targets.as_slice(),
        }),
        primitive: pipeline_meta.state.primitive.clone(),
        depth_stencil: pipeline_meta.state.depth_stencil.clone(),
        multisample: pipeline_meta.state.multisample.clone(),
        multiview: pipeline_meta.state.multiview.clone(),
    });
    pipeline_receiver.receive(hash, Ok(RenderRes::new(pipeline, 5))).await
}

pub async fn calc_pipeline(
    shader_meta: &PipelineMeta,

    device: &RenderDevice,

    pipeline_map: &Share<AssetMgr<RenderRes<RenderPipeline>>>,
    shader_map: &Share<AssetMgr<RenderRes<Program>>>,
) -> Result<Handle<RenderRes<RenderPipeline>>, std::io::Error> {
    let hash = calc_hash(shader_meta, 0);

    let load = AssetMgr::load(pipeline_map, &hash);
    match load {
        LoadResult::Ok(pipeline) => Ok(pipeline),
        LoadResult::Wait(r) => r.await,
        LoadResult::Receiver(pipeline_receiver) => async_calc_pipeline(shader_meta, device, shader_map, pipeline_receiver, hash).await,
    }
}
