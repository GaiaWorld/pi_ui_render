//! 计算内容包围盒
//! 内容包围盒是指： **自身+递归子节点**的包围盒

use pi_style::style::TransformWillChange;
use pi_world::{event::{ComponentChanged, ComponentAdded}, prelude::{App, IntoSystemConfigs, Plugin}, query::Query};
use pi_bevy_ecs_extend::prelude::{ EntityTree, OrInitSingleResMut};
use crate::{resource::MatrixDirty, system::{base::node::world_matrix::cal_matrix, system_set::UiSystemSet}};
use pi_bevy_ecs_extend::system_param::layer_dirty::{RemainDirty, OutDirty};

use pi_null::Null;

use crate::{
    components::{
        calc::{ContentBox, EntityKey, LayoutResult, Quad, TransformWillChangeMatrix, WorldMatrix},
        user::{Aabb2, BoxShadow, Point2, TextShadow},
    }, utils::tools::calc_bound_box
};
use crate::prelude::UiStage;

pub struct ContentBoxPlugin;

impl Plugin for ContentBoxPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_system(UiStage, calc_content_box.in_set(UiSystemSet::PassSetting)
				.after(cal_matrix)
		);
	}
}

/// 计算内容包围盒（包含布局的包围盒，和世界坐标系的包围盒）
pub fn calc_content_box(
    mut dirty: OrInitSingleResMut<MatrixDirty>,
    node_box: Query<(&Quad, &LayoutResult, Option<&TextShadow>, Option<&BoxShadow>, &WorldMatrix)>,
    entity_tree: EntityTree,
    text_shadow_dirty: ComponentChanged<TextShadow>,
    box_shadow_dirty: ComponentChanged<BoxShadow>,
    text_shadow_added: ComponentAdded<TextShadow>, 
    box_shadow_added: ComponentAdded<BoxShadow>, 
    mut content_box: Query<(&mut ContentBox, Option<&TransformWillChange>, Option<&TransformWillChangeMatrix>)>,
	// r: OrInitSingleRes<IsRun>
) {
	// if r.0 {
	// 	return;
	// }
    let dirty = &mut ***dirty;
    if text_shadow_dirty.len() > 0 || box_shadow_dirty.len() > 0 || text_shadow_added.len() > 0 || box_shadow_added.len() > 0 {
        for i in text_shadow_dirty.iter().chain(box_shadow_dirty.iter()).chain(text_shadow_added.iter()).chain(box_shadow_added.iter()) {
            dirty.marked_dirty(*i, *i, &entity_tree);
        }
    }

    if dirty.dirty.count() == 0 {
        return;
    }
    let mut end = dirty.dirty.end();

    // 从最大的层开始迭代
    while end > 0 {
        // 将脏劈分为两部分：1.当前迭代的层， 2.剩余部分
        // 在迭代当前层的过程中，可能继续设置父脏，因此将当前迭代层劈分出来
        let (remain, cur) = dirty.dirty.split(end - 1);
        let (mut remain, mut cur) = (RemainDirty(remain), OutDirty(cur, &mut dirty.dirty_mark_list));
        for id in cur.iter() {
            let mut chilren_change = false;
            // 当前节点的oct
            let (mut oct, mut layout, x, y, text_shadow, box_shadow, world_matrix) = match node_box.get(id) {
                Ok(r) => (
                    r.0 .0.clone(),
                    Aabb2::new(
                        Point2::new(0.0, 0.0),
                        Point2::new(r.1.rect.right - r.1.rect.left, r.1.rect.bottom - r.1.rect.top),
                    ),
                    r.1.rect.left + r.1.border.left + r.1.padding.left,
                    r.1.rect.top + r.1.border.top + r.1.padding.top,
                    r.2,
                    r.3,
                    r.4,
                ),
                _ => continue,
            };

            let mut has_extends = false;
            let mut offset: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0); // 左上右下
            if let Some(text_shadow) = text_shadow {
                has_extends = true;
                for shadow in text_shadow.iter() {
                    let e = (shadow.blur * 1.5).ceil();
                    offset.0 = offset.0.min(shadow.h - e);
                    offset.1 = offset.1.min(shadow.v - e);
                    offset.2 = offset.2.max(shadow.h + e);
                    offset.3 = offset.3.max(shadow.v + e);
                }
            }

            if let Some(shadow) = box_shadow {
                has_extends = true;
                offset.0 = offset.0.min(shadow.h - shadow.blur - shadow.spread);
                offset.1 = offset.1.min(shadow.v - shadow.blur - shadow.spread);
                offset.2 = offset.2.max(shadow.h + shadow.blur + shadow.spread);
                offset.3 = offset.3.max(shadow.v + shadow.blur + shadow.spread);
            }

            if has_extends {
                // 由于阴影的影响， 重新计算layout和oct
                layout.mins.x += offset.0;
                layout.mins.y += offset.1;
                layout.maxs.x += offset.2;
                layout.maxs.y += offset.3;

                oct = calc_bound_box(&layout, world_matrix);
            }

            // Aabb2::new(
            // 	Point2::new(0.0, 0.0),
            // 	Point2::new(r.1.rect.right - r.1.rect.left, r.1.rect.bottom - r.1.rect.top),
            // )
            // log::warn!("context_box parent======: id: {:?} box_shadow: {:?}, text_shadow: {:?}", id, box_shadow, text_shadow);
            // 如果存在子节点，求所有子节点的ContextBox和自身的Oct的并
            if let Some(down_item) = entity_tree.get_down(id) {
                let mut child = down_item.head();

                while !EntityKey(child).is_null() {
                    // 如果content_box不存在，则节点不是一个真实的节点，可能是一个文字节点，不需要计算
                    if let Ok((content_box_item, transform_will_change, transform_will_change_matrix)) = content_box.get(child) {
                        // log::warn!("context_box======: id: {:?} child: {:?} layout: {:?}, layout: {:?}", id, child, layout, content_box_item.layout);
                        // log::warn!("context_box1======: id: {:?} child: {:?} layout: {:?}, layout: {:?}", id, child, oct, content_box_item.oct);
                        if let (Some(_transform_will_change), Some(TransformWillChangeMatrix(Some(transform_will_change_matrix)))) = (transform_will_change, transform_will_change_matrix) {
                            
                            let child_oct = calc_bound_box(&content_box_item.oct, &transform_will_change_matrix.primitive );
                            box_and(&mut oct, &child_oct);
                        } else {
                            box_and(&mut oct, &content_box_item.oct);
                        }
                        
                        box_and(&mut layout, &content_box_item.layout);
                        
                        
                        let up = entity_tree.get_up(child).unwrap();
                        child = up.next();
                    } else {
                        break;
                    }
                }
            }

            layout.mins.x += x;
            layout.mins.y += y;
            layout.maxs.x += x;
            layout.maxs.y += y;

            let (mut old, _, _) = content_box.get_mut(id).unwrap();

            if old.oct != oct || 
            old.layout != layout {
                chilren_change = true;
            }

            log::debug!("content_box===================={:?}", (id, &old.oct, &oct, chilren_change));
            // 如果内容包围盒发生改变，则重新插入内容包围盒，并标记父脏
            if chilren_change {
                old.oct = oct;
                old.layout = layout;
                if let Some(up) = entity_tree.get_up(id) {
                    if !EntityKey(up.parent()).is_null() {
                        let layer = entity_tree.get_layer(up.parent()).unwrap();
                        remain.mark(up.parent(), layer.layer());
                    }
                }
            }
        }
        end -= 1;
    }
    dirty.clear();
}

// 两个aabb的并
fn box_and(aabb1: &mut Aabb2, aabb2: &Aabb2) {
    aabb1.mins.x = aabb1.mins.x.min(aabb2.mins.x);
    aabb1.mins.y = aabb1.mins.y.min(aabb2.mins.y);
    aabb1.maxs.x = aabb1.maxs.x.max(aabb2.maxs.x);
    aabb1.maxs.y = aabb1.maxs.y.max(aabb2.maxs.y);
}