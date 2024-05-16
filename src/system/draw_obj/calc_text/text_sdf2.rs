//! sdf2文字功能
use pi_world::prelude::{Changed, With, Query, SingleResMut, Entity, Plugin, OrDefault, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes, Up, Layer};
use pi_bevy_render_plugin::FrameDataPrepare;
use pi_hal::font::font::FontType;
use pi_hal::font::sdf2_table::TexInfo;
use pi_hal::pi_sdf::glyphy::geometry::aabb::AabbEXT;
use pi_render::font::{FontSheet, GlyphId, Font};
use pi_style::style::{TextOverflow, Aabb2, FontStyle};

use crate::components::calc::{LayoutResult, NodeState};
use crate::components::draw_obj::{TextMark, RenderCount};
use crate::components::user::{get_size, Point2, TextContent, TextOuterGlow, TextOverflowData, TextShadow, TextStyle};
use crate::components::user::Color;
use crate::resource::{NodeChanged, ShareFontSheet, TextRenderObjType};
use crate::shader1::{InstanceData, GpuBuffer};
use crate::system::draw_obj::life_drawobj::{draw_object_life_new, update_render_instance_data};
use crate::system::draw_obj::sdf2_texture_update::update_sdf2_texture;
use crate::system::draw_obj::set_box;
use crate::prelude::UiStage;
use crate::system::node::layout::calc_layout;

use super::text_glyph::text_glyph;
use super::{IsRun, TEXT_ORDER};
use super::text_split::{get_line_height, text_split};

use crate::components::calc::{WorldMatrix, DrawList};
use crate::components::draw_obj::InstanceIndex;
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{ColorUniform, GradientEndUniform, GradientPositionUniform, RenderFlagType, Sdf2InfoUniform, ShadowUniform, TextGradientColorUniform, TextOuterGlowUniform, TextOutlineUniform, TextShadowColorUniform, TextWeightUniform, TyUniform};
use crate::components::user::Vector2;
use crate::system::system_set::UiSystemSet;

/// 使用sdf2的方式渲染文字
pub struct Sdf2TextPlugin;

impl Plugin for Sdf2TextPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		let font_sheet = ShareFontSheet::new(&mut app.world, FontType::Sdf2);
		app.world.insert_single_res(font_sheet);
        app
			// .insert_single_res(font_sheet)
            // .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            // .add_frame_event::<ComponentEvent<Changed<TextContent>>>()
            // 文字劈分为字符
            .add_system(UiStage, text_split
				.before(calc_layout)
				.in_set(UiSystemSet::Layout))
            // 字形计算
            .add_system(UiStage, text_glyph
				.after(text_split)
				.in_set(UiSystemSet::Layout)
				.before(update_sdf2_texture)
			)
			// 创建drawobj 
			.add_system(
				UiStage, 
				draw_object_life_new::<
						TextContent,
						TextRenderObjType,
						(TextMark, RenderCount),
						{ TEXT_ORDER }>
						.in_set(UiSystemSet::LifeDrawObject)
						.before(calc_sdf2_text_len),
			)
			// 统计drawobj的实例长度（文字包含多个字符，每个字符一个实例， 并且可能包含多层阴影， 每阴影每字符也需要一个实例）
			// 由于当前一个文字实例可附带渲染一个阴影，因此最终的实例个数为`text.len() * (shadow.len() > 1? shadow.len() - 1: 1)`个实例
			.add_system(
				UiStage, 
				calc_sdf2_text_len
					
					.after(UiSystemSet::LifeDrawObjectFlush)
					.before(update_render_instance_data)
					.after(calc_layout)
			)
			// 更新实例数据
			.add_system(
				UiStage, 
				calc_sdf2_text
					.in_set(UiSystemSet::PrepareDrawObj)
			)
		;
    }
}

