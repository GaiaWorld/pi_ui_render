use bitvec::array::BitArray;
use pi_assets::asset::Handle;
use pi_ecs::entity::Id;
use pi_render::{
	graph::node::NodeId, 
	rhi::{bind_group::BindGroup, asset::RenderRes, buffer::Buffer}, components::view::target_alloc::ShareTargetView
};
use pi_slotmap::{DefaultKey, SecondaryMap};

use super::{
	draw_obj::DrawKey, 
	user::{Aabb2, Point2}
};

/// 一个渲染Pass
pub struct Pass2D;

pub type Pass2DKey = Id<Pass2D>;

/// 相机
#[derive(Debug)]
pub struct Camera {
	// pub view: Matrix4,
    // pub project: Matrix4,
	pub view_bind_group: Option<Handle<RenderRes<BindGroup>>>,
	pub project_bind_group: Option<Handle<RenderRes<BindGroup>>>,
	pub view_port: Aabb2,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
			// view: Default::default(), 
			// project: Default::default(), 
			view_bind_group: Default::default(), 
			project_bind_group: Default::default(), 
			view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
		}
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct ViewMatrix(pub Option<Handle<RenderRes<BindGroup>>>);

#[derive(Debug, Default, Deref, DerefMut)]
pub struct ParentPassId(pub Id<Pass2D>);

/// 渲染图节点
#[derive(Debug, Default, Deref, DerefMut)]
pub struct GraphId(pub NodeId);

// 渲染 物件 列表
pub struct Draw2DList {
	pub all_list: Vec<DrawIndex>,
    /// 不透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub opaque: Vec<DrawIndex>,

    /// 透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub transparent: Vec<DrawIndex>,
}

impl Default for Draw2DList {
    fn default() -> Self {
        Self {
			all_list: Vec::default(),
            opaque: Vec::default(),
            transparent: Vec::default(),
        }
    }
}

/// 渲染对象的索引
#[derive(Debug, Clone, Copy, Hash)]
pub enum DrawIndex {
	// 一个渲染对象
	DrawObj(DrawKey),
	// 一个Pass2D的内容
	Pass2D(Pass2DKey),
}

#[derive(Default, Deref, DerefMut)]
pub struct DirtyMark(BitArray);

pub enum DirtyType {
	List = 1, // 列表脏 （需要重新组织渲染列表）
	Depth = 2, // 深度脏 （需要重新排序）
	DirtyRect = 3, // 脏区域
}

/// 脏区域
#[derive(Clone, Debug)]
pub struct DirtyRect {
	pub value: Aabb2,
	pub state: DirtyRectState,
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self{
			value: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
			state: DirtyRectState::UnInit,
		}
    }
}

/// 上下文自身的脏区域(集考虑TransformWillchange)
#[derive(Clone, Debug)]
pub struct LastDirtyRect {
	pub last: Aabb2,
	pub no_will_change: Aabb2,
}

impl Default for LastDirtyRect {
    fn default() -> Self {
		LastDirtyRect{
			last: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
			no_will_change: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
		}
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DirtyRectState {
	UnInit,
	Inited,
	Active,
}

/// 后处理
#[derive(Clone)]
pub struct PostProcess {
	pub draw_obj_key: DrawKey, // 渲染对象的Key
	pub texture_bind_index: usize, // 纹理bing_group的索引
	pub uv_vb_index: usize, // uv buffer在vbs中的索引

	pub result: Option<PostTemp>, // 处理目标

	pub width: u32, // 后处理渲染目标宽度
	pub height: u32, // 后处理渲染目标高度
}

#[derive(Clone)]
pub struct PostTemp {
	pub target: ShareTargetView,
	pub texture_group: Handle<RenderRes<BindGroup>>,
	pub matrix: Handle<RenderRes<BindGroup>>,
	pub uv: Handle<RenderRes<Buffer>>,
}

impl PostProcess {
	pub fn new(
		draw_obj_key: DrawKey, 
		texture_bind_index: usize, 
		uv_vb_index: usize,
		width: u32,
		height: u32,
	) -> Self {
		Self {
			draw_obj_key,
			texture_bind_index,
			uv_vb_index,
			width,
			height,
			result: None,
		}
	}
}

/// 后处理列表
#[derive(Default, Clone)]
pub struct PostProcessList(pub SecondaryMap<DefaultKey, PostProcess>, pub DefaultKey/*最后一个后处理的key */);

/// 
pub enum RenderTarget {
	// 渲染到一个指定的离屏fbo
	OffScreen(ShareTargetView),
	// 渲染到屏幕
	Screen {
		aabb: Aabb2,
		depth: Option<Handle<RenderRes<wgpu::TextureView>>>,
	},
	// // 自动分配一个fbo来进行渲染
	// Auto(Option<ShareTargetView>),
}
