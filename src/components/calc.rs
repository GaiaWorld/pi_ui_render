//! 定义计算组件（非用户设置的组件）

use pi_render::components::view::target_alloc::ShareTargetView;
use pi_world::insert::Component;
use pi_world::prelude::Entity;
use pi_style::style::AllTransform;
use pi_world::world::FromWorld;
use smallvec::SmallVec;
use std::hash::Hash;
/// 中间计算的组件
use std::{
    intrinsics::transmute,
    ops::{Deref, DerefMut, Mul},
};

use bitvec::prelude::BitArray;
use nalgebra::Matrix4;
use ordered_float::NotNan;
use pi_assets::asset::Handle;
use pi_null::Null;
use pi_render::rhi::asset::{TextureRes, AssetWithId};
use pi_share::Share;
use pi_slotmap::Key;

use crate::resource::{RenderObjType, ShareFontSheet};
use crate::utils::tools::calc_hash;

use super::user::*;

pub use super::root::RootDirtyRect;
pub use super::user::{NodeState, StyleType};

/// 布局结果
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Component)]

pub struct LayoutResult {
    pub rect: Rect<f32>,
    pub border: Rect<f32>,
    pub padding: Rect<f32>,
}

impl Default for LayoutResult {
    fn default() -> LayoutResult {
        LayoutResult {
            rect: Rect {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            border: Rect {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            padding: Rect {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
        }
    }
}

impl LayoutResult {
    pub fn size(&self) -> (f32, f32){
		(
			self.rect.right - self.rect.left,
			self.rect.bottom - self.rect.top,
        )
	}

	pub fn padding_box(&self) -> [f32; 4]{
		[
			self.border.left,
			self.border.top,
			self.rect.right - self.rect.left - self.border.left - self.border.right,
			self.rect.bottom - self.rect.top - self.border.top - self.border.bottom
		]
	}

    pub fn padding_rect(&self) -> Rect<f32>{
		Rect {
            left: self.border.left,
			top: self.border.top,
			right: self.rect.right - self.rect.left - self.border.right,
			bottom: self.rect.bottom - self.rect.top - self.border.bottom
        }
	}

	pub fn padding_aabb(&self) -> Aabb2 {
		Aabb2::new(
			Point2::new(
				self.border.left,
				self.border.top,
			),
			Point2::new(
				self.rect.right - self.rect.left - self.border.right,
				self.rect.bottom - self.rect.top - self.border.bottom
			)
		)
	}

	pub fn content_box(&self) -> [f32; 4]{
		[
			self.border.left + self.padding.left,
			self.border.top + self.padding.top,
			self.rect.right - self.rect.left - self.border.left - self.padding.left - self.border.right - self.padding.right,
			self.rect.bottom - self.rect.top - self.border.top - self.padding.top - self.border.bottom - self.padding.bottom
		]
	}

	pub fn content_aabb(&self) -> Aabb2{
		Aabb2::new(
			Point2::new(
				self.border.left + self.padding.left,
				self.border.top + self.padding.top,
			),
			Point2::new(
				self.rect.right - self.rect.left - self.border.left - self.padding.left,
				self.rect.bottom - self.rect.top - self.border.top - self.padding.top
			)
		)
	}

	pub fn border_box(&self) -> [f32; 4]{
		[
			0.0,
			0.0,
			self.rect.right - self.rect.left,
			self.rect.bottom - self.rect.top,
		]
	}

	pub fn border_aabb(&self) -> Aabb2{
		Aabb2::new(
			Point2::new(
				0.0,
				0.0,
			),
			Point2::new(
				self.rect.right - self.rect.left,
				self.rect.bottom - self.rect.top,
			)
		)
	}

    pub fn border_rect(&self) -> Rect<f32>{
        Rect {
            left: 0.0,
			top: 0.0,
			right: self.rect.right - self.rect.left,
			bottom: self.rect.bottom - self.rect.top
        }
	}
}

/// 内容最大包围盒范围(所有递归子节点的包围盒的最大范围，不包含自身)
#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct ContentBox {
    // 内容包围盒（自身+递归子节点的并）(不包含阴影的扩展)
    pub oct: Aabb2,
    // 布局包围盒（以父节点content的左上角为原点， 自身+递归子节点的并）（还包含了阴影的扩展）
    pub layout: Aabb2,
}

impl Default for ContentBox {
    fn default() -> Self {
        Self {
            oct: Aabb2::new(Point2::new(f32::MAX, f32::MAX), Point2::new(f32::MIN, f32::MIN)),
            layout: Aabb2::new(Point2::new(f32::MAX, f32::MAX), Point2::new(f32::MIN, f32::MIN)),
        }
    }
}

// ZIndex计算结果， 按照节点的ZIndex分配的一个全局唯一的深度表示
#[derive(Default, Deref, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Component)]
pub struct ZRange(pub std::ops::Range<usize>);

/// 渲染顺序
#[derive(Default, Deref, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Component)]
pub struct DrawInfo(pub u32);

impl DrawInfo {
    pub fn new(order: u8, is_opacity: bool) -> Self {
        // let r = unsafe { transmute::<_, u8>(false) };
        // let r1 = unsafe { transmute::<_, u8>(true) };
        // println!("xxxxxxxxx: {}, {}, {}, {}", r, r1,  ((unsafe { transmute::<_, u8>(is_opacity) } as u32) << 31), order as u32 | ((unsafe { transmute::<_, u8>(is_opacity) } as u32) << 31));
        Self(order as u32 | ((unsafe { transmute::<_, u8>(is_opacity) } as u32) << 31))
    }

    pub fn order(&self) -> i8 {
        self.0 as i8
        // (self.0 << 24 >> 24) as i8
    }

    pub fn is_opacity(&self) -> bool { (self.0 & (1 << 31)) > 0 }

	pub fn is_visibility(&self) -> bool { (self.0 & (1 << 30)) > 0 }

	pub fn set_visibility(&mut self, value: bool) { 
        self.0 = self.0 & !(1 << 30) | ((unsafe { transmute::<_, u8>(value) } as u32) << 30); 
    }

    pub fn is_by_cross(&self) -> bool { (self.0 & (1 << 29)) > 0 }
    pub fn set_by_cross(&mut self, value: bool) { 
        self.0 = self.0 & !(1 << 29) | ((unsafe { transmute::<_, u8>(value) } as u32) << 29); 
    }

	// 不透明排前面，透明排后面
	pub fn opacity_order(&self) -> usize { 
		if self.is_opacity() {
			0
		} else {
			1
		}
	}


    pub fn set_is_opacity(&mut self, value: bool) { self.0 = self.0 << 1 >> 1 | ((unsafe { transmute::<_, u8>(value) } as u32) << 31); }
}

// 世界矩阵，  WorldMatrix(矩阵, 矩阵描述的变换是存在旋转变换)， 如果不存在旋转变换， 可以简化矩阵的乘法
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct WorldMatrix(pub Matrix4<f32>, pub bool);

impl Hash for WorldMatrix {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.0.as_slice().iter() {
            unsafe { NotNan::new_unchecked(*i).hash(state) };
        }

        self.1.hash(state);
    }
}

// 默认值设置为单位阵
impl Default for WorldMatrix {
    fn default() -> Self { Self(Matrix4::new_translation(&Vector3::new(0.0, 0.0, 0.0)), false) }
}

impl WorldMatrix {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        if self.1 {
            let r = &*self * WorldMatrix(Matrix4::new_translation(&Vector3::new(x, y, z)), false);
            *self = r;
        } else {
            let slice = self.0.as_mut_slice();
            slice[12] += slice[0] * x;
            slice[13] += slice[5] * y;
            slice[14] += slice[10] * z;
        }
        self
    }

