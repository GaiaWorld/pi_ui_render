use bevy_ecs::{
	query::{Changed, With},
	system::{ParamSet, Query},
    prelude::{DetectChanges, EventReader, Mut, Event, Entity, Or, Ref},
};

use pi_bevy_ecs_extend::{prelude::Layer, system_param::res::OrInitRes};
use pi_style::style::Aabb2;

use crate::{
    components::{
        calc::{ContentBox, InPassId, NodeId, Quad, RootDirtyRect, TransformWillChangeMatrix},
        draw_obj::DrawState,
        pass_2d::{ChildrenPass, DirtyMark, DirtyRect, DirtyRectState, ParentPassId, PostProcess},
        user::{Canvas, TransformWillChange, Viewport},
    },
    system::{node::world_matrix::OldQuad, draw_obj::calc_text::IsRun},
    utils::tools::{box_aabb, calc_aabb},
};

pub struct OldTransformWillChange {
    pub matrix: TransformWillChangeMatrix,
    pub entity: Entity,
    pub inpass_id: Entity,
    pub root: Entity,
}

impl Event for OldTransformWillChange {}

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
    // query_transform_will_change: Query<&NodeId, Changed<TransformWillChangeMatrix>>,
    mut quad_olds: EventReader<OldQuad>,
    query_node1: Query<(&InPassId, &Quad)>,
    query_node2: Query<&InPassId>,
    mut query_dirty_mark: Query<(&mut DirtyMark, &ParentPassId)>,
    mut transform_willchange_olds: EventReader<OldTransformWillChange>,
    // 这里不检测TransformWillChangeMatrix的修改， 因为TransformWillChange修改后会递归修改TransformWillChangeMatrix
    // transform_willchange: Query<(&Quad, &ContentBox, &ParentPassId, &TransformWillChangeMatrix, &Overflow), Changed<TransformWillChange>>,

    // Canvas改变，脏区域发生变化
    query_show_change: Query<(&Quad, &InPassId), Changed<Canvas>>,

    mut query_pass: ParamSet<(
        Query<
            (
                &mut DirtyRect,
                &Layer,
                &TransformWillChangeMatrix,
                Ref<PostProcess>,
                Ref<ChildrenPass>,
                &ContentBox,
                Option<Ref<TransformWillChange>>,
                Entity,
                &ParentPassId,
            ),
            Or<(
                Changed<DirtyRect>,
                Changed<PostProcess>,
                Changed<ChildrenPass>,
                Changed<TransformWillChange>,
            )>,
        >,
        Query<(&mut DirtyRect, &ContentBox)>,
        Query<&mut DirtyRect>,
        Query<(Ref<Viewport>, Entity, &mut DirtyRect), With<Viewport>>,
    )>,
    mut query_root: Query<(&mut RootDirtyRect, Ref<Viewport>), With<Viewport>>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 如果有节点修改了ShowChange，需要设置脏区域
    let mut p2 = query_pass.p2();
    for (quad, in_pass_id) in query_show_change.iter() {
        mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
    }

    // DrawState修改，脏区域发生变化
    // let mut p2 = query_pass.p2();
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

    // // transform_willchange修改后， 或更新父的脏区域
    // for (quad, content_box, parent_pass_id, will_change_matrix, overflow) in transform_willchange.iter() {
    // 	let aabb = if overflow.0 {
    // 		&content_box.oct
    // 	} else {
    // 		&quad.0
    // 	};
    // 	let aabb = match &will_change_matrix.0 {
    // 		Some(matrix) => calc_aabb(&aabb, &matrix.will_change),
    // 		None => pass_dirty_rect.value.clone(),
    // 	};
    // 	log::warn!("mmmmm======================");
    //     mark_pass_dirty_rect(***parent_pass_id, quad, &mut p2);
    // }


    // 迭代根节点，先将根节点的脏区域恢复到初始状态
    for (mut dirty_rect, viewport) in query_root.iter_mut() {
        // 视口改变，全局脏区域就为视口
        if viewport.is_changed() {
            dirty_rect.state = DirtyRectState::Inited;
            dirty_rect.value = viewport.0.clone();
        } else {
            // 先恢复到初始状态
            dirty_rect.state = DirtyRectState::UnInit;
        }
    }

    // 新增了fbo缓冲的功能， 因此这里总设置根节点在时候范围内脏了（通常应该设置非跟节点缓冲，才能充分利用脏更）
    for (viewport, _root_node, mut pass_dirty_rect) in query_pass.p3().iter_mut() {
        pass_dirty_rect.value = viewport.0.clone();
        pass_dirty_rect.state = DirtyRectState::Inited;
    }

    // 遍历所有pass的脏区域，求并，得全局脏区域
    for (mut pass_dirty_rect, layer, will_change_matrix, post_ref, _children_ref, content_box, transform_willchange_ref, entity, parent_pass_id) in
        query_pass.p0().iter_mut()
    {
        // postlist修改，Pass2d需要设置脏区域，暂时将其直接设置为内容box（实际上应该设置更精确一点，TODO）
        // 这里注释掉， 是因为后处理应该修改父节点的脏区域
        // if post_ref.is_changed() {
        //     mark_pass_dirty_rect1(&content_box.oct, &mut pass_dirty_rect);
        // }

        let (mut dirty_rect, viewport_tracker) = match query_root.get_mut(layer.root()) {
            Ok(r) => r,
            _ => continue,
        };

        // 视口改变，全局脏区域就为视口
        if viewport_tracker.is_changed() {
            pass_dirty_rect.state = DirtyRectState::UnInit;
            continue;
        }

        let willchange_changed = match transform_willchange_ref {
            Some(transform_willchange_ref) => transform_willchange_ref.is_changed(),
            None => false,
        };

        if pass_dirty_rect.state == DirtyRectState::Inited || willchange_changed || post_ref.is_changed() {
            let mut start_dirty = if pass_dirty_rect.state == DirtyRectState::Inited {
                entity
            } else {
                // 只有transfrom_will_change， 从父开始设脏
                ***parent_pass_id
            };
            // 标记脏
            while let Ok((mut dirty_mark, parent_pass_id)) = query_dirty_mark.get_mut(start_dirty) {
				if dirty_mark.0 == true {
					break;
				}
				dirty_mark.0 = true;
				start_dirty = *** parent_pass_id;
                
            }
            // 本地脏区域合并到全局脏区域中
            merge_dirty_rect(
                &mut pass_dirty_rect,
                &mut dirty_rect,
                content_box,
                &will_change_matrix,
            );

            // 如果transform_willchange已经改变， 先不重置state， 后续一定能遍历到旧的transform_willchange， 到时候再重置
            if !willchange_changed {
                pass_dirty_rect.state = DirtyRectState::UnInit;
            }
        }
    }
    let mut p1 = query_pass.p1();
    // 旧的transformwillchange也需要考虑到脏区域中
    for old_willchange in transform_willchange_olds.iter() {
        let (mut dirty_rect, _viewport_tracker) = match query_root.get_mut(old_willchange.root) {
            Ok(r) => r,
            _ => continue,
        };

        if let Ok((mut pass_dirty_rect, content_box)) = p1.get_mut(old_willchange.inpass_id) {
            merge_dirty_rect(&mut pass_dirty_rect, &mut dirty_rect, content_box, &old_willchange.matrix);
            pass_dirty_rect.state = DirtyRectState::UnInit;
        }
    }
}

fn merge_dirty_rect(
    pass_dirty_rect: &mut Mut<DirtyRect>,
    dirty_rect: &mut Mut<RootDirtyRect>,
    content_box: &ContentBox,
    will_change_matrix: &TransformWillChangeMatrix,
) {
    let aabb = match &will_change_matrix.0 {
        Some(matrix) => {
            // 对content_box和脏区域求交，再乘以当前的will_change
            let mut aabb = content_box.oct.clone();
            if pass_dirty_rect.state == DirtyRectState::Inited {
                box_aabb(&mut aabb, &pass_dirty_rect.value)
            }
            calc_aabb(&pass_dirty_rect.value, &matrix.will_change)
        }
        None => pass_dirty_rect.value.clone(),
    };

    if dirty_rect.state == DirtyRectState::UnInit {
        dirty_rect.value = aabb;
        dirty_rect.state = DirtyRectState::Inited;
    } else {
        box_aabb(&mut dirty_rect.value, &aabb);
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
    // println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
}
