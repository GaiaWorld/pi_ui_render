//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行


use pi_world::{prelude::{Changed, Entity, Has, ParamSet, Query, Removed, SingleResMut}, system_params::Local, world::World};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_time::Instant;

use crate::{
    components::{user::{
        serialize::{DefaultStyle, Setting},
        Animation,
    }, SettingComponentIds},
    resource::{
        animation_sheet::KeyFramesSheet, fragment::NodeTag, TimeInfo, UserCommands
    }, system::draw_obj::calc_text::IsRun,
};

use super::user_setting::{set_styles, SingleId};

/// * 记录帧推时间（暂时性的，时间应该是全局共享的，应该挪到pi_bevy_render,组委共享资源）
/// * 为删除了Animation组件的节点，解绑动画
/// * 为修改了Animation组件的节点，绑定动画
/// * 推动动画执行
pub fn calc_animation_1(
    // world: &mut World,
    mut style_query: ParamSet<(
        Query<(Entity, &'static Animation), Changed<Animation>>,
        Query<(Has<&'static Animation>, Entity), Removed<Animation>>,
    )>,
    mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
    mut time_info: SingleResMut<TimeInfo>,
    mut user_commands: SingleResMut<UserCommands>,
    r: OrInitSingleRes<IsRun>,
) {

    let time = Instant::now();

	if r.0 {
		return;
	}
	
    *time_info = TimeInfo {
        cur_time: time,
        delta: (time - time_info.cur_time).as_millis() as u64,
    };

    // 解绑定动画
    for (has_animation, del) in style_query.p1().iter() {
        if has_animation {
            continue;
        }
        keyframes_sheet.unbind_animation_all(del);
        keyframes_sheet.remove_runtime_keyframs(del);
    }

    // 绑定动画
    for (node, animation) in style_query.p0().iter() {
        if let Err(e) = keyframes_sheet.bind_static_animation(node, animation) {
            log::error!("{:?}", e);
        }
    }

    // 推动动画执行
    keyframes_sheet.run(&mut user_commands.style_commands, time_info.delta);
}


/// * 将动画执行结果作用到组件上
pub fn calc_animation_2(
    world: &mut World,
    id: Local<SingleId>,
    setting_components: Local<SettingComponentIds>,
    default_style: Local<DefaultStyle>,
) {
    let mut w1 = world.unsafe_world();

    let user_commands = w1.index_single_res_mut::<UserCommands>(id.user_commands).unwrap().0;

    // let t4 = std::time::Instant::now();
    // let mut commands = replace(&mut user_commands.style_commands, StyleCommands::default());
    let mut setting = Setting {world,  style: &setting_components, default_value: &default_style};
    // 添加基础组件id
    let base_component_ids = UserCommands::init_component_ids(NodeTag::Div, &setting_components);
    // 添加基础组件id
    let v_node_base_component_ids = UserCommands::init_component_ids(NodeTag::VNode, &setting_components);
    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, base_component_ids, v_node_base_component_ids);
}