    pub fn form_transform_funcs(transformfuncs: &TransformFuncs, width: f32, height: f32) -> WorldMatrix {
        if transformfuncs.len() > 0 {
            let mut m = Self::get_matrix(&transformfuncs[0], width, height);
            for i in 1..transformfuncs.len() {
                m = m * Self::get_matrix(&transformfuncs[i], width, height);
            }
            m
        } else {
            WorldMatrix::default()
        }
    }


    // pub fn form_transform(transform: &Transform, width: f32, height: f32) -> WorldMatrix {
    //     if transform.funcs.len() > 0 {
    //         let mut m = Self::get_matrix(&transform.funcs[0], width, height);
    //         for i in 1..transform.funcs.len() {
    //             m = m * Self::get_matrix(&transform.funcs[i], width, height);
    //         }
    //         m
    //     } else {
    //         WorldMatrix::default()
    //     }
    // }

    pub fn form_transform_layout(all_transform: &AllTransform, origin: &TransformOrigin, width: f32, height: f32, left_top: &Point2) -> WorldMatrix {
        // M = T * R * S
        // let mut m = cg::Matrix4::new(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 1.0, 0.0,
        //     0.0, 0.0, 0.0, 1.0,
        // );


        // 矩阵变换是以父节点的左上角为原点变换的，left_top表明本节点左上角相对父节点左上角的位移
        let orgin_move_value = origin.to_value(width, height);
        let move_value = Point2::new(left_top.x + orgin_move_value.x, left_top.y + orgin_move_value.y);

        // 变换前先将transform描述的原点位置移动到父节点的左上角
        let mut m = WorldMatrix(Matrix4::new_translation(&Vector3::new(move_value.x, move_value.y, 0.0)), false);

        if let Some(scale) = &all_transform.scale {
            m = m * WorldMatrix(Matrix4::new_nonuniform_scaling(&Vector3::new(scale[0], scale[1], 1.0)), false);
        }

        if let Some(rotate) = &all_transform.rotate {
            m = m * WorldMatrix(
                Matrix4::new_rotation(Vector3::new(0.0, 0.0, *rotate / 180.0 * std::f32::consts::PI)),
                true,
            );
        }

        if let Some(translate) = &all_transform.translate {
            m.translate(translate[0].get_absolute_value(width), translate[1].get_absolute_value(height), 0.0);
        }

        // 计算tranform
        for func in all_transform.transform.iter() {
            m = m * Self::get_matrix(func, width, height);
        }

        // 变化后再将节点移动回来
        m * WorldMatrix(
            Matrix4::new_translation(&Vector3::new(-orgin_move_value.x, -orgin_move_value.y, 0.0)),
            false,
        )
    }

