//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。

use pi_world::{filter::Or, prelude::{Changed, Entity, Mut, OrDefault, ParamSet, Query, SingleResMut, With}};
use pi_bevy_ecs_extend::{
    prelude::OrInitSingleResMut,
    system_param::res::OrInitSingleRes,
};
// use pi_sdf::utils::SdfInfo2;
use pi_render::font::{Font, FontSheet};

use crate::{
    components::{
        calc::{NodeState, StyleBit},
        user::{get_size, TextContent, TextOuterGlow, TextOverflowData, TextStyle},
    },
    resource::{GlobalDirtyMark, IsRun, ShareFontSheet},
};

use super::text_sdf2::TEXT_LAYOUT_DIRTY;

// pub struct Sdf2GlpyhAwaitList(pub Share<ShareMutex<Vec<(Vec<Entity>, Share<ShareMutex<(usize, Vec<(DefaultKey, TexInfo, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>)>>)>>>);

// #[derive(Default)]
// pub struct Sdf2GlpyhAwaitList(pub VecDeque<(Vec<Entity>, Share<AtomicBool>, Arc<ShareMutex<(usize, Vec<(DefaultKey, SdfInfo2)>)>>)>);
// Share<ShareMutex<(usize, Vec<(DefaultKey, TexInfo, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>)>>
// impl Default for Sdf2GlpyhAwaitList {
//     fn default() -> Self {
//         Self(Share::new(ShareMutex::new(Vec::new())))
//     }
// }

