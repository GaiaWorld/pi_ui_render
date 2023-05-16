use std::{collections::VecDeque, mem::replace};

use bevy::{ecs::{
    prelude::{Bundle, Entity},
    system::Command,
    world::{FromWorld, World},
}, prelude::{Events, Changed, Component}};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_print_any::out_any;
use pi_style::style_parse::{Attribute, ClassItem, ClassMap, KeyFrameList};

use crate::{
    components::user::serialize::{DefaultStyle, StyleTypeReader},
    resource::animation_sheet::KeyFramesSheet,
};

use super::{ClassSheet};

#[derive(Debug, Clone)]
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

#[derive(Clone)]
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
                    keyframes_sheet.add_keyframes(key_frames.scope_hash, name, value);
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Debug, Clone)]
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

// #[derive(Clone)]
// pub struct AnimationListenCmd(pub Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>) + Send + Sync + 'static>);
// impl Command for AnimationListenCmd {
//     fn write(self, world: &mut World) {
// 		world.get_resource_mut::<KeyFramesSheet>().unwrap().set_event_listener(self.0);
//     }
// }
