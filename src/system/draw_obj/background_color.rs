use bevy::ecs::query::{Changed, Or, With};
use bevy::ecs::{
	prelude::Ref,
	system::{Query, Res},
};
use bevy::prelude::DetectChanges;
use pi_assets::mgr::AssetMgr;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_flex_layout::prelude::Size;
use pi_polygon::{find_lg_endp, interp_mult_by_lg, mult_to_triangle, split_by_lg, to_triangle, LgCfg};
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderVertices, RenderIndices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_share::Share;
use wgpu::IndexFormat;

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::{PipelineMeta, BackgroundColorMark};
use crate::resource::draw_obj::{PosColorVertexLayout, PosVertexLayout};
use crate::shader::color::VERT_COLOR_DEFINE;
use crate::shader::ui_meterial::ColorUniform;
use crate::utils::tools::{calc_hash, get_content_rect};
use crate::{
    components::{
        calc::NodeId,
        draw_obj::{BoxType, DrawState},
        user::{BackgroundColor, Color},
    },
    resource::draw_obj::UnitQuadBuffer,
};

pub const BACKGROUND_COLOR_ORDER: u8 = 2;

/// 创建RenderObject，用于渲染背景颜色
pub fn calc_background(
    query: Query<
		(
			&BackgroundColor,
			&LayoutResult,
			Ref<BackgroundColor>,
			Ref<LayoutResult>,
		),
		Or<(Changed<BackgroundColor>, Changed<LayoutResult>)>,
	>,

	mut query_draw: Query<(&mut DrawState, &mut BoxType, &mut PipelineMeta, &NodeId), With<BackgroundColorMark>>,
    device: Res<PiRenderDevice>,

    unit_quad_buffer: Res<UnitQuadBuffer>,

    buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,
    vert_layout1: OrInitRes<PosVertexLayout>,
    vert_layout2: OrInitRes<PosColorVertexLayout>,
) {
    for (mut draw_state, mut old_unit_quad, mut pipeline_meta, node_id) in query_draw.iter_mut() {
		if let Ok((background_color, layout, background_color_change, layout_change)) = query.get(***node_id) {
			let new_unit_quad = modify(
				&background_color,
				layout,
				&mut draw_state,
				&device,
				&buffer_assets,
				&background_color_change,
				&layout_change,
				&unit_quad_buffer,
			);
			if *old_unit_quad != new_unit_quad {
				*old_unit_quad = new_unit_quad;
			}

			let (vert_layout, has_vert) = match &**background_color {
				Color::LinearGradient(_) => (&***vert_layout2, true),
				Color::RGBA(_) => (&***vert_layout1, false),
			};

			// 修改顶点布局
			if !Share::ptr_eq(vert_layout, &pipeline_meta.vert_layout) {
				pipeline_meta.vert_layout = vert_layout.clone();
				if has_vert {
					pipeline_meta.defines.insert(VERT_COLOR_DEFINE.clone());
				} else {
					pipeline_meta.defines.remove(&*VERT_COLOR_DEFINE);
				}
			}
		}
	}
}

// 返回当前需要的StaticIndex
fn modify<'a>(
    color: &Color,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bg_color_change: &Ref<BackgroundColor>,
    layout_change: &Ref<LayoutResult>,
    unit_quad_buffer: &UnitQuadBuffer,
) -> BoxType {
    // modify_radius_linear_geo
    match color {
        Color::RGBA(color) => {
            // 颜色改变，重新设置color_group
            if bg_color_change.is_changed() {
                draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));
            }
        }
        _ => (),
    };

    if let Color::LinearGradient(_) = color {
    } else {
        if bg_color_change.is_changed() {
			draw_state.vertex = 0..4;
			draw_state.insert_vertices(RenderVertices { slot: 0, buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.vertex.clone()), buffer_range: None, size_per_value: 16 });
    		draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.index.clone()), buffer_range: None, format: IndexFormat::Uint16 } );
        }
        return BoxType::ContentRect;
    }

    // 否则，需要切分顶点，如果是渐变色，还要设置color vb
    // ib、position vb、color vb
    if bg_color_change.is_changed() || layout_change.is_changed() {
        try_modify_as_radius_linear_geo(layout, device, draw_state, buffer_assets, color);
    }

    BoxType::ContentNone
}

