//! 与Pass相关的system

use blur::BlurPlugin;
use hsi::HsiPlugin;
use mask_image::UiMaskImagePlugin;
use opacity::OpacityPlugin;
use overflow::OverflowPlugin;
use pi_world::prelude::{App, Plugin, IntoSystemConfigs, WorldPluginExtent};
use transform_will_change::TransformWillChangePlugin;

use crate::components::user::{Overflow, TransformWillChange};

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
            .add_system(UiStage, root::root_calc.in_set(UiSystemSet::PassMark))

            .add_plugins(UiAsImagePlugin)
            .add_plugins(BlurPlugin)
            .add_plugins(UiClipPathPlugin)
            .add_plugins(UiMaskImagePlugin)
            .add_plugins(HsiPlugin)
            .add_plugins(OpacityPlugin)
            .add_plugins(OverflowPlugin)
			.add_plugins(RadialWavePlugin)
            .add_plugins(TransformWillChangePlugin)
		;
    }
}
