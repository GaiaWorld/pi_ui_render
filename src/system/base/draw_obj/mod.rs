use pi_world::{query::Query, world::Entity};

use crate::{components::draw_obj::{BoxType, RenderCount}, resource::{GlobalDirtyMark, OtherDirtyType}};

pub mod image_texture_load;
pub mod life_drawobj;
pub mod set_geo_uniform;
pub mod sdf_gen;

pub fn set_box_type_count(draw_id: Entity, box_type: BoxType, render_count: RenderCount,  query_draw: &mut Query<(&mut BoxType, &mut RenderCount)>, global_mark: &mut GlobalDirtyMark) {
	if let Ok((mut box_type1, mut render_count1)) = query_draw.get_mut(draw_id) {
		if box_type != *box_type1 {
			*box_type1 = box_type;
		}
		if render_count.opacity as u32  != render_count1.opacity ||
			render_count.transparent as u32  != render_count1.transparent
		{
			log::debug!("instance_count======={:?}", render_count);
			*render_count1 = render_count;
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
		}
	}
}

pub fn set_box_type(draw_id: Entity, box_type: BoxType, query_draw: &mut Query<(&mut BoxType, &mut RenderCount)>) {
	if let Ok((mut box_type1, mut _render_count1)) = query_draw.get_mut(draw_id) {
		if box_type != *box_type1 {
			*box_type1 = box_type;
		}
	}
}