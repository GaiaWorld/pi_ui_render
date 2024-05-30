use pi_world::prelude::{Changed, SingleRes, Alter, SingleResMut};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

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
    mut query: Alter<(&Viewport, Option<&mut DynTargetType>), Changed<Viewport>, (DynTargetType, )>,

    atlas_allocator: SingleRes<PiSafeAtlasAllocator>,
    mut max_view_size: SingleResMut<MaxViewSize>,

    // mut commands: Commands,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

    let mut iter = query.iter_mut();
    while let Some((view_port, dyn_target_type)) = iter.next() {
        max_view_size.width = max_view_size.width.max((view_port.maxs.x - view_port.mins.x).ceil() as u32);
        max_view_size.height = max_view_size.height.max((view_port.maxs.y - view_port.mins.y).ceil() as u32);
        let ty = create_dyn_target_type(&atlas_allocator, max_view_size.width, max_view_size.height);
        // println!("calc_dyn_target_type==========={:?}, {:?}, {:?} ", view_port, dyn_target_type, ty);
        match dyn_target_type {
            Some(mut r) => *r = ty,
            None => {
                let _ = iter.alter((ty, ));
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
