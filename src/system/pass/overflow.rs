use bevy::{ecs::{
    prelude::{Entity, Ref},
    query::With,
    system::Query,
}, prelude::DetectChanges};

use crate::{components::{pass_2d::{PostProcessList, ChildrenPass}, calc::{ViewBox, ContentBox}, user::Vector2}, system::utils::rotatequad_quad_intersection, resource::RenderContextMarkType, utils::tools::intersect_or_zero};

use pi_bevy_ecs_extend::{prelude::{Layer, Root, OrDefault}, system_param::res::OrInitRes};
use pi_postprocess::effect::copy::CopyIntensity;


use crate::{
    components::{
        calc::{LayoutResult, OveflowRotate, View, Quad, TransformWillChangeMatrix, WorldMatrix},
        user::{Aabb2, Matrix4, Overflow, Point2, Vector4},
    },
};

/// 采用全遍历的方式，每帧扫描所有pass2d，如果父上下文改变或自身改变，计算overflow
pub fn overflow_post_process(
	roots: Query<Entity, With<Root>>,
	mut pass_mut: Query<(
		&mut PostProcessList,
		&mut View,
	)>,
	pass_read: Query<(
		&'static WorldMatrix,
		&'static TransformWillChangeMatrix,
		&'static LayoutResult,
		&'static Quad,
		&'static ContentBox,
		&'static ChildrenPass,
		OrDefault<Overflow>,

		Ref<WorldMatrix>,
		Ref<TransformWillChangeMatrix>,
		Option<Ref<Overflow>>,
		Ref<Layer>
	)>,
	mark_type: OrInitRes<RenderContextMarkType<Overflow>>,
) {
	let max = 100000.0;
	let view = View {
		view_box: ViewBox { aabb: Aabb2::new(Point2::new(-max, -max), Point2::new(max, max)), quad: (Vector2::new(-max, -max), Vector2::new(-max, max), Vector2::new(max, max), Vector2::new(max, -max)) },
		matrix: None,
	};
	// 从根节点遍历， 修改OverflowAabb
	for root in roots.iter() {
		// log::warn!("root======{:?}", root);
		// log::warn!("recursive_cal_overflow======{:?}, {:?}", pass_mut.get_mut(root).is_ok(), pass_read1.get(root));

        recursive_cal_overflow(
			false,
			root,
			&view.view_box,
			&None,
			&mut pass_mut,
			&pass_read,
			&mark_type,
		);
	}

}

