//! 与Pass相关的system

use blur::BlurPlugin;
use hsi::HsiPlugin;
use opacity::OpacityPlugin;
use pi_world::prelude::{App, Plugin, WorldPluginExtent};

use self::{as_image::UiAsImagePlugin, clip_path::UiClipPathPlugin, radial_wave::RadialWavePlugin};

pub mod as_image;
pub mod blur;
pub mod clip_path;
pub mod hsi;
// pub mod mask_image;
pub mod opacity;
pub mod radial_wave;

pub struct UiEffectPlugin;

impl Plugin for UiEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            // .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            // .add_frame_event::<OldTransformWillChange>()

            .add_plugins(UiAsImagePlugin)
            .add_plugins(BlurPlugin)
            .add_plugins(UiClipPathPlugin)
            // .add_plugins(mask_image::UiMaskImagePlugin)
            .add_plugins(HsiPlugin)
            .add_plugins(OpacityPlugin)
			.add_plugins(RadialWavePlugin)
		;
    }
}
