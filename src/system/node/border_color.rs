//! 圆角从有到删除，没有正确处理顶点（TODO）

use bevy::ecs::prelude::{DetectChanges, Entity};
use bevy::ecs::query::{ChangeTrackers, Changed, Or, With};
use bevy::ecs::system::{Commands, Local, ParamSet, Query, RemovedComponents, Res};
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_bevy_assert::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::renderer::vertices::{RenderVertices, EVerticesBufferUsage, RenderIndices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::shader::BindLayout;
use pi_share::Share;
use wgpu::IndexFormat;

use crate::components::calc::{DrawInfo, EntityKey, LayoutResult};
use crate::components::draw_obj::{BoxType, PipelineMeta};
use crate::components::user::{BorderRadius, CgColor};
use crate::components::DrawBundle;
use crate::components::{
    calc::{DrawList, NodeId},
    draw_obj::DrawState,
    user::BorderColor,
};
use crate::resource::draw_obj::{PosVertexLayout, ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup};
use crate::resource::RenderObjType;
use crate::shader::color::ProgramMeta;
use crate::shader::ui_meterial::{ClipSdfUniform, ColorUniform, TextureSizeOrBottomLeftBorderUniform, UiMaterialBind};
use crate::system::utils::clear_draw_obj;
use crate::utils::tools::{cal_border_radius, calc_float_hash, calc_hash, BorderRadiusPixel};
// use crate::utils::tools::calc_hash;

lazy_static! {
    static ref BORDER: Atom = Atom::from("BORDER");
}

/// 创建RenderObject，用于渲染背景颜色
pub fn calc_border_color(
    render_type: Local<RenderObjType>,
    del: RemovedComponents<BorderColor>,
    mut query: ParamSet<(
        // 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
        Query<
            (
                Entity,
                &BorderColor,
                Option<&BorderRadius>,
                &LayoutResult,
                &mut DrawList,
                ChangeTrackers<BorderColor>,
                Option<ChangeTrackers<BorderRadius>>,
                ChangeTrackers<LayoutResult>,
            ),
            (
                With<BorderColor>,
                Or<(Changed<BorderColor>, Changed<BorderRadius>, Changed<LayoutResult>)>,
            ),
        >,
        // BorderColor删除，需要删除对应的DrawObject
        Query<(Option<&'static BorderColor>, &'static mut DrawList)>,
    )>,

    mut query_draw: Query<(&mut DrawState, &mut PipelineMeta)>,

    mut commands: Commands,

    device: Res<PiRenderDevice>,

    ui_material_group: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,

    buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,

    program_meta: OrInitRes<ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitRes<PosVertexLayout>,
    shader_catch: OrInitRes<ShaderInfoCache>,
) {
    // 删除对应的DrawObject
    clear_draw_obj(*render_type, &del, &mut query.p1(), &mut commands);

    let mut init_spawn_drawobj = Vec::new();
    for (node_id, border_color, radius, layout, mut draw_list, background_color_change, radius_change, layout_change) in query.p0().iter_mut() {
        match draw_list.get(**render_type) {
            // background_color已经存在一个对应的DrawObj， 则修改color group
            Some(r) => {
                let (mut draw_state, mut pipeline_meta) = match query_draw.get_mut(*r) {
                    Ok(r) => r,
                    _ => continue,
                };

                let count = pipeline_meta.defines.len();
                let new_fs = pipeline_meta.bypass_change_detection();
                modify(
                    border_color,
                    radius,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &background_color_change,
                    &radius_change,
                    &layout_change,
                    new_fs,
                );

                if new_fs.defines.len() != count {
                    pipeline_meta.set_changed();
                }
            }
            // 否则，创建一个新的DrawObj，并设置color group;
            // 修改以下组件：
            // * <Node, BackgroundDrawId>
            // * <Node, DrawList>
            // * <DrawObject, DrawState>
            // * <DrawObject, NodeId>
            // * <DrawObject, IsUnitQuad>
            None => {
                // 创建新的DrawObj
                let new_draw_obj = commands.spawn_empty().id();
                // 设置DrawState（包含color group）
                let mut draw_state = DrawState::default();

                // 創建color材质
                let color_material_group = ui_material_group.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), color_material_group);
                let mut pipeline_meta = PipelineMeta {
                    program: program_meta.clone(),
                    state: shader_catch.common.clone(),
                    vert_layout: vert_layout.clone(),
                    defines: Default::default(),
                };
                modify(
                    &border_color,
                    radius,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &background_color_change,
                    &radius_change,
                    &layout_change,
                    &mut pipeline_meta,
                );

                init_spawn_drawobj.push((
                    new_draw_obj,
                    DrawBundle {
                        node_id: NodeId(EntityKey(node_id)),
                        draw_state,
                        box_type: BoxType::Border,
                        pipeline_meta,
                        draw_info: DrawInfo::new(9, false), //TODO
                    },
                ));
                // 建立Node对DrawObj的索引
                draw_list.insert(**render_type, new_draw_obj);
                // draw_state_commands.insert(new_draw_obj, draw_state);
                // fs_defines_commands.insert(new_draw_obj, fs_defines);
                // is_unit_quad_commands.insert(new_draw_obj, BoxType::Border);
                // // 建立DrawObj对Node的索引
                // node_id_commands.insert(new_draw_obj, NodeId(node));
                // shader_static_commands.insert(new_draw_obj, color_static_index.clone());
                // order_commands.insert(new_draw_obj, DrawInfo::new(12, border_color.w >= 1.0));
            }
        }
    }
    if init_spawn_drawobj.len() > 0 {
        commands.insert_or_spawn_batch(init_spawn_drawobj.into_iter());
    }
}

