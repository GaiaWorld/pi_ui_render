
use bevy_app::Plugin;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::{DetectChanges, Ref};
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::system::{Query, ResMut};
use bevy_ecs::prelude::DetectChangesMut;

use pi_bevy_ecs_extend::query::or_default::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_ecs_extend::system_param::tree::{Up, Layer};
use pi_bevy_render_plugin::FrameDataPrepare;
use pi_hal::font::font::FontType;
use pi_hal::font::sdf2_table::TexInfo;
use pi_render::font::{FontSheet, GlyphId, Font};
use pi_style::style::{TextOverflow, Aabb2, FontStyle};

use crate::components::calc::{LayoutResult, NodeState};
use crate::components::draw_obj::{TextMark, RenderCount};
use crate::components::user::{TextStyle, get_size, TextOverflowData, TextContent, Point2};
use crate::components::user::Color;
use crate::events::EntityChange;
use crate::resource::{ShareFontSheet, TextRenderObjType};
use crate::shader1::{InstanceData, GpuBuffer};
// use crate::shader::text;
// use crate::shader::text_sdf;

// use crate::shader1::ui_meterial::{ColorUniform, StrokeColorOrURectUniform, TextureSizeOrBottomLeftBorderUniform, ClipSdfOrSdflineUniform, DataTexSizeUniform, UGradientStarteEndUniform, ScaleUniform};
use crate::system::draw_obj::life_drawobj::{draw_object_life_new, update_render_instance_data};
use crate::system::draw_obj::sdf2_texture_update::update_sdf2_texture;
use crate::system::draw_obj::set_box;
use crate::system::node::layout::calc_layout;
use crate::prelude::UiSchedule;

use super::text_glyph::text_glyph;
use super::{IsRun, TEXT_ORDER};
use super::text_split::{get_line_height, text_split};

use bevy_ecs::schedule::IntoSystemConfigs;

use bevy_window::AddFrameEvent;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitResMut;

use crate::components::calc::{WorldMatrix, DrawList};
use crate::components::draw_obj::InstanceIndex;
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{TextGradientColorUniform, ColorUniform, GradientPositionUniform, RenderFlagType, GradientEndUniform, TextOutlineUniform, TextWeightUniform, Sdf2InfoUniform, TyUniform};
use crate::components::user::Vector2;
use crate::system::system_set::UiSystemSet;

/// 使用sdf2的方式渲染文字
pub struct Sdf2TextPlugin;

impl Plugin for Sdf2TextPlugin {
    fn build(&self, app: &mut bevy_app::App) {
		let font_sheet = ShareFontSheet::new(&mut app.world, FontType::Sdf2);
        app
			.insert_resource(font_sheet)
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_frame_event::<ComponentEvent<Changed<TextContent>>>()
            // 文字劈分
            .add_systems(UiSchedule, text_split.before(calc_layout).in_set(UiSystemSet::Layout))
            // 字形计算
            .add_systems(UiSchedule, text_glyph.after(text_split).in_set(UiSystemSet::Layout).before(update_sdf2_texture))
			// 创建drawobj 
			.add_systems(
				UiSchedule, 
				draw_object_life_new::<
						TextContent,
						TextRenderObjType,
						(TextMark, RenderCount),
						{ TEXT_ORDER }>
						.in_set(UiSystemSet::LifeDrawObject)
						.before(calc_sdf2_text_len),
			)
			// 统计drawobj的实例长度
			.add_systems(
				UiSchedule, 
				calc_sdf2_text_len
					.in_set(FrameDataPrepare)
					.after(UiSystemSet::LifeDrawObjectFlush)
					.before(update_render_instance_data)
					.after(calc_layout)
			)
			// 更新实例数据
			.add_systems(
				UiSchedule, 
				calc_sdf2_text
					.in_set(UiSystemSet::PrepareDrawObj)
			)
		;
    }
}

