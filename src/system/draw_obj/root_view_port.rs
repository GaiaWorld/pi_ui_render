use bevy_ecs::{
    prelude::Entity,
    query::Changed,
    system::{Commands, Query, Res, ResMut},
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::PiSafeAtlasAllocator;
use pi_render::{
    components::view::target_alloc::{SafeAtlasAllocator, TargetDescriptor, TextureDescriptor},
    rhi::texture::PiRenderDefault,
};
use smallvec::SmallVec;

use crate::{
    components::{draw_obj::DynTargetType, user::Viewport},
    resource::draw_obj::MaxViewSize,
};

use super::calc_text::IsRun;

/// 创建图节点所需要的数据
/// 如： DynTargetType (需要根据视口变化及时调整)
pub fn calc_dyn_target_type(
    mut query: Query<(&Viewport, Option<&mut DynTargetType>, Entity), Changed<Viewport>>,

    atlas_allocator: Res<PiSafeAtlasAllocator>,
    mut max_view_size: ResMut<MaxViewSize>,

    mut commands: Commands,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    for (view_port, dyn_target_type, entity) in query.iter_mut() {
        max_view_size.width = max_view_size.width.max((view_port.maxs.x - view_port.mins.x).ceil() as u32);
        max_view_size.height = max_view_size.height.max((view_port.maxs.y - view_port.mins.y).ceil() as u32);
        let ty = create_dyn_target_type(&atlas_allocator, max_view_size.width, max_view_size.height);
        match dyn_target_type {
            Some(mut r) => *r = ty,
            None => {
                commands.entity(entity).insert(ty);
            }
        };
    }
}

pub fn create_dyn_target_type(atlas_allocator: &SafeAtlasAllocator, width: u32, height: u32) -> DynTargetType {
    DynTargetType {
        has_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
            colors_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::pi_render_default(),
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                base_mip_level: 0,
                base_array_layer: 0,
                array_layer_count: None,
                view_dimension: None,
            }]),
            depth_descriptor: None,
            need_depth: true,
            default_width: width,
            default_height: height,
        }),
        no_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
            colors_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::pi_render_default(),
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                base_mip_level: 0,
                base_array_layer: 0,
                array_layer_count: None,
                view_dimension: None,
            }]),
            depth_descriptor: None,
            need_depth: false,
            default_width: width,
            default_height: height,
        }),
    }
}
