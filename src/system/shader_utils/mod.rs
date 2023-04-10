//! 一些关于shader的静态信息与rust的对应，通常Shader一旦确定，这些对应关系就确定了
//! 其中包括： GroupLayout、SharderModule、pipline_state、vertLayout等
//! 在wgpu中，一些信息是可变的，如pipline_state、GroupLayout中的一些描述。
//! 目前上不知道这些信息在何种情况下、需要怎样变化
//! 将他们的值认为是确定的，目前对编程没有影响
//! TODO: 后续，可能将不可变因素通过shader静态编译出来（尚不确定哪些通常不变），当前通过手动编写代码的方式来确定
//!

use bevy::app::Plugin;
use bevy::ecs::{
    prelude::EventReader,
    system::{Commands, Res, ResMut},
};
use bevy::prelude::{Query, With, IntoSystemConfig};
use bevy::window::{WindowCreated, WindowResized, Window, PrimaryWindow};
use pi_assets::{
    asset::{GarbageEmpty, Handle},
    mgr::AssetMgr,
};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice};
use pi_render::{
    components::view::target_alloc::DEPTH_TEXTURE,
    rhi::{
        asset::RenderRes, bind_group::BindGroup, bind_group_layout::BindGroupLayout, buffer::Buffer, device::RenderDevice, texture::PiRenderDefault,
    },
};
use pi_share::Share;
use pi_style::style::Aabb2;
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, MultisampleState, StencilState, TextureFormat, TextureView};

use crate::{
    components::{
        pass_2d::ScreenTarget,
        user::{Matrix4, Point2},
    },
    resource::draw_obj::{CommonSampler, PipelineState, Program, ShareLayout, UnitQuadBuffer},
    utils::tools::{calc_float_hash, calc_hash},
};

use super::render_run;
use super::system_set::UiSystemSet;

// pub mod image;
// pub mod color;
// pub mod text;
// pub mod post;
// pub mod with_vert_color;

// pub mod image;
// pub mod color_shadow;

pub struct UiShaderPlugin;

impl Plugin for UiShaderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // let texture_res_mgr = app.world.get_resource::<ShareAssetMgr<RenderRes<TextureView>>>().unwrap().clone();
        // let device = app.world.get_resource::<PiRenderDevice>().unwrap().clone();
        // let window = app.world.get_resource::<PiRenderWindow>().unwrap().clone();

        // let width = window.width;
        // let height = window.height;

        // log::info!("xxxxxxxxxxxxxxxxxxxxxxxx========={:?}, {:?}", width, height);

        // window宽高变化时，需要重新创建，TODO
        // let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, width, height);
        // app.world.insert_resource(ScreenTarget {
        // 	aabb: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(width as f32, height as f32)),
        // 	depth: Some(depth_buffer), // 深度缓冲区
        // 								// depth: None,
        // });

        app
			// .init_resource::<Shaders>()
			// .init_resource::<ShaderCatch>()
			// .init_resource::<ShaderMap>()
			// .init_resource::<StateMap>()
			// .init_resource::<DynBindGroups>()
			// .init_resource::<VertexBufferLayoutMap>()
			// .init_resource::<post::PostBindGroupLayout>()
			// .init_resource::<DynBindGroupLayout<ColorMaterialGroup>>()
			// .init_resource::<DynBindGroupLayout<CameraMatrixGroup>>()
			// .init_resource::<DynBindGroupLayout<UiMaterialGroup>>()
			// .init_resource::<DynBindGroupLayout<TextMaterialGroup>>()
			// .init_resource::<CommonPipelineState>()
			// .init_resource::<DynUniformBuffer>()

			// .init_resource::<DynBindGroupIndex<ColorMaterialGroup>>()
			// .init_resource::<DynBindGroupIndex<CameraMatrixGroup>>()
			// .init_resource::<DynBindGroupIndex<UiMaterialGroup>>()
			// .init_resource::<DynBindGroupIndex<TextMaterialGroup>>()

			.init_resource::<CommonSampler>()
			.insert_resource(ShareAssetMgr(AssetMgr::<RenderRes<Program>>::new(
				GarbageEmpty(),
				false,
				60 * 1024 * 1024,
				3 * 60 * 1000,
			)))
			.init_resource::<ShareLayout>()
			.init_resource::<UnitQuadBuffer>()

			.add_system(screen_target_resize.run_if(render_run).before(UiSystemSet::Setting))
			// .add_startup_system(color::init)
			// .add_startup_system(image::init)
			// .add_startup_system(text::init)
		;
    }
}

