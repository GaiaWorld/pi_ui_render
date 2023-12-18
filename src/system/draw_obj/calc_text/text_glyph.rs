//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use bevy_ecs::{
    prelude::{Entity, EventWriter},
    query::{Changed, Or, With},
    system::{ParamSet, Query, ResMut}, change_detection::DetectChangesMut,
};
use ordered_float::NotNan;
use pi_bevy_ecs_extend::{
    prelude::{Layer, OrDefault},
    system_param::{layer_dirty::ComponentEvent, res::OrInitRes},
};
use pi_render::font::{Font, FontSheet};

use crate::{
    components::{
        calc::{NodeState, WorldMatrix},
        user::{get_size, TextContent, TextStyle, Vector4, TextOverflowData},
    },
    resource::ShareFontSheet,
};

use super::IsRun;

/// 文字字形计算
pub fn text_glyph(
    mut query: ParamSet<(
        Query<
            (
                Entity,
                &'static WorldMatrix,
                OrDefault<TextStyle>,
                // &'static TextContent,
                &'static mut NodeState,
				&'static Layer,
				Option<&'static mut TextOverflowData>,
            ),
            (
                Or<(Changed<TextContent>, Changed<TextStyle>, Changed<WorldMatrix>, Changed<TextOverflowData>)>,
                With<TextContent>,
            ),
        >,
        Query<
            (
                Entity,
                &'static WorldMatrix,
                OrDefault<TextStyle>,
                // &'static TextContent,
                &'static mut NodeState,
				&'static Layer,
				Option<&'static mut TextOverflowData>,
            ),
            With<TextContent>,
        >,
    )>,
    font_sheet: ResMut<ShareFontSheet>,
    mut event_writer: EventWriter<ComponentEvent<Changed<NodeState>>>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    let mut font_sheet = font_sheet.borrow_mut();
    let mut is_reset = false;
    for (
        entity,
        world_matrix,
        text_style,
        // text_content,
        mut node_state,
		layer,
		text_overflow_data,
    ) in query.p0().iter_mut()
    {
        if let Err(_) = set_gylph(entity, layer, world_matrix, text_style, &mut node_state, &mut font_sheet, &mut event_writer, text_overflow_data) {
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
            mut node_state,
			layer,
			text_overflow_data,
        ) in query.p1().iter_mut()
        {
            set_gylph(entity, layer, world_matrix, text_style, &mut node_state, &mut font_sheet, &mut event_writer, text_overflow_data).unwrap();
        }
    }


    // 绘制文字
    font_sheet.draw();
}


fn set_gylph(
    entity: Entity,
	layer: &Layer,
    world_matrix: &WorldMatrix,
    text_style: &TextStyle,
    // text_content: &TextContent,
    node_state: &mut NodeState,
    font_sheet: &mut FontSheet,
    event_writer: &mut EventWriter<ComponentEvent<Changed<NodeState>>>,
	text_overflow_data: Option<bevy_ecs::change_detection::Mut<'_, TextOverflowData>>
) -> Result<(), ()> {
	if layer.layer() == 0 {
		return Ok(());
	}
    let scale = Vector4::from(world_matrix.fixed_columns(1));
    let scale = scale.dot(&scale).sqrt();
    // log::warn!("set_gylph============={:?}, {:?}, {:?}", entity, text_content, scale);
    if scale < 0.000001 {
        node_state.0.scale = scale;
        return Ok(());
    }

    let font_size = (get_size(&text_style.font_size) as f32 * scale).round() as usize;
    let font_id = font_sheet.font_id(Font::new(
        text_style.font_family.clone(),
        font_size,
        text_style.font_weight,
         unsafe { NotNan::new_unchecked((*text_style.text_stroke.width as f32 * scale).round())},
    ));


    // TODO
    // let weight = text_style.font_weight;
    // let sw = text_style.text_stroke.width;

    node_state.0.scale = scale;
    let chars = &mut node_state.0.text;
    let mut char_id;

    for char_node in chars.iter_mut() {
        if char_node.ch > ' ' {
            let glyph_id = font_sheet.glyph_id(font_id, char_node.ch);
            // 异常，无法计算字形
            char_id = match glyph_id {
                Some(r) => r,
                None => {
                    // 纹理空间不足
                    log::info!(
                        "异常，无法计算字形,char:{:?}, family:{:?}, id:{:?}, texture_width: {:?}, texture_height: {:?}",
                        char_node.ch,
                        text_style.font_family,
                        entity,
						font_sheet.font_mgr().size().width,
						font_sheet.font_mgr().size().height,
                    );
                    return Err(());
                }
            };
            char_node.ch_id = *char_id;
        }
    }

	if let Some(mut text_overflow) = text_overflow_data {
		let text_overflow = text_overflow.bypass_change_detection();
		for char_node in text_overflow.text_overflow_char.iter_mut() {
			let glyph_id = font_sheet.glyph_id(font_id, char_node.ch);
			// 异常，无法计算字形
			char_id = match glyph_id {
                Some(r) => r,
                None => {
                    // 纹理空间不足
                    log::info!(
                        "异常，无法计算字形,char:{:?}, family:{:?}, id:{:?}, texture_width: {:?}, texture_height: {:?}",
                        char_node.ch,
                        text_style.font_family,
                        entity,
						font_sheet.font_mgr().size().width,
						font_sheet.font_mgr().size().height,
                    );
                    return Err(());
                }
            };
            char_node.ch_id = *char_id;
		}
	}
    event_writer.send(ComponentEvent::new(entity));
    Ok(())
}
