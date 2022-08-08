//! 颜色渲染 shader

use std::borrow::Cow;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader, ShaderProcessor}, bind_group_layout::BindGroupLayout, bind_group::BindGroup, asset::RenderRes, dyn_uniform_buffer::Group};
use pi_share::Share;
use pi_slotmap::{DefaultKey, Key};

use crate::{
	resource::draw_obj::{StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap, Program, VertexBufferLayout, VertexBufferLayouts, StaticIndex, ShaderStatic, DynBindGroupLayout, ColorStaticIndex, GradientColorStaticIndex, CommonPipelineState}, 
	components::draw_obj::{VSDefines, FSDefines}, utils::tools::calc_hash, shaders::color::{CameraMatrixGroup, ColorMaterialGroup}
};

use super::{GlslShaderStatic, create_shader_common_static, create_common_pipeline_state};

const COLOR_SHADER_VS: &'static str = "color_shader_vs";
const COLOR_SHADER_FS: &'static str = "color_shader_fs";
const COLOR_PIPELINE: &'static str = "color_pipeline";

pub struct CalcColorShader;

#[setup]
impl CalcColorShader {
	#[init]
	pub fn init(
		mut shader_static_map: ResMut<Shaders>,
		mut state_map: ResMut<StateMap>,
		mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		color_layout: Res<DynBindGroupLayout<ColorMaterialGroup>>,
		camera_layout: Res<DynBindGroupLayout<CameraMatrixGroup>>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		mut static_index: WriteRes<ColorStaticIndex>,
		mut common_state: Res<CommonPipelineState>,
		mut gradient_static_index: WriteRes<GradientColorStaticIndex>,
		device: Res<RenderDevice>,
	) {
		let shader = GlslShaderStatic::init(
			COLOR_SHADER_VS,
			COLOR_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../../resource/color.vert")}, 
			||{include_str!("../../../resource/color.frag")});
	
		let vertex_buffer = create_vertex_buffer_layout();
		let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

		let vertex_buffer1 = create_vertex_buffer_layout_with_color();
		let vertex_buffer_index1 = vertex_buffer_map.insert(vertex_buffer1);

		let mut bind_group_layout = VecMap::new();
		bind_group_layout.insert(CameraMatrixGroup::id() as usize, (*camera_layout).clone());
		bind_group_layout.insert(ColorMaterialGroup::id() as usize, (*color_layout).clone());
		
		shader_static_map.0.push(ShaderStatic {
			vs_shader_soruce: shader.shader_vs,
			fs_shader_soruce: shader.shader_fs,
			bind_group_layout,
		});

		// 插入背景颜色shader的索引
		let shader_index = shader_static_map.0.len() - 1;
		static_index.write(ColorStaticIndex(StaticIndex {
			shader: shader_index,
			pipeline_state: common_state.common,
			vertex_buffer_index,
			name: COLOR_PIPELINE,
		}));

		gradient_static_index.write(GradientColorStaticIndex(StaticIndex {
			shader: shader_index,
			pipeline_state: common_state.common,
			vertex_buffer_index: vertex_buffer_index1,
			name: COLOR_PIPELINE,
		}));
	}
}

fn create_shader_info(
	vs_shader_id: &ShaderId,
	fs_shader_id: &ShaderId,
	vs_defines: &VSDefines, 
	fs_defines: &FSDefines, 
	bind_group_layout: VecMap<Share<BindGroupLayout>>,
	_empty_group_layout: &Share<BindGroupLayout>,
	device: &RenderDevice,
	shaders: &XHashMap<ShaderId, Shader>,
) -> Program {
	let processor = ShaderProcessor::default();
	let imports = XHashMap::default();

	let vs = processor
            .process(&vs_shader_id, vs_defines, shaders, &imports)
            .unwrap();
	let vs = vs.get_glsl_source().unwrap();

	// 优化 TODO
	let mut vs_defines1 = naga::FastHashMap::default();
	for f in vs_defines.iter() {
		vs_defines1.insert(f.clone(), f.clone());
	}

	// 优化 TODO
	let mut fs_defines1 = naga::FastHashMap::default();
	for  f in fs_defines.iter() {
		fs_defines1.insert(f.clone(), f.clone());
	}

	let vs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("post_process_vs_shader_module"),
		source: wgpu::ShaderSource::Glsl {
			shader: Cow::Borrowed(vs),
			stage: naga::ShaderStage::Vertex,
			defines: vs_defines1,
		},
	});

	let fs = processor
            .process(&fs_shader_id, fs_defines, shaders, &imports)
            .unwrap();
	let fs = fs.get_glsl_source().unwrap();
	let fs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("post_process_fs_shader_module"),
		source: wgpu::ShaderSource::Glsl {
			shader: Cow::Borrowed(fs),
			stage: naga::ShaderStage::Fragment,
			defines: fs_defines1,
		},
	});
	
	
	let mut v = Vec::new();
	for r in bind_group_layout.iter() {
		if let Some(r) = r {
			v.push(&***r);
		}
	}

	// 根据defines， 删除layout(TODO)
	// for d in fs_defines.iter() {
	// 	match d {
	// 		"OPACITY" => {}
	// 		_ => {}
	// 	}
	// }
	//list_share_as_ref(bind_group_layout.iter());
	let slice = v.as_slice();
	let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("cerate post process pipeline_layout"),
		bind_group_layouts: slice,
		push_constant_ranges: &[],
	});
	
	Program {
		pipeline_layout: Share::new(pipeline_layout),
		vs_shader: Share::new(vs),
		fs_shader: Share::new(fs),
	}
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
	vec![VertexBufferLayout {
		array_stride: 8 as wgpu::BufferAddress,
		step_mode: wgpu::VertexStepMode::Vertex,
		attributes: vec![
			wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x2,
				offset: 0,
				shader_location: 0,
			},
		],
	}]
}

pub fn create_vertex_buffer_layout_with_color() -> VertexBufferLayouts {
	vec![
		VertexBufferLayout {
			array_stride: 8 as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x2,
					offset: 0,
					shader_location: 0,
				},
			],
		},
		VertexBufferLayout {
			array_stride: 16 as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x4,
					offset: 0,
					shader_location: 1,
				},
			],
		},
	]
}

pub fn create_color_group_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("color layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(16), // rgba四个通道，每个通道为一个f32, 大小为 4 * 4（每个通道一个u8， todo）
				},
				count: None,
			},
		],
	})
}

pub fn create_empty_bind_group(
	device: &RenderDevice, 
	group_layout: &BindGroupLayout,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&"empty bind", 0);
	let r = device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: group_layout,
		entries: &[],
		label: Some("color group create"),
	});

	bind_group_assets.insert(key, RenderRes::new(r, 5)).unwrap()
}

pub const COLOR_GROUP: usize = 4;
pub const POSITION_LOCATION: usize = 0;


