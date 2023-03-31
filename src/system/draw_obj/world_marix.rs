use bevy::{ecs::{
    prelude::{Entity},
    query::{Added, Changed, Or},
    system::Query,
}, prelude::DetectChangesMut};
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_bevy_ecs_extend::prelude::{OrDefault, Up};
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer, device::RenderDevice, RenderQueue};
use pi_share::Share;
// use wgpu::BindGroupLayout;

use crate::{
    components::{
        calc::{DrawList, LayoutResult, NodeState, WorldMatrix},
        draw_obj::{BoxType, DrawState},
        pass_2d::Camera,
        user::Matrix4,
    },
    shader::ui_meterial::WorldUniform,
    utils::tools::{calc_float_hash, calc_hash},
};

pub struct CalcWorldMatrixGroup;

/// 计算DrawObj的matrix group
pub fn calc_matrix_group(
    query: Query<(&WorldMatrix, &LayoutResult, &DrawList, Entity, &NodeState), Or<(Added<Camera>, Changed<DrawList>, Changed<WorldMatrix>)>>,
    query_parent: Query<&Up>,
    query_matrix: Query<&WorldMatrix>,
    mut query_draw: Query<(&mut DrawState, OrDefault<BoxType>)>,
) {
    // let mut i = 0;
    for (mut matrix, layout_result, draw_list, node, state) in query.iter() {
        if state.is_vnode() {
            // 虚拟节点，现阶段只有图文混排的文字节点，直接使用父节点的世界矩阵
            if let Ok(up) = query_parent.get(node) {
                if let Ok(r) = query_matrix.get(up.parent()) {
                    matrix = r;
                }
            }
        }

        let mut border_matrix = None;
        // 遍历当前节点下所有的DrawObject，为其设置
        for draw_obj in draw_list.iter() {
            let draw_obj = match draw_obj {
                Some(r) => r,
                None => continue,
            };
            if let Ok((mut draw_data, box_type)) = query_draw.get_mut(*draw_obj) {
                // 如果，渲染对象的顶点流为单位四边形，则需要将宽高乘到世界矩阵中
                // 如果，渲染对象的顶点流为单位四边形，则需要将宽高乘到世界矩阵中
                let matrix_slice = match box_type {
                    // BoxType::Content => {
                    // 	match &content_matrix {
                    // 		Some(r) => r,
                    // 		None => {
                    // 			let matrix = create_unit_offset_matrix_by_layout(
                    // 				layout_result,
                    // 				layout_result.border.left, layout_result.border.top,
                    // 				matrix
                    // 			);
                    // 			content_matrix = Some(matrix);
                    // 			content_matrix.as_ref().unwrap()
                    // 		}
                    // 	}
                    // },
                    BoxType::ContentRect | BoxType::BorderRect => match &border_matrix {
                        Some(r) => r,
                        None => {
                            let matrix = create_unit_offset_matrix_by_layout(layout_result, 0.0, 0.0, matrix);
                            border_matrix = Some(matrix);
                            border_matrix.as_ref().unwrap()
                        }
                    },
                    BoxType::ContentNone | BoxType::BorderNone | BoxType::Border => matrix, // 否者，世界矩阵使用节点的世界矩阵
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
    create_unit_offset_matrix(width, height, h, v, matrix)
}

#[inline]
fn create_unit_offset_matrix(width: f32, height: f32, h: f32, v: f32, matrix: &WorldMatrix) -> WorldMatrix {
    matrix
        * WorldMatrix(
            Matrix4::new(width, 0.0, 0.0, h, 0.0, height, 0.0, v, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
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
