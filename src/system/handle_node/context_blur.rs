//! 处理blur属性，对blur设置大于0.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;

use crate::{components::{user::{Node, Blur}, calc::RenderContextMark}, resource::RenderContextMarkType};


#[listen(component=(Node, Blur, (Create, Modify, Delete)))]
pub fn blur_change(
	e: Event,
	blur: Query<Node, &Blur>,
	render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let blur_item = blur.get(e.id);

	let mut render_mark_item = render_mark.get_unchecked(e.id);
	let mut render_mark_value = render_mark_item.get_or_default().clone();

	match blur_item {
		Some(blur_item) if **blur_item > 0.0 => {
			render_mark_value.set(**local, true);
		},
		_ => {
			render_mark_value.set(**local, false);
		},
	};

	render_mark_item.write(render_mark_value);
	
}