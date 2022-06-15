use std::collections::hash_map::Entry;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::prelude::{Query, Changed, Added, ResMut, Res, OrDefault, Or};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, pipeline::RenderPipeline, bind_group_layout::BindGroupLayout, bind_group::BindGroup, asset::RenderRes};
use pi_share::Share;

use crate::{
	components::draw_obj::{VSDefines, FSDefines, DrawObject, DrawState}, 
	resource::draw_obj::{Shaders, ShaderInfoMap, VertexBufferLayoutMap, PipelineMap, StateMap, ShaderCatch, ShareLayout}, system::shader_utils::StaticIndex
};
use crate::utils::tools::calc_hash;

pub struct CalcPipeline;

#[setup]
impl CalcPipeline {
	/// 计算DrawObj的pipeline
	#[system]
	pub fn calc_node_pipeline<'a>(
		mut query_draw: Query<
			DrawObject, 
			(
				OrDefault<VSDefines>,
				OrDefault<FSDefines>,
				&StaticIndex,
				&mut DrawState,
			),
			Or<(Changed<VSDefines>, Changed<FSDefines>, Added<DrawState>)>>,
		device: Res<'a, RenderDevice>,
		mut shader_map: ResMut<'a, ShaderInfoMap>,
		shader_statics: Res<'a, Shaders>,
		state_map: Res<'a, StateMap>,
		mut pipeline_map: ResMut<'a, PipelineMap>,
		vertex_buffer_layout_map: Res<'a, VertexBufferLayoutMap>,
		shader_catch: Res<'a, ShaderCatch>,
		share_layout: Res<'a, ShareLayout>,
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
		println!("====={:?}, {:?}", &vs_defines.0, &fs_defines.0);
		match pipeline_map.0.entry(calc_hash(&(static_index.shader, vs_defines, fs_defines, static_index.pipeline_state))) {
			Entry::Vacant(_r) => {
				// 缓存未命中pipeline，取到编译后的shader（也是先从缓存命中）
				let shader_info = match shader_map.0.entry(calc_hash(&(static_index.shader, vs_defines, fs_defines))) {
					Entry::Vacant(r) => {
						// 如果缓存未命中shader，从缓存表中取到shader的静态信息
						let shader = match shader_statics.0.get(static_index.shader) {
							Some(r) => r,
							None => panic!("shader is not exist, create pipeline fail!"),
						};
						
						// 静态信息的group_layout， 插入depth_layout和camera_layout
						let bind_group_layout = shader.bind_group.clone();
						
						// 创建编译后的shader
						let shader_info = (shader.create_shader_info)(
							&shader.vs_shader_soruce,
							&shader.fs_shader_soruce,
							&vs_defines,
							&fs_defines,
							bind_group_layout,
							&share_layout.empty,
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
					label: Some("bg_color pipeline"),
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
				Share::new(pipeline)
			},
			Entry::Occupied(r) => r.get().clone(),
		}
	}
}

pub fn create_empty_bind_group(
	device: &RenderDevice, 
	group_layout: &BindGroupLayout,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&"empty group");
	let r = device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: group_layout,
		entries: &[],
		label: Some("color group create"),
	});

	bind_group_assets.cache(key, RenderRes::new(r, 5));
	bind_group_assets.get(&key).unwrap()
}

