//! 定义所需资源

pub mod animation_sheet;
pub mod cmd;
pub mod draw_obj;
pub mod fragment;
pub use cmd::*;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::TextureKeyAlloter;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_hal::font::font::FontType;
use pi_map::Map;
use pi_null::Null;
use pi_render::font::FontSheet;
use pi_render::rhi::asset::{TextureRes, AssetWithId};
use pi_share::{Share, ShareCell};
use pi_style::style::{Aabb2, CgColor};
use pi_world::world::ComponentIndex;
use pi_key_alloter::Key;
use smallvec::SmallVec;

use std::marker::{ConstParamTy, PhantomData};
use std::mem::transmute;
use std::ops::{Index, IndexMut};

use pi_style::style_parse::{parse_animation, parse_class_map_from_string, parse_style_list_from_string};
use pi_style::style_type::{ AnimationDelayType, AnimationDirectionType, AnimationDurationType, AnimationFillModeType, AnimationIterationCountType, AnimationNameType, AnimationPlayStateType, AnimationTimingFunctionType, VNodeType, ZIndexType};
use pi_time::Instant;
use pi_hal::font::sdf_table::FontCfg;

// use pi_ecs::prelude::{FromWorld, Id, World};
use pi_world::prelude::{Entity, FromWorld, World, Command, CommandQueue};

use crate::components::calc::{EntityKey, Quad, StyleMarkType};
use crate::components::user::serialize::{AttrSet, StyleAttr};
use crate::components::user::{AsImage, ClipPath, MaskImage, Point2, RenderTargetType, Vector2, Viewport};
use crate::components::SettingComponentIds;
use crate::utils::tools::LayerDirty;
use pi_spatial::quad_helper::QuadTree as QuadTree1;
// use crate::utils::cmd::{CommandQueue, Command, DataQuery};
// use pi_world::prelude::{CommandQueue, Commands, World};
use crate::components::user::ClassName;

use self::draw_obj::{CommonBlendState, DrawObjDefault};
use self::fragment::NodeTag;


#[derive(Default, Deref, Serialize, Deserialize)]
pub struct ClassSheet(pi_style::style_type::ClassSheet);


#[derive(Serialize, Deserialize, Clone, Copy, Hash, ConstParamTy, PartialEq, Eq)]
pub enum OtherDirtyType {
    // NodeCreate = 127, // 添加到树上也算创建
    Canvas = 128, // 遮罩纹理
    NodeTreeAdd = 127, // 树结构改变
    NodeTreeDel = 126, // 树结构改变
    DrawObjCreate = 125, // drawObj创建
    DrawObjDelete = 124, // drawObj删除
    Rebatch = 123, // 需要重新批处理（比如纹理变化会设置此标记）
    WorldMatrix = 122, // 世界矩阵
    BackgroundImageTexture = 121, // 背景纹理
    BorderImageTexture = 120, // 背景纹理
    MaskImageTexture = 119, // 遮罩纹理
    // Canvas = 117, // canvas修改
    CanvasBylist = 117, // canvas bylist改变（可改变drawobject渲染排序）
    PassLife = 116, // Pass3D生命周期（添加或移除）
    InstanceCount = 115, // 实例数量修改
    NodeState = 114, // NodeState修改
    NodeTreeRemove = 113, // 树结构改变
    
}