    fn get_matrix(func: &TransformFunc, width: f32, height: f32) -> WorldMatrix {
        match func {
            TransformFunc::TranslateX(x) => WorldMatrix(Matrix4::new_translation(&Vector3::new(x.get_absolute_value(width), 0.0, 0.0)), false),
            TransformFunc::TranslateY(y) => WorldMatrix(Matrix4::new_translation(&Vector3::new(0.0, y.get_absolute_value(height), 0.0)), false),
            TransformFunc::Translate(x, y) => WorldMatrix(
                Matrix4::new_translation(&Vector3::new(x.get_absolute_value(width), y.get_absolute_value(height), 0.0)),
                false,
            ),

            TransformFunc::ScaleX(x) => WorldMatrix(Matrix4::new_nonuniform_scaling(&Vector3::new(*x, 1.0, 1.0)), false),
            TransformFunc::ScaleY(y) => WorldMatrix(Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, *y, 1.0)), false),
            TransformFunc::Scale(x, y) => WorldMatrix(Matrix4::new_nonuniform_scaling(&Vector3::new(*x, *y, 1.0)), false),

            TransformFunc::RotateZ(z) => WorldMatrix(Matrix4::new_rotation(Vector3::new(0.0, 0.0, *z / 180.0 * std::f32::consts::PI)), true),
            TransformFunc::RotateX(x) => WorldMatrix(Matrix4::new_rotation(Vector3::new(*x / 180.0 * std::f32::consts::PI, 0.0, 0.0)), true),
            TransformFunc::RotateY(y) => WorldMatrix(Matrix4::new_rotation(Vector3::new(0.0, *y / 180.0 * std::f32::consts::PI, 0.0)), true),
            TransformFunc::SkewX(x) => WorldMatrix(
                Matrix4::new(
                    1.0,
                    (*x / 180.0 * std::f32::consts::PI).tan(),
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ),
                true,
            ),
            TransformFunc::SkewY(y) => WorldMatrix(
                Matrix4::new(
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    (*y / 180.0 * std::f32::consts::PI).tan(),
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ),
                true,
            ),
        }
    }
}

impl Deref for WorldMatrix {
    type Target = Matrix4<f32>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for WorldMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, 'b> Mul<&'a WorldMatrix> for &'b WorldMatrix {
    type Output = WorldMatrix;
    fn mul(self, other: &'a WorldMatrix) -> Self::Output {
        if self.1 == false && other.1 == false {
            WorldMatrix(self.0 * other.0, false)
        } else {
            WorldMatrix(self.0 * other.0, true)
        }
    }
}

impl<'a> Mul<&'a WorldMatrix> for WorldMatrix {
    type Output = WorldMatrix;
    #[inline]
    fn mul(self, other: &'a WorldMatrix) -> Self::Output {
        if self.1 == false && other.1 == false {
            WorldMatrix(self.0 * other.0, false)
        } else {
            WorldMatrix(self.0 * other.0, true)
        }
    }
}

impl<'a> Mul<WorldMatrix> for &'a WorldMatrix {
    type Output = WorldMatrix;
    #[inline]
    fn mul(self, other: WorldMatrix) -> Self::Output {
        if self.1 == false && other.1 == false {
            WorldMatrix(self.0 * other.0, false)
        } else {
            WorldMatrix(self.0 * other.0, true)
        }
    }
}

impl Mul<WorldMatrix> for WorldMatrix {
    type Output = WorldMatrix;
    #[inline]
    fn mul(self, other: WorldMatrix) -> Self::Output {
        if self.1 == false && other.1 == false {
            WorldMatrix(self.0 * other.0, false)
        } else {
            WorldMatrix(self.0 * other.0, true)
        }
    }
}

impl<'a> Mul<&'a Vector4> for WorldMatrix {
    type Output = Vector4;
    fn mul(self, other: &'a Vector4) -> Vector4 { self.0 * other }
}

impl<'a> Mul<&'a Vector4> for &'a WorldMatrix {
    type Output = Vector4;
    fn mul(self, other: &'a Vector4) -> Vector4 { &self.0 * other }
}

impl<'a> Mul<Vector4> for &'a WorldMatrix {
    type Output = Vector4;
    fn mul(self, other: Vector4) -> Vector4 { self.0 * other }
}

impl WorldMatrix {
    pub fn invert(&self) -> Option<Self> {
        match self.0.try_inverse() {
            Some(r) => Some(Self(r, self.1)),
            None => None,
        }
    }
}

// #[storage = ]
#[derive(Clone, Debug, Serialize, Deserialize, Component)]
// #[storage(QuadTree)]
pub struct Quad(pub Aabb2, );

impl Quad {
    pub fn new(aabb: Aabb2) -> Quad { Quad(aabb) }
}

impl Default for Quad {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0))) }
}

impl Deref for Quad {
    type Target = Aabb2;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Quad {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Component)]
pub struct IsShow(usize);

// impl Default for IsShow {
//     fn default() -> IsShow { IsShow(ShowType::Visibility as usize | ShowType::Enable as usize) }
// }

impl IsShow {
    #[inline]
    pub fn get_visibility(&self) -> bool { (self.0 & (ShowType::Visibility as usize)) != 0 }

    #[inline]
    pub fn set_visibility(&mut self, visibility: bool) {
        if visibility {
            self.0 |= ShowType::Visibility as usize;
        } else {
            self.0 &= !(ShowType::Visibility as usize);
        }
    }

