use bevy::{
    ecs::event::Event,
    prelude::{App, Events, IntoSystemConfig, Resource},
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::should_run;

use self::system_set::UiSystemSet;

pub mod draw_obj;
pub mod node;
pub mod pass_effect;
pub mod pass;
pub mod shader_utils;
pub mod system_set;
pub mod utils;

// 运行状态
bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Resource, Default)]
    pub struct RunState: u32 {
        const NONE                       = 0;
        const SETTING            = (1 << 0); // 设置
        const LAYOUT      = (1 << 1); // 计算布局
        const MATRIX     = (1 << 2); // 计算世界矩阵
        const RENDER     = (1 << 2); // 渲染
    }
}

pub trait AddEvent {
    // 添加事件， 该实现每帧清理一次
    fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
    fn add_frame_event<T: Event>(&mut self) -> &mut Self {
        if !self.world.contains_resource::<Events<T>>() {
            self.init_resource::<Events<T>>()
                .add_system(Events::<T>::update_system.run_if(should_run).after(UiSystemSet::PassCalc));
        }
        self
    }
}


pub fn setting_run(state: OrInitRes<RunState>) -> bool {
    if **state >= RunState::SETTING {
        true
    } else {
        false
    }
}

pub fn layout_run(state: OrInitRes<RunState>) -> bool {
    if **state >= RunState::LAYOUT {
        true
    } else {
        false
    }
}

pub fn matrix_run(state: OrInitRes<RunState>) -> bool {
    if **state >= RunState::MATRIX {
        true
    } else {
        false
    }
}
pub fn render_run(state: OrInitRes<RunState>) -> bool {
    if **state >= RunState::RENDER {
        true
    } else {
        false
    }
}
