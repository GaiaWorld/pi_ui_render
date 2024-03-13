//! 定义与Pass2D相关的组件

use std::ops::Range;

use bevy_ecs::{prelude::Component, system::Resource};
use pi_assets::asset::{Handle, Size, Asset, Droper};
pub use pi_bevy_render_plugin::render_cross::GraphId;
use pi_postprocess::postprocess::PostProcess as PostProcess1;
use pi_render::{
    components::view::target_alloc::ShareTargetView,
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer, buffer_alloc::BufferIndex},
};
use pi_share::{ShareWeak, Share};
use wgpu::RenderPipeline;

use crate::resource::RenderContextMarkType;

use super::{
    calc::{DrawInfo, EntityKey, WorldMatrix, ZRange},
    user::{Aabb2, AsImage, Matrix4, Point2},
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
    pub is_active: bool,       // 是否激活相机（如果未激活，该相机不会渲染任何物体），通常相机不在脏区域内， 或相机内无任何drawobj，则该值为false
    pub is_change: bool, // 表示相机内的渲染内容是否改变， is_active为false时，该值为任何值都无所谓，is_active为true时，仅仅当内容相对于上一帧发生改变时，该值为true
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
            is_change: true,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct ViewMatrix {
    pub bind_group: Option<Handle<RenderRes<BindGroup>>>,
    // pub value: WorldMatrix,
}

#[derive(Debug, Default, Deref, Component, Copy, Clone)]
pub struct ParentPassId(pub EntityKey);

#[derive(Debug, Default, Deref, Component, Clone)]
pub struct ChildrenPass(pub Vec<EntityKey>);

#[derive(Debug)]
pub enum DrawElement {
	DrawInstance {
		draw_state: InstanceDrawState,
		draw_range: Range<usize>, // 在排序后的all_list列表中的范围
		depth_start: usize,
	}, 
	Pass2D{
		id: EntityKey,
		depth: f32,
	},
	GraphDrawList{
		id: EntityKey,
		depth_start: f32,
	}, // 由另一个图节点渲染，需要调用图节点的run, EntityKey为DrawObj节点id
	GraphFbo{
		id: EntityKey,
		draw_state: InstanceDrawState,
		draw_range: Range<usize>, // 在排序后的all_list列表中的范围
		depth_start: usize,
	}, // 由另一个图节点渲染，需要调用图节点的run, EntityKey为DrawObj节点id
}

#[derive(Debug)]
pub struct InstanceDrawState {
	pub instance_data_range: Range<usize>, // 在单列RenderInstances中的范围
	pub pipeline: Option<Share<RenderPipeline>>, // 为None时， 默认使用全局默认的pipeline
	pub texture_bind_group: Option<wgpu::BindGroup>, // 为None时， 不需要绑定texture_bind
}


// 渲染 物件 列表
#[derive(Debug, Component)]
pub struct Draw2DList {
	pub clear_instance: usize, // 清屏实例数据（清屏需要一次draw）
	// 渲染列表的长度
	// 在收集渲染列表的过程中，all_list保留了上一帧的列表数据，此字段用于记录all_list中有多少元素是当前帧有效的
	// 在收集过程中， 任何一个push的元素，与all_list[all_list_len]中的描述不匹配，都应该清理掉all_list_len之后的元素，并标记list_is_change为true
	// 在收集结束后，如果all_list_len与all_list.len不相等， 也应该清理掉all_list_len之后的元素，并标记list_is_change为true
	pub all_list_len: usize, 
	// 列表内容是否改变
	// 如果列表内容发生改变，则需要对all_list重新排序（排序结果记录在新的列表中）
	pub list_is_change: bool,
	// 绘制列表，每个上下文按此列表顺序绘制
	// 绘制内容可能是一个实例化Draw，也可能是一个上下文draw
	// 此列表根据all_list的排序结果，根据其中的DrawIndex::Pass2D将all_list劈分为多个"段"，每个段收缩为一个或多个实例化draw（肯呢个由于纹理个数的限制变成多个，通常为1个）
	// 并按原有的顺序，将实例化draw和pass2d存储在此结构体中
	pub draw_list: Vec<DrawElement>,
	pub need_dyn_fbo_index: Vec<usize>, // 一组在draw_list中的索引， 表示该draw_element需要在渲染图的build阶段动态创建资源（渲染资源，通常是fbo作为纹理的bindgroup，和uv）

	// 用于收集上下文中的渲染列表
    pub all_list: Vec<(DrawIndex, ZRange, DrawInfo)>,
	// all_list的排序结果
	pub all_list_sort: Vec<(DrawIndex, ZRange, DrawInfo)>,
    pub single_list: Vec<DrawIndex>, // 单独一个drawObj绘制在一个fbo上（需要做后处理的drawObj）
    /// 不透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub opaque: Vec<(DrawIndex, usize /*DepthGroup在DepthCache中的偏移*/)>,

    /// 透明 列表
    /// 注：渲染时，假设 Vec已经 排好序 了
    pub transparent: Vec<(DrawIndex, usize /*DepthGroup在DepthCache中的偏移*/)>,
}

impl Default for Draw2DList {
    fn default() -> Self {
        Self {
			clear_instance: pi_null::Null::null(),
			list_is_change: false,
			all_list_len: 0,
			draw_list: Vec::default(),
			need_dyn_fbo_index: Vec::default(),
			all_list_sort: Vec::default(),
            all_list: Vec::default(),
            single_list: Vec::default(),
            opaque: Vec::default(),
            transparent: Vec::default(),
        }
    }
}

impl Draw2DList {
	// push一个元素
	pub fn push_element(&mut self, draw_index: DrawIndex, z_range: ZRange, draw_info: DrawInfo) {
		log::trace!("push_element============{:?}, {:?}", self.all_list_len, self.all_list.len());
		if self.all_list_len == self.all_list.len() {
			self.all_list.push((draw_index, z_range, draw_info));
			self.list_is_change = true;
			self.all_list_len += 1;
			return;
		}

		let r = &self.all_list[self.all_list_len];

		if r.0 != draw_index || r.1.start != z_range.start || r.2 != draw_info {
			self.all_list[self.all_list_len] = (draw_index, z_range, draw_info);
			self.all_list_len += 1;
			self.shrink();
			return;
		}

		// push的元素与当前位置元素一样，则不需要push
		self.all_list_len += 1;
	}

	// 删除多余元素
	pub fn shrink(&mut self) {
		for _ in 0..self.all_list.len() - self.all_list_len {
			self.all_list.pop();
		}
		self.list_is_change = true;
	}

	pub fn reset(&mut self) {
		self.all_list_len = 0;
	}
}

/// 渲染对象的索引
#[derive(Debug, Clone, Copy, Hash, Component, PartialEq, Eq)]
pub enum DrawIndex {
	// // 清理屏幕
	// ClearScreen,
    // 一个渲染对象
    DrawObj(EntityKey),
    // 一个Pass2D的内容
    Pass2D(EntityKey),
    // 一个经过后处理的渲染对象
    DrawObjPost(EntityKey),
}

// #[derive(Default, Deref)]
// pub struct DirtyMark(BitArray);

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

// 标记Pass2D是否脏（子Pass2D脏， 该标记也为true）
// 用于判断缓存的fbo是否需要渲染
#[derive(Clone, Debug, Component, Default)]
pub struct DirtyMark(pub bool);

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

#[derive(Component, Deref, Default)]
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

    // 除了as_image， 是否存在其他上下文属性
    pub fn is_only_as_image(&self, as_image_mark_type: &RenderContextMarkType<AsImage>) -> bool {
        let mut r = self.effect_mark.clone();
        r.set(**as_image_mark_type, false);
        return r.any();
    }
}

