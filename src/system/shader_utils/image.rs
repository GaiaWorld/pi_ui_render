use std::borrow::Cow;

use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader, ShaderProcessor}, bind_group_layout::BindGroupLayout};
use pi_share::Share;
use wgpu::{DepthStencilState, CompareFunction, TextureFormat, StencilState, DepthBiasState, MultisampleState};

use crate::{
	resource::draw_obj::{StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap, Program, VertexBufferLayout, VertexBufferLayouts, PipelineState}, 
	components::draw_obj::{VSDefines, FSDefines}, utils::shader_helper::VIEW_GROUP
};

use super::{GlslShaderStatic, create_shader_common_static, StaticIndex};

const IMAGE_SHADER_VS: &'static str = "image_shader_vs";
const IMAGE_SHADER_FS: &'static str = "image_shader_fs";

pub struct CalcPostProcessShader;

#[setup]
impl CalcPostProcessShader {
	#[init]
	pub fn init(
		shader_static_map: ResMut<Shaders>,
		state_map: ResMut<StateMap>,
		vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		device: Res<RenderDevice>,
		mut static_index: WriteRes<ImageStaticIndex>,
	) {
		let shader = GlslShaderStatic::init(
			IMAGE_SHADER_VS,
			IMAGE_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../source/shader/common.vert")}, 
			||{include_str!("../../source/shader/image.frag")});
		
		let r = init_static(
			shader_static_map,
			state_map,
			vertex_buffer_map,
			share_layout,
			device,
			shader,
		);

		// 插入背景颜色shader的索引
		static_index.write(ImageStaticIndex(r));
	}
}

pub fn init_static(
	mut shader_static_map: ResMut<Shaders>,
	mut state_map: ResMut<StateMap>,
	mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
	share_layout: Res<ShareLayout>,
	device: Res<RenderDevice>,
	shader: GlslShaderStatic,
) -> StaticIndex {
	let mut shader_static = create_shader_common_static(
		&share_layout,
		shader,
		create_shader_info);
	// opacity group layout
	shader_static.bind_group.insert(OPACITY_GROUP, Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("opacity_layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(4), // f32 四字节
				},
				count: None,
			},
		],
	})));

	// texture
	shader_static.bind_group.insert(POST_TEXTURE_GROUP, Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
	})));

	shader_static_map.0.push(shader_static);
	let shader_index = shader_static_map.0.len() - 1;

	let pipeline_state = create_pipeline_state();
	let pipeline_state = state_map.insert(pipeline_state);

	let vertex_buffer = create_vertex_buffer_layout();
	let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

	StaticIndex {
		shader: shader_index,
		pipeline_state,
		vertex_buffer_index,
	}
}

fn create_shader_info(
	vs_shader_id: &ShaderId,
	fs_shader_id: &ShaderId,
	vs_defines: &VSDefines, 
	fs_defines: &FSDefines, 
	bind_group_layout: VecMap<Share<BindGroupLayout>>,
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

	println!("vs======{:?}", vs_defines1);
	let vs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("post_process_vs_shader_module"),
		source: wgpu::ShaderSource::Glsl {
			shader: Cow::Borrowed(vs),
			stage: naga::ShaderStage::Vertex,
			defines: vs_defines1,
		},
	});

	println!("fs======{:?}", fs_defines1);
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
	let mut i = 0;
	println!("definesxx================={:?}, {:?}", &fs_defines.0, &vs_defines.0);
	for r in bind_group_layout.iter() {
		if let Some(r) = r {
			if (i == OPACITY_GROUP && !fs_defines.contains("OPACITY")) || 
				(i == VIEW_GROUP && !vs_defines.contains("VIEW")) {
				
				println!("defines================={}", i, );
				i += 1;
				continue;
			}
			println!("definesdd================={}", i);
			v.push(&***r);
		}
		i += 1;
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
	println!("len===={}, {}", slice.len(), v.len());
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
	]
}

pub fn create_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![wgpu::ColorTargetState {
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			blend: Some(wgpu::BlendState {
				color: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::SrcAlpha,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent::REPLACE,
			}),
			write_mask: wgpu::ColorWrites::ALL,
		}],
		primitive: wgpu::PrimitiveState {
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			..Default::default()
		},
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth32Float,
			depth_write_enabled: true,
			depth_compare: CompareFunction::Always,
			stencil: StencilState::default(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState::default(),
		multiview: None,
	}
}

#[derive(Deref)]
pub struct ImageStaticIndex(pub StaticIndex);

pub const POST_TEXTURE_GROUP: usize = 4;
pub const OPACITY_GROUP: usize = 5;
pub const POST_UV_LOCATION: usize = 1;


