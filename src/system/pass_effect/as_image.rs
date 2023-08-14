use bevy::{
    ecs::{prelude::RemovedComponents, query::Changed, system::Query},
    prelude::{Added, IntoSystemConfig, Or, ParamSet, Plugin},
};
use pi_assets::homogeneous::HomogeneousMgr;
use pi_bevy_asset::{AssetConfig, AssetDesc, ShareHomogeneousMgr};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

use crate::{
    components::{
        pass_2d::{CacheTarget, PostProcessInfo},
        user::{AsImage, Overflow},
    },
    resource::RenderContextMarkType,
    system::{
        node::user_setting::user_setting,
        pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
        render_run,
    },
};
use pi_postprocess::prelude::CopyIntensity;

use crate::{components::pass_2d::PostProcess, system::pass::pass_life};

pub struct UiAsImagePlugin;

impl Plugin for UiAsImagePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let assets_mgr = {
            let w = app.world.cell();
            let asset_config = w.get_resource::<AssetConfig>().unwrap();
            let default_cfg = AssetDesc {
                ref_garbage: false,
                min: 0,
                max: 32 * 1024 * 1024, // 默认32M的fbo缓存
                timeout: 0,            // 并不会启用超时整理， 这里的数值无所谓（记得该资源管理器中的资源需要手动删除）
            };
            let desc = asset_config.get::<CacheTarget>().unwrap_or(&default_cfg);
            HomogeneousMgr::<CacheTarget>::new(pi_assets::homogeneous::GarbageEmpty(), desc.min, desc.timeout)
        };

        app.insert_resource(ShareHomogeneousMgr(assets_mgr))
            // 标记有AsImage组件的节点为渲染上下文
            .add_system(
                pass_life::pass_mark::<AsImage>
                    .after(user_setting)
                    .before(pass_life::cal_context)
                    .run_if(render_run),
            )
            // // 处理AsImage组件的删除逻辑
            // .add_system(
            // 	as_image_del
            // 		.after(user_setting)
            // 		.run_if(render_run)
            // )
            .add_system(
                as_image_post_process
                    .before(last_update_wgpu)
                    .after(calc_camera_depth_and_renderlist)
                    .run_if(render_run),
            );
    }
}

/// 处理AsImage组件
/// 如果Opacity删除， 设置PostProcessList的alpha属性为None
/// 如果Opacity修改， 设置PostProcessList的alpha属性为对应值
pub fn as_image_post_process(
    mut del: RemovedComponents<AsImage>,
    mark_type: OrInitRes<RenderContextMarkType<AsImage>>,
    overflow_mark_type: OrInitRes<RenderContextMarkType<Overflow>>,
    mut query: ParamSet<(
        Query<(&AsImage, &mut PostProcess, &mut PostProcessInfo), Or<(Changed<AsImage>, Added<PostProcess>)>>,
        Query<(&mut PostProcess, &mut PostProcessInfo)>,
    )>,
) {
    // AsImage 如果删除， 取消AsImage的后处理
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok((mut post_list, mut post_info)) = p1.get_mut(del) {
            post_info.effect_mark.set(***mark_type, false);

            let mut effect_mark = post_info.effect_mark.clone();
            effect_mark.set(***overflow_mark_type, false);
            if post_info.effect_mark.get(***overflow_mark_type).as_deref() != Some(&true) {
                post_list.copy = None;
            }
        }
    }

    for (as_image, mut post_list, mut post_info) in query.p0().iter_mut() {
        match as_image.0 {
            pi_style::style::AsImage::None => {
                post_info.effect_mark.set(***mark_type, false);
                if post_info.effect_mark.get(***overflow_mark_type).as_deref() != Some(&true) {
                    post_list.copy = None;
                }
            }
            _ => {
				log::warn!("as_image============");
                post_info.effect_mark.set(***mark_type, true);
                post_list.copy = Some(CopyIntensity::default());
            }
        }
    }
}

// // 处理ClipPath的删除
// pub fn as_image_del(
//     mut del: RemovedComponents<AsImage>,
//     mut query: Query<&mut PostProcess, Without<ClipPath>>,
// ) {
//     for del in del.iter() {
//         if let Ok(mut post_list) = query.get_mut(del) {
//             post_list.clip_sdf = None;
//         }
//     }
// }

