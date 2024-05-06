//! 为canvas创建DrawObj
//! canvas暂不支持圆角
use std::io::Result;

use pi_assets::asset::Asset;
use pi_ecs::prelude::{Changed, Commands, EntityCommands, Id, Query, SingleRes, Write, SingleResMut};
use pi_ecs::prelude::{Deleted, ParamSet, With};
use pi_ecs_macros::setup;
use pi_render::graph::NodeId as GraphId;
use pi_render::rhi::dyn_uniform_buffer::Group;
use wgpu::IndexFormat;
use smallvec::smallvec;

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::{BoxType, DynDrawGroup, DrawGroup};
use crate::components::user::Canvas;
use crate::resource::draw_obj::{ImageStaticIndex, StaticIndex, DynUniformBuffer, DynBindGroupIndex};

use crate::shaders::image::{PositionVertexBuffer, UiMaterialBind, UiMaterialGroup, UvVertexBuffer};
use crate::{
    components::{
        calc::{DrawList, NodeId},
        draw_obj::{DrawObject, DrawState},
        user::Node,
    },
    resource::draw_obj::UnitQuadBuffer,
};

pub struct CalcCanvas;

#[setup]
impl CalcCanvas {
    /// 创建RenderObject，用于渲染背景颜色
    #[system]
    pub async fn calc_background_image(
        mut query: ParamSet<(
            // 布局修改、BackgroundImage修改、BackgroundImageClip修改、圆角修改或删除，需要修改或创建背景图片的DrawObject
            Query<
                'static,
                'static,
                Node,
                (
                    Id<Node>,
                    &'static Canvas,
                    Write<CanvasDrawId>,
                    Write<DrawList>,
                ),
                (
                    With<Canvas>,
					With<LayoutResult>,
                    Changed<Canvas>,
                ),
            >,
            // Canvas删除，需要删除对应的DrawObject
            Query<
                'static,
                'static,
                Node,
                (Option<&'static Canvas>, Write<CanvasDrawId>, Write<DrawList>),
                Deleted<Canvas>,
            >,
        )>,

        query_draw: Query<'static, 'static, DrawObject, Write<DrawState>>,
        mut draw_obj_commands: EntityCommands<DrawObject>,
        mut draw_state_commands: Commands<DrawObject, DrawState>,
        mut node_id_commands: Commands<DrawObject, NodeId>,
        mut shader_static_commands: Commands<DrawObject, StaticIndex>,
        mut is_unit_quad_commands: Commands<DrawObject, BoxType>,
		mut graph_id_commands: Commands<DrawObject, GraphId>,

        static_index: SingleRes<'static, ImageStaticIndex>,
        unit_quad_buffer: SingleRes<'static, UnitQuadBuffer>,
		mut dyn_uniform_buffer: SingleResMut<'static, DynUniformBuffer>,
        image_material_bind_group: SingleRes<'static, DynBindGroupIndex<UiMaterialGroup>>,
    ) -> Result<()> {
        for (canvas, mut draw_index, mut render_list) in query.p1_mut().iter_mut() {
            // BackgroundColor不存在时，删除对应DrawObject
            if canvas.is_some() {
                continue;
            };

            // 删除对应的DrawObject
            if let Some(draw_index_item) = draw_index.get() {
                draw_obj_commands.despawn(draw_index_item.0.clone());
                if let Some(r) = render_list.get_mut() {
                    for i in 0..r.len() {
                        let item = &r[i];
                        if item == &draw_index_item.0 {
                            r.swap_remove(i);
                        }
                    }
                }
                draw_index.remove();
            }
        }

        for (
            node,
            canvas,
            mut draw_index,
            mut render_list,
        ) in query.p0_mut().iter_mut()
        {
            match draw_index.get() {
                // canvas修改，只需要发出通知（canvas使用单位矩形渲染，没有需要修改的其他属性）
                Some(r) => {
                    let mut draw_state_item = query_draw.get_unchecked(**r);
                    draw_state_item.notify_modify();
                }
                // 否则，创建一个新的DrawObj;
                None => {
                    // 创建新的DrawObj
                    let new_draw_obj = draw_obj_commands.spawn();
                    // 设置DrawState（包含color group）
                    let mut draw_state = DrawState::default();

					let image_material_dyn_offset = dyn_uniform_buffer.alloc_binding::<UiMaterialBind>();
                    let group = DrawGroup::Dyn(DynDrawGroup::new(
                        (*image_material_bind_group).clone(),
                        smallvec![image_material_dyn_offset],
                    ));
                    draw_state.bind_groups.insert_group(UiMaterialGroup::id(), group);

                    let new_unit_quad = modify(
                        &mut draw_state,
                        &unit_quad_buffer,
                    )
                    .await;
                    draw_state_commands.insert(new_draw_obj, draw_state);
                    // 建立DrawObj对Node的索引
                    node_id_commands.insert(new_draw_obj, NodeId(node));
                    is_unit_quad_commands.insert(new_draw_obj, new_unit_quad);
                    shader_static_commands.insert(new_draw_obj, static_index.clone());
					graph_id_commands.insert(new_draw_obj, canvas.0.clone());

                    // 建立Node对DrawObj的索引
                    draw_index.write(CanvasDrawId(new_draw_obj));
                    match render_list.get_mut() {
                        Some(r) => {
                            r.push(new_draw_obj);
                            render_list.notify_modify();
                        }
                        None => {
                            let mut r = DrawList::default();
                            r.push(new_draw_obj);
                            render_list.write(r);
                        }
                    };
                }
            }
        }
        return Ok(());
    }
}

#[derive(Deref, Default)]
pub struct CanvasDrawId(Id<DrawObject>);

// 返回当前需要的StaticIndex
async fn modify<'a>(
    draw_state: &mut DrawState,
    unit_quad_buffer: &UnitQuadBuffer,
) -> BoxType {

    let (vertex_buffer, uv_buffer, index_buffer, is_unit) = (
		unit_quad_buffer.vertex.clone(),
		unit_quad_buffer.vertex.clone(),
		unit_quad_buffer.index.clone(),
		BoxType::ContentRect,
	);

    draw_state.vbs.insert(PositionVertexBuffer::id() as usize, (vertex_buffer, 0));
	draw_state.vbs.insert(UvVertexBuffer::id() as usize, (uv_buffer, 0));
    let len = index_buffer.size() / 2;
    draw_state.ib = Some((index_buffer, len as u64, IndexFormat::Uint16));
    is_unit
}
