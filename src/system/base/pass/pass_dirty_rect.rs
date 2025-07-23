use pi_world::{event::{ComponentAdded, ComponentChanged, Event}, fetch::{Mut, Ticker}, param_set::ParamSet, prelude::{Changed, Entity, Or, Query, With}, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};

use pi_style::style::Aabb2;

use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, ContentBox, InPassId, MaskTexture, Quad, RootDirtyRect, TransformWillChangeMatrix},
        pass_2d::{ChildrenPass, DirtyMark, DirtyRect, DirtyRectState, ParentPassId, PostProcess},
        user::{Canvas, TransformWillChange, Viewport},
    }, resource::{IsRun, RenderDirty}, system::base::node::{user_setting::StyleChange, world_matrix::OldQuad}, utils::tools::{box_aabb, calc_bound_box}
};
use crate::resource::draw_obj::InstanceContext;

pub struct OldTransformWillChange {
    pub matrix: TransformWillChangeMatrix,
    pub entity: Entity,
    // pub parent_id: Entity,
    // pub root: Entity,
}

pub struct CalcDirtyRect;

/// 1. DrawState修改，更新脏区域
/// 2. DrawState删除，更新脏区域
/// 3. ShowChange修改，更新脏区域
/// 4. 收到Oct Modify事件， 更新脏区域（Oct更新前，应该发出事件，否则无法知道修改前的Oct）
/// 5. Pass2d子节点发生改变，修改脏区域
/// 6. 如果设置了全局脏，则直接设置所有pass2d脏，不需要遍历检查（TODO）
/// 根据每个Pass的脏区域，计算全局脏区域
pub fn calc_global_dirty_rect(
    // query_draw_obj: Query<&NodeId, Changed<DrawState>>,
    // query_transform_will_change: Query<&NodeId, Changed<TransformWillChangeMatrix>>,
    quad_olds: Event<OldQuad>,
    dirty_list: Event<StyleChange>,

    query_node1: Query<(&InPassId, &Quad)>,
    // query_node2: Query<&InPassId>,
    mut query_dirty_mark: Query<(&mut DirtyMark, &ParentPassId)>,
    transform_willchange_olds: Event<OldTransformWillChange>,
    // 这里不检测TransformWillChangeMatrix的修改， 因为TransformWillChange修改后会递归修改TransformWillChangeMatrix
    // transform_willchange: Query<(&Quad, &ContentBox, &ParentPassId, &TransformWillChangeMatrix, &Overflow), Changed<TransformWillChange>>,

    // Canvas改变，脏区域发生变化
    query_show_change: Query<(&Quad, &InPassId)>,
    canvas_changed: ComponentChanged<Canvas>,
    canvas_added: ComponentAdded<Canvas>,

    background_changed: ComponentChanged<BackgroundImageTexture>,
    border_changed: ComponentChanged<BorderImageTexture>,
    mask_changed: ComponentChanged<MaskTexture>,

    mut query_pass: ParamSet<(
        Query<
            (
                &mut DirtyRect,
                &Layer,
                &TransformWillChangeMatrix,
                Ticker<&PostProcess>,
                Option<Ticker<&TransformWillChange>>,
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
        Query<(Ticker<&Viewport>, Entity, &mut DirtyRect), With<Viewport>>,
        Query<(&mut DirtyRect, &ParentPassId)>,
    )>,
    mut query_root: Query<(&mut RootDirtyRect, Ticker<&Viewport>), With<Viewport>>,
    mut render_dirty: OrInitSingleResMut<RenderDirty>,
    instance_context: SingleRes<InstanceContext>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

    // 如果全局脏， 则不需要计算脏区域
    if render_dirty.0 {
        canvas_changed.mark_read();
        canvas_added.mark_read();
        background_changed.mark_read();
        border_changed.mark_read();
        mask_changed.mark_read();
        dirty_list.mark_read();
        quad_olds.mark_read();
        transform_willchange_olds.mark_read();
        render_dirty.1 = true; // 标记本帧脏
        return;
    }

    // 如果有节点修改了ShowChange，需要设置脏区域
    let mut p2 = query_pass.p2();
    if canvas_changed.len() > 0 || canvas_added.len() > 0 {
        for entity in canvas_changed.iter().chain(canvas_added.iter()) {
            if let Ok((quad, in_pass_id)) = query_show_change.get(*entity) {
                log::trace!("canvas_changed========{:?}, {:?}", entity, quad);
                mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
            }
        }
       
    }

    if background_changed.len() > 0 {
        for entity in background_changed.iter() {
            if let Ok((quad, in_pass_id)) = query_show_change.get(*entity) {
                log::trace!("background_changed========{:?}, {:?}", entity, quad);
                mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
            }
        }
       
    }

    if border_changed.len() > 0 {
        for entity in canvas_changed.iter() {
            if let Ok((quad, in_pass_id)) = query_show_change.get(*entity) {
                log::trace!("border_changed========{:?}, {:?}", entity, quad);
                mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
            }
        }
    }

    if mask_changed.len() > 0 {
        for entity in mask_changed.iter() {
            if let Ok((quad, in_pass_id)) = query_show_change.get(*entity) {
                log::trace!("mask_changed========{:?}, {:?}", entity, quad);
                mark_pass_dirty_rect(***in_pass_id, &*quad, &mut p2);
            }
        }
       
    }
    

    // 用户修改，脏区域发生变化
    // let mut p2 = query_pass.p2();
    if dirty_list.len() > 0 {
        for node_id in dirty_list.iter() {
            let (in_pass_id, quad) = match query_node1.get(**node_id) {
                Ok(r) => r,
                _ => continue,
            };
            log::trace!("style========{:?}, {:?}", node_id, quad);
            mark_pass_dirty_rect(***in_pass_id, quad, &mut p2);
        }
    }
    // 处理包围盒改变前的区域，与脏区域求并
    if quad_olds.len() > 0 {
        for OldQuad { quad, entity, .. } in quad_olds.iter() {
            let (in_pass_id, cur_quad) = match query_node1.get(*entity) {
                Ok(r) => r,
                _ => continue,
            };
            log::trace!("quad==============={:?}", (entity, &cur_quad, &quad));
            mark_pass_dirty_rect(***in_pass_id, cur_quad, &mut p2);
            mark_pass_dirty_rect(***in_pass_id, quad, &mut p2);
        }
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

    // // 新增了fbo缓冲的功能， 因此这里总设置根节点在视口范围内脏了（通常应该设置非根节点缓冲，才能充分利用脏更）
    // for (viewport, _root_node, mut pass_dirty_rect) in query_pass.p3().iter_mut() {
    //     pass_dirty_rect.value = viewport.0.clone();
    //     pass_dirty_rect.state = DirtyRectState::Inited;
    // }

    let mut is_dirty = false;
    for (mut pass_dirty_rect, layer, will_change_matrix, post_ref, transform_willchange_ref, entity, parent_pass_id) in
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
            is_dirty = true;
            continue;
        }

        let willchange_changed = match transform_willchange_ref {
            Some(transform_willchange_ref) => transform_willchange_ref.is_changed(),
            None => false,
        };

        if pass_dirty_rect.state == DirtyRectState::Inited || willchange_changed || post_ref.is_changed() {
            is_dirty = true;
            let mut start_dirty = if pass_dirty_rect.state == DirtyRectState::Inited {
                entity
            } else {
                // 只有transfrom_will_change， 从父开始设脏
                parent_pass_id.0
            };
            // 标记脏
            while let Ok((mut dirty_mark, parent_pass_id)) = query_dirty_mark.get_mut(start_dirty) {
				if dirty_mark.0 == true {
					break;
				}
				dirty_mark.0 = true;
				start_dirty = parent_pass_id.0;
                
            }
            // 本地脏区域合并到全局脏区域中
            // let aabb = merge_dirty_rect(
            //     &mut pass_dirty_rect,
            //     &mut dirty_rect,
            //     &will_change_matrix,
            // );

            // 变换脏区域
            let aabb = match &will_change_matrix.0 {
                Some(matrix) => {
                    // 对content_box和脏区域求交，再乘以当前的will_change
                    // let mut aabb = content_box.oct.clone();
                    // if pass_dirty_rect.state == DirtyRectState::Inited {
                    //     box_aabb(&mut aabb, &pass_dirty_rect.value)
                    // }
                    calc_bound_box(&dirty_rect.value, &matrix.will_change)
                }
                None => dirty_rect.value.clone(),
            };
            pass_dirty_rect.value = aabb;
        }
    }
    let p4 = query_pass.p4();
    // 旧的transformwillchange也需要考虑到脏区域中
    if transform_willchange_olds.len() > 0 {
        for old_willchange in transform_willchange_olds.iter() {
            // let (mut dirty_rect, _viewport_tracker) = match query_root.get_mut(old_willchange.root) {
            //     Ok(r) => r,
            //     _ => continue,
            // };
            if let Ok((pass_dirty_rect, parent_id)) = p4.get_mut(old_willchange.entity) {
                let dirty_rect = pass_dirty_rect.clone();
                let p = parent_id.0;
                if let Ok((mut parent_dirty_rect, _parent_id)) = p4.get_mut(p) {
                    merge_dirty_rect(&dirty_rect, &mut parent_dirty_rect,  &old_willchange.matrix);
                }
            }
        }
    }

    let p4 = query_pass.p4();
    // 遍历所有pass的脏区域，将脏区域递归合并到父
    for entity in instance_context.pass_toop_list.iter() {
        let (dirty_rect, parent) = match p4.get_mut(*entity) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if dirty_rect.state == DirtyRectState::Inited {
            let aabb = dirty_rect.value.clone();
            let p = parent.0;
            let (mut dirty_rect, _parent) = match p4.get_mut(p) {
                Ok(r) => r,
                Err(_) => continue,
            };
            box_dirty_rect(aabb, &mut dirty_rect);
        }
    }
    render_dirty.1 = is_dirty;
}

