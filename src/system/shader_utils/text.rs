use pi_ecs::prelude::{res::WriteRes, Res, ResMut};
use pi_ecs_macros::setup;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, dyn_uniform_buffer::Group};

use crate::{
    resource::draw_obj::{
        CommonPipelineState, DynBindGroupLayout, ShaderCatch, ShaderMap, ShaderStatic, Shaders, StaticIndex, TextStaticIndex, VertexBufferLayout,
        VertexBufferLayoutMap, VertexBufferLayouts,
    },
    shaders::{
        color::CameraMatrixGroup,
        text::{SampTex2DGroup, TextMaterialGroup},
    },
};

use super::GlslShaderStatic;

const TEXT_SHADER_VS: &'static str = "text_shader_vs";
const TEXT_SHADER_FS: &'static str = "text_shader_fs";
const TEXT_PILEPINE: &'static str = "text_pipeline";

pub struct CalcTextShader;

#[setup]
impl CalcTextShader {
    #[init]
    pub fn init(
        mut shader_static_map: ResMut<Shaders>,
        mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
        device: Res<RenderDevice>,
        mut shader_catch: ResMut<ShaderCatch>,
        mut shader_map: ResMut<ShaderMap>,
        mut static_index: WriteRes<TextStaticIndex>,

        text_layout: Res<DynBindGroupLayout<TextMaterialGroup>>,
        camera_layout: Res<DynBindGroupLayout<CameraMatrixGroup>>,
        common_state: Res<CommonPipelineState>,
    ) {
        let shader = GlslShaderStatic::init(
            TEXT_SHADER_VS,
            TEXT_SHADER_FS,
            &mut shader_catch,
            &mut shader_map,
            || include_str!("../../../resource/text.vert"),
            || include_str!("../../../resource/text.frag"),
        );

        let vertex_buffer = create_vertex_buffer_layout();
        let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

        let mut bind_group_layout = VecMap::new();
        bind_group_layout.insert(CameraMatrixGroup::id() as usize, (*camera_layout).clone());
        bind_group_layout.insert(TextMaterialGroup::id() as usize, (*text_layout).clone());
        bind_group_layout.insert(SampTex2DGroup::id() as usize, SampTex2DGroup::create_layout(&device, false));

        shader_static_map.0.push(ShaderStatic {
            vs_shader_soruce: shader.shader_vs,
            fs_shader_soruce: shader.shader_fs,
            bind_group_layout,
        });

        // 插入背景颜色shader的索引
        let shader_index = shader_static_map.0.len() - 1;
        static_index.write(TextStaticIndex(StaticIndex {
            shader: shader_index,
            pipeline_state: common_state.common,
            vertex_buffer_index,
            name: TEXT_PILEPINE,
        }));
    }
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
    vec![
        // position
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        // uv
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 1,
            }],
        },
        // color
        VertexBufferLayout {
            array_stride: 16 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 2,
            }],
        },
    ]
}
