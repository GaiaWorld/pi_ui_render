// use pi_world::{event::Event, prelude::{Changed, Entity, Query}};
// use pi_bevy_ecs_extend::prelude::OrInitSingleRes;


// use crate::{
//     components::{
//         calc::{InPassId, TransformWillChangeMatrix},
//         pass_2d::{PassDirty, ParentPassId},
//         user::Canvas,
//     }, 
//     resource::IsRun, system::base::node::user_setting::StyleChange,
// };

// pub struct OldTransformWillChange {
//     pub matrix: TransformWillChangeMatrix,
//     pub entity: Entity,
//     pub inpass_id: Entity,
//     pub root: Entity,
// }

// pub struct CalcDirtyRect;


// pub fn calc_pass_dirty(
//     // Canvas改变，脏区域发生变化
//     query_canvas_change: Query<&InPassId, Changed<Canvas>>,
//     dirty_list: Event<StyleChange>,
//     quad_changed: Event<StyleChange>,

//     mut query: Query<(&mut PassDirty, &ParentPassId)>,
//     mut query1: Query<&InPassId>,
// 	r: OrInitSingleRes<IsRun>
// ) {
// 	if r.0 {
// 		return;
// 	}
//     // 如果有节点修改了ShowChange，需要设置脏区域
//     for in_pass_id in query_canvas_change.iter() {
//         mark_pass_dirty(***in_pass_id, &mut query);
//     }

//     // 用户修改，脏区域发生变化
//     // let mut p2 = query_pass.p2();
//     for node_id in dirty_list.iter() {
//         let in_pass_id = match query1.get(**node_id) {
//             Ok(r) => r,
//             _ => continue,
//         };
// 		// log::warn!("dirty========{:?}, {:?}", node_id, quad);
//         mark_pass_dirty(***in_pass_id, &mut query);
//     }
//     // 处理包围盒改变前的区域，与脏区域求并
//     for node_id in quad_changed.iter() {
//         let in_pass_id = match query1.get(**node_id) {
//             Ok(r) => r,
//             _ => continue,
//         };
//         mark_pass_dirty(***in_pass_id, &mut query);
//     }
// }


// #[inline]
// fn mark_pass_dirty(mut pass_id: Entity, query: &mut Query<(&mut PassDirty, &ParentPassId)>,) {
//     while let Ok((mut is_dirty, parent_pass)) = query.get_mut(pass_id) {
//         if is_dirty.draw_changed {
//             break;
//         }

//         is_dirty.bypass_change_detection().draw_changed = true;
//         pass_id = parent_pass.0.0;
//     }
// }

