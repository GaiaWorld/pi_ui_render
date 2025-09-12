//! 处理TransformWillChange
//! 参考： https://developer.mozilla.org/zh-CN/docs/Web/CSS/will-change
//! TransformWillChange中存在值时， 等同于w3c标准中设置了**will-change: transform**
//! 应用场景： 滚动或其他高频修改Transform的场景，可优化该节点下所有节点的WorldMatrix的高频计算
//! 实现依据： 在渲染该节点下所有子节点的渲染实例前， 修改渲染时的视图矩阵(ViewMatrix),即可将这些节点做整体变换
//! 具体实施：以TransformWillChange设置scale:2，Transform中设置scale:3为例
//! 1. 设置了TransformWillChange的节点，其上设置的Transfrom自动失效， 最终效果是将节点缩放2倍
//! 2. WorldMatrix中，已经包含Transform的变换， 根据公式， 最终渲染位置P = ViewMatrix * WorldMatrix * PLayout，
//!     为了使得最终渲染位置正确，                                    P = ParentWorldMatrix * TransformWillChangeMatrix * PLayout， 
//!     有              ParentWorldMatrix * TransformWillChangeMatrix = ViewMatrix * WorldMatrix 
//!                     ParentWorldMatrix * TransformWillChangeMatrix = ParentWorldMatrix * TransformWillChangeMatrix * WorldMatrix逆 * WorldMatrix
//!                     ViewMatrix = ParentWorldMatrix * TransformWillChangeMatrix * WorldMatrix逆
//! 3. 当然， 可能存在多层TransformWillChange嵌套的情况， 如， A、B、C三个节点形成父子关系链， A->B->C, 其中A和C都设置了TransformWillChange，
//!     根据上述公式，                                            A.P = A.ViewMatrix * A.WorldMatrix
//!     要计算C.P, 只需要将  A.WorldMatrix 替换为 C的变换矩阵， 即  C.P = A.ViewMatrix * C.ViewMatrix * C.WorldMatrix
//!     因此， C最终的视图矩阵为                            ViewMatrix = A.ViewMatrix * C.ViewMatrix
use pi_null::Null;
use pi_style::style::StyleType;
use pi_world::{event::EventSender, filter::Or, prelude::{Changed, Entity, Has, Local, OrDefault, ParamSet, Query, Ticker}, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Up, Layer, DirtyMark};

use crate::{
    components::{calc::{style_bit, StyleBit, StyleMarkType}, pass_2d::WorldMatrixInvert, user::TransformWillChange}, resource::{GlobalDirtyMark, IsRun, OtherDirtyType}, system::{base::{node::world_matrix, pass::{content_box, pass_dirty_rect::OldTransformWillChange, pass_life, world_invert}}, system_set::UiSystemSet}
};

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
use pi_world::prelude::{App, Plugin, IntoSystemConfigs};
use crate::prelude::UiStage;

pub struct TransformWillChangePlugin;

impl Plugin for TransformWillChangePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(UiStage, 
                pass_life::pass_mark::<TransformWillChange>
                    .run_if(transform_will_change_change1)
                    .in_set(UiSystemSet::PassMark))
            .add_system(UiStage, transform_will_change_post_process
                .run_if(transform_will_change_change)
                .after(pass_life::calc_pass_children)
                .after(world_matrix::cal_matrix)
                .after(world_invert::calc_world_invert)
                .before(content_box::calc_content_box)
                .in_set(UiSystemSet::PassSetting))
        ;
    }
}

