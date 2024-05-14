//! 定义与DrawObject相关的组件

use std::{hash::{Hash, Hasher}, ops::Range};


use crate::resource::{
    draw_obj::{PipelineStateWithHash, ProgramMetaInner, VertexBufferLayoutWithHash},
    RenderObjType,
};
use pi_assets::asset::Handle;
use pi_atom::Atom;
use pi_hash::XHashSet;
use pi_postprocess::image_effect::PostProcessDraw;
use pi_render::{renderer::draw_obj::DrawObj as DrawState1, rhi::asset::{TextureRes, AssetWithId}, components::view::target_alloc::ShareTargetView};
use pi_share::Share;
use pi_world::insert::Component;

use super::{calc::{BorderImageTexture, BackgroundImageTexture}, user::{Canvas, BackgroundColor, BorderColor, BoxShadow, TextContent, SvgContent, SvgInnerContent}};
pub use super::root::{CopyFboToScreen, DynTargetType};

pub struct DrawObject;

#[derive(Debug, Default, Deref, Component)]
pub struct DrawState(DrawState1);

/// 是否使用单位四边形渲染
#[derive(EnumDefault, PartialEq, Eq, Debug, Component)]
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
// #[derive(Deref, Default, Debug, Clone)]
// pub struct VSDefines(pub XHashSet<Atom>);

// impl Hash for VSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }

// /// fs shader的宏开关
// #[derive(Deref, Default, Debug, Clone)]
// pub struct FSDefines(pub XHashSet<Atom>);

// impl Hash for FSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }


// #[derive(Clone, Debug, PartialEq, Eq, Default)]
// pub struct StaticIndex {
// 	pub shader: usize,
// 	pub pipeline_state: DefaultKey,
// 	pub vertex_buffer_index: DefaultKey,
// 	pub name: &'static str,
// }

// #[derive(Clone, Deref, Hash)]
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
#[derive(Debug, Default, Component)]
pub struct BackgroundColorMark;

// 标记BorderShadow 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct BoxShadowMark;

// 标记背景图片 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct BackgroundImageMark;

// 标记文字 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct TextMark;

// 标记文字 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct SvgMark;

// 标记文字阴影 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
// TextShadowMark.0表示是第几个Shadow创建的DrawObj
#[derive(Debug, Default, Deref, Component)]
pub struct TextShadowMark(pub usize);

// 标记BorderColor 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct BorderColorMark;

// 标记BorderImage 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct BorderImageMark;

// 标记Canvas 放在DrawObject原型中，可以区分不同类型的DarwObject， 使得系统能够更好的并行
#[derive(Debug, Default, Component)]
pub struct CanvasMark;

// 实例索引(当只有一个实例时， InstanceIndex.0.start为该实例的属性， 当存在多个时，在从InstanceIndex.0范围内， 从InstanceIndex.0.start起， 每间隔一个实例长度，就是一个实例数据)
#[derive(Debug, Clone, Deref, Component)]
pub struct InstanceIndex(pub Range<usize>);

impl Default for InstanceIndex {
	fn default() -> Self {
		Self(pi_null::Null::null())
	}
}

// 渲染属性（像文字这类特殊的渲染， 每个字符都是一个实例渲染， 因此一个span可能存在多个渲染实例， 如果不存在该组件， 表示一个渲染实例， 否则用该组件描述渲染实例的数量）
#[derive(Debug, Clone, Component)]
pub struct RenderCount(pub u32);

impl Default for RenderCount {
	fn default() -> Self {
		Self(1)
	}
}

#[derive(Default, Component)]
pub struct FboInfo {
	pub fbo: Option<ShareTargetView>, // canvas,后处理杰斯安都会分配fbo， 该fbo在渲染图build阶段产生
	pub out: Option<ShareTargetView>, // canvas,后处理杰斯安都会分配fbo， 该fbo在渲染图build阶段产生
	pub in_batch: usize, // 当为null时， 表示还没有分批， 否则表示所在批的索引
	pub post_draw: Option<(Vec<PostProcessDraw>, pi_render::renderer::draw_obj::DrawObj)>,
}

// // 渲染标记(是什么类型的渲染， 如文字类型， 图片类型， 是否存在裁剪等等)
// #[derive(Debug, Default, Clone)]
// pub struct RenderFlag(pub u32);

/// 实例劈分方式
/// DrawObj的组件：
/// 1. 可能是因为存在纹理而劈分
/// 2. 可能是因为pipeline不同而需要劈分
#[derive(Debug, Component)]
pub enum InstanceSplit {
	ByTexture(Handle<AssetWithId<TextureRes>>),
	ByCross(bool), // 交叉渲染， 表示该节点的渲染为一个外部系统的渲染， bool表示是否用运行图的方式渲染（如果是false，则为外部渲染为一个fbo，gui内部需要将其作为渲染对象渲染）
}

#[derive(Debug, Deref, Component)]
pub struct Pipeline(pub Share<wgpu::RenderPipeline>);

// DepthUniform

pub trait GetInstanceSplit {
	fn get_split(&self) -> Option<InstanceSplit>;
}

impl GetInstanceSplit for BorderImageTexture {
	fn get_split(&self) -> Option<InstanceSplit> {
		match &self.0 {
			Some(r) => Some(InstanceSplit::ByTexture(r.clone())),
			None => None,
		}
		
	}
}

impl GetInstanceSplit for BackgroundImageTexture {
	fn get_split(&self) -> Option<InstanceSplit> {
		match &self.0 {
			Some(r) => Some(InstanceSplit::ByTexture(r.clone())),
			None => None,
		}
	}
}

impl GetInstanceSplit for Canvas {
	fn get_split(&self) -> Option<InstanceSplit> {
		Some(InstanceSplit::ByCross(self.by_draw_list))
	}
}

impl GetInstanceSplit for BackgroundColor {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}

impl GetInstanceSplit for BorderColor {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}

impl GetInstanceSplit for BoxShadow {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}

impl GetInstanceSplit for TextContent {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}

impl GetInstanceSplit for SvgContent {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}

impl GetInstanceSplit for SvgInnerContent {
	fn get_split(&self) -> Option<InstanceSplit> {
		None
	}
}