fn merge_dirty_rect(
    dirty_rect: &DirtyRect,
    parent_dirty_rect: &mut Mut<DirtyRect>,
    // content_box: &ContentBox,
    will_change_matrix: &TransformWillChangeMatrix,
) {
    let aabb = match &will_change_matrix.0 {
        Some(matrix) => {
            // 对content_box和脏区域求交，再乘以当前的will_change
            // let mut aabb = content_box.oct.clone();
            // if pass_dirty_rect.state == DirtyRectState::Inited {
            //     box_aabb(&mut aabb, &pass_dirty_rect.value)
            // }
            calc_bound_box(&dirty_rect.value, &matrix.will_change)
        }
        None => dirty_rect.value.clone(),
    };
    box_dirty_rect(aabb, parent_dirty_rect);
}

fn box_dirty_rect(
    aabb: Aabb2,
    parent_dirty_rect: &mut Mut<DirtyRect>,
) {
    if parent_dirty_rect.state == DirtyRectState::UnInit {
        parent_dirty_rect.value = aabb;
        parent_dirty_rect.state = DirtyRectState::Inited;
        parent_dirty_rect.draw_changed = true;
    } else {
        box_aabb(&mut parent_dirty_rect.value, &aabb);
    }
}

#[inline]
fn mark_pass_dirty_rect(pass_id: Entity, rect: &Aabb2, query_pass: &mut Query<&mut DirtyRect>) {
    let mut dirty_rect = match query_pass.get_mut(pass_id) {
        Ok(r) => r,
        _ => {
            log::debug!("mark_pass_dirty_rect fail!!!, {:?}", pass_id);
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
            draw_changed: true,
        },
        // 如果脏区域已经初始化，这设置脏区域为当前DrawObject对应节点的包围盒与当前脏区域的合并包围盒
        DirtyRectState::Inited => {
            box_aabb(&mut dirty_rect.value, &rect);
            DirtyRect {
                value: dirty_rect.value.clone(),
                state: DirtyRectState::Inited,
                draw_changed: true,
            }
        }
    };
    *dirty_rect = new_dirty_rect;
    // println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
}
