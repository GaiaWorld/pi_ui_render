use bevy::ecs::prelude::{DetectChanges, Ref};
use bevy::ecs::query::{Changed, Or, With};
use bevy::ecs::system::{Local, Query, Res};
use bevy::prelude::DetectChangesMut;
use ordered_float::NotNan;
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::{PiRenderDevice, PiVertexBufferAlloter};
use pi_polygon::{find_lg_endp, interp_mult_by_lg, mult_to_triangle, split_by_lg, LgCfg};
use pi_render::font::{FontSheet, Glyph, GlyphId};
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderIndices, RenderVertices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_share::Share;
use wgpu::IndexFormat;

use crate::components::calc::{LayoutResult, NodeState, NodeId};
use crate::components::draw_obj::{PipelineMeta, TextMark};
use crate::components::user::{CgColor, TextStyle};
use crate::resource::draw_obj::{EmptyVertexBuffer, TextTextureGroup};
use crate::resource::{ShareFontSheet};
use crate::shader::text::{PositionVert, SampBind, UvVert, STROKE_DEFINE, VERTEX_COLOR_DEFINE, VcolorVert};
use crate::shader::ui_meterial::{ColorUniform, StrokeColorOrURectUniform, TextureSizeOrBottomLeftBorderUniform};
use crate::components::{draw_obj::DrawState, user::Color};
use crate::system::utils::set_vert_buffer;
use crate::utils::tools::{calc_hash, calc_hash_slice};


/// 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
#[allow(unused_must_use)]
pub fn calc_text(
    query: Query<
		(
			&NodeState,
			&LayoutResult,
			OrDefault<TextStyle>,
			Ref<NodeState>,
		),
		Or<(Changed<TextStyle>, Changed<NodeState>)>, // TextContent改变，NodeState必然改;存在NodeState， 也必然存在TextContent
	>,

    mut query_draw: Query<(&mut DrawState, &mut PipelineMeta, &NodeId), With<TextMark>>,

    text_texture_group: OrInitRes<TextTextureGroup>,

    res: (
        Res<PiRenderDevice>,
        Res<ShareAssetMgr<RenderRes<Buffer>>>,
        Res<ShareFontSheet>,
        Res<EmptyVertexBuffer>,
    ),
    mut buffer: Local<(Vec<f32>, Vec<f32>)>,
    vertex_buffer_alloter: OrInitRes<PiVertexBufferAlloter>,
) {
    let (device, buffer_assets, font_sheet, empty_vert_buffer) = res;
    let font_sheet = font_sheet.borrow();

    // 更新纹理尺寸
    let size = font_sheet.texture_size();
	let texture_group = match &***text_texture_group {
		Some(r) => r,
		None => panic!(), // 必须要创建TextTextureGroup
	};

    let buffer = &mut *buffer;
    // let mut init_spawn_drawobj = Vec::new();
	for (mut draw_state, mut pipeline_meta, node_id) in query_draw.iter_mut() {
		if let Ok((node_state, layout, text_style, node_state_change)) = query.get(***node_id) {
			if node_state.0.scale < 0.000001 {
				continue;
			}

			// 如果不存在，插入默认值（只有刚创建时不存在）
            if draw_state.vertices.get(VcolorVert::location()).is_none() {
                draw_state.insert_vertices(RenderVertices {
                    slot: VcolorVert::location(),
                    buffer: EVerticesBufferUsage::GUI((*empty_vert_buffer).clone()),
                    buffer_range: None,
                    size_per_value: 16,
                });
                draw_state
                    .bindgroups
                    .insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group.clone()));
                draw_state
                    .bindgroups
                    .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[size.width as f32, size.height as f32]));
            }

            let old_hash = calc_hash(&*pipeline_meta, 0);
            let pipeline_meta1 = pipeline_meta.bypass_change_detection();
            modify(
                &font_sheet,
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
                &vertex_buffer_alloter,
            );
            if old_hash != calc_hash(pipeline_meta1, 0) {
                pipeline_meta.set_changed()
            }
		}
	}
    
}

