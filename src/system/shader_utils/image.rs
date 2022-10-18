use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_ecs::prelude::{res::WriteRes, Res, ResMut};
use pi_ecs_macros::setup;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{asset::RenderRes, bind_group::BindGroup, bind_group_layout::BindGroupLayout, device::RenderDevice, dyn_uniform_buffer::Group, texture::PiRenderDefault};
use pi_share::Share;
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, MultisampleState, StencilState, TextureFormat};

use crate::{
    resource::draw_obj::{
        CommonPipelineState, DynBindGroupLayout, ImageStaticIndex, PipelineState, PosUvVertexLayout, ShaderCatch, ShaderMap, ShaderStatic, Shaders,
        StaticIndex, VertexBufferLayout, VertexBufferLayoutMap, VertexBufferLayouts,
    },
    shaders::{
        color::CameraMatrixGroup,
        image::{ImageMaterialGroup, SampTex2DGroup},
    },
    utils::tools::calc_hash,
};

use super::GlslShaderStatic;

const IMAGE_SHADER_VS: &'static str = "image_shader_vs";
const IMAGE_SHADER_FS: &'static str = "image_shader_fs";
const IMAGE_PIPELINE: &'static str = "image_pipeline";

pub struct CalcImageShader;

#[setup]
impl CalcImageShader {
    #[init]
    pub fn init(
        mut shader_static_map: ResMut<Shaders>,
        mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
        post_layout: Res<DynBindGroupLayout<ImageMaterialGroup>>,
        camera_layout: Res<DynBindGroupLayout<CameraMatrixGroup>>,
        mut shader_catch: ResMut<ShaderCatch>,
        mut shader_map: ResMut<ShaderMap>,
        device: Res<RenderDevice>,
        mut static_index: WriteRes<ImageStaticIndex>,

        mut pos_uv_vertex_layout: WriteRes<PosUvVertexLayout>,
        common_state: Res<CommonPipelineState>,
    ) {
        let shader = GlslShaderStatic::init(
            IMAGE_SHADER_VS,
            IMAGE_SHADER_FS,
            &mut shader_catch,
            &mut shader_map,
            || include_str!("../../../resource/image.vert"),
            || include_str!("../../../resource/image.frag"),
        );

        let vertex_buffer = create_vertex_buffer_layout();
        let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

        let mut bind_group_layout = VecMap::new();
        bind_group_layout.insert(CameraMatrixGroup::id() as usize, (*camera_layout).clone());
        bind_group_layout.insert(ImageMaterialGroup::id() as usize, (*post_layout).clone());
        bind_group_layout.insert(SampTex2DGroup::id() as usize, SampTex2DGroup::create_layout(&device, false));

        shader_static_map.0.push(ShaderStatic {
            vs_shader_soruce: shader.shader_vs,
            fs_shader_soruce: shader.shader_fs,
            bind_group_layout,
        });

        // 插入背景颜色shader的索引
        let shader_index = shader_static_map.0.len() - 1;
        static_index.write(ImageStaticIndex(StaticIndex {
            shader: shader_index,
            pipeline_state: common_state.common,
            vertex_buffer_index,
            name: IMAGE_PIPELINE,
        }));

        let vertex_buffer_layout = create_vertex_buffer_layout_p_v();
        let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer_layout);
        pos_uv_vertex_layout.write(PosUvVertexLayout(vertex_buffer_index));
    }
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
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
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 1,
            }],
        },
    ]
}

// position 和uv放在同一个buffer中（一些情况，position和uv严格相关，没必要将buffer分开）
pub fn create_vertex_buffer_layout_p_v() -> VertexBufferLayouts {
    vec![VertexBufferLayout {
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
    }]
}

pub fn create_pipeline_state() -> PipelineState {
    PipelineState {
        targets: vec![Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::pi_render_default(),
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
        })],
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::GreaterEqual,
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

pub const POST_TEXTURE_GROUP: usize = 4;
pub const OPACITY_GROUP: usize = 5;
pub const POST_UV_LOCATION: usize = 1;
