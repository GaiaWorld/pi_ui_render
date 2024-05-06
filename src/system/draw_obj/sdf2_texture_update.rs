//! 为sdf2的纹理创建bindgroup
use pi_world::prelude::{SingleRes, SingleResMut};
use pi_bevy_ecs_extend::prelude::OrInitSingleResMut;

use pi_bevy_render_plugin::PiRenderDevice;
use pi_share::Share;
use crate:: resource::{draw_obj::InstanceContext, ShareFontSheet};

/// 更新sdf2的纹理
pub fn update_sdf2_texture(
    mut instances: OrInitSingleResMut<InstanceContext>,
    font_sheet: SingleResMut<ShareFontSheet>,
    device: SingleRes<PiRenderDevice>,
    common_sampler: SingleRes<crate::resource::draw_obj::CommonSampler>,
) {
    let font_sheet = font_sheet.0.borrow();
    if let (Some(sdf2_index_texture_view), Some(sdf2_data_texture_view), Some(sdf2_shadow_texture_view)) = (
        &font_sheet.sdf2_index_texture_view,
        &font_sheet.sdf2_data_texture_view,
        &font_sheet.sdf2_shadow_texture_view,
    ) {
        if instances.sdf2_texture_group.is_none() {
            let group = (***device).create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &instances.sdf2_texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&sdf2_index_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&sdf2_data_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&sdf2_shadow_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.default),
                    },
                ],
                label: Some("sdf2 texture bind group create"),
            });

            instances.sdf2_texture_group = Some(Share::new(group));
        }
    }
}