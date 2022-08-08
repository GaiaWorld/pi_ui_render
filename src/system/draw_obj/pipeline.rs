use std::collections::hash_map::Entry;

use pi_assets::{mgr::{AssetMgr, LoadResult}, asset::Handle};
use pi_ecs::prelude::{Query, Changed, Added, ResMut, Res, OrDefault, Or};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, pipeline::RenderPipeline, bind_group_layout::BindGroupLayout, bind_group::BindGroup, asset::RenderRes};
use pi_share::Share;

use crate::{
	components::draw_obj::{VSDefines, FSDefines, DrawObject, DrawState}, 
	resource::draw_obj::{Shaders, ShaderInfoMap, VertexBufferLayoutMap, PipelineMap, StateMap, ShaderCatch, ShareLayout, StaticIndex, Program}
};
use crate::utils::tools::calc_hash;

pub struct CalcPipeline;

#[setup]
impl CalcPipeline {
	/// 计算DrawObj的pipeline
	#[system]
	pub fn calc_node_pipeline<'a>(
		mut query_draw: Query<'a, 'a,
			DrawObject, 
			(
				OrDefault<VSDefines>,
				OrDefault<FSDefines>,
				&'a StaticIndex,
				&'a mut DrawState,
			),
			Or<(Changed<VSDefines>, Changed<FSDefines>, Added<DrawState>)>>,
		device: Res<'a, RenderDevice>,
		shader_statics: Res<'a, Shaders>,
		state_map: Res<'a, StateMap>,

		vertex_buffer_layout_map: Res<'a, VertexBufferLayoutMap>,
		shader_catch: Res<'a, ShaderCatch>,
		share_layout: Res<'a, ShareLayout>,

		// pipeline_map: Res<'a, Share<AssetMgr<RenderRes<RenderPipeline>>>>,
		// shader_map: Res<'a, Share<AssetMgr<RenderRes<Program>>>>,
		mut pipeline_map: ResMut<'a, PipelineMap>,
		mut shader_map: ResMut<'a, ShaderInfoMap>,
	) {
		for (
			vs_defines, 
			fs_defines, 
			static_index,
			mut draw_state) in query_draw.iter_mut() {
			
			// 根据shader_id、vs_defines、fs_defines、pipeline_state的hash命中RenderPipeline
			let pipeline = Self::calc_pipeline(
				&vs_defines,
				&fs_defines,
				&static_index,

				&shader_statics,
				&device,
				&vertex_buffer_layout_map,
				&state_map,
				&shader_catch,

				&mut pipeline_map,
				&mut shader_map,
				&share_layout,
			);

			// 设置pipeline
			draw_state.pipeline = Some(pipeline);
		}
	}

	pub async fn async_calc_pipeline(
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
		pipeline_map: &Share<AssetMgr<RenderRes<RenderPipeline>>>,
		shader_map: &Share<AssetMgr<RenderRes<Program>>>,

		share_layout: &ShareLayout,
	) -> Result<Handle<RenderRes<RenderPipeline>>, std::io::Error>  {
		// println!("====={:?}, {:?}", &vs_defines.0, &fs_defines.0);
		// let time = std::time::Instant::now();
		let hash = calc_hash(&(static_index.shader, static_index.vertex_buffer_index, vs_defines, fs_defines, static_index.pipeline_state), 0);

		match AssetMgr::load(&pipeline_map, &hash) {
			LoadResult::Ok(r) => Ok(r),
			LoadResult::Wait(r) => r.await,
			LoadResult::Receiver(pipeline_receiver) => {
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
			},
		}
	}

	pub fn calc_pipeline(
		vs_defines: &VSDefines,
		fs_defines: &FSDefines,
		static_index: &StaticIndex,

		shader_statics: &Shaders,
		device: &RenderDevice,
		vertex_buffer_layout_map: &VertexBufferLayoutMap,
		state_map: &StateMap,
		shader_catch: &ShaderCatch,

		pipeline_map: &mut PipelineMap,
		shader_map: &mut ShaderInfoMap,
		share_layout: &ShareLayout,
	) -> Share<RenderPipeline> {
		// println!("====={:?}, {:?}", &vs_defines.0, &fs_defines.0);
		// let time = std::time::Instant::now();
		let r = match pipeline_map.0.entry(calc_hash(&(static_index.shader, static_index.vertex_buffer_index, vs_defines, fs_defines, static_index.pipeline_state), 0)) {
			Entry::Vacant(r) => {
				// 缓存未命中pipeline，取到编译后的shader（也是先从缓存命中）
				let shader_info = match shader_map.0.entry(calc_hash(&(static_index.shader, vs_defines, fs_defines), 0)) {
					Entry::Vacant(r) => {
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
						r.insert(Share::new(shader_info)).clone()
					},
					Entry::Occupied(r) => r.get().clone(),
				};
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
				let p = Share::new(pipeline);
				r.insert(p).clone()
			},
			Entry::Occupied(r) => r.get().clone(),
		};
		// log::warn!("calc_pipeline, time: {:?}, vs_defines: {:?}, fs_defines: {:?}, static_index: {:?}", std::time::Instant::now() - time, vs_defines, fs_defines, static_index.name);
		r
	}
}

pub fn create_empty_bind_group(
	device: &RenderDevice, 
	group_layout: &BindGroupLayout,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&"empty group", 0);
	let r = device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: group_layout,
		entries: &[],
		label: Some("color group create"),
	});

	bind_group_assets.insert(key, RenderRes::new(r, 5)).unwrap()
}

