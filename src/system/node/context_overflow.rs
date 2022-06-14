//! 处理overflow属性，
//! 1. 对overflow设置为true的节点，标记为渲染上下文（设置RenderContextMark中的位标记）
//! 2. 

use pi_assets::mgr::AssetMgr;
use pi_dirty::LayerDirty;
use pi_ecs::{monitor::Event, prelude::{Query, Write, Local, ChangeTrackers, With, ParamSet, Join, FromWorld, Res, Commands, EntityCommands}, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Layer;
use pi_render::rhi::{asset::RenderRes, buffer::Buffer, bind_group::BindGroup, device::RenderDevice};
use pi_share::Share;
use pi_slotmap::{DefaultKey, KeyData};
use wgpu::IndexFormat;

use crate::{components::{user::{Node, Overflow, Aabb2, Vector4, Point2, Matrix4, Transform, TransformFunc}, calc::{RenderContextMark, WorldMatrix, Pass2DId, TransformWillChangeMatrix, NodeId, OverflowAabb, LayoutResult, Quad}, pass_2d::{Pass2D, ParentPassId, PostProcessList, PostProcess}, draw_obj::{DrawObject, DrawState, ShaderKey, PipelineKey, VertexBufferLayoutKey, VSDefines}}, resource::{RenderContextMarkType, draw_obj::{UnitQuadBuffer, ShareLayout}}, utils::{tools::intersect, shader_helper::VIEW_GROUP}, system::shader_utils::{post_process::PostProcessStaticIndex, create_camera_bind_group, image::{POST_TEXTURE_GROUP, POST_UV_LOCATION}}};

pub struct CalcOverflow;

/// overflow 后处理的索引
#[derive(Deref)]
pub struct OverflowRenderContextMarkType(RenderContextMarkType);

impl FromWorld for OverflowRenderContextMarkType{
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
        Self(RenderContextMarkType::from_world(world))
    }
}

