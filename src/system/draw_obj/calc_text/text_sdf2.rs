
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{DetectChanges, Ref};
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::system::{Local, Query, Res, ResMut};
use bevy_ecs::prelude::DetectChangesMut;
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_ecs_extend::system_param::tree::{Up, Layer};
use pi_bevy_render_plugin::{PiRenderDevice, PiVertexBufferAlloter};
use pi_hal::font::font::FontType;
use pi_hal::font::sdf2_table::TexInfo;
use pi_render::font::{FontSheet, Glyph, GlyphId, Font};
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderVertices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::buffer_alloc::BufferIndex;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_share::Share;
use pi_slotmap::{SecondaryMap, DefaultKey};
use pi_style::style::TextOverflow;

use crate::components::calc::{LayoutResult, NodeId, NodeState};
use crate::components::draw_obj::{BoxType, PipelineMeta, TextMark};
use crate::components::user::{CgColor, TextStyle, get_size, TextOverflowData};
use crate::components::{draw_obj::DrawState, user::Color};
use crate::resource::draw_obj::TextTextureGroup;
use crate::resource::ShareFontSheet;
use crate::shader::text::{SampBind, VcolorVert, STROKE_DEFINE};
use crate::shader1::text_sdf2::{DataTexSampBind, AGlyphVertexVert, IndexInfoVert, TranslationVert, DataOffsetVert, InfoVert};
// use crate::shader::text;
// use crate::shader::text_sdf;

use crate::shader::ui_meterial::{ColorUniform, StrokeColorOrURectUniform, TextureSizeOrBottomLeftBorderUniform, ClipSdfOrSdflineUniform, DataTexSizeUniform, UGradientStarteEndUniform, ScaleUniform};
use crate::system::utils::set_vert_buffer;
use crate::utils::tools::{calc_hash, calc_hash_slice};

use super::IsRun;
use super::text_split::get_line_height;

/// 每个字体buffer不同
#[derive(Default)]
pub struct VertexBuffers {
	buffers: SecondaryMap<DefaultKey, Share<BufferIndex>>,
}

/// 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
#[allow(unused_must_use)]
pub fn calc_text_sdf2(
    query: Query<
        (&NodeState, &LayoutResult, OrDefault<TextStyle>, Ref<NodeState>, Option<&TextOverflowData>, Entity, &Layer),
		// TextContent改变，NodeState必然改, TextOverflowData改变，NodeState也必然改变 ;存在NodeState， 也必然存在TextContent
        Or<(Changed<TextStyle>, Changed<NodeState>)>, 
    >,
	query_layout: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,

    mut query_draw: Query<(&mut DrawState, &mut BoxType, &mut PipelineMeta, &NodeId), With<TextMark>>,

    text_texture_group: OrInitRes<TextTextureGroup>,

    res: (
        Res<PiRenderDevice>,
        Res<ShareAssetMgr<RenderRes<Buffer>>>,
        ResMut<ShareFontSheet>,
    ),
    mut buffer: Local<(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>)>,
    vertex_buffer_alloter: OrInitRes<PiVertexBufferAlloter>,

	r: OrInitRes<IsRun>,
	mut vertex_buffers: Local<VertexBuffers>,
) {
	if r.0 {
		return;
	}
    let (device, buffer_assets, font_sheet) = res;
    let mut font_sheet = font_sheet.borrow_mut();

    // 更新纹理尺寸
    let index_size = font_sheet.texture_size();
	let data_size = font_sheet.font_mgr().table.sdf2_table.data_packer_size();
    let texture_group: std::sync::Arc<pi_assets::asset::Droper<RenderRes<pi_render::rhi::bind_group::BindGroup>>> = match &text_texture_group.0 {
        Some(r) => r.clone(),
        None => panic!(), // 必须要创建TextTextureGroup
    };
	let texture_group1 = match &text_texture_group.1 {
        Some(r) => r.clone(),
        None => panic!(), // 必须要创建TextTextureGroup
    };

    let buffer = &mut *buffer;
    // let mut init_spawn_drawobj = Vec::new();
    for (mut draw_state, mut box_type, mut pipeline_meta, node_id) in query_draw.iter_mut() {
        if let Ok((node_state, layout, text_style, node_state_change, text_overflow_data, entity, layer)) = query.get(***node_id) {
            if node_state.0.scale < 0.000001 {
                continue;
            }

			if layer.layer() == 0 {
				continue;
			}

            // 如果不存在，插入默认值（只有刚创建时不存在）
            if draw_state.vertices.get(VcolorVert::location()).is_none() {
                draw_state
                    .bindgroups
                    .insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group.clone()));
				draw_state
                    .bindgroups
                    .insert_group(DataTexSampBind::set(), DrawBindGroup::Independ(texture_group1.clone()));
                draw_state
                    .bindgroups
                    .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[index_size.width as f32, index_size.height as f32]));
				draw_state
                    .bindgroups
                    .set_uniform(&DataTexSizeUniform(&[data_size.width as f32, data_size.height as f32]));
                *box_type = BoxType::ContentRect;
            }

            let old_hash = calc_hash(&*pipeline_meta, 0);
            let pipeline_meta1 = pipeline_meta.bypass_change_detection();
            modify(
                &mut font_sheet,
                &node_state,
                &text_style,
                layout,
                &mut draw_state,
                &device,
                &buffer_assets,
                &node_state_change,
                pipeline_meta1,
                &mut 100,
                node_state.0.scale,
                &mut buffer.0,
                &mut buffer.1,
				&mut buffer.2,
                &mut buffer.3,
                &vertex_buffer_alloter,
				text_overflow_data,
				&query_layout,
				entity,
				&mut vertex_buffers,
            );
            if old_hash != calc_hash(pipeline_meta1, 0) {
                pipeline_meta.set_changed()
            }
        }
    }
}

