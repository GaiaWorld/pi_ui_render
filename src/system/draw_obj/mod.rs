use bevy::{app::{CoreStage, Plugin, StartupStage}, prelude::IntoSystemDescriptor};

use crate::resource::draw_obj::MaxViewSize;

use super::render_run;


pub mod blur;
pub mod border_radius;
pub mod clear_draw_obj;
pub mod hsi;
pub mod opacity;
pub mod overflow;
pub mod pipeline;
pub mod root_clear_color;
pub mod root_view_port;
pub mod transform_will_change;
pub mod world_marix;
pub mod context;

pub struct UiReadyDrawPlugin;

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, clear_draw_obj::init)
            .init_resource::<MaxViewSize>()
            .add_system_to_stage(CoreStage::PreUpdate, root_view_port::calc_dyn_target_type.with_run_criteria(render_run))
			.add_system_to_stage(CoreStage::Update, context::calc_pass_children_and_clear.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, pipeline::calc_node_pipeline.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, world_marix::calc_matrix_group.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, blur::blur_post_process.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, hsi::hsi_post_process.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, opacity::opacity_post_process.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, overflow::overflow_post_process.after(context::calc_pass_children_and_clear).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, transform_will_change::transform_will_change_post_process.after(context::calc_pass_children_and_clear).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, root_view_port::view_port_change.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, root_clear_color::clear_change.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::Update, border_radius::calc_border_radius.with_run_criteria(render_run));
    }
}
