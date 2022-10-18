//! 计算内容包围盒
//! 内容包围盒是指： **自身+递归子节点**的包围盒

use pi_ecs::prelude::{Changed, Query, Write};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::{LayerDirty, Down, Up, Layer};
use pi_null::Null;

use crate::components::{calc::{Quad, ContentBox}, user::{Node, Aabb2}};

pub struct CalcContentBox;

#[setup]
impl CalcContentBox {
	#[system]
	pub fn calc_content_box(
		mut dirty: LayerDirty<Node, Changed<Quad>>,
		oct: Query<Node, &Quad>,
		down: Query<Node, Option<&Down<Node>>>,
		up: Query<Node, &Up<Node>>,
		layer: Query<Node, &Layer<Node>>,
		content_box: Query<Node, Write<ContentBox>>
	) {
		if dirty.count() == 0 {
			return;
		}
		let mut end = dirty.end();
		
		// 从最大的层开始迭代
		while end > 0 {
			// 将脏劈分为两部分：1.当前迭代的层， 2.剩余部分
			// 在迭代当前层的过程中，可能继续设置父脏，因此将当前迭代层劈分出来
			let (mut remain, mut out) = dirty.split(end - 1);
			for id in out.iter() {
				let mut chilren_change = false;
				
				// 当前节点的oct
				let mut oct = match oct.get(id) {
					Some(r) => *r.clone(),
					None => continue,
				};
		
				// 如果存在子节点，求所有子节点的ContextBox和自身的Oct的并
				if let Some(down_item) = down.get_unchecked(id) {
					let mut child = down_item.head();
					
					while !child.is_null() {
						// 如果content_box不存在，则节点不是一个真实的节点，可能是一个文字节点，不需要计算
						if let Some(content_box_item) = content_box.get_unchecked(child).get() {
							box_and(&mut oct, &content_box_item.0);
							let up = up.get_unchecked(child);
							child = up.next();
						} else {
							break;
						}
					}
				}
				
				let mut old = content_box.get_unchecked(id);
	
				if let Some(old) = old.get() {
					if old.0 != oct {
						chilren_change = true;
					}
				} else {
					chilren_change = true;
				}
				
				// 如果内容包围盒发生改变，则重新插入内容包围盒，并标记父脏
				if chilren_change {
					old.write(ContentBox(oct));
					if let Some(up) = up.get(id) {
						if !up.parent().is_null() {
							let layer = layer.get_unchecked(id);
							remain.mark(up.parent(), layer.layer());
						}
					}
				}
			}
			end -= 1;
		}
	}
}

// 两个aabb的并
fn box_and(aabb1: &mut Aabb2, aabb2: &Aabb2) {
	aabb1.mins.x = aabb1.mins.x.min(aabb2.mins.x);
	aabb1.mins.y = aabb1.mins.y.min(aabb2.mins.y);
	aabb1.maxs.x = aabb1.maxs.x.max(aabb2.maxs.x);
	aabb1.maxs.y = aabb1.maxs.y.max(aabb2.maxs.y);
}