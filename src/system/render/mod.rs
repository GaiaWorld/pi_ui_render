use bevy::app::Plugin;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig};

use super::render_run;
use super::system_set::UiSystemSet;

// pub mod pass_mark;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_render;
pub mod update_graph;

pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut bevy::app::App) {
		app.configure_set(UiSystemSet::PreparePass.run_if(render_run));

        app
			.add_system(update_graph::update_graph.in_set(UiSystemSet::PreparePass))
            .add_system(pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PreparePass))
            .add_system(pass_render::calc_camera_depth_and_renderlist.after(pass_dirty_rect::calc_global_dirty_rect).in_set(UiSystemSet::PreparePass));
    }
}
