use crate::{
    components::{
        calc::{ContentBox, LayoutResult, Quad},
        user::Transform,
    },
    resource::{animation_sheet::KeyFramesSheet, ClassSheet, QuadTree, TimeInfo, UserCommands},
};
use bevy_ecs::prelude::{IntoSystemConfigs, Changed};
use bevy_app::{App, Plugin, PostUpdate};
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::FrameDataPrepare;

use self::{world_matrix::OldQuad, user_setting::{StyleChange, clear_dirty_mark}, transition::TransitionPlugin, show::ShowPlugin};

use super::system_set::UiSystemSet;
use crate::prelude::UiSchedule;
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
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>()
			.add_frame_event::<StyleChange>()
            .init_resource::<UserCommands>()
            .init_resource::<ClassSheet>()
            .init_resource::<TimeInfo>()
            .init_resource::<KeyFramesSheet>()

			// 维护脏列表
			.add_systems(PostUpdate, clear_dirty_mark.in_set(FrameDataPrepare).after(bevy_window::FrameSet))
			
            // 设置用户指令
            .add_systems(UiSchedule, user_setting::user_setting.in_set(UiSystemSet::Setting))

            // 运行动画
            .add_systems(UiSchedule, animation::calc_animation.in_set(UiSystemSet::NextSetting))
             // 计算Transition
			.add_plugins(TransitionPlugin)

            // 布局相关
            .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_systems(UiSchedule,   layout::calc_layout.in_set(UiSystemSet::Layout))

            // 世界矩阵、包围盒、内容包围盒
            .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            .add_frame_event::<OldQuad>()
            .init_resource::<QuadTree>()
            .add_systems(UiSchedule, world_matrix::cal_matrix.in_set(UiSystemSet::Matrix))
            .add_systems(UiSchedule, content_box::calc_content_box.after(world_matrix::cal_matrix).in_set(UiSystemSet::BaseCalc))
            // zindex
            .add_systems(UiSchedule, z_index::calc_zindex.in_set(UiSystemSet::BaseCalc))
			// 计算是否可见
            .add_plugins(ShowPlugin)
		;
    }
}
