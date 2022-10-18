use std::hash::Hash;
/// 中间计算的组件
use std::{
    intrinsics::transmute,
    ops::{Deref, DerefMut, Index, IndexMut, Mul},
};

use bitvec::prelude::BitArray;
use nalgebra::Matrix4;
use ordered_float::NotNan;
use pi_assets::asset::Handle;
use pi_ecs::prelude::{Id, LocalVersion};
use pi_ecs_macros::Component;
use pi_map::Map;
use pi_render::rhi::asset::TextureRes;
use pi_share::Share;
use pi_spatialtree::QuadTree as QuadTree1;
use smallvec::SmallVec;

use super::{draw_obj::DrawObject, pass_2d::Pass2D, user::*};
use pi_flex_layout::prelude::*;

pub use super::user::{NodeState, StyleType};

/// 布局结果
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

/// 内容最大包围盒范围(所有递归子节点的包围盒的最大范围，不包含自身)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentBox(pub Aabb2);

impl Default for ContentBox {
    fn default() -> Self { Self(Aabb2::new(Point2::new(f32::MAX, f32::MAX), Point2::new(f32::MIN, f32::MIN))) }
}

// ZIndex计算结果， 按照节点的ZIndex分配的一个全局唯一的深度表示
#[derive(Default, Deref, DerefMut, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ZRange(pub std::ops::Range<usize>);

/// 渲染顺序
#[derive(Default, Deref, DerefMut, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy)]
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

    pub fn set_is_opacity(&mut self, value: bool) { self.0 = self.0 << 1 >> 1 | ((unsafe { transmute::<_, u8>(value) } as u32) << 31); }
}

// 世界矩阵，  WorldMatrix(矩阵, 矩阵描述的变换是存在旋转变换)， 如果不存在旋转变换， 可以简化矩阵的乘法
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        if self.1 {
            let r = &*self * WorldMatrix(Matrix4::new_translation(&Vector3::new(x, y, z)), false);
            *self = r;
        } else {
            let slice = self.0.as_mut_slice();
            slice[12] += slice[0] * x;
            slice[13] += slice[5] * y;
            slice[14] += slice[10] * z;
        }
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

    pub fn form_transform_layout(
        transform_funcs: &TransformFuncs,
        origin: &TransformOrigin,
        width: f32,
        height: f32,
        left_top: &Point2,
    ) -> WorldMatrix {
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

        // 计算tranform
        for func in transform_funcs.iter() {
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
            TransformFunc::TranslateX(x) => WorldMatrix(Matrix4::new_translation(&Vector3::new(*x, 0.0, 0.0)), false),
            TransformFunc::TranslateY(y) => WorldMatrix(Matrix4::new_translation(&Vector3::new(0.0, *y, 0.0)), false),
            TransformFunc::Translate(x, y) => WorldMatrix(Matrix4::new_translation(&Vector3::new(*x, *y, 0.0)), false),

            TransformFunc::TranslateXPercent(x) => WorldMatrix(Matrix4::new_translation(&Vector3::new(*x * width, 0.0, 0.0)), false),
            TransformFunc::TranslateYPercent(y) => WorldMatrix(Matrix4::new_translation(&Vector3::new(0.0, *y * height, 0.0)), false),
            TransformFunc::TranslatePercent(x, y) => WorldMatrix(Matrix4::new_translation(&Vector3::new(*x * width, *y * height, 0.0)), false),

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
#[derive(Clone, Debug, Component, Serialize, Deserialize)]
#[storage(QuadTree)]
pub struct Quad(Aabb2, ());

impl Quad {
    pub fn new(aabb: Aabb2) -> Quad { Quad(aabb, ()) }
}

impl Default for Quad {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)), ()) }
}

impl Deref for Quad {
    type Target = Aabb2;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Quad {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Deref, DerefMut)]
pub struct QuadTree(QuadTree1<LocalVersion, f32, ()>);

impl Map for QuadTree {
    type Key = LocalVersion;
    type Val = Quad;
    fn len(&self) -> usize { self.0.len() }
    fn with_capacity(_capacity: usize) -> Self {
        let max = Vector2::new(100f32, 100f32);
        let min = max / 100f32;
        Self(QuadTree1::new(
            Aabb2::new(Point2::new(-1024f32, -1024f32), Point2::new(4096f32, 4096f32)),
            max,
            min,
            0,
            0,
            16, //????
        ))
    }
    fn capacity(&self) -> usize { 0 }
    fn mem_size(&self) -> usize { 0 }
    fn contains(&self, key: &Self::Key) -> bool { self.0.contains_key(key.clone()) }
    fn get(&self, key: &Self::Key) -> Option<&Self::Val> { unsafe { transmute(self.0.get(key.clone())) } }
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val> { unsafe { transmute(self.0.get_mut(key.clone())) } }
    unsafe fn get_unchecked(&self, key: &Self::Key) -> &Self::Val { transmute(self.0.get_unchecked(key.clone())) }
    unsafe fn get_unchecked_mut(&mut self, key: &Self::Key) -> &mut Self::Val { transmute(self.0.get_unchecked_mut(key.clone())) }
    unsafe fn remove_unchecked(&mut self, key: &Self::Key) -> Self::Val { transmute(self.0.remove(key.clone()).unwrap()) }
    fn insert(&mut self, key: Self::Key, val: Self::Val) -> Option<Self::Val> {
		if self.0.contains_key(key) {
            self.0.update(key, val.0);
        } else {
            self.0.add(key, val.0, val.1);
        }
        return None;
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Val> { unsafe { transmute(self.0.remove(key.clone())) } }
}

impl Index<LocalVersion> for QuadTree {
    type Output = Quad;

