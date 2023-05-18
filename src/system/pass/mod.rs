//! 

use bevy::prelude::{Plugin, Changed, IntoSystemConfig, apply_system_buffers, IntoSystemSetConfigs, IntoSystemSetConfig};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;

use crate::components::{calc::RenderContextMark, pass_2d::ParentPassId};

use super::{system_set::UiSystemSet, AddEvent, render_run, node::{content_box, world_matrix}};

pub mod blur;
pub mod border_radius;
pub mod hsi;
pub mod opacity;
pub mod overflow;
pub mod transform_will_change;

pub mod context;
pub mod mark_blur;
pub mod mark_hsi;
pub mod mark_opacity;
pub mod mark_root;
pub mod mark_transform_will_change;
pub mod mark_overflow;

pub struct UiContextPlugin;

impl Plugin for UiContextPlugin {
    fn build(&self, app: &mut bevy::app::App) {
		app
			.configure_sets(
				(
					UiSystemSet::ContextMark,
					UiSystemSet::ContextFlush,
				).chain())
			.configure_set(UiSystemSet::ContextMark.run_if(render_run))
			.configure_set(UiSystemSet::ContextFlush.run_if(render_run))
			.configure_set(UiSystemSet::ContextCalc.run_if(render_run))

			.add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_system(mark_opacity::opacity_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
            .add_system(mark_overflow::overflow_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
            .add_system(mark_hsi::hsi_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
            .add_system(mark_blur::blur_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
            .add_system(mark_transform_will_change::transform_willchange_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
			.add_system(mark_root::root_calc.in_set(UiSystemSet::ContextMark).before(context::cal_context))
			
			.add_system(context::cal_context.in_set(UiSystemSet::ContextMark))
			.add_system(apply_system_buffers.in_set(UiSystemSet::ContextFlush))
			.add_system(context::calc_pass_children_and_clear.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
			
			.add_system(overflow::overflow_post_process
				.after(context::calc_pass_children_and_clear)
				.after(content_box::calc_content_box)
				.after(transform_will_change::transform_will_change_post_process)
				.in_set(UiSystemSet::ContextCalc))
            .add_system(transform_will_change::transform_will_change_post_process
				.after(context::calc_pass_children_and_clear)
				.after(world_matrix::cal_matrix)
				.in_set(UiSystemSet::ContextCalc))

            .add_system(blur::blur_post_process.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
            .add_system(hsi::hsi_post_process.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
            .add_system(opacity::opacity_post_process.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
            .add_system(border_radius::calc_border_radius
				.in_set(UiSystemSet::ContextCalc)
				.after(UiSystemSet::ContextFlush)
				.after(UiSystemSet::PrepareDrawObj))

			;
	}
}
