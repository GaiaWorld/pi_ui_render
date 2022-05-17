//! 处理hsi属性，对hsi中存在不为0的属性时，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;

use crate::{components::{user::{Node, Hsi}, calc::RenderContextMark}, resource::RenderContextMarkType};


#[listen(component=(Node, Hsi, (Create, Modify, Delete)))]
pub fn hsi_change(
	e: Event,
	hsi: Query<Node, &Hsi>,
	render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let hsi_item = hsi.get_by_entity(e.id);

	let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
	let mut render_mark_value = render_mark_item.get_or_default().clone();

	match hsi_item {
		Some(hsi_item) if hsi_item.saturate != 0.0 || hsi_item.hue_rotate != 0.0 || hsi_item.bright_ness != 0.0 => {
			render_mark_value.set(**local, true);
		},
		_ => {
			render_mark_value.set(**local, false);
			if render_mark_value.not_any() {
				render_mark_item.remove();
				return;
			}
		},
	};

	render_mark_item.write(render_mark_value);
	
}