#[setup]
impl CalcOverflow {
	#[system]
	pub fn calc_overflow(
		mark_type: Res<OverflowRenderContextMarkType>,
		query: Query<Node, (
			Id<Node>,
			&Pass2DId,
			&Layer,
			// transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
			// 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
			ChangeTrackers<WorldMatrix>, 
			ChangeTrackers<TransformWillChangeMatrix>, 
			ChangeTrackers<Overflow>, 
			ChangeTrackers<Layer>), With<Overflow>>,
		mut write: ParamSet<(
			Query<Pass2D, (
				Write<OverflowAabb>, 
				Write<PostProcessList>, 
				Join<NodeId, Node, (
					&'static WorldMatrix, 
					Option<&'static TransformWillChangeMatrix>, 
					&'static LayoutResult, 
					&'static Quad
				)>
			)>,
			Query<Pass2D, 
				Join<ParentPassId, Pass2D, (Id<Pass2D>, ChangeTrackers<OverflowAabb>, Option<&'static OverflowAabb>)>
			>,
		)>,
		mut local: Local<LayerDirty<(Id<Node>, Id<Pass2D>, bool)>>,

		device: Res<RenderDevice>,
		mut query_draw: Query<DrawObject, Write<DrawState>>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut shader_id_commands: Commands<DrawObject, ShaderKey>,
		mut pipeline_state_commands: Commands<DrawObject, PipelineKey>,
		mut vertex_buffer_layout_commands: Commands<DrawObject, VertexBufferLayoutKey>,
		mut vs_defines_commands: Commands<DrawObject, VSDefines>,
		share_layout: Res<ShareLayout>,

		unit_quad_buffer: Res<UnitQuadBuffer>,
		static_index: Res<PostProcessStaticIndex>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,

	) {
		for (id, pass_id, layer, tracker_matrix, tracker_willchange, tracker_overflow, tracker_layer) in query.iter() {
			local.mark( 
				(id, 
					**pass_id, 
					tracker_overflow.is_changed() || 
					tracker_willchange.is_changed() || 
						tracker_layer.is_changed() || 
						tracker_matrix.is_changed()
				), **layer);
		}

		for ((id, pass_id, is_changed), _layer) in local.iter() {
			let mut parent_changed = false;
			let mut parent_aabb = None;
			let mut cur_id = pass_id.clone();
			let p1 = write.p1();
			loop {
				if let Some((parent_id, parent_overflow_aabb_tracker, parent_overflow_aabb)) = p1.get(cur_id) {
					if let Some(r) = parent_overflow_aabb {
						parent_changed = parent_overflow_aabb_tracker.is_changed();
						parent_aabb = Some(r.clone());
						break;
					}
					cur_id = parent_id;
				}
				break;
			}
			
			if parent_changed || *is_changed {
				if let Some((
					mut overflow_aabb, 
					mut post_list,
					(
						matrix, 
						will_change, 
						layout, 
						quad))) = write.p0_mut().get_mut(*pass_id) {
					let matrix_temp;
					let (matrix, is_rotation) = match will_change {
						Some(r) => if matrix.1 || r.will_change.1 {
							matrix_temp = &r.will_change * matrix;
							(&matrix_temp, matrix.1)
						} else {
							(&r.will_change, false)
						},
						None => (matrix, matrix.1)
					};
					println!("xxx:{:?}", matrix.0.clone());
					if is_rotation {
						// 如果存在旋转，需要在逆旋转渲染，然后对逆旋转的渲染结果进行后处理
						let rotate_matrix_invert = calc_rotate_matrix(matrix.0.clone());
						println!("zzzz: \n{:?},\n{:?}, \n{:?}", rotate_matrix_invert, &matrix.0, rotate_matrix_invert * &matrix.0);
						let rotate_matrix = rotate_matrix_invert.try_inverse().unwrap();

						let width = layout.rect.right - layout.rect.left;
						let height = layout.rect.bottom - layout.rect.top;

						let aabb = cal_no_rotate_box(&Aabb2::new(Point2::new(0.0, 0.0), Point2::new(width, height)), &(rotate_matrix_invert * matrix.0));
						println!("==={:?}\n{:?}\n{:?}\n{:?}\n{:?}", width, height, rotate_matrix_invert, &matrix.0, rotate_matrix_invert * matrix.0);
						

						overflow_aabb.write(OverflowAabb {
							aabb: Some(aabb),
							matrix: Some(rotate_matrix_invert),
						});

						let post_list = post_list.get_mut_or_default();
						let post_key = DefaultKey::from(KeyData::from_ffi(***mark_type as u64));

						match post_list.0.get(post_key) {
							Some(r) => {
								let mut draw_state = query_draw.get_unchecked_mut(r.draw_obj_key);
								let bind_group = create_camera_bind_group(
									&rotate_matrix,
									&share_layout.view,
									&device,
									&buffer_assets,
									&bind_group_assets,
								);
								draw_state.get_mut().unwrap().bind_groups.insert(VIEW_GROUP, bind_group);
							},
							None => {
								let new_draw_obj = draw_obj_commands.spawn();
								// 设置DrawState（包含color group）
								let mut draw_state = DrawState::default();
								draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
								draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
								// opacity
								let bind_group = create_camera_bind_group(
									&rotate_matrix,
									&share_layout.view,
									&device,
									&buffer_assets,
									&bind_group_assets,
								);
								draw_state.bind_groups.insert(VIEW_GROUP, bind_group);
								
								draw_state_commands.insert(new_draw_obj, draw_state);
								// 建立DrawObj对Node的索引
								node_id_commands.insert(new_draw_obj, NodeId(*id));
								//shader
								shader_id_commands.insert(new_draw_obj, ShaderKey(static_index.shader));
								// pipeline
								pipeline_state_commands.insert(new_draw_obj, PipelineKey(static_index.pipeline_state));
								// vertex_buffer_layout
								vertex_buffer_layout_commands.insert(new_draw_obj, VertexBufferLayoutKey(static_index.vertex_buffer_index));
								// fs defines - OPACITY
								let mut vs_defines = VSDefines::default();
								vs_defines.insert("VIEW".to_string());
								vs_defines_commands.insert(new_draw_obj, vs_defines);

								// 创建PostPprocess,并插入后处理列表中
								let post_process = PostProcess::new(
									new_draw_obj,
									POST_TEXTURE_GROUP,
									POST_UV_LOCATION,
									0,
									0,
								);
								post_list.0.insert(post_key, post_process);
							},
						};
					} else {
						// 如果不存在旋转，则计算裁剪区域（还要与父裁剪区域求相交）
						let aabb_temp;
						let aabb = match will_change {
							Some(r) => {
								aabb_temp = cal_no_rotate_box(&**quad, &r.will_change);
								&aabb_temp
							},
							None => &**quad,
						};

						let r = match parent_aabb {
							Some(parent_aabb) => match parent_aabb.aabb {
								Some(parent_aabb) => intersect(aabb, &parent_aabb),
								None => None,
							},
							None => intersect(aabb, aabb)
						};
						
						overflow_aabb.write(OverflowAabb {
							aabb: r,
							matrix: None,
						})
					}
				}
			}
		}

		local.clear();
	}

	#[listen(component=(Node, Overflow, (Create, Modify, Delete)))]
	pub fn overflow_change(
		e: Event,
		overflow: Query<Node, &Overflow>,
		render_mark: Query<Node, Write<RenderContextMark>>,
		mark_type: Res<OverflowRenderContextMarkType>,
	) {
		let mut render_context_mark_item = match render_mark.get_mut_by_entity(e.id) {
			Some(r) => r,
			// 正常情况不会进入该分支，除非e.id实体在Node中不存在
			None => return,
		};
		let overflow_item = overflow.get_by_entity(e.id);
		let mut render_mark_value = render_context_mark_item.get_or_default().clone();

		// Oveflow为true时，标记render_context_mark对应的位
		// Oveflow为false时, 取消render_context_mark对应的位，如果发现位标记全为空，则删除RenderContextMark组件
		match overflow_item {
			Some(overflow_item) if **overflow_item == true => {
				render_mark_value.set(***mark_type, true);
			},
			_ => {
				render_mark_value.set(***mark_type, false);
				// 如果所有的位标记都被清除，则调用remove方法
				if render_mark_value.not_any() {
					render_context_mark_item.remove();
					return;
				}
			},
		};

		render_context_mark_item.write(render_mark_value);
		
	}
}

// 非旋转矩阵计算包围盒
fn cal_no_rotate_box(aabb: &Aabb2, matrix: &Matrix4) -> Aabb2 {
	println!("====={:?}", matrix);
	let left_top = matrix * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
	let right_bottom = matrix * Vector4::new(aabb.maxs.x,  aabb.maxs.y, 0.0, 1.0);

	Aabb2::new(Point2::new(left_top.x, left_top.y), Point2::new(right_bottom.x, right_bottom.y))
}

fn calc_rotate_matrix(mut matrix: Matrix4) -> Matrix4 {
	let m = Matrix4::new(
		1.0, 0.0, 0.0, matrix[(0, 3)],
		0.0, 1.0, 0.0, matrix[(1, 3)],
		0.0, 0.0, 1.0, matrix[(2, 3)],
		0.0, 0.0, 0.0, matrix[(3, 3)],
	);
	let m_invert = Matrix4::new(
		1.0, 0.0, 0.0, -matrix[(0, 3)],
		0.0, 1.0, 0.0, -matrix[(1, 3)],
		0.0, 0.0, 1.0, -matrix[(2, 3)],
		0.0, 0.0, 0.0, matrix[(3, 3)],
	);

	let scale_x = Vector4::from(matrix.fixed_columns(0));
	let scale_y = Vector4::from(matrix.fixed_columns(1));
	let scale_x = scale_x.dot(&scale_x);
	let scale_y = scale_y.dot(&scale_y);

	println!("scale_x==={:?}, scale_y==={:?}, {:?}", scale_x, scale_y, matrix);
	if scale_x != 0.0 {
		matrix[(0, 0)] = matrix[(0, 0)]/scale_x;
		matrix[(1, 0)] = matrix[(1, 0)]/scale_x;
		matrix[(2, 0)] = matrix[(2, 0)]/scale_x;
	}

	if scale_y != 0.0 {
		matrix[(0, 1)] = matrix[(0, 1)]/scale_y;
		matrix[(1, 1)] = matrix[(1, 1)]/scale_y;
		matrix[(2, 1)] = matrix[(2, 1)]/scale_y;
	}

	matrix.set_column(3, &Vector4::new(0.0, 0.0, 0.0, 1.0));

	let invert =  matrix.try_inverse().unwrap();
	m * invert * m_invert
	// matrix
}

#[test]
fn test() {
// 	let mut transform = Transform::default();
// 	transform.funcs.push(TransformFunc::RotateZ(45.0));
// 	transform.funcs.push(TransformFunc::RotateZ(45.0));
// 	transform.funcs.push(TransformFunc::RotateZ(45.0));

// 	let m = WorldMatrix::form_transform(transform, 0.0,0.0)
}
