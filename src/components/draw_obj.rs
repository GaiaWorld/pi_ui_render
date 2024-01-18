//! 定义与DrawObject相关的组件

use std::hash::{Hash, Hasher};


use crate::resource::{
    draw_obj::{PipelineStateWithHash, ProgramMetaInner, VertexBufferLayoutWithHash},
    RenderObjType,
};
use bevy_ecs::prelude::Component;
use pi_atom::Atom;
use pi_bevy_render_plugin::NodeId;
use pi_hash::XHashSet;
use pi_render::renderer::draw_obj::DrawObj as DrawState1;
use pi_share::Share;

pub use super::root::{ClearColorBindGroup, CopyFboToScreen, DynTargetType};

pub struct DrawObject;

#[derive(Debug, Default, Component, Deref)]
pub struct DrawState(DrawState1);

/// 是否使用单位四边形渲染
#[derive(EnumDefault, PartialEq, Eq, Component, Debug)]
pub enum BoxType {
    /// 渲染为content区，顶点不是单位四边形，世界矩阵需要变换到原点为内容区左上角
    ContentRect,
    /// 渲染为padding区，世界矩阵不变换
    PaddingNone,
    /// 渲染为content区，世界矩阵不变换
    ContentNone,
    /// 渲染为border区，世界矩阵不变换
    BorderNone,
    /// 渲染为content区，世界矩阵需要变换(此时顶点流是单位四边形)
    ContentUnitRect,
    /// 渲染为padding区，世界矩阵需要变换(此时顶点流是单位四边形)
    PaddingUnitRect,
    /// 渲染为border区，世界矩阵需要变换(此时顶点流是单位四边形)
    BorderUnitRect,
    /// 渲染为边框部分
    Border,
    /// 不变（由于像背景图这一类的渲染， 需要异步加载资源， 在资源未成功加载之前， 所有的渲染属性都不应该改变， 否则可能出现混乱）
    /// 例如， 一个动画会修改图片路径和位置两种属性， 但如果某一帧图片未加载成功， 那么渲染应该保持不变， 而不应该在此时修改位置
    NotChange,
}

// /// vs shader的宏开关
// #[derive(Deref, Default, Debug, Clone, Component)]
// pub struct VSDefines(pub XHashSet<Atom>);

// impl Hash for VSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }

// /// fs shader的宏开关
// #[derive(Deref, Default, Debug, Clone, Component)]
// pub struct FSDefines(pub XHashSet<Atom>);

// impl Hash for FSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }


// #[derive(Clone, Debug, PartialEq, Eq, Component, Default)]
// pub struct StaticIndex {
// 	pub shader: usize,
// 	pub pipeline_state: DefaultKey,
// 	pub vertex_buffer_index: DefaultKey,
// 	pub name: &'static str,
// }

// #[derive(Clone, Component, Deref, Hash)]
// pub struct PipelineMeta(pub Share<ProgramMetaInner>);

#[derive(Clone, Component)]
pub struct PipelineMeta {
    // 类型标记（如文字、图片、颜色等，它们属于不同的类型，用一个数字代表每个不同的类型）
    // 可以通过该类型标记动态地映射到该类型特有的属性值
    // 比如，可以映射到canvas的默认混合模式
    pub type_mark: RenderObjType,
    pub program: Share<ProgramMetaInner>,
    pub state: Share<PipelineStateWithHash>,
    pub vert_layout: Share<VertexBufferLayoutWithHash>,
    pub defines: XHashSet<Atom>,
}

impl Hash for PipelineMeta {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.program.hash(state);
        self.state.hash(state);
        self.vert_layout.hash(state);
        for i in self.defines.iter() {
            i.hash(state);
        }
    }
}

// 标记背景颜色 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct BackgroundColorMark;

// 标记BorderShadow 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct BoxShadowMark;

// 标记背景图片 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct BackgroundImageMark;

// 标记文字 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct TextMark;

// 标记文字阴影 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
// TextShadowMark.0表示是第几个Shadow创建的DrawObj
#[derive(Debug, Component, Default, Deref)]
pub struct TextShadowMark(pub usize);

// 标记BorderColor 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct BorderColorMark;

// 标记BorderImage 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct BorderImageMark;

// 标记Canvas 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Component, Default)]
pub struct CanvasMark;

// 实例索引
#[derive(Debug, Component)]
pub struct InstanceIndex(pub usize);

impl Default for InstanceIndex {
	fn default() -> Self {
		Self(pi_null::Null::null())
	}
}
