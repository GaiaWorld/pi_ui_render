//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use pi_world::prelude::{Entity, OrDefault, Changed, With, ParamSet, Query, SingleResMut, Local, Mut};
use ordered_float::NotNan;
use pi_bevy_ecs_extend::{
    prelude::Layer,
    system_param::res::OrInitSingleRes,
};
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, font::sdf2_table::TexInfo};
use pi_key_alloter::DefaultKey;
use pi_render::font::{Font, FontSheet, FontType};
use pi_share::{Share, ShareMutex};

use crate::{
    components::{
        calc::{NodeState, WorldMatrix},
        user::{get_size, TextContent, TextStyle, TextOverflowData},
    },
    resource::ShareFontSheet,
};
use pi_async_rt::prelude::AsyncRuntime;

use super::IsRun;

pub struct Sdf2GlpyhAwaitList(pub Share<ShareMutex<Vec<(Vec<Entity>, Share<ShareMutex<(usize, Vec<(DefaultKey, TexInfo, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>)>>)>>>);

impl Default for Sdf2GlpyhAwaitList {
    fn default() -> Self {
        Self(Share::new(ShareMutex::new(Vec::new())))
    }
}

/// 文字字形计算
pub fn text_glyph(
    mut query: ParamSet<(
        Query<
            (
                Entity,
                // &'static WorldMatrix,
                OrDefault<TextStyle>,
                // &'static TextContent,
                &'static mut NodeState,
				&'static Layer,
				Option<&'static mut TextOverflowData>,
            ),
            (
                (Changed<TextContent>, Changed<TextStyle>, Changed<WorldMatrix>, Changed<TextOverflowData>),
                With<TextContent>,
            ),
        >,
        Query<
            (
                Entity,
                // &'static WorldMatrix,
                OrDefault<TextStyle>,
                // &'static TextContent,
                &'static mut NodeState,
				&'static Layer,
				Option<&'static mut TextOverflowData>,
            ),
            With<TextContent>,
        >,
		Query<&mut NodeState, With<TextContent>>,
    )>,
    font_sheet: SingleResMut<ShareFontSheet>,
    // mut event_writer: EventWriter<ComponentEvent<Changed<NodeState>>>,
	r: OrInitSingleRes<IsRun>,
	await_list: Local<Sdf2GlpyhAwaitList>,
) {
	if r.0 {
		return;
	}
    let mut font_sheet = font_sheet.borrow_mut();
    let mut is_reset = false;

	let mut await_set_gylph = Vec::new();
    for (
        entity,
        // world_matrix,
        text_style,
        // text_content,
        node_state,
		layer,
		text_overflow_data,
    ) in query.p0().iter_mut()
    {
		let r = set_gylph(entity, layer, text_style, node_state, &mut font_sheet, text_overflow_data);
        if let Err(_) = r {
            // 清空文字纹理TODO（清屏为玫红色）

            is_reset = true;
            // 清空字形信息
            font_sheet.clear();
            break;
        } else if let Ok(false) = r {
			await_set_gylph.push(entity);
		}
    }

    if is_reset {
		await_set_gylph.clear();
        // 为当前所有需要显示的字符，重新分配字形信息
        for (
            entity,
            // world_matrix,
            text_style,
            // text_content,
            node_state,
			layer,
			text_overflow_data,
        ) in query.p1().iter_mut()
        {
            let r = set_gylph(entity, layer, text_style, node_state, &mut font_sheet, text_overflow_data).unwrap();
			if r == false {
				await_set_gylph.push(entity);
			}
        }
    }

	let font_type = font_sheet.font_mgr().font_type;

	// 如果是sdf2， 则设置就绪字形对应节点的NodeState的修改版本
	if let FontType::Sdf2 = font_type {
		if await_set_gylph.len() > 0 {
			let list = (*await_list).0.clone();
			let cur_await = font_sheet.draw_await();
			MULTI_MEDIA_RUNTIME.spawn(async move {
			    let r = cur_await.await;
				list.lock().unwrap().push((await_set_gylph, r));
			}).unwrap();
		}

		let p2 = query.p2();
		for (await_set_gylph, result) in await_list.0.lock().unwrap().drain(..) {
			font_sheet.update_sdf2(result); // 更新纹理
			for entity in await_set_gylph.iter() {
				if let Ok(mut node_state) = p2.get_mut(*entity) {
					node_state.set_changed();
				}
			}
			log::debug!("await_set_gylph================{:?}", await_set_gylph);
		}
	} else {
		font_sheet.update()
	}
}


fn set_gylph(
    entity: Entity,
	layer: &Layer,
    // world_matrix: &WorldMatrix,
    text_style: &TextStyle,
    // text_content: &TextContent,
    mut node_state: Mut<NodeState>,
    font_sheet: &mut FontSheet,
	text_overflow_data: Option<Mut<'_, TextOverflowData>>
) -> Result<bool, ()> { // 返回字形是否已经准备就绪
	if layer.layer() == 0 {
		return Ok(true);
	}
	let font_type = font_sheet.font_mgr().font_type;
    // let scale = Vector4::from(world_matrix.fixed_columns(1));
    // let scale = scale.dot(&scale).sqrt();
    // // log::warn!("set_gylph============={:?}, {:?}, {:?}", entity, text_content, scale);
    // if scale < 0.000001 {
    //     node_state.0.scale = scale;
    //     return Ok(true);
    // }

	let mut is_ready = true;
    // let font_size = (get_size(&text_style.font_size) as f32 * scale).round() as usize;
    let font_size = get_size(&text_style.font_size);
    let font_id = font_sheet.font_id(Font::new(
        text_style.font_family.clone(),
        font_size,
        text_style.font_weight,
         unsafe { NotNan::new_unchecked((*text_style.text_stroke.width as f32).round())},
    ));


    // TODO
    // let weight = text_style.font_weight;
    // let sw = text_style.text_stroke.width;

    // node_state.0.scale = scale;
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

			// 如果是sdf2渲染，需要修改准备状态
			if let FontType::Sdf2 = font_type {

				log::trace!("sdf2 texture is ready =============={:?},{:?}, {:?}", char_node.ch, char_id, font_sheet.font_mgr().table.sdf2_table.glyph(char_id).cell_size);
				if font_sheet.font_mgr().table.sdf2_table.glyph(char_id).cell_size == 0.0 {
					is_ready = false;
				}
			}
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
			if let FontType::Sdf2 = font_type {
				log::trace!("sdf2 texture is ready=============={:?},{:?}, {:?}", char_node.ch, char_id, font_sheet.font_mgr().table.sdf2_table.glyph(char_id).cell_size);
				if font_sheet.font_mgr().table.sdf2_table.glyph(char_id).cell_size == 0.0 {
					is_ready = false;
				}
			}
		}
	}
    Ok(is_ready)
}
