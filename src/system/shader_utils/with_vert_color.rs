use std::borrow::Cow;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader, ShaderProcessor}, bind_group_layout::BindGroupLayout, bind_group::BindGroup, asset::RenderRes};
use pi_share::Share;

use crate::{
	resource::draw_obj::{StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap, Program, VertexBufferLayout, VertexBufferLayouts}, 
	components::draw_obj::{VSDefines, FSDefines}, utils::tools::calc_hash
};

use super::{GlslShaderStatic, create_shader_common_static, StaticIndex, create_common_pipeline_state};

const WITH_VERT_COLOR_SHADER_VS: &'static str = "with_vert_color_shader_vs";
const WITH_VERT_COLOR_SHADER_FS: &'static str = "with_vert_color_shader_fs";
const WITH_VERT_COLOR_PILEPINE: &'static str = "with_vert_color_pipeline";

pub struct WithColorShader;

#[setup]
impl WithColorShader {
	#[init]
	pub fn init(
		shader_static_map: ResMut<Shaders>,
		state_map: ResMut<StateMap>,
		vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		mut static_index: WriteRes<WithVertColorStaticIndex>,
	) {
		let shader = GlslShaderStatic::init(
			WITH_VERT_COLOR_SHADER_VS,
			WITH_VERT_COLOR_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../source/shader/with_vert_color.vert")}, 
			||{include_str!("../../source/shader/with_vert_color.frag")});
		
		let r = init_static(
			shader_static_map,
			state_map,
			vertex_buffer_map,
			&share_layout,
			shader,
		);

		// 插入背景颜色shader的索引
		static_index.write(WithVertColorStaticIndex(r));

	}
}

pub fn init_static(
	mut shader_static_map: ResMut<Shaders>,
	mut state_map: ResMut<StateMap>,
	mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
	share_layout: &ShareLayout,
	shader: GlslShaderStatic,
) -> StaticIndex {
	let shader_static = create_shader_common_static(
		&share_layout,
		shader,
		create_shader_info);
	shader_static_map.0.push(shader_static);
	let shader_index = shader_static_map.0.len() - 1;

	let pipeline_state = create_common_pipeline_state();
	let pipeline_state = state_map.insert(pipeline_state);

	let vertex_buffer = create_vertex_buffer_layout();
	let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

	StaticIndex {
		shader: shader_index,
		pipeline_state,
		vertex_buffer_index,
		name: WITH_VERT_COLOR_PILEPINE,
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

#[derive(Deref)]
pub struct WithVertColorStaticIndex(pub StaticIndex);

// pub const POST_TEXTURE_GROUP: usize = 4;
// pub const OPACITY_GROUP: usize = 5;
// pub const POST_UV_LOCATION: usize = 1;