	#[inline]
    pub fn get_display(&self) -> bool { (self.0 & (ShowType::Display as usize)) != 0 }

    #[inline]
    pub fn set_display(&mut self, display: bool) {
        if display {
            self.0 |= ShowType::Display as usize;
        } else {
            self.0 &= !(ShowType::Display as usize);
        }
    }

    #[inline]
    pub fn get_enable(&self) -> bool { (self.0 & (ShowType::Enable as usize)) != 0 }

    #[inline]
    pub fn set_enable(&mut self, enable: bool) {
        if enable {
            self.0 |= ShowType::Enable as usize;
        } else {
            self.0 &= !(ShowType::Enable as usize);
        }
    }
}

/// canvas图节点
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanvasGraph {
    // 外部系统的图节点
    pub old_canvas_graph_id: pi_render::depend_graph::NodeId,
    // 由于外部系统只在图节点输出中曝露其渲染目标， gui系统单独创建一个图节点，连接到外部图节点， 以便将其输出放在在canvas实体的组件上，供gui使用
	pub copy_graph_id: pi_render::depend_graph::NodeId, 
}

// // 上一个版本在IsShow组件中体现， 现在独立出来
// // 节点layer为0是，IsDisplay会设置为false 
// // 是否为display（单独为一个组件， display会印象渲染实例个数， display为false的节点，不组织渲染数据）
// // 组织渲染数据比较费， 将display独立为一个组件，有利于ecs过滤
// #[derive(Deref, Clone, Debug, PartialEq, Serialize, Deserialize, Default, Component)]
// pub struct IsDisplay(pub bool);

// 样式标记
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Component)]
pub struct StyleMark {
    pub dirty_style: StyleMarkType, // 样式脏（标记有哪些样式在本帧中脏了）
    pub local_style: StyleMarkType, // 本地样式， 表示节点样式中，哪些样式是由style设置的（而非class设置）
    pub class_style: StyleMarkType, // class样式， 表示节点样式中，哪些样式是由class设置的
}

pub type StyleMarkType = BitArray<[u32; 5]>;

/// 标记渲染context中需要的效果， 如Blur、Opacity、Hsi、MasImage等
/// 此数据结构仅记录位标记，具体哪些属性用哪一位来标记，这里并不关心，由逻辑保证
#[derive(Clone, Debug, Default, Deref, Serialize, Deserialize, Component)]
pub struct RenderContextMark(bitvec::prelude::BitArray<[u32; 1]>);

pub trait NeedMark {
    fn need_mark(&self) -> bool;
}

