use std::{mem::transmute, sync::atomic::{AtomicUsize, Ordering}};

use pi_assets::{mgr::{AssetMgr, LoadResult, Receiver}, asset::{Handle, GarbageEmpty}};
use pi_async::prelude::{AsyncRuntime, AsyncVariable};
use pi_ecs::prelude::{Query, Changed, Added, Res, OrDefault, Or, Id, ParamSet, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, pipeline::RenderPipeline, asset::RenderRes};
use pi_share::Share;
use pi_hal::runtime::RENDER_RUNTIME;

use crate::{
	components::draw_obj::{VSDefines, FSDefines, DrawObject, DrawState}, 
	resource::{draw_obj::{Shaders, VertexBufferLayoutMap, StateMap, ShaderCatch, StaticIndex, Program, ClearDrawObj}}
};
use crate::utils::tools::calc_hash;

pub struct CalcPipeline;

#[setup]
impl CalcPipeline {
	/// 计算DrawObj的pipeline
	#[system]
	pub async fn calc_node_pipeline(
		mut query_draw: ParamSet<(
			Query<'static, 'static,
				DrawObject, 
				(
					Id<DrawObject>,
					OrDefault<VSDefines>,
					OrDefault<FSDefines>,
					&'static StaticIndex,
					&'static mut DrawState,
				),
				Or<(Changed<VSDefines>, Changed<FSDefines>, Added<DrawState>)>>,
			Query<'static, 'static, DrawObject, &'static mut DrawState>
		)>,
		device: Res<'static, RenderDevice>,
		shader_statics: Res<'static, Shaders>,
		state_map: Res<'static, StateMap>,

		vertex_buffer_layout_map: Res<'static, VertexBufferLayoutMap>,
		shader_catch: Res<'static, ShaderCatch>,

		pipeline_map: Res<'static, Share<AssetMgr<RenderRes<RenderPipeline>>>>,
		shader_map: Res<'static, Share<AssetMgr<RenderRes<Program>>>>,
		// mut pipeline_map: ResMut<'static, PipelineMap>,
		// mut shader_map: ResMut<'static, ShaderInfoMap>,

		mut clear_color_obj: ResMut<'static, ClearDrawObj>,
	) -> std::io::Result<()> {
		let value = AsyncVariable::<(Vec<(Id<DrawObject>, Handle<RenderRes<RenderPipeline>>)>, usize, Option<Handle<RenderRes<RenderPipeline>>>)>::new();
		let count = Share::new(AtomicUsize::new(0));
		
		let (
			shader_statics, 
			state_map, 
			vertex_buffer_layout_map, 
			shader_catch,
			device) = (
				unsafe {transmute::<_, &'static Shaders>(&*shader_statics)} , 
				unsafe {transmute::<_, &'static StateMap>(&*state_map)}, 
				unsafe {transmute::<_, &'static VertexBufferLayoutMap>(&*vertex_buffer_layout_map)}, 
				unsafe {transmute::<_, &'static ShaderCatch>(&*shader_catch)},
				unsafe {transmute::<_, &'static RenderDevice>(&*device)},
			);
		for (
			id,
			vs_defines, 
			fs_defines, 
			static_index,
			mut draw_state) in query_draw.p0_mut().iter_mut() {
			let hash = calc_hash(&(static_index.shader, static_index.vertex_buffer_index, vs_defines, fs_defines, static_index.pipeline_state), 0);
			
			let load = AssetMgr::load(&pipeline_map, &hash);
			let pipeline_receiver = match load {
				LoadResult::Ok(pipeline) => {
					draw_state.pipeline = Some(pipeline);
					continue;
				},
				LoadResult::Wait(r) => {
					let value_copy = value.clone();
					let count_copy = count.clone();
					count_copy.fetch_add(1, Ordering::Relaxed);
					RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
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
							},
							Err(e) => {
								let locked = value_copy.lock().unwrap();
								if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
									locked.finish();
								}
								log::error!("{:?}", e);
							},
						};
					}).unwrap();
					continue;
				},
				LoadResult::Receiver(r) => r
			};

			let (
				vs_defines, 
				fs_defines, 
				static_index, 
				shader_map
			) = (vs_defines.clone(), fs_defines.clone(), static_index.clone(), shader_map.clone());
			let value_copy = value.clone();
			let count_copy = count.clone();
			count_copy.fetch_add(1, Ordering::Relaxed);
			RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
				match Self::async_calc_pipeline(
					&vs_defines,
					&fs_defines,
					&static_index,

					shader_statics,
					&device,
					&vertex_buffer_layout_map,
					&state_map,
					&shader_catch,

					&shader_map,
					pipeline_receiver,
					hash
				).await {
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
					},
					Err(e) => {
						let locked = value_copy.lock().unwrap();
						if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
							locked.finish();
						}
						log::error!("{:?}", e);
					},
				}
			}).unwrap();
		}

		// let clear_pipeline = Share::new(ShareMutex::new(None));
		if let None = clear_color_obj.0.pipeline {
			let (vs_defines, fs_defines) = (VSDefines::default(), FSDefines::default());
			let static_index = &clear_color_obj.1;
			let hash = calc_hash(&(static_index.shader, static_index.vertex_buffer_index, &vs_defines, &fs_defines, static_index.pipeline_state), 0);
			
			let load = AssetMgr::load(&pipeline_map, &hash);
			match load {
				LoadResult::Ok(pipeline) => {
					clear_color_obj.0.pipeline = Some(pipeline);
				},
				LoadResult::Wait(r) => {
					let value_copy = value.clone();
					let count_copy = count.clone();
					count_copy.fetch_add(1, Ordering::Relaxed);
					RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
						match r.await {
							Ok(r) => {
								let mut locked = value_copy.lock().unwrap();
								if let &None = &*locked {
									*locked = Some((Vec::new(), 0, None));
								}
								let value = locked.as_mut().unwrap();
								value.2 = Some(r);
								if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
									locked.finish();
								}
								
							},
							Err(e) => {
								let locked = value_copy.lock().unwrap();
								if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
									locked.finish();
								}
								log::error!("{:?}", e);
							}
						};
						()
					}).unwrap();
				},
				LoadResult::Receiver(r) => {
					let value_copy = value.clone();
					let count_copy = count.clone();
					count_copy.fetch_add(1, Ordering::Relaxed);
					let (
						vs_defines, 
						fs_defines, 
						static_index, 
						shader_map
					) = (vs_defines.clone(), fs_defines.clone(), static_index.clone(), shader_map.clone());
					RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
						match Self::async_calc_pipeline(
							&vs_defines,
							&fs_defines,
							&static_index,
		
							shader_statics,
							&device,
							&vertex_buffer_layout_map,
							&state_map,
							&shader_catch,
		
							&shader_map,
							r,
							hash
						).await {
							Ok(r) => {
								let mut locked = value_copy.lock().unwrap();
								if let &None = &*locked {
									*locked = Some((Vec::new(), 0, None));
								}
								let value = locked.as_mut().unwrap();
								value.2 = Some(r);
								if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
									locked.finish();
								}
							},
							Err(e) => {
								let locked = value_copy.lock().unwrap();
								if count_copy.fetch_sub(1, Ordering::Relaxed) == 1 {
									locked.finish();
								}
								log::error!("{:?}", e);
							},
						}
						()
					}).unwrap();
				}
			};
		}

		// 没有任务，返回
		if count.load(Ordering::Relaxed) == 0 {
			return Ok(());
		}

		let mut result = value.await;
		Self::set_result(query_draw.p1_mut(), &mut clear_color_obj, &mut result);
		Ok(())
	}

	fn set_result(
		query_draw: &mut Query<'static, 'static, DrawObject, &'static mut DrawState>,
		clear_color_obj: &mut ClearDrawObj,
		result: &mut (Vec<(Id<DrawObject>, Handle<RenderRes<RenderPipeline>>)>, usize, Option<Handle<RenderRes<RenderPipeline>>>),
	) {
		while let Some((id, pipeline)) = result.0.pop() {
			if let Some(mut draw_state) = query_draw.get_mut(id) {
				draw_state.pipeline = Some(pipeline);
			}
		}

		if let Some(r) = &result.2 { 
			clear_color_obj.0.pipeline = Some(r.clone());
		}
	}

	async fn async_calc_pipeline(
		vs_defines: &VSDefines,
		fs_defines: &FSDefines,
		static_index: &StaticIndex,

		shader_statics: &Shaders,
		device: &RenderDevice,
		vertex_buffer_layout_map: &VertexBufferLayoutMap,
		state_map: &StateMap,
		shader_catch: &ShaderCatch,

		// pipeline_map: &PipelineMap,
		// shader_map: &ShaderInfoMap,
		shader_map: &Share<AssetMgr<RenderRes<Program>>>,

		pipeline_receiver: Receiver<RenderRes<RenderPipeline>, GarbageEmpty>,
		hash: u64,
	) -> Result<Handle<RenderRes<RenderPipeline>>, std::io::Error>  {
		// println!("====={:?}, {:?}", &vs_defines.0, &fs_defines.0);
		// let time = std::time::Instant::now();
		
		let shader_hash = calc_hash(&(static_index.shader, vs_defines, fs_defines), 0);
		let shader_info = match AssetMgr::load(&shader_map, &shader_hash) {
			LoadResult::Ok(r) => Ok(r),
			LoadResult::Wait(r) => r.await,
			LoadResult::Receiver(shader_receiver) => {
				// 如果缓存未命中shader，从缓存表中取到shader的静态信息
				let shader = match shader_statics.0.get(static_index.shader) {
					Some(r) => r,
					None => panic!("shader is not exist, create pipeline fail!"),
				};
				
				// 创建编译后的shader
				let shader_info = shader.create_shader_info(
					&vs_defines,
					&fs_defines,
					&device,
					&shader_catch.0,
				);
				shader_receiver.receive(shader_hash, Ok(RenderRes::new(shader_info, 5))).await
			}
		}?;


		let vertex_buffer_layout = (*vertex_buffer_layout_map).get(static_index.vertex_buffer_index).unwrap();
		let vertex_buffer_layout: Vec<wgpu::VertexBufferLayout> = vertex_buffer_layout.iter().map(|r| {
			wgpu::VertexBufferLayout {
				array_stride: r.array_stride,
				step_mode: r.step_mode,
				attributes: &r.attributes,
			}
		}).collect();
		let pipeline_state = state_map.get(static_index.pipeline_state).unwrap();

		// 创建pipline
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some(static_index.name),
			layout: Some(&shader_info.pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader_info.vs_shader,
				entry_point: "main",
				buffers: vertex_buffer_layout.as_slice(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader_info.fs_shader,
				entry_point: "main",
				targets: pipeline_state.targets.as_slice(),
			}),
			primitive: pipeline_state.primitive.clone(),
			depth_stencil: pipeline_state.depth_stencil.clone(),
			multisample: pipeline_state.multisample.clone(),
			multiview: pipeline_state.multiview.clone(),
		});
		pipeline_receiver.receive(hash, Ok(RenderRes::new(pipeline, 5))).await
	}

	pub async fn calc_pipeline(
		vs_defines: &VSDefines,
		fs_defines: &FSDefines,
		static_index: &StaticIndex,

		shader_statics: &Shaders,
		device: &RenderDevice,
		vertex_buffer_layout_map: &VertexBufferLayoutMap,
		state_map: &StateMap,
		shader_catch: &ShaderCatch,

		pipeline_map: &Share<AssetMgr<RenderRes<RenderPipeline>>>,
		shader_map: &Share<AssetMgr<RenderRes<Program>>>,
	) -> Result<Handle<RenderRes<RenderPipeline>>, std::io::Error> {
		let hash = calc_hash(&(static_index.shader, static_index.vertex_buffer_index, vs_defines, fs_defines, static_index.pipeline_state), 0);

		let load = AssetMgr::load(pipeline_map, &hash);
		match load {
			LoadResult::Ok(pipeline) => Ok(pipeline),
			LoadResult::Wait(r) => r.await,
			LoadResult::Receiver(pipeline_receiver) => Self::async_calc_pipeline(vs_defines, fs_defines, static_index, shader_statics, device, vertex_buffer_layout_map, state_map, shader_catch, shader_map, pipeline_receiver, hash).await
		}
	}
}



