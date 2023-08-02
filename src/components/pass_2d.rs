//! 定义与Pass2D相关的组件

use bevy::ecs::{prelude::Component, system::Resource};
use bitvec::array::BitArray;
use pi_assets::asset::Handle;
pub use pi_bevy_render_plugin::component::GraphId;
use pi_postprocess::postprocess::{PostProcess as PostProcess1};
use pi_render::{
    components::view::target_alloc::ShareTargetView,
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer},
};

pub use super::root::RenderTarget;
use super::{
    calc::{DrawInfo, EntityKey, WorldMatrix, ZRange},
    user::{Aabb2, Matrix4, Point2},
};

/// 一个渲染Pass
pub struct Pass2D;

/// 相机
#[derive(Debug, Component)]
pub struct Camera {
    pub view: Matrix4,
    pub project: Matrix4,
    pub bind_group: Option<DrawBindGroup>,
    pub view_port: Aabb2,      // 视口区域（相对于全局的0,0点）
    pub world_matrix: Matrix4, // 将该相机内容整体渲染到其他目标时，所用的世界矩阵
    pub is_active: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            view: Matrix4::default(),
            project: Default::default(),
            bind_group: None,
            view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            world_matrix: Matrix4::default(),
            is_active: false,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct ViewMatrix {
    pub bind_group: Option<Handle<RenderRes<BindGroup>>>,
    // pub value: WorldMatrix,
}

#[derive(Debug, Default, Deref, Component)]
pub struct ParentPassId(pub EntityKey);

#[derive(Debug, Default, Deref, Component, Clone)]
pub struct ChildrenPass(pub Vec<EntityKey>);

// 渲染 物件 列表
#[derive(Debug, Component)]
pub struct Draw2DList {
    pub all_list: Vec<(DrawIndex, ZRange, DrawInfo)>,
	pub single_list: Vec<DrawIndex>, // 单独一个drawObj绘制在一个fbo上（需要做后处理的drawObj）
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
			single_list: Vec::default(),
            opaque: Vec::default(),
            transparent: Vec::default(),
        }
    }
}

/// 渲染对象的索引
#[derive(Debug, Clone, Copy, Hash, Component)]
pub enum DrawIndex {
    // 一个渲染对象
    DrawObj(EntityKey),
    // 一个Pass2D的内容
    Pass2D(EntityKey),
	// 一个经过后处理的渲染对象
	DrawObjPost(EntityKey),
}

#[derive(Default, Deref)]
pub struct DirtyMark(BitArray);

pub enum DirtyType {
    List = 1,      // 列表脏 （需要重新组织渲染列表）
    Depth = 2,     // 深度脏 （需要重新排序）
    DirtyRect = 3, // 脏区域
}

/// 脏区域, 设置在每个渲染上下文上
#[derive(Clone, Debug, Component)]
pub struct DirtyRect {
    pub value: Aabb2, // 在世界坐标上表示的脏区域
    pub state: DirtyRectState,
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            value: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            state: DirtyRectState::UnInit,
        }
    }
}

/// 上下文自身的脏区域(已考虑TransformWillchange)
#[derive(Clone, Debug, Component)]
pub struct LastDirtyRect {
    // pub last: Aabb2,
    pub no_will_change: Aabb2,
}

impl Default for LastDirtyRect {
    fn default() -> Self {
        LastDirtyRect {
            // last: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            no_will_change: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DirtyRectState {
    UnInit,
    Inited,
}

// /// 后处理
// #[derive(Clone)]
// pub struct PostProcess {
// 	pub draw_obj_key: DrawKey, // 渲染对象的Key
// 	pub texture_bind_index: usize, // 纹理bing_group的索引
// 	pub uv_vb_index: usize, // uv buffer在vbs中的索引

// 	pub result: Option<PostTemp>, // 处理目标

// 	pub width: u32, // 后处理渲染目标宽度
// 	pub height: u32, // 后处理渲染目标高度
// }

#[derive(Clone)]
pub struct PostTemp {
    pub target: ShareTargetView,
    pub texture_group: Handle<RenderRes<BindGroup>>,
    // pub matrix: Handle<RenderRes<BindGroup>>,
    pub uv: Handle<RenderRes<Buffer>>,
}

// impl PostProcess {
// 	pub fn new(
// 		draw_obj_key: DrawKey,
// 		texture_bind_index: usize,
// 		uv_vb_index: usize,
// 		width: u32,
// 		height: u32,
// 	) -> Self {
// 		Self {
// 			draw_obj_key,
// 			texture_bind_index,
// 			uv_vb_index,
// 			width,
// 			height,
// 			result: None,
// 		}
// 	}
// }

// /// 后处理列表

// // pub struct PostProcessList(pub SecondaryMap<DefaultKey, PostProcess>, pub DefaultKey/*最后一个后处理的key */);
// #[derive(Component, Debug)]
// pub struct PostProcessList {
//     post: PostProcess1,
//     pub info: PostProcessInfo,
//     // pub src: Option<ShareTargetView>,
// }

#[derive(Component, Debug)]
pub struct PostProcessInfo {
	pub effect_mark: bitvec::prelude::BitArray<[u32; 1]>,
    pub view_port: Aabb2,
    pub matrix: WorldMatrix, // 矩阵变换
}

impl Default for PostProcessInfo {
    fn default() -> Self {
        Self {
            effect_mark: bitvec::prelude::BitArray::default(),
            view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            matrix: Default::default(),
        }
    }
}

#[derive(Component, Debug, Deref, Default)]
pub struct PostProcess {
	#[deref]
	pub post: PostProcess1,
	pub depth: usize,
}


impl PostProcessInfo {
    pub fn has_effect(&self) -> bool {
        self.effect_mark.any()
        // let post = &self.post;
        // !(post.alpha.is_none()
        //     && post.copy.is_none()
        //     && post.hsb.is_none()
        //     && post.color_balance.is_none()
        //     && post.color_scale.is_none()
        //     && post.vignette.is_none()
        //     && post.color_filter.is_none()
        //     && post.blur_dual.is_none()
        //     && post.blur_direct.is_none()
        //     && post.blur_direct.is_none()
        //     && post.blur_radial.is_none()
        //     && post.blur_bokeh.is_none()
        //     && post.bloom_dual.is_none()
        //     && post.radial_wave.is_none()
        //     && post.filter_sobel.is_none()
        //     && post.horizon_glitch.is_none())
    }
}

#[derive(Resource)]
pub struct ScreenTarget {
    pub aabb: Aabb2,
    pub depth: Option<Handle<RenderRes<wgpu::TextureView>>>,
}
