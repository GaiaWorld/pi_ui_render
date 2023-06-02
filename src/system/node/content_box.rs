//! 计算内容包围盒
//! 内容包围盒是指： **自身+递归子节点**的包围盒

use bevy::ecs::{prelude::EventWriter, query::Changed, system::Query};
use pi_bevy_ecs_extend::{
    prelude::{Down, Layer, LayerDirty, Up},
    system_param::layer_dirty::ComponentEvent,
};
use pi_null::Null;

use crate::{components::{
    calc::{ContentBox, EntityKey, LayoutResult, Quad, WorldMatrix},
    user::{Aabb2, Point2, TextShadow, BoxShadow},
}, utils::tools::calc_bound_box};

pub struct CalcContentBox;

/// 计算内容包围盒（包含布局的包围盒，和世界坐标系的包围盒）
pub fn calc_content_box(
    mut dirty: LayerDirty<Changed<Quad>>,
    node_box: Query<(&Quad, &LayoutResult, Option<&TextShadow>, Option<&BoxShadow>, &WorldMatrix)>,
    down: Query<&Down>,
    up: Query<&Up>,
    layer: Query<&Layer>,
    mut content_box: Query<&mut ContentBox>,
    mut event_writer: EventWriter<ComponentEvent<Changed<ContentBox>>>,
) {
    if dirty.count() == 0 {
        return;
    }
    let mut end = dirty.end();

    // 从最大的层开始迭代
    while end > 0 {
        // 将脏劈分为两部分：1.当前迭代的层， 2.剩余部分
        // 在迭代当前层的过程中，可能继续设置父脏，因此将当前迭代层劈分出来
        let (mut remain, mut cur) = dirty.split(end - 1);
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
                    r.1.rect.left,
                    r.1.rect.top,
					r.2,
					r.3,
					r.4
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
				// 由于阴影的影响， 重新阶段layout和oct
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

            // log::warn!("oct====={:?}, {:?}", id, oct);

            // 如果存在子节点，求所有子节点的ContextBox和自身的Oct的并
            if let Ok(down_item) = down.get(id) {
                let mut child = down_item.head();

                while !EntityKey(child).is_null() {
                    // 如果content_box不存在，则节点不是一个真实的节点，可能是一个文字节点，不需要计算
                    if let Ok(content_box_item) = content_box.get(child) {
                        // log::warn!("content_box_item====={:?}, {:?}, {:?}, {:?}", id, child, oct, content_box_item.oct);
                        box_and(&mut oct, &content_box_item.oct);
                        box_and(&mut layout, &content_box_item.layout);
                        // log::warn!("content_box_item1====={:?}, {:?}", child, oct);
                        let up = up.get(child).unwrap();
                        child = up.next();
                    } else {
                        break;
                    }
                }
            }

            let mut old = content_box.get_mut(id).unwrap();

            if old.oct != oct || old.layout != layout {
                chilren_change = true;
            }

            layout.mins.x += x;
            layout.mins.y += y;
            layout.maxs.x += x;
            layout.maxs.y += y;

            // 如果内容包围盒发生改变，则重新插入内容包围盒，并标记父脏
            if chilren_change {
                old.oct = oct;
                old.layout = layout;
                event_writer.send(ComponentEvent::new(id));
                if let Ok(up) = up.get(id) {
                    if !EntityKey(up.parent()).is_null() {
                        let layer = layer.get(id).unwrap();
                        remain.mark(up.parent(), layer.layer());
                    }
                }
            }
        }
        end -= 1;
    }
}

// 两个aabb的并
fn box_and(aabb1: &mut Aabb2, aabb2: &Aabb2) {
    aabb1.mins.x = aabb1.mins.x.min(aabb2.mins.x);
    aabb1.mins.y = aabb1.mins.y.min(aabb2.mins.y);
    aabb1.maxs.x = aabb1.maxs.x.max(aabb2.maxs.x);
    aabb1.maxs.y = aabb1.maxs.y.max(aabb2.maxs.y);
}
