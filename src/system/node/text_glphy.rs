//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use pi_ecs::prelude::{Query, Changed, Or, ResMut, OrDefault,With, Id};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::Layer;
use pi_share::{Share, ShareCell};
use pi_render::font::{FontSheet, Font};

use crate::{
	components::{
		user::{TextContent, Node, TextStyle, Vector4, get_size},
		calc::{NodeState, WorldMatrix}
	}
};

pub struct CalcTextGlyph;

#[setup]
impl CalcTextGlyph {
	/// 文字劈分
	/// 将可以简单布局的问文字节点转化为。。
	/// 将需要图文混排的文字节点，劈分为单个文字节点
	#[system]
	pub fn text_split(
		mut query: Query<
			Node, 
			(
				Id<Node>,
				&WorldMatrix,
				OrDefault<TextStyle>,
				&mut NodeState,
			), 
			(Or<(Changed<NodeState>, Changed<WorldMatrix>)>, With<TextContent>, With<Layer>)
		>,
		mut query1: Query<
			Node, 
			(
				Id<Node>,
				&WorldMatrix,
				OrDefault<TextStyle>,
				&mut NodeState,
			), 
			(With<TextContent>, With<Layer>)
		>,
		font_sheet: ResMut<Share<ShareCell<FontSheet>>>
	) {

		let mut font_sheet = font_sheet.borrow_mut();
		let mut is_loop = true;
		while is_loop {
			is_loop = false;
			for (
				entity,
				world_matrix,
				text_style, 
				mut node_state) in query.iter_mut() {
				
				match set_gylph(entity, world_matrix, text_style, &mut node_state, &mut font_sheet) {
					Ok(_) => continue,
					Err(_) => {
						// 清空文字纹理TODO（清屏为玫红色）

						// 清空字形信息
						font_sheet.clear();

						// 为当前所有需要显示的字符，重新分配字形信息
						for (
							entity,
							world_matrix,
							text_style, 
							mut node_state) in query1.iter_mut() {
							set_gylph(entity, world_matrix, text_style, &mut node_state, &mut font_sheet).unwrap();
						}
					}
				};
			}

		}

		// 绘制文字
		font_sheet.draw();
		
	}
}



pub fn set_gylph(
	entity: Id<Node>,
	world_matrix: &WorldMatrix,
	text_style: &TextStyle, 
	node_state: &mut NodeState,
	font_sheet: &mut FontSheet) -> Result<(), ()> {
	
	let scale = Vector4::from(world_matrix.fixed_columns(1));
	let scale = scale.dot(&scale).sqrt();
	if scale < 0.000001 {
		return Ok(());
	}

	let font_size = (get_size(&text_style.font_size) as f32 * scale).round() as usize;
	let font_id = font_sheet.font_id(
		Font::new(
			text_style.font_family.clone(),
			font_size,
			text_style.font_weight,
			text_style.text_stroke.width, // todo 或许应该设置比例
		)
	);

	let weight = text_style.font_weight;
	let sw = text_style.text_stroke.width;

	node_state.0.scale = scale;
	let chars = &mut node_state.0.text;
	let mut char_id;

	for char_node in chars.iter_mut(){
		if char_node.ch > ' ' {
			let glyph_id = font_sheet.glyph_id(font_id, char_node.ch);
			// 异常，无法计算字形
			char_id = match glyph_id {
				Some(r) => r,
				None => {
					// 纹理空间不足
					log::warn!("异常，无法计算字形,char:{:?}, family:{:?}, id:{:?}", char_node.ch, text_style.font_family, entity);
					return Err(());
				}
			};
			char_node.ch_id = *char_id;
		}
	}
	Ok(())
}



