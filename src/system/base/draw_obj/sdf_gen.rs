
use std::{collections::VecDeque, sync::atomic::AtomicBool};

use pi_world::{prelude::{SingleRes, SingleResMut}, schedule_config::IntoSystemConfigs, system_params::Local};
use pi_bevy_ecs_extend::prelude::OrInitSingleResMut;

use pi_bevy_render_plugin::PiRenderDevice;
use pi_share::Share;
use crate::{ resource::{draw_obj::InstanceContext, ShareFontSheet}, system::system_set::UiSystemSet};
use pi_world::prelude::Plugin;
use pi_hal::{font::{font::FontType, sdf2_table::SdfResult}, runtime::MULTI_MEDIA_RUNTIME};
use crate::prelude::UiStage;
use pi_async_rt::prelude::AsyncRuntime;

pub struct SdfPlugin;

impl Plugin for SdfPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		let font_sheet = ShareFontSheet::new(&mut app.world, FontType::Sdf2);
		app.world.insert_single_res(font_sheet);

        // 更新sdf2纹理
        app
            .add_startup_system(
                UiStage, 
                update_sdf2_texture
                    .in_set(UiSystemSet::PrepareDrawObj)
            )
            .add_system(
                UiStage, 
                draw_sdf
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(update_sdf2_texture)
            )
        ;
    }
}

/// 更新sdf2的纹理
pub fn update_sdf2_texture(
    mut instances: OrInitSingleResMut<InstanceContext>,
    font_sheet: SingleResMut<ShareFontSheet>,
    device: SingleRes<PiRenderDevice>,
    common_sampler: SingleRes<crate::resource::draw_obj::CommonSampler>,
) {
    log::debug!("create sdf2 binding==============");
    let font_sheet = font_sheet.0.borrow();
    if let Some(sdf_texture) = &font_sheet.sdf_texture_view {
        if instances.sdf2_texture_group.is_none() {
            let group = (***device).create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &instances.sdf2_texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&sdf_texture.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.default),
                    },
                ],
                label: Some("sdf2 texture bind group create"),
            });

            instances.sdf2_texture_group = Some(Share::new(group));
            log::debug!("create sdf2 binding1==============");
        }
    }
}

pub fn draw_sdf(
    font_sheet: SingleResMut<ShareFontSheet>, 
    mut await_list: Local<VecDeque<(SdfResult, Share<AtomicBool>, usize)>>,
) {
    let mut font_sheet: pi_share::cell::RefMut<'_, pi_render::font::FontSheet> = font_sheet.borrow_mut();
    let draw_count = font_sheet.draw_count();

    if draw_count > 0 {
        let result = SdfResult::default();
        let mark = Share::new(AtomicBool::new(false));
        await_list.push_back((result.clone(), mark.clone(), draw_count));
        let cur_await = font_sheet.draw_await(result.clone(), 0, draw_count);
        MULTI_MEDIA_RUNTIME.spawn(async move {
            let t1 = pi_time::Instant::now();
            cur_await.await;
            log::error!("draw sdf2==========={:?}", (draw_count,  pi_time::Instant::now() - t1));
            mark.store(true, std::sync::atomic::Ordering::Relaxed);
        }).unwrap();
    }


    let mut next = await_list.front();
    loop {
        if let Some((_result, is_load, _)) = next {
            // println!("await================{:?}", &await_set_gylph);
            if is_load.load(std::sync::atomic::Ordering::Relaxed) == true {
                let (result, _, draw_count) = await_list.pop_front().unwrap();
                let t1 = pi_time::Instant::now();
                font_sheet.update_sdf2(result); // 更新纹理
                log::debug!("update_sdf2================{:?}", (draw_count, pi_time::Instant::now() - t1));
                
                next = await_list.front();
                continue;
            }
        }
        break;
    }
    
}