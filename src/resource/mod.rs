//! 定义所需资源

pub mod animation_sheet;
pub mod cmd;
pub mod draw_obj;
pub use cmd::*;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_map::Map;
use pi_render::font::FontSheet;
use pi_render::rhi::asset::TextureRes;
use pi_share::{Share, ShareCell};
use pi_style::style::Aabb2;

use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::{Index, IndexMut};

use pi_print_any::out_any;
use pi_style::style_parse::{parse_class_map_from_string, parse_style_list_from_string, parse_animation};
use pi_style::style_type::Attr;
use pi_time::Instant;

// use pi_ecs::prelude::{FromWorld, Id, World};
use bevy::ecs::prelude::{Entity, FromWorld, Resource, World};
use bevy::ecs::system::{Command, CommandQueue};

use crate::components::calc::{EntityKey, Quad};
use crate::components::user::serialize::StyleAttr;
use crate::components::user::{Point2, Vector2, MaskImage, ClipPath};
use pi_sparialtree::QuadTree as QuadTree1;
// use crate::utils::cmd::{CommandQueue, Command, DataQuery};
// use bevy::prelude::{CommandQueue, Commands, World};
use crate::components::user::ClassName;

#[derive(Default, Deref, Resource, Serialize, Deserialize)]
pub struct ClassSheet(pi_style::style_type::ClassSheet);

/// 用户指令缓冲区
#[derive(Default, Resource)]
pub struct UserCommandsCache(pub UserCommands);

/// 用户指令

#[derive(Default, Resource)]
pub struct UserCommands {
    /// 节点指令
    pub node_commands: Vec<NodeCommand>,
    /// 样式指令
    pub style_commands: StyleCommands,
    // /// 文本指令
    // pub text_commands: Vec<(Entity, Option<TextContent>)>,
    /// class指令(class指令单独放，使得class设置可以在style设置之后执行，性能会更高，因为style设置了的属性，class不会再重复设置)
    pub class_commands: Vec<(Entity, ClassName)>,

    // css 内容增加指令
    // pub css_commands: Vec<ClassSheet>,
    /// single指令
    pub other_commands: CommandQueue,
}

impl UserCommands {
    /// 将节点作为子节点挂在父上
    pub fn append(&mut self, entity: Entity, parent: Entity) {
        log::debug!("append====={:?}, {:?}", entity, parent);
        self.node_commands.push(NodeCommand::AppendNode(entity, parent));
    }

    /// 将节点插入到某个节点之前
    pub fn insert_before(&mut self, entity: Entity, anchor: Entity) {
        log::debug!("insert_before====={:?}, {:?}", entity, anchor);
        self.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
    }

    /// 从父节点上移除节点
    pub fn remove_node(&mut self, entity: Entity) {
        log::debug!("remove_node====={:?}", entity);
        self.node_commands.push(NodeCommand::RemoveNode(entity));
    }

    /// 从父节点上移除节点，并销毁该节点及所有子节点
    pub fn destroy_node(&mut self, entity: Entity) {
        log::debug!("destroy_node===={:?}", &entity);
        self.node_commands.push(NodeCommand::DestroyNode(entity));
    }

    /// 设置节点样式
    pub fn set_style<T: Attr>(&mut self, entity: Entity, value: T) {
        out_any!(log::debug, "set_style, entity: {:?}, value: {:?}", entity, &value);
        // out_any!(trace, "set_style, entity: {:?}, value: {:?}", entity, &value);
        let start = self.style_commands.style_buffer.len();
        unsafe { StyleAttr::write(value, &mut self.style_commands.style_buffer) };
        if let Some(r) = self.style_commands.commands.last_mut() {
            if r.0 == entity {
                r.2 = self.style_commands.style_buffer.len();
                return;
            }
        }
        self.style_commands.commands.push((entity, start, self.style_commands.style_buffer.len()));
    }

    /// 设置默认样式（字符串）TODO
    pub fn set_default_style_by_str(&mut self, class: &str, scope_hash: usize) {
        match parse_style_list_from_string(class, scope_hash) {
            Ok(r) => {
                self.other_commands.push(DefaultStyleCmd(r));
            }
            Err(e) => {
                log::error!("set_default_style_by_str fail, parse style err: {:?}", e);
                return;
            }
        };
    }

