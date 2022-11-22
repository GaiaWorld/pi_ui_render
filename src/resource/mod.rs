pub mod animation_sheet;
pub mod draw_obj;


use pi_time::Instant;

// use pi_ecs::prelude::{FromWorld, Id, World};
use bevy::prelude::{Resource, Entity, FromWorld, World};

use crate::{
    components::{
        calc::StyleMark,
        user::{ClassName, TextContent},
    },
    utils::cmd::CommandQueue,
};

#[derive(Default, Deref, DerefMut, Resource)]
pub struct ClassSheet(pi_style::style_type::ClassSheet);

/// 用户指令缓冲区
#[derive(Default)]
pub struct UserCommandsCache(pub UserCommands);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct MuexUserCommands(pi_share::ShareMutex<UserCommands>);

/// 用户指令

#[derive(Default, Resource)]
pub struct UserCommands {
    /// 节点指令
    pub node_commands: Vec<NodeCommand>,
    /// 样式指令
    pub style_commands: StyleCommands,
    /// 文本指令
    pub text_commands: Vec<(Entity, Option<TextContent>)>,
    /// class指令
    pub class_commands: Vec<(Entity, ClassName)>,

    // css 内容增加指令
    pub css_commands: Vec<ClassSheet>,

    /// single指令
    pub other_commands: CommandQueue,
}

/// 节点指令
pub enum NodeCommand {
    /// 插入节点（节点，父节点）,
    AppendNode(Entity, Entity),
    /// 插入节点（节点，锚点）,
    InsertBefore(Entity, Entity),
    /// 删除节点,
    RemoveNode(Entity),
    /// 销毁节点
    DestroyNode(Entity),
}

/// style设置指令
#[derive(Default)]
pub struct StyleCommands {
    /// 样式列表
    // pub style_list: Vec<Attribute>,
    pub style_buffer: Vec<u8>,
    /// 设置样式（节点，开始索引，结束索引），其中开始索引和结束索引是指在style_list中的索引
    pub commands: Vec<(Entity, usize, usize)>,
}

#[derive(Default)]
pub struct DefaultStyle;
#[derive(Default)]
pub struct DefaultStyleMark(pub StyleMark);

/// 渲染上下文标记分配器，每一种可以使节点成为渲染上下文的属性，都可以让全局单例RenderContextMarkAlloc分配一个id
#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct RenderContextMarkAlloc(usize);

/// 渲染上下文类型，每一种可以使节点成为渲染上下文的属性，都对应一个RenderContextMarkType，类型值是在初始化时，找RenderContextMarkAlloc分配的。
#[derive(Debug, Deref, DerefMut)]
pub struct RenderContextMarkType(usize);

impl FromWorld for RenderContextMarkType {
    fn from_world(world: &mut World) -> Self {
        let mut cur_mark_index = match world.get_resource_mut::<RenderContextMarkAlloc>() {
            Some(r) => r,
            None => {
                world.insert_resource(RenderContextMarkAlloc::default());
                world.get_resource_mut::<RenderContextMarkAlloc>().unwrap()
            }
        };
        **cur_mark_index += 1;
        Self(**cur_mark_index)
    }
}

// 当前时间
#[derive(Clone, Debug, Resource)]
pub struct TimeInfo {
    pub cur_time: Instant,
    pub delta: u64,
}

impl Default for TimeInfo {
    fn default() -> Self {
        Self {
            cur_time: Instant::now(),
            delta: 0,
        }
    }
}
