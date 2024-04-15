//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行

use std::mem::{replace, transmute};

use bevy_ecs::{
    prelude::{Entity, World},
    query::Changed,
    removal_detection::RemovedComponents,
    system::{Local, Query, ResMut, SystemState}, event::EventWriter,
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_time::Instant;

use crate::{
    components::user::{
        serialize::{Setting, StyleQuery},
        Animation,
    },
    resource::{
        animation_sheet::{KeyFramesSheet, ObjKey},
        StyleCommands, TimeInfo, UserCommands,
    }, system::draw_obj::calc_text::IsRun,
};

use super::user_setting::{set_styles, StyleChange, StyleDirtyList};

/// * 记录帧推时间（暂时性的，时间应该是全局共享的，应该挪到pi_bevy_render,组委共享资源）
/// * 为删除了Animation组件的节点，解绑动画
/// * 为修改了Animation组件的节点，绑定动画
/// * 推动动画执行
/// * 将动画执行结果作用到组件上
pub fn calc_animation(
    world: &mut World,
    style_query: Local<StyleQuery>,

    animation: &mut SystemState<(
        Query<(Entity, &'static Animation), Changed<Animation>>,
        RemovedComponents<Animation>,
        ResMut<KeyFramesSheet>,
        ResMut<TimeInfo>,
        ResMut<UserCommands>,
		OrInitRes<IsRun>,
		EventWriter<StyleChange>,
    )>,

    user_commands: &mut SystemState<ResMut<UserCommands>>,
	mut dirty_mark: Local<bitvec::vec::BitVec<usize>>,
) {

    let time = Instant::now();

    let (animation, mut del, mut keyframes_sheet, mut time_info, mut user_commands1, r, events) = animation.get_mut(world);

	if r.0 {
		return;
	}
    // let t1 = std::time::Instant::now();

	// 此处强制转换是安全的， 本system逻辑保证， events访问不会读写冲突， 且生命周期足够
	let events: EventWriter<'static, StyleChange> = unsafe { transmute(events) };
	let mut dirty_list = StyleDirtyList {
		list: events,
		mark: &mut *dirty_mark,
	};
	
    *time_info = TimeInfo {
        cur_time: time,
        delta: (time - time_info.cur_time).as_millis() as u64,
    };

    // 解绑定动画
    for del in del.iter() {
        if let Err(_) = animation.get(del) {
            keyframes_sheet.unbind_animation_all(ObjKey(del));
            keyframes_sheet.remove_runtime_keyframs(ObjKey(del));
        }
    }
    // let t2 = std::time::Instant::now();

    // 绑定动画
    for (node, animation) in animation.iter() {
        if let Err(e) = keyframes_sheet.bind_static_animation(ObjKey(node), animation) {
            log::error!("{:?}", e);
        }
    }
    // let t3 = std::time::Instant::now();
    // log::warn!("time_info.delta==============={:?}", time_info.delta);
    // 推动动画执行
    keyframes_sheet.run(&mut user_commands1.style_commands, time_info.delta);
    // let t4 = std::time::Instant::now();
    let mut commands = replace(&mut user_commands1.style_commands, StyleCommands::default());

    let mut setting = Setting { style: &style_query, world };
    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut commands, &mut setting, &mut dirty_list);
    // let t5 = std::time::Instant::now();
    // println!("calc_animation time: {:?}", (t5 - t4, t4-t3,t3-t2,t2-t1));
    user_commands.get_mut(world).style_commands = commands;
}
