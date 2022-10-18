//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use pi_ecs::{prelude::{Query, Changed, Or, ResMut, OrDefault,With, Id, ParamSet}, query::{Write, WriteItem}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::Layer;
use pi_share::{Share, ShareCell};
use pi_render::font::{FontSheet, Font};
use pi_style::style::FontStyle;

use crate::{
	components::{
		user::{TextContent, Node, TextStyle, Vector4, get_size},
		calc::{NodeState, WorldMatrix}
	}
};

pub struct CalcTextGlyph;

#[setup]
impl CalcTextGlyph {
	/// 文字字形计算
	/// 将需要图文混排的文字节点，劈分为单个文字节点
	#[system]
	pub fn text_glyph(
		mut query: ParamSet<(
			Query<
				Node, 
				(
					Id<Node>,
					&'static WorldMatrix,
					OrDefault<TextStyle>,
					// &'static TextContent,
					Write<NodeState>,
				), 
				(Or<(Changed<TextContent>, Changed<FontStyle>, Changed<WorldMatrix>)>, With<TextContent>, With<Layer<Node>>)
			>,
			Query<
				Node, 
				(
					Id<Node>,
					&'static WorldMatrix,
					OrDefault<TextStyle>,
					// &'static TextContent,
					Write<NodeState>,
				), 
				(With<TextContent>, With<Layer<Node>>)
			>
		)>,
		font_sheet: ResMut<Share<ShareCell<FontSheet>>>
	) {

		let mut font_sheet = font_sheet.borrow_mut();
		let mut is_reset = false;
		for (
			entity,
			world_matrix,
			text_style, 
			// text_content,
			node_state) in query.p0_mut().iter_mut() {
			
			if let Err(_) = set_gylph(entity, world_matrix, text_style, node_state, &mut font_sheet) {
				// 清空文字纹理TODO（清屏为玫红色）

				is_reset = true;
				// 清空字形信息
				font_sheet.clear();
				break;
			}
		}

		if is_reset {
			// 为当前所有需要显示的字符，重新分配字形信息
			for (
				entity,
				world_matrix,
				text_style, 
				// text_content,
				node_state) in query.p1_mut().iter_mut() {
				set_gylph(entity, world_matrix, text_style, node_state, &mut font_sheet).unwrap();
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
	// text_content: &TextContent,
	mut node_state: WriteItem<NodeState>,
	font_sheet: &mut FontSheet) -> Result<(), ()> {
	
	let scale = Vector4::from(world_matrix.fixed_columns(1));
	let scale = scale.dot(&scale).sqrt();
	// log::warn!("set_gylph============={:?}, {:?}, {:?}", entity, text_content, scale);
	if scale < 0.000001 {
		let state = node_state.get_mut().unwrap();
		state.0.scale = scale;
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


	// TODO
	// let weight = text_style.font_weight;
	// let sw = text_style.text_stroke.width;

	let state = node_state.get_mut().unwrap();
	state.0.scale = scale;
	let chars = &mut state.0.text;
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
	node_state.notify_modify();
	Ok(())
}