// 返回当前需要的StaticIndex
pub fn modify<'a>(
    font_sheet: &mut FontSheet,
    node_state: &NodeState,
    text_style: &TextStyle,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    node_state_change: &Ref<NodeState>,
    pipeline_meta: &mut PipelineMeta,
    index_buffer_max_len: &mut usize,
    scale: f32,
    translation: &mut Vec<f32>, 
	index_info: &mut Vec<f32>, 
	data_offset: &mut Vec<f32>, 
	u_info: &mut Vec<f32>, 
    vertex_buffer_alloter: &PiVertexBufferAlloter,
	text_overflow_data: Option<&TextOverflowData>,
	query_layout: &Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	entity: Entity,
	vertex_buffers: &mut VertexBuffers,
) {
	let font_size = get_size(&text_style.font_size) as f32;
	let font_id = font_sheet.font_id(Font::new(
		text_style.font_family.clone(),
		font_size as usize,
		text_style.font_weight,
		text_style.text_stroke.width, // todo 或许应该设置比例
	));
	let font_height = font_sheet.font_height(font_id, font_size as usize);


    // 修改vert buffer
     match &text_style.color {
        // 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
        Color::RGBA(color) => draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w])),
        // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
        Color::LinearGradient(color) => {
			let mut p = [0.0, 0.0, 0.0, 0.0];
			let mut c = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
			if color.list.len() > 0 {
				for i in 0..4 {
					let c1 = match color.list.get(i) {
						Some(r) => r,
						None => color.list.last().unwrap(),
					};
					p[i] = c1.position;
					let p1 = i * 4;
					c[p1] = c1.rgba.x;
					c[p1 + 1] = c1.rgba.y;
					c[p1 + 2] = c1.rgba.z;
					c[p1 + 3] = c1.rgba.w;
				}
			}

			draw_state.bindgroups.set_uniform(&UGradientStarteEndUniform(&p));
			draw_state.bindgroups.set_uniform(&ClipSdfOrSdflineUniform(&c)); // 这里实际是用作渐变颜色
		},
    };

	draw_state.bindgroups.set_uniform(&ScaleUniform(&[font_size, font_size]));

    // 如果顶点流需要重新计算，则修改顶点流
    if node_state_change.is_changed() {
		// 顶点（一个四边形）
		let font_info = font_sheet.font_mgr().font_info(font_id);
		let font_family_id = font_info.font_family_id;
		let first_font_face_id = font_info.font_ids[0];
		if !vertex_buffers.buffers.contains_key(font_family_id.0) {
			let buffer = font_sheet.font_mgr().table.sdf2_table.fonts[first_font_face_id.0].verties();
			let index = vertex_buffer_alloter.alloc(bytemuck::cast_slice(buffer.as_slice()));
			vertex_buffers.buffers.insert(font_family_id.0, Share::new(index));
		}
		draw_state.insert_vertices(RenderVertices {
			slot: AGlyphVertexVert::location(),
			buffer: EVerticesBufferUsage::Part(vertex_buffers.buffers[font_family_id.0].clone()),
			buffer_range: None,
			size_per_value: 8,
		});

        modify_geo(
            node_state,
            draw_state,
            layout,
            &text_style.color,
            font_sheet,
            device,
            index_buffer_max_len,
            buffer_assets,
            scale,
            text_style,
            translation,
            index_info,
			data_offset,
			u_info,
            vertex_buffer_alloter,
			font_height,
			text_overflow_data,
			query_layout,
			entity,
        );
    }


    // 修改color_group
    let color_temp;
    let stroke = if *text_style.text_stroke.width > 0.0 {
        &text_style.text_stroke.color
    } else {
        color_temp = CgColor::new(0.0, 0.0, 0.0, 0.0);
        &color_temp
    };

    // let buffer = &[color.x, color.y, color.z, color.w, stroke.x, stroke.y, stroke.z, stroke.w];
   
    draw_state
        .bindgroups
        .set_uniform(&StrokeColorOrURectUniform(&[stroke.x, stroke.y, stroke.z, stroke.w]));

    if *text_style.text_stroke.width > 0.0 {
        pipeline_meta.defines.insert(STROKE_DEFINE.clone());
    } else {
        pipeline_meta.defines.remove(&STROKE_DEFINE);
    }

	if font_sheet.font_mgr().font_type == FontType::Sdf1{
		let font_info = font_sheet.font_mgr().font_info(font_id);
		let metrics = font_sheet.font_mgr().table.sdf_table.metrics_info(&font_info.font_ids[0]);
		let scale0 = scale * font_height / (metrics.ascender - metrics.descender);
		let sw = (scale * *text_style.text_stroke.width).round();
		let distance_px_range = scale0 * metrics.distance_range;
		let fill_bound = 0.5 - (text_style.font_weight as f32 / 500 as f32 - 1.0) / distance_px_range;
		let stroke = sw/2.0/distance_px_range;
		let stroke_bound = fill_bound - stroke;
		draw_state.bindgroups.set_uniform(&ClipSdfOrSdflineUniform(&[distance_px_range, fill_bound, stroke_bound]));

		// log::warn!("distance_px_range======{:?}", [distance_px_range, fill_bound, stroke_bound, scale0, font_height as f32, metrics.ascender - metrics.descender, font_size, scale,  metrics.distance_range as f32, metrics.font_size]);
		// fill_bound = fill_bound + stroke;
		// log::info!("=====state_scale:{:?}, scale: {}, font_height:{:?}, sw: {:?}, stroke_width: {:?}, distance_px_range: {:?}, ", node_states[*id].0.scale, scale, font_height, sw, text_style.text.stroke.width, distance_px_range);
		// render_obj.paramter.set_single_uniform("line", UniformValue::Float4(distance_px_range, fill_bound, stroke_bound, 0.0));
	}
}

