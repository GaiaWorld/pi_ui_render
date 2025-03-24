use pi_flex_layout::style::Dimension;
use pi_style::style::{LengthUnit, TransformOrigin};
use pi_world::prelude::{Alter, Changed, SingleRes, SingleResMut};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_bevy_render_plugin::PiSafeAtlasAllocator;
use pi_render::{
    components::view::target_alloc::{SafeAtlasAllocator, TargetDescriptor, TextureDescriptor},
    rhi::texture::PiRenderDefault,
};
use smallvec::SmallVec;

use crate::{
    components::{draw_obj::DynTargetType, root::RootScale, user::{Size, Transform, Viewport}},
    resource::draw_obj::MaxViewSize,
};

use crate::resource::IsRun;


/// 创建图节点所需要的数据
/// 如： DynTargetType (需要根据视口变化及时调整)
pub fn calc_dyn_target_type(
    mut query: Alter<(&Viewport, &Size), Changed<Viewport>, (DynTargetType, Transform, RootScale)>,

    atlas_allocator: SingleRes<PiSafeAtlasAllocator>,
    mut max_view_size: SingleResMut<MaxViewSize>,

    // mut commands: Commands,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

    let mut iter = query.iter_mut();
    while let Some((view_port, size)) = iter.next() {
        let view_width = (view_port.maxs.x - view_port.mins.x).ceil();
        let view_height = (view_port.maxs.y - view_port.mins.y).ceil();
        max_view_size.width = max_view_size.width.max(view_width as u32);
        max_view_size.height = max_view_size.height.max(view_height as u32);
        let ty = create_dyn_target_type(&atlas_allocator, max_view_size.width, max_view_size.height, wgpu::TextureFormat::pi_render_default());
        // println!("calc_dyn_target_type==========={:?}, {:?}, {:?} ", view_port, dyn_target_type, ty);
        let width = if let Dimension::Points(r) = size.width {
            r
        } else {
            panic!("not support root width: {:?}", size.width);
        };
        let height = if let Dimension::Points(r) = size.height {
            r
        } else {
            panic!("not support root height: {:?}", size.height);
        };
        let mut transform = Transform::default();
        transform.add_func(pi_style::style::TransformFunc::Scale(view_width as f32/width, view_height as f32/height));
        transform.origin = TransformOrigin::XY(LengthUnit::Pixel(0.0), LengthUnit::Pixel(0.0));
        // log::warn!("calc_dyn_target_type==========={:?}, {:?}, {:?}", view_width as f32/width, view_height as f32/height, (width, height, view_width, view_height));
        let _ = iter.alter((ty, transform, RootScale{x: view_width as f32/width, y: view_height as f32/height}));
    }
}

pub fn create_dyn_target_type(atlas_allocator: &SafeAtlasAllocator, width: u32, height: u32, format: wgpu::TextureFormat) -> DynTargetType {
    DynTargetType {
        has_depth: atlas_allocator.get_or_create_type(TargetDescriptor {
            colors_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
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
                format,
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
