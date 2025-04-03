//! 定义与Pass2D相关的组件

use std::ops::Range;

use pi_world::{insert::Component, prelude::Entity};
use pi_assets::asset::{Handle, Size, Asset};
pub use pi_bevy_render_plugin::render_cross::GraphId;
use pi_postprocess::postprocess::PostProcess as PostProcess1;
use pi_render::{
    components::view::target_alloc::{SafeTargetView, ShareTargetView},
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer},
};
use pi_share::{ShareWeak, Share};
use wgpu::RenderPipeline;

use crate::resource::RenderContextMarkType;

use super::{
    calc::{DrawInfo, EntityKey, WorldMatrix, ZRange},
    user::{Aabb2, AsImage, Point2},
};

/// 一个渲染Pass
pub struct Pass2D;

/// 世界矩阵的逆矩阵
#[derive(Default, Component, Debug)]
pub struct WorldMatrixInvert {
    pub value: Option<WorldMatrix>,
    // 标记哪些类型的上下文需要世界矩阵的逆矩阵
    // 目前， 设置TransfromWillchange、ClipPath的节点， 需要世界矩阵的逆矩阵计算
    // pub mark: bitvec::prelude::BitArray<[u32; 1]> 
    pub is_valid: bool, // 标记逆矩阵是否有效（一些pass节点不需要逆矩阵， 即便存在该组件， 也没有正确的计算出逆矩阵）
}

// /// 一个Pass2D必需含有该组件
// #[derive(Debug, Default)]
// pub struct Pass2DMark;

/// 相机
#[derive(Debug, Component)]
pub struct Camera {
    // pub view: Matrix4,
    // pub project: Matrix4,
    pub bind_group: Option<DrawBindGroup>,
    pub view_port: Aabb2,      // 非渲染视口区域（相对于全局的0,0点）
    // 是否渲染自身内容（如果为false，该相机不会渲染任何物体）
    // draw_changed为true时， is_render_own一定为true
    // draw_changed为false时，还需要看，从当前上下文开始向上递归，是否有上下文渲染目标被缓存，如果有，则is_render_own为false，否则为true
    pub is_render_own: bool,
    // 表示相机内的渲染内容是否改变
    pub draw_changed: bool,
    // 是否渲染到父目标(表示该pass是否渲染到父目标上)
    pub is_render_to_parent: bool,  
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            // view: Matrix4::default(),
            // project: Default::default(),
            bind_group: None,
            view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            is_render_own: false,
            draw_changed: true,
            is_render_to_parent: true,
        }
    }
}

// #[derive(Debug, Default)]
// pub struct ViewMatrix {
//     pub bind_group: Option<Handle<RenderRes<BindGroup>>>,
//     // pub value: WorldMatrix,
// }

#[derive(Debug, Default, Deref, Copy, Clone, Component)]
pub struct ParentPassId(pub EntityKey);

#[derive(Debug, Default, Deref, Clone, Component)]
pub struct ChildrenPass {
	#[deref]
	pub list: Vec<EntityKey>, 
	pub temp_count: usize,/*一个临时计数， 用于对依赖关系进行topp排序*/
    pub temp_has_effect: bool,
}

#[derive(Debug, Component)]
pub enum DrawElement {
	DrawInstance {
		draw_state: InstanceDrawState,
		draw_range: Range<usize>, // 在排序后的all_list列表中的范围
		depth_start: usize,
		pass: Entity, // 所在的psss
	}, 
    // 清理屏幕(清理多个pass对应的多个区域)
    Clear {
		draw_state: InstanceDrawState,
        // 每个pass的相机的is_active求或关系（即有一个相机被激活， 都必须清屏）
        // 需要注意： 
        // 所有pass中有一些pass处于激活状态， 而有一个asImage为force的节点， 处于未激活状态（应该使用原有的纹理，而不应该被清屏），
        // 这类节点的清屏， 其可见性应该设置为不可见TODO
		is_active: bool,
	}, 
    // // Pass2D类型， 需要递归渲染其对应的实例
	// Pass2D{
	// 	id: EntityKey,
	// 	depth: f32,
	// },
	GraphDrawList {
		id: EntityKey,
		depth_start: f32,
	}, // 由另一个图节点渲染，需要调用图节点的run, EntityKey为DrawObj节点id
    // 绘制后处理
    DrawPost(Range<usize>)
	// GraphFbo{
	// 	id: EntityKey,
	// 	draw_state: InstanceDrawState,
	// 	draw_range: Range<usize>, // 在排序后的all_list列表中的范围
	// 	depth_start: usize,
	// }, // 由另一个图节点渲染，需要调用图节点的run, EntityKey为DrawObj节点id
}

