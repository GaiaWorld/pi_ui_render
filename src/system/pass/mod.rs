use bevy::app::Plugin;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig, IntoSystemSetConfigs, apply_system_buffers};
use pi_bevy_render_plugin::PiRenderSystemSet;

use super::render_run;
use super::system_set::UiSystemSet;

pub mod pass_life;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_camera;
pub mod last_update_wgpu;
pub mod update_graph;


pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut bevy::app::App) {

		// 设置运行条件和运行顺序
        app
			.configure_set(UiSystemSet::PassCalc.run_if(render_run))
			.configure_sets((
				UiSystemSet::Setting,
				UiSystemSet::PassCalc,
				PiRenderSystemSet,
			).chain())
			.configure_sets((
				UiSystemSet::PassFlush,
				UiSystemSet::PassCalc,
				PiRenderSystemSet,
			).chain())
			.configure_sets((
				UiSystemSet::PrepareDrawObj,
				UiSystemSet::PassCalc,
			).chain())
		
			
			// 创建、删除Pass，为Pass组织树结构
			.add_system(pass_life::cal_context.in_set(UiSystemSet::PassMark))
            .add_system(apply_system_buffers.in_set(UiSystemSet::PassFlush))
            .add_system(
                pass_life::calc_pass_children_and_clear
                    .in_set(UiSystemSet::PassSetting)
                    .after(UiSystemSet::PassFlush),
            )

			// 计算图节点及其依赖
			.add_system(update_graph::update_graph.after(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))

			// 渲染前，计算Pass的属性
			// 脏区域、相机、深度，更新uniform不顶点buffer到wgpu
            .add_system(pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PassCalc))
            .add_system(
                pass_camera::calc_camera_depth_and_renderlist
                    .after(pass_dirty_rect::calc_global_dirty_rect)
                    .in_set(UiSystemSet::PassCalc),
            )
			.add_system(
                last_update_wgpu::last_update_wgpu
                    .after(pass_camera::calc_camera_depth_and_renderlist)
					.in_set(UiSystemSet::PassCalc)
            )
		;
    }
}