/// 更新sdf2的纹理
// pub fn update_sdf2_texture(
// 	mut instances: OrInitResMut<InstanceContext>, 
// 	font_sheet: ResMut<ShareFontSheet>,
// 	device: Res<PiRenderDevice>,
//     common_sampler: Res<crate::resource::draw_obj::CommonSampler>,
// ) {
// 	let font_sheet = font_sheet.0.borrow();
// 	if let (Some(sdf2_index_texture_view), Some(sdf2_data_texture_view)) = (&font_sheet.sdf2_index_texture_view, &font_sheet.sdf2_data_texture_view) {
// 		if instances.sdf2_texture_group.is_none() {
// 			let group = (***device).create_bind_group(&wgpu::BindGroupDescriptor {
// 				layout: &instances.sdf2_texture_layout,
// 				entries: &[
// 					wgpu::BindGroupEntry {
// 						binding: 0,
// 						resource: wgpu::BindingResource::TextureView(&sdf2_index_texture_view.texture_view),
// 					},
// 					wgpu::BindGroupEntry {
// 						binding: 1,
// 						resource: wgpu::BindingResource::TextureView(&sdf2_data_texture_view.texture_view),
// 					},
// 					wgpu::BindGroupEntry {
// 						binding: 2,
// 						resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
// 					},
// 				],
// 				label: Some("sdf2 texture bind group create"),
// 			});

// 			instances.sdf2_texture_group = Some(Share::new(group));
// 		}
// 	}
// }

/// 共计sdf文字的的实例数量
pub fn calc_sdf2_text_len(
    query: Query<(Entity, Ref<NodeState>, Option<&TextOverflowData>, &DrawList, &LayoutResult, OrDefault<TextStyle>), (Or<(Changed<NodeState>, Changed<TextOverflowData>, Changed<LayoutResult>, Changed<TextStyle>)>, With<TextContent>)>,
    mut query_draw: Query<&mut RenderCount, With<TextMark>>,
	query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	r: OrInitRes<IsRun>,
	render_type: OrInitRes<TextRenderObjType>,
	mut events: EventWriter<EntityChange>, // 有节点创建
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
		text_style) in query.iter() {
		
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

			let c = render_count.bypass_change_detection();
			if c.0 != new_count as u32 {
				c.0 = new_count as u32;
				render_count.set_changed();
				events.send(EntityChange);
			}
		}

		
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_text(
	// sdf2_texture_version
	mut instances: OrInitResMut<InstanceContext>,
    query: Query<(Entity, Ref<WorldMatrix>, Ref<NodeState>, Option<&TextOverflowData>, Option<Ref<TextStyle>>, Ref<LayoutResult>, &DrawList, &Layer), (Or<(Changed<TextStyle>, Changed<NodeState>, Changed<WorldMatrix>)>, With<TextContent>)>,
    mut query_draw: Query<(&InstanceIndex, &RenderCount), With<TextMark>>,
	query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	r: OrInitRes<IsRun>,
	render_type: OrInitRes<TextRenderObjType>,
	font_sheet: ResMut<ShareFontSheet>,
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
		layer) in query.iter() {
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

			let is_added = node_state.is_changed();

			let (text_style_change, text_style) = match text_style {
				Some(r) => (is_added || r.is_changed(), r.into_inner()),
				None => (is_added, &default_text_style), // TextStyle组件在设计上不会被删除， 当TextStyle为None时， TextStyle一定没有改变过
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
				text_style_change, 
				is_added,
				world_matrix.is_changed(),
				text_style,
				&layout,
				matrix.clone(),
			);

			text_vert(&node_state,
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
fn instance_data(
	is_style_change: bool, 
	is_content_change: bool, 
	is_matrix_change: bool, 
	text_style: &TextStyle, 
	layout: &LayoutResult,
	world_matrix: WorldMatrix,
) -> UniformData {
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
				is_style_change,
				is_content_change,
				is_matrix_change,
				font_style: text_style.font_style,
				color: ColorData::Rgba([color.x, color.y, color.z, color.w]),
				world_matrix,
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
				is_style_change,
				is_content_change,
				is_matrix_change,
				font_style: text_style.font_style,
				color: ColorData::LinearGradient {
					colors,
					positions,
					end,
				},
				world_matrix,
			}
		},
	}
}

