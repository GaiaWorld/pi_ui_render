use pi_world::{filter::Or, prelude::{App, Changed, ComponentRemoved, Has, IntoSystemConfigs, Plugin, Query}, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_flex_layout::prelude::Rect;
use pi_style::style::{Aabb2, BaseShape, LengthUnit, StyleType};

use crate::{
    components::{
        calc::{ContentBox, LayoutResult, OverflowDesc, Quad, TransformWillChangeMatrix, View},
        pass_2d::{Camera, RenderTarget, WorldMatrixInvert},
        user::{ClipPath, Point2, Vector4},
    }, resource::{GlobalDirtyMark, IsRun}, system::{base::{
        // node::user_setting::user_setting,
        // pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
        node::user_setting::user_setting2, pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera, pass_life, world_invert::calc_world_invert},
    }, system_set::UiSystemSet}, utils::tools::{cal_border_radius, eq_f32}
};
use pi_postprocess::prelude::ClipSdf;

use crate::components::pass_2d::PostProcess;
use crate::prelude::UiStage;

pub struct UiClipPathPlugin;

impl Plugin for UiClipPathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UiStage, 
            pass_life::pass_mark::<ClipPath>
                .after(user_setting2)
                .before(pass_life::cal_context)
                .run_if(clip_change)
                .in_set(UiSystemSet::IsRun)
                // ,
        )
        .add_system(UiStage, 
            clip_path_del
                .after(user_setting2)
                .in_set(UiSystemSet::IsRun)
                // 
            )
        .add_system(UiStage, 
            clip_path_post_process
                .before(last_update_wgpu)
                .after(calc_camera)
                .after(calc_world_invert)
                .in_set(UiSystemSet::IsRun)
                // ,
        );
    }
}

// 处理ClipPath的删除
pub fn clip_path_del( 
	mut query: Query<(&mut PostProcess, Has<ClipPath>)>,
    remove: ComponentRemoved<ClipPath>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    if remove.len() == 0 {
        return;
    }
    for i in remove.iter() {
        if let Ok((mut post_list, has_clip )) = query.get_mut(*i) {
            if has_clip {
                continue
            }
            post_list.clip_sdf = None;
        }
    }
    
}

