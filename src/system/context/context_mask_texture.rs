//! 处理mask_image属性，对mask_image存在的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}};
use pi_ecs_macros::listen;

use crate::{components::{user::Node, calc::{RenderContextMark, MaskTexture}}, resource::RenderContextMarkType};


#[listen(component=(Node, MaskTexture, (Create, Modify, Delete)))]
pub fn mask_image_change(
	e: Event,
	mask_image: Query<Node, &MaskTexture>,
	render_mark: Query<Node, Write<RenderContextMark>>,
	local: Local<RenderContextMarkType>,
) {
	let mask_image_item = mask_image.get_by_entity(e.id);

	let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
	let mut render_mark_value = render_mark_item.get_or_default().clone();

	match mask_image_item {
		Some(_) => {
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