// 返回当前需要的StaticIndex
fn modify<'a>(
    color: &CgColor,
    border_radius: Option<&BorderRadius>,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bg_color_change: &ChangeTrackers<BorderColor>,
    border_change: &Option<ChangeTrackers<BorderRadius>>,
    layout_change: &ChangeTrackers<LayoutResult>,
    pipeline_meta: &mut PipelineMeta,
) {
    // 颜色改变，重新设置color_group
    if bg_color_change.is_changed() || border_change.map_or(false, |r| r.is_changed()) || layout_change.is_changed() {
        draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));

        if let Some(border_radius) = border_radius {
            let border_radius = cal_border_radius(border_radius, layout);
            let (width, height) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
            draw_state.bindgroups.set_uniform(&ClipSdfUniform(&[
                width / 2.0,
                height / 2.0,
                1.0,
                1.0,
                width / 2.0,
                height / 2.0,
                layout.border.top,
                layout.border.right,
                border_radius.y[0],
                border_radius.x[0],
                border_radius.x[1],
                border_radius.y[1],
                border_radius.y[2],
                border_radius.x[2],
                border_radius.x[3],
                border_radius.y[3],
            ]));
            draw_state
                .bindgroups
                .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[layout.border.bottom, layout.border.left]));
        }
    }

    // 否则，需要切分顶点，如果是渐变色，还要设置color vb
    // ib、position vb、color vb
    if border_change.map_or(false, |r| r.is_changed()) || layout_change.is_changed() {
        let (radius_hash, border_radius) = match border_radius {
            Some(r) => {
                let r = cal_border_radius(r, layout);
                pipeline_meta.defines.insert(BORDER.clone());
                (calc_float_hash(&r.y, calc_float_hash(&r.x, 0)), Some(r))
            }
            None => {
                pipeline_meta.defines.remove(&*BORDER);
                (0, None)
            }
        };

        let vert_key = calc_float_hash(
            &[
                layout.rect.right - layout.rect.left,
                layout.rect.bottom - layout.rect.top,
                layout.border.top,
                layout.border.right,
                layout.border.bottom,
                layout.border.left,
            ],
            calc_hash(&("vert radius", radius_hash), 0),
        ); // layout TODO
        let index_key = calc_float_hash(
            &[
                layout.rect.right - layout.rect.left,
                layout.rect.bottom - layout.rect.top,
                layout.border.top,
                layout.border.right,
                layout.border.bottom,
                layout.border.left,
            ],
            calc_hash(&("index radius", radius_hash), 0),
        ); // layout TODO
        let (vert, index) = match (buffer_assets.get(&vert_key), buffer_assets.get(&index_key)) {
            (Some(v), Some(i)) => (v, i),
            (v, i) => {
                let (vert, indices) = get_geo_flow(&border_radius, layout);
                (
                    match v {
                        Some(r) => r,
                        None => {
                            let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("border Vertex Buffer"),
                                contents: bytemuck::cast_slice(vert.as_slice()),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                            buffer_assets.insert(vert_key, RenderRes::new(buf, vert.len() * 4)).unwrap()
                        }
                    },
                    match i {
                        Some(r) => r,
                        None => {
                            let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("border Index Buffer"),
                                contents: bytemuck::cast_slice(indices.as_slice()),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                            buffer_assets.insert(index_key, RenderRes::new(buf, indices.len() * 2)).unwrap()
                        }
                    },
                )
            }
        };
		draw_state.vertex = 0..(vert.size()/8) as u32;
		draw_state.insert_vertices(RenderVertices { slot: 0, buffer: EVerticesBufferUsage::GUI(vert), buffer_range: None, size_per_value: 8 });
		draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(index), buffer_range: None, format: IndexFormat::Uint16 } );
    }
}

