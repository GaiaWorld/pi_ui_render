use std::io::Result;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::{prelude::{Query, Changed, Added, Write, Res, Or, OrDefault}, entity::Id, storage::Offset};
use pi_ecs_macros::setup;
use pi_render::rhi::{
	device::RenderDevice, 
	asset::RenderRes, 
	buffer::Buffer, 
	bind_group::BindGroup};
use pi_share::Share;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
// use wgpu::BindGroupLayout;

use crate::{
	components::{
		user::{Node, Matrix4}, 
		calc::{DrawList, WorldMatrix, LayoutResult, Pass2DId}, 
		draw_obj::{BoxType, DrawObject, DrawState}
	}, 
	utils::{shader_helper::WORLD_MATRIX_GROUP, tools::{calc_float_hash, calc_hash}}, resource::draw_obj::Shaders, system::shader_utils::StaticIndex};

pub struct CalcWorldMatrixGroup;

#[setup]
impl CalcWorldMatrixGroup {
	/// 计算DrawObj的matrix group
	#[system]
	pub async fn calc_matrix_group<'a>(
		mut query: Query<'a, 'a, Node, (&WorldMatrix, &LayoutResult, &DrawList, Id<Node>), Or<(Added<Pass2DId>,Changed<DrawList>, Changed<WorldMatrix>)>>,
		query_draw: Query<'a, 'a, DrawObject, (Write<DrawState>, OrDefault<BoxType>, &StaticIndex)>,
		device: Res<'a, RenderDevice>,
		shader_static: Res<'a, Shaders>,

		buffer_assets: Res<'a, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'a, Share<AssetMgr<RenderRes<BindGroup>>>>,
	) -> Result<()> {
		for (matrix, layout_result, draw_list, node) in query.iter_mut() {
			let mut content_matrix = None;
			let mut border_matrix = None;
			// 遍历当前节点下所有的DrawObject，为其设置
			for draw_obj in draw_list.iter() {
				if let Some((
					mut draw_data, 
					box_type, 
					static_index,
				)) = query_draw.get(*draw_obj) {
					let matrix_static = match shader_static.get(static_index.shader) {
						Some(r) => r.bind_group.get(WORLD_MATRIX_GROUP).unwrap(),
						None => continue,
					};
					// 如果，渲染对象的定点流为单位四边形，则需要将宽高乘到世界矩阵中
					let matrix_slice = match box_type {
						BoxType::Content => {
							match &content_matrix {
								Some(r) => r,
								None => {
									let matrix = create_unit_offset_matrix_by_layout(
										layout_result,
										layout_result.border.left, layout_result.border.top,
										matrix
									);
									content_matrix = Some(matrix);
									content_matrix.as_ref().unwrap()
								}
							}
						},
						BoxType::Border => {
							match &border_matrix {
								Some(r) => r,
								None => {
									let matrix = create_unit_offset_matrix_by_layout(
										layout_result,
										0.0, 0.0,
										matrix
									);
									border_matrix = Some(matrix);
									border_matrix.as_ref().unwrap()
								}
							}
						}
						BoxType::None => matrix, // 否者，世界矩阵使用节点的世界矩阵
						
					};
					let mut matrix_slice = matrix_slice.clone();
					matrix_slice.column_mut(3)[2] = node.offset() as f32;
					let matrix_slice = &matrix_slice;
					modify_world_matrix(matrix_slice, draw_data.get_mut().unwrap(), &device, &matrix_static, &buffer_assets, &bind_group_assets);
					draw_data.notify_modify();
				}
			}
		}
		Ok(())
	}
}

fn create_unit_offset_matrix_by_layout(
    layout: &LayoutResult,
    h: f32,
    v: f32,
    matrix: &WorldMatrix,
) -> WorldMatrix {
	let width = layout.rect.right - 
		layout.rect.left - 
		layout.border.left - 
		layout.border.right;
	let height = layout.rect.bottom - 
		layout.rect.top - 
		layout.border.bottom - 
		layout.border.top;
	create_unit_offset_matrix(
		width, height,
		h, v,
		matrix
	)
	
}

#[inline]
fn create_unit_offset_matrix(
    width: f32,
    height: f32,
    h: f32,
    v: f32,
    matrix: &WorldMatrix,
) -> WorldMatrix {
    matrix
        * WorldMatrix(
			Matrix4::new(
				width,0.0,0.0, h,
                0.0,height,0.0, v,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0,
            ),
            false,
        )
}

pub fn modify_world_matrix(
	matrix: &WorldMatrix, 
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	matrix_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) {
	let bind_group = create_world_matrix_bind(&matrix.0, device, matrix_layout, buffer_assets, bind_group_assets);

	// 修改DrawState中，world_matrix对应的group
	draw_state.bind_groups.insert(WORLD_MATRIX_GROUP, bind_group);
}

pub fn create_world_matrix_bind(
	matrix: &Matrix4, 
	device: &RenderDevice, 
	matrix_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_float_hash(matrix.as_slice(), calc_hash(&"matrix", 0));
	let uniform_buf = match buffer_assets.get(&key) {
		Some(r) => r,
		None => {
			let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
				label: Some("world matrix init"),
				contents: bytemuck::cast_slice(matrix.as_slice()),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			});
			buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
		}
	};
	match bind_group_assets.get(&key){
		Some(r) => r,
		None => {
			let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: matrix_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: uniform_buf.as_entire_binding(),
					},
				],
				label: Some("world matrix group create"),
			});
			bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
		},
	}
}