// // 字符节点， 对应一个字符的
// #[derive(Debug, Clone, Default)]
// pub struct CharNode{
//     pub ch: char,              // 字符
//     pub ch_id_or_count: usize, // 字符id或单词的字符数量
// 	pub base_width: f32,       // font_size 为32 的字符宽度
// 	pub width: f32,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct CharNode {
//     pub ch: char,                // 字符
//     pub margin_start: f32, // margin
//     pub size: (f32, f32),        // 字符大小
//     pub pos: (f32, f32),         // 位置
//     pub ch_id_or_count: usize,   // 字符id或单词的字符数量
//     pub base_width: f32,         // font_size 为32 的字符宽度
// 	pub char_i: isize,// 字符在整个节点中的索引
// 	pub context_id: isize, // 如果是多字符文字中的某个字符，则存在一个容易索引
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct CharNode {
//     pub ch: char,                // 字符
//     pub margin: Rect<Dimension>, // margin
//     pub size: Size<Dimension>,        // 字符大小
//     pub pos: Rect<f32>,         // 位置
//     pub ch_id_or_count: usize,   // 字符id或单词的字符数量 ch==char::from(0)时，表示单词容器节点，此时ch_id_or_count表示该节点中的字符数量
//     pub base_width: f32,         // font_size 为32 的字符宽度
// 	pub char_i: isize,// 字符在整个节点中的索引
// 	pub context_id: isize, // 如果是多字符文字中的某个字符，则存在一个上下文索引
// 	pub prev: usize,
// 	pub next: usize,
// }

// #[derive(Deref, Clone, Debug, Serialize, Deserialize)]
// pub struct TextChars(Vec<CharNode>);

// TransformWillChange的矩阵计算结果， 用于优化Transform的频繁改变
#[derive(Debug, Clone, Default, Deref, Component)]
pub struct TransformWillChangeMatrix(pub Option<Share<TransformWillChangeMatrixInner>>);

impl TransformWillChangeMatrix {
    pub fn new(will_change_invert: WorldMatrix, will_change: WorldMatrix, primitive: WorldMatrix) -> TransformWillChangeMatrix {
        TransformWillChangeMatrix(Some(Share::new(TransformWillChangeMatrixInner {
            will_change,
            will_change_invert,
            primitive: primitive.clone(), 
            invert: primitive,
        })))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransformWillChangeMatrixInner {
    // will_change矩阵（will_change*worldmatrix*顶点=真实的世界坐标位置） = ParentWorldMatrix * primitive * ParentWorldMatrix逆
    pub will_change: WorldMatrix, 
    // will_change 逆
    pub will_change_invert: WorldMatrix, 
    // pub primitive: WorldMatrix;// = Parent1.WillChangeTransform * Parent2.WillChangeTransform * ... * this.WillChangeTransform
    pub invert: WorldMatrix,  // will_change属性所在节点的世界矩阵的逆， 
    pub primitive: WorldMatrix,  // will_change属性所在节点的willchange_transfrom对应的矩阵，
}

#[derive(Debug, Clone)]
pub struct SdfSlice {
    pub sdf_slice: Rect<f32>, // 0~1
    pub layout_slice: Rect<f32>,
}

impl Default for SdfSlice {
    fn default() -> Self {
        Self { 
        sdf_slice: Rect {
            top: 0.0,
            left: 0.0,
            right: 1.0,
            bottom: 1.0,
        }, 
        layout_slice: Rect {
            top: 0.0,
            left: 0.0,
            right: 1.0,
            bottom: 1.0,
        },
     }
    }
}

// 单位： 像素
#[derive(Debug, Clone)]
pub struct SdfUv (pub Rect<f32>, pub f32);


// 单位： 像素
#[derive(Debug, Clone, Component)]
pub struct BorderSdfUv (pub Rect<f32>, pub f32);

impl Default for BorderSdfUv {
    fn default() -> Self {
        Self (Rect {
            top: 0.0,
            left: 0.0,
            right: 1.0,
            bottom: 1.0,
        }, 1.0)
    }
}

impl FromWorld for SdfUv {
    fn from_world(world: &mut pi_world::world::World) -> Self {
        if let Some(sdf_uv) = world.get_single_res::<SdfUv>() {
            return (**sdf_uv).clone();
        }
        let font_sheet = world.get_single_res_mut::<ShareFontSheet>().unwrap();
        
        let mut font_sheet = font_sheet.borrow_mut();
        let rect = pi_hal::svg::Rect::new(0.0, 0.0, 32.0, 32.0);
        let hash = calc_hash(&"rect sdf", 0);
        font_sheet.font_mgr_mut().table.sdf2_table.add_shape(calc_hash(&"rect sdf", 0), rect.get_svg_info(), 32, 128, 2);

        let info = font_sheet.font_mgr_mut().table.sdf2_table.get_shape(hash).unwrap();
        log::debug!("rect SdfUv========================{:?}",Rect {
            top: info.y as f32 + 5.0,
            left: info.x as f32 + 5.0,
            right: info.x as f32 + info.width as f32 - 5.0,
            bottom: info.y as f32 + info.height as f32 - 5.0,
        });
        // 矩形由于水平和竖直方向缩放比例不一致， 因此不能通过统一的pxrange进行抗锯齿， 这里直接将矩形范围缩小， 使得采样结果总在矩形内，使结果渲染正确， 缺点是不能抗锯齿
        Self (Rect {
            top: info.y as f32 + 5.0,
            left: info.x as f32 + 5.0,
            right: info.x as f32 + info.width as f32 - 5.0,
            bottom: info.y as f32 + info.height as f32 - 5.0,
        }, 128.0 * 2.0)
    }
}

#[derive(Debug, Clone, Default, Deref)]
pub struct MaskTexture(pub Option<Texture>);

impl Null for MaskTexture {
    fn null() -> Self { 
		Self (None)
	}

    fn is_null(&self) -> bool { self.0.is_none() }
}

impl PartialEq for MaskTexture {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
		match (&self.0, &other.0) {
			(None, None) => true,
			(None, Some(_)) => false,
			(Some(_), None) => false,
			(Some(r1), Some(r2)) =>  r1.eq(r2),
		}
    }
}
impl Eq for MaskTexture {}


impl From<Texture> for MaskTexture {
    fn from(handle: Texture) -> Self { MaskTexture(Some(handle)) }
}

impl From<Option<Handle<AssetWithId<TextureRes>>>> for MaskTexture {
    fn from(handle: Option<Handle<AssetWithId<TextureRes>>>) -> Self { MaskTexture(match handle {
        Some(handle) => Some(Texture::All(handle)),
        None => None,
    }) }
}

// impl From<MaskTexture> for Option<Handle<AssetWithId<TextureRes>>>{
//     fn from(mask_texture: MaskTexture) -> Self { mask_texture.0 }
// }

#[derive(Deref, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct EntityKey(pub Entity);

impl Key for EntityKey {
    fn data(&self) -> pi_slotmap::KeyData {
        // (u64::from(self.version.get()) << 32) | u64::from(self.idx)
        self.0.data()
        // pi_slotmap::KeyData::from_ffi((u64::from(self.0.generation()) << 32) | u64::from(self.0.index()))
    }

	fn index(&self) -> usize {
		self.0.index() as usize
	}

    fn with(idx: usize) -> Self {
        Self(Entity::with(idx))
        // Self(Entity::from_raw(idx as u32))
    }
}

impl From<pi_slotmap::KeyData> for EntityKey {
    fn from(value: pi_slotmap::KeyData) -> Self { 
        Self(Entity::from(value))
    }
}

impl Default for EntityKey {
    fn default() -> Self { 
        Self(Entity::null())
    }
}

impl Null for EntityKey {
    fn null() -> Self { 
        Self(Entity::null())
    }

    fn is_null(&self) -> bool { self.0.is_null() }
}

// /// 上下文的实体ID，作为Node的组件，关联由其创建的渲染上下文
// #[derive(Deref, Default, Debug, Hash, Clone, Copy)]
// pub struct Pass2DId(pub EntityKey);

/// 作为Node的组件，表示节点所在的渲染上下文的实体
#[derive(Clone, Copy, Deref, Default, PartialEq, Eq, Debug, Hash, Serialize, Deserialize, Component)]
pub struct InPassId(pub EntityKey);


pub enum FlexStyleType {
    Width = 54,
    Height = 55,

    Direction = 38,
    AspectRatio = 39,
    Order = 40,
    FlexBasis = 41,

    MarginTop = 56,
    MarginRight = 57,
    MarginBottom = 58,
    MarginLeft = 59,

    PaddingTop = 60,
    PaddingRight = 61,
    PaddingBottom = 62,
    PaddingLeft = 63,

    BorderTop = 64,
    BorderRight = 65,
    BorderBottom = 66,
    BorderLeft = 67,

    PositionTop = 68,
    PositionRight = 69,
    PositionBottom = 70,
    PositionLeft = 71,

    MinWidth = 72,
    MinHeight = 73,
    MaxHeight = 74,
    MaxWidth = 75,
    JustifyContent = 76,
    FlexShrink = 77,
    FlexGrow = 78,
    PositionType = 79,
    FlexWrap = 80,
    FlexDirection = 81,
    AlignContent = 82,
    AlignItems = 83,
    AlignSelf = 84,
    BlendMode = 85,
}

/// 节点的实体id，作为RenderContext的组件，引用创建该渲染上下文的节点
#[derive(Deref, Debug, Clone, Copy, Hash, Default, Component)]
pub struct NodeId(pub EntityKey);

/// 每节点的渲染列表
#[derive(Deref, Default, Debug, Clone, Serialize, Deserialize, Component)]
pub struct DrawList(pub SmallVec<[DrawObjId; 1]>); // 通常只会有一个DrawObject

impl DrawList {
    #[inline]
    pub fn push(&mut self, ty: RenderObjType, id: Entity) { self.0.push(DrawObjId { ty, id }); }

    // 取到一个（大部分只有一个drawobj， 少数有多个，如text_shadow）
    pub fn get_one(&self, ty: RenderObjType) -> Option<&DrawObjId> {
        for i in 0..self.0.len() {
            if self.0[i].ty == ty {
                return Some(&self.0[i]);
            }
        }
        None
    }

    // 移除全部
    pub fn remove<F: FnMut(DrawObjId)>(&mut self, ty: RenderObjType, mut cb: F) {
        let mut i: usize = 0;
        while i < self.0.len() {
            if self.0[i].ty == ty {
                cb(self.0.swap_remove(i));
            } else {
                i += 1;
            }
        }
    }

    pub fn count(&self, ty: RenderObjType) -> usize {
        let mut i: usize = 0;
        let mut count = 0;
        while i < self.0.len() {
            if self.0[i].ty == ty {
                count += 1;
            }
            i += 1;
        }
        count
    }

    pub fn get_first(&self, ty: RenderObjType) -> usize {
        let mut i: usize = 0;

        while i < self.0.len() {
            if self.0[i].ty == ty {
                return i;
            }
            i += 1;
        }
        Null::null()
    }
}

/// 节点上握住DrawObj的id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawObjId {
    pub ty: RenderObjType,
    pub id: Entity,
}

/// 视图
/// 每个Pass2d都必须存在一个视图
#[derive(Clone, Default, Debug, Component, Serialize, Deserialize)]
pub struct View {
    /// 为some时，节点上下文渲染需要新的视口，否则应该继承父节点的视口
    pub view_box: ViewBox,
    /// 旋转情况下是Some， 记录旋转矩阵和旋转逆矩阵
    pub desc: OverflowDesc,
}

/// 可视包围盒
/// 已经考虑了Overflow、TransformWillChange因素，得到了该节点的真实可视区域
#[derive(Clone, Debug, Component, Serialize, Deserialize)]
pub struct ViewBox {
    /// 当前节点的可视包围盒(已考虑transformwillchange)
    /// 其原点位置是对世界原点作本节点旋转变换的逆变换所得
    /// 如果该节点overflow为**false**
    /// ---如果当前节点**存在旋转**，则为当前节点的**ContentBox.oct * 旋转逆矩阵**
    /// ---如果当前节点**不存在旋转**， 则为当前节点**ContentBox.oct 与 父上下文的ViewBox.aabb相交**
    /// 如果节点overflow为**true**
    /// ---如果当前节点**存在旋转**，则为当前节点的**ContentBox.layout * WorldMatrix * 旋转逆矩阵**
    /// ---如果当前节点**不存在旋转**， 则为当前节点**ContentBox.layout * WorldMatrix 与 父上下文的ViewBox.aabb相交**
    pub aabb: Aabb2,
    /// 与aabb表示同一个矩形区域，只是原点为世界坐标原点（由于可能存在旋转， 如果原点为世界坐标原点时， 该区域不能用Aabb表示）
    pub world_quad: (Vector2, Vector2, Vector2, Vector2),
}

impl Default for ViewBox {
    fn default() -> Self {
        Self {
            aabb: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            world_quad: Default::default(),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct OveflowRotate {
    // 相对于父上下文的旋转
    pub from_context_rotate: WorldMatrix,
    // 节点相对于世界坐标的渲染
    pub world_rotate: WorldMatrix,
    // 节点相对于世界坐标的旋转的逆
    pub world_rotate_invert: Matrix4<f32>,
}

// 描述oveflow
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverflowDesc {
    Rotate(OveflowRotate), // 所在节点存在旋转的情况下， 描述旋转信息
    NoRotate(Aabb2),       // 所在节点不存在旋转的情况下，描述自身内容的世界aabb
}

impl Default for OverflowDesc {
    fn default() -> Self { OverflowDesc::NoRotate(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0))) }
}

/// BorderImageTexture.0只有在设置了图片路径，但纹理还未加载成功的情况下，才会为none
/// 如果删除了图片路径，会删除该组件
#[derive(Deref, Default)]
pub struct BorderImageTexture(pub Option<Texture>);

impl PartialEq for BorderImageTexture {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
		match (&self.0, &other.0) {
			(None, None) => true,
			(None, Some(_)) => false,
			(Some(_), None) => false,
			(Some(r1), Some(r2)) => r1.eq( r2),
		}
    }
}
impl Eq for BorderImageTexture {}


impl From<Texture> for BorderImageTexture {
    fn from(handle: Texture) -> Self { Self(Some(handle)) }
}


impl Null for BorderImageTexture {
    fn null() -> Self { Self(None) }

    fn is_null(&self) -> bool { self.0.is_none() }
}

#[derive(Debug, Clone)]
pub enum Texture {
    All(Handle<AssetWithId<TextureRes>>),
    Part(ShareTargetView, Entity)
}

impl Texture {
    pub fn size(&self) -> FlexSize<u32> {
        match self {
            Texture::All(handle) => FlexSize {
                width: handle.width,
                height: handle.height,
            },
            Texture::Part(r, _) => {
                let rect = r.rect();
                FlexSize {
                    width: rect.width() as u32,
                    height: rect.height() as u32,
                }
            }
        }
    }

    pub fn to_uv(&self, clip: &NotNanRect) -> (Point2, Point2) {
        match self {
            Texture::All(_handle) => (
                Point2::new(*clip.left, *clip.top),
                Point2::new(*clip.right, *clip.bottom),
            ),
            Texture::Part(r, _) => {
                let rect = r.rect();
                let size = FlexSize {
                    width: rect.width() as f32,
                    height: rect.height() as f32,
                };
                let (top, right, bottom, left) = (
                    *clip.top * size.height,
                    *clip.right * size.width,
                    *clip.bottom * size.height,
                    *clip.left * size.width,
                );
                let (width, height) = (r.target().width as f32, r.target().height as f32);
                (
                    Point2::new((left + rect.min.x as f32) / width, (top + rect.min.y as f32) / height),
                    Point2::new((right + rect.min.x as f32) / width, (bottom + rect.min.y as f32) / height),
                )
            }
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Texture::All(r1), Texture::All(r2)) =>  Share::ptr_eq(r1, r2),
            (Texture::Part(r1, e1), Texture::Part(r2, e2)) =>  Share::ptr_eq(r1, r2) && e2 == e1,
            _ => false,
        }
    }
}