/// 文字字形计算
pub fn text_glyph(
    mut query: ParamSet<(
        Query<
            (
                Entity,
                // &'static WorldMatrix,
                OrDefault<TextStyle>,
                Option<&'static TextOuterGlow>,
                // &'static TextContent,
                &'static mut NodeState,
				// &'static Layer,
				Option<&'static mut TextOverflowData>,
                // &TextContent,

                // Option<pi_world::fetch::Ticker<&TextStyle>>,
                // pi_world::fetch::Ticker<&TextContent>,
                // Option<pi_world::fetch::Ticker<&TextOverflowData>>,
            ),
            (
                Or<(Changed<TextContent>, Changed<TextStyle>, Changed<TextOverflowData>, Changed<TextOuterGlow>)>,
                With<TextContent>,
            ),
        >,
        Query<
            (
                Entity,
                // &'static WorldMatrix,
                OrDefault<TextStyle>,
                Option<&'static TextOuterGlow>,
                // &'static TextContent,
                &'static mut NodeState,
				// &'static Layer,
				Option<&'static mut TextOverflowData>,
            ),
            With<TextContent>,
        >,
		Query<&mut NodeState, With<TextContent>>,
    )>,
    font_sheet: SingleResMut<ShareFontSheet>,
    global_mark: OrInitSingleResMut<GlobalDirtyMark>,
    // mut event_writer: EventWriter<ComponentEvent<Changed<NodeState>>>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
    let mut font_sheet = font_sheet.borrow_mut();
    let mut is_reset = false;

	let mut await_set_gylph = Vec::new();
    // let mut ii1 = Vec::new();
    // let t0 = pi_time::Instant::now();
    if global_mark.mark.has_any(&*TEXT_LAYOUT_DIRTY) {
        for (
            entity,
            // world_matrix,
            text_style,
            text_outer_glow,
            // text_content,
            node_state,
            // layer,
            text_overflow_data,
            // text_content,

            // t1, t2, t3
            // t2
        ) in query.p0().iter_mut()
        {
            // log::warn!("set_gylph============{:?}", entity);
            // ii1.push(entity);
            // println!("text_glyph======{:?}", (t1.map(|t| {t.is_changed()}), t2.is_changed(), t3.map(|t| {t.is_changed()})));
            let r = set_gylph(entity,  text_style, text_outer_glow, node_state, &mut font_sheet, text_overflow_data);
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
        // let t1 = pi_time::Instant::now();
        // println!("t2======================={:?}", (ii1.len(), ii1));

        if is_reset {
            log::warn!("reset=======================");
            await_set_gylph.clear();
            // 为当前所有需要显示的字符，重新分配字形信息
            for (
                entity,
                // world_matrix,
                text_style,
                text_outer_glow,
                // text_content,
                node_state,
                // layer,
                text_overflow_data,
            ) in query.p1().iter_mut()
            {
                let r = set_gylph(entity, text_style, text_outer_glow, node_state, &mut font_sheet, text_overflow_data).unwrap();
                if r == false {
                    await_set_gylph.push(entity);
                }
            }
        }
    }
    // let t2 = pi_time::Instant::now();
    // let l: usize = await_set_gylph.len();
	// let font_type = font_sheet.font_mgr().font_type;

	// // 如果是sdf2， 则设置就绪字形对应节点的NodeState的修改版本
	// if let FontType::Sdf2 = font_type {
	// 	if await_set_gylph.len() > 0 {
            
    //         let index = *await_index;
    //         let result = Share::new(ShareMutex::new((0, Vec::new())));
    //         let load_mark = Share::new(AtomicBool::new(false));
    //         // println!("await_set_gylph1================{:?}", (index, &await_set_gylph, r));
    //         await_list.0.push_back((await_set_gylph, load_mark.clone(), result.clone()));
	// 		let cur_await = font_sheet.draw_await( result, index);
    //         // let i = list.len();
    //         // let await_count1 = (*await_count).clone();
	// 		MULTI_MEDIA_RUNTIME.spawn(async move {
    //             // println!("await_set_gylph start================{:?}", index);
	// 		    cur_await.await;
    //             load_mark.store(true, std::sync::atomic::Ordering::Relaxed);
    //             // await_count1.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    //             // println!("await_set_gylph end================{:?}", index);
	// 		}).unwrap();
    //         *await_index += 1;
	// 	}

	// 	let p2 = query.p2();

    //     let mut next = await_list.0.front();
    //     loop {
    //         if let Some((await_set_gylph, is_load, _result)) = next {
    //             // println!("await================{:?}", &await_set_gylph);
    //             if is_load.load(std::sync::atomic::Ordering::Relaxed) == true {
    //                 let (await_set_gylph, _, result) = await_list.0.pop_front().unwrap();
    //                 font_sheet.update_sdf2(result); // 更新纹理
                    
    //                 if await_set_gylph.len() > 0 {
    //                     for entity in await_set_gylph.iter() {
    //                         // println!("set_changed================{:?}", entity);
    //                         if let Ok(mut node_state) = p2.get_mut(*entity) {
    //                             node_state.set_changed();
    //                             log::debug!("node_state============{:?}", entity);
    //                             // println!("set_changed================{:?}", entity);
    //                         }
    //                     }
    //                     global_mark.mark.set(OtherDirtyType::NodeState as usize, true);
    //                 }
                    
    //                 next = await_list.0.front();
    //                 continue;
    //             }
    //         }
    //         break;
    //     }
    //     // for (await_set_gylph, _, result) in await_list.0.drain(..) {
    //     //     font_sheet.update_sdf2(result); // 更新纹理
    //     //     for entity in await_set_gylph.iter() {
    //     //         if let Ok(mut node_state) = p2.get_mut(*entity) {
    //     //             node_state.set_changed();
    //     //             // println!("set_changed================{:?}", entity);
    //     //         }
    //     //     }
    //     //     // println!("await_set_gylph1================{:?}", await_set_gylph);
    //     // }
	// } else {
	// 	font_sheet.update()
	// }
    // let t3 = pi_time::Instant::now();
    // if is_change || l > 0 {
    //     println!("set_gylph======================={:?}", ( t1.duration_since(t0) , t2.duration_since(t1), t3.duration_since(t2),));
    // }
}

// #[derive(Debug, Clone, Serialize, Deserialize, Default, Deref)]
// pub struct TextShadow(pub TextShadowList);

// #[derive(Debug, Clone, Serialize, Deserialize, Default, Deref)]
// pub struct TextOuterGlow(pub OuterGlow);
fn set_gylph(
    entity: Entity,
	// layer: &Layer,
    // world_matrix: &WorldMatrix,
    text_style: &TextStyle,
    text_outer_glow: Option<&TextOuterGlow>,
    // text_content: &TextContent,
    mut node_state: Mut<NodeState>,
    font_sheet: &mut FontSheet,
	text_overflow_data: Option<Mut<'_, TextOverflowData>>
) -> Result<bool, ()> { // 返回字形是否已经准备就绪
	// if layer.layer().is_null() {
	// 	return Ok(true);
	// }
    // let scale = Vector4::from(world_matrix.fixed_columns(1));
    // let scale = scale.dot(&scale).sqrt();
    // // log::warn!("set_gylph============={:?}, {:?}, {:?}", entity, text_content, scale);
    // if scale < 0.000001 {
    //     node_state.0.scale = scale;
    //     return Ok(true);
    // }

	let is_ready = true;
    // let font_size = (get_size(&text_style.font_size) as f32 * scale).round() as usize;
    let font_size = get_size(&text_style.font_size);
    let font_id = font_sheet.font_id(Font::new(
        text_style.font_family.clone(),
        font_size,
        text_style.font_weight,
        //  unsafe { NotNan::new_unchecked((*text_style.text_stroke.width as f32).round())},
        //  None,
        //  None,
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
            // log::debug!("sdf2 texture is ready =============={:?} ,{:?},{:?}", &text_style.font_family, char_node.ch, &glyph_id);
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

            // if let Some(text_shadow) = text_shadow {
            //     for item in text_shadow.iter() {
            //         println!("add_font_shadow1===============: {:?}", (font_id, char_node.ch));
            //         font_sheet.font_mgr_mut().add_font_shadow(font_id, char_id, item.blur as u32, NotNan::new(text_style.font_weight()).unwrap());
            //     }
            // }

            if let Some(text_outer_glow) = text_outer_glow {
                font_sheet.font_mgr_mut().add_font_outer_glow(font_id, char_id, text_outer_glow.distance as u32);
            }

			// // 如果是sdf2渲染，需要修改准备状态
			// if let FontType::Sdf2 = font_type {

			// 	log::trace!("sdf2 texture is ready =============={:?},{:?}, {:?}", char_node.ch, char_id, font_sheet.font_mgr().table.sdf2_table.glyph(char_id).advance);
			// 	if font_sheet.font_mgr().table.sdf2_table.glyph(char_id).advance == 0.0 {
			// 		is_ready = false;
			// 	}
			// }
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

            // if let Some(text_shadow) = text_shadow {
            //     for item in text_shadow.iter() {
            //         font_sheet.font_mgr_mut().add_font_shadow(font_id, char_id, item.blur as u32, NotNan::new(text_style.font_weight()).unwrap());
            //     }
            // }

            if let Some(text_outer_glow) = text_outer_glow {
                font_sheet.font_mgr_mut().add_font_outer_glow(font_id, char_id, text_outer_glow.distance as u32);
            }
			// if let FontType::Sdf2 = font_type {
			// 	log::trace!("sdf2 texture is ready=============={:?},{:?}, {:?}", char_node.ch, char_id, font_sheet.font_mgr().table.sdf2_table.glyph(char_id).advance);
			// 	if font_sheet.font_mgr().table.sdf2_table.glyph(char_id).advance == 0.0 {
			// 		is_ready = false;
			// 	}
			// }
		}
	}
    Ok(is_ready)
}
