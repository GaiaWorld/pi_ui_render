use std::borrow::Cow;

use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{
	device::RenderDevice, 
	shader::{ShaderId, Shader, ShaderProcessor}, 
	bind_group_layout::BindGroupLayout
};
use pi_share::Share;

use crate::{
	resource::draw_obj::{
		StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap, Program, VertexBufferLayout, VertexBufferLayouts}, 
	components::draw_obj::{VSDefines, FSDefines}
};

use super::{GlslShaderStatic, create_shader_common_static, StaticIndex, create_common_pipeline_state};

const TEXT_SHADER_VS: &'static str = "text_shader_vs";
const TEXT_SHADER_FS: &'static str = "text_shader_fs";
const TEXT_PILEPINE: &'static str = "text_pipeline";

pub struct TextShader;

#[setup]
impl TextShader {
	#[init]
	pub fn init(
		shader_static_map: ResMut<Shaders>,
		state_map: ResMut<StateMap>,
		vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		device: Res<RenderDevice>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		mut static_index: WriteRes<TextStaticIndex>,
	) {
		let shader = GlslShaderStatic::init(
			TEXT_SHADER_VS,
			TEXT_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../source/shader/text.vert")}, 
			||{include_str!("../../source/shader/text.frag")});
		
		let r = init_static(
			shader_static_map,
			state_map,
			vertex_buffer_map,
			&share_layout,
			shader,
			&device,
		);

		// 插入背景颜色shader的索引
		static_index.write(TextStaticIndex(r));

	}
}

pub fn init_static(
	mut shader_static_map: ResMut<Shaders>,
	mut state_map: ResMut<StateMap>,
	mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
	share_layout: &ShareLayout,
	shader: GlslShaderStatic,
	device: &RenderDevice,
) -> StaticIndex {
	// worldmatrix、viewmatrix、projectmatrix、depth
	let mut shader_static = create_shader_common_static(
		&share_layout,
		shader,
		create_shader_info);
	
	// color、stroke_color
	shader_static.bind_group.insert(
		TEXT_COLOR_GROUP, 
		Share::new(create_text_color_group_layout(device))
	);

	// textureSize
	shader_static.bind_group.insert(
		TEXT_TEXTURE_SIZE_GROUP, 
		Share::new(create_text_size_group_layout(device))
	);

	// texture
	shader_static.bind_group.insert(
		TEXT_TEXTURE_GROUP, 
		Share::new(device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor {
				label: Some("opacity_layout"),
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
							sample_type: wgpu::TextureSampleType::Float { filterable: true },
							view_dimension: wgpu::TextureViewDimension::D2,
							multisampled: false,
						},
						count: None,
					},
				],
			}
		))
	);

	shader_static_map.0.push(shader_static);
	let shader_index = shader_static_map.0.len() - 1;

	let pipeline_state = create_common_pipeline_state();
	let pipeline_state = state_map.insert(pipeline_state);
	
	// vertex layout
	let vertex_buffer = create_vertex_buffer_layout();
	let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

	StaticIndex {
		shader: shader_index,
		pipeline_state,
		vertex_buffer_index,
		name: TEXT_PILEPINE,
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
		label: Some("text_vs_shader_module"),
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
		label: Some("text_fs_shader_module"),
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
	vec![
		// position
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
		// uv
		VertexBufferLayout {
			array_stride: 8 as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x2,
					offset: 0,
					shader_location: 1,
				},
			],
		},
		// color
		VertexBufferLayout {
			array_stride: 16 as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x4,
					offset: 0,
					shader_location: 2,
				},
			],
		},
	]
}

pub fn create_text_color_group_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("text_color_layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(32), // color + strokeColor
				},
				count: None,
			},
		],
	})
}

pub fn create_text_size_group_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("text_texture_size_layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(8), // textureSize vec2
				},
				count: None,
			},
		],
	})
}

#[derive(Deref)]
pub struct TextStaticIndex(pub StaticIndex);

pub const TEXT_TEXTURE_GROUP: usize = 4;
pub const TEXT_TEXTURE_SIZE_GROUP: usize = 6;
pub const TEXT_COLOR_GROUP: usize = 5;

pub const TEXT_POSITION_LOCATION: usize = 0;
pub const TEXT_UV_LOCATION: usize = 1;
pub const TEXT_COLOR_LOCATION: usize = 2;