    pub fn add_css(&mut self, css: &str, scope_hash: usize) {
        let r = match parse_class_map_from_string(css, scope_hash as usize) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return;
            }
        };
        self.push_cmd(ExtendCssCmd(vec![r]));
    }

	// 添加运行时动画
	pub fn add_runtime_animation(&mut self, node: Entity, animation: &str, css: &str, scope_hash: usize) {
		let mut input = cssparser::ParserInput::new(animation);
        let mut parse = cssparser::Parser::new(&mut input);
		let mut animations = match parse_animation(&mut parse) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return;
            }
        };
		animations.name.scope_hash = scope_hash as usize;

        let css = match parse_class_map_from_string(css, scope_hash as usize) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return;
            }
        };
        self.push_cmd(RuntimeAnimationBindCmd(css.key_frames.frames, unsafe {transmute(animations)}, node));
    }

    /// 设置节点的class
    pub fn set_class(&mut self, entity: Entity, value: ClassName) {
        // println_any!("set_class===={:?}", &value);
        self.class_commands.push((entity, value));
    }

    /// 添加指令
    pub fn push_cmd<T: Command>(&mut self, cmd: T) {
        // println_any!("push_cmd===={:?}", 1);
        self.other_commands.push(cmd);
    }
}

/// style设置指令
#[derive(Default, Clone)]
pub struct StyleCommands {
    /// 样式列表
    // pub style_list: Vec<Attribute>,
    pub style_buffer: Vec<u8>,
    /// 设置样式（节点，开始索引，结束索引），其中开始索引和结束索引是指在style_list中的索引
    pub commands: Vec<(Entity, usize, usize)>,
}

#[derive(Default)]
pub struct DefaultStyle;

// 需要进行后处理的上下文标记
#[derive(Clone, Debug, Default, Deref, Serialize, Deserialize, Resource)]
pub struct EffectRenderContextMark(bitvec::prelude::BitArray);

pub trait Effect {}
impl Effect for MaskImage {}
impl Effect for ClipPath {}

/// 渲染上下文标记分配器，每一种可以使节点成为渲染上下文的属性，都可以让全局单例RenderContextMarkAlloc分配一个id
#[derive(Debug, Default, Deref, Resource)]
pub struct RenderContextMarkAlloc(usize);

/// 渲染上下文类型，每一种可以使节点成为渲染上下文的属性，都对应一个RenderContextMarkType，类型值是在初始化时，找RenderContextMarkAlloc分配的。
#[derive(Debug, Deref, Clone, Resource)]
pub struct RenderContextMarkType<T> {
	#[deref]
	value: usize,
	mark: PhantomData<T>,
}

impl<T> FromWorld for RenderContextMarkType<T> {
    default fn from_world(world: &mut World) -> Self {
        let mut cur_mark_index = match world.get_resource_mut::<RenderContextMarkAlloc>() {
            Some(r) => r,
            None => {
                world.insert_resource(RenderContextMarkAlloc::default());
                world.get_resource_mut::<RenderContextMarkAlloc>().unwrap()
            }
        };
        **cur_mark_index += 1;
        Self {
			value: **cur_mark_index,
			mark: PhantomData
		}
    }
}

impl<T: Effect> FromWorld for RenderContextMarkType<T> {
    fn from_world(world: &mut World) -> Self {
		world.init_resource::<EffectRenderContextMark>();
        let mut cur_mark_index = match world.get_resource_mut::<RenderContextMarkAlloc>() {
            Some(r) => r,
            None => {
                world.insert_resource(RenderContextMarkAlloc::default());
                world.get_resource_mut::<RenderContextMarkAlloc>().unwrap()
            }
        };

        **cur_mark_index += 1;
		let index = **cur_mark_index;
		// 标记效果类型
		let mut effect_mark = world.get_resource_mut::<EffectRenderContextMark>().unwrap();
		effect_mark.set(index, true);
		
        Self {
			value: index,
			mark: PhantomData,
		}
    }
}

#[derive(Debug, Default, Deref, Resource)]
pub struct RenderObjTypeAlloc(usize);

/// 渲染类型分配器
#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq)]
pub struct RenderObjType(usize);

impl FromWorld for RenderObjType {
    fn from_world(world: &mut World) -> Self {
        let mut cur_mark_index = match world.get_resource_mut::<RenderObjTypeAlloc>() {
            Some(r) => r,
            None => {
                world.insert_resource(RenderObjTypeAlloc::default());
                world.get_resource_mut::<RenderObjTypeAlloc>().unwrap()
            }
        };
        **cur_mark_index += 1;
        Self(**cur_mark_index)
    }
}

