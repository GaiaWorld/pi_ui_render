//! 定义计算组件（非用户设置的组件）

use bevy::ecs::prelude::{Component, Entity};
use pi_style::style::AllTransform;
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
use pi_render::rhi::asset::TextureRes;
use pi_share::Share;
use pi_slotmap::Key;

use crate::resource::RenderObjType;

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

/// 内容最大包围盒范围(所有递归子节点的包围盒的最大范围，不包含自身)
#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct ContentBox {
    // 内容包围盒(不包含阴影的扩展)
    pub oct: Aabb2,
    // 布局包围盒（还包含了阴影的扩展）
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
#[derive(Default, Deref, Clone, PartialEq, Eq, Hash, Debug, Component, Serialize, Deserialize)]
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
        for func in all_transform.transform.iter().rev() {
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
#[derive(Clone, Debug, Component, Serialize, Deserialize)]
// #[storage(QuadTree)]
pub struct Quad(pub Aabb2);

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

#[derive(Debug, Component, Clone, Serialize, Deserialize)]
pub struct IsShow(usize);

impl Default for IsShow {
    fn default() -> IsShow { IsShow(ShowType::Visibility as usize | ShowType::Enable as usize) }
}

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

// 样式标记
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Component)]
pub struct StyleMark {
    pub local_style: BitArray<[u32; 3]>, // 本地样式， 表示节点样式中，哪些样式是由style设置的（而非class设置）
    pub class_style: BitArray<[u32; 3]>, // class样式， 表示节点样式中，哪些样式是由class设置的
}

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
            primitive,
        })))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransformWillChangeMatrixInner {
    pub will_change: WorldMatrix, // 节点真实的世界矩阵（即will_change*worldmatrix*顶点=真实的世界坐标位置） = ParentWorldMatrix * primitive * ParentWorldMatrix逆
    pub will_change_invert: WorldMatrix, // will_change 逆
    pub primitive: WorldMatrix,   // = Parent1.WillChangeTransform * Parent2.WillChangeTransform * ... * this.WillChangeTransform
}

#[derive(Debug, Clone, Default, Component)]
pub struct MaskTexture(pub Option<Handle<TextureRes>>);

impl Null for MaskTexture {
    fn null() -> Self { Self(None) }

    fn is_null(&self) -> bool { self.0.is_none() }
}

impl From<Handle<TextureRes>> for MaskTexture {
    fn from(handle: Handle<TextureRes>) -> Self { MaskTexture(Some(handle)) }
}

impl From<Option<Handle<TextureRes>>> for MaskTexture {
    fn from(handle: Option<Handle<TextureRes>>) -> Self { MaskTexture(handle) }
}

impl From<MaskTexture> for Option<Handle<TextureRes>> {
    fn from(mask_texture: MaskTexture) -> Self { mask_texture.0 }
}

#[derive(Deref, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct EntityKey(pub Entity);

unsafe impl Key for EntityKey {
    fn data(&self) -> pi_slotmap::KeyData {
        // (u64::from(self.version.get()) << 32) | u64::from(self.idx)

        pi_slotmap::KeyData::from_ffi((u64::from(self.0.generation()) << 32) | u64::from(self.0.index()))
    }
}

impl From<pi_slotmap::KeyData> for EntityKey {
    fn from(value: pi_slotmap::KeyData) -> Self { Self(Entity::from_bits(value.as_ffi())) }
}

impl Default for EntityKey {
    fn default() -> Self { Self(Entity::from_bits(u64::null())) }
}

impl Null for EntityKey {
    fn null() -> Self { Self(Entity::from_bits(u64::null())) }

    fn is_null(&self) -> bool { self.0.to_bits().is_null() }
}

// /// 上下文的实体ID，作为Node的组件，关联由其创建的渲染上下文
// #[derive(Deref, Default, Debug, Hash, Clone, Copy, Component)]
// pub struct Pass2DId(pub EntityKey);

/// 作为Node的组件，表示节点所在的渲染上下文的实体
#[derive(Clone, Copy, Deref, Default, PartialEq, Eq, Debug, Hash, Component, Serialize, Deserialize)]
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

/// 宏标记(最多支持size::of::<usize>()个宏开关)
pub struct DefineMark(bitvec::prelude::BitArray);

/// 每节点的渲染列表
#[derive(Deref, Default, Debug, Component, Clone, Serialize, Deserialize)]
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
}

/// 节点上握住DrawObj的id
#[derive(Debug, Component, Clone, Serialize, Deserialize)]
pub struct DrawObjId {
    pub ty: RenderObjType,
    pub id: Entity,
}

/// 视图
/// 每个Pass2d都必须存在一个视图
#[derive(Clone, Default, Debug, Component)]
pub struct View {
    /// 为some时，节点山下文渲染需要新的视口，否则应该继承父节点的视口
    pub view_box: ViewBox,
    /// 旋转情况下是Some， 记录旋转矩阵和旋转逆矩阵
    pub desc: OverflowDesc,
}

/// 可视包围盒
/// 已经考虑了Overflow、TransformWillChange因素，得到了该节点的真实可视区域
#[derive(Clone, Debug)]
pub struct ViewBox {
    /// 当前节点的可视包围盒
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

#[derive(Clone, Default, Debug)]
pub struct OveflowRotate {
    // 相对于父上下文的旋转
    pub from_context_rotate: Matrix4<f32>,
    // 节点相对于世界坐标的渲染
    pub world_rotate: Matrix4<f32>,
    // 节点相对于世界坐标的旋转的逆
    pub world_rotate_invert: Matrix4<f32>,
}

// 描述oveflow
#[derive(Clone, Debug)]
pub enum OverflowDesc {
    Rotate(OveflowRotate), // 所在节点存在旋转的情况下， 描述旋转信息
    NoRotate(Aabb2),       // 所在节点不存在旋转的情况下，描述自身的aabb
}

impl Default for OverflowDesc {
    fn default() -> Self { OverflowDesc::NoRotate(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0))) }
}

/// BorderImageTexture.0只有在设置了图片路径，但纹理还未加载成功的情况下，才会为none
/// 如果删除了图片路径，会删除该组件
#[derive(Deref, Component, Default)]
pub struct BorderImageTexture(pub Option<Handle<TextureRes>>);

impl From<Handle<TextureRes>> for BorderImageTexture {
    fn from(h: Handle<TextureRes>) -> Self { Self(Some(h)) }
}

impl Null for BorderImageTexture {
    fn null() -> Self { Self(None) }

    fn is_null(&self) -> bool { self.0.is_none() }
}

/// BackgroundImageTexture.0只有在设置了图片路径，但纹理还未加载成功的情况下，才会为none
/// 如果删除了图片路径，会删除该组件
#[derive(Deref, Component, Default)]
pub struct BackgroundImageTexture(pub Option<Handle<TextureRes>>);

impl From<Handle<TextureRes>> for BackgroundImageTexture {
    fn from(h: Handle<TextureRes>) -> Self { Self(Some(h)) }
}


impl Null for BackgroundImageTexture {
    fn null() -> Self { Self(None) }

    fn is_null(&self) -> bool { self.0.is_none() }
}
