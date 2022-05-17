use std::collections::hash_map::Entry;

use pi_ecs::prelude::{Query, Changed, Added, ResMut, Res, OrDefault, Or};
use pi_ecs_macros::setup;
use pi_render::rhi::device::RenderDevice;
use pi_share::Share;

use crate::{components::draw_obj::{VSDefines, FSDefines, VertexBufferLayoutKey, PipelineKey, ShaderKey, DrawObject, DrawState}, resource::draw_obj::{Shaders, ShaderInfoMap, VertexBufferLayoutMap, PipelineMap, StateMap, ShaderCatch}};
use crate::utils::tools::calc_hash;

pub struct CalcPipeline;

#[setup]
impl CalcPipeline {
	// /// 初始化
	// /// RenderDevice必须已经存在于单例中，否则将会崩溃
	// #[init]
	// pub fn init(world: &mut World) {
	// 	let device = match unsafe { &mut *(world as *mut World as usize as *mut World)} .get_resource::<RenderDevice>() {
	// 		Some(r) => r,
	// 		None => panic!("init ColorStatic fail, RenderDevice is not exist")
	// 	};
	// }

	/// 计算DrawObj的pipeline
	#[system]
	pub fn calc_pipeline<'a>(
		mut query_draw: Query<
			DrawObject, 
			(
				OrDefault<VSDefines>,
				OrDefault<FSDefines>,
				&ShaderKey,
				&mut DrawState,
				&PipelineKey,
				&VertexBufferLayoutKey,
			),
			Or<(Changed<VSDefines>, Changed<FSDefines>, Added<DrawState>)>>,
		device: Res<'a, RenderDevice>,
		mut shader_map: ResMut<'a, ShaderInfoMap>,
		shader_statics: Res<'a, Shaders>,
		state_map: Res<'a, StateMap>,
		mut pipeline_map: ResMut<'a, PipelineMap>,
		 vertex_buffer_layout_map: Res<'a, VertexBufferLayoutMap>,
		shader_catch: Res<'a, ShaderCatch>,
	) {
		for (
			vs_defines, 
			fs_defines, 
			shader_id, 
			mut draw_state,
			pipeline_state,
			vertex_buffer_layouts) in query_draw.iter_mut() {
			
			// 根据shader_id、vs_defines、fs_defines、pipeline_state的hash命中RenderPipeline
			let pipeline = match pipeline_map.0.entry(calc_hash(&(shader_id, vs_defines, fs_defines, &pipeline_state.0))) {
				Entry::Vacant(_r) => {
					// 缓存未命中pipeline，取到编译后的shader（也是先从缓存命中）
					let shader_info = match shader_map.0.entry(calc_hash(&(shader_id, vs_defines, fs_defines))) {
						Entry::Vacant(r) => {
							// 如果缓存未命中shader，从缓存表中取到shader的静态信息
							let shader = match shader_statics.0.get(shader_id.0) {
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
								&device,
								&shader_catch.0,
							);
							r.insert(Share::new(shader_info)).clone()
						},
						Entry::Occupied(r) => r.get().clone(),
					};
					let vertex_buffer_layout = (*vertex_buffer_layout_map).get(vertex_buffer_layouts.0).unwrap();
					let vertex_buffer_layout: Vec<wgpu::VertexBufferLayout> = vertex_buffer_layout.iter().map(|r| {
						wgpu::VertexBufferLayout {
							array_stride: r.array_stride,
							step_mode: r.step_mode,
							attributes: &r.attributes,
						}
					}).collect();
					let pipeline_state = state_map.get(pipeline_state.0).unwrap();
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
			};

			// 设置pipeline
			draw_state.pipeline = Some(pipeline);
		}
	}
}

