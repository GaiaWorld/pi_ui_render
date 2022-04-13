//! 处理overflow属性，对overflow设置为true的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;

use crate::{components::{user::{Node, Overflow}, calc::RenderContextMark}, resource::RenderContextMarkType};


#[listen(component=(Node, Overflow, (Create, Modify, Delete)))]
pub fn overflow_change(
	e: Event,
	overflow: Query<Node, &Overflow>,
	mut render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let mut render_context_mark_item = match render_mark.get_mut(e.id) {
		Some(r) => r,
		// 正常情况不会进入该分支，除非e.id实体在Node中不存在
		None => return,
	};
	let overflow_item = overflow.get(e.id);
	let mut render_mark_value = render_context_mark_item.get_or_default().clone();

	// Oveflow为true时，标记render_context_mark对应的位
	// Oveflow为false时, 取消render_context_mark对应的位，如果发现位标记全为空，则删除RenderContextMark组件
	match overflow_item {
		Some(overflow_item) if **overflow_item == true => {
			render_mark_value.set(**local, true);
		},
		_ => {
			render_mark_value.set(**local, false);
			// 如果所有的位标记都被清除，则调用remove方法
			if render_mark_value.not_any() {
				render_context_mark_item.remove();
				return;
			}
		},
	};

	render_context_mark_item.write(render_mark_value);
	
}