fn recursive_cal_overflow(
	parent_is_change: bool,
	id: Entity,
	parent_aabb: &ViewBox,
	context_matrix: &Option<OveflowRotate>,
	pass_mut: &mut Query<(
		&mut PostProcessList,
		&mut View,
	)>,
	pass_read: &Query<(
		&'static WorldMatrix,
		&'static TransformWillChangeMatrix,
		&'static LayoutResult,
		&'static Quad,
		&'static ContentBox,
		&'static ChildrenPass,
		OrDefault<Overflow>,

		Ref<WorldMatrix>,
		Ref<TransformWillChangeMatrix>,
		Option<Ref<Overflow>>,
		Ref<Layer>
	)>,
	mark_type: &RenderContextMarkType<Overflow>,
) {
	if let (
		Ok((mut post_list, mut oveflow_aabb)), 
		Ok((world_matrix, will_change, layout, quad, content_box, children, overflow, tracker_matrix, tracker_willchange, tracker_overflow, tracker_layer))
	) = (pass_mut.get_mut(id), pass_read.get(id)) {
		let overflow_is_change = match tracker_overflow {
			Some(r) => r.is_changed(),
			None => false
		};
		let is_change = parent_is_change || 
			tracker_matrix.is_changed() || 
			tracker_willchange.is_changed() || 
			overflow_is_change || 
			tracker_layer.is_changed();

		// log::warn!("is_change======{:?}, overflow: {:?}, entity: {:?}, \nparent_aabb: {:?}", is_change, overflow, entity, parent_aabb);
		if is_change {
			
			let matrix_temp;
			let matrix = match &will_change.0 {
				Some(r) => {
					matrix_temp = &r.will_change * world_matrix;
					&matrix_temp
				}
				None => world_matrix,
			};

			//
			if **overflow && matrix.1 {
				match &mut post_list.copy {
					Some(_r) => {}
					None => {
						post_list.copy = Some(CopyIntensity::default());
					}
				};
				post_list.effect_mark.set(**mark_type, true);
			} else {
				post_list.effect_mark.set(**mark_type, false);
			}

			if **overflow || post_list.has_effect() {

				if matrix.1 { // 如果矩阵含有旋转变换
					let (left, top, right, bottom) = if**overflow {
						(
							layout.border.left + layout.padding.left,
							layout.border.top + layout.padding.top,
							layout.rect.right - (layout.border.right + layout.padding.right) - layout.rect.left,
							layout.rect.bottom - (layout.border.top + layout.padding.top) - layout.rect.top
						)
					} else {
						(
							0.0,
							0.0,
							content_box.layout.maxs.x - content_box.layout.mins.x,
							content_box.layout.maxs.y - content_box.layout.mins.y,
						)
					};

					// log::warn!("content_box=====id: {:?}, {:?}, layout: {:?}, left: {}, top: {}, right: {}, bottom: {}", entity, content_box, layout, left, top, right, bottom);

					let left_top = matrix * Vector4::new(left, top, 0.0, 1.0);
					let left_bottom = matrix * Vector4::new(left, bottom, 0.0, 1.0);
					let right_bottom = matrix * Vector4::new(right, bottom, 0.0, 1.0);
					let right_top = matrix * Vector4::new(right, top, 0.0, 1.0);

					// 如果存在旋转，需要逆旋转渲染，然后对逆旋转的渲染结果进行后处理
					let world_rotate_invert = calc_rotate_matrix(matrix.0.clone());
					let world_rotate = world_rotate_invert.try_inverse().unwrap();

					let mut aabb = cal_no_rotate_box(&Aabb2::new(Point2::new(left_top.x, left_top.y), Point2::new(right_bottom.x, right_bottom.y)), &world_rotate_invert);

					// 可视区域是当前aabb与父的aabb相交得到
					aabb = rotatequad_quad_intersection(&parent_aabb.quad, &world_rotate_invert, &aabb);

					*oveflow_aabb = View {
						view_box: ViewBox {
							aabb,
							quad: (
								Vector2::new(left_top.x, left_top.y), 
								Vector2::new(left_bottom.x, left_bottom.y),
								Vector2::new(right_bottom.x, right_bottom.y),
								Vector2::new(right_top.x, right_top.y),
							)
						},
						matrix: Some(OveflowRotate {
							world_rotate_invert: world_rotate_invert.clone(),
							from_context_rotate: match context_matrix {
								Some(r) => r.world_rotate_invert * world_rotate,
								None => world_rotate,
							},
							world_rotate, // TODO
						}),
					};
				} else {
					let quad_temp;
					
					let quad = if **overflow {
						// 自身overflow为true，并且非旋转，overflow_aabb为父的aabb与本节点裁剪包围盒的交
						// 裁剪包围盒为内容部分，而非oct部分
						let (left, top) = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
						if left > 0.0 || top > 0.0 {
							let right = layout.rect.right - (layout.border.right + layout.padding.right);
							let bottom = layout.rect.bottom - (layout.border.top + layout.padding.top);
							quad_temp = cal_no_rotate_box(&Aabb2::new(Point2::new(left, top), Point2::new(right, bottom)), &world_matrix.0);
							&quad_temp
						} else {
							&**quad
						}
					} else {
						&content_box.oct
					};
		
					// 如果存在will_change， 则需要给包围盒乘上willchange，结果才是节点的真实裁剪框（坐标是相对世界原点）
					let aabb_temp;
					let quad = match &will_change.0 {
						Some(r) => {
							aabb_temp = cal_no_rotate_box(quad, &r.will_change);
							&aabb_temp
						}
						None => quad,
					};
					
					// 存在父裁剪框，则与父裁剪框相交
					let r = intersect_or_zero(&quad, &parent_aabb.aabb);

					*oveflow_aabb = View { matrix: None, view_box: ViewBox {
						aabb: r,
						quad: (
							Vector2::new(r.mins.x, r.mins.y),
							Vector2::new(r.mins.x, r.maxs.y),
							Vector2::new(r.maxs.x, r.maxs.y),
							Vector2::new(r.maxs.x, r.mins.y)
						),
					} };
				}
			} else {
				// 继承父上下文的视图
				// *oveflow_aabb = View {
				// 	view_box: parent_aabb.clone(),
				// 	matrix: None,
				// };
				*oveflow_aabb = View {
					view_box: parent_aabb.clone(),
					matrix: context_matrix.clone(),
				};
			}
		};
		
		// 存在子pass， 递归设置
		if children.len() > 0 {
			let context_rotate = if **overflow || post_list.has_effect() {
				oveflow_aabb.matrix.clone()
			} else {
				oveflow_aabb.matrix.clone()
			};

			let oveflow_aabb = oveflow_aabb.clone();
			for i in children.iter() {
				recursive_cal_overflow(
					is_change,
					**i,
					&oveflow_aabb.view_box,
					&context_rotate,
					pass_mut,
					pass_read,
					mark_type,
				);
			}
		}

		// if !**overflow {
		// 	// 如果父存在旋转， 则设置本节点的旋转为单位矩阵（旋转只在overflow节点上体现）
		// 	if let Some(m) = &mut oveflow_aabb.matrix {
		// 		let rotate_matrix_invert = calc_rotate_matrix(matrix.0.clone());
		// 		m.rotate_matrix = WorldMatrix::default().0;
		// 		m.all_rotate_invert = rotate_matrix_invert; // TODO
		// 	}
		// }
	}
}

