use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut, Up};
use pi_null::Null;
use pi_world::{event::ComponentChanged, fetch::OrDefault, filter::{Changed, Or}, query::Query, world::Entity};

use crate::{components::{calc::{DrawList, LayoutResult, NodeState, WorldMatrix}, draw_obj::{BoxType, InstanceIndex, RenderCount}}, resource::{draw_obj::InstanceContext, GlobalDirtyMark, OtherDirtyType}, shader1::batch_meterial::{LayoutUniform, WorldMatrixMeterial}};
use crate::resource::IsRun;

pub fn set_matrix_uniform(
    global_mark: OrInitSingleRes<GlobalDirtyMark>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    matrix_change: ComponentChanged<WorldMatrix>,
	// dirty_list: Event<StyleChange>,
	query: Query<(Entity, &DrawList, &WorldMatrix, &LayoutResult, &NodeState, &Up), Or<(Changed<WorldMatrix>, Changed<DrawList>)>>,
    query_parent: Query<(&NodeState, &WorldMatrix, &Up)>,
    query_draw: Query<(&InstanceIndex, &BoxType, OrDefault<RenderCount>)>,
	r: OrInitSingleRes<IsRun>,
) {

    if r.0 {
		return;
	}

    if global_mark.mark.get(OtherDirtyType::DrawObjCreate as usize).map_or(false, |display| {*display == true}) || 
        global_mark.mark.get(OtherDirtyType::InstanceCount as usize).map_or(false, |display| {*display == true})  {
        for data in query.iter() {
            set_matrix_uniform_inner(data, &mut instances, &query_draw, &query_parent); 
        }
        matrix_change.mark_read();
    } else {
        for entity in matrix_change.iter() {
            if let Ok(data) = query.get(*entity) {
                set_matrix_uniform_inner(data, &mut instances, &query_draw, &query_parent); 
            }
        }
    }
}

pub fn set_matrix_uniform_inner(
    data: (Entity, &DrawList, &WorldMatrix, &LayoutResult, &NodeState, &Up),
	instances: &mut InstanceContext,
    query_draw: &Query<(&InstanceIndex, &BoxType, OrDefault<RenderCount>)>,
    query_parent: &Query<(&NodeState, &WorldMatrix, &Up)>,
) {
    let (_entity, draw_list, mut world_matrix, layout, mut node_state,  mut up) = data;
    if draw_list.0.len() == 0 {
        return;
    }
    for draw_id in draw_list.0.iter() {
        if let Ok((instance_index, box_type, render_count)) = query_draw.get(draw_id.id) {
            // log::debug!("set_matrix_uniform_inner==========={:?}", (draw_id.id, render_count));
            while node_state.is_vnode() {
                if let Ok((node_state1, world_matrix1, up1)) = query_parent.get(up.parent()) {
                    node_state = node_state1;
                    world_matrix = world_matrix1;
                    up = up1;
                } else {
                    break;
                }
            }
            // 节点可能设置为dispaly none， 此时instance_index可能为Null TODO
            // log::debug!("set_matrix_uniform!!!!: draw_id={:?}, instance_index={:?}", draw_id.id, instance_index);
            // if pi_null::Null::is_null(&instance_index.0.start) {
            //     continue;
            // }
            
            let mut calc = |index: usize, count: usize| {
                if index.is_null() {
                    return;
                }
                let aabb = match box_type {
                    BoxType::Padding => layout.padding_aabb(),
                    BoxType::Content => layout.content_aabb(),
                    BoxType::Border => layout.border_aabb(),
                    BoxType::None => {
                        // if entity.index() == 257 && entity.data().version() == 4 {
                        //     println!("=============layout1=============={:?}", (entity));
                        // }
                        // log::debug!("matrix uniform none==========================={:?}", (draw_id, world_matrix.as_slice().len()));
                        instances.instance_data.set_data_mult(index, count, &WorldMatrixMeterial(world_matrix.as_slice()));
                        return;
                    },
                    BoxType::None2 => return,
                };
                // if entity.index() == 257 && entity.data().version() == 4 {
                // log::debug!("matrix uniform==========================={:?}", (draw_id, &aabb));
                // }
                
                instances.instance_data.set_data_mult(index, count,&LayoutUniform(&[aabb.mins.x, aabb.mins.y, aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y]));
                instances.instance_data.set_data_mult(index, count,&WorldMatrixMeterial(world_matrix.as_slice()));
            };
            calc(instance_index.opacity.start, render_count.opacity as usize);
            calc(instance_index.transparent.start, render_count.transparent as usize);
        }
    }
}