/// 处理ClipPath属性
pub fn clip_path_post_process(
    mut query: Query<
        (
            pi_world::world::Entity,
            &ClipPath,
            &LayoutResult,
            &ContentBox,
            &Quad,
            &View,
            &Camera,
            &RenderTarget,
            &mut PostProcess,
            &WorldMatrixInvert,
            &TransformWillChangeMatrix,
        ),
        Or<(
            Changed<ClipPath>,
            Changed<LayoutResult>,
            Changed<ContentBox>,
            Changed<Camera>,
        )>,
    >,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    for (entity, clip_path, layout, content_box, quad, view, camera, render_target, mut post, world_matrix_invert, will_change_matrix) in query.iter_mut() {
        if !camera.is_render_own {
            continue;
        }
        // 节点可视区域（没有与父裁剪区域相交的部分， 可能是节点的ContentBox，也可能是布局的内容区域）
        let (view_aabb, is_rotate) = match &view.desc {
            OverflowDesc::Rotate(_) => (&view.view_box.aabb, true),
            OverflowDesc::NoRotate(r) => (r, false),
        };
        let (w, h) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
        let v_w1 = view_aabb.maxs.x - view_aabb.mins.x;
        let v_h1 = view_aabb.maxs.y - view_aabb.mins.y;
        // view_aabb表示的布局范围
        let view_aabb_layout = if eq_f32(content_box.oct.mins.x, quad.mins.x) &&
        eq_f32(content_box.oct.mins.y, quad.mins.y) &&
        eq_f32(content_box.oct.maxs.x, quad.maxs.x) &&
        eq_f32(content_box.oct.maxs.x, quad.maxs.y) {
            // 如果content_box.oct与当前节点的content_box.layout完全一致，说明content_box.oct对应的范围与layout的范围一致， 直接返回layout
            Aabb2::new(
                Point2::new(layout.border.left + layout.padding.left, layout.border.top + layout.padding.top),
                Point2::new(
                    layout.rect.right - layout.border.left - layout.padding.left,
                    layout.rect.bottom - layout.border.top - layout.padding.top,
                ),
            )
        } else if is_rotate {
            todo!()
            // TODO
            // Aabb2::new(
            //     Point2::new(content_box.layout.mins.x - layout.rect.left, content_box.layout.mins.y - layout.rect.top),
            //     Point2::new(
            //         content_box.layout.maxs.x - layout.rect.left,
            //         content_box.layout.maxs.y - layout.rect.top,
            //     ),
            // )

        } else if let TransformWillChangeMatrix(Some(will_change_matrix)) = will_change_matrix {
            // 存在transform时， 用will_change_invert反乘，计算出 节点可视区域在布局坐标系中的表示 
            let will_change_matrix = &will_change_matrix.will_change_invert;
            let left_top = will_change_matrix * Vector4::new(view_aabb.mins.x, view_aabb.mins.y, 0.0, 1.0);
            let right_bottom = will_change_matrix * Vector4::new(view_aabb.maxs.x, view_aabb.maxs.y, 0.0, 1.0);
            Aabb2::new(
                Point2::new(left_top.x, left_top.y),
                Point2::new(right_bottom.x, right_bottom.y),
            )
        } else if let Some(world_matrix_invert) = &world_matrix_invert.value {
            // 否则用world_matrix_invert反乘， 计算出 节点可视区域在布局坐标系中的表示
            // TODO, world_matrix_invert应该为transform_willchange_matrix的逆矩阵
            let left_top = world_matrix_invert * Vector4::new(view_aabb.mins.x, view_aabb.mins.y, 0.0, 1.0);
            let right_bottom = world_matrix_invert * Vector4::new(view_aabb.maxs.x, view_aabb.maxs.y, 0.0, 1.0);
            Aabb2::new(
                Point2::new(left_top.x, left_top.y),
                Point2::new(right_bottom.x, right_bottom.y),
            )
        } else {
            // worldmatrix不可逆, 大小为0
            Aabb2::new(
                Point2::new(0.0, 0.0),
                Point2::new(0.0, 0.0),
            )
        };
        let h_ratio = (view_aabb_layout.maxs.x - view_aabb_layout.mins.x) / v_w1;
        let v_ratio = (view_aabb_layout.maxs.y - view_aabb_layout.mins.y) / v_h1;

        let context_rect = (
            (camera.view_port.maxs.x - camera.view_port.mins.x) * h_ratio, // width
            (camera.view_port.maxs.y - camera.view_port.mins.y) * v_ratio, // height
            (camera.view_port.mins.x - view_aabb.mins.x) * h_ratio + view_aabb_layout.mins.x, // x
            (camera.view_port.mins.y - view_aabb.mins.y) * v_ratio + view_aabb_layout.mins.y, // y
        );


        let clip_sdf = match &clip_path.0 {
            BaseShape::Circle { radius, center } => {
                let s = f32::sqrt(w * w + h * h) / f32::sqrt(2.0);
                ClipSdf::circle((len_value(&center.x, w), len_value(&center.y, h)), len_value(radius, s), context_rect)
            }
            BaseShape::Ellipse { rx, ry, center } => ClipSdf::ellipse(
                (len_value(&center.x, w), len_value(&center.y, h)),
                len_value(rx, w),
                len_value(ry, h),
                context_rect,
            ),
            BaseShape::Inset { rect_box, border_radius } => {
                let mut rect = Rect {
                    left: len_value(&rect_box[0], h),
                    right: w - len_value(&rect_box[1], w),
                    top: len_value(&rect_box[2], h),
                    bottom: h - len_value(&rect_box[3], w),
                };
                if rect.bottom < rect.top {
                    rect.bottom = rect.top;
                }
                if rect.right < rect.left {
                    rect.right = rect.left;
                }
                let (width, height) = (rect.right - rect.left, rect.bottom - rect.top);

                let border_radius = cal_border_radius(&border_radius, &rect);
                // log::warn!("clip0================{:?}", (entity, (v_w1, v_h1, ), &view_aabb_layout, &view_aabb, quad, &camera.view_port, layout, content_box));

                if border_radius.x[0] <= 0.0
                    && border_radius.x[1] <= 0.0
                    && border_radius.x[2] <= 0.0
                    && border_radius.x[3] <= 0.0
                    && border_radius.y[0] <= 0.0
                    && border_radius.y[1] <= 0.0
                    && border_radius.y[2] <= 0.0
                    && border_radius.y[3] <= 0.0
                {
                    // log::warn!("clip1============{:?}", (&rect_box, &border_radius, width, height, &context_rect, (
                    //     (rect.left + width / 2.0, rect.top + height / 2.0),
                    // width / 2.0,
                    // height / 2.0,
                    // context_rect,)));
                    ClipSdf::rect(
                        (rect.left + width / 2.0, rect.top + height / 2.0),
                        width / 2.0,
                        height / 2.0,
                        context_rect,
                    )
                } else {
                    // log::warn!("clip2============{:?}", ( &rect_box, &border_radius, width, height, &context_rect, (
                    //     (rect.left + width / 2.0, rect.top + height / 2.0),
                    // width,
                    // height,
                    // &border_radius.x,
                    // &border_radius.y,
                    // context_rect)));
                    ClipSdf::border_radius(
                        (rect.left + width / 2.0, rect.top + height / 2.0),
                        width,
                        height,
                        &border_radius.x,
                        &border_radius.y,
                        context_rect,
                    )
                }
            }
            BaseShape::Sector {
                rotate,
                angle,
                radius,
                center,
            } => {
                let half_angle = angle / 2.0;
                let half_rotate = rotate + half_angle;
                let s = f32::sqrt(w * w + h * h) / f32::sqrt(2.0);
                // log::warn!("context_rect============={:?}", context_rect);
                ClipSdf::sector(
                    (len_value(&center.x, w), len_value(&center.y, h)),
                    len_value(radius, s),
                    (half_rotate.sin(), half_rotate.cos()),
                    (half_angle.sin(), half_angle.cos()),
                    context_rect,
                )
            }
        };
        log::debug!("clip-path=======entity: {:?}, \nclip_path: {:?}, \nview_port: {:?}, \nview_aabb_layout: {:?}, \nview_aabb: {:?}, 
        \nclip_sdf: {:?}, \nbound_box: {:?}, \ncontent_box: {:?}, \nquad: {:?}, \nlayout: {:?}", 
            entity, 
            &clip_path.0,
            &camera.view_port, 
            &view_aabb_layout, 
            &view_aabb,
            &clip_sdf,
            &render_target.bound_box,
            &content_box.oct,
            &quad,
            &layout,
        );
        post.clip_sdf = Some(clip_sdf)
    }
}


fn len_value(v: &LengthUnit, c: f32) -> f32 {
    match v {
        LengthUnit::Pixel(r) => *r,
        LengthUnit::Percent(r) => r * c,
    }
}

pub fn clip_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::ClipPath as usize).map_or(false, |display| {*display == true})
}
