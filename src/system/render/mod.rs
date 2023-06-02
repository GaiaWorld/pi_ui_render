use bevy::app::Plugin;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig};

use super::render_run;
use super::system_set::UiSystemSet;

// pub mod pass_mark;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_camera;
pub mod last_update_wgpu;

pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_set(UiSystemSet::PreparePass.run_if(render_run));

        app
            .add_system(pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PreparePass))
            .add_system(
                pass_camera::calc_camera_depth_and_renderlist
                    .after(pass_dirty_rect::calc_global_dirty_rect)
                    .in_set(UiSystemSet::PreparePass),
            )
			.add_system(
                last_update_wgpu::last_update_wgpu
                    .after(pass_camera::calc_camera_depth_and_renderlist)
                    .run_if(render_run)
            )
		;
    }
}