/// 共计sdf文字的的实例数量
pub fn calc_sdf2_text_len(
    query: Query<(
		Entity, 
		&NodeState, 
		Option<&TextOverflowData>, 
		&DrawList, 
		&LayoutResult,
		Option<&TextShadow>, 
		OrDefault<TextStyle>,
	), (
		(Changed<NodeState>, 
		Changed<TextOverflowData>, 
		Changed<LayoutResult>, 
		Changed<TextStyle>,
		Changed<TextShadow>), 
		With<TextContent>,
	)>,
    mut query_draw: Query<&mut RenderCount, With<TextMark>>,
	query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	r: OrInitSingleRes<IsRun>,
	mut node_changed: OrInitSingleResMut<NodeChanged>,
	render_type: OrInitSingleRes<TextRenderObjType>,
) {
	if r.0 {
		return;
	}

	for (
		mut entity,
		node_state, 
		text_overflow_data,
		draw_list,
		mut layout,
		text_shadow,
		text_style,
	) in query.iter() {
		
		let render_type = ***render_type;
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		if let Ok(mut render_count) = query_draw.get_mut(draw_id) {
			let mut new_count = 0;
			if node_state.0.text.len() > 0 {
				
				let text_overflow = calc_text_overflow_data(text_overflow_data, text_style);

				let mut line_max = 0.0;
				if text_overflow.0 && node_state.is_vnode() {
					while let Ok((p_layout, up, p_node_state)) = query_up.get(entity) {
						if !p_node_state.is_vnode() {
							line_max = p_layout.rect.right - p_layout.border.right - p_layout.padding.right - p_layout.border.left - p_layout.padding.left - p_layout.rect.left ;
							layout = p_layout;
							break;
						} else {
							entity = up.parent();
						}
					}
				} else {
					line_max = layout.rect.right - layout.border.right - layout.padding.right - layout.border.left - layout.padding.left - layout.rect.left ;
				}

				let offset = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
				let mut word_pos = offset.clone();
				let mut count = 0;

				// 文字是否存在换行（如果存在换行， text_overflow无效）
				let start_y = node_state.0.text[0].pos.top + offset.1; 


				for c in node_state.0.text.iter() {
					if c.ch == char::from(0) {
						if c.count > 0 {
							word_pos = (c.pos.left + offset.0, c.pos.top + offset.1);
							count = c.count - 1;
						}
						continue;
					}
					if c.ch <= ' ' {
						continue;
					}

					let mut left = word_pos.0 + c.pos.left;
					let w = c.pos.right - c.pos.left;

					if  text_overflow.0 && c.pos.top == start_y && left + w + text_overflow.1 > line_max {
						if let Some(text_overflow_data) = text_overflow_data {
							let mut max = 1;
							if let TextOverflow::Ellipsis = text_overflow_data.text_overflow {
								max = 3;
							}
							let mut i = 0;
							while i < max {
								for c1 in text_overflow_data.text_overflow_char.iter() {
									left += c1.width + text_style.letter_spacing;
									new_count += 1;
								}
								i += 1;
							}
						}
						break;
					}

					new_count += 1;
					if count > 0 {
						count -= 1;
						if count == 0 {
							word_pos = offset;
						}
					}
				}
			}
			let shadow_factor = match text_shadow {
				Some(r) if r.0.len() > 0 => r.0.len(),
				_ => 1,
			};
			new_count = new_count * shadow_factor;

			let c = render_count.bypass_change_detection();
			if c.0 != new_count as u32 {
				c.0 = new_count as u32;
				render_count.set_changed();
				node_changed.0 = true;
			}
			
		}

		
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_text(
	// sdf2_texture_version
	mut instances: OrInitSingleResMut<InstanceContext>,
    query: Query<(
		Entity, 
		&WorldMatrix, 
		&NodeState, 
		Option<&TextOverflowData>, 
		Option<&TextStyle>, 
		&LayoutResult, 
		&DrawList, 
		&Layer, 
		Option<&TextShadow>, 
		Option<&TextOuterGlow>, 
	), (
		(Changed<TextStyle>, Changed<NodeState>, Changed<WorldMatrix>), With<TextContent>)>,
    mut query_draw: Query<(&InstanceIndex, &RenderCount), With<TextMark>>,
	query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<TextRenderObjType>,
	font_sheet: SingleResMut<ShareFontSheet>,
	query_parent: Query<&Up>,
	query_matrix: Query<(&WorldMatrix, &NodeState, &LayoutResult)>,
) {
	if r.0 {
		return;
	}
	let render_type = ***render_type;
	let default_text_style = TextStyle::default();
	let mut font_sheet = font_sheet.borrow_mut();
	
	for (
		entity,
		world_matrix, 
		node_state, 
		text_overflow_data, 
		text_style, 
		layout, 
		draw_list, 
		layer,
		text_shadow,
		text_outer_glow,) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		log::trace!("calc_sdf2_text1;");
		

		if let Ok((instance_index, render_count)) = query_draw.get_mut(draw_id) {
			// 节点可能设置为dispaly none， 此时instance_index可能为Null
			if pi_null::Null::is_null(&instance_index.0.start) {
				continue;
			}

			if  node_state.0.text.len() == 0 {
				continue;
			}
			// if node_state.0.scale < 0.000001 {
            //     continue;
            // }
			if layer.layer() == 0 {
				continue;
			}
			log::trace!("calc_sdf2_text3;");

			let mut n = entity;
			let mut state = &*node_state;
			let mut matrix = &*world_matrix;
			let mut layout1 = &*layout;
			while state.is_vnode() {
				// 虚拟节点，现阶段只有图文混排的文字节点，直接使用父节点的世界矩阵
				if let Ok(up) = query_parent.get(n) {
					if let Ok((m, s, l)) = query_matrix.get(up.parent()) {
						if s.is_vnode() {
							n = up.parent();
							continue;
						}
						matrix = m;
						state = s;
						layout1 = l;
					}
				
				}
			}

			// let is_added = node_state.is_changed();

			let text_style = match text_style {
				Some(r) => r,
				None => &default_text_style, // TextStyle组件在设计上不会被删除， 当TextStyle为None时， TextStyle一定没有改变过
			};

			let font_size = get_size(&text_style.font_size) as f32;
			let font_id = font_sheet.font_id(Font::new(
				text_style.font_family.clone(),
				font_size as usize,
				text_style.font_weight,
				text_style.text_stroke.width, // todo 或许应该设置比例
			));
			// let mut instance_data = instances.bypass_change_detection().instance_data.instance_data_mut(instance_index.0);
			// 修改vert buffer
			// let set_color = set_color_fn(text_style_change, &mut render_flag, text_style, &layout);
			let instance_data = instance_data(
				// text_style_change, 
				// is_added,
				// world_matrix.is_changed(),
				text_style,
				&layout,
				&matrix,
				text_shadow,
				text_outer_glow,
			);

			set_chars_data(&node_state,
				layout1,
				&mut font_sheet,
				text_style, 
				text_overflow_data,
				&query_up,
				entity,
				draw_id,
				instance_data,
				instance_index.clone(),
				&mut instances.instance_data,
				font_id, 
				render_count);
			


			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			// let is_add =  node_state.is_added();
			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
			// if is_add || layout.is_changed() {
			// 	instance_data.set_data(&BoxUniform(layout.padding_box().as_slice()));
			// }
		}
	}
}

#[inline]
fn instance_data<'a>(
	// is_style_change: bool, 
	// is_content_change: bool, 
	// is_matrix_change: bool, 
	text_style: &'a TextStyle, 
	layout: &'a LayoutResult,
	world_matrix: &'a WorldMatrix,
	text_shadow: Option<&'a TextShadow>,
	text_outer_glow: Option<&'a TextOuterGlow>,
) -> UniformData<'a> {
	let stroke = if *text_style.text_stroke.width > 0.0 {
        [text_style.text_stroke.color.x, text_style.text_stroke.color.y, text_style.text_stroke.color.z, *text_style.text_stroke.width]
    } else {
        [0.0, 0.0, 0.0, *text_style.text_stroke.width]
    };
	let weight = if text_style.font_weight == 500 {
		[-0.0]
    } else if  text_style.font_weight < 500 {
        [-1.0]
    } else {
		[1.0]
	};

	match &text_style.color {
		// 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
		Color::RGBA(color) => {
			UniformData {
				stroke,
				weight,
				// is_style_change,
				// is_content_change,
				// is_matrix_change,
				font_style: text_style.font_style,
				color: ColorData::Rgba([color.x, color.y, color.z, color.w]),
				world_matrix,
				text_shadow,
				text_outer_glow
			}
		},
		// 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
		Color::LinearGradient(color) => {
			// TODO， 渐变端点
			let mut colors: [f32; 12] = [0.0; 12];
			let mut positions: [f32; 4] = [1.0; 4];
			if color.list.len() > 0 {
				for i in 0..4 {
					match color.list.get(i) {
						Some(r) => {
							positions[i] = r.position;
							let j = i * 3;
							colors[j] = r.rgba.x;
							colors[j + 1] = r.rgba.y;
							colors[j + 2] = r.rgba.z;
							// colors[j + 3] = r.rgba.w;
						},
						None => {
							positions[i] = 1.0;
							let j = i * 3;
							colors[j] = colors[j - 4];
							colors[j + 1] = colors[j - 3];
							colors[j + 2] = colors[j - 2];
							// colors[j + 3] = colors[j - 1];
						},
					}
				}
			}
			log::trace!("sdf2 LinearGradient======{:?}, {:?}", color, positions);
			let normalize_direction = Vector2::new(color.direction.cos(), color.direction.sin());
			let r = [
				Vector2::new(layout.border.left, layout.border.top).dot(&normalize_direction), 
				Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.border.top).dot(&normalize_direction),
				Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.rect.bottom - layout.border.bottom - layout.rect.top).dot(&normalize_direction),
				Vector2::new(layout.border.left, layout.rect.bottom - layout.border.bottom - layout.rect.top).dot(&normalize_direction),
			];
			let (min, max) = (r[0].min(r[1]).min(r[2]).min(r[3]), r[0].max(r[1]).max(r[2]).max(r[3]));
			let end = (normalize_direction * min, normalize_direction * max);
			let end = [end.0.x, end.0.y, end.1.x, end.1.y];

			log::trace!("sdf2 LinearGradient======{:?}, {:?}, {:?}, {:?}, {:?}, {:?}", normalize_direction, r, min, max, end, [
					Vector2::new(layout.border.left, layout.border.top), 
					Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.border.top),
					Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.rect.bottom - layout.border.bottom - layout.rect.top),
					Vector2::new(layout.border.left, layout.rect.bottom - layout.border.bottom - layout.rect.top),
				]);
			


			UniformData {
				stroke,
				weight,
				// is_style_change,
				// is_content_change,
				// is_matrix_change,
				font_style: text_style.font_style,
				color: ColorData::LinearGradient {
					colors,
					positions,
					end,
				},
				world_matrix,
				text_shadow,
				text_outer_glow,
			}
		},
	}
}