    fn index(&self, index: LocalVersion) -> &Self::Output { unsafe { self.get_unchecked(&index) } }
}

impl IndexMut<LocalVersion> for QuadTree {
    fn index_mut(&mut self, index: LocalVersion) -> &mut Self::Output { unsafe { self.get_unchecked_mut(&index) } }
}

//是否可见,
#[derive(Deref, DerefMut, Clone, Debug)]
pub struct Visibility(pub bool);

impl Default for Visibility {
    fn default() -> Self { Self(true) }
}

//是否响应事件
#[derive(Deref, DerefMut, Clone, Debug)]
pub struct IsEnable(pub bool);

impl Default for IsEnable {
    fn default() -> Self { Self(true) }
}

// 是否被裁剪
#[derive(Clone, Debug, Default)]
pub struct Culling(pub bool);

// gui支持最多32个裁剪面， 该值按位表示节点被哪些裁剪面裁剪， 等于0时， 表示不被任何裁剪面裁剪， 等于1时， 被第一个裁剪面裁剪， 等于2时，表示被第二个裁剪面裁剪， 等于3表示被第一个和第二个裁剪面共同裁剪。。。。。
#[derive(Clone, Default, Deref, DerefMut, Debug)]
pub struct ByOverflow(pub usize);

// 样式标记
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StyleMark {
    pub local_style: BitArray<[u32; 3]>, // 本地样式， 表示节点样式中，哪些样式是由style设置的（而非class设置）
    pub class_style: BitArray<[u32; 3]>, // class样式， 表示节点样式中，哪些样式是由class设置的
}

/// 标记渲染context中需要的效果， 如Blur、Opacity、Hsi、MasImage等
/// 此数据结构仅记录位标记，具体哪些属性用哪一位来标记，这里并不关心，由逻辑保证
#[derive(Clone, Debug, Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct RenderContextMark(bitvec::prelude::BitArray);

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

// #[derive(Deref, DerefMut, Clone, Debug, Serialize, Deserialize)]
// pub struct TextChars(Vec<CharNode>);

// TransformWillChange的矩阵计算结果， 用于优化Transform的频繁改变
#[derive(Debug, Clone, Default, Deref)]
pub struct TransformWillChangeMatrix(Share<TransformWillChangeMatrixInner>);

impl TransformWillChangeMatrix {
    pub fn new(will_change_invert: WorldMatrix, will_change: WorldMatrix, primitive: WorldMatrix) -> TransformWillChangeMatrix {
        TransformWillChangeMatrix(Share::new(TransformWillChangeMatrixInner {
            will_change_invert,
            will_change,
            primitive,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransformWillChangeMatrixInner {
    pub will_change: WorldMatrix,        // = ParentWorldMatrix * primitive * ParentWorldMatrix逆
    pub will_change_invert: WorldMatrix, // will_change 逆
    pub primitive: WorldMatrix,          // = Parent1.primitive * Parent2.primitive * ... * Transform
}

#[derive(Debug, Clone, Default)]
pub struct MaskTexture;

/// 上下文的实体ID，作为Node的组件，关联由其创建的渲染上下文
#[derive(Deref, DerefMut, Default, Debug)]
pub struct Pass2DId(pub Id<Pass2D>);

/// 作为Node的组件，表示节点所在的渲染上下文的实体
#[derive(Clone, Copy, Deref, DerefMut, Default, PartialEq, Eq, Debug)]
pub struct InPassId(pub Id<Pass2D>);


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
#[derive(Deref, DerefMut, Default, Debug)]
pub struct NodeId(pub Id<Node>);

/// 宏标记(最多支持size::of::<usize>()个宏开关)
pub struct DefineMark(bitvec::prelude::BitArray);

/// 每节点的渲染列表
#[derive(Deref, DerefMut, Default, Debug)]
pub struct DrawList(pub SmallVec<[Id<DrawObject>; 1]>);

/// 裁剪框
/// 非旋转情况下，由世界矩阵、布局、TarnsformWillChange（包含父）计算、并与父的裁剪框相交而得
/// 旋转情况下，由世界矩阵、布局、TarnsformWillChange（自身）计算，并逆旋转为矩形而得
#[derive(Clone, Default, Debug)]
pub struct OverflowAabb {
    pub aabb: Option<Aabb2>,
    pub matrix: Option<OveflowRotate>,
}

#[derive(Clone, Default, Debug)]
pub struct OveflowRotate {
    pub rotate_matrix: Matrix4<f32>,
    pub rotate_matrix_invert: Matrix4<f32>,
}

#[derive(Deref, DerefMut)]
pub struct BorderImageTexture(pub Handle<TextureRes>);

impl From<Handle<TextureRes>> for BorderImageTexture {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}

#[derive(Deref, DerefMut)]
pub struct BackgroundImageTexture(pub Handle<TextureRes>);

impl From<Handle<TextureRes>> for BackgroundImageTexture {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}