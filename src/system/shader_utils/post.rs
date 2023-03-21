use bevy::ecs::{world::{FromWorld, World}, system::Resource};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::rhi::bind_group_layout::BindGroupLayout;

#[derive(Deref, Resource)]
pub struct PostBindGroupLayout(pub BindGroupLayout);

impl FromWorld for PostBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("post_process_texture_layout"),
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
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::default(),
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        Self(layout)
    }
}