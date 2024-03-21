//! 指令

use std::{collections::VecDeque, mem::replace, borrow::BorrowMut};

use bevy_ecs::{
	system::Command,
	world::{FromWorld, World},
    prelude::{Changed, Component, Events, Bundle, Entity}, change_detection::DetectChangesMut,
};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiClearOptions;
use pi_hal::font::sdf_table::FontCfg;
use pi_hash::XHashMap;
use pi_print_any::out_any;
use pi_style::{style_parse::{Attribute, ClassItem, ClassMap, KeyFrameList}, style::{CgColor, StrokeDasharray, Stroke}};
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        user::{
            serialize::{DefaultStyle, StyleTypeReader},
            Animation, Canvas, RenderDirty, RenderTargetType, Viewport, AsImage, SvgContent,
        },
        NodeBundle, calc::EntityKey,
    },
    resource::animation_sheet::KeyFramesSheet,
};

use super::{
    animation_sheet::ObjKey,
    fragment::{FragmentMap, Fragments},
    ClassSheet, ShareFontSheet,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStyleCmd(pub VecDeque<Attribute>);

impl Command for DefaultStyleCmd {
    fn apply(self, world: &mut World) {
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
    fn apply(self, world: &mut World) {
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
    fn apply(self, world: &mut World) {
        let mut fragment_map = world.get_resource_mut::<FragmentMap>().unwrap();
        fragment_map.extend(self.0);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCmd<T>(pub T, pub Entity);
impl<T: Bundle> Command for NodeCmd<T> {
    fn apply(self, world: &mut World) {
        if let Some(mut r) = world.get_entity_mut(self.1) {
            out_any!(log::debug, "NodeCmd====================node：{:?}, anchor： {:?}", self.1, &self.0);
            r.insert(self.0);
        } else {
            out_any!(log::debug, "node_cmd fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostProcessCmd(pub EntityKey, pub Entity);
impl Command for PostProcessCmd {
    fn apply(self, world: &mut World) {
        if let Some(mut r) = world.get_entity_mut(self.1) {
			if let Some(mut r) = r.get_mut::<AsImage>() { 
				out_any!(log::debug, "PostProcessCmd====================node：{:?}, post {:?}", self.1, &self.0);
            	r.post_process = self.0;
			} else {
				r.insert(AsImage {
					level: pi_style::style::AsImage::None,
					post_process: self.0,
				});
			}
				
            
        } else {
            out_any!(log::debug, "PostProcess fail======================={:?}, {:?}", &self.1, &self.0);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentCmd<T>(pub T, pub Entity);
impl<T: Component> Command for ComponentCmd<T> {
    fn apply(self, world: &mut World) {
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
    fn apply(mut self, world: &mut World) {
        if world.get_entity(self.2).is_some() {
            let mut sheet = world.get_resource_mut::<KeyFramesSheet>().unwrap();
            self.1.name.scope_hash = self.2.index() as usize; // 因为每个运行时动画是节点独有的，以节点的index作为scope_hash(不能同时有两个index相等的实体)
            let _ = sheet.add_runtime_keyframes(ObjKey(self.2), &self.1, self.0);
            world.entity_mut(self.2).insert(self.1);
        }
    }
}

// 运行时动画绑定指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontCfgCmd(pub FontCfg);
impl Command for FontCfgCmd {
    fn apply(self, world: &mut World) {
        let sheet = &***world.get_resource_mut::<ShareFontSheet>().unwrap();
		let mut sheet = (*sheet).borrow_mut();
		sheet.font_mgr_mut().add_sdf_cfg(self.0);
    }
}

// 添加sdf2字体指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontSdf2Cmd(pub Atom, pub Vec<u8>);
impl Command for FontSdf2Cmd {
    fn apply(self, world: &mut World) {
        let sheet = &***world.get_resource_mut::<ShareFontSheet>().unwrap();
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
        let sheet = &***world.get_resource_mut::<ShareFontSheet>().unwrap();
		let mut sheet = (*sheet).borrow_mut();
		sheet.borrow_mut().font_mgr_mut().add_sdf_default_char(self.font_face, self.char);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClearColorCmd(pub CgColor);

impl Command for ClearColorCmd {
    fn apply(self, world: &mut World) {
		let color = wgpu::Color { r: self.0.x as f64, g: self.0.y as f64, b: self.0.z as f64, a: self.0.w as f64 };
		match world.get_resource_mut::<PiClearOptions>() {
			Some(mut r) => r.color = color ,
			None => {
				let mut option = PiClearOptions::default();
				option.color = color;
				world.insert_resource(option);
			},
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
    SvgStrokeCmd(SvgStrokeCmd),
    StrokeDasharrayCmd(StrokeDasharrayCmd),
    SvgShapeCmd(SvgShapeCmd),
}

// #[derive(Clone)]
// pub struct AnimationListenCmd(pub Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>) + Send + Sync + 'static>);
// impl Command for AnimationListenCmd {
//     fn write(self, world: &mut World) {
// 		world.get_resource_mut::<KeyFramesSheet>().unwrap().set_event_listener(self.0);
//     }
// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvgColorCmd(pub Entity, pub CgColor);

impl Command for SvgColorCmd {
    fn apply(self, world: &mut World) {
        let component_id = world.init_component::<SvgContent>();
        println!("component_id3: {:?}", component_id);
        if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
            component.set_changed();
            let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
            v.style.fill_color = pi_style::style::Color::RGBA(self.1);
        } else {
            let mut svg = SvgContent::default();
            svg.style.fill_color = pi_style::style::Color::RGBA(self.1);
            world.entity_mut(self.0).insert(svg);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvgStrokeCmd(pub Entity, pub Stroke);

impl Command for SvgStrokeCmd {
    fn apply(self, world: &mut World) {
        let component_id = world.init_component::<SvgContent>();
        if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
            component.set_changed();
            let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
            v.style.stroke = self.1;
        } else {
            let mut svg = SvgContent::default();
            svg.style.stroke = self.1;
            world.entity_mut(self.0).insert(svg);
        }

        // event_writer.send(ComponentEvent::<Changed<SvgContent>>::new(*entity));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrokeDasharrayCmd(pub Entity, pub StrokeDasharray);

impl Command for StrokeDasharrayCmd {
    fn apply(self, world: &mut World) {
        let component_id = world.init_component::<SvgContent>();
        if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
            component.set_changed();
            let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
            v.style.stroke_dasharray = self.1;
        } else {
            let mut svg = SvgContent::default();
            svg.style.stroke_dasharray = self.1;
            world.entity_mut(self.0).insert(svg);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rect { x: f32, y: f32, width: f32, height: f32 },
    Circle { cx: f32, cy: f32, radius: f32 },
    Ellipse { cx: f32, cy: f32, rx: f32, ry: f32 },
    Segment { ax: f32, ay: f32, bx: f32, by: f32},
    Polygon { points: Vec<[f32; 2]> },
    Polyline { points: Vec<[f32; 2]> },
    Path { points: Vec<[f32; 2]>, verb: Vec<pi_hal::pi_sdf::shape::PathVerb>}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgShapeCmd(pub Entity, pub Shape);

impl Command for SvgShapeCmd {
    fn apply(self, world: &mut World) {
        let component_id = world.init_component::<SvgContent>();
        if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
            component.set_changed();
            let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
            v.shape = Some(self.1);
        } else {
            let mut svg = SvgContent::default();
            svg.shape = Some(self.1);
            world.entity_mut(self.0).insert(svg);
        }
    }
}
