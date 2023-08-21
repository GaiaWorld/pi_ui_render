//! 定义与Pass2D相关的组件

use bevy::ecs::{prelude::Component, system::Resource};
use pi_assets::asset::{Handle, Size, Asset, Droper};
pub use pi_bevy_render_plugin::component::GraphId;
use pi_postprocess::postprocess::PostProcess as PostProcess1;
use pi_render::{
    components::view::target_alloc::{GetTargetView, SafeAtlasAllocator, SafeTargetView, ShareTargetView},
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer},
};
use pi_share::{Share, ShareWeak};
use pi_style::style::AsImage as AsImage1;

use crate::resource::{RenderContextMarkType, draw_obj::TargetCacheMgr};

use super::{
    calc::{DrawInfo, EntityKey, WorldMatrix, ZRange},
    draw_obj::DynTargetType,
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

// 渲染 物件 列表
#[derive(Debug, Component)]
pub struct Draw2DList {
    pub all_list: Vec<(DrawIndex, ZRange, DrawInfo)>,
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
    // 当前target所对应的在该节点的非旋转坐标系下的包围盒
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

impl RenderTarget {
    // 返回(渲染目标, 是否使用了新的渲染目标)
    // 如果未分配新的渲染目标，渲染时应该做脏更
    pub fn get_or_create<G: GetTargetView, T: Iterator<Item = G>>(
        &mut self,
        atlas_allocator: &SafeAtlasAllocator,
        as_image: Option<&AsImage>,
        assets: &TargetCacheMgr,
        as_image_mark_type: &RenderContextMarkType<AsImage>,
        post_info: &PostProcessInfo,
        t_type: &DynTargetType,
        max_cache: usize,
        exclude: T,
        is_force_alloc: bool,
    ) -> Option<Share<SafeTargetView>> {
        if is_force_alloc || post_info.has_effect() {
            match &self.target {
                StrongTarget::Asset(r) => return Some(r.0.clone()),
				StrongTarget::Raw(r) => return Some(r.0.clone()),
                StrongTarget::None => {
                    let width = (self.bound_box.maxs.x - self.bound_box.mins.x).ceil() as u32;
                    let height = (self.bound_box.maxs.y - self.bound_box.mins.y).ceil() as u32;

                    let as_image = match as_image {
                        Some(r) => r.0.clone(),
                        None => pi_style::style::AsImage::None,
                    };

					let capacity_overflow = assets.assets.size() as u32 + width * height * 4 > max_cache as u32;
                    // 如果设置节点为建议缓存，在显存已经超出max_cache的情况下， 不为其分配target， 该相机下的物体直接渲染到父target上
                    if AsImage1::Advise == as_image && post_info.is_only_as_image(as_image_mark_type) && capacity_overflow
                    {
                        return None;
                    };

                    // 分配渲染目标
                    let t = CacheTarget(atlas_allocator.allocate(width, height, t_type.has_depth, exclude));

                    match as_image {
                        AsImage1::None => {
							return Some(t.0);
							// // 放入资产管理器，由资产管理器管理
							// if capacity_overflow {
							// 	// self.target = StrongTarget::Raw(t.clone());
							// 	return Some(t.0);
							// } else {
							// 	let t = assets.push(t.clone());
							// 	// self.target = StrongTarget::Asset(t.clone());
							// 	return Some(t.0.clone());
							// }
							
						},
						r => {
							let t = assets.push(t.clone());
							match r {
								AsImage1::Advise => self.cache = RenderTargetCache::Weak(Share::downgrade(&t)),
								AsImage1::Force => self.cache = RenderTargetCache::Strong(t.clone()),
								_ => (),
							};
							// self.target = StrongTarget::Asset(t.clone());
							return Some(t.0.clone());
						}
                    };
                    
                    
                }
            }
        // // if let None = target {
        // // 如果后处理效果不只包含as_image，则
        // if post_info.is_only_as_image(as_image_mark_type) {
        // 	// || assets.size() as u32 + width * height * 4 <= max_cache as u32

        // 	return (Some(t.0), true)
        // }
        // }
        } else {
            None
        }
    }
}