// 系统运行标志， 为false， 所有guisystem暂停运行
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SystemRunFlag(pub bool);
impl Default for SystemRunFlag {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RenderDirty(pub bool/*外部需要渲染设脏， 设置此字段，每帧会被清理*/, pub bool/*当前帧是否脏*/, pub bool/*上一帧是否脏*/);

#[derive(Default, Deref, Serialize, Deserialize)]
pub struct GlobalDirtyMark {
    pub mark: StyleMarkType,
}


#[derive(Default, Deref)]
pub struct MatrixDirty(pub LayerDirty<Entity>);

/// 用户指令缓冲区
pub struct UserCommandsCache(pub UserCommands);

// pub struct ShareSetting{
//     pub compoents: Option<Share<SettingComponentIds>>,
//     pub default_style: Option<Share<DefaultStyle>>,
// }

// impl FromWorld for ShareSetting {
//     fn from_world(world: &mut World) -> Self {
//         Self {
//             compoents: Some(Share::new(SettingComponentIds::from_world(world))),
//             default_style: Some(Share::new(DefaultStyle::from_world(world))),
//         }
//     }
// }


#[derive(Debug, Default)]
pub struct IsRun(pub bool);
/// 用户指令
#[derive(Default)]
pub struct UserCommands {
    pub is_node_change: bool,
    pub node_create: bool,
    /// 节点指令
    pub node_commands: Vec<NodeCommand>,
	// /// 节点初始化
	// pub node_init_commands: Vec<(Entity, NodeTag)>,
    pub fragment_commands: Vec<FragmentCommand>,
    /// 样式指令
    pub style_commands: StyleCommands,
    // /// 文本指令
    // pub text_commands: Vec<(Entity, Option<TextContent>)>,
    /// class指令(class指令单独放，使得class设置可以在style设置之后执行，性能会更高，因为style设置了的属性，class不会再重复设置)
    pub class_commands: Vec<(Entity, ClassName)>,

    /// selector指令
    pub selector_commands: Vec<(Entity, ClassName)>,

    // css 内容增加指令
    // pub css_commands: Vec<ClassSheet>,
    /// single指令
    pub other_commands: CommandQueue,

    #[cfg(feature = "debug")]
    pub other_commands_list: Vec<CmdType>, // 是CommandQueue中元素的枚举形式，便于序列化
    pub is_record: bool,

	pub version: usize,
}

// /// 节点变动标记（不一定是节点变动，主要用于判断实例数据是否应该重新组织）
// #[derive(Default, Debug)]
// pub struct NodeChanged {
//     pub node_changed: bool,
//     pub rebatch: bool,
// }

impl UserCommands {
	// 初始化节点
	#[inline]
	pub fn init_node(&mut self, entity: Entity, tag: NodeTag) {
        self.node_create = true;
        let start = self.style_commands.style_buffer.len();
        if tag == NodeTag::VNode {
            self.style_commands.set_style(entity, ZIndexType(-1));
            self.style_commands.set_style(entity, VNodeType(true));
        }
        self.style_commands.commands.push((entity, start, start, Some(tag)));    
	}

    pub fn init_component_ids(tag: NodeTag, ids: &SettingComponentIds) -> Vec<(ComponentIndex, bool)> {
        let mut type_arr = Vec::with_capacity(16);
        type_arr.extend_from_slice(&[
            (ids.down, true),
            (ids.up, true),
            (ids.layer, true),
            (ids.class_name, true),
            (ids.node_state, true),
            (ids.size, true),

            (ids.style_mark, true),
            (ids.matrix, true),
            (ids.z_range, true),
            (ids.content_box, true),
            (ids.layout, true),
            (ids.quad, true),
            (ids.in_pass_id, true),
            (ids.render_context_mark, true),
            (ids.draw_list, true),
            (ids.is_show, true),
            (ids.has_animation, true),
            // (ids.is_display, true),
        ]);
        if tag == NodeTag::VNode {
            type_arr.push((ids.z_index, true));

            // type_arr.push((ids.down, true));
            // type_arr.push((ids.up, true));
            // type_arr.push((ids.layer, true));
            // type_arr.push((ids.node_state, true));
            // type_arr.push((ids.size, true));
            
            
            // type_arr.push((ids.matrix, true));
            // type_arr.push((ids.z_range, true));
            // type_arr.push((ids.content_box, true));
            // type_arr.push((ids.layout, true));
        } else {
            
        }
        type_arr
    }
    /// 将节点作为子节点挂在父上
    pub fn append(&mut self, entity: Entity, parent: Entity) -> &mut Self {
        // log::debug!("append====={:?}, {:?}", entity, parent);
        self.node_commands.push(NodeCommand::AppendNode(entity, parent));
		self
    }

