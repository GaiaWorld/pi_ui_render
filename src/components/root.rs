//! 定义根节点相关的组件

use bevy_ecs::prelude::Component;
use pi_render::components::view::target_alloc::TargetType;
use pi_style::style::{Aabb2, CgColor, Point2};

use super::{draw_obj::DrawState, pass_2d::DirtyRect};

/// 视口
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

#[derive(Clone, Debug, Deref, Default, Component, Serialize, Deserialize)]
pub struct RenderDirty(pub bool);

// /// 清屏颜色的bindgroup（用户设置）
// #[derive(Component, Default)]
// pub struct ClearColorBindGroup(pub Option<DrawBindGroup>); // meterial

/// 渲染目标类型（有深度缓冲区和无深度缓冲区两种，rgba格式）
/// 后处理通常使用无深度缓冲区的渲染目标
/// 普通节点渲染使用有深度缓冲器
#[derive(Debug, Clone, Component)]
pub struct DynTargetType {
    pub has_depth: TargetType,
    pub no_depth: TargetType,
}

#[derive(Debug, Clone, Copy, EnumDefault, Component, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderTargetType {
    Screen,
    OffScreen,
}

// 将需要渲染到屏幕的fbo渲染到屏幕
#[derive(Component, Default)]
pub struct CopyFboToScreen(pub Option<DrawState>);

#[derive(Component, Default, Deref, Debug)]
pub struct RootDirtyRect(pub DirtyRect);

// /// 该根节点下需要渲染的元素
// #[derive(Component, Default, Debug)]
// pub struct RootInstance {
//     // pub draw_list: Vec<DrawElement>, // 渲染元素
//     /// 批处理是否需要调整
//     /// 当draw_obj新增和删除、RenderCount发生改变、纹理发生改变（包含动态分配的fbo）时， 需要重新调整批处理
//     // pub rebatch: bool, 
//     // pub posts: Vec<Entity>, // 渲染元素
//     pub draw_range: Range<usize>, // 渲染范围（在全局draw_list上的范围）

//     // pub pass_toop_list: Vec<Entity>, //该根下 从叶子开始的广度遍历排序
// 	// pub next_node_with_depend: Vec<usize>, // 层分割（下一个依赖未就绪的节点，在list中的顺序）
//     // pub temp: (Vec<Entity>, Vec<Entity>), // 排序需要的临时数据
//     pub draw_index: usize, // 将当前结果绘制到屏幕上的实例化draw的索引， 为null时， 不需要draw
// }
