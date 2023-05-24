use bevy::ecs::{
    prelude::{Entity, RemovedComponents},
    query::Changed,
    system::Query,
};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;

use crate::components::user::TransformWillChange;

use bevy::{
    ecs::{
        prelude::{EventReader, Ref},
        query::With,
        system::Local,
    },
    prelude::DetectChanges,
};
use pi_bevy_ecs_extend::{prelude::OrDefault, system_param::layer_dirty::DirtyMark};
use pi_hash::{XHashMap, XHashSet};

use crate::{
    components::{
        pass_2d::ChildrenPass,
        user::{Point2, Transform},
    },
    utils::tools::LayerDirty,
};

use pi_bevy_ecs_extend::prelude::{Layer, Up};


use crate::components::{
    calc::{LayoutResult, TransformWillChangeMatrix, WorldMatrix},
    pass_2d::ParentPassId,
};

// 处理transform_will_change属性，计算出TransformWillChangeMatrix
// TransformWillChange属性常用于，子节点数量较多，又频繁改变Transform的节点
// 将变换Transform设置到TransformWillChange上，所有的子节点不需要重新计算WorldMatrix
// 假定某个节点A上设置的TransformWillChange为T1， A的世界矩阵为Wa，
// A存在一个子节点B，由B的Transform变换所得的局部矩阵为Tb，因此B的世界矩阵为Wa * Tb, 记作Wb
// 又由于A上存在TransformWillChange T1，其也能影响B，
// B的最终变换应该为Wa * T1 * Tb = Wa * T1 * Wa逆 * Wa * Tb = Wa * T1 * Wa逆 * Wb;
// 将Wa * T1 * Wa称为TransformWillChangeMatrix TW。
// 渲染A下所有子节点时，将TW作为视图矩阵。
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
            &Layer,
            // transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
            // 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
            Ref<WorldMatrix>,
            Ref<TransformWillChange>,
            Ref<Layer>,
        ),
        With<TransformWillChange>,
    >,
    mut del: RemovedComponents<TransformWillChange>,

    mut event_reader: EventReader<ComponentEvent<Changed<ParentPassId>>>,
    mut layer_dirty: Local<LayerDirty<Entity>>,
    mut matrix_invert: Local<(XHashMap<Entity, WorldMatrix>, XHashSet<Entity>)>,
) {
	// log::warn!("transform_will_change_post_process=====================");
    for del in del.iter() {
        matrix_invert.0.remove(&del);
    }

    // 世界矩阵变化、layer变化、tansform_will_change变化，设置层脏
    for (id,layer, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
        if tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed() {
            layer_dirty.marked_with_layer(id, id, layer.layer());
        }

        // 插入需要更新逆矩阵的节点
        if tracker_matrix.is_changed() {
            matrix_invert.1.insert(id);
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
    let (matrix_invert0, matrix_invert1) = &mut *matrix_invert;
    for id in matrix_invert1.drain() {
        let invert = match query_matrix.get(id) {
			Ok(r) if let Some(w) = r.0.invert() => w,
			_ => WorldMatrix::default(),
		};

	// 	log::warn!("will_change p==============id: {:?}, {:?}, \n {:?}", id, invert, query_matrix.get(id));
	matrix_invert0.insert(id, invert);
    }
    matrix_invert1.clear();

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
            &matrix_invert.0,
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
        Ok((will_change, transform, up, layout)) => {
            let ((p_matrix, parent_layout), invert) = match (query_matrix.get(up.parent()), inverts.get(&id)) {
                (Ok(r), Some(invert)) => (r, invert),
                _ => return,
            };

			let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top;
            let offset = (layout.rect.left + parent_layout.padding.left, layout.rect.top + parent_layout.padding.top);
            let mut will_change_matrix = WorldMatrix::form_transform_layout(&will_change.0, &transform.origin, width, height, &Point2::new(offset.0, offset.1));

            let mut m = p_matrix * &will_change_matrix * invert;

            if let Some(parent_will_change_matrix) = &parent_will_change_matrix.0 {
                m = &parent_will_change_matrix.will_change * &m;
                will_change_matrix = &parent_will_change_matrix.primitive * &will_change_matrix;
            }

            if let Ok(mut r) = query.get_mut(id) {
                let will_change_matrix = TransformWillChangeMatrix::new(m.invert().unwrap(), m, will_change_matrix);
                *r = will_change_matrix.clone();

                parent_will_change_matrix = will_change_matrix;
            };
        }
        _ => {
            // 不是TransformWillChange节点，则直接继承父节点的matrix
            if let Ok(mut r) = query.get_mut(id) {
                *r = parent_will_change_matrix.clone();
            }
        }
    }

    // 设置子节点
    if let Ok(children) = query_children.get(id) {
        // log::warn!("id===={:?}, {:?}", id, children);
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
