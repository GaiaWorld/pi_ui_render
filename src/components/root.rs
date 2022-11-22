use pi_render::components::view::target_alloc::{TargetType, ShareTargetView};
use pi_style::style::{Aabb2, Point2, CgColor};
use bevy::prelude::Component;

use super::draw_obj::{DrawState, DrawGroup};

/// 视口
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut, Debug, Component)]
pub struct Viewport(pub Aabb2);

impl Default for Viewport {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0))) }
}

// 清屏颜色(rgba, 是否清窗口)
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct ClearColor(pub CgColor, pub bool);

#[derive(Clone, Debug, Deref, DerefMut, Default, Component)]
pub struct RenderDirty(pub bool);

/// 清屏颜色的bindgroup（用户设置）
pub struct ClearColorBindGroup(pub Option<DrawGroup>);

impl Default for ClearColorBindGroup {
    fn default() -> Self {
        ClearColorBindGroup(None)
    }
}

/// 渲染目标类型（有深度缓冲区和无深度缓冲区两种，rgba格式）
/// 后处理通常使用无深度缓冲区的渲染目标
/// 普通节点渲染使用有深度缓冲器
#[derive(Debug, Clone)]
pub struct DynTargetType {
    pub has_depth: TargetType,
    pub no_depth: TargetType,
}

/// 
pub struct RenderTarget(pub ShareTargetView);

#[derive(Debug, Clone, Copy, EnumDefault, Component)]
pub enum RenderTargetType {
	Screen,
	OffScreen,
}

// 将需要渲染到屏幕的fbo渲染到屏幕
pub struct CopyFboToScreen(pub DrawState);