use bevy_ecs::prelude::Resource;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

#[cfg(feature = "debug")]
pub mod cmd_play;
pub mod draw_obj;
pub mod node;
pub mod pass;
pub mod pass_effect;
pub mod shader_utils;
pub mod system_set;
pub mod utils;
pub mod res_load; //外部进行资源加载

// 运行状态
bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Resource, Default, Serialize, Deserialize)]
    pub struct RunState: u32 {
        const NONE                       = 0;
        const SETTING            = (1 << 0); // 设置
        const LAYOUT      = (1 << 1); // 计算布局
        const MATRIX     = (1 << 2); // 计算世界矩阵
        // const RENDER     = (1 << 2); // 渲染
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

