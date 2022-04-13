//! 处理opacity属性，对opacity设置小于1.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;

use crate::{components::{user::{Node, Opacity}, calc::RenderContextMark}, resource::RenderContextMarkType};


#[listen(component=(Node, Opacity, (Create, Modify, Delete)))]
pub fn opacity_change(
	e: Event,
	opacity: Query<Node, &Opacity>,
	render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let opacity_item = opacity.get(e.id);

	let mut render_mark_item = render_mark.get_unchecked(e.id);
	let mut render_mark_value = render_mark_item.get_or_default().clone();

	match opacity_item {
		Some(opacity_item) if **opacity_item < 1.0 => {
			render_mark_value.set(**local, true);
		},
		_ => {
			render_mark_value.set(**local, false);
		},
	};

	render_mark_item.write(render_mark_value);
	
}