//
// #[inline]
pub fn modify_geo(
    node_state: &NodeState,
    draw_state: &mut DrawState,
    layout: &LayoutResult,
    color: &Color,
    font_sheet: &mut FontSheet,
    device: &RenderDevice,
    index_buffer_max_len: &mut usize,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    scale: f32,
    text_style: &TextStyle,
	translation: &mut Vec<f32>, 
	index_info: &mut Vec<f32>, 
	data_offset: &mut Vec<f32>, 
	u_info: &mut Vec<f32>, 
    vertex_buffer_alloter: &PiVertexBufferAlloter,
	font_height: f32,
	text_overflow_data: Option<&TextOverflowData>,
	query_layout: &Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	entity: Entity,
	
) {
    translation.clear();
    index_info.clear();
	data_offset.clear();
    u_info.clear();

    text_vert(node_state, layout, font_sheet, scale, text_style, translation, index_info, data_offset, u_info, font_height, text_overflow_data,  query_layout, entity);

    // 顶点长度为0，删除geo
    if translation.len() == 0 {
        draw_state.indices = None;
        draw_state.vertices.clear();
		draw_state.vertex = 0..0;
        return;
    }

	set_vert_buffer(
		IndexInfoVert::location(),
		8,
		bytemuck::cast_slice(&index_info),
		vertex_buffer_alloter,
		draw_state,
	);
	set_vert_buffer(
		TranslationVert::location(),
		8,
		bytemuck::cast_slice(&translation),
		vertex_buffer_alloter,
		draw_state,
	);
	set_vert_buffer(
		DataOffsetVert::location(),
		8,
		bytemuck::cast_slice(&data_offset),
		vertex_buffer_alloter,
		draw_state,
	);
	set_vert_buffer(
		InfoVert::location(),
		8,
		bytemuck::cast_slice(&u_info),
		vertex_buffer_alloter,
		draw_state,
	);
    draw_state.vertex = 0..4;
}

