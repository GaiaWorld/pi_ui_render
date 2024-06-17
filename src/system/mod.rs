

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
pub mod debug;

// 运行状态
bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Default, Serialize, Deserialize)]
    pub struct RunState: u32 {
        const NONE                       = 0;
        const SETTING            = (1 << 0); // 设置
        const LAYOUT      = (1 << 1); // 计算布局
        const MATRIX     = (1 << 2); // 计算世界矩阵
        // const RENDER     = (1 << 2); // 渲染
    }
}


// pub fn setting_run(state: OrInitSingleRes<RunState>, frame_state: OrInitSingleRes<FrameState>) -> bool {
//     if **state >= RunState::SETTING{
//         true
//     } else if let FrameState::Active = **frame_state {
//         true
//     } else {
//         false
//     }
// }

// pub fn layout_run(state: OrInitSingleRes<RunState>, frame_state: OrInitSingleRes<FrameState>) -> bool {
//     if **state >= RunState::LAYOUT {
//         true
//     } else if let FrameState::Active = **frame_state {
//         true
//     } else {
//         false
//     }
// }

// pub fn matrix_run(state: OrInitSingleRes<RunState>, frame_state: OrInitSingleRes<FrameState>) -> bool {
//     if **state >= RunState::MATRIX {
//         true
//     } else if let FrameState::Active = **frame_state {
//         true
//     }  else {
//         false
//     }
// }

