//! 与Pass相关的system

use bevy::prelude::{Changed, IntoSystemConfig, IntoSystemSetConfig, IntoSystemSetConfigs, Plugin};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;

use crate::components::{
    calc::RenderContextMark,
    pass_2d::ParentPassId,
    user::{Blur, Hsi, Opacity, Overflow, TransformWillChange},
};

use self::{mask_image::UiMaskImagePlugin, clip_path::UiClipPathPlugin};

use super::{
    node::{content_box, world_matrix},
    render_run,
    system_set::UiSystemSet,
    AddEvent, pass::pass_life,
};

pub mod blur;
pub mod hsi;
pub mod opacity;
pub mod overflow;
pub mod root;
pub mod transform_will_change;
pub mod mask_image;
pub mod clip_path;


pub struct UiEffectPlugin;

impl Plugin for UiEffectPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_sets((UiSystemSet::Setting, UiSystemSet::PassMark, UiSystemSet::PassFlush).chain())
            .configure_set(UiSystemSet::PassMark.run_if(render_run))
            .configure_set(UiSystemSet::PassFlush.run_if(render_run))
            .configure_set(UiSystemSet::PassSetting.run_if(render_run))
            .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_system(
                pass_life::pass_mark::<Opacity>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_system(
                pass_life::pass_mark::<Overflow>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_system(pass_life::pass_mark::<Hsi>.before(pass_life::cal_context).in_set(UiSystemSet::PassMark))
            .add_system(pass_life::pass_mark::<Blur>.before(pass_life::cal_context).in_set(UiSystemSet::PassMark))
            .add_system(
                pass_life::pass_mark::<TransformWillChange>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_system(root::root_calc.in_set(UiSystemSet::PassMark).before(pass_life::cal_context))
            
            .add_system(
                overflow::overflow_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(content_box::calc_content_box)
                    .after(transform_will_change::transform_will_change_post_process)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_system(
                transform_will_change::transform_will_change_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_system(blur::blur_post_process.in_set(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))
            .add_system(hsi::hsi_post_process.in_set(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))
            .add_system(
                opacity::opacity_post_process
                    .in_set(UiSystemSet::PassSetting)
                    .after(UiSystemSet::PassFlush),
            )
			.add_plugin(UiMaskImagePlugin)
			.add_plugin(UiClipPathPlugin)
		;
    }
}
