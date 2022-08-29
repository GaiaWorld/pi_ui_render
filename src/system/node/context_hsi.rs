//! 处理hsi属性，对hsi中存在不为0的属性时，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_ecs::{monitor::Event, prelude::{Query, Write, Local}, query::{Deleted, Changed, Or}};
use pi_ecs_macros::{listen, setup};
use pi_postprocess::effect::hsb::HSB;

use crate::{components::{user::{Node, Hsi}, calc::{RenderContextMark, Pass2DId}, pass_2d::{Pass2D, PostProcessList}}, resource::RenderContextMarkType};

pub struct CalcHsi;

#[setup]
impl CalcHsi {

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

	#[system]
	pub fn calc_hsi(
		his_dirty: Query<Node, (Option<&Hsi>, Option<&Pass2DId>), Or<(Changed<Hsi>, Deleted<Hsi>)>>,
		// mark_type: Res<OpacityRenderContextMarkType>,
		mut pass_query: Query<Pass2D, Write<PostProcessList>>,
	) {

		for (hsi, pass2d_id) in his_dirty.iter() {
			let pass2d_id = match pass2d_id {
				Some(r) => r,
				None => continue
			};

			match (hsi, pass_query.get_mut(pass2d_id.0)) {
				(Some(hsi), Some(mut post_list)) => {
					let post_list = match post_list.get_mut() {
						Some(r) => r,
						None => {
							post_list.write(PostProcessList::default());
							post_list.get_mut().unwrap()
						}
					};
					post_list.hsb = Some(HSB {hue: (hsi.hue_rotate * 360.0) as i16, saturate: (hsi.saturate * 100.0) as i8, brightness: (hsi.bright_ness * 100.0) as i8});
					
				},
				(_, None) => {},
				(_, Some(mut post_list)) =>  {
					// opacity不存在，或者opacity为1.0，则删除对应的后处理
					if let Some(post_list) = post_list.get_mut() {
						post_list.hsb = None;
					}
				},
			}
		}
	}
}