#[allow(unused_variables)]
fn set_chars_data(
    node_state: &NodeState,
    layout: &LayoutResult,
    font_sheet: &mut FontSheet,
    text_style: &TextStyle, 
	text_overflow_data: Option<&TextOverflowData>,
	query_layout: &Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	mut entity: Entity,
	draw_id: Entity,
	uniform_data: UniformData,
	instance_index: InstanceIndex,
	instances: &mut GpuBuffer,
	font_id: pi_hal::font::font::FontId,
	render_count: &RenderCount,
) {

	let font_size = get_size(&text_style.font_size) as f32;
	let line_height = get_line_height(font_size as usize, &text_style.line_height);
	

	let font_type = font_sheet.font_mgr().font_type;
    let offset = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
	let mut word_pos = offset.clone();
    let mut count = 0;
    let half_stroke = *text_style.text_stroke.width / 2.0;

	// 文字是否存在换行（如果存在换行， text_overflow无效）
	let start_y = node_state.0.text[0].pos.top + offset.1; 
	let text_overflow = calc_text_overflow_data(text_overflow_data, text_style);

	let mut line_max = 0.0;
	if text_overflow.0 {
		while let Ok((p_layout, up, p_node_state)) = query_layout.get(entity) {
			if !p_node_state.is_vnode() {
				line_max = p_layout.rect.right - p_layout.border.right - p_layout.padding.right - p_layout.border.left - p_layout.padding.left - p_layout.rect.left ;
				break;
			} else {
				entity = up.parent();
			}
		}
	}

	// let render_range = font_sheet.font_mgr().table.sdf2_table.fonts[];
	let fontface_ids = &font_sheet.font_mgr().sheet.fonts[font_id.0].font_ids;

	let mut cur_instance_index = instance_index.0.start;
	let default_range = Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0));

    for c in node_state.0.text.iter() {
        if c.ch == char::from(0) {
            if c.count > 0 {
                word_pos = (c.pos.left + offset.0, c.pos.top + offset.1);
				// log::warn!("pos==================={:?}", c.pos);
                count = c.count - 1;
            }
            continue;
        }
        if c.ch <= ' ' {
            continue;
        }

		let mut left = word_pos.0 + c.pos.left;
		let w = c.pos.right - c.pos.left;
		let top = word_pos.1 + c.pos.top;
		let shadow_factor = match &uniform_data.text_shadow {
			Some(r) if r.0.len() > 0 => r.0.len(),
			_ => 1,
		};

		if  text_overflow.0 && c.pos.top == start_y && left + w + text_overflow.1 > line_max {
			if let Some(text_overflow_data) = text_overflow_data {
				let mut max = 1;
				if let TextOverflow::Ellipsis = text_overflow_data.text_overflow {
					max = 3;
				}
				let mut i = 0;
				while i < max {
					for c1 in text_overflow_data.text_overflow_char.iter() {
						if cur_instance_index > instance_index.0.end {
							panic!("text len error, cur_instance_index: {}, instance_index end: {}, entity: {:?}, len: {}, node_state: {:?}, render_count: {:?}", cur_instance_index, instance_index.0.end, entity, node_state.0.text.len(), &node_state, render_count);
						}
						let glyph = match font_type {
							FontType::Bitmap => todo!(),
							FontType::Sdf1 => todo!(),
							FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.glyph(GlyphId(c1.ch_id)),
						};

						let face_id = fontface_ids[font_sheet.font_mgr().table.sdf2_table.glyphs[c1.ch_id].font_face_index];
						let render_range = match (font_sheet.font_mgr().table.sdf2_table.fonts.get(face_id.0), glyph.grid_w > 0.0 ) {
							(Some(r), true) => r.max_box_normaliz(),
							_ => &default_range,
						};
											// let offset_y = (line_height - font_height) / 2.0;
						for i in 0..shadow_factor {
							uniform_data.set_data(instances.instance_data_mut(cur_instance_index), glyph, render_range, (left + text_style.letter_spacing, top + (line_height - (render_range.maxs.y - render_range.mins.y) * font_size) / 2.0), font_size, shadow_factor - i - 1);
							cur_instance_index = instances.next_index(cur_instance_index);
						}
						
						left += c1.width + text_style.letter_spacing;
					}
					i += 1;
				}
			}
			break;
		}

		if cur_instance_index > instance_index.0.end {
			panic!("text len error, cur_instance_index: {}, instance_index: {:?}, entity: {:?}, len: {}, node_state: {:?}, render_count: {:?}", cur_instance_index, &instance_index.0, entity, node_state.0.text.len(), &node_state, render_count);
		}
        // log::warn!("glyph!!!==================={:?}, {:?}, {left:?}, {top:?}", c.ch_id, c.ch);
		let glyph = match font_type {
			FontType::Bitmap => todo!(),
			FontType::Sdf1 => todo!(),
			FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.glyph(GlyphId(c.ch_id)),
		};
		let face_id = fontface_ids[font_sheet.font_mgr().table.sdf2_table.glyphs[c.ch_id].font_face_index];
		let render_range = match (font_sheet.font_mgr().table.sdf2_table.fonts.get(face_id.0), glyph.grid_w > 0.0 ){
			(Some(r), true) => r.max_box_normaliz(),
			_ => &default_range,
		};

		// if font_sheet.font_mgr().table.sdf2_table.fonts.get(face_id.0).is_none() {
		// 	log::warn!("default_range============{}, {:?}, {:?}, {:?}, {:?}", font_sheet.font_mgr().table.sdf2_table.glyphs[c.ch_id].font_face_index, c.ch, fontface_ids, font_sheet.font_mgr().sheet.fonts[font_id.0].font_family_id,&font_sheet.font_mgr().sheet.font_familys[font_sheet.font_mgr().sheet.fonts[font_id.0].font_family_id.0]);
		// }
		for i in 0..shadow_factor {
			uniform_data.set_data(instances.instance_data_mut(cur_instance_index), glyph, render_range, (left, top + (line_height - (render_range.maxs.y - render_range.mins.y) * font_size) / 2.0), font_size, shadow_factor - i - 1);
			cur_instance_index = instances.next_index(cur_instance_index);
		}

		if count > 0 {
			count -= 1;
			if count == 0 {
				word_pos = offset;
			}
		}
    }
}