#[inline]
fn fill_uv(positions: &mut Vec<f32>, uvs: &mut Vec<f32>, i: usize, start: usize) {
    let pi = i * 2;
    let uvi = i * 2;
    let len = positions.len() - pi;
    let (p1, p4) = ((positions[pi], positions[pi + 1]), (positions[pi + 4], positions[pi + 5]));
    let (u1, u4) = ((uvs[uvi], uvs[uvi + 1]), (uvs[uvi + 4], uvs[uvi + 5]));
    if len > 8 {
        let mut i = start;
        while i < positions.len() {
            let pos_x = positions[i];
            let pos_y = positions[i + 1];
            let uv;
            if (pos_x - p1.0).abs() < 0.001 {
                let base = p4.1 - p1.1;
                let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
                uv = (u1.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
            } else if (pos_x - p4.0).abs() < 0.001 {
                let base = p4.1 - p1.1;
                let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
                uv = (u4.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
            } else if (pos_y - p1.1).abs() < 0.001 {
                let base = p4.0 - p1.0;
                let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
                uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u1.1);
            } else {
                // }else if pos_y == p4.1{
                let base = p4.0 - p1.0;
                let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
                uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u4.1);
            }
            uvs.push(uv.0);
            uvs.push(uv.1);
            i += 2;
        }
    }
}

fn get_or_create_index_buffer(count: usize, device: &RenderDevice, buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>) -> Handle<RenderRes<Buffer>> {
    let key = calc_hash(&count, calc_hash(&"index", 0));
    match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            let mut index_data: Vec<u16> = Vec::with_capacity(count * 6);
            let mut i: u16 = 0;
            while (i as usize) < count * 6 {
                index_data.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
                i += 4;
            }

            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some("text color index buffer init"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX,
            });
            buffer_assets.insert(key, RenderRes::new(uniform_buf, index_data.len() * 2)).unwrap()
        }
    }
}

fn get_or_create_buffer_index(
    buffer: &[u8],
    label: &'static str,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
) -> Handle<RenderRes<Buffer>> {
    let key = calc_hash_slice(buffer, calc_hash(&"index", 0));
    match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: buffer,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
            buffer_assets.insert(key, RenderRes::new(uniform_buf, buffer.len())).unwrap()
        }
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

#[allow(unused_variables)]
pub fn text_vert(
    node_state: &NodeState,
    layout: &LayoutResult,
    font_sheet: &mut FontSheet,
    scale: f32,
    text_style: &TextStyle,
    translation: &mut Vec<f32>, 
	index_info: &mut Vec<f32>, 
	data_offset: &mut Vec<f32>, 
	u_info: &mut Vec<f32>, 
	font_height: f32,
	text_overflow_data: Option<&TextOverflowData>,
	query_layout: &Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	mut entity: Entity,
) {
	if  node_state.0.text.len() == 0 {
		return;
	}
	let line_height = get_line_height(font_height as usize, &text_style.line_height);

	let offset_y = (line_height - font_height) / 2.0;
	let font_type = font_sheet.font_mgr().font_type;
	let is_sdf = match font_type {
		FontType::Bitmap => false,
		FontType::Sdf1 => true,
		FontType::Sdf2 => false,
	};
    let mut word_pos = (0.0, 0.0);
    let offset = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
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
						let glyph = match font_type {
							FontType::Bitmap => todo!(),
							FontType::Sdf1 => todo!(),
							FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.glyph(GlyphId(c1.ch_id)),
						};
						push_instance_data(
							translation,
							index_info,
							data_offset,
							u_info,
							glyph,
							left + text_style.letter_spacing,
							top + offset_y,
						);

						left += c1.width + text_style.letter_spacing;
					}
					i += 1;
				}
			}
			break;
		}

        // log::warn!("glyph!!!==================={:?}, {:?}, {left:?}, {top:?}", c.ch_id, c.ch);
		let glyph = match font_type {
			FontType::Bitmap => todo!(),
			FontType::Sdf1 => todo!(),
			FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.glyph(GlyphId(c.ch_id)),
		};
		push_instance_data(
			translation,
			index_info,
			data_offset,
			u_info,
			glyph,
			left,
			top + offset_y,
		);
		if count > 0 {
			count -= 1;
			if count == 0 {
				word_pos = offset;
			}
		}
    }
}

