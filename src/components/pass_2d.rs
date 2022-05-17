use bitvec::array::BitArray;
use pi_ecs::entity::Id;
use pi_render::{graph::node::NodeId, rhi::bind_group::BindGroup};

use super::{draw_obj::DrawKey, user::{Aabb2, Point2, Matrix4}};

/// 一个渲染Pass
pub struct Pass2D;

pub type Pass2DKey = Id<Pass2D>;

/// 相机
#[derive(Debug)]
pub struct Camera {
	pub view: Matrix4,
    pub project: Matrix4,
	pub bind_group: Option<BindGroup>,
	pub view_port: Aabb2,
}

impl Default for Camera {
    fn default() -> Self {
        Self { view: Default::default(), project: Default::default(), bind_group: Default::default(), view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)) }
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct ParentPassId(pub Id<Pass2D>);

/// 渲染图节点
#[derive(Debug, Default, Deref, DerefMut)]
pub struct GraphId(pub NodeId);

// 渲染 物件 列表
pub struct Draw2DList {
	pub all_list: Vec<DrawKey>,
    /// 不透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub opaque: Vec<DrawKey>,

    /// 透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub transparent: Vec<DrawKey>,
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DirtyRectState {
	UnInit,
	Inited,
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self{
			value: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
			state: DirtyRectState::UnInit,
		}
    }
}