    /// 将节点插入到某个节点之前
    pub fn insert_before(&mut self, entity: Entity, anchor: Entity) -> &mut Self {
        // log::debug!("insert_before====={:?}, {:?}", entity, anchor);
        self.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
		self
    }

    /// 从父节点上移除节点
    pub fn remove_node(&mut self, entity: Entity) -> &mut Self {
        // log::debug!("remove_node====={:?}", entity);
        self.node_commands.push(NodeCommand::RemoveNode(entity));
		self
    }

    /// 从父节点上移除节点，并销毁该节点及所有子节点
    pub fn destroy_node(&mut self, entity: Entity) -> &mut Self {
        // log::debug!("destroy_node===={:?}", &entity);
        self.node_commands.push(NodeCommand::DestroyNode(entity));
		self
    }

    /// 设置节点样式
    #[inline]
    pub fn set_style<T: AttrSet>(&mut self, entity: Entity, value: T) -> &mut Self {
        // out_any!(log::debug, "set_style, entity: {:?}, value: {:?}", entity, &value);
        self.style_commands.set_style(entity, value);
		self
    }

    /// 设置默认样式（字符串）TODO
    pub fn set_default_style_by_str(&mut self, class: &str, scope_hash: usize) -> &mut Self {
        match parse_style_list_from_string(class, scope_hash) {
            Ok(r) => {
                #[cfg(feature = "debug")]
                if self.is_record {
                    self.other_commands_list.push(CmdType::DefaultStyleCmd(DefaultStyleCmd(r.clone())));
                }
                self.other_commands.push(DefaultStyleCmd(r));
            }
            Err(e) => {
                log::error!("set_default_style_by_str fail, parse style err: {:?}", e);
                return self;
            }
        };
		self
    }

