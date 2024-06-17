use pi_world::{filter::Or, prelude::{Changed, ComponentRemoved, Entity, Has, Local, OrDefault, ParamSet, Query, Ticker}};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Up, Layer, DirtyMark};

use crate::{
    components::user::TransformWillChange,
    system::draw_obj::calc_text::IsRun,
};

use pi_hash::XHashMap;

use crate::{
    components::{
        pass_2d::ChildrenPass,
        user::{Point2, Transform},
    },
    utils::tools::LayerDirty,
};


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
    query_node1: Query<(&TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult)>,
    mut query_will_change_matrix: ParamSet<(
        Query<&'static mut TransformWillChangeMatrix>,
        Query<(&'static mut TransformWillChangeMatrix, &'static Layer, Has<ParentPassId>, Entity, Has<TransformWillChange>)>,
    )> ,
    remove: ComponentRemoved<TransformWillChange>,
    query_children: Query<&'static ChildrenPass>,
    query_parent_pass: Query<&ParentPassId>,
    query_parent_pass_changed: Query<(&Layer, Entity), Changed<ParentPassId>>,

    query: Query<
        (
            Entity,
            // transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
            // 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
            Ticker<&WorldMatrix>,
            Ticker<&TransformWillChange>,
            Ticker<&Layer>,
        ),
        Or<(
            Changed<WorldMatrix>,
            Changed<Layer>,
            Changed<TransformWillChange>,
        )>,
    >,

    // mut event_reader: EventReader<ComponentEvent<Changed<ParentPassId>>>,
    mut layer_dirty: Local<LayerDirty<Entity>>,
    mut matrix_invert: Local<XHashMap<Entity, WorldMatrix>>, // 放置世界矩阵的逆矩阵
    // mut events_writer: EventWriter<OldTransformWillChange>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 处理移除TransformWillChange的节点
    for i in remove.iter() {
        if let Ok((mut m, layer, has_parent_pass_id, entity, has_willchange)) = query_will_change_matrix.p1().get_mut(*i) {
            if has_willchange {
                continue;
            }
            matrix_invert.remove(&entity);
            
            if has_parent_pass_id {
                // 如果该节点仍然是渲染上下文， 则标记层脏， 后续重新计算TransformWillChangeMatrix
                layer_dirty.marked_with_layer(entity, entity, layer.layer());
            } else {
                // 否则清理TransformWillChangeMatrix
                m.0 = None;
            }
        }
    }

    // 世界矩阵变化、layer变化、tansform_will_change变化，设置层脏
    for (id, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
		if query_parent_pass.get(id).is_ok() { // 如果是渲染上下文
			if tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed() {
				layer_dirty.marked_with_layer(id, id, tracker_layer.layer());
			}
	
			// 如果世界矩阵改变， 或不存在世界矩阵的逆矩阵， 则需要重新计算世界矩阵逆矩阵
			if tracker_matrix.is_changed() || matrix_invert.get(&id).is_none() {
                if let Some(invert) = tracker_matrix.invert() {
                    matrix_invert.insert(id, invert);
                }
			}
		} else if tracker_willchange.is_changed() { // 如果不是渲染上下文， 清理willchangematrix
			if let Ok(mut m) = query_will_change_matrix.p0().get_mut(id) {
				if m.0.is_some() {
					m.0 = None;
				}
			}
		}
        
    }

    // ParentPassId修改的节点，也需要插入到层脏
    for (layer, entity) in query_parent_pass_changed.iter() {
        if layer.layer() == 0 {
            continue;
        }

        layer_dirty.marked_with_layer(entity, entity, layer.layer());
    }

    // // 为TransformWillChange的父节点插入逆矩阵（存储在本地变量中）
    // let (matrix_invert0, matrix_invert1) = &mut *matrix_invert;
    // for id in matrix_invert1.drain() {
    //     let invert = match query_matrix.get(id) {
	// 		Ok(r) if let Some(w) = r.0.invert() => w,
	// 		_ => WorldMatrix::default(),
	// 	};

    //     matrix_invert0.insert(id, invert);
    // }
    // matrix_invert1.clear();

    // 迭代层脏
    let LayerDirty { dirty, dirty_mark_list } = &mut *layer_dirty;
    for (id, _layer) in dirty.iter() {
        dirty_mark_list.remove(id);
        let parent_pass_id = query_parent_pass.get(*id).unwrap();

        let parent_will_change_matrix = match query_will_change_matrix.p0().get(***parent_pass_id) {
            Ok(r) => r.clone(),
            _ => TransformWillChangeMatrix(None),
        };
        recursive_set_matrix(
            *id,
            parent_will_change_matrix,
            &query_node1,
            &query_matrix,
            &mut query_will_change_matrix.p0(),
            &query_children,
            &matrix_invert,
            dirty_mark_list,
        );
    }

    layer_dirty.clear();
}

pub fn recursive_set_matrix(
    id: Entity,
    mut parent_will_change_matrix: TransformWillChangeMatrix,

    query_node: &Query<(&TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult)>,
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
        Ok((will_change1, transform, up, layout)) if will_change1.0.is_some() => {
			let will_change = will_change1.0.as_ref().unwrap();
            let ((p_matrix, parent_layout), invert) = match (query_matrix.get(up.parent()), inverts.get(&id)) {
                (Ok(r), Some(invert)) => (r, invert),
                _ => return,
            };

            let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top;
            let offset = (layout.rect.left + parent_layout.padding.left, layout.rect.top + parent_layout.padding.top);
            // TransformWillChange跟Transform是替换的关系， 而不是补充的关系（一旦设置了TransformWillChange， Transform不再有效）
            let mut will_change_matrix =
                WorldMatrix::form_transform_layout(&will_change.all_transform, &transform.origin, width, height, &Point2::new(offset.0, offset.1));

            // 如果父上没有TransformWillChange， 此处m为TransformWillChange作用后， 节点真实的世界矩阵
            let mut m = p_matrix * &will_change_matrix * invert;

            if let Some(parent_will_change_matrix) = &parent_will_change_matrix.0 {
                // 如果父上下文上存在TransformWillChange， 真实的世界矩阵应该需要与父上下文作用
                m = &parent_will_change_matrix.will_change * &m;
                will_change_matrix = &parent_will_change_matrix.primitive * &will_change_matrix;
            }

			// log::warn!("will_change: {:?}, {:?}, \nparent_will_change_matrix: {:?}, will_change: {:?}, invert: {:?}", id, will_change_matrix, parent_will_change_matrix, will_change, invert);

            if let Ok(mut r) = query.get_mut(id) {
                // will_change修改， 发送就的Willchange矩阵
                // if will_change1.is_changed() {
                //     events_writer.send(OldTransformWillChange {
                //         matrix: (*r).clone(),
                //         entity: id,
                //         inpass_id: ***inpass_id,
                //         root: layer.root(),
                //     });
                // }
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