struct UniformData<'a> {
	stroke: [f32; 4],
	weight: [f32; 1],
	// is_style_change: bool,
	// is_content_change: bool,
	// is_matrix_change: bool,
	font_style: FontStyle,
	color: ColorData,
	world_matrix: &'a WorldMatrix,
	text_shadow: Option<&'a TextShadow>,
	text_outer_glow: Option<&'a TextOuterGlow>,
}

enum ColorData {
	Rgba([f32; 4]),
	LinearGradient {
		colors: [f32; 12],
		positions: [f32; 4],
		end: [f32; 4],
	},
}

impl<'a> UniformData<'a> {
	#[inline]
	fn set_data(&self, mut instance_data: InstanceData, tex_info: &TexInfo, render_range: &Aabb2, mut offset: (f32, f32), font_size: f32, shadow_index: usize) {
		log::trace!("set_data===================={:?}, {:?}, offset={:?}, font_size={}, shadow_index={}", instance_data, tex_info, offset, font_size, shadow_index);
		let mut render_flag = instance_data.get_render_ty();
		render_flag |= 1 << RenderFlagType::Sdf2 as usize;

		let mut width = render_range.width() * font_size;
		let mut height = render_range.height() * font_size;

		// 设置阴影
		if let Some(shadows) = self.text_shadow {
			if let Some(shadow) = shadows.0.get(shadow_index) {
				let (h, v) = (shadow.h/width, shadow.v/height);
                if h > 0.0 {
					width += shadow.h;
                    // render_range.maxs.x = render_range.maxs.x + h;
                }else{
					offset.0 += shadow.h;
					width -= shadow.h;
                    // render_range.mins.x = render_range.mins.x + shadow.h/width;
                }

                if v > 0.0{
					height += shadow.v;
                    // render_range.maxs.y = render_range.maxs.y + v;
                }else{
					offset.1 += shadow.v;
					height -= shadow.v;
                    // render_range.mins.y = render_range.mins.y + v;
                }
				
				// y轴坐标反了
				let shadow_data = [shadow.h / width, -shadow.v / height, shadow.blur];
				log::trace!("set shadow instance data: {:?}, {:?}, {:?}, {:?}", shadow_data, shadow, render_range, (width, height));
				instance_data.set_data(&ShadowUniform(&shadow_data));
				instance_data.set_data(&TextShadowColorUniform(&shadow.color.as_slice()));
				render_flag |=  1<< RenderFlagType::Sdf2Shadow as usize;
				render_flag &= !(1 << RenderFlagType::Sdf2OutGlow as usize);
			} else {
				render_flag &= !(1 << RenderFlagType::Sdf2Shadow as usize);
			}
		} else {
			render_flag &= !(1 << RenderFlagType::Sdf2Shadow as usize);
			if let Some(text_outer_glow) = self.text_outer_glow {
				if text_outer_glow.distance > 0.0 {
					// width += text_outer_glow.distance * 2.0;
					// height += text_outer_glow.distance * 2.0;
					// offset.1 -= text_outer_glow.distance;
					// offset.0 -= text_outer_glow.distance;
					render_flag |=  1<< RenderFlagType::Sdf2OutGlow as usize;
			    	instance_data.set_data(&TextOuterGlowUniform(&[text_outer_glow.color.x, text_outer_glow.color.y, text_outer_glow.color.z, text_outer_glow.distance]));
				} else {
					render_flag &= !(1 << RenderFlagType::Sdf2OutGlow as usize);
				}
			}
		}

		// 设置位置、大小、是否为斜体
		// if self.is_style_change || self.is_content_change || self.is_matrix_change {
			let (mut scope_factor, mut scope_y ) = (0.0, 0.0);
			if self.font_style == FontStyle::Oblique {
				scope_y = -render_range.mins.y * font_size; // 基线位置的y
				scope_factor = 0.35;
			}
			
			// sdf信息[max_offset, min_sdf, sdf_step, check, index_offset_x, index_offset_y, index_w, index_h, data_offset_x, data_offset_y, scope_factor, scope_y]
			let data = [
				tex_info.max_offset as f32, tex_info.min_sdf as f32, tex_info.sdf_step as f32, tex_info.cell_size * 0.5 * 2.0f32.sqrt(),
				tex_info.index_offset.0 as f32, tex_info.index_offset.1 as f32, tex_info.grid_w, tex_info.grid_w,
				tex_info.data_offset.0 as f32, tex_info.data_offset.1 as f32,
				scope_factor, scope_y,

			];
			instance_data.set_data(&Sdf2InfoUniform(&data));

			// 设置文字在布局空间的偏移和宽高
			// instance_data.set_data(&BoxUniform(&[offset.0, offset.1, (render_range.maxs.x - render_range.mins.x) * font_size, (render_range.maxs.y - render_range.mins.y) * font_size]));
			// println!("self.world_matrix: {:?}", self.world_matrix);
			set_box(&self.world_matrix, &Aabb2::new(Point2::new(offset.0, offset.1), Point2::new(width + offset.0, height + offset.1)), &mut instance_data);
		// }

		if shadow_index == 0 {

			// if self.is_style_change {
				let scale = self.world_matrix.0[0];
				let weight = self.weight[0] * scale;
				let stroke = [self.stroke[0], self.stroke[1], self.stroke[2], self.stroke[3] * scale];
				instance_data.set_data(&TextOutlineUniform(&stroke));
				instance_data.set_data(&TextWeightUniform(&[weight]));
				match &self.color {
					ColorData::Rgba(r) => {
						render_flag |= 1 << RenderFlagType::Color as usize;
						render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
						instance_data.set_data(&ColorUniform(r))
					},
					ColorData::LinearGradient { colors, positions, end } => {
						render_flag |= 1 << RenderFlagType::LinearGradient as usize;
						render_flag &= !(1 << RenderFlagType::Color as usize);
						instance_data.set_data(&TextGradientColorUniform(colors));
						instance_data.set_data(&GradientPositionUniform(positions));
						instance_data.set_data(&GradientEndUniform(end));
					},
				}
			// }
		} else {
			let scale = self.world_matrix.0[0];
			let weight = self.weight[0] * scale;
			render_flag |= 1 << RenderFlagType::Color as usize;
			render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
			instance_data.set_data(&TextOutlineUniform(&[0.0, 0.0, 0.0, 0.0]));
			instance_data.set_data(&TextWeightUniform(&[weight]));
			instance_data.set_data(&ColorUniform(&[0.0, 0.0, 0.0, 0.0]))
		}
		// 设置渲染类型
		instance_data.set_data(&TyUniform(&[render_flag as f32]));
	}
}

#[inline]
fn calc_text_overflow_data(text_overflow_data: Option<&TextOverflowData>, text_style: &TextStyle) -> (bool, f32) {
	if let Some(text_overflow_data) = text_overflow_data {
		let width = match &text_overflow_data.text_overflow {
			pi_style::style::TextOverflow::None => return (false, 0.0),
			pi_style::style::TextOverflow::Clip => return (true, 0.0),
			pi_style::style::TextOverflow::Ellipsis => text_overflow_data.text_overflow_char[0].width * 3.0 + text_style.letter_spacing * 3.0,
			pi_style::style::TextOverflow::Custom(_) => {
				let mut width = text_style.letter_spacing * text_overflow_data.text_overflow_char.len() as f32;
				for c in text_overflow_data.text_overflow_char.iter() {
					width += c.width;
				}
				width
			},
		};
		return (true, width)
	}
	(false, 0.0)
}