// /// 是否不向下收集DrawList(一些PassBundle是由于DrawObj的需要，比如TextShadow)
// #[derive(Clone, Debug, Default, Deref, Serialize, Deserialize, Resource)]
// pub struct NotDrawListMark(bitvec::prelude::BitArray);

// 文字渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct TextRenderObjType(RenderObjType);
impl FromWorld for TextRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}

// 文字阴影渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct TextShadowRenderObjType(RenderObjType);
impl FromWorld for TextShadowRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}


// 背景颜色渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct BackgroundColorRenderObjType(RenderObjType);
impl FromWorld for BackgroundColorRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}

// 边框颜色渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct BorderColorRenderObjType(RenderObjType);
impl FromWorld for BorderColorRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}


// 背景图片渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct BackgroundImageRenderObjType(RenderObjType);
impl FromWorld for BackgroundImageRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}

// 边框图片渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct BorderImageRenderObjType(RenderObjType);
impl FromWorld for BorderImageRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}

// 阴影渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct BoxShadowRenderObjType(RenderObjType);
impl FromWorld for BoxShadowRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
}

// canvas渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy, Resource)]
pub struct CanvasRenderObjType(RenderObjType);
impl FromWorld for CanvasRenderObjType {
    fn from_world(world: &mut World) -> Self { Self(RenderObjType::from_world(world)) }
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


#[derive(Deref, Resource)]
pub struct QuadTree(QuadTree1<EntityKey, ()>);

impl Default for QuadTree {
    fn default() -> Self { Self::with_capacity(0) }
}

impl Map for QuadTree {
    type Key = EntityKey;
    type Val = Quad;
    fn len(&self) -> usize { self.0.len() }
    fn with_capacity(_capacity: usize) -> Self {
        let max = Vector2::new(100f32, 100f32);
        let min = max / 100f32;
        Self(QuadTree1::new(
            Aabb2::new(Point2::new(-1024f32, -1024f32), Point2::new(4096f32, 4096f32)),
            max,
            min,
            0,
            0,
            16, //????
        ))
    }
    fn capacity(&self) -> usize { 0 }
    fn mem_size(&self) -> usize { 0 }
    fn contains(&self, key: &Self::Key) -> bool { self.0.contains_key(key.clone()) }
    fn get(&self, key: &Self::Key) -> Option<&Self::Val> { unsafe { transmute(self.0.get(key.clone())) } }
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val> { unsafe { transmute(self.0.get_mut(key.clone())) } }
    unsafe fn get_unchecked(&self, key: &Self::Key) -> &Self::Val { transmute(self.0.get_unchecked(key.clone())) }
    unsafe fn get_unchecked_mut(&mut self, key: &Self::Key) -> &mut Self::Val { transmute(self.0.get_unchecked_mut(key.clone())) }
    unsafe fn remove_unchecked(&mut self, key: &Self::Key) -> Self::Val { transmute(self.0.remove(key.clone()).unwrap()) }
    fn insert(&mut self, key: Self::Key, val: Self::Val) -> Option<Self::Val> {
        if self.0.contains_key(key) {
            self.0.update(key, val.0);
        } else {
            self.0.add(key, val.0, ());
        }
        return None;
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Val> { unsafe { transmute(self.0.remove(key.clone())) } }
}

impl Index<EntityKey> for QuadTree {
    type Output = Quad;

    fn index(&self, index: EntityKey) -> &Self::Output { unsafe { self.get_unchecked(&index) } }
}

impl IndexMut<EntityKey> for QuadTree {
    fn index_mut(&mut self, index: EntityKey) -> &mut Self::Output { unsafe { self.get_unchecked_mut(&index) } }
}

#[derive(Deref, Resource)]
pub struct ShareFontSheet(pub Share<ShareCell<FontSheet>>);
#[cfg(target_arch = "wasm32")]
unsafe impl Send for ShareFontSheet {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for ShareFontSheet {}

impl FromWorld for ShareFontSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_res_mgr = world.get_resource::<ShareAssetMgr<TextureRes>>().unwrap();
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let queue = world.get_resource::<PiRenderQueue>().unwrap();
        ShareFontSheet(Share::new(ShareCell::new(FontSheet::new(&device.0, &texture_res_mgr.0, &queue.0))))
    }
}
