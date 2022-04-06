//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;
use pi_ecs_utils::prelude::Root;

use crate::{components::{user::Node, calc::RenderContextMark}, resource::RenderContextMarkType};


#[listen(component=(Node, Root, (Create, Delete)))]
pub fn root_change(
	e: Event,
	root: Query<Node, &Root>,
	render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let root_item = root.get(e.id);

	let mut render_mark_item = render_mark.get_unchecked(e.id);
	let mut render_mark_value = render_mark_item.get_or_default().clone();

	match root_item {
		Some(_) => {
			render_mark_value.set(**local, true);
		},
		_ => {
			render_mark_value.set(**local, false);
			// 如果所有的位标记都被清除，则调用remove方法
			if render_mark_value.not_any() {
				render_mark_item.remove();
				return;
			}
		},
	};

	render_mark_item.write(render_mark_value);
	
}