#[allow(unused_variables)]
fn text_vert(
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
						uniform_data.set_data(instances.instance_data_mut(cur_instance_index), glyph, render_range, (left + text_style.letter_spacing, top + (line_height - (render_range.maxs.y - render_range.mins.y) * font_size) / 2.0), font_size);
						left += c1.width + text_style.letter_spacing;
						cur_instance_index = instances.next_index(cur_instance_index);
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
		log::trace!("set_data1===================={:?}, {:?}, {:?}, face_id={:?}", c.ch_id, c.ch, glyph, face_id, );
		let render_range = match (font_sheet.font_mgr().table.sdf2_table.fonts.get(face_id.0), glyph.grid_w > 0.0 ){
			(Some(r), true) => r.max_box_normaliz(),
			_ => &default_range,
		};

		// if font_sheet.font_mgr().table.sdf2_table.fonts.get(face_id.0).is_none() {
		// 	log::warn!("default_range============{}, {:?}, {:?}, {:?}, {:?}", font_sheet.font_mgr().table.sdf2_table.glyphs[c.ch_id].font_face_index, c.ch, fontface_ids, font_sheet.font_mgr().sheet.fonts[font_id.0].font_family_id,&font_sheet.font_mgr().sheet.font_familys[font_sheet.font_mgr().sheet.fonts[font_id.0].font_family_id.0]);
		// }
		uniform_data.set_data(instances.instance_data_mut(cur_instance_index), glyph, render_range, (left, top + (line_height - (render_range.maxs.y - render_range.mins.y) * font_size) / 2.0), font_size);
		cur_instance_index = instances.next_index(cur_instance_index);
		if count > 0 {
			count -= 1;
			if count == 0 {
				word_pos = offset;
			}
		}
    }
}


struct UniformData {
	stroke: [f32; 4],
	weight: [f32; 1],
	is_style_change: bool,
	is_content_change: bool,
	is_matrix_change: bool,
	font_style: FontStyle,
	color: ColorData,
	world_matrix: WorldMatrix,
}

enum ColorData {
	Rgba([f32; 4]),
	LinearGradient {
		colors: [f32; 12],
		positions: [f32; 4],
		end: [f32; 4],
	},
}

impl UniformData {
	#[inline]
	fn set_data(&self, mut instance_data: InstanceData, tex_info: &TexInfo, render_range: &Aabb2, offset: (f32, f32), font_size: f32) {
		log::trace!("set_data===================={:?}, {:?}, offset={:?}, font_size={}", instance_data, tex_info, offset, font_size);
		let mut render_flag = instance_data.get_render_ty();
		render_flag |= 1 << RenderFlagType::Sdf2 as usize;

		if self.is_style_change {
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
		}
		

		if self.is_style_change || self.is_content_change || self.is_matrix_change {
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
			set_box(&self.world_matrix, &Aabb2::new(Point2::new(offset.0, offset.1), Point2::new((render_range.maxs.x - render_range.mins.x) * font_size + offset.0, (render_range.maxs.y - render_range.mins.y) * font_size + offset.1)), &mut instance_data);

			// 设置渲染类型
			instance_data.set_data(&TyUniform(&[render_flag as f32]));
		}

		// if self.is_matrix_change || self.is_content_change {
			// instance_data.set_data(&WorldUniform(self.world_matrix.as_slice()));
		// }
	}
}


// /// 每个字体buffer不同
// #[derive(Default)]
// pub struct VertexBuffers {
// 	buffers: SecondaryMap<DefaultKey, Share<BufferIndex>>,
// }

// /// 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
// #[allow(unused_must_use)]
// pub fn calc_text_sdf2(
//     query: Query<
//         (&NodeState, &LayoutResult, OrDefault<TextStyle>, Ref<NodeState>, Option<&TextOverflowData>, Entity, &Layer),
// 		// TextContent改变，NodeState必然改, TextOverflowData改变，NodeState也必然改变 ;存在NodeState， 也必然存在TextContent
//         Or<(Changed<TextStyle>, Changed<NodeState>)>, 
//     >,
// 	query_layout: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,

//     mut query_draw: Query<(&mut DrawState, &mut BoxType, &mut PipelineMeta, &NodeId), With<TextMark>>,

//     text_texture_group: OrInitRes<TextTextureGroup>,