/// BackgroundImageTexture.0只有在设置了图片路径，但纹理还未加载成功的情况下，才会为none
/// 如果删除了图片路径，会删除该组件
#[derive(Deref, Default)]
pub struct BackgroundImageTexture(pub Option<Texture>);

impl PartialEq for BackgroundImageTexture {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
		match (&self.0, &other.0) {
			(None, None) => true,
			(None, Some(_)) => false,
			(Some(_), None) => false,
			(Some(r1), Some(r2)) => r1.eq(r2),
		}
    }
}
impl Eq for BackgroundImageTexture {}

impl From<Texture> for BackgroundImageTexture {
    fn from(handle: Texture) -> Self { Self(Some(handle)) }
}

impl Null for BackgroundImageTexture {
    fn null() -> Self { Self(None) }

    fn is_null(&self) -> bool { self.0.is_none() }
}

#[inline]
pub const fn style_bit() -> StyleMarkType {
	BitArray::ZERO
}

pub trait StyleBit {
	fn set_bit(self, index: usize) -> Self;
    fn has_any(&self, other: &Self) -> bool;
}

impl StyleBit for StyleMarkType {
    fn set_bit(mut self, index: usize) -> Self {
        self.set(index, true);
		self
    }

    fn has_any(&self, other: &Self) -> bool {
        (self.data[0] & other.data[0]).trailing_zeros() != 32 || 
        (self.data[1] & other.data[1]).trailing_zeros() != 32 || 
        (self.data[2] & other.data[2]).trailing_zeros() != 32 || 
        (self.data[3] & other.data[3]).trailing_zeros() != 32
    }
}