pub fn create_rgba_bind_group(
    color: &CgColor,
    device: &RenderDevice,
    color_group_layout: &BindGroupLayout,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
    let key = calc_hash(&color, calc_hash(&"uniform", 0));
    match bind_group_assets.get(&key) {
        Some(r) => r,
        None => {
            let uniform_buf = match buffer_assets.get(&key) {
                Some(r) => r,
                None => {
                    let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("color buffer init"),
                        contents: bytemuck::cast_slice(&[color.x, color.y, color.z, color.w]),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });
                    buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
                }
            };
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: color_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }],
                label: Some("color group create"),
            });
            bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
        }
    }
}


#[inline]
/// 取几何体的顶点流和属性流
fn get_geo_flow(radius: &Option<BorderRadiusPixel>, layout: &LayoutResult) -> (Vec<f32>, Vec<u16>) {
    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;

    let border = &layout.border;

    let border_start_x = border.left;
    let border_start_y = border.top;
    let border_end_x = width - border.right;
    let border_end_y = height - border.bottom;
    match radius {
        None => (
            vec![
                0.0,
                0.0,
                0.0,
                height,
                width,
                height,
                width,
                0.0,
                border_start_x,
                border_start_y,
                border_start_x,
                border_end_y,
                border_end_x,
                border_end_y,
                border_end_x,
                border_start_y,
            ],
            vec![0, 1, 4, 0, 4, 3, 3, 4, 7, 3, 7, 2, 2, 7, 6, 2, 6, 1, 1, 6, 5, 1, 5, 4],
        ),
        Some(radius) => {
            let mut vert = Vec::new();
            let mut index = Vec::new();

            // 索引位置
            // 0         4      5       9
            //   ________|______|________
            //  |                        |
            //  |        3      6        |
            //  |     ___|______|___     |
            //  |    |              |    |
            //1 |-   |-2          7-|   -|8
            //  |    |              |    |
            //15|-   |-19        14-|   -|13
            //  |    |              |    |
            //  |    |___|_______|__|    |
            //  |        18     10       |
            //  |                        |
            //16|_______|_______|________|
            //         17     11     12
            vert.extend_from_slice(&[
                0.0,
                0.0, // 0
                0.0,
                radius.y[0], // 1
                border.left,
                radius.y[0], // 2
                radius.x[0],
                border.top, // 3
                radius.x[0],
                0.0, // 4
                width - radius.x[1],
                0.0, // 5
                width - radius.x[1],
                border.top, // 6
                width - border.right,
                radius.y[1], // 7
                width,
                radius.y[1], // 8
                width,
                0.0, // 9
                width - radius.x[2],
                height - border.bottom, // 10
                width - radius.x[2],
                height, // 11
                width,
                height, // 12
                width,
                height - radius.y[2], // 13
                width - border.right,
                height - radius.y[2], // 14
                0.0,
                height - radius.y[3], // 15
                0.0,
                height, // 16
                radius.x[3],
                height, // 17
                radius.x[3],
                height - border.bottom, // 18
                border.left,
                height - radius.y[3], // 19
            ]);
            index.extend_from_slice(&[
                0, 1, 2, 0, 2, 3, 0, 3, 4, // 左上
                5, 6, 9, 6, 7, 9, 7, 8, 9, // 右上
                10, 11, 12, 10, 12, 14, 14, 12, 13, // 右下
                15, 16, 19, 19, 16, 18, 18, 16, 17, // 左下
                4, 3, 6, 4, 6, 5, // 上
                7, 14, 13, 7, 13, 8, // 右
                18, 17, 11, 18, 11, 10, // 下
                1, 15, 19, 1, 19, 2, // 左
            ]);
            (vert, index)
        }
    }
}