//     res: (
//         Res<PiRenderDevice>,
//         Res<ShareAssetMgr<RenderRes<Buffer>>>,
//         ResMut<ShareFontSheet>,
//     ),
//     mut buffer: Local<(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>)>,
//     vertex_buffer_alloter: OrInitRes<PiVertexBufferAlloter>,

// 	r: OrInitRes<IsRun>,
// 	mut vertex_buffers: Local<VertexBuffers>,
// ) {
// 	if r.0 {
// 		return;
// 	}
//     let (device, buffer_assets, font_sheet) = res;
//     let mut font_sheet = font_sheet.borrow_mut();

//     // 更新纹理尺寸
//     let index_size = font_sheet.texture_size();
// 	let data_size = font_sheet.font_mgr().table.sdf2_table.data_packer_size();
//     let texture_group: std::sync::Arc<pi_assets::asset::Droper<RenderRes<pi_render::rhi::bind_group::BindGroup>>> = match &text_texture_group.0 {
//         Some(r) => r.clone(),
//         None => panic!(), // 必须要创建TextTextureGroup
//     };
// 	let texture_group1 = match &text_texture_group.1 {
//         Some(r) => r.clone(),
//         None => panic!(), // 必须要创建TextTextureGroup
//     };

//     let buffer = &mut *buffer;
//     // let mut init_spawn_drawobj = Vec::new();
//     for (mut draw_state, mut box_type, mut pipeline_meta, node_id) in query_draw.iter_mut() {
//         if let Ok((node_state, layout, text_style, node_state_change, text_overflow_data, entity, layer)) = query.get(***node_id) {
//             if node_state.0.scale < 0.000001 {
//                 continue;
//             }

// 			if layer.layer() == 0 {
// 				continue;
// 			}

//             // 如果不存在，插入默认值（只有刚创建时不存在）
//             if draw_state.vertices.get(VcolorVert::location()).is_none() {
//                 draw_state
//                     .bindgroups
//                     .insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group.clone()));
// 				draw_state
//                     .bindgroups
//                     .insert_group(DataTexSampBind::set(), DrawBindGroup::Independ(texture_group1.clone()));
//                 draw_state
//                     .bindgroups
//                     .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[index_size.width as f32, index_size.height as f32]));
// 				draw_state
//                     .bindgroups
//                     .set_uniform(&DataTexSizeUniform(&[data_size.width as f32, data_size.height as f32]));
//                 *box_type = BoxType::ContentRect;
//             }

//             let old_hash = calc_hash(&*pipeline_meta, 0);
//             let pipeline_meta1 = pipeline_meta.bypass_change_detection();
//             modify(
//                 &mut font_sheet,
//                 &node_state,
//                 &text_style,
//                 layout,
//                 &mut draw_state,
//                 &device,
//                 &buffer_assets,
//                 &node_state_change,
//                 pipeline_meta1,
//                 &mut 100,
//                 node_state.0.scale,
//                 &mut buffer.0,
//                 &mut buffer.1,
// 				&mut buffer.2,
//                 &mut buffer.3,
//                 &vertex_buffer_alloter,
// 				text_overflow_data,
// 				&query_layout,
// 				entity,
// 				&mut vertex_buffers,
//             );
//             if old_hash != calc_hash(pipeline_meta1, 0) {
//                 pipeline_meta.set_changed()
//             }
//         }
//     }
// }

// // 返回当前需要的StaticIndex
// pub fn modify<'a>(
//     font_sheet: &mut FontSheet,
//     node_state: &NodeState,
//     text_style: &TextStyle,
//     layout: &LayoutResult,
//     draw_state: &mut DrawState,
//     device: &RenderDevice,
//     buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
//     node_state_change: &Ref<NodeState>,
//     pipeline_meta: &mut PipelineMeta,
//     index_buffer_max_len: &mut usize,
//     scale: f32,
//     translation: &mut Vec<f32>, 
// 	index_info: &mut Vec<f32>, 
// 	data_offset: &mut Vec<f32>, 
// 	u_info: &mut Vec<f32>, 
//     vertex_buffer_alloter: &PiVertexBufferAlloter,
// 	text_overflow_data: Option<&TextOverflowData>,
// 	query_layout: &Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
// 	entity: Entity,
// 	vertex_buffers: &mut VertexBuffers,
// ) {
// 	let font_size = get_size(&text_style.font_size) as f32;
// 	let font_id = font_sheet.font_id(Font::new(
// 		text_style.font_family.clone(),
// 		font_size as usize,
// 		text_style.font_weight,
// 		text_style.text_stroke.width, // todo 或许应该设置比例
// 	));
// 	let font_height = font_sheet.font_height(font_id, font_size as usize);


