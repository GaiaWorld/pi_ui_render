//! 一些关于shader的静态信息与rust的对应，通常Shader一旦确定，这些对应关系就确定了
//! 其中包括： GroupLayout、SharderModule、pipline_state、vertLayout等
//! 在wgpu中，一些信息是可变的，如pipline_state、GroupLayout中的一些描述。
//! 目前上不知道这些信息在何种情况下、需要怎样变化
//! 将他们的值认为是确定的，目前对编程没有影响
//! TODO: 后续，可能将不可变因素通过shader静态编译出来（尚不确定哪些通常不变），当前通过手动编写代码的方式来确定
//!

use std::collections::hash_map::Entry;

use naga::ShaderStage;
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_render::rhi::{
    asset::RenderRes,
    bind_group::BindGroup,
    bind_group_layout::BindGroupLayout,
    buffer::Buffer,
    device::RenderDevice,
    shader::{Shader, ShaderId},
};
use pi_share::Share;
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, MultisampleState, StencilState, TextureFormat};

use crate::{
    components::user::Matrix4,
    resource::draw_obj::{PipelineState, ShaderCatch, ShaderMap, ShareLayout},
    utils::tools::{calc_float_hash, calc_hash},
};

use super::pass::pass_render::DEPTH;

pub mod image;
// pub mod with_vert_color;
pub mod color;
pub mod text;
// pub mod image;
// pub mod color_shadow;

pub struct GlslShaderStatic {
    pub shader_vs: ShaderId,
    pub shader_fs: ShaderId,
}

impl GlslShaderStatic {
    fn init(
        vs_name: &'static str,
        fs_name: &'static str,
        shader_catch: &mut ShaderCatch,
        shader_map: &mut ShaderMap,
        load_vs: impl Fn() -> &'static str,
        load_fs: impl Fn() -> &'static str,
    ) -> Self {
        let (shader_vs, shader_fs) = {
            (
                match shader_map.entry(vs_name) {
                    Entry::Vacant(r) => {
                        let shader = Shader::from_glsl(load_vs(), ShaderStage::Vertex);
                        let r = r.insert(shader.id()).clone();
                        shader_catch.insert(shader.id(), shader);
                        r
                    }
                    Entry::Occupied(r) => r.get().clone(),
                },
                match shader_map.entry(fs_name) {
                    Entry::Vacant(r) => {
                        let shader = Shader::from_glsl(load_fs(), ShaderStage::Fragment);
                        let r = r.insert(shader.id()).clone();
                        shader_catch.insert(shader.id(), shader);
                        r
                    }
                    Entry::Occupied(r) => r.get().clone(),
                },
            )
        };
        Self { shader_vs, shader_fs }
    }
}

pub fn create_camera_bind_group(
    view: &Matrix4,
    layout: &BindGroupLayout,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
    let key = calc_float_hash(view.as_slice(), calc_hash(&"camera", 0));

    match bind_group_assets.get(&key) {
        Some(r) => r,
        None => {
            let buf = match buffer_assets.get(&key) {
                Some(r) => r,
                None => {
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("camera buffer init"),
                        contents: bytemuck::cast_slice(view.as_slice()),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });
                    buffer_assets.insert(key, RenderRes::new(buf, 5)).unwrap()
                }
            };
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buf.as_entire_binding(),
                }],
                label: Some("camera create"),
            });
            bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
        }
    }
}

pub fn create_depth_group(
    cur_depth: usize,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
    depth_cache: &mut Vec<Handle<RenderRes<BindGroup>>>,
    device: &RenderDevice,
    share_layout: &ShareLayout,
) -> Handle<RenderRes<BindGroup>> {
    match depth_cache.get(cur_depth) {
        Some(r) => r.clone(),
        None => {
            // let value = cur_depth as f32 / 600000.0;
            let key = calc_hash(&(DEPTH.clone(), cur_depth), calc_hash(&"depth uniform", 0)); // TODO
            let d = match bind_group_assets.get(&key) {
                Some(r) => r,
                None => {
                    let uniform_buf = match buffer_assets.get(&key) {
                        Some(r) => r,
                        None => {
                            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("depth buffer init"),
                                contents: bytemuck::cast_slice(&[cur_depth as f32]),
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            });
                            buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
                        }
                    };
                    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &share_layout.depth,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buf.as_entire_binding(),
                        }],
                        label: Some("depth group create"),
                    });
                    bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
                }
            };
            depth_cache.push(d.clone());
            d
        }
    }
}

pub fn create_common_pipeline_state() -> PipelineState {
    PipelineState {
        targets: vec![wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Bgra8Unorm,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
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
            depth_compare: CompareFunction::GreaterEqual,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        multiview: None,
    }
}
