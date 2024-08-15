use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};
use pi_world::{event::ComponentChanged, filter::{Changed, Or}, query::Query};

use crate::{components::{calc::{DrawList, LayoutResult, WorldMatrix}, draw_obj::{BoxType, InstanceIndex, TextMark}}, resource::{draw_obj::InstanceContext, GlobalDirtyMark, OtherDirtyType}, system::draw_obj::{calc_text::IsRun, set_box}};


pub fn set_matrix_uniform(
    global_mark: OrInitSingleRes<GlobalDirtyMark>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    matrix_change: ComponentChanged<WorldMatrix>,
	// dirty_list: Event<StyleChange>,
	query: Query<(&DrawList, &WorldMatrix, &LayoutResult), Or<(Changed<WorldMatrix>, Changed<DrawList>)>>,
    query_draw: Query<(&InstanceIndex, &BoxType, Option<&TextMark>)>,
	r: OrInitSingleRes<IsRun>,
) {

    if r.0 {
		return;
	}

    if global_mark.mark.get(OtherDirtyType::DrawObjCreate as usize).map_or(false, |display| {*display == true}) {
        for data in query.iter() {
            set_matrix_uniform_inner(data, &mut instances, &query_draw); 
        }
    } else {
        for entity in matrix_change.iter() {
            if let Ok(data) = query.get(*entity) {
                set_matrix_uniform_inner(data, &mut instances, &query_draw); 
            }
        }
    }
}

pub fn set_matrix_uniform_inner(
    data: (&DrawList, &WorldMatrix, &LayoutResult),
	instances: &mut InstanceContext,
    query_draw: &Query<(&InstanceIndex, &BoxType, Option<&TextMark>)>,
) {
    let (draw_list, world_matrix, layout) = data;
    if draw_list.0.len() == 0 {
        return;
    }
    for draw_id in draw_list.0.iter() {
        if let Ok((instance_index, box_type, text)) = query_draw.get(draw_id.id) {
            if text.is_some() {
                continue; // 不处理文字
            }
            // 节点可能设置为dispaly none， 此时instance_index可能为Null TODO
            log::debug!("set_matrix_uniform!!!!: draw_id={:?}, instance_index={:?}", draw_id.id, instance_index);
            if pi_null::Null::is_null(&instance_index.0.start) {
                continue;
            }
            let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
            let aabb = match box_type {
                BoxType::Padding => layout.padding_aabb(),
                BoxType::Content => layout.content_aabb(),
                BoxType::Border => layout.border_aabb(),
            };
            set_box(&world_matrix, &aabb, &mut instance_data);
        }
    }

}