//     // 修改vert buffer
//      match &text_style.color {
//         // 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
//         Color::RGBA(color) => draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w])),
//         // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
//         Color::LinearGradient(color) => {
// 			let mut p = [0.0, 0.0, 0.0, 0.0];
// 			let mut c = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
// 			if color.list.len() > 0 {
// 				for i in 0..4 {
// 					let c1 = match color.list.get(i) {
// 						Some(r) => r,
// 						None => color.list.last().unwrap(),
// 					};
// 					p[i] = c1.position;
// 					let p1 = i * 4;
// 					c[p1] = c1.rgba.x;
// 					c[p1 + 1] = c1.rgba.y;
// 					c[p1 + 2] = c1.rgba.z;
// 					c[p1 + 3] = c1.rgba.w;
// 				}
// 			}

// 			draw_state.bindgroups.set_uniform(&UGradientStarteEndUniform(&p));
// 			draw_state.bindgroups.set_uniform(&ClipSdfOrSdflineUniform(&c)); // 这里实际是用作渐变颜色
// 		},
//     };

// 	draw_state.bindgroups.set_uniform(&ScaleUniform(&[font_size, font_size]));

//     // 如果顶点流需要重新计算，则修改顶点流
//     if node_state_change.is_changed() {
// 		// 顶点（一个四边形）
// 		let font_info = font_sheet.font_mgr().font_info(font_id);
// 		let font_family_id = font_info.font_family_id;
// 		let first_font_face_id = font_info.font_ids[0];
// 		if !vertex_buffers.buffers.contains_key(font_family_id.0) {
// 			let buffer = font_sheet.font_mgr().table.sdf2_table.fonts[first_font_face_id.0].verties();
// 			let index = vertex_buffer_alloter.alloc(bytemuck::cast_slice(buffer.as_slice()));
// 			vertex_buffers.buffers.insert(font_family_id.0, Share::new(index));
// 		}
// 		draw_state.insert_vertices(RenderVertices {
// 			slot: AGlyphVertexVert::location(),
// 			buffer: EVerticesBufferUsage::Part(vertex_buffers.buffers[font_family_id.0].clone()),
// 			buffer_range: None,
// 			size_per_value: 8,
// 		});

//         modify_geo(
//             node_state,
//             draw_state,
//             layout,
//             &text_style.color,
//             font_sheet,
//             device,
//             index_buffer_max_len,
//             buffer_assets,
//             scale,
//             text_style,
//             translation,
//             index_info,
// 			data_offset,
// 			u_info,
//             vertex_buffer_alloter,
// 			font_height,
// 			text_overflow_data,
// 			query_layout,
// 			entity,
//         );
//     }


//     // 修改color_group
//     let color_temp;
//     let stroke = if *text_style.text_stroke.width > 0.0 {
//         &text_style.text_stroke.color
//     } else {
//         color_temp = CgColor::new(0.0, 0.0, 0.0, 0.0);
//         &color_temp
//     };

//     // let buffer = &[color.x, color.y, color.z, color.w, stroke.x, stroke.y, stroke.z, stroke.w];
   
//     draw_state
//         .bindgroups
//         .set_uniform(&StrokeColorOrURectUniform(&[stroke.x, stroke.y, stroke.z, stroke.w]));

//     if *text_style.text_stroke.width > 0.0 {
//         pipeline_meta.defines.insert(STROKE_DEFINE.clone());
//     } else {
//         pipeline_meta.defines.remove(&STROKE_DEFINE);
//     }