#[derive(Resource)]
pub struct ScreenTarget {
    pub aabb: Aabb2,
    pub depth: Option<Handle<RenderRes<wgpu::TextureView>>>,
}


// ///
// #[derive(Component, Default)]
// pub struct RenderTarget(pub Option<ShareTargetView>);


#[derive(Deref, Debug, Clone)]
pub struct CacheTarget(pub ShareTargetView);

impl Size for CacheTarget {
	#[inline]
    fn size(&self) -> usize {
		self.0.size()
    }
}

impl Asset for CacheTarget {
    type Key = usize;
}

/// 渲染目标
// 缓冲ShareTargetView，当RenderTargetCache为Strong时， 强制缓冲， 当ShareTargetView为Weak时，此处并不强行缓冲ShareTargetView， 可能会被资产管理器回收
#[derive(Component)]
pub struct RenderTarget {
    // 渲染目标
    pub target: StrongTarget,
    // 缓冲ShareTargetView，当RenderTargetCache为Strong时， 强制缓冲， 当ShareTargetView为Weak时，此处并不强行缓冲ShareTargetView， 可能会被资产管理器回收
    pub cache: RenderTargetCache,
    // 当前target所对应的在该节点的非旋转坐标系下的包围盒（分配fbo的尺寸）
    pub bound_box: Aabb2,
}

impl Default for RenderTarget {
    fn default() -> Self {
        RenderTarget {
            target: Default::default(),
            cache: Default::default(),
            bound_box: Aabb2 {
                mins: Point2::new(0.0, 0.0),
                maxs: Point2::new(0.0, 0.0),
            },
        }
    }
}

#[derive(Debug, Default)]
pub enum StrongTarget {
	#[default]
	None,
    Asset(Handle<CacheTarget>),
    Raw(CacheTarget),
}

#[derive(Debug, Default)]
pub enum RenderTargetCache {
	#[default]
	None,
    Strong(Handle<CacheTarget>),
    Weak(ShareWeak<Droper<CacheTarget>>),
}

