use crate::{
    components::{
        calc::{ContentBox, LayoutResult, Quad},
        user::Transform,
    },
    resource::{animation_sheet::KeyFramesSheet, ClassSheet, QuadTree, TimeInfo, UserCommands}, events::EntityChange,
};
use bevy_ecs::prelude::{IntoSystemConfigs, IntoSystemSetConfig, Changed};
use bevy_app::{Plugin, Update, App};
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::FrameDataPrepare;

use self::{world_matrix::OldQuad, user_setting::{StyleChange, clear_dirty_mark}, transition::TransitionPlugin, show::ShowPlugin};

use super::system_set::UiSystemSet;
use super::{layout_run, matrix_run};
use bevy_window::AddFrameEvent;

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
pub mod transition;

// pub fn clear_dirty_list(mut dirty_list: ResMut<DirtyList>, system_tick: SystemChangeTick) {
// 	dirty_list.clear(system_tick.this_run());
// }

pub struct UiNodePlugin;

impl Plugin for UiNodePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, UiSystemSet::Load.run_if(layout_run));
        app.configure_set(Update, UiSystemSet::Layout.run_if(layout_run));
        app.configure_set(Update, UiSystemSet::Matrix.run_if(matrix_run));
        app.configure_set(Update, UiSystemSet::BaseCalc.in_set(FrameDataPrepare));
        app.configure_set(Update, UiSystemSet::LifeDrawObject.in_set(FrameDataPrepare));

        app.add_frame_event::<ComponentEvent<Changed<Layer>>>()
			.add_frame_event::<StyleChange>()
            .init_resource::<UserCommands>()
            .init_resource::<ClassSheet>()
            .init_resource::<TimeInfo>()
            .init_resource::<KeyFramesSheet>()

			// // 维护脏列表
			.add_systems(Update, clear_dirty_mark.in_set(FrameDataPrepare).after(UiSystemSet::PassCalc))
			
            // 设置相关
            .add_systems(Update, user_setting::user_setting.in_set(UiSystemSet::Setting))
            .add_systems(Update, 
                animation::calc_animation
                    .after(user_setting::user_setting)
                    .in_set(UiSystemSet::Setting)
                    .in_set(FrameDataPrepare),
            )
            // 布局相关
            .add_systems(Update, user_setting::set_image_default_size.in_set(UiSystemSet::Layout))
            .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_systems(Update, 
                layout::calc_layout
                    .after(user_setting::set_image_default_size)
                    .in_set(UiSystemSet::Layout),
            )
            // 与世界矩阵、包围盒、内容包围盒
            .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            .add_frame_event::<OldQuad>()
            .init_resource::<QuadTree>()
            .add_systems(Update, world_matrix::cal_matrix.after(layout::calc_layout).in_set(UiSystemSet::Matrix))
            .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            .add_systems(Update, 
                content_box::calc_content_box
                    .after(world_matrix::cal_matrix)
                    .in_set(UiSystemSet::BaseCalc),
            )
            // zinde、show、contex等于其他计算无关，仅仅与用户设置属性相关的system运行
            .add_systems(Update, z_index::calc_zindex.in_set(UiSystemSet::BaseCalc))
			.add_plugins(ShowPlugin)
			.add_plugins(TransitionPlugin)
		;
    }
}