lazy_static! {
	// margin标记
	pub static ref LAYOUT_MARGIN_MARK: StyleMarkType =
	style_bit().set_bit(StyleType::MarginTop as usize).set_bit(StyleType::MarginRight as usize).set_bit(StyleType::MarginBottom as usize).set_bit(StyleType::MarginLeft as usize);
	// pading标记
	pub static ref LAYOUT_PADDING_MARK: StyleMarkType =
	style_bit().set_bit(StyleType::PaddingTop as usize).set_bit(StyleType::PaddingRight as usize).set_bit(StyleType::PaddingBottom as usize).set_bit(StyleType::PaddingLeft as usize);
	// border标记
	pub static ref LAYOUT_BORDER_MARK: StyleMarkType =
	style_bit().set_bit(StyleType::BorderTop as usize).set_bit(StyleType::BorderRight as usize).set_bit(StyleType::BorderBottom as usize).set_bit(StyleType::BorderLeft as usize);
	// border标记
	pub static ref LAYOUT_POSITION_MARK: StyleMarkType =
	style_bit().set_bit(StyleType::PositionTop as usize).set_bit(StyleType::PositionRight as usize).set_bit(StyleType::PositionBottom as usize).set_bit(StyleType::PositionLeft as usize);
	// 矩形属性标记
	pub static ref LAYOUT_RECT_MARK: StyleMarkType = style_bit().set_bit(StyleType::Width as usize).set_bit(StyleType::Height as usize) | &*LAYOUT_MARGIN_MARK;


	// 矩形区域脏，绝对定位下，设自身self_dirty，相对定位下，设自身self_dirty后，还要设父child_dirty
	pub static ref RECT_DIRTY: StyleMarkType = style_bit().set_bit(StyleType::Width as usize)
	.set_bit(StyleType::Height as usize)
    .set_bit(StyleType::MinWidth as usize)
    .set_bit(StyleType::MinHeight as usize)
		| &*LAYOUT_POSITION_MARK
		| &*LAYOUT_MARGIN_MARK;

	// 普通脏及子节点添加或移除， 设父child_dirty
	pub static ref NORMAL_DIRTY: StyleMarkType = //StyleType::FlexBasis as usize 
		//.set_bit(StyleType::Order as usize)
		style_bit().set_bit(StyleType::FlexShrink as usize)
		.set_bit(StyleType::FlexGrow as usize)
		.set_bit(StyleType::AlignSelf as usize)
		.set_bit(StyleType::PositionType as usize);

	// 自身脏， 仅设自身self_dirty
	pub static ref SELF_DIRTY: StyleMarkType = LAYOUT_PADDING_MARK.clone() 
		| &*LAYOUT_BORDER_MARK;

	// 子节点脏， 仅设自身child_dirty
	pub static ref CHILD_DIRTY: StyleMarkType = style_bit().set_bit(StyleType::FlexDirection as usize)
		.set_bit(StyleType::FlexWrap as usize)
		.set_bit(StyleType::AlignItems as usize)
		.set_bit(StyleType::JustifyContent as usize)
		.set_bit(StyleType::AlignContent as usize)
        .set_bit(StyleType::TextContent as usize)
        .set_bit(StyleType::FontStyle as usize)
        .set_bit(StyleType::FontSize as usize)
        .set_bit(StyleType::FontFamily as usize)
        .set_bit(StyleType::LetterSpacing as usize)
        .set_bit(StyleType::WordSpacing as usize)
        .set_bit(StyleType::LineHeight as usize)
        .set_bit(StyleType::TextIndent as usize)
        .set_bit(StyleType::WhiteSpace as usize)
        .set_bit(StyleType::TextAlign as usize)
        .set_bit(StyleType::VerticalAlign as usize);


	pub static ref DIRTY2: StyleMarkType = style_bit()
		.set_bit(StyleType::Display as usize)
		.set_bit(StyleType::FlexBasis as usize)
		.set_bit(StyleType::FlexDirection as usize)
		.set_bit(StyleType::FlexWrap as usize)
		.set_bit(StyleType::AlignItems as usize)
		.set_bit(StyleType::JustifyContent as usize)
		.set_bit(StyleType::AlignContent as usize) | &*RECT_DIRTY | &*NORMAL_DIRTY | &*SELF_DIRTY;

    // 布局脏
	pub static ref LAYOUT_DIRTY: StyleMarkType = RECT_DIRTY.clone() | &*NORMAL_DIRTY | &*SELF_DIRTY | &*CHILD_DIRTY | &*DIRTY2;

    // 世界矩阵脏
	pub static ref MATRIX_DIRTY: StyleMarkType = LAYOUT_DIRTY.clone()
        .set_bit(StyleType::Transform as usize)
        .set_bit(StyleType::TransformOrigin as usize);

    // 内容区域脏
	pub static ref CONTENT_BOX_DIRTY: StyleMarkType = MATRIX_DIRTY.clone()
        .set_bit(StyleType::TextShadow as usize)
        .set_bit(StyleType::BoxShadow as usize);

    // 内容区域脏
	pub static ref SHOW_DIRTY: StyleMarkType = style_bit()
        .set_bit(StyleType::Display as usize)
        .set_bit(StyleType::Visibility as usize)
        .set_bit(StyleType::Enable as usize);
}

// 是否存在动画， 用于优化渲染
#[derive(Debug, Serialize, Deserialize, Component, Default, Clone)]
pub struct HasAnimation {
    pub child_count_width_animation: usize, // 包含动画的直接子节点的数量（子节点自身或递归子节点有动画，都算作包含动画）+ 自身数量（自身有动画为1，否则为0）
    pub old_set_parent: Entity, // 曾经为该父节点贡献过有动画的子节点数量（父节点的child_count_width_animation， 当前节点为其贡献1）
    pub old_has_animation: bool, // 自身在前一次渲染中是否有动画
}
