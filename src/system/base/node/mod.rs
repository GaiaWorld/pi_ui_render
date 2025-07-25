
use crate::resource::{animation_sheet::KeyFramesSheet, ClassSheet, QuadTree, TimeInfo, UserCommands};
use crate::system::base::node::animation::AnimationPlugin;
use layout::layout_change;
use pi_world::prelude::{IntoSystemConfigs, Plugin, App, PostUpdate, WorldPluginExtent};

use self::{user_setting::clear_dirty_mark, transition::TransitionPlugin, show::ShowPlugin};

use crate::system::system_set::{UiSchedule, UiSystemSet};
use crate::prelude::UiStage;

// pub mod flush;
pub mod layout;
pub mod world_matrix;
// pub mod quad;
pub mod show;
pub mod user_setting;
pub mod z_index;
// pub mod context_mask_texture;
pub mod animation;
pub mod transition;
#[cfg(feature = "debug")]
pub mod cmd_play;

// pub fn clear_dirty_list(mut dirty_list: SingleResMut<DirtyList>, system_tick: SystemChangeTick) {
// 	dirty_list.clear(system_tick.this_run());
// }



pub struct UiNodePlugin;

impl Plugin for UiNodePlugin {
    fn build(&self, app: &mut App) {
        app.world.init_single_res::<UserCommands>();
        app.world.init_single_res::<ClassSheet>();
        app.world.init_single_res::<TimeInfo>();
        app.world.init_single_res::<KeyFramesSheet>();

        app
            // .add_frame_event::<ComponentEvent<Changed<Layer>>>()
			// .add_frame_event::<StyleChange>()
            // .init_single_res::<UserCommands>()
            // .init_single_res::<ClassSheet>()
            // .init_single_res::<TimeInfo>()
            // .init_single_res::<KeyFramesSheet>()

			// 维护脏列表, 没帧结束时清理
			.add_system(PostUpdate, clear_dirty_mark
                .after(bevy_window::FrameSet)
                .in_set(UiSystemSet::ClearSetting)
            )
			
            // 设置用户指令
            .add_system(UiStage, user_setting::user_setting1.in_set(UiSystemSet::Setting))
            .add_system(UiStage, user_setting::user_setting2.in_set(UiSystemSet::Setting).after(user_setting::user_setting1))

            // 计算动画
            .add_plugins(AnimationPlugin)

             // 计算Transition
			.add_plugins(TransitionPlugin)
            // 布局相关
            // .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_system(UiStage,   layout::calc_layout
                .in_set(UiSystemSet::Layout)
                .run_if(layout_change)
                .in_schedule(UiSchedule::Layout)
                .in_schedule(UiSchedule::Calc)
                .in_schedule(UiSchedule::Geo));

            // 世界矩阵、包围盒、内容包围盒
            // .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            // .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            // .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            // .add_frame_event::<OldQuad>()
        app.world.init_single_res::<QuadTree>();
        app
            .add_system(UiStage, world_matrix::cal_matrix.in_set(UiSystemSet::LayoutAfter).in_schedule(UiSchedule::Calc).in_schedule(UiSchedule::Geo))
            // .add_system(UiStage, content_box::calc_content_box
            //     .after(world_matrix::cal_matrix)
            //     .in_set(UiSystemSet::BaseCalc))
            // zindex
            .add_system(UiStage, z_index::calc_zindex.in_set(UiSystemSet::BaseCalc).run_if(z_index::zindex_change) )
			// 计算是否可见
            .add_plugins(ShowPlugin)
		;
    }
}
