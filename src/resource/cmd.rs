//! 指令

use std::{collections::VecDeque, mem::replace};

use bevy::{
    ecs::{
        prelude::{Bundle, Entity},
        system::Command,
        world::{FromWorld, World},
    },
    prelude::{Changed, Component, Events},
};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_hash::XHashMap;
use pi_print_any::out_any;
use pi_style::{style_parse::{Attribute, ClassItem, ClassMap, KeyFrameList}};
use serde::{Serialize, Deserialize};

use crate::{
    components::{user::{serialize::{DefaultStyle, StyleTypeReader}, Animation, Viewport, RenderTargetType, ClearColor, RenderDirty, Canvas}, NodeBundle},
    resource::animation_sheet::KeyFramesSheet,
};

use super::{ClassSheet, animation_sheet::ObjKey, fragment::{FragmentMap, Fragments}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStyleCmd(pub VecDeque<Attribute>);

impl Command for DefaultStyleCmd {
    fn write(self, world: &mut World) {
        let default_style_query = DefaultStyle::from_world(world);

        let len = self.0.len();
        let class_map = ClassMap {
            attrs: self.0,
            classes: vec![ClassItem { count: len, class_name: 0 }],
            key_frames: Default::default(),
        };
        let mut class_sheet = pi_style::style_type::ClassSheet::default();
        class_map.to_class_sheet(&mut class_sheet);

        if let Some(class) = class_sheet.class_map.get(&0) {
            let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
            while style_reader.write_to_default(&default_style_query, world).is_some() {}
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendCssCmd(pub Vec<pi_style::style_parse::ClassMap>);
impl Command for ExtendCssCmd {
    fn write(self, world: &mut World) {
        for mut item in self.0.into_iter() {
            let key_frames = replace(&mut item.key_frames, KeyFrameList::default());
            // 处理帧动画
            if key_frames.frames.len() > 0 {
                log::debug!("create keyframs, count: {}", key_frames.frames.len());
                let mut keyframes_sheet = world.get_resource_mut::<KeyFramesSheet>().unwrap();
                for (name, value) in key_frames.frames.into_iter() {
                    keyframes_sheet.add_static_keyframes(key_frames.scope_hash, name, value);
                }
            }

            // 处理css
            let mut class_sheet_single = world.get_resource_mut::<ClassSheet>().unwrap();
            let mut class_sheet = pi_style::style_type::ClassSheet::default();
            item.to_class_sheet(&mut class_sheet);
            class_sheet_single.extend_from_class_sheet(class_sheet);
        }
    }
}

/// 添加模板
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendFragmentCmd(pub Fragments);
impl Command for ExtendFragmentCmd {
    fn write(self, world: &mut World) {
		let mut fragment_map = world.get_resource_mut::<FragmentMap>().unwrap();
        fragment_map.extend(self.0);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCmd<T>(pub T, pub Entity);
impl<T: Bundle> Command for NodeCmd<T> {
    fn write(self, world: &mut World) {
        if let Some(mut r) = world.get_entity_mut(self.1) {
            out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            r.insert(self.0);
        } else {
            out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentCmd<T>(pub T, pub Entity);
impl<T: Component> Command for ComponentCmd<T> {
    fn write(self, world: &mut World) {
        if let Some(mut r) = world.get_entity_mut(self.1) {
            out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            r.insert(self.0);
            if let Some(mut r) = world.get_resource_mut::<Events<ComponentEvent<Changed<T>>>>() {
                r.send(ComponentEvent::new(self.1));
            }
        } else {
            out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeAnimationBindCmd(pub XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>, pub Animation, pub Entity);
impl Command for RuntimeAnimationBindCmd {
    fn write(mut self, world: &mut World) {
		if world.get_entity(self.2).is_some() {
			let mut sheet = world.get_resource_mut::<KeyFramesSheet>().unwrap();
			self.1.name.scope_hash = self.2.index() as usize; // 因为每个运行时动画是节点独有的，以节点的index作为scope_hash(不能同时有两个index相等的实体)
			let _ = sheet.add_runtime_keyframes(ObjKey(self.2), &self.1, self.0);
			world.entity_mut(self.2).insert(self.1);
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 创建模板（作用该指令时， 注意检查entitys的长度和模板实际的长度是否相等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentCommand {
	pub key: u32, // 模板的key
	pub entitys: Vec<Entity>, // 模板对应的实体
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CmdType {
	RuntimeAnimationBindCmd(RuntimeAnimationBindCmd),
	ComponentCmd(ComponentCmd<Canvas>),

	NodeCmdViewport(NodeCmd<Viewport>),
	NodeCmdRenderTargetType(NodeCmd<RenderTargetType>),
	NodeCmdRenderClearColor(NodeCmd<ClearColor>),
	NodeCmdRenderRenderDirty(NodeCmd<RenderDirty>),
	NodeCmdRenderNodeBundle(NodeCmd<NodeBundle>),

	ExtendFragmentCmd(ExtendFragmentCmd),
	ExtendCssCmd(ExtendCssCmd),
	DefaultStyleCmd(DefaultStyleCmd),
}

// #[derive(Clone)]
// pub struct AnimationListenCmd(pub Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>) + Send + Sync + 'static>);
// impl Command for AnimationListenCmd {
//     fn write(self, world: &mut World) {
// 		world.get_resource_mut::<KeyFramesSheet>().unwrap().set_event_listener(self.0);
//     }
// }