// 返回当前需要的StaticIndex
pub fn modify<'a>(
    font_sheet: &FontSheet,
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
    positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    vertex_buffer_alloter: &PiVertexBufferAlloter,
) {
    // 修改vert buffer
    let is_change_geo = match &text_style.color {
        // 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
        Color::RGBA(_) => {
            if node_state_change.is_changed() || pipeline_meta.defines.get(&Atom::from("VERTEX_COLOR")).is_some() {
                true
            } else {
                false
            }
        }
        // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
        Color::LinearGradient(_) => true,
    };

    // 如果顶点流需要重新计算，则修改顶点流
    if is_change_geo {
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
            text_style.text_stroke.width,
            positions,
            uvs,
            vertex_buffer_alloter,
        );
    }


    // 修改color_group
    let color_temp;
    let color = match &text_style.color {
        Color::RGBA(c) => {
            pipeline_meta.defines.remove(&VERTEX_COLOR_DEFINE);
            c
        }
        Color::LinearGradient(_) => {
            pipeline_meta.defines.insert(VERTEX_COLOR_DEFINE.clone());
            color_temp = CgColor::default();
            &color_temp
        }
    };

    let color_temp;
    let stroke = if *text_style.text_stroke.width > 0.0 {
        &text_style.text_stroke.color
    } else {
        color_temp = CgColor::new(0.0, 0.0, 0.0, 0.0);
        &color_temp
    };

    // let buffer = &[color.x, color.y, color.z, color.w, stroke.x, stroke.y, stroke.z, stroke.w];
    draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));
    draw_state
        .bindgroups
        .set_uniform(&StrokeColorOrURectUniform(&[stroke.x, stroke.y, stroke.z, stroke.w]));

    if *text_style.text_stroke.width > 0.0 {
        pipeline_meta.defines.insert(STROKE_DEFINE.clone());
    } else {
        pipeline_meta.defines.remove(&STROKE_DEFINE);
    }
}

