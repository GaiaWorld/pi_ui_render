// //! 处理opacity属性，对opacity设置小于1.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

// use pi_ecs::{monitor::Event, prelude::{Query, Write, Or, Changed, Deleted, FromWorld, SingleRes}};
// use pi_ecs_macros::{listen, setup};
// use pi_postprocess::effect::alpha::Alpha;

// use crate::{components::{user::{Node, Opacity}, calc::{RenderContextMark, Pass2DId}, pass_2d::{PostProcessList, Pass2D}}, resource::{RenderContextMarkType, draw_obj::{ImageStaticIndex,CommonPipelineState}}};

// pub struct CalcClipPath;

// #[derive(Deref)]
// pub struct OpacityRenderContextMarkType(RenderContextMarkType);

// impl FromWorld for OpacityRenderContextMarkType{
//     fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
//         Self(RenderContextMarkType::from_world(world))
//     }
// }

// #[setup]
// impl CalcClipPath {
// 	#[listen(component=(Node, Opacity, (Create, Modify, Delete)))]
// 	pub fn opacity_change(
// 		e: Event,
// 		opacity: Query<Node, &Opacity>,
// 		render_mark: Query<Node, Write<RenderContextMark>>,
// 		mark_type: SingleRes<OpacityRenderContextMarkType>,
// 	) {
// 		let opacity_item = opacity.get_by_entity(e.id);

// 		let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
// 		let mut render_mark_value = render_mark_item.get_or_default().clone();

// 		match opacity_item {
// 			Some(opacity_item) if **opacity_item < 1.0 => {
// 				render_mark_value.set(***mark_type, true);
// 			},
// 			_ => {
// 				render_mark_value.set(***mark_type, false);
// 				if render_mark_value.not_any() {
// 					render_mark_item.remove();
// 					return;
// 				}
// 			},
// 		};

// 		render_mark_item.write(render_mark_value);
		
// 	}
// }

// /// 计算半透明后处理
// pub struct CalcOpacityPostProcess;

// #[setup]
// impl CalcOpacityPostProcess {
// 	#[system]
// 	pub fn opacity_change(
// 		opacity_dirty: Query<Node, (Option<&Opacity>, Option<&Pass2DId>), Or<(Changed<Opacity>, Deleted<Opacity>)>>,
// 		// mark_type: SingleRes<OpacityRenderContextMarkType>,
// 		mut pass_query: Query<Pass2D, Write<PostProcessList>>,
// 		static_index: SingleRes<ImageStaticIndex>,
// 		common_state: SingleRes<CommonPipelineState>,
// 	) {
// 		let mut static_index = (*static_index).clone();
// 		static_index.pipeline_state = common_state.premultiply.clone();

// 		for (opacity, pass2d_id) in opacity_dirty.iter() {
// 			let pass2d_id = match pass2d_id {
// 				Some(r) => r,
// 				None => continue
// 			};

// 			match (opacity, pass_query.get_mut(pass2d_id.0)) {
// 				(Some(opacity), Some(mut post_list)) if opacity.0 < 1.0 => {
// 					let post_list = match post_list.get_mut() {
// 						Some(r) => r,
// 						None => {
// 							post_list.write(PostProcessList::default());
// 							post_list.get_mut().unwrap()
// 						}
// 					};
// 					post_list.alpha = Some(Alpha{a: opacity.0});
// 				},
// 				(_, None) => {},
// 				(_, Some(mut post_list)) =>  {
// 					// opacity不存在，或者opacity为1.0，则删除对应的后处理
// 					if let Some(post_list) = post_list.get_mut() {
// 						post_list.alpha = None;
// 					}
// 				},
// 			}
// 		}
// 	}
// }
