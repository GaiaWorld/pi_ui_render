//! 与Pass相关的system

use pi_world::prelude::{App, Plugin, IntoSystemConfigs, WorldPluginExtent};

use crate::components::user::{Blur, Hsi, Opacity, Overflow, TransformWillChange};

use self::{as_image::UiAsImagePlugin, clip_path::UiClipPathPlugin, radial_wave::RadialWavePlugin};

use super::{
    node::world_matrix::{self, cal_matrix}, pass::pass_life, system_set::UiSystemSet
};
use crate::prelude::UiStage;

pub mod as_image;
pub mod blur;
pub mod clip_path;
pub mod hsi;
pub mod mask_image;
pub mod opacity;
pub mod overflow;
pub mod root;
pub mod transform_will_change;
pub mod radial_wave;

pub struct UiEffectPlugin;

impl Plugin for UiEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            // .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            // .add_frame_event::<OldTransformWillChange>()
            .add_system(UiStage, 
                pass_life::pass_mark::<Opacity>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_system(UiStage, 
                pass_life::pass_mark::<Overflow>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_system(UiStage, pass_life::pass_mark::<Hsi>.in_set(UiSystemSet::PassMark))
            .add_system(UiStage, pass_life::pass_mark::<Blur>.in_set(UiSystemSet::PassMark))
            .add_system(UiStage, 
                pass_life::pass_mark::<TransformWillChange>
                    .in_set(UiSystemSet::PassMark)
            )
            .add_system(UiStage, root::root_calc.in_set(UiSystemSet::PassMark))
            .add_system(UiStage, 
                overflow::overflow_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    // .after(content_box::calc_content_box)
                    .after(cal_matrix)
                    .after(transform_will_change::transform_will_change_post_process)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_system(UiStage, 
                transform_will_change::transform_will_change_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_system(UiStage, blur::blur_post_process.in_set(UiSystemSet::PassSetting)
                .after(UiSystemSet::PassFlush)
            )
            .add_system(UiStage, hsi::hsi_post_process.in_set(UiSystemSet::PassSetting)
                .after(UiSystemSet::PassFlush)
            )
            .add_system(UiStage, 
                opacity::opacity_post_process
                    .in_set(UiSystemSet::PassSetting)
                    .after(UiSystemSet::PassFlush),
            )
            // .add_plugins(UiMaskImagePlugin)
            .add_plugins(UiClipPathPlugin)
            .add_plugins(UiAsImagePlugin)
			.add_plugins(RadialWavePlugin)
		;
    }
}
