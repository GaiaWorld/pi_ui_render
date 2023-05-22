use std::slice;

use bevy::ecs::query::{Changed, Or, With};
use bevy::ecs::system::{Query, Res};
use pi_assets::mgr::AssetMgr;
use pi_cg2d::Polygon;

use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderIndices, RenderVertices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::shader::Input;
use pi_share::Share;
use polygon2::difference;
use wgpu::IndexFormat;

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::{BoxShadowMark, PipelineMeta};
use crate::components::user::{BoxShadow, Point2};
use crate::components::{calc::NodeId, draw_obj::DrawState};
use crate::shader::color::{PositionVert, SHADOW_DEFINE};
use crate::shader::ui_meterial::{BlurUniform, ColorUniform, StrokeColorOrURectUniform};
use crate::utils::tools::{calc_float_hash, calc_hash, get_box_rect};
// use crate::utils::tools::calc_hash;

pub const BOX_SHADOW_ORDER: u8 = 1;

/// 设置阴影的顶点、索引，和阴影颜色、阴影模糊半径的uniform
pub fn calc_box_shadow(
    // 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
    query: Query<(&BoxShadow, &LayoutResult), Or<(Changed<BoxShadow>, Changed<LayoutResult>)>>,
    mut query_draw: Query<(&mut DrawState, &mut PipelineMeta, &NodeId), With<BoxShadowMark>>,

    device: Res<PiRenderDevice>,

    buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,
) {
    for (mut draw_state, mut pipeline_meta, node_id) in query_draw.iter_mut() {
        if let Ok((box_shadow, layout)) = query.get(***node_id) {
            modify(&device, &mut draw_state, layout, &box_shadow, &buffer_assets);
            pipeline_meta.defines.insert(SHADOW_DEFINE.clone());
        }
    }
}

fn modify(
    device: &RenderDevice,
    draw_state: &mut DrawState,
    layout: &LayoutResult,
    shadow: &BoxShadow,
    buffer_assets_mgr: &Share<AssetMgr<RenderRes<Buffer>>>,
) {
    let g_b = get_box_rect(layout);
    if *(g_b.right) - *(g_b.left) == 0.0 || *(g_b.bottom) - *(g_b.top) == 0.0 {
        return;
    }

    let left = *(g_b.left) + shadow.h - shadow.spread - (shadow.blur / 2.0);
    let top = *(g_b.top) + shadow.v - shadow.spread - (shadow.blur / 2.0);
    let right = *g_b.right + shadow.spread + shadow.blur;
    let bottom = *g_b.bottom + shadow.spread + shadow.blur;

    let vb_hash = calc_hash(&"shadow vert", calc_float_hash(&[left, top, right, bottom, shadow.blur], 0));
    let ib_hash = calc_hash(&"shadow index", calc_float_hash(&[left, top, right, bottom, shadow.blur], 0));

    let (vb, ib) = match (buffer_assets_mgr.get(&vb_hash), buffer_assets_mgr.get(&ib_hash)) {
        (Some(vb), Some(ib)) => (vb, ib),
        (vb, ib) => {
            let bg = vec![*g_b.left, *g_b.top, *g_b.left, *g_b.bottom, *g_b.right, *g_b.bottom, *g_b.right, *g_b.top];
            let shadow = vec![left, top, left, bottom, right, bottom, right, top];

            let polygon_shadow = convert_to_f32_tow(shadow.as_slice());
            let polygon_bg = convert_to_f32_tow(bg.as_slice());
            let difference_polygons = difference(polygon_shadow, polygon_bg);

            let mut curr_index = 0;
            let mut positions: Vec<f32> = vec![];
            let mut indices: Vec<u16> = vec![];
            for p_slice in difference_polygons.into_iter() {
                let p = Polygon::new(convert_to_point(convert_to_f32(p_slice.as_slice())));
                positions.extend_from_slice(convert_to_f32(p_slice.as_slice()));

                let tri_indices = p.triangulation();
                indices.extend_from_slice(tri_indices.iter().map(|&v| (v + curr_index) as u16).collect::<Vec<u16>>().as_slice());

                curr_index += p.vertices.len();
            }

            if positions.len() == 0 {
                return;
            }

            let vb = match vb {
                Some(r) => r,
                None => {
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("radius or linear Vertex Buffer"),
                        contents: bytemuck::cast_slice(positions.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    buffer_assets_mgr.insert(vb_hash, RenderRes::new(buf, positions.len() * 4)).unwrap()
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
                    buffer_assets_mgr.insert(ib_hash, RenderRes::new(buf, indices.len() * 2)).unwrap()
                }
            };
            (vb, ib)
        }
    };
    draw_state.vertex = 0..(vb.size() / 8) as u32;
    draw_state.insert_vertices(RenderVertices {
        slot: PositionVert::location(),
        buffer: EVerticesBufferUsage::GUI(vb),
        buffer_range: None,
        size_per_value: 8,
    });
    draw_state.indices = Some(RenderIndices {
        buffer: EVerticesBufferUsage::GUI(ib),
        buffer_range: None,
        format: IndexFormat::Uint16,
    });

    let mut blur = shadow.blur;

    let min_size = (right - left).min(bottom - top);
    if blur * 2.0 > min_size {
        blur = min_size / 2.0
    }

    // uniform
    let color = &shadow.color;
    draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));
    draw_state
        .bindgroups
        .set_uniform(&StrokeColorOrURectUniform(&[left + blur, top + blur, right - blur, bottom - blur]));
    draw_state.bindgroups.set_uniform(&BlurUniform(&[shadow.blur]));
}

#[inline]
fn convert_to_point(pts: &[f32]) -> &[Point2] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const Point2;
    unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
}

// #[inline]
// fn convert_to_f32(pts: &[Point2]) -> &[f32] {
//     let ptr = pts.as_ptr();
//     let ptr = ptr as *const f32;
//     unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
// }

#[inline]
fn convert_to_f32_tow(pts: &[f32]) -> &[[f32; 2]] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const [f32; 2];
    unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
}

#[inline]
fn convert_to_f32(pts: &[[f32; 2]]) -> &[f32] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const f32;
    unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
}