#[derive(Debug, Component, Default)]
pub struct InstanceDrawState {
	pub instance_data_range: Range<usize>, // 在单列RenderInstances中的范围
	pub pipeline: Option<Share<RenderPipeline>>, // 为None时， 默认使用全局默认的pipeline
	pub texture_bind_group: Option<Share<wgpu::BindGroup>>, // 为None时， 不需要绑定texture_bind
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
	/// 实例范围(不包含清屏实例): 当一些节点发生改变， 而当前pass的节点未发生变动， 则根据该范围从旧的实例数据拷贝到新的实例数据
	pub instance_range: Range<usize>,
    /// draw_call范围
    pub draw_range: Range<usize>,
	// pub need_dyn_fbo_index: Vec<usize>, // 一组在draw_list中的索引， 表示该draw_element需要在渲染图的build阶段动态创建资源（渲染资源，通常是fbo作为纹理的bindgroup，和uv）

	// 用于收集上下文中的渲染列表
    pub all_list: Vec<(DrawIndex, ZRange, DrawInfo)>,
	// all_list的排序结果
	pub all_list_sort: Vec<(DrawIndex, ZRange, DrawInfo)>,
    pub canvas_list: Vec<Entity>, // 单独一个drawObj绘制在一个fbo上（需要做后处理的drawObj）
    /// 不透明 列表
    pub opaque: Vec<(DrawIndex, ZRange, DrawInfo)>,

    /// 透明 列表
    pub transparent: Vec<(DrawIndex, ZRange, DrawInfo)>,
}

impl Default for Draw2DList {
    fn default() -> Self {
        Self {
			clear_instance: pi_null::Null::null(),
			list_is_change: false,
			draw_range: Default::default(),
			all_list_len: 0,
			instance_range: Default::default(),
			// need_dyn_fbo_index: Vec::default(),
			all_list_sort: Vec::default(),
            all_list: Vec::default(),
            canvas_list: Vec::default(),
            opaque: Vec::default(),
            transparent: Vec::default(),
        }
    }
}

impl Draw2DList {
	// push一个元素
	pub fn push_element(&mut self, draw_index: DrawIndex, z_range: ZRange, draw_info: DrawInfo) {
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component)]
pub enum DrawIndex {
	// // 清理屏幕
	// ClearScreen,
    // 一个渲染对象
    DrawObj{
        draw_entity: EntityKey,
        // #[cfg(debug_assertions)]
        node_entity: EntityKey,
    },
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
#[derive(Clone, Debug, Default, Component)]
pub struct DirtyMark(pub bool);

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
// #[derive(Debug)]
// pub struct PostProcessList {
//     post: PostProcess1,
//     pub info: PostProcessInfo,
//     // pub src: Option<ShareTargetView>,
// }

#[derive(Debug, Component)]
pub struct PostProcessInfo {
    pub effect_mark: bitvec::prelude::BitArray<[u32; 1]>,
    // pub view_port: Aabb2,
    // pub matrix: WorldMatrix, // 矩阵变换
}

impl Default for PostProcessInfo {
    fn default() -> Self {
        Self {
            effect_mark: bitvec::prelude::BitArray::default(),
            // view_port: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            // matrix: Default::default(),
        }
    }
}

#[derive(Deref, Default, Component)]
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
    pub fn is_not_only_as_image(&self, as_image_mark_type: &RenderContextMarkType<AsImage>) -> bool {
        let mut r = self.effect_mark.clone();
        r.set(**as_image_mark_type, false);
        return r.any();
    }
}

#[derive(Component)]
pub struct ScreenTarget {
    pub aabb: Aabb2,
    pub depth: Option<Handle<RenderRes<wgpu::TextureView>>>,
}

impl Default for ScreenTarget {
    fn default() -> Self {
        Self { aabb: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)), depth: None }
    }
}


// ///
// #[derive(Default)]
// pub struct RenderTarget(pub Option<ShareTargetView>);


#[derive(Deref, Debug, Clone, Component)]
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
    // 精确的bound_box，由于bound_box是整数数据， 并非渲染时的精确包围盒， 需要记录精确包围盒，以免出现渲染下次（范围： 0~1， 表示距离边界的偏移比例）
    pub accurate_bound_box: Aabb2,
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
            accurate_bound_box: Aabb2 {
                mins: Point2::new(0.0, 0.0),
                maxs: Point2::new(0.0, 0.0),
            }
        }
    }
}

#[derive(Debug, Default)]
pub enum StrongTarget {
	#[default]
	None,
    Asset(ShareTargetView),
}


#[derive(Debug, Default)]
pub enum RenderTargetCache {
	#[default]
	None,
    Strong(ShareTargetView),
    Weak(ShareWeak<SafeTargetView>),
}

