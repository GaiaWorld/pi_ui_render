use bevy::app::{CoreStage, Plugin};
use bevy::ecs::schedule::IntoSystemDescriptor;

use super::render_run;

// pub mod pass_mark;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_render;
pub mod update_graph;

pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_to_stage(CoreStage::PostUpdate, update_graph::update_graph.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PostUpdate, pass_dirty_rect::calc_global_dirty_rect.with_run_criteria(render_run))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                pass_render::calc_camera_depth_and_renderlist.after(pass_dirty_rect::calc_global_dirty_rect).with_run_criteria(render_run),
            );
    }
}
