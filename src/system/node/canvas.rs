//! 为canvas创建DrawObj
//! canvas暂不支持圆角
use std::io::Result;

use bevy::ecs::prelude::{DetectChanges, Entity};
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Local, ParamSet, Query, RemovedComponents, Res};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_render::renderer::vertices::{RenderVertices, EVerticesBufferUsage, RenderIndices};
use pi_render::rhi::shader::{BindLayout, Input};
use wgpu::IndexFormat;

use crate::components::calc::{EntityKey, LayoutResult};
use crate::components::draw_obj::{BoxType, GraphOut, PipelineMeta};
use crate::components::user::Canvas;
use crate::components::DrawBundle;
use crate::resource::draw_obj::{PosUv1VertexLayout, ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup};
use crate::resource::RenderObjType;

use crate::shader::image::{PositionVert, ProgramMeta, UvVert};
use crate::shader::ui_meterial::UiMaterialBind;
use crate::system::utils::clear_draw_obj;
use crate::{
    components::{
        calc::{DrawList, NodeId},
        draw_obj::DrawState,
    },
    resource::draw_obj::UnitQuadBuffer,
};

/// 创建RenderObject，用于渲染背景颜色
pub fn calc_background_image(
    render_type: Local<RenderObjType>,
    del: RemovedComponents<Canvas>,
    mut query: ParamSet<(
        // 布局修改、BackgroundImage修改、BackgroundImageClip修改、圆角修改或删除，需要修改或创建背景图片的DrawObject
        Query<(Entity, &Canvas, &mut DrawList), (With<Canvas>, With<LayoutResult>, Changed<Canvas>)>,
        // Canvas删除，需要删除对应的DrawObject
        Query<(Option<&'static Canvas>, &'static mut DrawList)>,
    )>,

    mut query_draw: Query<&mut DrawState>,
    mut commands: Commands,

    unit_quad_buffer: Res<UnitQuadBuffer>,
    ui_material_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,

    program_meta: OrInitRes<ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitRes<PosUv1VertexLayout>,
    shader_catch: OrInitRes<ShaderInfoCache>,
) -> Result<()> {
    // 删除对应的DrawObject
    clear_draw_obj(*render_type, &del, &mut query.p1(), &mut commands);

    let mut init_spawn_drawobj = Vec::new();
    for (node_id, canvas, mut draw_list) in query.p0().iter_mut() {
        match draw_list.get(**render_type) {
            // canvas修改，只需要发出通知（canvas使用单位矩形渲染，没有需要修改的其他属性）
            Some(r) => {
                let mut draw_state = match query_draw.get_mut(*r) {
                    Ok(r) => r,
                    _ => continue,
                };

                // 设置改变
                draw_state.set_changed();
            }
            // 否则，创建一个新的DrawObj;
            None => {
                // 创建新的DrawObj
                let new_draw_obj = commands.spawn_empty().id();
                // 设置DrawState（包含color group）
                let mut draw_state = DrawState::default();

                let ui_material_group = ui_material_alloter.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                let box_type = modify(&mut draw_state, &unit_quad_buffer);

                init_spawn_drawobj.push((
                    new_draw_obj,
                    (
                        DrawBundle {
                            node_id: NodeId(EntityKey(node_id)),
                            draw_state,
                            box_type,
                            pipeline_meta: PipelineMeta {
                                program: program_meta.clone(),
                                state: shader_catch.premultiply.clone(),
                                vert_layout: vert_layout.clone(),
                                defines: Default::default(),
                            },
                            draw_info: Default::default(),
                        },
                        GraphOut(canvas.0.clone()),
                    ),
                ));
                // 建立Node对DrawObj的索引
                draw_list.insert(**render_type, new_draw_obj);
            }
        }
    }
    return Ok(());
}

// 返回当前需要的StaticIndex
fn modify(draw_state: &mut DrawState, unit_quad_buffer: &UnitQuadBuffer) -> BoxType {
    let (vertex_buffer, uv_buffer, index_buffer, is_unit) = (
        unit_quad_buffer.vertex.clone(),
        unit_quad_buffer.vertex.clone(),
        unit_quad_buffer.index.clone(),
        BoxType::ContentRect,
    );

	draw_state.insert_vertices(RenderVertices { slot: PositionVert::location(), buffer: EVerticesBufferUsage::GUI(vertex_buffer), buffer_range: None, size_per_value: 8 });
	draw_state.insert_vertices(RenderVertices { slot: UvVert::location(), buffer: EVerticesBufferUsage::GUI(uv_buffer), buffer_range: None, size_per_value: 8 });
	draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(index_buffer), buffer_range: None, format: IndexFormat::Uint16 } );

    is_unit
}
