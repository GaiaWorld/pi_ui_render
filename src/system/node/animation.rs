//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行

use std::mem::replace;

use bevy::ecs::{
    prelude::{Entity, World},
    query::Changed,
    system::{Local, Query, RemovedComponents, Res, ResMut, SystemState},
};

use crate::{
    components::user::{
        serialize::{Setting, StyleQuery},
        Animation,
    },
    resource::{
        animation_sheet::{KeyFramesSheet, ObjKey},
        StyleCommands, TimeInfo, UserCommands,
    },
};

use super::user_setting::set_style;


pub fn calc_animation(
    world: &mut World,
    style_query: Local<StyleQuery>,

    animation: &mut SystemState<(
        Query<(Entity, &'static Animation), Changed<Animation>>,
        RemovedComponents<Animation>,
        ResMut<KeyFramesSheet>,
        Res<TimeInfo>,
        ResMut<UserCommands>,
    )>,

    user_commands: &mut SystemState<ResMut<UserCommands>>,
) {
    let (animation, del, mut keyframes_sheet, cur_time, mut user_commands1) = animation.get_mut(world);
    // 解绑定动画
    for del in del.iter() {
        if let Err(_) = animation.get(del) {
            keyframes_sheet.unbind_animation(ObjKey(del));
        }
    }

    // 绑定动画
    for (node, animation) in animation.iter() {
        if let Err(e) = keyframes_sheet.bind_animation(ObjKey(node), animation) {
            log::error!("{:?}", e);
        }
    }

    // 推动动画执行
    keyframes_sheet.run(&mut user_commands1.style_commands, cur_time.delta);

    let mut commands = replace(&mut user_commands1.style_commands, StyleCommands::default());

    let mut setting = Setting { style: &style_query, world };
    // 设置style只要节点存在,样式一定能设置成功
    set_style(&mut commands, &mut setting);

    user_commands.get_mut(world).style_commands = commands;
}
