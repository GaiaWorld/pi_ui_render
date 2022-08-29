//! 处理blur属性，对blur设置大于0.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}, query::{Changed, Deleted, Or}};
use pi_ecs_macros::{listen, setup};
use pi_postprocess::effect::blur_dual::BlurDual;

use crate::{components::{user::{Node, Blur}, calc::{RenderContextMark, Pass2DId}, pass_2d::{PostProcessList, Pass2D}}, resource::RenderContextMarkType};

pub struct CalcBlur;

#[setup]
impl CalcBlur {
	#[listen(component=(Node, Blur, (Create, Modify, Delete)))]
	pub fn blur_change(
		e: Event,
		blur: Query<Node, &Blur>,
		render_mark: Query<Node, Write<RenderContextMark>>,
		local: Local<RenderContextMarkType>,
	) {
		let blur_item = blur.get_by_entity(e.id);

		let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
		let mut render_mark_value = render_mark_item.get_or_default().clone();

		match blur_item {
			Some(blur_item) if **blur_item > 0.0 => {
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

	#[system]
	pub fn calc_blur(
		his_dirty: Query<Node, (Option<&Blur>, Option<&Pass2DId>), Or<(Changed<Blur>, Deleted<Blur>)>>,
		// mark_type: Res<OpacityRenderContextMarkType>,
		mut pass_query: Query<Pass2D, Write<PostProcessList>>,
	) {

		for (blur, pass2d_id) in his_dirty.iter() {
			let pass2d_id = match pass2d_id {
				Some(r) => r,
				None => continue
			};

			match (blur, pass_query.get_mut(pass2d_id.0)) {
				(Some(blur), Some(mut post_list)) => {
					let post_list = match post_list.get_mut() {
						Some(r) => r,
						None => {
							post_list.write(PostProcessList::default());
							post_list.get_mut().unwrap()
						}
					};
					post_list.blur_dual = Some(BlurDual { radius: blur.0 as u8, iteration: 2, intensity: 1.0, simplified_up: false });
					
				},
				(_, None) => {},
				(_, Some(mut post_list)) =>  {
					// opacity不存在，或者opacity为1.0，则删除对应的后处理
					if let Some(post_list) = post_list.get_mut() {
						post_list.blur_dual = None;
					}
				},
			}
		}
	}
}