//
// #[inline]
pub fn modify_geo(
    node_state: &NodeState,
    draw_state: &mut DrawState,
    layout: &LayoutResult,
    color: &Color,
    font_sheet: &FontSheet,
    device: &RenderDevice,
    index_buffer_max_len: &mut usize,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    scale: f32,
    stroke_width: NotNan<f32>,
    mut positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    vertex_buffer_alloter: &PiVertexBufferAlloter,
) {
    positions.clear();
    uvs.clear();
    let rect = &layout.rect;
    text_vert(node_state, layout, font_sheet, scale, stroke_width, positions, uvs);

    // 顶点长度为0，删除geo
    if positions.len() == 0 {
        draw_state.indices = None;
        draw_state.vertices.clear();
        return;
    }
	let mut positions1;
    match color {
        Color::RGBA(_) => {
            // 更新ib
            let l = positions.len() / 8;
            while l > *index_buffer_max_len {
                *index_buffer_max_len = l + 50;
            }
            let index_buffer = get_or_create_index_buffer(*index_buffer_max_len, device, buffer_assets);
            draw_state.indices = Some(RenderIndices {
                buffer: EVerticesBufferUsage::GUI(index_buffer),
                buffer_range: Some(0..(l * 6 * 2) as u64),
                format: IndexFormat::Uint16,
            });
			set_vert_buffer(
				PositionVert::location(),
				8,
				bytemuck::cast_slice(&positions),
				vertex_buffer_alloter,
				draw_state,
			);
        }
        Color::LinearGradient(color) => {
            let mut i = 0;
            let mut colors = vec![Vec::new()];
            let mut indices = Vec::with_capacity(6);

            let endp = match node_state.0.is_vnode() {
                // 如果是虚拟节点，则节点自身的布局信息会在顶点上体现，此时找渐变端点需要考虑布局结果的起始点
                true => find_lg_endp(
                    &[
                        rect.left,
                        rect.top,
                        rect.left,
                        rect.bottom,
                        rect.right,
                        rect.bottom,
                        rect.right,
                        rect.top, //渐变端点
                    ],
                    color.direction,
                ),
                // 非虚拟节点，顶点总是以0，0作为起始点，布局起始点体现在世界矩阵上
                false => find_lg_endp(
                    &[
                        0.0,
                        0.0,
                        0.0,
                        rect.bottom - rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        rect.right - rect.left,
                        0.0,
                    ],
                    color.direction,
                ),
            };

            let mut lg_pos = Vec::with_capacity(color.list.len());
            let mut lg_color = Vec::with_capacity(color.list.len() * 4);
            for v in color.list.iter() {
                lg_pos.push(v.position);
                lg_color.extend_from_slice(&[v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w]);
            }
            let lg_color = vec![LgCfg { unit: 4, data: lg_color }];
			

            let len = positions.len() / 2;
            let mut old_len = positions.len();
			positions1 = positions.clone();
            while (i as usize) < len {
                // log::info!("position: {:?}, {:?}, {:?}, {:?}, {:?}", positions, node_state.0.text, lg_pos, &endp.0, &endp.1);
                let (ps1, indices_arr) = split_by_lg(
                    positions1,
                    vec![i, i + 1, i + 2, i + 3],
                    lg_pos.as_slice(),
                    endp.0.clone(),
                    endp.1.clone(),
                );
				positions1 = ps1;

                // 颜色插值
                colors = interp_mult_by_lg(
                    positions1.as_slice(),
                    &indices_arr,
                    colors,
                    lg_color.clone(),
                    lg_pos.as_slice(),
                    endp.0.clone(),
                    endp.1.clone(),
                );

                // 尝试为新增的点计算uv
                if positions1.len() > old_len {
                    fill_uv(&mut positions1, uvs, i as usize, old_len);
                    old_len = positions1.len();
                }

                indices = mult_to_triangle(&indices_arr, indices);
                i = i + 4;
            }

            let index_buffer = get_or_create_buffer_index(bytemuck::cast_slice(&indices), "text vert index buffer", device, buffer_assets);
            draw_state.indices = Some(RenderIndices {
                buffer: EVerticesBufferUsage::GUI(index_buffer),
                buffer_range: None,
                format: IndexFormat::Uint16,
            });

            let colors = colors.pop().unwrap();
            set_vert_buffer(VcolorVert::location(), 16, bytemuck::cast_slice(&colors), vertex_buffer_alloter, draw_state);
			set_vert_buffer(
				PositionVert::location(),
				8,
				bytemuck::cast_slice(&positions1),
				vertex_buffer_alloter,
				draw_state,
			);
            // draw_state.insert_vertices(RenderVertices { slot: 2, buffer: EVerticesBufferUsage::GUI(color_buffer), buffer_range: None, size_per_value: 16 });
        }
    }
   
    // let positions_buffer = get_or_create_buffer(bytemuck::cast_slice(&positions), "text position buffer", device, buffer_assets);
    // draw_state.insert_vertices(RenderVertices { slot: PositionVert::location(), buffer: EVerticesBufferUsage::GUI(positions_buffer), buffer_range: None, size_per_value: 8 });
    set_vert_buffer(UvVert::location(), 8, bytemuck::cast_slice(&uvs), vertex_buffer_alloter, draw_state);
    // let uv_buffer = get_or_create_buffer(bytemuck::cast_slice(&uvs), "text uv buffer", device, buffer_assets);
    // draw_state.insert_vertices(RenderVertices { slot: UvVert::location(), buffer: EVerticesBufferUsage::GUI(uv_buffer), buffer_range: None, size_per_value: 8 });
    draw_state.vertex = 0..(positions.len() * 4 / 8) as u32;
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

#[allow(unused_variables)]
pub fn text_vert(
    node_state: &NodeState,
    layout: &LayoutResult,
    font_sheet: &FontSheet,
    scale: f32,
    stroke_width: NotNan<f32>,
    positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
) {
    let mut word_pos = (0.0, 0.0);
    let mut count = 0;
    let half_stroke = *stroke_width / 2.0;
    for c in node_state.0.text.iter() {
        if c.ch == char::from(0) {
            if c.count > 0 {
                word_pos = (c.pos.left, c.pos.top);
                count = c.count - 1;
            }
            continue;
        }
        if c.ch <= ' ' {
            continue;
        }

        // log::warn!("glyph!!!==================={:?}, {:?}", c.ch_id, c.ch);
        let glyph = font_sheet.glyph(GlyphId(c.ch_id));
        if count > 0 {
            count -= 1;
            push_pos_uv(
                positions,
                uvs,
                word_pos.0 + c.pos.left - half_stroke,
                word_pos.1 + c.pos.top,
                &glyph,
                c.pos.right - c.pos.left,
                c.pos.bottom - c.pos.top,
                scale,
            );
        } else {
            push_pos_uv(
                positions,
                uvs,
                c.pos.left - half_stroke,
                c.pos.top,
                &glyph,
                c.pos.right - c.pos.left,
                c.pos.bottom - c.pos.top,
                scale,
            );
        }
    }
}

#[allow(unused_variables)]
pub fn push_pos_uv(positions: &mut Vec<f32>, uvs: &mut Vec<f32>, x: f32, mut y: f32, glyph: &Glyph, width: f32, height: f32, scale: f32) {
    // let font_ratio = width/glyph.width;
    let w = glyph.width / scale;
    let h = glyph.height / scale;

    // height为行高， 当行高高于字体高度时，需要居中
    y += (height - h) / 2.0;
    let left_top = (
        (x * scale).round() / scale,
        (y * scale).round() / scale, // 保证顶点对应整数像素
    );
    let right_bootom = (left_top.0 + w, left_top.1 + h);

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

    let gx = glyph.x as f32;
    let gy = glyph.y as f32;
    let uv = [gx, gy, gx, gy + glyph.height, gx + glyph.width, gy + glyph.height, gx + glyph.width, gy];
    uvs.extend_from_slice(&uv);
    // log::warn!("uv=================={:?}, {:?}, w:{:?},h:{:?},scale:{:?},glyph:{:?}", uv, ps, width, height, scale, glyph);
    positions.extend_from_slice(&ps[..]);
}