#[inline]
fn try_modify_as_radius_linear_geo(
    layout: &LayoutResult,
    device: &RenderDevice,
    draw_state: &mut DrawState,
    buffer_asset_mgr: &Share<AssetMgr<RenderRes<Buffer>>>,
    color: &Color,
) {
    let rect = get_content_rect(layout);
    let size = Size {
        width: rect.right - rect.left,
        height: rect.bottom - rect.top,
    };
    let vb_hash = calc_hash(&rect, calc_hash(&"color vert", 0));
    let ib_hash = calc_hash(&rect, calc_hash(&"color index", 0)); // 计算颜色hash， TODO

    let (vb, ib) = match (buffer_asset_mgr.get(&vb_hash), buffer_asset_mgr.get(&ib_hash)) {
        (Some(vb), Some(ib)) => (vb, ib),
        (vb, ib) => {
            let (mut positions, mut indices) = (
                vec![
                    *rect.left,
                    *rect.top, // left_top
                    *rect.left,
                    *rect.bottom, // left_bootom
                    *rect.right,
                    *rect.bottom, // right_bootom
                    *rect.right,
                    *rect.top, // right_top
                ],
                vec![0, 1, 2, 3],
            );
            if let Color::LinearGradient(color) = color {
                let mut lg_pos = Vec::with_capacity(color.list.len());
                let mut colors = Vec::with_capacity(color.list.len() * 4);
                for v in color.list.iter() {
                    lg_pos.push(v.position);
                    colors.extend_from_slice(&[v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w]);
                }

                //渐变端点
                let endp = find_lg_endp(
                    &[0.0, 0.0, 0.0, *size.height, *size.width, *size.height, *size.width, 0.0],
                    color.direction,
                );

                let (positions1, indices1) = split_by_lg(positions, indices, lg_pos.as_slice(), endp.0.clone(), endp.1.clone());

                let mut colors = interp_mult_by_lg(
                    positions1.as_slice(),
                    &indices1,
                    vec![Vec::new()],
                    vec![LgCfg { unit: 4, data: colors }],
                    lg_pos.as_slice(),
                    endp.0,
                    endp.1,
                );

                indices = mult_to_triangle(&indices1, Vec::new());
                positions = positions1;

                let colors = colors.pop().unwrap();
                let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                    label: Some("radius or linear Color Buffer"),
                    contents: bytemuck::cast_slice(colors.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let color_hash = calc_hash(&rect, calc_hash(&"vert color", 0));

				let color_size = colors.len() * 4;
                let color = buffer_asset_mgr
                    .get(&color_hash)
                    .unwrap_or_else(|| buffer_asset_mgr.insert(color_hash, RenderRes::new(buf, color_size)).unwrap());
				draw_state.insert_vertices(RenderVertices { slot: 1, buffer: EVerticesBufferUsage::GUI(color), buffer_range: None, size_per_value: 16 });
            } else {
                indices = to_triangle(&indices, Vec::with_capacity(indices.len()));
            }
            let vb = match vb {
                Some(r) => r,
                None => {
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("radius or linear Vertex Buffer"),
                        contents: bytemuck::cast_slice(positions.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    buffer_asset_mgr.insert(vb_hash, RenderRes::new(buf, positions.len() * 4)).unwrap()
                }
            };
            let ib = match ib {
                Some(r) => r,
                None => {
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("radius or linear Index Buffer"),
                        contents: bytemuck::cast_slice(indices.as_slice()),
                        usage: wgpu::BufferUsages::INDEX,
                    });
                    buffer_asset_mgr.insert(ib_hash, RenderRes::new(buf, indices.len() * 2)).unwrap()
                }
            };
            (vb, ib)
        }
    };

	draw_state.insert_vertices(RenderVertices { slot: 0, buffer: EVerticesBufferUsage::GUI(vb), buffer_range: None, size_per_value: 8 });
	draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(ib), buffer_range: None, format: IndexFormat::Uint16 } );
}