// 	if font_sheet.font_mgr().font_type == FontType::Sdf1{
// 		let font_info = font_sheet.font_mgr().font_info(font_id);
// 		let metrics = font_sheet.font_mgr().table.sdf_table.metrics_info(&font_info.font_ids[0]);
// 		let scale0 = scale * font_height / (metrics.ascender - metrics.descender);
// 		let sw = (scale * *text_style.text_stroke.width).round();
// 		let distance_px_range = scale0 * metrics.distance_range;
// 		let fill_bound = 0.5 - (text_style.font_weight as f32 / 500 as f32 - 1.0) / distance_px_range;
// 		let stroke = sw/2.0/distance_px_range;
// 		let stroke_bound = fill_bound - stroke;
// 		draw_state.bindgroups.set_uniform(&ClipSdfOrSdflineUniform(&[distance_px_range, fill_bound, stroke_bound]));

// 		// log::warn!("distance_px_range======{:?}", [distance_px_range, fill_bound, stroke_bound, scale0, font_height as f32, metrics.ascender - metrics.descender, font_size, scale,  metrics.distance_range as f32, metrics.font_size]);
// 		// fill_bound = fill_bound + stroke;
// 		// log::info!("=====state_scale:{:?}, scale: {}, font_height:{:?}, sw: {:?}, stroke_width: {:?}, distance_px_range: {:?}, ", node_states[*id].0.scale, scale, font_height, sw, text_style.text.stroke.width, distance_px_range);
// 		// render_obj.paramter.set_single_uniform("line", UniformValue::Float4(distance_px_range, fill_bound, stroke_bound, 0.0));
// 	}
// }




// #[inline]
// fn fill_uv(positions: &mut Vec<f32>, uvs: &mut Vec<f32>, i: usize, start: usize) {
//     let pi = i * 2;
//     let uvi = i * 2;
//     let len = positions.len() - pi;
//     let (p1, p4) = ((positions[pi], positions[pi + 1]), (positions[pi + 4], positions[pi + 5]));
//     let (u1, u4) = ((uvs[uvi], uvs[uvi + 1]), (uvs[uvi + 4], uvs[uvi + 5]));
//     if len > 8 {
//         let mut i = start;
//         while i < positions.len() {
//             let pos_x = positions[i];
//             let pos_y = positions[i + 1];
//             let uv;
//             if (pos_x - p1.0).abs() < 0.001 {
//                 let base = p4.1 - p1.1;
//                 let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
//                 uv = (u1.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
//             } else if (pos_x - p4.0).abs() < 0.001 {
//                 let base = p4.1 - p1.1;
//                 let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
//                 uv = (u4.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
//             } else if (pos_y - p1.1).abs() < 0.001 {
//                 let base = p4.0 - p1.0;
//                 let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
//                 uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u1.1);
//             } else {
//                 // }else if pos_y == p4.1{
//                 let base = p4.0 - p1.0;
//                 let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
//                 uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u4.1);
//             }
//             uvs.push(uv.0);
//             uvs.push(uv.1);
//             i += 2;
//         }
//     }
// }

// fn get_or_create_index_buffer(count: usize, device: &RenderDevice, buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>) -> Handle<RenderRes<Buffer>> {
//     let key = calc_hash(&count, calc_hash(&"index", 0));
//     match buffer_assets.get(&key) {
//         Some(r) => r,
//         None => {
//             let mut index_data: Vec<u16> = Vec::with_capacity(count * 6);
//             let mut i: u16 = 0;
//             while (i as usize) < count * 6 {
//                 index_data.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
//                 i += 4;
//             }

//             let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                 label: Some("text color index buffer init"),
//                 contents: bytemuck::cast_slice(&index_data),
//                 usage: wgpu::BufferUsages::INDEX,
//             });
//             buffer_assets.insert(key, RenderRes::new(uniform_buf, index_data.len() * 2)).unwrap()
//         }
//     }
// }

// fn get_or_create_buffer_index(
//     buffer: &[u8],
//     label: &'static str,
//     device: &RenderDevice,
//     buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
// ) -> Handle<RenderRes<Buffer>> {
//     let key = calc_hash_slice(buffer, calc_hash(&"index", 0));
//     match buffer_assets.get(&key) {
//         Some(r) => r,
//         None => {
//             let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                 label: Some(label),
//                 contents: buffer,
//                 usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
//             });
//             buffer_assets.insert(key, RenderRes::new(uniform_buf, buffer.len())).unwrap()
//         }
//     }
// }

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


