//! 定义根节点相关的组件

use pi_render::components::view::target_alloc::TargetType;
use pi_style::style::{Aabb2, CgColor, Point2};
use pi_world::prelude::Component;

use super::pass_2d::DirtyRect;


#[derive(Clone, Serialize, Deserialize, Deref, Debug, Component)]
pub struct Viewport(pub Aabb2);

impl Default for Viewport {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0))) }
}

// 清屏颜色(rgba, 是否清窗口)
#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct ClearColor(pub CgColor, pub bool);

impl Default for ClearColor {
    fn default() -> Self { Self(CgColor::new(0.0, 0.0, 0.0, 0.0), false) }
}

#[derive(Clone, Debug, Deref, Default, Serialize, Deserialize, Component)]
pub struct RenderDirty(pub bool);

/// 渲染目标类型（有深度缓冲区和无深度缓冲区两种，rgba格式）
/// 后处理通常使用无深度缓冲区的渲染目标
/// 普通节点渲染使用有深度缓冲器
#[derive(Debug, Clone, Component)]
pub struct DynTargetType {
    pub has_depth: TargetType,
    pub no_depth: TargetType,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize, PartialEq, Eq, Component)]
pub enum RenderTargetType {
    Screen,
    OffScreen,
}


#[derive(Default, Deref, Debug, Component)]
pub struct RootDirtyRect(pub DirtyRect);

#[derive(Default, Component)]
pub struct RootScale {
    pub x: f32,
    pub y: f32,
}