//! 与Pass相关的system

use bevy::prelude::{apply_system_buffers, Changed, IntoSystemConfig, IntoSystemSetConfig, IntoSystemSetConfigs, Plugin};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;

use crate::components::{
    calc::RenderContextMark,
    pass_2d::ParentPassId,
    user::{Blur, Hsi, Opacity, Overflow, TransformWillChange},
};

use super::{
    node::{content_box, world_matrix},
    render_run,
    system_set::UiSystemSet,
    AddEvent,
};

pub mod blur;
pub mod calc_pass;
pub mod hsi;
pub mod opacity;
pub mod overflow;
pub mod root;
pub mod transform_will_change;

pub struct UiContextPlugin;

impl Plugin for UiContextPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_sets((UiSystemSet::ContextMark, UiSystemSet::ContextFlush).chain())
            .configure_set(UiSystemSet::ContextMark.run_if(render_run))
            .configure_set(UiSystemSet::ContextFlush.run_if(render_run))
            .configure_set(UiSystemSet::ContextCalc.run_if(render_run))
            .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_system(
                calc_pass::pass_mark::<Opacity>
                    .in_set(UiSystemSet::ContextMark)
                    .before(calc_pass::cal_context),
            )
            .add_system(
                calc_pass::pass_mark::<Overflow>
                    .in_set(UiSystemSet::ContextMark)
                    .before(calc_pass::cal_context),
            )
            .add_system(calc_pass::pass_mark::<Hsi>.before(calc_pass::cal_context))
            .add_system(calc_pass::pass_mark::<Blur>.before(calc_pass::cal_context))
            .add_system(
                calc_pass::pass_mark::<TransformWillChange>
                    .in_set(UiSystemSet::ContextMark)
                    .before(calc_pass::cal_context),
            )
            .add_system(root::root_calc.in_set(UiSystemSet::ContextMark).before(calc_pass::cal_context))
            .add_system(calc_pass::cal_context.in_set(UiSystemSet::ContextMark))
            .add_system(apply_system_buffers.in_set(UiSystemSet::ContextFlush))
            .add_system(
                calc_pass::calc_pass_children_and_clear
                    .in_set(UiSystemSet::ContextCalc)
                    .after(UiSystemSet::ContextFlush),
            )
            .add_system(
                overflow::overflow_post_process
                    .after(calc_pass::calc_pass_children_and_clear)
                    .after(content_box::calc_content_box)
                    .after(transform_will_change::transform_will_change_post_process)
                    .in_set(UiSystemSet::ContextCalc),
            )
            .add_system(
                transform_will_change::transform_will_change_post_process
                    .after(calc_pass::calc_pass_children_and_clear)
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::ContextCalc),
            )
            .add_system(blur::blur_post_process.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
            .add_system(hsi::hsi_post_process.in_set(UiSystemSet::ContextCalc).after(UiSystemSet::ContextFlush))
            .add_system(
                opacity::opacity_post_process
                    .in_set(UiSystemSet::ContextCalc)
                    .after(UiSystemSet::ContextFlush),
            );
    }
}
