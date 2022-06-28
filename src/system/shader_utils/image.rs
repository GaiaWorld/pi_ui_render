use std::borrow::Cow;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader, ShaderProcessor}, bind_group_layout::BindGroupLayout, bind_group::BindGroup, asset::RenderRes};
use pi_share::Share;
use pi_slotmap::DefaultKey;
use wgpu::{DepthStencilState, CompareFunction, TextureFormat, StencilState, DepthBiasState, MultisampleState};

use crate::{
	resource::draw_obj::{StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap, Program, VertexBufferLayout, VertexBufferLayouts, PipelineState}, 
	components::draw_obj::{VSDefines, FSDefines}, utils::tools::calc_hash
};

use super::{GlslShaderStatic, create_shader_common_static, StaticIndex};

const IMAGE_SHADER_VS: &'static str = "image_shader_vs";
const IMAGE_SHADER_FS: &'static str = "image_shader_fs";
const IMAGE_PIPELINE: &'static str = "image_pipeline";

pub struct ImageShader;

#[setup]
impl ImageShader {
	#[init]
	pub fn init(
		shader_static_map: ResMut<Shaders>,
		state_map: ResMut<StateMap>,
		mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		device: Res<RenderDevice>,
		mut static_index: WriteRes<ImageStaticIndex>,
		mut pos_uv_vertex_layout: WriteRes<PosUvVertexLayout>,
	) {
		let shader = GlslShaderStatic::init(
			IMAGE_SHADER_VS,
			IMAGE_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../source/shader/image.vert")}, 
			||{include_str!("../../source/shader/image.frag")});
		
		let r = init_static(
			shader_static_map,
			state_map,
			&mut vertex_buffer_map,
			&share_layout,
			&device,
			shader,
		);

		// 插入背景颜色shader的索引
		static_index.write(ImageStaticIndex(r));

		let vertex_buffer_layout = create_vertex_buffer_layout_p_v();
		let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer_layout);
		pos_uv_vertex_layout.write(PosUvVertexLayout(vertex_buffer_index));
	}
}

pub fn init_static(
	mut shader_static_map: ResMut<Shaders>,
	mut state_map: ResMut<StateMap>,
	vertex_buffer_map: &mut ResMut<VertexBufferLayoutMap>,
	share_layout: &ShareLayout,
	device: &RenderDevice,
	shader: GlslShaderStatic,
) -> StaticIndex {
	let mut shader_static = create_shader_common_static(
		&share_layout,
		shader,
		create_shader_info);

	// texture
	shader_static.bind_group.insert(IMAGE_TEXTURE_GROUP, Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
		name: IMAGE_PIPELINE,
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

	println!("vs======{:?}", vs_defines1);
	let vs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("image_process_vs_shader_module"),
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
		label: Some("image_process_fs_shader_module"),
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
	println!("len===={}, {}", slice.len(), v.len());
	let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("cerate image pipeline_layout"),
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
	]
}

// position 和uv放在同一个buffer中（一些情况，position和uv严格相关，没必要将buffer分开）
pub fn create_vertex_buffer_layout_p_v() -> VertexBufferLayouts {
	vec![
		VertexBufferLayout {
			array_stride: 16 as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![
				// position
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x2,
					offset: 0,
					shader_location: 0,
				},
				// uv
				wgpu::VertexAttribute {
					format: wgpu::VertexFormat::Float32x2,
					offset: 8,
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
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
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

pub fn create_empty_bind_group(
	device: &RenderDevice, 
	group_layout: &BindGroupLayout,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&"empty bind");
	let r = device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: group_layout,
		entries: &[],
		label: Some("color group create"),
	});

	bind_group_assets.cache(key, RenderRes::new(r, 5));
	bind_group_assets.get(&key).unwrap()
}

#[derive(Deref)]
pub struct ImageStaticIndex(pub StaticIndex);

#[derive(Deref)]
pub struct PosUvVertexLayout(pub DefaultKey);

pub const IMAGE_TEXTURE_GROUP: usize = 4;
pub const IMAGE_POSITION_LOCATION: usize = 0;
pub const IMAGE_UV_LOCATION: usize = 1;