// /// 处理ClipPath属性
// /// 如果hsi删除，设置PostProcess中的hsb位None
// /// 如果hsi修改，将其设置在PostProcess中
// pub fn as_image_post_process(
//     mut query: Query<(&ClipPath, &LayoutResult, &ContentBox, OrDefault<Overflow>, &View, &Camera, &mut PostProcess), Or<(Changed<ClipPath>, Added<PostProcess>, Changed<LayoutResult>, Changed<ContentBox>, Changed<Camera>)>>,
// ) {
//     for (clip_path, layout, content_box, overflow, view, camera, mut post) in query.iter_mut() {
// 		if !camera.is_active {
// 			continue;
// 		}
// 		// 节点可视区域（没有与父裁剪区域相交的部分， 可能是节点的ContentBox，也可能是布局的内容区域）
// 		let view_aabb = match &view.desc {
// 			OverflowDesc::Rotate(_) => &view.view_box.aabb,
// 			OverflowDesc::NoRotate(r) => r,
// 		};
// 		let (w, h) = (
// 			layout.rect.right - layout.rect.left,
// 			layout.rect.bottom - layout.rect.top,
// 		);
// 		let v_w1 = view_aabb.maxs.x - view_aabb.mins.x;
// 		let v_h1 = view_aabb.maxs.y - view_aabb.mins.y;
// 		// view_aabb表示的布局范围
// 		let view_aabb_layout = if overflow.0 {
// 			Aabb2::new(Point2::new(
// 				layout.border.left + layout.padding.left,
// 				layout.border.top + layout.padding.top,

// 			), Point2::new(
// 				layout.rect.right - layout.border.left - layout.padding.left,
// 				layout.rect.bottom - layout.border.top - layout.padding.top,
// 			))
// 		} else {
// 			Aabb2::new(Point2::new(
// 				content_box.layout.mins.x - layout.rect.left,
// 				content_box.layout.mins.y - layout.rect.top,

// 			), Point2::new(
// 				content_box.layout.maxs.x - content_box.layout.mins.x,
// 				content_box.layout.maxs.y - content_box.layout.mins.y,
// 			))

// 			// &content_box.layout
// 		};
// 		let h_ratio = (view_aabb_layout.maxs.x - view_aabb_layout.mins.x) / v_w1;
// 		let v_ratio = (view_aabb_layout.maxs.y - view_aabb_layout.mins.y) / v_h1;

// 		let context_rect = (
// 			(camera.view_port.maxs.x - camera.view_port.mins.x) * v_ratio,
// 			(camera.view_port.maxs.y - camera.view_port.mins.y) * v_ratio,
// 			(camera.view_port.mins.x - view_aabb.mins.x) * h_ratio + view_aabb_layout.mins.x,
// 			(camera.view_port.mins.y - view_aabb.mins.y) * v_ratio + view_aabb_layout.mins.y,

// 		);


// 		let clip_sdf = match &clip_path.0 {
// 			BaseShape::Circle { radius, center } => {
// 				let s = f32::sqrt(w * w + h * h)/f32::sqrt(2.0);
// 				ClipSdf::circle((len_value(&center.x, w), len_value(&center.y, h)), len_value(radius, s) , context_rect)
// 			},
// 			BaseShape::Ellipse { rx, ry, center } => {
// 				ClipSdf::ellipse((len_value(&center.x, w), len_value(&center.y, h)), len_value(rx, w), len_value(ry, h), context_rect)
// 			},
// 			BaseShape::Inset { rect_box, border_radius } => {
// 				let mut rect = Rect {
// 					left: len_value(&rect_box[0], h),
// 					right: w - len_value(&rect_box[1], w),
// 					top: len_value(&rect_box[2], h),
// 					bottom: h - len_value(&rect_box[3], w),
// 				};
// 				if rect.bottom < rect.top {
// 					rect.bottom = rect.top;
// 				}
// 				if rect.right < rect.left {
// 					rect.right = rect.left;
// 				}
// 				let (width, height)  = (rect.right - rect.left, rect.bottom - rect.top);

// 				let border_radius = cal_border_radius(&border_radius, &rect);

// 				if border_radius.x[0] <= 0.0 && border_radius.x[1] <= 0.0 && border_radius.x[2] <= 0.0 && border_radius.x[3] <= 0.0 &&
// 					border_radius.y[0] <= 0.0 && border_radius.y[1] <= 0.0 && border_radius.y[2] <= 0.0 && border_radius.y[3] <= 0.0 {
// 					ClipSdf::rect((rect.left + width/2.0, rect.top + height/2.0), width/2.0, height/2.0, context_rect)
// 				} else {
// 					ClipSdf::border_radius((rect.left + width/2.0, rect.top + height/2.0), width, height, &border_radius.x, &border_radius.y, context_rect)
// 				}
// 			},
// 			BaseShape::Sector { rotate, angle, radius, center } => {
// 				let half_angle = angle / 2.0;
// 				let half_rotate = rotate + half_angle;
// 				let s = f32::sqrt(w * w + h * h)/f32::sqrt(2.0);
// 				// log::warn!("context_rect============={:?}", context_rect);
// 				ClipSdf::sector((len_value(&center.x, w), len_value(&center.y, h)), len_value(radius, s), (half_rotate.sin(), half_rotate.cos()), (half_angle.sin(), half_angle.cos()), context_rect)
// 			}
// 		};
// 		post.clip_sdf = Some(clip_sdf)
//     }
// }
