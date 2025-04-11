use pass_life::pass_life_children;
use pi_world::prelude::{App, Plugin, PostUpdate, IntoSystemConfigs, WorldPluginExtent};
use pi_bevy_render_plugin::{GraphBuild, GraphRun};

use self::pass_camera::calc_camera;
use self::pass_life::calc_pass;
use self::update_graph::init_root_graph;
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

pub mod last_update_wgpu;
pub mod pass_camera;
pub mod pass_dirty_rect;
pub mod pass_graph_node;
pub mod pass_life;
pub mod update_graph;
pub mod root;
pub mod pass_dirty;
pub mod world_invert;
pub mod content_box;
pub mod calc_render_steady;


pub struct UiPassPlugin;

impl Plugin for UiPassPlugin {
    fn build(&self, app: &mut App) {
        // 设置运行条件和运行顺序
        app
            // 创建、删除Pass，为Pass组织树结构
            .add_system(UiStage, 
				calc_pass
					.after(calc_camera)
                    .after(UiSystemSet::PassFlush)
					
			)
            .add_system(UiStage, pass_life::cal_pass_life.run_if(pass_life::pass_life_change).in_set(UiSystemSet::PassLife))
            .add_system(UiStage, pass_life::cal_context
                .run_if(pass_life::pass_life_change)
                .after(UiSystemSet::PassFlush) // 在上下文创建之后执行
                .in_set(UiSystemSet::PassSetting))
            // .add_system(UiStage, apply_deferred.in_set(UiSystemSet::PassFlush))
            .add_system(UiStage, 
                pass_life::calc_pass_children_and_clear
                    .in_set(UiSystemSet::PassSetting)
                    .run_if(pass_life_children)
                    .after(pass_life::cal_context)
					.before(UiSystemSet::PassSettingWithParent) // 在所有依赖父子关系的system之前执行
                    .after(UiSystemSet::PassFlush), // 在上下文创建之后执行
            )
            .add_system(UiStage, pass_life::calc_pass_toop_sort
                // .run_if(pass_life_children)
                .after(update_graph::update_graph)
            )
            .add_startup_system(UiStage, init_root_graph)
            // 计算图节点及其依赖
            .add_system(UiStage, update_graph::update_graph
                .after(UiSystemSet::PassSettingWithParent)
                .after(UiSystemSet::PassSetting)
            )
            // 渲染前，计算Pass的属性
            // 脏区域、相机、深度，更新uniform不顶点buffer到wgpu
            // .add_system(UiStage, pass_dirty_rect::calc_global_dirty_rect.in_set(UiSystemSet::PassCalc))
            .add_system(UiStage, 
                pass_camera::calc_pass_dirty
                    // .after(pass_dirty_rect::calc_global_dirty_rect)
					.after(UiSystemSet::BaseCalcFlush)
                    .in_set(UiSystemSet::PassCalc),
            )
            .add_system(UiStage, 
                pass_camera::calc_pass_active
                    // .after(pass_dirty_rect::calc_global_dirty_rect)
                    .after(update_graph::update_graph)
					.after(UiSystemSet::BaseCalcFlush),
            )
            
            .add_system(UiStage, 
                pass_camera::calc_camera
                    .after(pass_camera::calc_pass_dirty)
                    .before(pass_camera::calc_pass_active)
                    .after(calc_render_steady::calc_is_steady)
					.after(UiSystemSet::BaseCalcFlush)
                    .in_set(UiSystemSet::PassCalc),
            )
			
            .add_system(PostUpdate, 
                last_update_wgpu::last_update_wgpu
                    .after(GraphBuild)
					.before(GraphRun)
                    ,
            )
            .add_system(UiStage, root::root_calc.in_set(UiSystemSet::PassMark))
            .add_plugins(world_invert::WorldInvertPlugin)
            .add_plugins(content_box::ContentBoxPlugin)
            .add_plugins(calc_render_steady::RenderSteadyPlugin)
        ;
    }
}