// // push实例数据
// #[allow(unused_variables)]
// pub fn push_instance_data(
// 	translation: &mut Vec<f32>, 
// 	index_info: &mut Vec<f32>, 
// 	data_offset: &mut Vec<f32>, 
// 	u_info: &mut Vec<f32>, 
// 	info: &TexInfo, 
// 	x: f32,
// 	y: f32,
// ) {
// 	index_info.push(info.index_offset.0 as f32);
// 	index_info.push(info.index_offset.1 as f32);
// 	index_info.push(info.grid_w);
// 	index_info.push(info.grid_w);

// 	data_offset.push(info.data_offset.0 as f32);
// 	data_offset.push(info.data_offset.1 as f32);

// 	let check = info.cell_size * 0.5 * 2.0f32.sqrt();
// 	u_info.push(info.max_offset as f32);
// 	u_info.push(info.min_sdf as f32);
// 	u_info.push(info.sdf_step as f32);
// 	u_info.push(check);

// 	translation.push(x as f32);
// 	translation.push(y as f32);
// }

// fn push_pos_uv_sdf(
//     positions: &mut Vec<f32>,
//     uvs: &mut Vec<f32>,
// 	mut x: f32,
// 	mut y: f32,
//     glyph: &Glyph,
// 	width: f32,
// 	height: f32,
// 	weight: usize,
// 	stroke_width: f32,
// 	font_height: f32, // 单位： 逻辑像素
// 	// scale: f32,
// 	// c: char,
// ) {

// 	y += (height - font_height) / 2.0;
// 	let (xx, font_width) = pi_hal::font::font::fix_box(true, width, weight, stroke_width);
// 	x += xx;

// 	let font_ratio = font_width/glyph.advance;

// 	let ox = font_height * glyph.ox;
// 	let oy = font_height * glyph.oy;

// 	let w = (glyph.width - 1.0)*font_ratio;
// 	let h = (glyph.height - 1.0)*font_ratio;
// 	// height为行高， 当行高高于字体高度时，需要居中
// 	// if is_pixel {
// 	// 	y += (height - h)/2.0;
// 	// } else {
// 	// 	y += yy;
// 	// 	y += (height - oy - h) / 2.0;

		
// 	// }
	
// 	let left_top = (
// 		x + ox,
// 		y  + oy, // 保证顶点对应整数像素
// 	);
// 	let right_bootom = (
// 		left_top.0 + w,
// 		left_top.1 + h,
// 	);
// 	// log::info!("y=====c: {:?},is_pixel: {:?},left_top: {:?}, right_bootom: {:?}, font_width: {:?}, font_height: {:?}, glyph: {:?}, x: {}, y: {}, width: {}, height: {}, ox: {}, oy: {}, w: {}, h: {}", c, is_pixel, left_top, right_bootom, font_width, font_height, glyph, x, y, width, height, ox, oy, w, h);

//     let ps = [
//         left_top.0,
//         left_top.1,
//         left_top.0,
//         right_bootom.1,
//         right_bootom.0,
//         right_bootom.1,
//         right_bootom.0,
//         left_top.1,
// 	];
// 	// 加0.5和减0.5，是为了保证采样不超出文字范围
// 	let uv = [
//         glyph.x + 0.5,
//         glyph.y + 0.5,
//         glyph.x + 0.5,
//         glyph.y + glyph.height - 0.5,
//         glyph.x + glyph.width - 0.5,
//         glyph.y + glyph.height - 0.5,
//         glyph.x + glyph.width - 0.5,
//         glyph.y + 0.5,
// 	];
//     uvs.extend_from_slice(&uv);
// 	// log::info!("uv=================={:?}, {:?}, w:{:?},h:{:?},glyph:{:?}, font_height: {:?}, stroke_width: {:?}", uv, ps, width, height, glyph, font_height, stroke_width,);
//     positions.extend_from_slice(&ps[..]);
	
// }
