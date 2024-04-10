use std::sync::atomic::AtomicUsize;

use bevy_ecs::{
    prelude::RemovedComponents, query::Changed, system::Query,
    prelude::{Or, ParamSet, IntoSystemConfigs},
};
use bevy_app::{Plugin, Update, App};
use pi_bevy_asset::{AssetConfig, AssetDesc, ShareAssetMgr};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::FrameDataPrepare;
use pi_null::Null;

use crate::{
    components::{
        pass_2d::{CacheTarget, PostProcessInfo},
        user::{AsImage, Overflow},
    },
    resource::{RenderContextMarkType, draw_obj::TargetCacheMgr},
    system::{
        node::user_setting::user_setting,
        pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
        draw_obj::calc_text::IsRun,
    },
};
use pi_postprocess::prelude::CopyIntensity;

use crate::{components::pass_2d::PostProcess, system::pass::pass_life};

pub struct UiAsImagePlugin;

impl Plugin for UiAsImagePlugin {
    fn build(&self, app: &mut App) {
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
            ShareAssetMgr::<CacheTarget>::new(pi_assets::asset::GarbageEmpty(), false, desc.max, desc.timeout)
        };

        app.insert_resource(TargetCacheMgr { key: AtomicUsize::new(0), assets: assets_mgr })
            // 标记有AsImage组件的节点为渲染上下文
            .add_systems(Update, 
                pass_life::pass_mark::<AsImage>
                    .after(user_setting)
                    .before(pass_life::cal_context)
                    .in_set(FrameDataPrepare),
            )
            .add_systems(Update, 
                as_image_post_process
                    .before(last_update_wgpu)
                    .after(calc_camera_depth_and_renderlist)
                    .in_set(FrameDataPrepare),
            );
    }
}

/// 处理AsImage组件
/// 如果AsImage删除， 设置PostProcessList的copy属性为None
/// 如果AsImage修改， 设置PostProcessList的copy属性为对应值
pub fn as_image_post_process(
    mut del: RemovedComponents<AsImage>,
    mark_type: OrInitRes<RenderContextMarkType<AsImage>>,
    overflow_mark_type: OrInitRes<RenderContextMarkType<Overflow>>,
    mut query: ParamSet<(
        Query<(&AsImage, &mut PostProcess, &mut PostProcessInfo), Or<(Changed<AsImage>, Changed<PostProcess>)>>,
        Query<(&mut PostProcess, &mut PostProcessInfo)>,
    )>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
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
        match (as_image.level, as_image.post_process.is_null()) {
            (pi_style::style::AsImage::None, true) => {
                post_info.effect_mark.set(***mark_type, false);
                if post_list.copy.is_some() && post_info.effect_mark.get(***overflow_mark_type).as_deref() != Some(&true) {
                    post_list.copy = None;
                }
            }
            _ => {
				// log::warn!("as_image================{:?}, {:?}, {:?}", as_image.post_process.is_null(), post_list.copy.is_some(), post_info.effect_mark.get(***overflow_mark_type).as_deref() != Some(&true));
				if as_image.post_process.is_null() && post_list.copy.is_some() && post_info.effect_mark.get(***overflow_mark_type).as_deref() != Some(&true){
					let mut effect_mark = post_info.effect_mark.clone();
					effect_mark.set(***mark_type, false);
					// log::warn!("as_image================{:?}, {:?}", effect_mark, &effect_mark.any());
					// 除了Asimage以外， 还有其他后处理效果， 但没有overflow， 则不需要再copy
					if effect_mark.any() {
						post_list.copy = None;
						return;
					}
				}
				
				if post_list.copy.is_none() {
					post_info.effect_mark.set(***mark_type, true);
					post_list.copy = Some(CopyIntensity::default());
				}
                
            }
        }
    }
}