    pub fn add_css(&mut self, css: &str, scope_hash: usize) -> &mut Self {
        let r = match parse_class_map_from_string(css, scope_hash as usize) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return self;
            }
        };
        self.push_cmd(ExtendCssCmd(vec![r]));
		self
    }

    // 添加运行时动画
    pub fn add_runtime_animation(&mut self, node: Entity, animation: &str, css: &str, scope_hash: usize) -> &mut Self {
        let mut input = cssparser::ParserInput::new(animation);
        let mut parse = cssparser::Parser::new(&mut input);
        let mut animations = match parse_animation(&mut parse) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return self;
            }
        };
        animations.name.scope_hash = node.index() as usize; // 因为每个ji运行时动画是节点独有的，以节点的index作为scope_hash(不能同时有两个index相等的实体)

        let css = match parse_class_map_from_string(css, scope_hash as usize) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("set_default_style_by_str fail, parse style err: {:?}", e);
                return self;
            }
        };

        // 避免被其他通过style或class设置的静态动画覆盖， 这里需要将动画设置添加到style指令中
        self.set_style(node, AnimationNameType(animations.name.clone()));
        self.set_style(node, AnimationDurationType(animations.duration.clone()));
        self.set_style(node, AnimationTimingFunctionType(animations.timing_function.clone()));
        self.set_style(node, AnimationIterationCountType(animations.iteration_count.clone()));
        self.set_style(node, AnimationDelayType(animations.delay.clone()));
        self.set_style(node, AnimationDirectionType(animations.direction.clone()));
        self.set_style(node, AnimationFillModeType(animations.fill_mode.clone()));
        self.set_style(node, AnimationPlayStateType(animations.play_state.clone()));

        let r = RuntimeAnimationBindCmd(css.key_frames.frames, unsafe { transmute(animations) }, node);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::RuntimeAnimationBindCmd(r.clone()));
        }

        self.push_cmd(r);
		self
    }

    /// 设置节点的class
    pub fn set_class(&mut self, entity: Entity, value: ClassName) -> &mut Self {
        // println_any!("set_class===={:?}", &value);
		// out_any!(log::warn, "set_class, entity: {:?}, {:?}, value: {:?}", entity, unsafe {transmute::<_, f64>(entity.to_bits())}, &value);
        self.class_commands.push((entity, value));
		self
    }

    pub fn set_selector(&mut self, entity: Entity, value: ClassName) -> &mut Self {
        self.selector_commands.push((entity, value));
		self
    }

    /// 添加指令
    pub fn push_cmd<T: Command>(&mut self, cmd: T) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        self.other_commands.push(cmd);
		self
    }

	/// 添加sdf字体配置
    pub fn add_sdf_font(&mut self, cfg: FontCfg) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        let r = FontCfgCmd(cfg);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::SdfCfgCmd(r.clone()));
        }

        self.other_commands.push(r);
		self
    }

	/// 添加sdf2字体
    pub fn add_sdf2_font(&mut self, name: Atom, buffer: Share<Vec<u8>>) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        let r = FontSdf2Cmd(name, buffer);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::Sdf2CfgCmd(r.clone()));
        }

        self.other_commands.push(r);
		self
    }

	/// 添加默认文字
    pub fn add_sdf_default_char(&mut self, font_face: Atom, char: char) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        let r = SdfDefaultCharCmd{font_face, char};

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::SdfDefaultCharCmd(r.clone()));
        }

        self.other_commands.push(r);
		self
    }

    /// 设置视口
    pub fn set_view_port(&mut self, node: Entity, cmd: Viewport) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        let r = NodeCmd(cmd, node);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::NodeCmdViewport(r.clone()));
        }

        self.other_commands.push(r);
		self
    }

    /// 设置目标类型
    pub fn set_target_type(&mut self, node: Entity, cmd: RenderTargetType) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        let r = NodeCmd(cmd, node);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::NodeCmdRenderTargetType(r.clone()));
        }

        self.other_commands.push(r);
		self
    }

    /// 设置清屏颜色
    pub fn set_clear_color(&mut self, color: CgColor) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
		let cmd = ClearColorCmd(color);
        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::NodeCmdRenderClearColor(cmd.clone()));
        }

        self.other_commands.push(cmd);
		self
    }

    /// 设置渲染脏
    pub fn set_render_dirty(&mut self, _node: Entity, cmd: RenderDirty) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);
        // let r = NodeCmd(cmd, node);

        #[cfg(feature = "debug")]
        self.other_commands_list.push(CmdType::NodeCmdRenderRenderDirty(RenderDirtyCmd(cmd.clone())));

        self.other_commands.push(RenderDirtyCmd(cmd));
		self
    }

    /// 添加片段
    pub fn extend_fragment_bin(&mut self, cmd: ExtendFragmentCmd) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::ExtendFragmentCmd(cmd.clone()));
        }

        self.other_commands.push(cmd);
		self
    }

    /// 添加片段
    pub fn add_css_bin(&mut self, cmd: ExtendCssCmd) -> &mut Self {
        // println_any!("push_cmd===={:?}", 1);

        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::ExtendCssCmd(cmd.clone()));
        }

        self.other_commands.push(cmd);
		self
    }

    /// 添加默认样式
    pub fn add_default_css(&mut self, cmd: DefaultStyleCmd) -> &mut Self {
        #[cfg(feature = "debug")]
        if self.is_record {
            self.other_commands_list.push(CmdType::DefaultStyleCmd(cmd.clone()));
        }

        self.other_commands.push(cmd);
		self
    }
}

/// style设置指令
#[derive(Clone, Default)]
pub struct StyleCommands {
    /// 样式列表
    // pub style_list: Vec<Attribute>,
    pub style_buffer: Vec<u8>,
    /// 设置样式（节点，需要操作的组件Id列表，开始索引，结束索引），其中开始索引和结束索引是指在style_list中的索引， 组件Id列表是需要对该实体的哪些组件进行操作
    pub commands: Vec<(Entity, usize, usize, Option<NodeTag>)>,
}

