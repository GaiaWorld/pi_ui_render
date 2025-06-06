//! 处理overflow属性，
//! 1. 对overflow设置为true的节点，标记为渲染上下文（设置RenderContextMark中的位标记）
//! 2.

use pi_style::style::StyleType;
use pi_world::prelude::{With, Query, Entity, Ticker};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Layer, Root};
use pi_world::single_res::SingleRes;

use crate::components::calc::RenderContextMark;
use crate::resource::{GlobalDirtyMark, IsRun};
use crate::system::base::pass::{content_box, pass_life};
use crate::system::system_set::UiSystemSet;
use crate::{components::calc::OverflowDesc, resource::RenderContextMarkType};

use crate::components::user::{AsImage, Overflow};


use crate::{
    components::{
        calc::{ContentBox, ViewBox},
        pass_2d::{ChildrenPass, PostProcess, PostProcessInfo},
        user::Vector2,
    },
    system::utils::rotatequad_quad_intersection,
    utils::tools::intersect_or_zero,
};

use pi_postprocess::effect::CopyIntensity;


use crate::components::{
    calc::{LayoutResult, OveflowRotate, Quad, TransformWillChangeMatrix, View, WorldMatrix},
    user::{Aabb2, Matrix4, Point2, Vector4},
};
use pi_world::prelude::{App, Plugin, IntoSystemConfigs};
use crate::prelude::UiStage;


pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(UiStage, pass_life::pass_mark::<Overflow>
                .in_set(UiSystemSet::PassMark)
                .run_if(overflow_change)
                .before(pass_life::cal_context))
            .add_system(UiStage, 
                overflow_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(content_box::calc_content_box)
                    .in_set(UiSystemSet::PassSetting))
        ;
    }
}

/// 采用全遍历的方式，每帧扫描所有pass2d，如果父上下文改变或自身改变，计算overflow(全遍历是否能优化？TODO)
pub fn overflow_post_process(
    roots: Query<Entity, With<Root>>,
    mut pass_mut: Query<(&mut PostProcess, &mut PostProcessInfo, &mut View)>,
    pass_read: Query<(
        Ticker<&WorldMatrix>,
        Ticker<&TransformWillChangeMatrix>,
        &'static LayoutResult,
        &'static Quad,
        &'static ContentBox,
        &'static ChildrenPass,
        Option<Ticker<&Overflow>>,
        Ticker<&'static Layer>,
        &'static RenderContextMark,
    )>,
    mark_type: OrInitSingleRes<RenderContextMarkType<Overflow>>,
    mark_type1: OrInitSingleRes<RenderContextMarkType<crate::components::user::Hsi>>,
    as_image_mark_type: OrInitSingleRes<RenderContextMarkType<AsImage>>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let max = 100000.0;
    let view = View {
        view_box: ViewBox {
            aabb: Aabb2::new(Point2::new(-max, -max), Point2::new(max, max)),
            world_quad: (
                Vector2::new(-max, -max),
                Vector2::new(-max, max),
                Vector2::new(max, max),
                Vector2::new(max, -max),
            ),
        },
        desc: OverflowDesc::NoRotate(Aabb2::new(Point2::new(-max, -max), Point2::new(max, max))),
    };
    // 从根节点遍历， 修改OverflowAabb
    for root in roots.iter() {
        // log::debug!("recursive_cal_overflow======{:?}, {:?}", pass_mut.get_mut(root).is_ok(), pass_read1.get(root));

        recursive_cal_overflow(
            false,
            root,
            &view.view_box,
            &view.desc,
            &mut pass_mut,
            &pass_read,
            &mark_type,
            &mark_type1,
            &as_image_mark_type,
        );
    }
}

pub fn overflow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::Overflow as usize).map_or(false, |display| {*display == true})
}

