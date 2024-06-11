use pi_world::{filter::Or, prelude::{App, Changed, ComponentRemoved, Has, IntoSystemConfigs, OrDefault, Plugin, Query}};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_flex_layout::prelude::Rect;
use pi_style::style::{Aabb2, BaseShape, LengthUnit};

use crate::{
    components::{
        calc::{ContentBox, LayoutResult, OverflowDesc, View},
        pass_2d::Camera,
        user::{ClipPath, Overflow, Point2},
    },
    system::{
        // node::user_setting::user_setting,
        // pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
        draw_obj::calc_text::IsRun, node::user_setting::user_setting2, pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
    },
    utils::tools::cal_border_radius,
};
use pi_postprocess::prelude::ClipSdf;

use crate::{components::pass_2d::PostProcess, system::pass::pass_life};
use crate::prelude::UiStage;

pub struct UiClipPathPlugin;

impl Plugin for UiClipPathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UiStage, 
            pass_life::pass_mark::<ClipPath>
                .after(user_setting2)
                .before(pass_life::cal_context)
                // ,
        )
        .add_system(UiStage, 
            clip_path_del
                .after(user_setting2)
                // 
            )
        .add_system(UiStage, 
            clip_path_post_process
                .before(last_update_wgpu)
                .after(calc_camera_depth_and_renderlist)
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
            &ClipPath,
            &LayoutResult,
            &ContentBox,
            OrDefault<Overflow>,
            &View,
            &Camera,
            &mut PostProcess,
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
    for (clip_path, layout, content_box, overflow, view, camera, mut post) in query.iter_mut() {
        if !camera.is_active {
            continue;
        }
        // 节点可视区域（没有与父裁剪区域相交的部分， 可能是节点的ContentBox，也可能是布局的内容区域）
        let view_aabb = match &view.desc {
            OverflowDesc::Rotate(_) => &view.view_box.aabb,
            OverflowDesc::NoRotate(r) => r,
        };
        let (w, h) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
        let v_w1 = view_aabb.maxs.x - view_aabb.mins.x;
        let v_h1 = view_aabb.maxs.y - view_aabb.mins.y;
        // view_aabb表示的布局范围
        let view_aabb_layout = if overflow.0 {
            Aabb2::new(
                Point2::new(layout.border.left + layout.padding.left, layout.border.top + layout.padding.top),
                Point2::new(
                    layout.rect.right - layout.border.left - layout.padding.left,
                    layout.rect.bottom - layout.border.top - layout.padding.top,
                ),
            )
        } else {
            Aabb2::new(
                Point2::new(content_box.layout.mins.x - layout.rect.left, content_box.layout.mins.y - layout.rect.top),
                Point2::new(
                    content_box.layout.maxs.x - content_box.layout.mins.x,
                    content_box.layout.maxs.y - content_box.layout.mins.y,
                ),
            )

            // &content_box.layout
        };
        let h_ratio = (view_aabb_layout.maxs.x - view_aabb_layout.mins.x) / v_w1;
        let v_ratio = (view_aabb_layout.maxs.y - view_aabb_layout.mins.y) / v_h1;

        let context_rect = (
            (camera.view_port.maxs.x - camera.view_port.mins.x) * v_ratio,
            (camera.view_port.maxs.y - camera.view_port.mins.y) * v_ratio,
            (camera.view_port.mins.x - view_aabb.mins.x) * h_ratio + view_aabb_layout.mins.x,
            (camera.view_port.mins.y - view_aabb.mins.y) * v_ratio + view_aabb_layout.mins.y,
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

                if border_radius.x[0] <= 0.0
                    && border_radius.x[1] <= 0.0
                    && border_radius.x[2] <= 0.0
                    && border_radius.x[3] <= 0.0
                    && border_radius.y[0] <= 0.0
                    && border_radius.y[1] <= 0.0
                    && border_radius.y[2] <= 0.0
                    && border_radius.y[3] <= 0.0
                {
                    ClipSdf::rect(
                        (rect.left + width / 2.0, rect.top + height / 2.0),
                        width / 2.0,
                        height / 2.0,
                        context_rect,
                    )
                } else {
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
        post.clip_sdf = Some(clip_sdf)
    }
}


fn len_value(v: &LengthUnit, c: f32) -> f32 {
    match v {
        LengthUnit::Pixel(r) => *r,
        LengthUnit::Percent(r) => r * c,
    }
}