impl StyleCommands {
    /// 设置节点样式
    pub fn set_style<T: AttrSet>(&mut self, entity: Entity, value: T) {
        // out_any!(log::debug, "set_style, entity: {:?}, value: {:?}", entity, &value);
        pi_print_any::out_any!(log::trace, "set_style, entity: {:?}, {:?}, value: {:?}", entity, unsafe {transmute::<_, f64>(entity)}, &value);
        
        let start = self.style_buffer.len();
        unsafe { StyleAttr::write(value, &mut self.style_buffer) };
        if let Some(r) = self.commands.last_mut() {
            if r.0 == entity {
                r.2 = self.style_buffer.len(); 
                return;
            }
        } 

        self.commands.push((entity, start, self.style_buffer.len(), None));
    }
}

#[derive(Default)]
pub struct DefaultStyle;

// 需要进行后处理的上下文标记
#[derive(Clone, Debug, Default, Deref, Serialize, Deserialize)]
pub struct EffectRenderContextMark(bitvec::prelude::BitArray<[u32; 1]>);

pub trait Effect {}
impl Effect for MaskImage {}
impl Effect for ClipPath {}
impl Effect for AsImage {}

/// 渲染上下文标记分配器，每一种可以使节点成为渲染上下文的属性，都可以让全局单例RenderContextMarkAlloc分配一个id
#[derive(Debug, Default, Deref)]
pub struct RenderContextMarkAlloc(usize);

/// 渲染上下文类型，每一种可以使节点成为渲染上下文的属性，都对应一个RenderContextMarkType，类型值是在初始化时，找RenderContextMarkAlloc分配的。
#[derive(Debug, Deref, Clone)]
pub struct RenderContextMarkType<T> {
    #[deref]
    value: usize,
    mark: PhantomData<T>,
}

impl<T> FromWorld for RenderContextMarkType<T> {
    default fn from_world(world: &mut World) -> Self {
        let cur_mark_index = match world.get_single_res_mut::<RenderContextMarkAlloc>() {
            Some(r) => r,
            None => {
                world.insert_single_res(RenderContextMarkAlloc::default());
                world.get_single_res_mut::<RenderContextMarkAlloc>().unwrap()
            }
        };
        ***cur_mark_index += 1;
        Self {
            value: ***cur_mark_index,
            mark: PhantomData,
        }
    }
}

impl<T: Effect> FromWorld for RenderContextMarkType<T> {
    fn from_world(world: &mut World) -> Self {
        world.init_single_res::<EffectRenderContextMark>();
        let cur_mark_index = match world.get_single_res_mut::<RenderContextMarkAlloc>() {
            Some(r) => r,
            None => {
                world.insert_single_res(RenderContextMarkAlloc::default());
                world.get_single_res_mut::<RenderContextMarkAlloc>().unwrap()
            }
        };

        ***cur_mark_index += 1;
        let index = ***cur_mark_index;
        // 标记效果类型
        let effect_mark = world.get_single_res_mut::<EffectRenderContextMark>().unwrap();
        effect_mark.set(index, true);

        Self {
            value: index,
            mark: PhantomData,
        }
    }
}

#[derive(Debug, Default, Deref)]
pub struct RenderObjTypeAlloc(usize);

/// 渲染类型分配器
#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderObjType(usize);

impl FromWorld for RenderObjType {
    fn from_world(world: &mut World) -> Self {
        let cur_mark_index = match world.get_single_res_mut::<RenderObjTypeAlloc>() {
            Some(r) => r,
            None => {
                world.insert_single_res(RenderObjTypeAlloc::default());
                world.get_single_res_mut::<RenderObjTypeAlloc>().unwrap()
            }
        };
        ***cur_mark_index += 1;
        Self(***cur_mark_index)
    }
}


// /// 是否不向下收集DrawList(一些PassBundle是由于DrawObj的需要，比如TextShadow)
// #[derive(Clone, Debug, Default, Deref, Serialize, Deserialize)]
// pub struct NotDrawListMark(bitvec::prelude::BitArray);

