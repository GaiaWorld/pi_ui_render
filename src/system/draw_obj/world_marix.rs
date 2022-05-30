use std::io::Result;

use pi_assets::mgr::{AssetMgr, LoadResult};
use pi_ecs::prelude::{Query, Changed, Added, Write, Res, Or};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, asset::RenderRes, buffer::Buffer, bind_group::BindGroup};
use pi_share::Share;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
// use wgpu::BindGroupLayout;

use crate::{components::{user::{Node, BackgroundColor, Matrix4}, calc::{DrawList, WorldMatrix, LayoutResult}, draw_obj::{IsUnitQuad, ShaderKey, DrawObject, DrawState}}, utils::{shader_helper::WORLD_MATRIX_GROUP, tools::calc_hash}, resource::draw_obj::Shaders};

pub struct CalcWorldMatrixGroup;

#[setup]
impl CalcWorldMatrixGroup {
	/// 计算DrawObj的matrix group
	#[system]
	pub async fn calc_matrix_group<'a>(
		mut query: Query<Node, (&WorldMatrix, &LayoutResult, &DrawList), Or<(Added<BackgroundColor>, Changed<WorldMatrix>)>>,
		query_draw: Query<DrawObject, (Write<DrawState>, Option<&IsUnitQuad>, &ShaderKey)>,
		// mut draw_state_commands: Commands<DrawState>,
		// mut draw_obj_commands: EntityCommands<DrawObject>,
		// mut shader_commands: Commands<GlslShaderStatic>,
		// mut node_id_commands: Commands<NodeId>,
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'a, RenderDevice>,
		shader_static: Res<'a, Shaders>,

		buffer_assets: Res<'a, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'a, Share<AssetMgr<RenderRes<BindGroup>>>>,
	) -> Result<()> {
		let mut unit_quad_matrix = None;
		for (matrix, layout_result, draw_list) in query.iter_mut() {
			// log::info!("draw_list=============={}", draw_list.len());
			if draw_list.len() == 0 {
				continue;
			}
			
			// 遍历当前节点下所有的DrawObject，为其设置
			for draw_obj in draw_list.iter() {
				if let Some((
					mut draw_data, 
					is_unit_quad, 
					shader_key,
				)) = query_draw.get(*draw_obj) {
					let matrix_static = match shader_static.get(**shader_key) {
						Some(r) => r.bind_group.get(WORLD_MATRIX_GROUP).unwrap(),
						None => continue,
					};
					let is_unit_quad = is_unit_quad.map_or(false, |r| {r.0});
					// 如果，渲染对象的定点流为单位四边形，则需要将宽高乘到世界矩阵中
					let matrix_slice = if is_unit_quad {
						match &unit_quad_matrix {
							Some(r) => r,
							None => {
								let width = layout_result.rect.right - 
										layout_result.rect.left - 
										layout_result.border.left - 
										layout_result.border.right;
								let height = layout_result.rect.bottom - 
												layout_result.rect.top - 
												layout_result.border.bottom - 
												layout_result.border.top;
								let matrix = create_unit_offset_matrix(
									width, height,
									0.0, 0.0,
									matrix
								);
								unit_quad_matrix = Some(matrix);
								unit_quad_matrix.as_ref().unwrap()
							}
						}
					} else {
						// 否者，世界矩阵使用节点的世界矩阵
						matrix
					};
					modify_world_matrix(matrix_slice, draw_data.get_mut().unwrap(), &device, &matrix_static, &buffer_assets, &bind_group_assets).await;
					draw_data.notify_modify();
				}
			}
		}
		Ok(())
	}
}



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

async fn modify_world_matrix(
	matrix: &WorldMatrix, 
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	matrix_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) {
	let key = calc_hash(matrix);
	let uniform_buf = match AssetMgr::load(buffer_assets, &key) {
		LoadResult::Ok(r) => r,
		LoadResult::Wait(f) => f.await.unwrap(),
		LoadResult::Receiver(recv) => {
			let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
				label: Some("world matrix init"),
				contents: bytemuck::cast_slice(matrix.as_slice()),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			});
			recv.receive(key, Ok(RenderRes::new(uniform_buf, 5))).await.unwrap()
		},
	};
	let bind_group = match AssetMgr::load(bind_group_assets, &key) {
		LoadResult::Ok(r) => r,
		LoadResult::Wait(f) => f.await.unwrap(),
		LoadResult::Receiver(recv) => {
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
			recv.receive(key, Ok(RenderRes::new(group, 5))).await.unwrap()
		},
	};

	// 修改DrawState中，world_matrix对应的group
	draw_state.bind_groups.insert(WORLD_MATRIX_GROUP, bind_group);
	
}

// /// ColorLayout创建时，创建color布局
// #[derive(Deref)]
// pub struct WorldMatrixStatic(Share<BindGroupLayout>);

// impl FromWorld for WorldMatrixStatic {
//     fn from_world(world: &mut World) -> Self {
// 		let device = match unsafe { &mut *(world as *mut World as usize as *mut World)} .get_resource::<RenderDevice>() {
// 			Some(r) => r,
// 			None => panic!("init ColorStatic fail, RenderDevice is not exist")
// 		};

// 		let world_matrix_layout = Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
// 			label: Some("wolrd_matrix_layout"),
// 			entries: &[
// 				wgpu::BindGroupLayoutEntry {
// 					binding: 0,
// 					visibility: wgpu::ShaderStages::VERTEX,
// 					ty: wgpu::BindingType::Buffer {
// 						ty: wgpu::BufferBindingType::Uniform,
// 						has_dynamic_offset: false,
// 						min_binding_size: wgpu::BufferSize::new(64),
// 					},
// 					count: None,
// 				},
// 			],
// 		}));

// 		Self(world_matrix_layout)
//     }
// }