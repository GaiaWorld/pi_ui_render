//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行


use pi_style::style::StyleType;
use pi_world::{event::EventSender, prelude::{Changed, ComponentRemoved, Entity, Has, ParamSet, Query, SingleResMut}, single_res::SingleRes, system::{SystemMeta, TypeInfo}, system_params::{Local, SystemParam}, world::World};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_time::Instant;

use crate::{
    components::{calc::{style_bit, StyleBit, StyleMarkType}, user::{
        serialize::{DefaultStyle, Setting},
        Animation,
    }, SettingComponentIds},
    resource::{
        animation_sheet::KeyFramesSheet, GlobalDirtyMark, TimeInfo, UserCommands, IsRun
    }
};

use super::user_setting::{set_styles, SingleId, StyleChange, StyleDirtyList, StyleDirtyMark};

lazy_static! {
	pub static ref ANIMATION_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::AnimationDelay as usize)
		.set_bit(StyleType::AnimationDirection as usize)
		.set_bit(StyleType::AnimationDuration as usize)
		.set_bit(StyleType::AnimationFillMode as usize)
		.set_bit(StyleType::AnimationIterationCount as usize)
		.set_bit(StyleType::AnimationName as usize)
		.set_bit(StyleType::AnimationPlayState as usize)
		.set_bit(StyleType::AnimationTimingFunction as usize);
}

pub fn animation_change(mark: &GlobalDirtyMark) -> bool {
	mark.mark.has_any(&*ANIMATION_DIRTY)
}

/// * 记录帧推时间（暂时性的，时间应该是全局共享的，应该挪到pi_bevy_render,组委共享资源）
/// * 为删除了Animation组件的节点，解绑动画
/// * 为修改了Animation组件的节点，绑定动画
/// * 推动动画执行
pub fn calc_animation_1(
    // world: &mut World,
    mut style_query: ParamSet<(
        Query<(Entity, &'static Animation), Changed<Animation>>,
        Query<Has<&'static Animation>>,
    )>,
    removed: ComponentRemoved<Animation>,
    mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
    mut time_info: SingleResMut<TimeInfo>,
    mut user_commands: SingleResMut<UserCommands>,
    global_mark: SingleRes<GlobalDirtyMark>,
    r: OrInitSingleRes<IsRun>,

    // a: ComponentDebugIndex<Animation>,
) {

    let time = Instant::now();

	if r.0 {
		return;
	}
	
    *time_info = TimeInfo {
        cur_time: time,
        delta: (time - time_info.cur_time).as_millis() as u64,
    };
    // let time0 = pi_time::Instant::now();
    // 解绑定动画
    let p1 = style_query.p1();
    if removed.len() > 0 {
        for del in removed.iter() {
            if let Ok(has_animation) = p1.get(*del) {
                if has_animation {
                    continue;
                }
                keyframes_sheet.unbind_animation_all(*del);
                keyframes_sheet.remove_runtime_keyframs(*del);
            }
        }
    }
    
    
    // let time1 = pi_time::Instant::now();
    // 绑定动画
    if animation_change(&*global_mark) {
        // log::warn!("aaa========={:?}", a.0);
        for (node, animation) in style_query.p0().iter() {
            if let Err(e) = keyframes_sheet.bind_static_animation(node, animation) {
                log::error!("{:?}", e);
            }
        }
    }
    

    // let time2 = pi_time::Instant::now();
    // 推动动画执行
    keyframes_sheet.run(&mut user_commands.style_commands, time_info.delta);
    // println!("animation1====={:?}", (time_info.delta));
    // let time3 = pi_time::Instant::now();
    // println!("animation1====={:?}", (time1 - time0, time2 - time1, time3 - time2, user_commands.style_commands.style_buffer.len()));
}


/// * 将动画执行结果作用到组件上
pub fn calc_animation_2(
    world: &mut World,
    id: Local<SingleId>,
    setting_components: Local<SettingComponentIds>,
    default_style: Local<DefaultStyle>,
) {
    // let time1 = pi_time::Instant::now();
    let mut w1 = world.unsafe_world();
    let mut w2 = world.unsafe_world();
    let mut w3 = world.unsafe_world();
    let w4 = world.unsafe_world();
    let mut w5 = world.unsafe_world();

    let mut global_mark = w5.index_single_res_mut::<GlobalDirtyMark>(id.global_mark).unwrap();
    let user_commands = w1.index_single_res_mut::<UserCommands>(id.user_commands).unwrap();
    let dirty_mark = w2.index_single_res_mut::<StyleDirtyMark>(id.style_dirty_mark).unwrap();
    let mut s_meta = SystemMeta::new(TypeInfo::of::<()>());
    let mut events = EventSender::<'_, StyleChange>::init_state(&mut w3, &mut s_meta);

    let mut dirty_list = StyleDirtyList {
		list: EventSender::<'_, StyleChange>::get_param(&w4, &mut s_meta, &mut events, world.tick()),
		mark: &mut dirty_mark.0,
	};
    // let t4 = std::time::Instant::now();
    // let mut commands = replace(&mut user_commands.style_commands, StyleCommands::default());
    let mut setting = Setting {world,  style: &setting_components, default_value: &default_style};

    let base_component_ids = Vec::with_capacity(1);
    let v_node_base_component_ids = Vec::with_capacity(1);
    // let time2 = pi_time::Instant::now();
    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, base_component_ids, v_node_base_component_ids, &mut dirty_list, &mut global_mark);
    // let time3 = pi_time::Instant::now();
    // println!("animation2====={:?}", (time2 - time1, time3 - time2));
}