// push实例数据
#[allow(unused_variables)]
pub fn push_instance_data(
	translation: &mut Vec<f32>, 
	index_info: &mut Vec<f32>, 
	data_offset: &mut Vec<f32>, 
	u_info: &mut Vec<f32>, 
	info: &TexInfo, 
	x: f32,
	y: f32,
) {
	index_info.push(info.index_offset.0 as f32);
	index_info.push(info.index_offset.1 as f32);
	index_info.push(info.grid_w);
	index_info.push(info.grid_w);

	data_offset.push(info.data_offset.0 as f32);
	data_offset.push(info.data_offset.1 as f32);

	let check = info.cell_size * 0.5 * 2.0f32.sqrt();
	u_info.push(info.max_offset as f32);
	u_info.push(info.min_sdf as f32);
	u_info.push(info.sdf_step as f32);
	u_info.push(check);

	translation.push(x as f32);
	translation.push(y as f32);
}

fn push_pos_uv_sdf(
    positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
	mut x: f32,
	mut y: f32,
    glyph: &Glyph,
	width: f32,
	height: f32,
	weight: usize,
	stroke_width: f32,
	font_height: f32, // 单位： 逻辑像素
	// scale: f32,
	// c: char,
) {

	y += (height - font_height) / 2.0;
	let (xx, font_width) = pi_hal::font::font::fix_box(true, width, weight, stroke_width);
	x += xx;

	let font_ratio = font_width/glyph.advance;

	let ox = font_height * glyph.ox;
	let oy = font_height * glyph.oy;

	let w = (glyph.width - 1.0)*font_ratio;
	let h = (glyph.height - 1.0)*font_ratio;
	// height为行高， 当行高高于字体高度时，需要居中
	// if is_pixel {
	// 	y += (height - h)/2.0;
	// } else {
	// 	y += yy;
	// 	y += (height - oy - h) / 2.0;

		
	// }
	
	let left_top = (
		x + ox,
		y  + oy, // 保证顶点对应整数像素
	);
	let right_bootom = (
		left_top.0 + w,
		left_top.1 + h,
	);
	// log::info!("y=====c: {:?},is_pixel: {:?},left_top: {:?}, right_bootom: {:?}, font_width: {:?}, font_height: {:?}, glyph: {:?}, x: {}, y: {}, width: {}, height: {}, ox: {}, oy: {}, w: {}, h: {}", c, is_pixel, left_top, right_bootom, font_width, font_height, glyph, x, y, width, height, ox, oy, w, h);

    let ps = [
        left_top.0,
        left_top.1,
        left_top.0,
        right_bootom.1,
        right_bootom.0,
        right_bootom.1,
        right_bootom.0,
        left_top.1,
	];
	// 加0.5和减0.5，是为了保证采样不超出文字范围
	let uv = [
        glyph.x + 0.5,
        glyph.y + 0.5,
        glyph.x + 0.5,
        glyph.y + glyph.height - 0.5,
        glyph.x + glyph.width - 0.5,
        glyph.y + glyph.height - 0.5,
        glyph.x + glyph.width - 0.5,
        glyph.y + 0.5,
	];
    uvs.extend_from_slice(&uv);
	// log::info!("uv=================={:?}, {:?}, w:{:?},h:{:?},glyph:{:?}, font_height: {:?}, stroke_width: {:?}", uv, ps, width, height, glyph, font_height, stroke_width,);
    positions.extend_from_slice(&ps[..]);
	
}
