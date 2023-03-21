use bevy::ecs::prelude::Component;
use pi_render::{
    components::view::target_alloc::{ShareTargetView, TargetType},
    renderer::draw_obj::DrawBindGroup,
};
use pi_style::style::{Aabb2, CgColor, Point2};

use super::{draw_obj::DrawState, pass_2d::DirtyRect};

/// 视口
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut, Debug, Component)]
pub struct Viewport(pub Aabb2);

impl Default for Viewport {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0))) }
}

// 清屏颜色(rgba, 是否清窗口)
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct ClearColor(pub CgColor, pub bool);

impl Default for ClearColor {
    fn default() -> Self {
        Self(CgColor::new(0.0, 0.0, 0.0, 0.0), false)
    }
}

#[derive(Clone, Debug, Deref, DerefMut, Default, Component)]
pub struct RenderDirty(pub bool);

/// 清屏颜色的bindgroup（用户设置）
#[derive(Component, Default)]
pub struct ClearColorBindGroup(pub Option<(DrawBindGroup, DrawBindGroup)>); // meterial, depth

/// 渲染目标类型（有深度缓冲区和无深度缓冲区两种，rgba格式）
/// 后处理通常使用无深度缓冲区的渲染目标
/// 普通节点渲染使用有深度缓冲器
#[derive(Debug, Clone, Component)]
pub struct DynTargetType {
    pub has_depth: TargetType,
    pub no_depth: TargetType,
}

///
#[derive(Component, Default)]
pub struct RenderTarget(pub Option<ShareTargetView>);

#[derive(Debug, Clone, Copy, EnumDefault, Component)]
pub enum RenderTargetType {
    Screen,
    OffScreen,
}

// 将需要渲染到屏幕的fbo渲染到屏幕
#[derive(Component, Default)]
pub struct CopyFboToScreen(pub Option<DrawState>);

#[derive(Component, Default, Deref, DerefMut, Debug)]
pub struct RootDirtyRect(pub DirtyRect);
