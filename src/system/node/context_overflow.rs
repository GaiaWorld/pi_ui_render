//! 处理overflow属性，
//! 1. 对overflow设置为true的节点，标记为渲染上下文（设置RenderContextMark中的位标记）
//! 2.

use pi_dirty::LayerDirty;
use pi_ecs::{
    entity::Id,
    monitor::Event,
    prelude::{ChangeTrackers, FromWorld, Join, Local, ParamSet, Query, Res, With, Write},
};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Layer;
use pi_postprocess::effect::copy::CopyIntensity;


use crate::{
    components::{
        calc::{LayoutResult, NodeId, OveflowRotate, OverflowAabb, Pass2DId, Quad, RenderContextMark, TransformWillChangeMatrix, WorldMatrix},
        pass_2d::{ParentPassId, Pass2D, PostProcessList},
        user::{Aabb2, Matrix4, Node, Overflow, Point2, Vector4},
    },
    resource::RenderContextMarkType,
    utils::tools::intersect,
};

pub struct CalcOverflow;

/// overflow 后处理的索引
#[derive(Deref)]
pub struct OverflowRenderContextMarkType(RenderContextMarkType);

impl FromWorld for OverflowRenderContextMarkType {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self { Self(RenderContextMarkType::from_world(world)) }
}

#[setup]
impl CalcOverflow {
    #[system]
    pub fn calc_overflow(
        // mark_type: Res<OverflowRenderContextMarkType>,
        query: Query<
            Node,
            (
                Id<Node>,
                &Pass2DId,
                &Layer<Node>,
                // transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
                // 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
                ChangeTrackers<WorldMatrix>,
                ChangeTrackers<TransformWillChangeMatrix>,
                ChangeTrackers<Overflow>,
                ChangeTrackers<Layer<Node>>,
            ),
            With<Overflow>,
        >,
        mut write: ParamSet<(
            Query<
                Pass2D,
                (
                    Write<OverflowAabb>,
                    Write<PostProcessList>,
                    Join<
                        NodeId,
                        Node,
                        (
                            &'static WorldMatrix,
                            Option<&'static TransformWillChangeMatrix>,
                            &'static LayoutResult,
                            &'static Quad,
                        ),
                    >,
                ),
            >,
            Query<Pass2D, Join<ParentPassId, Pass2D, (Id<Pass2D>, ChangeTrackers<OverflowAabb>, Option<&'static OverflowAabb>)>>,
        )>,
        mut local: Local<LayerDirty<(Id<Node>, Id<Pass2D>, bool)>>,
    ) {
        // 将overflow组织为层的形式，处理overflow是，从根开始处理（子节点会受父节点的影响）
        for (id, pass_id, layer, tracker_matrix, tracker_willchange, tracker_overflow, tracker_layer) in query.iter() {
            local.mark(
                (
                    id,
                    **pass_id,
                    tracker_overflow.is_changed() || tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed(),
                ),
                layer.layer(),
            );
        }

        for ((_id, pass_id, is_changed), _layer) in local.iter() {
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
                } else {
                    break;
                }
            }

            if parent_changed || *is_changed {
                if let Some((mut overflow_aabb, mut post_list, (matrix, will_change, layout, quad))) = write.p0_mut().get_mut(*pass_id) {
                    let matrix_temp;
                    let (matrix, is_rotation) = match will_change {
                        Some(r) => {
                            if matrix.1 || r.will_change.1 {
                                matrix_temp = &r.will_change * matrix;
                                (&matrix_temp, matrix.1)
                            } else {
                                (&r.will_change, false)
                            }
                        }
                        None => (matrix, matrix.1),
                    };
                    let left = layout.border.left + layout.padding.left;
                    let top = layout.border.top + layout.padding.top;
                    let right = layout.rect.right - (layout.border.right + layout.padding.right) - layout.rect.left;
                    let bottom = layout.rect.bottom - (layout.border.top + layout.padding.top) - layout.rect.top;

                    if is_rotation {
                        // 如果存在旋转，需要逆旋转渲染，然后对逆旋转的渲染结果进行后处理
                        let rotate_matrix_invert = calc_rotate_matrix(matrix.0.clone());
                        let rotate_matrix = rotate_matrix_invert.try_inverse().unwrap();

                        let aabb = cal_no_rotate_box(
                            &Aabb2::new(Point2::new(left, top), Point2::new(right, bottom)),
                            &(rotate_matrix_invert * matrix.0),
                        );

                        overflow_aabb.write(OverflowAabb {
                            aabb: Some(aabb),
                            matrix: Some(OveflowRotate {
                                rotate_matrix_invert,
                                rotate_matrix,
                            }),
                        });

                        let post_list = match post_list.get_mut() {
                            Some(r) => r,
                            None => {
                                post_list.write(PostProcessList::default());
                                post_list.get_mut().unwrap()
                            }
                        };
                        match &mut post_list.copy {
                            Some(_r) => {}
                            None => {
                                post_list.copy = Some(CopyIntensity::default());
                            }
                        };
                    } else {
                        let quad_temp;
                        let quad = if left > 0.0 || top > 0.0 {
                            quad_temp = cal_no_rotate_box(&Aabb2::new(Point2::new(left, top), Point2::new(right, bottom)), &matrix.0);
                            &quad_temp
                        } else {
                            &**quad
                        };

                        // 如果不存在旋转，则计算裁剪区域（还要与父裁剪区域求相交）
                        let aabb_temp;
                        let aabb = match will_change {
                            Some(r) => {
                                aabb_temp = cal_no_rotate_box(quad, &r.will_change);
                                &aabb_temp
                            }
                            None => quad,
                        };

                        let r = match parent_aabb {
                            Some(parent_aabb) => match parent_aabb.aabb {
                                Some(parent_aabb) => intersect(aabb, &parent_aabb),
                                None => None,
                            },
                            None => intersect(aabb, aabb),
                        };

                        overflow_aabb.write(OverflowAabb { aabb: r, matrix: None })
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
            }
            _ => {
                render_mark_value.set(***mark_type, false);
                // 如果所有的位标记都被清除，则调用remove方法
                if render_mark_value.not_any() {
                    render_context_mark_item.remove();
                    return;
                }
            }
        };

        render_context_mark_item.write(render_mark_value);
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

#[test]
fn test() {
    // 	let mut transform = Transform::default();
    // 	transform.funcs.push(TransformFunc::RotateZ(45.0));
    // 	transform.funcs.push(TransformFunc::RotateZ(45.0));
    // 	transform.funcs.push(TransformFunc::RotateZ(45.0));

    // 	let m = WorldMatrix::form_transform(transform, 0.0,0.0)
}
