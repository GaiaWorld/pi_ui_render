use bevy::{
    ecs::{
        prelude::{Entity, Or, Ref},
        query::{Changed, With},
        system::{ParamSet, Query},
    },
    prelude::{DetectChanges, EventReader},
};

use pi_bevy_ecs_extend::prelude::Layer;
use pi_style::style::Aabb2;

use crate::{
    components::{
        calc::{ContentBox, InPassId, NodeId, Quad, RootDirtyRect, TransformWillChangeMatrix},
        draw_obj::DrawState,
        pass_2d::{ChildrenPass, DirtyRect, DirtyRectState, PostProcess},
        user::{ShowChange, Viewport},
    },
    system::node::world_matrix::OldQuad,
    utils::tools::{box_aabb, calc_aabb},
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

    // ShowChange改变，脏区域发生变化
    query_show_change: Query<(&Quad, &InPassId), Changed<ShowChange>>,

    mut query_pass: ParamSet<(
        Query<
            (
                &'static mut DirtyRect,
                &'static Layer,
                &'static TransformWillChangeMatrix,
                Ref<PostProcess>,
                Ref<ChildrenPass>,
                &ContentBox,
            ),
            Or<(Changed<DirtyRect>, Changed<PostProcess>, Changed<ChildrenPass>)>,
        >,
        Query<(&'static mut DirtyRect, &'static TransformWillChangeMatrix, &'static NodeId)>,
        Query<&mut DirtyRect>,
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
    for OldQuad { quad, entity, .. } in quad_olds.iter() {
        let in_pass_id = match query_node2.get(*entity) {
            Ok(r) => r,
            _ => continue,
        };
        mark_pass_dirty_rect(***in_pass_id, quad, &mut p2);
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
    for (mut pass_dirty_rect, layer, will_change_matrix, post_ref, children_ref, content_box) in query_pass.p0().iter_mut() {
        // ChildrenPass、 postlist修改，Pass2d需要设置脏区域，暂时将其直接设置为内容box（实际上应该设置更精确一点，TODO）
        if post_ref.is_changed() || children_ref.is_changed() {
            mark_pass_dirty_rect1(&content_box.oct, &mut pass_dirty_rect);
        }

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

#[inline]
fn mark_pass_dirty_rect(pass_id: Entity, rect: &Aabb2, query_pass: &mut Query<&mut DirtyRect>) {
    let mut dirty_rect = match query_pass.get_mut(pass_id) {
        Ok(r) => r,
        _ => {
            log::warn!("mark_pass_dirty_rect fail!!!, {:?}", pass_id);
            return;
        }
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
