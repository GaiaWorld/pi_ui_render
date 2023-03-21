use bevy::ecs::{
    prelude::{Entity, EventReader},
    query::{ChangeTrackers, Changed, With},
    system::{Local, Query, RemovedComponents},
};
use pi_bevy_ecs_extend::{system_param::layer_dirty::{ComponentEvent, DirtyMark}, prelude::OrDefault};
use pi_hash::XHashMap;

use crate::{components::{pass_2d::ChildrenPass, user::{Transform, Point2}}, utils::tools::LayerDirty};

use pi_bevy_ecs_extend::prelude::{Layer, Up};


use crate::components::{
    calc::{LayoutResult, TransformWillChangeMatrix, WorldMatrix},
    pass_2d::ParentPassId,
    user::TransformWillChange,
};

pub fn transform_will_change_post_process(
    query_matrix: Query<(&'static WorldMatrix, &'static LayoutResult)>,
    query_node: Query<(&Up, &Layer)>,
    query_node1: Query<(&'static TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult)>,
    mut query_will_change_matrix: Query<&'static mut TransformWillChangeMatrix>,
    query_children: Query<&'static ChildrenPass>,
    query_parent_pass: Query<&ParentPassId>,

    query: Query<
        (
            Entity,
            &Up,
            &Layer,
            // transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
            // 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
            ChangeTrackers<WorldMatrix>,
            ChangeTrackers<TransformWillChange>,
            ChangeTrackers<Layer>,
        ),
        With<TransformWillChange>,
    >,
    del: RemovedComponents<TransformWillChange>,

    mut event_reader: EventReader<ComponentEvent<Changed<ParentPassId>>>,
    mut layer_dirty: Local<LayerDirty<Entity>>,
    mut parent_matrix_invert: Local<(XHashMap<Entity, WorldMatrix>, XHashMap<Entity, ()>)>,
) {
    for del in del.iter() {
        //
        parent_matrix_invert.0.remove(&del);
    }

    // let mut min_layer: (usize/*layer*/, EntityKey, usize/*count*/) = (1000000, EntityKey::null(), 0);

    // 世界矩阵变化、layer变化、tansform_will_change变化，设置层脏
    for (id, up, layer, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
        if tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed() {
            layer_dirty.marked_with_layer(id, id, layer.layer());
        }

        // 插入需要更新逆矩阵的节点
        if tracker_matrix.is_changed() {
            parent_matrix_invert.1.insert(up.parent(), ());
        }
    }

    // ParentPassId修改的节点，也需要插入到层脏
    for i in event_reader.iter() {
        if let Ok((_, layer)) = query_node.get(i.id) {
            if layer.layer() == 0 {
                continue;
            }

            layer_dirty.marked_with_layer(i.id, i.id, layer.layer());
        }
    }

    // 为TransformWillChange的父节点插入逆矩阵（存储在本地变量中）
    let (parent_matrix_invert0, parent_matrix_invert1) = &mut *parent_matrix_invert;
    for (id, _) in parent_matrix_invert1.drain() {
        let invert = match query_matrix.get(id) {
			Ok(r) if let Some(w) = r.0.invert() => w,
			_ => WorldMatrix::default(),
		};
        parent_matrix_invert0.insert(id, invert);
    }
    parent_matrix_invert.1.clear();

    // 迭代层脏
    let LayerDirty { dirty, dirty_mark_list } = &mut *layer_dirty;
    for (id, _layer) in dirty.iter() {
        dirty_mark_list.remove(id);
        let parent_pass_id = query_parent_pass.get(*id).unwrap();

        let parent_will_change_matrix = match query_will_change_matrix.get(***parent_pass_id) {
            Ok(r) => r.clone(),
            _ => TransformWillChangeMatrix(None),
        };
        recursive_set_matrix(
            *id,
            parent_will_change_matrix,
            &query_node1,
            &query_matrix,
            &mut query_will_change_matrix,
            &query_children,
            &parent_matrix_invert.0,
            dirty_mark_list,
        );
    }

    layer_dirty.clear();
}

pub fn recursive_set_matrix(
    id: Entity,
    mut parent_will_change_matrix: TransformWillChangeMatrix,

    query_node: &Query<(&'static TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult)>,
    query_matrix: &Query<(&'static WorldMatrix, &LayoutResult)>,
    query: &mut Query<&'static mut TransformWillChangeMatrix>,
    query_children: &Query<&'static ChildrenPass>,
    inverts: &XHashMap<Entity, WorldMatrix>,
    dirty_mark: &DirtyMark,
) {
    // 已经脏了，等待脏迭代
    if dirty_mark.get(&id).is_some() {
        return;
    }

    match query_node.get(id) {
        Ok((will_change, transform,  up, layout)) => {
            let ((p_matrix, parent_layout), p_matrix_invert) = match (query_matrix.get(up.parent()), inverts.get(&up.parent())) {
                (Ok(r), Some(r1)) => (r, r1),
                _ => return,
            };

            let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top;
			let offset = (layout.rect.left + parent_layout.padding.left, layout.rect.top + parent_layout.padding.top);
            let mut matrix = WorldMatrix::form_transform_layout(&will_change.0, &transform.origin, width, height, &Point2::new(offset.0, offset.1));

            let mut m = p_matrix * &matrix * p_matrix_invert;

            if let Some(parent_will_change_matrix) = &parent_will_change_matrix.0 {
                m = &parent_will_change_matrix.will_change * &m;
                matrix = &parent_will_change_matrix.primitive * &matrix;
            }

            if let Ok(mut r) = query.get_mut(id) {
                let will_change_matrix = TransformWillChangeMatrix::new(m.invert().unwrap(), m, matrix);
                *r = will_change_matrix.clone();
                parent_will_change_matrix = will_change_matrix;
            };
        }
        _ => {
            // 不是TransformWillChange节点，则直接集成父节点的matrix
            if let Ok(mut r) = query.get_mut(id) {
                *r = parent_will_change_matrix.clone();
            }
        }
    }

    // 设置子节点
    if let Ok(children) = query_children.get(id) {
		log::warn!("id===={:?}, {:?}", id, children);
        for i in children.iter() {
            recursive_set_matrix(
                **i,
                parent_will_change_matrix.clone(),
                query_node,
                query_matrix,
                query,
                query_children,
                inverts,
                dirty_mark,
            );
        }
    }
}
