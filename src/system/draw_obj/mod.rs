use bevy::{app::Plugin, prelude::{StartupSet, IntoSystemConfig, IntoSystemSetConfig}};

use crate::resource::draw_obj::MaxViewSize;

use super::{system_set::UiSystemSet, render_run};


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
		app.configure_set(UiSystemSet::PrepareDrawOb.after(UiSystemSet::BaseCalc).run_if(render_run));
		
        app
			.add_startup_system(clear_draw_obj::init.in_base_set(StartupSet::PostStartup))
            .init_resource::<MaxViewSize>()
            .add_system(root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
			.add_system(context::calc_pass_children_and_clear.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawOb))
			// 在世界矩阵之后运行
            .add_system(world_marix::calc_matrix_group.in_set(UiSystemSet::BaseCalc)
				.after(crate::system::node::world_matrix::cal_matrix)
			)
            .add_system(blur::blur_post_process.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(hsi::hsi_post_process.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(opacity::opacity_post_process.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(overflow::overflow_post_process.after(context::calc_pass_children_and_clear).in_set(UiSystemSet::PrepareDrawOb))
            .add_system(transform_will_change::transform_will_change_post_process.after(context::calc_pass_children_and_clear).in_set(UiSystemSet::PrepareDrawOb))
            .add_system(root_view_port::view_port_change.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(root_clear_color::clear_change.in_set(UiSystemSet::PrepareDrawOb))
            .add_system(border_radius::calc_border_radius.in_set(UiSystemSet::PrepareDrawOb));
    }
}
