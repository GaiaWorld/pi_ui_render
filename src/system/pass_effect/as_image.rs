use std::sync::atomic::AtomicUsize;

use pi_world::{filter::Or, prelude::{App, Changed, ComponentRemoved, Has, IntoSystemConfigs, ParamSet, Plugin, Query}};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_bevy_asset::{Allocator, AssetConfig, AssetDesc, ShareAssetMgr};
use pi_null::Null;

use crate::{
    components::{
        pass_2d::{CacheTarget, PostProcessInfo},
        user::{AsImage, Overflow},
    },
    resource::{draw_obj::TargetCacheMgr, RenderContextMarkType},
    system::{
        // node::user_setting::user_setting,
        // pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
        draw_obj::calc_text::IsRun, node::user_setting::user_setting2, pass::{last_update_wgpu::last_update_wgpu, pass_camera::calc_camera_depth_and_renderlist},
    },
};
use pi_postprocess::prelude::CopyIntensity;

use crate::{components::pass_2d::PostProcess, system::pass::pass_life};
use crate::prelude::UiStage;

pub struct UiAsImagePlugin;

impl Plugin for UiAsImagePlugin {
    fn build(&self, app: &mut App) {
        let mut w1 =  app.world.unsafe_world();
        let allocator = w1.get_single_res_mut::<Allocator>().unwrap();
        let assets_mgr = {
            let w = app.world.unsafe_world();
            let asset_config = w.get_single_res::<AssetConfig>().unwrap();
            let default_cfg = AssetDesc {
                ref_garbage: false,
                min: 0,
                weight: 32 * 1024 * 1024, // 默认32M的fbo缓存
                timeout: 0,            // 并不会启用超时整理， 这里的数值无所谓（记得该资源管理器中的资源需要手动删除）
            };
            // let desc = asset_config.get::<CacheTarget>().unwrap_or(&default_cfg);
            ShareAssetMgr::<CacheTarget>::new_with_config(pi_assets::asset::GarbageEmpty(), &default_cfg, asset_config, allocator)
        };

        app.world.insert_single_res(TargetCacheMgr { key: AtomicUsize::new(0), assets: assets_mgr });
            // 标记有AsImage组件的节点为渲染上下文
        app
            .add_system(UiStage, 
                pass_life::pass_mark::<AsImage>
                    .after(user_setting2)
                    .before(pass_life::cal_context)
                    // ,
            )
            .add_system(UiStage, 
                as_image_post_process
                    .before(last_update_wgpu)
                    .after(calc_camera_depth_and_renderlist)
                    // ,
            );
    }
}

/// 处理AsImage组件
/// 如果AsImage删除， 设置PostProcessList的copy属性为None
/// 如果AsImage修改， 设置PostProcessList的copy属性为对应值
pub fn as_image_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<AsImage>>,
    overflow_mark_type: OrInitSingleRes<RenderContextMarkType<Overflow>>,
    mut query: ParamSet<(
        Query<(&AsImage, &mut PostProcess, &mut PostProcessInfo), Or<(Changed<AsImage>, Changed<PostProcess>)>>,
        Query<(&mut PostProcess, &mut PostProcessInfo, Has<AsImage>)>,
    )>,
    removed: ComponentRemoved<AsImage>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // AsImage 如果删除， 取消AsImage的后处理
    let p1 = query.p1();
    for i in removed.iter() {
        if let Ok((mut post_list, mut post_info, has_as_image)) = p1.get_mut(*i) {
            if has_as_image {
                continue;
            }
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

