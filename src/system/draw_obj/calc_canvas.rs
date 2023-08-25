use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::prelude::{DetectChangesMut, Without, DetectChanges, Ref};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::component::GraphId;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderIndices, RenderVertices};
use pi_render::rhi::shader::Input;
use wgpu::IndexFormat;

use crate::components::draw_obj::{BoxType, CanvasMark, PipelineMeta};
use crate::components::user::Canvas;
use crate::resource::draw_obj::ShaderInfoCache;

use crate::shader::image::PositionVert;
use crate::{
    components::{calc::NodeId, draw_obj::DrawState},
    resource::draw_obj::UnitQuadBuffer,
};

use super::calc_text::IsRun;

pub const CANVAS_ORDER: u8 = 6;

/// 设置canvas的顶点、索引
pub fn calc_canvas(
    query: Query<&Canvas>,
    mut query_draw: Query<(&mut DrawState, &mut PipelineMeta, &mut BoxType, &mut GraphId, &NodeId), With<CanvasMark>>,
    query_graph: Query<Ref<GraphId>, Without<CanvasMark>>,

    unit_quad_buffer: Res<UnitQuadBuffer>,
    shader_catch: OrInitRes<ShaderInfoCache>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    for (mut draw_state, mut pipeline_meta, mut box_type, mut graph_id, node_id) in query_draw.iter_mut() {
        if let Ok(canvas) = query.get(***node_id) {
            // draw_state创建时，设置对应drawobj的属性，并比规定GroupId
            if draw_state.is_added() {
                *box_type = modify(&mut draw_state, &unit_quad_buffer);

                pipeline_meta.state = shader_catch.premultiply.clone();
            } 
			if let Ok(src_graph_id) = query_graph.get(canvas.0) {
				if draw_state.is_added() || src_graph_id.is_changed() {
					*graph_id = src_graph_id.clone();
                    // canvas对应的图节点发生改变， 设置draw_state也改变，使得脏区域可以更新
                    draw_state.set_changed();
				}
            }
        }
    }
}

// 返回当前需要的StaticIndex
fn modify(draw_state: &mut DrawState, unit_quad_buffer: &UnitQuadBuffer) -> BoxType {
    let (vertex_buffer, index_buffer, is_unit) = (
        unit_quad_buffer.vertex.clone(),
        // unit_quad_buffer.vertex.clone(),
        unit_quad_buffer.index.clone(),
        BoxType::ContentUnitRect,
    );

    draw_state.vertex = 0..(vertex_buffer.size() / 8) as u32;
    draw_state.insert_vertices(RenderVertices {
        slot: PositionVert::location(),
        buffer: EVerticesBufferUsage::GUI(vertex_buffer),
        buffer_range: None,
        size_per_value: 8,
    });
    // draw_state.insert_vertices(RenderVertices { slot: UvVert::location(), buffer: EVerticesBufferUsage::GUI(uv_buffer), buffer_range: None, size_per_value: 8 });
    draw_state.indices = Some(RenderIndices {
        buffer: EVerticesBufferUsage::GUI(index_buffer),
        buffer_range: None,
        format: IndexFormat::Uint16,
    });

    is_unit
}