// 文字渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct TextRenderObjType(RenderObjType);
impl FromWorld for TextRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 文字渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct SvgRenderObjType(RenderObjType);
impl FromWorld for SvgRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 文字渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct SvgOuterGlowRenderObjType(RenderObjType);
impl FromWorld for SvgOuterGlowRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 文字渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct SvgShadowRenderObjType(RenderObjType);
impl FromWorld for SvgShadowRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 文字阴影渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct TextShadowRenderObjType(RenderObjType);
impl FromWorld for TextShadowRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 文字阴影渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct TextOuterGlowRenderObjType(RenderObjType);
impl FromWorld for TextOuterGlowRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}


// 背景颜色渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct BackgroundColorRenderObjType(RenderObjType);
impl FromWorld for BackgroundColorRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 边框颜色渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct BorderColorRenderObjType(RenderObjType);
impl FromWorld for BorderColorRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}


// 背景图片渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct BackgroundImageRenderObjType(RenderObjType);
impl FromWorld for BackgroundImageRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 边框图片渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct BorderImageRenderObjType(RenderObjType);
impl FromWorld for BorderImageRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// 阴影渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct BoxShadowRenderObjType(RenderObjType);
impl FromWorld for BoxShadowRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::NORMAL,
            },
        );
        Self(ty)
    }
}

// canvas渲染类型（在DrawList中分配槽位）
#[derive(Debug, Deref, Clone, Copy)]
pub struct CanvasRenderObjType(RenderObjType);
impl FromWorld for CanvasRenderObjType {
    fn from_world(world: &mut World) -> Self {
        let ty = RenderObjType::from_world(world);
        DrawObjDefault::add(
            world,
            ty,
            DrawObjDefault {
                blend_state: CommonBlendState::PREMULTIPLY,
            },
        );
        Self(ty)
    }
}

// 当前时间
#[derive(Clone, Debug)]
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


#[derive(Deref)]
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


// 用于debug的节点， 如果不为空， 则运行过程中会打印该节点的各种信息
#[cfg(debug_assertions)]
#[derive(Deref, Clone, Debug)]
pub struct DebugEntity(pub EntityKey);

#[cfg(debug_assertions)]
impl Default for DebugEntity {
    fn default() -> Self {
        DebugEntity(EntityKey::null())
    }
}

impl Index<EntityKey> for QuadTree {
    type Output = Quad;

    fn index(&self, index: EntityKey) -> &Self::Output { unsafe { self.get_unchecked(&index) } }
}

impl IndexMut<EntityKey> for QuadTree {
    fn index_mut(&mut self, index: EntityKey) -> &mut Self::Output { unsafe { self.get_unchecked_mut(&index) } }
}

#[derive(Deref)]
pub struct ShareFontSheet(pub Share<ShareCell<FontSheet>>);

#[cfg(target_arch = "wasm32")]
unsafe impl Send for ShareFontSheet {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for ShareFontSheet {}

// impl FromWorld for ShareFontSheet {
//     fn from_world(world: &mut World) -> Self {
//         let texture_res_mgr = world.get_single_res::<ShareAssetMgr<TextureRes>>().unwrap();
//         let device = world.get_single_res::<PiRenderDevice>().unwrap();
// 		let queue = world.get_single_res::<PiRenderQueue>().unwrap();
// 		let limits = device.limits();
//         ShareFontSheet(Share::new(ShareCell::new(FontSheet::new(&device.0, &texture_res_mgr.0, &queue.0, limits.max_texture_dimension_2d, false))))
//     }
// }

impl ShareFontSheet {
    pub fn new(world: &mut World, font_type: FontType) -> Self {
		world.init_single_res::<TextureKeyAlloter>();
        let texture_res_mgr = world.get_single_res::<ShareAssetMgr<AssetWithId<TextureRes>>>().unwrap();
		let alloter = world.get_single_res::<TextureKeyAlloter>().unwrap();
		
        let device = world.get_single_res::<PiRenderDevice>().unwrap();
		let queue = world.get_single_res::<PiRenderQueue>().unwrap();
		let limits = device.limits();
        ShareFontSheet(Share::new(ShareCell::new(FontSheet::new(&device.0, &texture_res_mgr.0, alloter.0.clone(),&queue.0, limits.max_texture_dimension_2d, font_type))))
    }
}