// 非旋转矩阵计算包围盒
fn cal_no_rotate_box(aabb: &Aabb2, matrix: &Matrix4) -> Aabb2 {
    let left_top = matrix * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
    let right_bottom = matrix * Vector4::new(aabb.maxs.x, aabb.maxs.y, 0.0, 1.0);

    Aabb2::new(Point2::new(left_top.x, left_top.y), Point2::new(right_bottom.x, right_bottom.y))
}

fn calc_rotate_matrix(mut matrix: Matrix4) -> Matrix4 {
    let m = Matrix4::new(
        1.0,
        0.0,
        0.0,
        matrix[(0, 3)],
        0.0,
        1.0,
        0.0,
        matrix[(1, 3)],
        0.0,
        0.0,
        1.0,
        matrix[(2, 3)],
        0.0,
        0.0,
        0.0,
        matrix[(3, 3)],
    );
    let m_invert = Matrix4::new(
        1.0,
        0.0,
        0.0,
        -matrix[(0, 3)],
        0.0,
        1.0,
        0.0,
        -matrix[(1, 3)],
        0.0,
        0.0,
        1.0,
        -matrix[(2, 3)],
        0.0,
        0.0,
        0.0,
        matrix[(3, 3)],
    );

    let scale_x = Vector4::from(matrix.fixed_columns(0));
    let scale_y = Vector4::from(matrix.fixed_columns(1));
    let scale_x = scale_x.dot(&scale_x);
    let scale_y = scale_y.dot(&scale_y);

    if scale_x != 0.0 {
        matrix[(0, 0)] = matrix[(0, 0)] / scale_x;
        matrix[(1, 0)] = matrix[(1, 0)] / scale_x;
        matrix[(2, 0)] = matrix[(2, 0)] / scale_x;
    }

    if scale_y != 0.0 {
        matrix[(0, 1)] = matrix[(0, 1)] / scale_y;
        matrix[(1, 1)] = matrix[(1, 1)] / scale_y;
        matrix[(2, 1)] = matrix[(2, 1)] / scale_y;
    }

    matrix.set_column(3, &Vector4::new(0.0, 0.0, 0.0, 1.0));

    let invert = matrix.try_inverse().unwrap();
    m * invert * m_invert
    // matrix
}
