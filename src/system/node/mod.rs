use crate::{
    components::{
        calc::{ContentBox, LayoutResult, Quad},
        user::Transform,
    },
    resource::{animation_sheet::KeyFramesSheet, ClassSheet, QuadTree, TimeInfo, UserCommands},
};
use bevy::app::Plugin;
use bevy::ecs::query::Changed;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig};
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::should_run;

use self::world_matrix::OldQuad;

use super::system_set::UiSystemSet;
use super::{layout_run, matrix_run, render_run, AddEvent};

// pub mod flush;
pub mod layout;
pub mod world_matrix;
// pub mod quad;
pub mod content_box;
pub mod show;
pub mod user_setting;
pub mod z_index;
// pub mod context_mask_texture;
pub mod animation;


pub struct UiNodePlugin;

impl Plugin for UiNodePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_set(UiSystemSet::Load.run_if(layout_run));
        app.configure_set(UiSystemSet::Layout.run_if(layout_run));
        app.configure_set(UiSystemSet::Matrix.run_if(matrix_run));
        app.configure_set(UiSystemSet::BaseCalc.run_if(render_run));
        app.configure_set(UiSystemSet::LifeDrawObject.run_if(render_run));

        app.add_frame_event::<ComponentEvent<Changed<Layer>>>()
            .init_resource::<UserCommands>()
            .init_resource::<ClassSheet>()
            .init_resource::<TimeInfo>()
            .init_resource::<KeyFramesSheet>()
            // 设置相关
            .add_system(user_setting::user_setting.in_set(UiSystemSet::Setting))
            .add_system(
                animation::calc_animation
                    .after(user_setting::user_setting)
                    .in_set(UiSystemSet::Setting)
                    .run_if(should_run),
            )
            // 布局相关
            .add_system(user_setting::set_image_default_size.in_set(UiSystemSet::Layout))
            .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_system(
                layout::calc_layout
                    .after(user_setting::set_image_default_size)
                    .in_set(UiSystemSet::Layout),
            )
            // 与世界矩阵、包围盒、内容包围盒
            .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            .add_frame_event::<OldQuad>()
            .init_resource::<QuadTree>()
            .add_system(world_matrix::cal_matrix.after(layout::calc_layout).in_set(UiSystemSet::Matrix))
            .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            .add_system(
                content_box::calc_content_box
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::BaseCalc),
            )
            // zinde、show、contex等于其他计算无关，仅仅与用户设置属性相关的system运行
            .add_system(z_index::calc_zindex.in_set(UiSystemSet::BaseCalc))
            .add_system(show::calc_show.in_set(UiSystemSet::BaseCalc));
    }
}