pub fn screen_target_resize(
    mut command: Commands,
	// events1: Res<Events<WindowCreated>>,
    events: EventReader<WindowCreated>,
    resize_events: EventReader<WindowResized>,
    windows: Query<&Window, With<PrimaryWindow>>,
    screen_target: Option<ResMut<ScreenTarget>>,
    texture_res_mgr: Res<ShareAssetMgr<RenderRes<TextureView>>>,
    device: Res<PiRenderDevice>,
) {
    if events.len() > 0 || resize_events.len() > 0 {
        let window = match windows.get_single() {
            Ok(r) => r,
            _ => return,
        };
        if window.physical_width() == 0 || window.physical_height() == 0 {
            return;
        }

        match screen_target {
            Some(mut r) => {
                if r.aabb.maxs.x - r.aabb.mins.x != window.physical_width() as f32 || r.aabb.maxs.y - r.aabb.mins.y != window.physical_height() as f32
                {
                    let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, window.physical_width(), window.physical_height());
                    *r = ScreenTarget {
                        aabb: Aabb2::new(
                            Point2::new(0.0, 0.0),
                            Point2::new(window.physical_width() as f32, window.physical_height() as f32),
                        ),
                        depth: Some(depth_buffer), // 深度缓冲区
                                                   // depth: None,
                    }
                }
            }
            None => {
                let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, window.physical_width(), window.physical_height());
                let r = ScreenTarget {
                    aabb: Aabb2::new(
                        Point2::new(0.0, 0.0),
                        Point2::new(window.physical_width() as f32, window.physical_height() as f32),
                    ),
                    depth: Some(depth_buffer), // 深度缓冲区
                                               // depth: None,
                };
                command.insert_resource(r);
            }
        }
    }
}

// pub fn

// pub struct GlslShaderStatic {
//     pub shader_vs: ShaderId,
//     pub shader_fs: ShaderId,
// }

// impl GlslShaderStatic {
//     fn init(
//         vs_name: &'static str,
//         fs_name: &'static str,
//         shader_catch: &mut ShaderCatch,
//         shader_map: &mut ShaderMap,
//         load_vs: impl Fn() -> &'static str,
//         load_fs: impl Fn() -> &'static str,
//     ) -> Self {
//         let (shader_vs, shader_fs) = {
//             (
//                 match shader_map.entry(vs_name) {
//                     Entry::Vacant(r) => {
//                         let shader = Shader::from_glsl(load_vs(), ShaderStage::Vertex);
//                         let r = r.insert(shader.id()).clone();
//                         shader_catch.insert(shader.id(), shader);
//                         r
//                     }
//                     Entry::Occupied(r) => r.get().clone(),
//                 },
//                 match shader_map.entry(fs_name) {
//                     Entry::Vacant(r) => {
//                         let shader = Shader::from_glsl(load_fs(), ShaderStage::Fragment);
//                         let r = r.insert(shader.id()).clone();
//                         shader_catch.insert(shader.id(), shader);
//                         r
//                     }
//                     Entry::Occupied(r) => r.get().clone(),
//                 },
//             )
//         };
//         Self { shader_vs, shader_fs }
//     }
// }

// 创建深度缓冲区
fn create_depth_buffer(
    texture_res_mgr: &Share<AssetMgr<RenderRes<TextureView>>>,
    device: &RenderDevice,
    width: u32,
    height: u32,
) -> Handle<RenderRes<TextureView>> {
    let texture = (**device).create_texture(&wgpu::TextureDescriptor {
        label: Some("first depth buffer"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
		view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let hash = calc_hash(&(DEPTH_TEXTURE.get_hash(), width, height), calc_hash(&"depth texture", 0));
    texture_res_mgr
        .insert(hash, RenderRes::new(texture_view, (width * height * 3) as usize))
        .unwrap()
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
            let key = calc_hash(&cur_depth, calc_hash(&"depth uniform", 0)); // TODO
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
        targets: vec![Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::pi_render_default(),
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
