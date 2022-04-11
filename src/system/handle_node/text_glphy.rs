//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use pi_ecs::{prelude::{Query, filter_change::Changed, Or, ResMut, OrDefault,With}, component::Component, entity::Entity};
use pi_ecs_utils::prelude::Layer;
use pi_share::Share;
use pi_slotmap::{Key, DelaySlotMap};

use crate::{
	components::{
		user::{TextContent, Node, TextStyle, Vector4},
		calc::{NodeState, WorldMatrix}
	}, 
	utils::font::{font_tex::FontTexture, font_sheet::{FontSheet, get_size}}, utils::stdcell::StdCell,
};

/// 文字劈分
/// 将可以简单布局的问文字节点转化为。。
/// 将需要图文混排的文字节点，劈分为单个文字节点
pub fn text_split<T: FontTexture + Component>(
	mut query: Query<
		Node, 
		(
			Entity,
			&WorldMatrix,
			OrDefault<TextStyle>,
			&mut NodeState,
		), 
		(Or<(Changed<NodeState>, Changed<WorldMatrix>)>, With<TextContent>, With<Layer>)
	>,
	mut query1: Query<
		Node, 
		(
			Entity,
			&WorldMatrix,
			OrDefault<TextStyle>,
			&mut NodeState,
		), 
		(With<TextContent>, With<Layer>)
	>,
	font_sheet: ResMut<Share<StdCell<FontSheet<T>>>>
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
					font_sheet.clear_gylph();

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
	
}

pub fn set_gylph<T: FontTexture + Component>(
	entity: Entity,
	world_matrix: &WorldMatrix,
	text_style: &TextStyle, 
	node_state: &mut NodeState,
	font_sheet: &mut FontSheet<T>) -> Result<(), ()> {
	
	let scale = Vector4::from(world_matrix.fixed_columns(1));
	let scale = scale.dot(&scale).sqrt();
	if scale < 0.000001 {
		return Ok(());
	}
	
	let (tex_font, font_size) = match font_sheet.get_font_info(&text_style.font_family) {
		Some(r) => (r.0.clone(), get_size(r.1, &text_style.font_size) as f32),
		None => {
			log::debug!("font is not exist, face_name: {:?}, entity: {:?}",
			text_style.font_family,
			entity);
			return Ok(());
		}
	};

	let weight = text_style.font_weight;
	let sw = text_style.text_stroke.width;

	node_state.0.scale = scale;
	let chars = &mut node_state.0.text;
	let mut char_id;
	// clear_gylph
	for char_node in chars.iter_mut(){
		if char_node.ch > ' ' {
			char_id = font_sheet.calc_gylph(
				&tex_font,
				font_size as usize,
				sw as usize,
				weight,
				scale,
				char_node.base_width,
				char_node.ch,
			);
			// 异常，无法计算字形
			if char_id.is_null() {
				log::warn!("异常，无法计算字形,char:{:?}, family:{:?}, id:{:?}", char_node.ch, text_style.font_family, entity);
				return Err(());
			}
			char_node.ch_id = char_id;
		}
	}
	Ok(())
}
