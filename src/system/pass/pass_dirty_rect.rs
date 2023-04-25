use bevy::{ecs::{
    prelude::{Entity, Ref},
    query::{Changed, With},
    system::{ParamSet, Query},
}, prelude::{DetectChanges, EventReader}};

use pi_bevy_ecs_extend::{prelude::Layer};
use pi_style::style::Aabb2;

use crate::{
    components::{
        calc::{ContentBox, InPassId, NodeId, Quad, RootDirtyRect, TransformWillChangeMatrix},
        draw_obj::DrawState,
        pass_2d::{ChildrenPass, DirtyRect, DirtyRectState},
        user::{ShowChange, Viewport},
    },
    utils::tools::{box_aabb, calc_aabb}, system::node::world_matrix::OldQuad,
};

pub struct CalcDirtyRect;

/// 1. DrawState修改，更新脏区域
/// 2. DrawState删除，更新脏区域
/// 3. ShowChange修改，更新脏区域
/// 4. 收到Oct Modify事件， 更新脏区域（Oct更新前，应该发出事件，否则无法知道修改前的Oct）
/// 5. Pass2d子节点发生改变，修改脏区域
/// 6. 如果设置了全局脏，则直接设置所有pass2d脏，不需要遍历检查（TODO）
/// 根据每个Pass的脏区域，计算全局脏区域
pub fn calc_global_dirty_rect(
    query_draw_obj: Query<&NodeId, Changed<DrawState>>,
	mut quad_olds: EventReader<OldQuad>,
    query_node1: Query<(&InPassId, &Quad)>,
	query_node2: Query<&InPassId>,
    query_node_content_box: Query<&ContentBox>,

    // ShowChange改变，脏区域发生变化
    query_show_change: Query<(&Quad, &InPassId), Changed<ShowChange>>,

    mut query_pass: ParamSet<(
        Query<(&'static mut DirtyRect, &'static Layer, &'static TransformWillChangeMatrix), Changed<DirtyRect>>,
        Query<(&'static mut DirtyRect, &'static TransformWillChangeMatrix, &'static NodeId)>,
        Query<&mut DirtyRect>,
        Query<(&'static NodeId, &'static mut DirtyRect), Changed<ChildrenPass>>,
    )>,
    mut query_root: Query<(&mut RootDirtyRect, &'static Viewport, Ref<Viewport>), With<Viewport>>,
) {
	// 如果有节点修改了ShowChange，需要设置脏区域
    let mut p2 = query_pass.p2();
    for (quad, in_pass_id) in query_show_change.iter() {
        mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
    }

    // DrawState修改，脏区域发生变化
    let mut p2 = query_pass.p2();
    for node_id in query_draw_obj.iter() {
        let (in_pass_id, quad) = match query_node1.get(***node_id) {
            Ok(r) => r,
            _ => continue,
        };
        mark_pass_dirty_rect(***in_pass_id, quad, &mut p2);
    }
	// 处理包围盒改变前的区域，与脏区域求并
	for OldQuad{ quad, entity, ..} in quad_olds.iter() {
		let in_pass_id = match query_node2.get(*entity) {
            Ok(r) => r,
            _ => continue,
        };
        mark_pass_dirty_rect(***in_pass_id, quad, &mut p2);
    }

    // ChildrenPass修改，Pass2d需要设置脏区域，暂时将其直接设置为内容box（实际上应该设置更精确一点，TODO）
    let mut p3 = query_pass.p3();
    for (node_id, mut dirty_rect) in p3.iter_mut() {
        let quad = match query_node_content_box.get(***node_id) {
            Ok(r) => r,
            _ => continue,
        };
        mark_pass_dirty_rect1(&quad.oct, &mut dirty_rect);
    }

    // 迭代根节点，先将根节点的脏区域恢复到初始状态
    for (mut dirty_rect, viewport, viewport_tracker) in query_root.iter_mut() {
        // 视口改变，全局脏区域就为视口
        if viewport_tracker.is_changed() {
            dirty_rect.state = DirtyRectState::Inited;
            dirty_rect.value = viewport.0.clone();
        } else {
            // 先恢复到初始状态
            dirty_rect.state = DirtyRectState::UnInit;
        }
    }

    // 遍历所有pass的脏区域，求并，得全局脏区域
    for (mut pass_dirty_rect, layer, will_change_matrix) in query_pass.p0().iter_mut() {
        let (mut dirty_rect, _viewport, viewport_tracker) = match query_root.get_mut(layer.root()) {
            Ok(r) => r,
            _ => continue,
        };

        // 视口改变，全局脏区域就为视口
        if viewport_tracker.is_changed() {
            pass_dirty_rect.state = DirtyRectState::UnInit;
            continue;
        }

		

        if pass_dirty_rect.state == DirtyRectState::Inited {
            let aabb = match &will_change_matrix.0 {
                Some(matrix) => calc_aabb(&pass_dirty_rect.value, &matrix.will_change),
                None => pass_dirty_rect.value.clone(),
            };

			if dirty_rect.state == DirtyRectState::UnInit {
				dirty_rect.value = aabb;
				dirty_rect.state = DirtyRectState::Inited;
			} else {
				box_aabb(&mut dirty_rect.value, &aabb);
			}
            pass_dirty_rect.state = DirtyRectState::UnInit;
        }
    }
}

/// 本函数执行先决条件： Quad、ContentBox已经准备好
/// 当DrawObject的DrawState组件修改时，需要更新脏区域
/// 当DrawObject删除时，需要更新脏区域
/// 当DrawObject的NodeId创建时，需要更新脏区域
// #[listen(component=(DrawObject, DrawState, Modify), entity=(DrawObject, Delete), component=(DrawObject, NodeId, Create))]
// pub fn cal_dirty_rect_by(
// 	e: Event,
// 	query: Query<DrawObject, Join<NodeId, Node, (&InPassId, &Quad)>>,
// 	mut query_pass: Query<Pass2D, Write<DirtyRect>>,
// ) {
// 	let (pass_id, quad) = match query.get_by_entity(e.id) {
// 		Some(r) => r,
// 		None => return
// 	};

// 	mark_pass_dirty_rect(pass_id.0, &*quad, &mut query_pass);
// }

/// 当pass2d删除时，更新父的Pass2d的脏区域
// #[listen(entity=(Pass2D, Delete))]
// pub fn cal_dirty_rect_by_pass_2d(
// 	e: Event,
// 	query: Query<Pass2D,(&ParentPassId, Join<NodeId, Node, &ContentBox>)>,
// 	mut query_pass: Query<Pass2D, Write<DirtyRect>>,
// ) {
// 	let (pass_id, content_box) = match query.get_by_entity(e.id) {
// 		Some(r) => r,
// 		None => return
// 	};

// 	// 如果存在父的pass2d， 则将子pass2d的内容包围盒更新到父的pass2d的脏区域中
// 	if !pass_id.0.is_null() {
// 		mark_pass_dirty_rect(pass_id.0, &content_box.0, &mut query_pass);
// 	}
// }

/// 监听quad的删除事件，更新脏区域（Quad的创建、修改，最终表现为DrawState的变化，已被其他监听器监听）
/// 注意，Quad实际上没有真正意义的删除事件，只有当节点销毁时才会删除，但节点销毁不会额外发出组件删除的事件
/// 这里的Quad删除事件，实际上是Quad在修改前，额外发出的事件，一遍某些system能了解Quad修改前的数据（此事件的处，见Quad System）
// #[listen(component=(Node, Quad, Delete))]
// pub fn cal_dirty_rect_by_quad(
// 	e: Event,
// 	// query: Query<Node, (&DrawList, &Quad, &InPassId)>,
// 	query: Query<Node, (&Quad, &InPassId)>,
// 	mut query_pass: Query<Pass2D, Write<DirtyRect>>,
// ) {
// 	let (quad, pass_id) = match query.get_by_entity(e.id) {
// 		Some(r) => r,
// 		None => return
// 	};

// 	// if draw_list.len() > 0 { // 本应该判断DrawList是否存在，但如果对DrawList进行查询，会脏成某些system依赖循环，因此暂时不判断，不影响渲染结果
// 		mark_pass_dirty_rect(pass_id.0, &*quad, &mut query_pass);
// 	// }
// }

#[inline]
fn mark_pass_dirty_rect(pass_id: Entity, rect: &Aabb2, query_pass: &mut Query<&mut DirtyRect>) {
    let mut dirty_rect = match query_pass.get_mut(pass_id) {
        Ok(r) => r,
        _ => {log::warn!("mark_pass_dirty_rect fail!!!"); return},
    };

    mark_pass_dirty_rect1(rect, &mut dirty_rect);
}


fn mark_pass_dirty_rect1(rect: &Aabb2, dirty_rect: &mut DirtyRect) {
    let new_dirty_rect = match dirty_rect.state {
        // 脏区域处于未初始化状态，则设置脏区域为当前DrawObject对应节点的包围盒
        DirtyRectState::UnInit => DirtyRect {
            value: rect.clone(),
            state: DirtyRectState::Inited,
        },
        // 如果脏区域已经初始化，这设置脏区域为当前DrawObject对应节点的包围盒与当前脏区域的合并包围盒
        DirtyRectState::Inited => {
            box_aabb(&mut dirty_rect.value, &rect);
            DirtyRect {
                value: dirty_rect.value.clone(),
                state: DirtyRectState::Inited,
            }
        }
    };
    *dirty_rect = new_dirty_rect;
    // log::warn!("new_dirty_rect============================={:?}", dirty_rect);
    // println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
}
