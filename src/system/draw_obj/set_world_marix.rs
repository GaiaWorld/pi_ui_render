use bevy::{
    ecs::{
        prelude::Entity,
        query::{Changed, Or},
        system::Query,
    },
    prelude::{DetectChangesMut, With},
};
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_bevy_ecs_extend::{prelude::{OrDefault, Up}, system_param::res::OrInitRes};
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer, device::RenderDevice, RenderQueue};
use pi_share::Share;
// use wgpu::BindGroupLayout;

use crate::{
    components::{
        calc::{DrawList, LayoutResult, NodeState, WorldMatrix},
        draw_obj::{BoxType, DrawState},
        user::{BackgroundColor, BackgroundImage, BorderColor, BorderImage, BoxShadow, Canvas, Matrix4, TextContent},
    },
    shader::ui_meterial::WorldUniform,
    utils::tools::{calc_float_hash, calc_hash},
};

use super::calc_text::IsRun;

pub struct CalcWorldMatrixGroup;

/// 设置DrawObj的matrix group
/// 必须保证，创建DrawObject的system运行在此system之前，并且已经执行了apply_buffer
/// 因为此system检测DrawList的变化，当DrawList改变时，如果对应的DrawObject还未插入World，system会忽略此节点，后面可能无机会再设置此节点的matrix
pub fn set_matrix_group(
    query: Query<
        (&WorldMatrix, &LayoutResult, &DrawList, Entity, &NodeState),
        (
            Or<(
                Changed<DrawList>,
                Changed<WorldMatrix>,
                Changed<BackgroundColor>,
                Changed<BackgroundImage>,
                Changed<TextContent>,
                Changed<BorderColor>,
                Changed<Canvas>,
                Changed<BoxShadow>,
            )>,
            Or<(
                With<BackgroundImage>,
                With<BackgroundColor>,
                With<BorderImage>,
                With<TextContent>,
                With<BorderColor>,
                With<Canvas>,
                With<BoxShadow>,
            )>,
        ),
    >,
    query_parent: Query<&Up>,
    query_matrix: Query<(&WorldMatrix, &NodeState, &LayoutResult)>,
    mut query_draw: Query<(&mut DrawState, OrDefault<BoxType>)>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // let mut i = 0;
    for (mut matrix, mut layout_result, draw_list, node, mut state) in query.iter() {
        if draw_list.len() == 0 {
            continue;
        }

        let mut n = node;
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
                    layout_result = l;
                }
            }
        }

        // 遍历当前节点下所有的DrawObject，为其设置
        for draw_obj in draw_list.iter() {
            if let Ok((mut draw_data, box_type)) = query_draw.get_mut(draw_obj.id) {
                // 如果，渲染对象的顶点流为单位四边形，则需要将宽高乘到世界矩阵中
                let matrix_slice = match box_type {
                    BoxType::ContentRect => create_scale_offset_matrix(
                        1.0,
                        1.0,
                        layout_result.border.left + layout_result.padding.left,
                        layout_result.border.top + layout_result.padding.top,
                        matrix,
                    ),
                    BoxType::BorderUnitRect => create_unit_offset_matrix_by_layout(layout_result, 0.0, 0.0, matrix),
                    BoxType::PaddingUnitRect => {
                        create_unit_offset_matrix_by_layout(layout_result, layout_result.border.left, layout_result.border.top, matrix)
                    }
                    BoxType::ContentUnitRect => create_unit_offset_matrix_by_layout(
                        layout_result,
                        layout_result.border.left + layout_result.padding.left,
                        layout_result.border.top + layout_result.padding.top,
                        matrix,
                    ),
                    BoxType::ContentNone | BoxType::BorderNone | BoxType::PaddingNone | BoxType::Border => matrix.clone(), // 否者，世界矩阵使用节点的世界矩阵
                    BoxType::NotChange => continue,
                };
                let mut matrix_slice = matrix_slice.clone();
                matrix_slice.column_mut(3)[2] = node.index() as f32; // 用于调试

                // i += 1;
                draw_data.bindgroups.set_uniform(&WorldUniform(matrix_slice.as_slice()));
                draw_data.set_changed();
            }
        }
    }
}

fn create_unit_offset_matrix_by_layout(layout: &LayoutResult, h: f32, v: f32, matrix: &WorldMatrix) -> WorldMatrix {
    let width = layout.rect.right - layout.rect.left - layout.border.left - layout.border.right;
    let height = layout.rect.bottom - layout.rect.top - layout.border.bottom - layout.border.top;
    create_scale_offset_matrix(width, height, h, v, matrix)
}

#[inline]
fn create_scale_offset_matrix(scale_x: f32, scale_y: f32, h: f32, v: f32, matrix: &WorldMatrix) -> WorldMatrix {
    matrix
        * WorldMatrix(
            Matrix4::new(scale_x, 0.0, 0.0, h, 0.0, scale_y, 0.0, v, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            false,
        )
}

// pub fn modify_world_matrix(
// 	matrix: &WorldMatrix,
// 	draw_state: &mut DrawState,
// 	device: &RenderDevice,
// 	queue: &RenderQueue,
// 	matrix_layout: &BindGroupLayout,
// 	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
// 	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
// ) {
// 	let bind_group = create_world_matrix_bind(&matrix.0, device, queue, matrix_layout, buffer_assets, bind_group_assets);

// 	// 修改DrawState中，world_matrix对应的group
// 	draw_state.bindgroups.insert(WORLD_MATRIX_GROUP, bind_group);
// }

pub fn create_world_matrix_bind(
    matrix: &Matrix4,
    device: &RenderDevice,
    queue: &RenderQueue,
    matrix_layout: &BindGroupLayout,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
    let key = calc_float_hash(matrix.as_slice(), calc_hash(&"matrix", 0));
    let uniform_buf = match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            // // let time = std::time::Instant::now();
            // let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
            // 	label: Some("world matrix init"),
            // 	contents: bytemuck::cast_slice(matrix.as_slice()),
            // 	usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            // });
            // // log::warn!("create matrix_buffer_time: {:?}",  std::time::Instant::now()- time);
            // buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
            let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("world matrix init"),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                size: 64,
                mapped_at_creation: false,
            });
            queue.write_buffer(&uniform_buf, 0, bytemuck::cast_slice(matrix.as_slice()));
            buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
        }
    };
    match bind_group_assets.get(&key) {
        Some(r) => r,
        None => {
            // let time = std::time::Instant::now();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: matrix_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }],
                label: Some("world matrix group create"),
            });
            // log::warn!("create matrix_group_time: {:?}",  std::time::Instant::now()- time);
            bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
        }
    }
}