fn recursive_cal_overflow(
    parent_is_change: bool,
    id: Entity,
    parent_aabb: &ViewBox,
    context_desc: &OverflowDesc,
    pass_mut: &mut Query<(&mut PostProcess, &mut PostProcessInfo, &mut View)>,
    pass_read: &Query<(
        Ticker<&WorldMatrix>,
        Ticker<&TransformWillChangeMatrix>,
        &'static LayoutResult,
        &'static Quad,
        &'static ContentBox,
        &'static ChildrenPass,
        Option<Ticker<&Overflow>>,
        Ticker<&'static Layer>,
        &'static RenderContextMark,
    )>,
    mark_type: &RenderContextMarkType<Overflow>,
    mark_type1: &RenderContextMarkType<crate::components::user::Hsi>,
    as_image_mark_type: &RenderContextMarkType<AsImage>,
) {
    if let (
        Ok((mut post_list, mut post_info, mut oveflow_aabb)),
        Ok((
            world_matrix,
            will_change,
            layout,
            quad,
            content_box,
            children,
            overflow,
            layer,
            mark,
        )),
    ) = (pass_mut.get_mut(id), pass_read.get(id))
    {
        let (overflow_is_change, overflow) = match overflow {
            Some(r) => (r.is_changed(), **r),
            None => (false, false),
        };
        let is_change =
            parent_is_change || world_matrix.is_changed() || will_change.is_changed() || overflow_is_change || layer.is_changed();

        let world_matrix = &*world_matrix;
        log::debug!("is_change======id: {:?}, is_change:{:?}, overflow: {:?}, \nparent_aabb: {:?}, mark: {:?}", id, is_change, overflow, parent_aabb, mark.get(**mark_type1).map(|r| {*r.as_ref()}));
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
            if overflow && matrix.1 {
                match &mut post_list.copy {
                    Some(_r) => {}
                    None => {
                        post_list.copy = Some(CopyIntensity::default());
                    }
                };
                post_info.effect_mark.set(**mark_type, true);
            } else {
                post_info.effect_mark.set(**mark_type, false);

                if post_info.effect_mark.get(**as_image_mark_type).as_deref() != Some(&true) {
                    post_list.copy = None;
                }
            }

            // if **overflow { // || post_info.has_effect()
            if matrix.1 {
                // 如果矩阵含有旋转变换
                let (left, top, right, bottom) = if overflow {
                    // oveflow需要裁剪子节点到内容区域（注意，同时也将自身裁剪到内容区域，这与浏览器标准不符）
                    (
                        layout.border.left + layout.padding.left,
                        layout.border.top + layout.padding.top,
                        layout.rect.right - (layout.border.right + layout.padding.right) - layout.rect.left,
                        layout.rect.bottom - (layout.border.top + layout.padding.top) - layout.rect.top,
                    )
                } else {
                    // 如果子节点设有transform， 并且使得超出了本节点的布局范围会有问题（如何解决？TODO）
                    (
                        0.0,
                        0.0,
                        content_box.layout.maxs.x - content_box.layout.mins.x,
                        content_box.layout.maxs.y - content_box.layout.mins.y,
                    )
                };

                // log::debug!("content_box=====id: {:?}, {:?}, layout: {:?}, left: {}, top: {}, right: {}, bottom: {}", entity, content_box, layout, left, top, right, bottom);

                let left_top = matrix * Vector4::new(left, top, 0.0, 1.0);
                let left_bottom = matrix * Vector4::new(left, bottom, 0.0, 1.0);
                let right_bottom = matrix * Vector4::new(right, bottom, 0.0, 1.0);
                let right_top = matrix * Vector4::new(right, top, 0.0, 1.0);

                // 如果存在旋转，需要逆旋转渲染，然后对逆旋转的渲染结果进行后处理
                let world_rotate_invert = calc_rotate_matrix(matrix.0.clone());
                let world_rotate = world_rotate_invert.try_inverse().unwrap();

                let mut aabb = cal_no_rotate_box(
                    &Aabb2::new(Point2::new(left_top.x, left_top.y), Point2::new(right_bottom.x, right_bottom.y)),
                    &world_rotate_invert,
                );

                // 可视区域是当前aabb与父的aabb相交得到
                aabb = rotatequad_quad_intersection(&parent_aabb.world_quad, &world_rotate_invert, &aabb);

                *oveflow_aabb = View {
                    view_box: ViewBox {
                        aabb,
                        world_quad: (
                            Vector2::new(left_top.x, left_top.y),
                            Vector2::new(left_bottom.x, left_bottom.y),
                            Vector2::new(right_bottom.x, right_bottom.y),
                            Vector2::new(right_top.x, right_top.y),
                        ),
                    },
                    desc: OverflowDesc::Rotate(OveflowRotate {
                        world_rotate_invert: world_rotate_invert.clone(),
                        from_context_rotate: match context_desc {
                            OverflowDesc::Rotate(r) => WorldMatrix(r.world_rotate_invert * world_rotate, true),
                            OverflowDesc::NoRotate(_) => WorldMatrix(world_rotate, false),
                        },
                        world_rotate: WorldMatrix(world_rotate, true), // TODO
                    }),
                };
            } else {
                let quad_temp;

                let quad = if overflow {
                    // 自身overflow为true，并且非旋转，overflow_aabb为父的aabb与本节点裁剪包围盒的交
                    // 裁剪包围盒为内容部分，而非oct部分
                    let (left, top) = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
                    let (right, bottom) = (layout.border.right + layout.padding.right, layout.border.top + layout.padding.top);
                    if left > 0.0 || top > 0.0 || right > 0.0 || bottom > 0.0 {
                        let right = layout.rect.right - right;
                        let bottom = layout.rect.bottom - bottom;
                        // log::debug!("overflow0=========={:?}", (id, [left, top, right, bottom]));
                        quad_temp = cal_no_rotate_box(&Aabb2::new(Point2::new(left, top), Point2::new(right, bottom)), &world_matrix.0);
                        &quad_temp
                    } else {
                        // log::debug!("overflow1=========={:?}", (id, quad));
                        &**quad
                    }
                } else {
                    &content_box.oct
                    // 
                    // if eq_f32(content_box.oct.mins.x, quad.mins.x) &&
                    // eq_f32(content_box.oct.mins.y, quad.mins.y) &&
                    // eq_f32(content_box.oct.maxs.x, quad.maxs.x) &&
                    // eq_f32(content_box.oct.maxs.x, quad.maxs.y) {
                    //     // 如果content_box.layout与当前节点的content_box.layout完全一致，可直接使用quad
                    //     &**quad
                    // } else {
                    //     // 否则计算出 content_box.layout对应的包围盒
                    //     quad_temp = cal_no_rotate_box(&content_box.layout, &world_matrix.0);
                    //     &quad_temp
                    // }
                };
                

                // 如果存在will_change， 则需要给包围盒乘上willchange，结果才是节点的真实裁剪框（坐标是相对世界原点）
                let aabb_temp;
                let quad1 = match &will_change.0 {
                    Some(r) => {
                        aabb_temp = cal_no_rotate_box(quad, &r.will_change);
                        &aabb_temp
                    }
                    None => quad,
                };

                // 存在父裁剪框，则与父裁剪框相交
                let r = intersect_or_zero(&quad1, &parent_aabb.aabb);
				// log::debug!("is_change======tracker_matrix: {:?}, tracker_willchange: {:?}, overflow: {:?}, entity: {:?}, \nparent_aabb: {:?}, willchange: {:?}, \nmatrix: {:?}", tracker_matrix.is_changed(),tracker_willchange.is_changed(), overflow, entity, parent_aabb, tracker_willchange, tracker_matrix);
                // use pi_key_alloter::Key;
                // if id.index() == 4 {
                    // log::debug!("overflow================id:{:?}, \nr:{:?}, \nlayout: {:?}, \nwill_change:{:?}, \nmatrix: {:?}, \nparent_aabb: {:?}, \nquad: {:?}, \nquad1: {:?}", 
                    // id, &r, &content_box.layout, &will_change.0, &world_matrix, &parent_aabb.aabb, quad, quad1);
                // }
                log::debug!("overflow4=========={:?}", (id, &quad1, &parent_aabb.aabb, &r));
                
                *oveflow_aabb = View {
                    desc: OverflowDesc::NoRotate(quad1.clone()),
                    view_box: ViewBox {
                        aabb: r,
                        world_quad: (
                            Vector2::new(r.mins.x, r.mins.y),
                            Vector2::new(r.mins.x, r.maxs.y),
                            Vector2::new(r.maxs.x, r.maxs.y),
                            Vector2::new(r.maxs.x, r.mins.y),
                        ),
                    },
                };
            }
            // } else {
            //     // 继承父上下文的视图
            //     // *oveflow_aabb = View {
            //     // 	view_box: parent_aabb.clone(),
            //     // 	matrix: None,
            //     // };
            //     *oveflow_aabb = View {
            //         view_box: parent_aabb.clone(),
            //         matrix: context_matrix.clone(),
            //     };
            // }
        };

        // 存在子pass， 递归设置
        if children.len() > 0 {
            let context_rotate = oveflow_aabb.desc.clone();

            let view_box = oveflow_aabb.view_box.clone();
            for i in children.iter() {
                recursive_cal_overflow(
                    is_change,
                    **i,
                    &view_box,
                    &context_rotate,
                    pass_mut,
                    pass_read,
                    mark_type,
                    mark_type1,
                    as_image_mark_type,
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
    // let m1 = matrix.clone();
    // let m = Matrix4::new(
    //     1.0,
    //     0.0,
    //     0.0,
    //     matrix[(0, 3)],
    //     0.0,
    //     1.0,
    //     0.0,
    //     matrix[(1, 3)],
    //     0.0,
    //     0.0,
    //     1.0,
    //     matrix[(2, 3)],
    //     0.0,
    //     0.0,
    //     0.0,
    //     matrix[(3, 3)],
    // );
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
    let scale_z = Vector4::from(matrix.fixed_columns(2));
    // log::debug!("scale_x================{:?}", scale_x);
    let scale_x = scale_x.norm();
    let scale_y = scale_y.norm();
    let scale_z = scale_z.norm();

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
    if scale_z != 0.0 {
        matrix[(0, 2)] = matrix[(0, 2)] / scale_z;
        matrix[(1, 2)] = matrix[(1, 2)] / scale_z;
        matrix[(2, 2)] = matrix[(2, 2)] / scale_z;
    }

    matrix.set_column(3, &Vector4::new(0.0, 0.0, 0.0, 1.0));

    let invert = match matrix.try_inverse() {
        Some(m) => m,
        None => return WorldMatrix::default().0, // 没有逆矩阵， 则返回单位阵（没有逆矩阵时， 空间被压缩， 2d界面实际上不会显示， 因此此矩阵为任何矩阵都无所谓）
    };
    // log::debug!("zz0================{:?}, \nscalex: {}, \nscaley: {:?}, \ninvert: {:?}", m1, scale_x, scale_y, invert);
    // invert
    // m * invert * m_invert
    // 之所以乘以m_invert， 是为了保持每次回到非旋转状态的原点的一致性（否则，在做fbo缓存时， 不能确定脏区域相对于原fbo的位置）
    invert * m_invert
    // matrix
}
