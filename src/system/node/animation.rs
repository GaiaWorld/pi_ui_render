//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行


use pi_world::prelude::{ParamSet, Changed, Entity, Has, Query, Removed, SingleResMut, With};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_time::Instant;

use crate::{
    components::{calc::StyleMark, user::{
        serialize::{DefaultStyle, Setting, StyleQuery},
        Animation, Size,
    }},
    resource::{
        animation_sheet::KeyFramesSheet,
        TimeInfo, UserCommands,
    }, system::draw_obj::calc_text::IsRun,
};

use super::user_setting::set_styles;

/// * 记录帧推时间（暂时性的，时间应该是全局共享的，应该挪到pi_bevy_render,组委共享资源）
/// * 为删除了Animation组件的节点，解绑动画
/// * 为修改了Animation组件的节点，绑定动画
/// * 推动动画执行
/// * 将动画执行结果作用到组件上
pub fn calc_animation(
    // world: &mut World,
    mut style_query: ParamSet<(
        (StyleQuery, Query<(), With<Size>>),
        Query<(Entity, &'static Animation), Changed<Animation>>,
        Query<(Has<&'static Animation>, Entity), Removed<Animation>>,
    )>,
    mut style_mark: Query<&mut StyleMark>,
    mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
    mut time_info: SingleResMut<TimeInfo>,
    mut user_commands: SingleResMut<UserCommands>,
    mut default_style: DefaultStyle,
    r: OrInitSingleRes<IsRun>,


    // animation: &mut SystemState<(
    //     Query<(Entity, &'static Animation), Changed<Animation>>,
    //     RemovedComponents<Animation>,
    //     SingleResMut<KeyFramesSheet>,
    //     SingleResMut<TimeInfo>,
    //     SingleResMut<UserCommands>,
	// 	OrInitSingleRes<IsRun>,
	// 	EventWriter<StyleChange>,
    // )>,

    // user_commands: &mut SystemState<SingleResMut<UserCommands>>,
	// mut dirty_mark: Local<bitvec::vec::BitVec<usize>>,
) {

    let time = Instant::now();

	if r.0 {
		return;
	}
    // let t1 = std::time::Instant::now();

	// 此处强制转换是安全的， 本system逻辑保证， events访问不会读写冲突， 且生命周期足够
	// let events: EventWriter<'static, StyleChange> = unsafe { transmute(events) };
	// let mut dirty_list = StyleDirtyList {
	// 	// list: events,
	// 	mark: &mut *dirty_mark,
	// };
	
    *time_info = TimeInfo {
        cur_time: time,
        delta: (time - time_info.cur_time).as_millis() as u64,
    };

    // 解绑定动画
    for (has_animation, del) in style_query.p2().iter() {
        if has_animation {
            continue;
        }
        keyframes_sheet.unbind_animation_all(del);
        keyframes_sheet.remove_runtime_keyframs(del);
    }
    // let t2 = std::time::Instant::now();

    // 绑定动画
    for (node, animation) in style_query.p1().iter() {
        if let Err(e) = keyframes_sheet.bind_static_animation(node, animation) {
            log::error!("{:?}", e);
        }
    }
    // let t3 = std::time::Instant::now();
    // log::warn!("time_info.delta==============={:?}", time_info.delta);
    // 推动动画执行
    keyframes_sheet.run(&mut user_commands.style_commands, time_info.delta);
    // let t4 = std::time::Instant::now();
    // let mut commands = replace(&mut user_commands.style_commands, StyleCommands::default());
    let mut setting = Setting { style: &mut style_query.p0().0, default_value: &mut default_style};
    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, &mut style_mark);
    // let t5 = std::time::Instant::now();
    // println!("calc_animation time: {:?}", (t5 - t4, t4-t3,t3-t2,t2-t1));
    // user_commands.get_mut(world).style_commands = commands;
}
