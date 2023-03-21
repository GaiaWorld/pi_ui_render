//! 颜色渲染 shader

use bevy::ecs::system::{ResMut, Res, Commands};
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_map::vecmap::VecMap;
use pi_render::rhi::{asset::RenderRes, bind_group::BindGroup, bind_group_layout::BindGroupLayout, device::RenderDevice, dyn_uniform_buffer::Group};
use pi_share::Share;

use crate::{
    resource::draw_obj::{DynBindGroupLayout, GradientColorStaticIndex, ShaderStatic, VertexBufferLayout, VertexBufferLayouts, ColorStaticIndex, Shaders, VertexBufferLayoutMap, ShaderCatch, ShaderMap, CommonPipelineState},
    shaders::color::{CameraMatrixGroup, ColorMaterialGroup},
    utils::tools::calc_hash, components::draw_obj::StaticIndex,
};

use super::GlslShaderStatic;

const COLOR_SHADER_VS: &'static str = "color_shader_vs";
const COLOR_SHADER_FS: &'static str = "color_shader_fs";
const COLOR_PIPELINE: &'static str = "color_pipeline";

pub fn init(
	mut shader_static_map: ResMut<Shaders>,
	mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
	color_layout: Res<DynBindGroupLayout<ColorMaterialGroup>>,
	camera_layout: Res<DynBindGroupLayout<CameraMatrixGroup>>,
	mut shader_catch: ResMut<ShaderCatch>,
	mut shader_map: ResMut<ShaderMap>,
	common_state: Res<CommonPipelineState>,
	mut command: Commands,
	// mut static_index: WriteRes<ColorStaticIndex>,
	// mut gradient_static_index: WriteRes<GradientColorStaticIndex>,
) {
	let shader = GlslShaderStatic::init(
		COLOR_SHADER_VS,
		COLOR_SHADER_FS,
		&mut shader_catch,
		&mut shader_map,
		|| include_str!("../../../resource/color.vert"),
		|| include_str!("../../../resource/color.frag"),
	);

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
	log::warn!("shader_static_map.0=================={:?}, {:p}", shader_static_map.0.len(), &shader_static_map.0);

	// 插入背景颜色shader的索引
	let shader_index = shader_static_map.0.len() - 1;
	command.insert_resource(ColorStaticIndex(StaticIndex {
		shader: shader_index,
		pipeline_state: common_state.common,
		vertex_buffer_index,
		name: COLOR_PIPELINE,
	}));

	command.insert_resource(GradientColorStaticIndex(StaticIndex {
		shader: shader_index,
		pipeline_state: common_state.common,
		vertex_buffer_index: vertex_buffer_index1,
		name: COLOR_PIPELINE,
	}));
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
    vec![VertexBufferLayout {
        array_stride: 8 as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: vec![wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }],
    }]
}

pub fn create_vertex_buffer_layout_with_color() -> VertexBufferLayouts {
    vec![
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        VertexBufferLayout {
            array_stride: 16 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 1,
            }],
        },
    ]
}

pub fn create_color_group_layout(device: &RenderDevice) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("color layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(16), // rgba四个通道，每个通道为一个f32, 大小为 4 * 4（每个通道一个u8， todo）
            },
            count: None,
        }],
    })
}

pub fn create_empty_bind_group(
    device: &RenderDevice,
    group_layout: &BindGroupLayout,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
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
