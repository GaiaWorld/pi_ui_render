use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_ecs::prelude::{IntoSystemSetConfig, IntoSystemSetConfigs, IntoSystemConfigs};
use bevy_ecs::schedule::apply_deferred;
use pi_bevy_render_plugin::{PiRenderSystemSet, FrameDataPrepare, GraphBuild, GraphRun};

use self::pass_camera::calc_camera_depth_and_renderlist;
use self::pass_life::calc_pass;
use self::update_graph::init_root_graph;
use super::system_set::UiSystemSet;
use crate::prelude::UiSchedule;

pub mod last_update_wgpu;
pub mod pass_camera;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_life;
pub mod update_graph;


pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut App) {
        // 设置运行条件和运行顺序
        app.configure_set(UiSchedule, UiSystemSet::PassMark.in_set(FrameDataPrepare))
			.configure_set(UiSchedule, UiSystemSet::PassLife.in_set(FrameDataPrepare))
			.configure_set(UiSchedule, UiSystemSet::PassFlush.in_set(FrameDataPrepare))
			.configure_set(UiSchedule, UiSystemSet::PassSetting.in_set(FrameDataPrepare))
			.configure_set(UiSchedule, UiSystemSet::PassSettingWithParent.in_set(FrameDataPrepare))
			.configure_set(UiSchedule, UiSystemSet::PassCalc.in_set(FrameDataPrepare))

            .configure_sets(UiSchedule, (UiSystemSet::PassMark, UiSystemSet::PassLife, UiSystemSet::PassFlush, UiSystemSet::PassSetting, UiSystemSet::PassCalc, PiRenderSystemSet).chain())
			.configure_sets(UiSchedule, (UiSystemSet::Setting, UiSystemSet::PassMark, PiRenderSystemSet).chain())	
            .configure_sets(UiSchedule, (UiSystemSet::PrepareDrawObj, UiSystemSet::PassCalc, PiRenderSystemSet).chain())
            // 创建、删除Pass，为Pass组织树结构
            .add_systems(UiSchedule, 
				calc_pass
					.after(calc_camera_depth_and_renderlist)
                    .after(UiSystemSet::PassFlush)
					.in_set(FrameDataPrepare)
			)
            .add_systems(UiSchedule, pass_life::cal_context.in_set(UiSystemSet::PassLife))
            .add_systems(UiSchedule, apply_deferred.in_set(UiSystemSet::PassFlush))
            .add_systems(UiSchedule, 
                pass_life::calc_pass_children_and_clear
                    .in_set(UiSystemSet::PassSetting)
					.before(UiSystemSet::PassSettingWithParent) // 在所有依赖父子关系的system之前执行
                    .after(UiSystemSet::PassFlush), // 在上下文创建之后执行
            )
            .add_systems(UiSchedule, pass_life::calc_pass_toop_sort.in_set(FrameDataPrepare).after(UiSystemSet::PassSetting))
            .add_systems(Startup, init_root_graph)
            // 计算图节点及其依赖
            .add_systems(UiSchedule, update_graph::update_graph.after(UiSystemSet::PassSettingWithParent).after(UiSystemSet::PassSetting))
            // 渲染前，计算Pass的属性
            // 脏区域、相机、深度，更新uniform不顶点buffer到wgpu
            .add_systems(UiSchedule, pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PassCalc))
            .add_systems(UiSchedule, 
                pass_camera::calc_camera_depth_and_renderlist
                    .after(pass_dirty_rect::calc_global_dirty_rect)
					.after(UiSystemSet::BaseCalcFlush)
                    .in_set(UiSystemSet::PassCalc),
            )
			
            .add_systems(PostUpdate, 
                last_update_wgpu::last_update_wgpu
                    .after(GraphBuild)
					.before(GraphRun)
                    .in_set(FrameDataPrepare),
            );
    }
}
