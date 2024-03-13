//! 与Pass相关的system

use bevy_ecs::prelude::{Changed, IntoSystemSetConfig, IntoSystemSetConfigs, IntoSystemConfigs};
use bevy_app::{Plugin, Update, App};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::FrameDataPrepare;

use crate::components::{
    calc::RenderContextMark,
    pass_2d::ParentPassId,
    user::{Blur, Hsi, Opacity, Overflow, TransformWillChange},
};

use self::{as_image::UiAsImagePlugin, clip_path::UiClipPathPlugin, radial_wave::RadialWavePlugin};

use super::{
    node::{content_box, world_matrix},
    pass::{pass_dirty_rect::OldTransformWillChange, pass_life},
    system_set::UiSystemSet,
};
use bevy_window::AddFrameEvent;

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
        app.configure_sets(Update, (UiSystemSet::Setting, UiSystemSet::PassMark, UiSystemSet::PassFlush).chain())
            .configure_set(Update, UiSystemSet::PassMark.in_set(FrameDataPrepare))
            .configure_set(Update, UiSystemSet::PassFlush.in_set(FrameDataPrepare))
            .configure_set(Update, UiSystemSet::PassSetting.in_set(FrameDataPrepare))
            .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_frame_event::<OldTransformWillChange>()
            .add_systems(Update, 
                pass_life::pass_mark::<Opacity>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_systems(Update, 
                pass_life::pass_mark::<Overflow>
                    .in_set(UiSystemSet::PassMark)
                    .before(pass_life::cal_context),
            )
            .add_systems(Update, pass_life::pass_mark::<Hsi>.in_set(UiSystemSet::PassMark))
            .add_systems(Update, pass_life::pass_mark::<Blur>.in_set(UiSystemSet::PassMark))
            .add_systems(Update, 
                pass_life::pass_mark::<TransformWillChange>
                    .in_set(UiSystemSet::PassMark)
            )
            .add_systems(Update, root::root_calc.in_set(UiSystemSet::PassMark))
            .add_systems(Update, 
                overflow::overflow_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(content_box::calc_content_box)
                    .after(transform_will_change::transform_will_change_post_process)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_systems(Update, 
                transform_will_change::transform_will_change_post_process
                    .after(pass_life::calc_pass_children_and_clear)
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::PassSetting),
            )
            .add_systems(Update, blur::blur_post_process.in_set(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))
            .add_systems(Update, hsi::hsi_post_process.in_set(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))
            .add_systems(Update, 
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
