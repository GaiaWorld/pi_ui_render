use crate::resource::{animation_sheet::KeyFramesSheet, ClassSheet, QuadTree, TimeInfo, UserCommands};
use pi_world::prelude::{IntoSystemConfigs, Plugin, App, PostUpdate, WorldPluginExtent};
use pi_bevy_render_plugin::FrameDataPrepare;

use self::{user_setting::clear_dirty_mark, transition::TransitionPlugin, show::ShowPlugin};

use super::system_set::UiSystemSet;
use crate::prelude::UiStage;

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

			// 维护脏列表
			.add_system(PostUpdate, clear_dirty_mark
                .after(bevy_window::FrameSet)
            )
			
            // 设置用户指令
            .add_system(UiStage, user_setting::user_setting1.in_set(UiSystemSet::Setting))
            .add_system(UiStage, user_setting::user_setting2.in_set(UiSystemSet::Setting).after(user_setting::user_setting1))

            // 运行动画
            .add_system(UiStage, animation::calc_animation_1.in_set(UiSystemSet::NextSetting))
            .add_system(UiStage, animation::calc_animation_2.in_set(UiSystemSet::NextSetting).after(animation::calc_animation_1))

             // 计算Transition
			.add_plugins(TransitionPlugin)

            // 布局相关
            // .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_system(UiStage,   layout::calc_layout.in_set(UiSystemSet::Layout));

            // 世界矩阵、包围盒、内容包围盒
            // .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            // .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            // .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            // .add_frame_event::<OldQuad>()
        app.world.init_single_res::<QuadTree>();
        app
            .add_system(UiStage, world_matrix::cal_matrix.in_set(UiSystemSet::Matrix))
            .add_system(UiStage, content_box::calc_content_box
                .after(world_matrix::cal_matrix)
                .in_set(UiSystemSet::BaseCalc))
            // zindex
            .add_system(UiStage, z_index::calc_zindex.in_set(UiSystemSet::BaseCalc))
			// 计算是否可见
            .add_plugins(ShowPlugin)
		;
    }
}
