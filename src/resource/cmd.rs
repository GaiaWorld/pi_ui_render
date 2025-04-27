//! 指令

use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    mem::replace, sync::Arc,
};

use pi_share::Share;
use pi_world::{prelude::{Bundle, Command, Entity, World}, world::FromWorld};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_bevy_render_plugin::PiClearOptions;
use pi_hal::font::sdf_table::FontCfg;
use pi_hash::XHashMap;
use pi_style::{
    style::CgColor,
    style_parse::{Attribute, ClassItem, ClassMap, KeyFrameList},
};
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        calc::{CanvasGraph, EntityKey}, user::{
            serialize::{DefaultStyle, StyleTypeReader},
            Animation, AsImage, Canvas, RenderTargetType, Viewport,
        }, NodeBundle
    },
    resource::animation_sheet::KeyFramesSheet,
};

use super::{
    fragment::{FragmentMap, Fragments}, ClassSheet, RenderDirty, ShareFontSheet
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStyleCmd(pub VecDeque<Attribute>);

impl Command for DefaultStyleCmd {
    fn apply(self, world: &mut World) {
        // let mut syetem_meta = SystemMeta::new::<()>();
        // let mut state = DefaultStyle::init_state(world, &mut syetem_meta);
        // let tick = world.tick();
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
            while style_reader.write_to_default(world, &default_style_query).is_some() {}
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendCssCmd(pub Vec<pi_style::style_parse::ClassMap>);
impl Command for ExtendCssCmd {
    fn apply(self, world: &mut World) {
        for mut item in self.0.into_iter() {
            let key_frames = replace(&mut item.key_frames, KeyFrameList::default());
            // 处理帧动画
            if key_frames.frames.len() > 0 {
                log::debug!("create keyframs, count: {}", key_frames.frames.len());
                let keyframes_sheet = world.get_single_res_mut::<KeyFramesSheet>().unwrap();
                for (name, value) in key_frames.frames.into_iter() {
                    keyframes_sheet.add_static_keyframes(key_frames.scope_hash, name, value);
                }
            }

            // 处理css
            let class_sheet_single = world.get_single_res_mut::<ClassSheet>().unwrap();
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
    fn apply(self, world: &mut World) {
        let fragment_map = world.get_single_res_mut::<FragmentMap>().unwrap();
        fragment_map.extend(self.0);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCmd<T: 'static + Send + Sync>(pub T, pub Entity);
impl<T: Bundle + 'static + Send + Sync> Command for NodeCmd<T> {
    fn apply(self, world: &mut World) {
        if world.contains_entity(self.1) {
            pi_print_any::out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            let id = world.init_component::<T>();
            let _ = world.make_entity_editor().alter_components_by_index(self.1,&[(id, true)]);
            if let Ok(mut r) = world.get_component_mut_by_index(self.1, id) {
                *r = self.0;
            }
        } else {
            pi_print_any::out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostProcessCmd(pub EntityKey, pub Entity);
impl Command for PostProcessCmd {
    fn apply(self, world: &mut World) {
        if world.contains_entity(self.1) {
            pi_print_any::out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            let id = world.init_component::<AsImage>();
            let _ = world.make_entity_editor().alter_components_by_index(self.1,&[(id, true)]);
            if let Ok(mut r) = world.get_component_mut_by_index::<AsImage>(self.1, id) {
                r.post_process = self.0;
            }
        } else {
            pi_print_any::out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasCmd(pub Entity, pub bool, pub Entity);
impl Command for CanvasCmd {
    fn apply(self, world: &mut World) {
        if world.contains_entity(self.2) {
            pi_print_any::out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            let id = world.init_component::<Canvas>();
            let id1 = world.init_component::<CanvasGraph>();
            let _ = world.make_entity_editor().alter_components_by_index(self.2,&[(id, true), (id1, true)]);
            if let Ok(mut r) = world.get_component_mut_by_index::<Canvas>(self.2, id) {
                r.id = self.0;
                r.by_draw_list = self.1;
            }
        } else {
            pi_print_any::out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentCmd<T>(pub T, pub Entity);
impl<T: 'static + Send + Sync> Command for ComponentCmd<T> {
    fn apply(self, world: &mut World) {
        if world.contains_entity(self.1) {
            pi_print_any::out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            let id = world.init_component::<T>();
            let _ = world.make_entity_editor().alter_components_by_index(self.1,&[(id, true)]);
            if let Ok(mut r) = world.get_component_mut_by_index::<T>(self.1, id) {
                *r = self.0;
            }
        } else {
            pi_print_any::out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeAnimationBindCmd(pub XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>, pub Animation, pub Entity);
impl Command for RuntimeAnimationBindCmd {
    fn apply(self, world: &mut World) {
        if world.contains_entity(self.2) {
            let id = world.init_component::<Animation>();
            let _ = world.make_entity_editor().alter_components_by_index(self.2,&[(id, true)]);
            if let Ok(mut r) = world.get_component_mut_by_index::<Animation>(self.2, id) {
                // 这里设置animation的值， 后续在style指令中还会再设置一次
                // 因为有可能， 在此指令执行后， style设置前， 节点被销毁了， 如果此时动画没有值， 不能销毁掉添加的动画帧
                *r = self.1.clone();
            }
            let sheet = world.get_single_res_mut::<KeyFramesSheet>().unwrap();
            let _ = sheet.add_runtime_keyframes(self.2, &self.1, self.0);
        }
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontCfgCmd(pub FontCfg);
impl Command for FontCfgCmd {
    fn apply(self, world: &mut World) {
        let sheet = &****world.get_single_res_mut::<ShareFontSheet>().unwrap();
        let mut sheet = (*sheet).borrow_mut();
        sheet.font_mgr_mut().add_sdf_cfg(self.0);
    }
}

// 添加sdf2字体指令
#[derive(Clone, Debug)]
pub struct FontSdf2Cmd(pub Atom, pub Share<Vec<u8>>);
impl Command for FontSdf2Cmd {
    fn apply(self, world: &mut World) {
        let sheet = &****world.get_single_res_mut::<ShareFontSheet>().unwrap();
        let mut sheet = (*sheet).borrow_mut();
        let face_id = sheet.font_mgr_mut().create_font_face(&self.0);
        sheet.font_mgr_mut().table.sdf2_table.add_font(face_id, self.1);
    }
}

impl Serialize for FontSdf2Cmd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut serde_state = serde::Serializer::serialize_tuple_struct(
            serializer,
            "FontSdf2Cmd",
            2,
        )?;
        serde::ser::SerializeTupleStruct::serialize_field(
            &mut serde_state,
            &self.0,
        )?;
        serde::ser::SerializeTupleStruct::serialize_field(
            &mut serde_state,
            &*self.1,
        )?;
        serde::ser::SerializeTupleStruct::end(serde_state)
    }
}

impl<'de> Deserialize<'de> for FontSdf2Cmd {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        #[doc(hidden)]
        struct Visitor<'de> {
            marker: serde::__private::PhantomData<FontSdf2Cmd>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
            type Value = FontSdf2Cmd;
            fn expecting(
                &self,
                formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    formatter,
                    "tuple struct FontSdf2Cmd",
                )
            }
            #[inline]
            fn visit_seq<__A>(
                self,
                mut __seq: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let field0 = match serde::de::SeqAccess::next_element::<
                    Atom,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"tuple struct FontSdf2Cmd with 2 elements",
                            ),
                        );
                    }
                };
                let field1 = match serde::de::SeqAccess::next_element::<
                    Vec<u8>,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                1usize,
                                &"tuple struct FontSdf2Cmd with 2 elements",
                            ),
                        );
                    }
                };
                serde::__private::Ok(FontSdf2Cmd(field0, Share::new(field1)))
            }
        }
        serde::Deserializer::deserialize_tuple_struct(
            deserializer,
            "B",
            2usize,
            Visitor {
                marker: serde::__private::PhantomData::<FontSdf2Cmd>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SdfDefaultCharCmd {
    pub font_face: Atom,
    pub char: char,
}
impl Command for SdfDefaultCharCmd {
    fn apply(self, world: &mut World) {
        let sheet = &****world.get_single_res::<ShareFontSheet>().unwrap();
        let mut sheet = (*sheet).borrow_mut();
        sheet.borrow_mut().font_mgr_mut().add_sdf_default_char(self.font_face, self.char);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClearColorCmd(pub CgColor);

impl Command for ClearColorCmd {
    fn apply(self, world: &mut World) {
        let color = wgpu::Color {
            r: self.0.x as f64,
            g: self.0.y as f64,
            b: self.0.z as f64,
            a: self.0.w as f64,
        };
        match world.get_single_res_mut::<PiClearOptions>() {
            Some(r) => r.color = color,
            None => {
                let mut option = PiClearOptions::default();
                option.color = color;
                world.insert_single_res(option);
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderDirtyCmd(pub RenderDirty);

impl Command for RenderDirtyCmd {
    fn apply(self, world: &mut World) {
        match world.get_single_res_mut::<RenderDirty>() {
            Some(r) => **r = self.0,
            None => {
                world.insert_single_res(self.0);
            }
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
    pub key: u32,             // 模板的key
    pub entitys: Vec<Entity>, // 模板对应的实体
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CmdType {
    RuntimeAnimationBindCmd(RuntimeAnimationBindCmd),
    ComponentCmd(ComponentCmd<Canvas>),

    NodeCmdViewport(NodeCmd<Viewport>),
    NodeCmdRenderTargetType(NodeCmd<RenderTargetType>),
    NodeCmdRenderClearColor(ClearColorCmd),
    NodeCmdRenderRenderDirty(RenderDirtyCmd),
    NodeCmdRenderNodeBundle(NodeCmd<NodeBundle>),

    ExtendFragmentCmd(ExtendFragmentCmd),
    ExtendCssCmd(ExtendCssCmd),
    DefaultStyleCmd(DefaultStyleCmd),
    SdfCfgCmd(FontCfgCmd),
    SdfDefaultCharCmd(SdfDefaultCharCmd),
    Sdf2CfgCmd(FontSdf2Cmd),
    // SVG
    // SvgStrokeCmd(SvgStrokeColorCmd),
    // SvgColorCmd(SvgColorCmd),
    // SvgStrokeWidthCmd(SvgStrokeWidthCmd),
    // StrokeDasharrayCmd(StrokeDasharrayCmd),
    // SvgShapeCmd(SvgShapeCmd),
    // SvgShapeWidthCmd(SvgShapeWidthCmd),
    // SvgShapeHeightCmd(SvgShapeHeightCmd),
    // SvgShapeXCmd(SvgShapeXCmd),
    // SvgShapeYCmd(SvgShapeYCmd),
}

// #[derive(Clone)]
// pub struct AnimationListenCmd(pub Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>) + Send + Sync + 'static>);
// impl Command for AnimationListenCmd {
//     fn write(self, world: &mut World) {
// 		world.get_single_res_mut::<KeyFramesSheet>().unwrap().set_event_listener(self.0);
//     }
// }

