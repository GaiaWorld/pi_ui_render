//! 指令

use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    mem::replace,
};

use pi_world::{prelude::{Bundle, Command, Entity, World, SystemParam}, system::SystemMeta};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_bevy_render_plugin::PiClearOptions;
use pi_hal::font::sdf_table::FontCfg;
use pi_hash::XHashMap;
use pi_key_alloter::Key;
use pi_style::{
    style::CgColor,
    style_parse::{Attribute, ClassItem, ClassMap, KeyFrameList},
};
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        calc::EntityKey,
        user::{
            serialize::{DefaultStyle, StyleTypeReader},
            Animation, AsImage, Canvas, RenderDirty, RenderTargetType, Viewport,
        },
        NodeBundle,
    },
    resource::animation_sheet::KeyFramesSheet,
};

use super::{
    fragment::{FragmentMap, Fragments},
    ClassSheet, ShareFontSheet,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStyleCmd(pub VecDeque<Attribute>);

impl Command for DefaultStyleCmd {
    fn apply(self, world: &mut World) {
        let mut syetem_meta = SystemMeta::new::<()>();
        let mut state = DefaultStyle::init_state(world, &mut syetem_meta);
        let tick = world.tick();
        let mut default_style_query = DefaultStyle::get_self(world, &syetem_meta, &mut state, tick);

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
            while style_reader.write_to_default(&mut default_style_query).is_some() {}
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
        let mut alter = world.make_alterer::<(), (), (T, ), ()>();
        let _ = alter.alter(self.1, (self.0, ));
        // TODO
        // if let Some(mut r) = world.get_entity_mut(self.1) {
        //     out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
        //     r.insert(self.0);
        // } else {
        //     out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        // }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostProcessCmd(pub EntityKey, pub Entity);
impl Command for PostProcessCmd {
    fn apply(self, world: &mut World) {
        let mut alter = world.make_alterer::<&mut AsImage, (), (AsImage, ), ()>();
        if let Ok(mut r) = alter.get_mut(self.1) {
            r.post_process = self.0;
        } else {
            let _ = alter.alter(self.1, (AsImage {
                level: pi_style::style::AsImage::None,
                post_process: self.0,
            }, ));
        }
        
        // TODO
        // if let Some(mut r) = world.get_entity_mut(self.1) {
        //     if let Some(mut r) = r.get_mut::<AsImage>() {
        //         out_any!(log::debug, "PostProcessCmd====================node：{:?}, post {:?}", self.1, &self.0);
        //         r.post_process = self.0;
        //     } else {
        //         r.insert(AsImage {
        //             level: pi_style::style::AsImage::None,
        //             post_process: self.0,
        //         });
        //     }
        // } else {
        //     out_any!(log::debug, "PostProcess fail======================={:?}, {:?}", &self.1, &self.0);
        // }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentCmd<T>(pub T, pub Entity);
impl<T: 'static + Send + Sync> Command for ComponentCmd<T> {
    fn apply(self, world: &mut World) {
        let mut alter = world.make_alterer::<(), (), (T, ), ()>();
        let _ = alter.alter(self.1, (self.0, ));
        // TODO
        // if let Some(mut r) = world.get_entity_mut(self.1) {
        //     out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
        //     r.insert(self.0);
        //     // if let Some(mut r) = world.get_single_res_mut::<Events<ComponentEvent<Changed<T>>>>() {
        //     //     r.send(ComponentEvent::new(self.1));
        //     // }
        // } else {
        //     out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        // }
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeAnimationBindCmd(pub XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>, pub Animation, pub Entity);
impl Command for RuntimeAnimationBindCmd {
    fn apply(mut self, world: &mut World) {
        let mut alter = world.make_alterer::<(), (), (Animation, ), ()>();
        if alter.contains(self.2) {
            self.1.name.scope_hash = self.2.index() as usize; // 因为每个运行时动画是节点独有的，以节点的index作为scope_hash(不能同时有两个index相等的实体)
            let _ = alter.alter(self.2, (self.1.clone(), ));
            {let _a = alter;} // 释放alter
            let sheet = world.get_single_res_mut::<KeyFramesSheet>().unwrap();
            let _ = sheet.add_runtime_keyframes(self.2, &self.1, self.0);
        }

        // TODO
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontCfgCmd(pub FontCfg);
impl Command for FontCfgCmd {
    fn apply(self, world: &mut World) {
        let sheet = &***world.get_single_res_mut::<ShareFontSheet>().unwrap();
        let mut sheet = (*sheet).borrow_mut();
        sheet.font_mgr_mut().add_sdf_cfg(self.0);
    }
}

// 添加sdf2字体指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontSdf2Cmd(pub Atom, pub Vec<u8>);
impl Command for FontSdf2Cmd {
    fn apply(self, world: &mut World) {
        let sheet = &***world.get_single_res_mut::<ShareFontSheet>().unwrap();
        let mut sheet = (*sheet).borrow_mut();
        let face_id = sheet.font_mgr_mut().create_font_face(&self.0);
        sheet.font_mgr_mut().table.sdf2_table.add_font(face_id, self.1);
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
        let sheet = &***world.get_single_res::<ShareFontSheet>().unwrap();
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
    NodeCmdRenderRenderDirty(NodeCmd<RenderDirty>),
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

