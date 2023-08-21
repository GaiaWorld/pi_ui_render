use bevy::app::Plugin;
use bevy::prelude::{IntoSystemSetConfig, IntoSystemSetConfigs, IntoSystemConfigs, Update};
use bevy::ecs::schedule::apply_deferred;
use pi_bevy_render_plugin::PiRenderSystemSet;

use super::render_run;
use super::system_set::UiSystemSet;

pub mod last_update_wgpu;
pub mod pass_camera;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_life;
pub mod update_graph;


pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // 设置运行条件和运行顺序
        app.configure_set(Update, UiSystemSet::PassCalc.run_if(render_run))
            .configure_sets(Update, (UiSystemSet::Setting, UiSystemSet::PassCalc, PiRenderSystemSet).chain())
            .configure_sets(Update, (UiSystemSet::PassFlush, UiSystemSet::PassCalc, PiRenderSystemSet).chain())
            .configure_sets(Update, (UiSystemSet::PrepareDrawObj, UiSystemSet::PassCalc).chain())
            // 创建、删除Pass，为Pass组织树结构
            .add_systems(Update, pass_life::cal_context.in_set(UiSystemSet::PassMark))
            .add_systems(Update, apply_deferred.in_set(UiSystemSet::PassFlush))
            .add_systems(Update, 
                pass_life::calc_pass_children_and_clear
                    .in_set(UiSystemSet::PassSetting)
                    .after(UiSystemSet::PassFlush),
            )
            // 计算图节点及其依赖
            .add_systems(Update, update_graph::update_graph.after(UiSystemSet::PassSetting).after(UiSystemSet::PassFlush))
            // 渲染前，计算Pass的属性
            // 脏区域、相机、深度，更新uniform不顶点buffer到wgpu
            .add_systems(Update, pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PassCalc))
            .add_systems(Update, 
                pass_camera::calc_camera_depth_and_renderlist
                    .after(pass_dirty_rect::calc_global_dirty_rect)
                    .in_set(UiSystemSet::PassCalc),
            )
            .add_systems(Update, 
                last_update_wgpu::last_update_wgpu
                    .after(pass_camera::calc_camera_depth_and_renderlist)
                    .in_set(UiSystemSet::PassCalc),
            );
    }
}