/// 计算 TransformWillChangeMatrix， 包含：
/// 1. own_view_matrix,忽略所有父的TransfromWillChange， 只考虑自身TransfromWillChange，计算而得的局部 view_matrix = ParentWorldMatrix * TransformWillChangeMatrix * WorldMatrix逆
/// 2. view_matrix: 当前节点需要的视图矩阵变换， view_matrix = Parent0.primitive * Parent1.primitive * ... * self.primitive
/// 3. view_matrix_invert: view_matrix 逆
/// 注意： TransformWillChange组件一旦添加， 不允许移除， 可设置为None
pub fn transform_will_change_post_process(
    query_matrix: Query<(&'static WorldMatrix, &'static LayoutResult)>,
    query_node1: Query<(&TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult, &'static WorldMatrixInvert)>,
    mut query_will_change_matrix: ParamSet<(
        Query<&'static mut TransformWillChangeMatrix>,
        Query<(&'static mut TransformWillChangeMatrix, &'static Layer, Has<ParentPassId>, Entity, Has<TransformWillChange>)>,
    )> ,
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
            Changed<TransformWillChange>
        )>,
    >,

    // mut event_reader: EventReader<ComponentEvent<Changed<ParentPassId>>>,
    mut layer_dirty: Local<LayerDirty<Entity>>,
    mut events_writer: EventSender<OldTransformWillChange>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 处理移除TransformWillChange的节点
    // for i in remove.iter() {
    //     if let Ok((mut m, layer, has_parent_pass_id, entity, has_willchange)) = query_will_change_matrix.p1().get_mut(*i) {
    //         if has_willchange {
    //             continue;
    //         }
    //         matrix_invert.remove(&entity);
            
    //         if has_parent_pass_id {
    //             // 如果该节点仍然是渲染上下文， 则标记层脏， 后续重新计算TransformWillChangeMatrix
    //             layer_dirty.marked_with_layer(entity, entity, layer.layer());
    //         } else {
    //             // 否则清理TransformWillChangeMatrix
    //             m.0 = None;
    //         }
    //     }
    // }

    // 世界矩阵变化、layer变化、tansform_will_change变化，设置层脏
    for (id, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
		if query_parent_pass.get(id).is_ok() { // 如果是渲染上下文
			if tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed() {
				layer_dirty.marked_with_layer(id, id, tracker_layer.layer());
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
        if layer.layer().is_null() {
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
    if dirty.count() > 0 {
        for (id, _layer) in dirty.iter() {
            dirty_mark_list.remove(id);
            let parent_pass_id = query_parent_pass.get(*id).unwrap();

            let parent_will_change_matrix = match query_will_change_matrix.p0().get(parent_pass_id.0) {
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
                dirty_mark_list,
                &mut events_writer,
            );
        }

        layer_dirty.clear();
    }
}

lazy_static! {
	pub static ref TRANSFORM_WILL_CHANGE_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::TransformWillChange as usize)
        .set_bit(StyleType::Transform as usize) // 设置Transform， 可能不会WorldMatrix修改， 但可能导致TransformWillChange修改
        .set_bit(StyleType::Translate as usize)
        .set_bit(StyleType::Scale as usize)
        .set_bit(StyleType::Rotate as usize)
		.set_bit(OtherDirtyType::WorldMatrix as usize)
		.set_bit(OtherDirtyType::PassLife as usize);
}

pub fn transform_will_change_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*TRANSFORM_WILL_CHANGE_DIRTY)
}

pub fn transform_will_change_change1(mark: SingleRes<GlobalDirtyMark>) -> bool {
    mark.mark.get(StyleType::TransformWillChange as usize).map_or(false, |display| {*display == true})
}

/// 递归设置Pass2D节点的TransformWillChangeMatrix
/// 1. 节点存在TransfromWillChange， 将父的TransformWillChangeMatrix与本节点的TransfromWillChange进行叠加， 得到新的TransformWillChangeMatrix， 设置到当前节点的TransformWillChangeMatrix组件中
/// 2. 节点不存在TransfromWillChange， 当前节点的TransformWillChangeMatrix直接继承父的Pass2D节点的TransformWillChangeMatrix
pub fn recursive_set_matrix(
    id: Entity,
    mut parent_will_change_matrix: TransformWillChangeMatrix,

    query_node: &Query<(&TransformWillChange, OrDefault<Transform>, &'static Up, &'static LayoutResult, &'static WorldMatrixInvert)>,
    query_matrix: &Query<(&'static WorldMatrix, &LayoutResult)>,
    query: &mut Query<&'static mut TransformWillChangeMatrix>,
    query_children: &Query<&'static ChildrenPass>,
    dirty_mark: &DirtyMark,
    events_writer: &mut EventSender<OldTransformWillChange>,
) {
    // 已经脏了，等待脏迭代
    if dirty_mark.get(&id).is_some() {
        return;
    }

    match query_node.get(id) {
        Ok((will_change1, transform, up, layout, world_matrix_invert)) if will_change1.0.is_some() => {
            let will_change = will_change1.0.as_ref().unwrap();
            let ((p_matrix, parent_layout), invert) = match (query_matrix.get(up.parent()), &world_matrix_invert.value) {
                (Ok(r), Some(world_matrix_invert)) => (r, world_matrix_invert),
                _ => return,
            };

            let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top;
            let offset = (layout.rect.left + parent_layout.padding.left, layout.rect.top + parent_layout.padding.top);
            // TransformWillChange跟Transform是替换的关系， 而不是补充的关系（一旦设置了TransformWillChange， Transform不再有效）
            let will_change_matrix =
                WorldMatrix::form_transform_layout(&will_change.all_transform, &transform.origin, width, height, &Point2::new(offset.0, offset.1));

            
            // 如果父上没有TransformWillChange， 此处m为TransformWillChange作用后， 节点真实的世界矩阵
            let m_owner = p_matrix * &will_change_matrix * invert;
            let mut m = m_owner.clone();

            if let Some(parent_will_change_matrix) = &parent_will_change_matrix.0 {
                // 如果父上下文上存在TransformWillChange， 真实的世界矩阵应该需要与父上下文作用
                m = &parent_will_change_matrix.view_matrix * &m;
                // will_change_matrix = &parent_will_change_matrix.primitive * &will_change_matrix;
            }

            // log::warn!("will_change_matrix: {:?}, \n{:?}", (id, &will_change.all_transform), transform);
			// log::warn!("will_change: {:?}, {:?}, \nparent_will_change_matrix: {:?}, will_change: {:?}, invert: {:?}", id, will_change_matrix, parent_will_change_matrix, will_change, invert);

            if let Ok(mut r) = query.get_mut(id) {
                // will_change修改， 发送就的Willchange矩阵
                if r.is_changed() {
                    events_writer.send(OldTransformWillChange {
                        matrix: (*r).clone(),
                        entity: id,
                        // inpass_id: ***inpass_id,
                        // root: layer.root(),
                    });
                }
                let will_change_matrix = TransformWillChangeMatrix::new(m.invert().unwrap(), m, m_owner);
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
                dirty_mark,
                events_writer,
            );
        }
    }
}
