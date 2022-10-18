//! * 定义样式类型
//! * 为所有的样式类型实现Attr这个tarit
//! * 为所有的样式类型实现Add和Scale trait，用于动画插值

use std::mem::forget;

use bitvec::array::BitArray;
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_flex_layout::{
    prelude::{Number, Rect},
    style::{
        AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType as PositionType1,
    },
};
use pi_hash::XHashMap;
use smallvec::SmallVec;

use crate::style::{
    Aabb2, AnimationDirection, AnimationFillMode, AnimationPlayState, AnimationTimingFunction, BlendMode, BorderImageSlice, BorderRadius, BoxShadow,
    CgColor, Color, Enable, FitType, FontSize, FontStyle, Hsi, ImageRepeat, IterationCount, LengthUnit, LineHeight, MaskImage, NotNanRect, Point2,
    Stroke, StyleType, TextAlign, TextContent, TextShadow, Time, TransformFunc, TransformFuncs, TransformOrigin, VerticalAlign, WhiteSpace, AnimationName,
};
use pi_curves::curve::frame::{FrameValueScale, KeyFrameCurveValue};
use std::ops::Add;

pub trait Attr: 'static + Sync + Send {
    /// 获取样式属性类型
    fn get_type() -> StyleType
    where
        Self: Sized;
    /// 获取样式属性索引（对应StyleAttrs的索引）
    fn get_style_index() -> u8
    where
        Self: Sized;
    /// 样式属性的牛内存大小
    fn size() -> usize
    where
        Self: Sized;
    /// 序列化自身到buffer中
    unsafe fn write(&self, buffer: &mut Vec<u8>);
}

// use pi_print_any::{println_any, out_any};

// 全局Class样式表
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClassSheet {
    pub style_buffer: Vec<u8>,                 // 所有class样式的buffer集合
    pub class_map: XHashMap<usize, ClassMeta>, // 每个class的元信息描述
}

impl ClassSheet {
    /// 从另一个ClassSheet扩充
    pub fn extend_from_class_sheet(&mut self, class_sheet: ClassSheet) {
        let old_len = self.style_buffer.len();
        self.style_buffer.extend_from_slice(class_sheet.style_buffer.as_slice());
        for (i, mut meta) in class_sheet.class_map.into_iter() {
            meta.start += old_len;
            meta.end += old_len;
            self.class_map.insert(i, meta);
        }
    }
}

/// class样式
/// 该类型单独存在没有意义，它与ClassSheet结合起来使用，用于描述该class的有效属性类型以及属性在classSheet中的位置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClassMeta {
    pub start: usize,                         // 在某个buffer中的开始偏移
    pub end: usize,                           // 在某个buffer中的结束偏移
    pub class_style_mark: BitArray<[u32; 3]>, // 标记class中的有效属性
}

macro_rules! get_type {
    ($key: expr) => {
        #[inline]
        fn get_type() -> StyleType { $key }
    };
}

macro_rules! size {
    ($value_ty: ty) => {
        #[inline]
        fn size() -> usize { std::mem::size_of::<$value_ty>() }
    };
}

macro_rules! write_buffer {
    () => {
        unsafe fn write(&self, buffer: &mut Vec<u8>) {
            let ty_size = std::mem::size_of::<StyleType>();
            let value_size = Self::size();
            let len = buffer.len();
            buffer.reserve(ty_size + value_size);
            buffer.set_len(len + ty_size + value_size);

            let ty = Self::get_style_index();
            // pi_print_any::out_any!(
            //     log::trace,
            //     "write, value: {:?}, start: {:?}, end: {:?}, ty: {:?}",
            //     self,
            //     len,
            //     len + ty_size + value_size,
            //     ty
            // );


            std::ptr::copy_nonoverlapping(&ty as *const u8, buffer.as_mut_ptr().add(len), ty_size);

            std::ptr::copy_nonoverlapping(
                self as *const Self as usize as *const u8,
                buffer.as_mut_ptr().add(len + ty_size),
                value_size,
            );
            forget(self)
        }
    };
}

macro_rules! impl_style {
    ($struct_name: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct $struct_name;

        impl Attr for $struct_name {
            fn get_style_index() -> u8 { 0 }
            get_type!(StyleType::PaddingBottom);
            fn size() -> usize { 0 }
            write_buffer!();
        }
    };
    ($struct_name: ident, $name: ident, $ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $ty);

        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    ($struct_name: ident, $name: ident, $ty: ident, $value_ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    ($struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ty) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    ($struct_name: ident, $name: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    (@func $struct_name: ident,  $name: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    // 方法设置，并且不实现set_default和reset
    (@func $struct_name: ident,  $name: ident, $set_func: ident, $ty: ident, $value_ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };

    (@box_model_single $struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $value_ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($value_ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
    (@box_model $struct_name: ident, $name: ident, $ty: ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
        pub struct $struct_name(pub $ty);
        impl Attr for $struct_name {
            fn get_style_index() -> u8 { Self::get_type() as u8 }
            get_type!(StyleType::$ty);
            size!($ty);
            write_buffer!();
        }

        $crate::paste::item! {
            #[derive(Debug, Clone)]
            pub struct[<Reset $struct_name>];

            impl Attr for [<Reset $struct_name>] {
                fn get_style_index() -> u8 {
                    Self::get_type() as u8 + 86
                }
                fn size() -> usize {
                    0
                }
                get_type!(StyleType::$ty);
                write_buffer!();
            }
        }
    };
}


impl_style!(EmptyType);
impl_style!(FontStyleType, text_style, font_style, FontStyle, FontStyle);

impl_style!(FontWeightType, text_style, font_weight, FontWeight, usize);
impl_style!(FontSizeType, text_style, font_size, FontSize, FontSize);
impl_style!(FontFamilyType, text_style, font_family, FontFamily, Atom);
impl_style!(LetterSpacingType, text_style, letter_spacing, LetterSpacing, f32);
impl_style!(WordSpacingType, text_style, word_spacing, WordSpacing, f32);
impl_style!(LineHeightType, text_style, line_height, LineHeight, LineHeight);
impl_style!(TextIndentType, text_style, text_indent, TextIndent, f32);
impl_style!(WhiteSpaceType, text_style, white_space, WhiteSpace, WhiteSpace);

impl_style!(TextContentType, text_content, TextContent);
impl_style!(TextAlignType, text_style, text_align, TextAlign, TextAlign);
impl_style!(VerticalAlignType, text_style, vertical_align, VerticalAlign, VerticalAlign);
impl_style!(ColorType, text_style, color, Color, Color);
impl_style!(TextStrokeType, text_style, text_stroke, TextStroke, Stroke);
impl_style!(TextShadowType, text_style, text_shadow, TextShadow, SmallVec<[TextShadow; 1]>);

impl_style!(BackgroundImageType, background_image, BackgroundImage, Atom);
impl_style!(BackgroundImageClipType, background_image_clip, BackgroundImageClip, NotNanRect);
impl_style!(ObjectFitType, background_image_mod, object_fit, ObjectFit, FitType);
impl_style!(BackgroundRepeatType, background_image_mod, repeat, BackgroundRepeat, ImageRepeat);

impl_style!(BorderImageType, border_image, BorderImage, Atom);
impl_style!(BorderImageClipType, border_image_clip, BorderImageClip, NotNanRect);
impl_style!(BorderImageSliceType, border_image_slice, BorderImageSlice);
impl_style!(BorderImageRepeatType, border_image_repeat, BorderImageRepeat, ImageRepeat);

impl_style!(BorderColorType, border_color, BorderColor, CgColor);

impl_style!(BackgroundColorType, background_color, BackgroundColor, Color);

impl_style!(BoxShadowType, box_shadow, BoxShadow);

impl_style!(OpacityType, opacity, Opacity, f32);
impl_style!(BorderRadiusType, border_radius, BorderRadius);
impl_style!(HsiType, hsi, Hsi);
impl_style!(BlurType, blur, Blur, f32);
impl_style!(TransformOriginType, transform, origin, TransformOrigin, TransformOrigin);
impl_style!(TransformType, transform, funcs, Transform, TransformFuncs);
impl_style!(DirectionType, flex_container, direction, Direction, Direction);
impl_style!(AspectRatioType, flex_normal, aspect_ratio, AspectRatio, Number);
impl_style!(OrderType, flex_normal, order, Order, isize);
impl_style!(FlexBasisType, flex_normal, flex_basis, FlexBasis, Dimension);


impl_style!(@func DisplayType, show, set_display, get_display, Display, Display);
impl_style!(@func VisibilityType, show, set_visibility, get_visibility, Visibility, bool);
impl_style!(@func EnableType, show, set_enable, get_enable, Enable, Enable);

impl_style!(@func TransformFuncType, transform, add_func, TransformFunc, TransformFunc);
impl_style!(@func VNodeType, node_state, set_vnode, NodeState, bool);

impl_style!(TransformWillChangeType, transform_will_change, TransformWillChange, TransformFuncs);

impl_style!(ZIndexType, z_index, ZIndex, isize);
impl_style!(OverflowType, overflow, Overflow, bool);

impl_style!(MaskImageType, mask_image, MaskImage);
impl_style!(MaskImageClipType, mask_image_clip, MaskImageClip, NotNanRect);

impl_style!(WidthType, size, width, Width, Dimension);
impl_style!(HeightType, size, height, Height, Dimension);


impl_style!(@box_model_single MarginTopType, margin, top, MarginTop, Dimension);
impl_style!(@box_model_single MarginRightType, margin, right, MarginRight, Dimension);
impl_style!(@box_model_single MarginBottomType, margin, bottom, MarginBottom, Dimension);
impl_style!(@box_model_single MarginLeftType, margin, left, MarginLeft, Dimension);

impl_style!(@box_model_single PaddingTopType, padding, top, PaddingTop, Dimension);
impl_style!(@box_model_single PaddingRightType, padding, right, PaddingRight, Dimension);
impl_style!(@box_model_single PaddingBottomType, padding, bottom, PaddingBottom, Dimension);
impl_style!(@box_model_single PaddingLeftType, padding, left, PaddingLeft, Dimension);

impl_style!(@box_model_single BorderTopType, border, top, BorderTop, Dimension);
impl_style!(@box_model_single BorderRightType, border, right, BorderRight, Dimension);
impl_style!(@box_model_single BorderBottomType, border, bottom, BorderBottom, Dimension);
impl_style!(@box_model_single BorderLeftType, border, left, BorderLeft, Dimension);

impl_style!(@box_model_single PositionTopType, position, top, PositionTop, Dimension);
impl_style!(@box_model_single PositionRightType, position, right, PositionRight, Dimension);
impl_style!(@box_model_single PositionBottomType, position, bottom, PositionBottom, Dimension);
impl_style!(@box_model_single PositionLeftType, position, left, PositionLeft, Dimension);

impl_style!(MinWidthType, min_max, min, width, MinWidth, Dimension);
impl_style!(MinHeightType, min_max, min, height, MinHeight, Dimension);
impl_style!(MaxHeightType, min_max, max, height, MaxHeight, Dimension);
impl_style!(MaxWidthType, min_max, max, width, MaxWidth, Dimension);
impl_style!(JustifyContentType, flex_container, justify_content, JustifyContent, JustifyContent);
impl_style!(FlexDirectionType, flex_container, flex_direction, FlexDirection, FlexDirection);
impl_style!(AlignContentType, flex_container, align_content, AlignContent, AlignContent);
impl_style!(AlignItemsType, flex_container, align_items, AlignItems, AlignItems);
impl_style!(FlexWrapType, flex_container, flex_wrap, FlexWrap, FlexWrap);

impl_style!(FlexShrinkType, flex_normal, flex_shrink, FlexShrink, f32);
impl_style!(FlexGrowType, flex_normal, flex_grow, FlexGrow, f32);
impl_style!(PositionTypeType, flex_normal, position_type, PositionType, PositionType1);
impl_style!(AlignSelfType, flex_normal, align_self, AlignSelf, AlignSelf);

impl_style!(BlendModeType, blend_mode, BlendMode);
impl_style!(AnimationNameType, animation, name, AnimationName, AnimationName);
impl_style!(AnimationDurationType, animation, duration, AnimationDuration, SmallVec<[Time; 1]>);
impl_style!(
    AnimationTimingFunctionType,
    animation,
    timing_function,
    AnimationTimingFunction,
    SmallVec<[AnimationTimingFunction; 1]>
);
impl_style!(AnimationDelayType, animation, delay, AnimationDelay, SmallVec<[Time; 1]>);
impl_style!(
    AnimationIterationCountType,
    animation,
    iteration_count,
    AnimationIterationCount,
    SmallVec<[IterationCount; 1]>
);
impl_style!(
    AnimationDirectionType,
    animation,
    direction,
    AnimationDirection,
    SmallVec<[AnimationDirection; 1]>
);
impl_style!(
    AnimationFillModeType,
    animation,
    fill_mode,
    AnimationFillMode,
    SmallVec<[AnimationFillMode; 1]>
);
impl_style!(
    AnimationPlayStateType,
    animation,
    play_state,
    AnimationPlayState,
    SmallVec<[AnimationPlayState; 1]>
);


macro_rules! impl_interpolation {
    (@keep, $ty: ident) => {
        impl Add for $ty {
            type Output = Self;
            fn add(self, _rhs: Self) -> Self::Output { self }
        }

        impl FrameValueScale for $ty {
            fn scale(&self, _rhs: KeyFrameCurveValue) -> Self { self.clone() }
        }
    };
    // 为数字实现
    (@number, $ty: ident, $inner: ident) => {
        impl Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output { $ty(self.0 + rhs.0) }
        }

        impl FrameValueScale for $ty {
            fn scale(&self, rhs: KeyFrameCurveValue) -> Self { Self((self.0 as f32 * rhs).round() as $inner) }
        }
    };

    (@animatable_value, $ty: ident) => {
        impl Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output { $ty(self.0.add(&rhs.0)) }
        }

        impl FrameValueScale for $ty {
            fn scale(&self, rhs: KeyFrameCurveValue) -> Self { Self(AnimatableValue::scale(&self.0, rhs)) }
        }
    };

    (@animatable_value_next, $ty: ident, $inner: ident) => {
        impl Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output { Self($inner(self.0 .0.add(&rhs.0 .0))) }
        }

        impl FrameValueScale for $ty {
            fn scale(&self, rhs: KeyFrameCurveValue) -> Self { Self($inner(AnimatableValue::scale(&self.0 .0, rhs))) }
        }
    };
}

impl_interpolation!(@keep, FontStyleType);

impl_interpolation!(@number, FontWeightType, usize);
impl_interpolation!(@animatable_value, FontSizeType);
impl_interpolation!(@keep, FontFamilyType);
impl_interpolation!(@number, LetterSpacingType, f32);
impl_interpolation!(@number, WordSpacingType, f32);
impl_interpolation!(@animatable_value, LineHeightType);
impl_interpolation!(@number, TextIndentType, f32);
impl_interpolation!(@keep, WhiteSpaceType);

impl_interpolation!(@keep, TextContentType);
impl_interpolation!(@keep, TextAlignType);
impl_interpolation!(@keep, VerticalAlignType);
impl_interpolation!(@animatable_value, ColorType);
impl_interpolation!(@animatable_value, TextStrokeType);
impl_interpolation!(@keep, TextShadowType);

impl_interpolation!(@keep, BackgroundImageType);
impl_interpolation!(@keep, BackgroundImageClipType);
impl_interpolation!(@keep, ObjectFitType);
impl_interpolation!(@keep, BackgroundRepeatType);

impl_interpolation!(@keep, BorderImageType);
impl_interpolation!(@keep, BorderImageClipType);
impl_interpolation!(@keep, BorderImageSliceType);
impl_interpolation!(@keep, BorderImageRepeatType);

impl_interpolation!(@animatable_value, BorderColorType);

impl_interpolation!(@animatable_value, BackgroundColorType);

impl_interpolation!(@animatable_value, BoxShadowType);

impl_interpolation!(@animatable_value, OpacityType);
impl_interpolation!(@animatable_value, BorderRadiusType);
impl_interpolation!(@animatable_value, HsiType);
impl_interpolation!(@animatable_value, BlurType);
impl_interpolation!(@animatable_value, TransformOriginType);
impl_interpolation!(@animatable_value, TransformType);
impl_interpolation!(@keep, DirectionType);
impl_interpolation!(@animatable_value, AspectRatioType);
impl_interpolation!(@number, OrderType, isize);
impl_interpolation!(@animatable_value, FlexBasisType);

impl_interpolation!(@keep, DisplayType);
impl_interpolation!(@keep, VisibilityType);
impl_interpolation!(@keep, EnableType);

impl_interpolation!(@keep, TransformFuncType);
impl_interpolation!(@keep, VNodeType);

impl_interpolation!(@keep, TransformWillChangeType);

impl_interpolation!(@animatable_value, ZIndexType);
impl_interpolation!(@keep, OverflowType);

impl_interpolation!(@keep, MaskImageType);
impl_interpolation!(@keep, MaskImageClipType);

impl_interpolation!(@animatable_value, WidthType);
impl_interpolation!(@animatable_value, HeightType);

impl_interpolation!(@animatable_value,  MarginTopType);
impl_interpolation!(@animatable_value,  MarginRightType);
impl_interpolation!(@animatable_value,  MarginBottomType);
impl_interpolation!(@animatable_value,  MarginLeftType);

impl_interpolation!(@animatable_value,  PaddingTopType);
impl_interpolation!(@animatable_value,  PaddingRightType);
impl_interpolation!(@animatable_value,  PaddingBottomType);
impl_interpolation!(@animatable_value,  PaddingLeftType);

impl_interpolation!(@animatable_value,  BorderTopType);
impl_interpolation!(@animatable_value,  BorderRightType);
impl_interpolation!(@animatable_value,  BorderBottomType);
impl_interpolation!(@animatable_value,  BorderLeftType);

impl_interpolation!(@animatable_value,  PositionTopType);
impl_interpolation!(@animatable_value,  PositionRightType);
impl_interpolation!(@animatable_value,  PositionBottomType);
impl_interpolation!(@animatable_value,  PositionLeftType);

impl_interpolation!(@animatable_value, MinWidthType);
impl_interpolation!(@animatable_value, MinHeightType);
impl_interpolation!(@animatable_value, MaxHeightType);
impl_interpolation!(@animatable_value, MaxWidthType);
impl_interpolation!(@keep, JustifyContentType);
impl_interpolation!(@keep, FlexDirectionType);
impl_interpolation!(@keep, AlignContentType);
impl_interpolation!(@keep, AlignItemsType);
impl_interpolation!(@keep, FlexWrapType);

impl_interpolation!(@number, FlexShrinkType, f32);
impl_interpolation!(@number, FlexGrowType, f32);
impl_interpolation!(@keep, PositionTypeType);
impl_interpolation!(@keep, AlignSelfType);

impl_interpolation!(@keep, BlendModeType);

pub trait AnimatableValue {
    fn add(&self, rhs: &Self) -> Self;
    fn scale(&self, other: f32) -> Self;
}

impl AnimatableValue for Dimension {
    fn add(&self, rhs: &Self) -> Self {
        log::trace!("add: {:?}, {:?}", self, rhs);
        match self {
            Dimension::Undefined => Dimension::Undefined,
            Dimension::Auto => Dimension::Auto,
            Dimension::Points(r1) => {
                if let Dimension::Points(r2) = rhs {
                    Dimension::Points(r1 + r2)
                } else {
                    Dimension::Points(*r1)
                }
            }
            Dimension::Percent(r1) => {
                if let Dimension::Percent(r2) = rhs {
                    Dimension::Percent(r1 + r2)
                } else {
                    Dimension::Percent(*r1)
                }
            }
        }
    }
    fn scale(&self, other: f32) -> Self {
        log::trace!("scale, {:?} {}", self, other);
        match self {
            Dimension::Undefined => Dimension::Undefined,
            Dimension::Auto => Dimension::Auto,
            Dimension::Points(r1) => Dimension::Points(r1 * other),
            Dimension::Percent(r1) => Dimension::Percent(r1 * other),
        }
    }
}

impl AnimatableValue for Aabb2 {
    fn add(&self, rhs: &Self) -> Self {
        Aabb2::new(
            Point2::new(self.mins.x + rhs.mins.x, self.mins.y + rhs.mins.y),
            Point2::new(self.maxs.x + rhs.maxs.x, self.maxs.y + rhs.maxs.y),
        )
    }
    fn scale(&self, other: f32) -> Self {
        Aabb2::new(
            Point2::new(self.mins.x * other, self.mins.y * other),
            Point2::new(self.maxs.x * other, self.maxs.y * other),
        )
    }
}

impl AnimatableValue for isize {
    #[inline]
    fn add(&self, rhs: &Self) -> Self { self + rhs }
    #[inline]
    fn scale(&self, other: f32) -> Self { (*self as f32 * other).round() as Self }
}

impl AnimatableValue for f32 {
    #[inline]
    fn add(&self, rhs: &Self) -> Self { self + rhs }
    #[inline]
    fn scale(&self, other: f32) -> Self { (self * other).round() as Self }
}

impl AnimatableValue for Number {
    #[inline]
    fn add(&self, rhs: &Self) -> Self {
        match self {
            Number::Undefined => Number::Undefined,
            Number::Defined(r1) => {
                if let Number::Defined(r2) = rhs {
                    Number::Defined(*r1 + r2)
                } else {
                    Number::Defined(*r1)
                }
            }
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            Number::Undefined => Number::Undefined,
            Number::Defined(r1) => Number::Defined(r1 * other),
        }
    }
}

impl AnimatableValue for CgColor {
    #[inline]
    fn add(&self, rhs: &Self) -> Self { CgColor::new(self.x + rhs.x, self.x + rhs.x, self.x + rhs.x, self.x + rhs.x) }
    #[inline]
    fn scale(&self, other: f32) -> Self { CgColor::new(self.x * other, self.x * other, self.x * other, self.x * other) }
}

impl AnimatableValue for Hsi {
    #[inline]
    fn add(&self, rhs: &Self) -> Self {
        Hsi {
            hue_rotate: self.hue_rotate + rhs.hue_rotate,
            saturate: self.saturate + rhs.saturate,
            bright_ness: self.bright_ness + rhs.bright_ness,
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        Hsi {
            hue_rotate: self.hue_rotate * other,
            saturate: self.saturate * other,
            bright_ness: self.bright_ness * other,
        }
    }
}

impl AnimatableValue for Stroke {
    #[inline]
    fn add(&self, rhs: &Self) -> Self {
        Stroke {
            width: self.width + rhs.width,
            color: self.color.add(&rhs.color),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        Stroke {
            width: self.width * other,
            color: self.color.scale(other),
        }
    }
}

impl AnimatableValue for FontSize {
    fn add(&self, rhs: &Self) -> Self {
        match self {
            FontSize::None => FontSize::None,
            FontSize::Length(r1) => {
                if let FontSize::Length(r2) = rhs {
                    FontSize::Length(r1 + r2)
                } else {
                    FontSize::Length(*r1)
                }
            }
            FontSize::Percent(r1) => {
                if let FontSize::Percent(r2) = rhs {
                    FontSize::Percent(r1 + r2)
                } else {
                    FontSize::Percent(*r1)
                }
            }
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            FontSize::None => FontSize::None,
            FontSize::Length(r1) => FontSize::Length((*r1 as f32 * other).round() as usize),
            FontSize::Percent(r1) => FontSize::Percent(*r1 as f32 * other),
        }
    }
}

impl AnimatableValue for LineHeight {
    fn add(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (LineHeight::Normal, _) => LineHeight::Normal,
            (LineHeight::Length(r1), LineHeight::Length(r2)) => LineHeight::Length(r1 + r2),
            (LineHeight::Length(r1), _) => LineHeight::Length(*r1),
            (LineHeight::Number(r1), LineHeight::Number(r2)) => LineHeight::Number(r1 + r2),
            (LineHeight::Number(r1), _) => LineHeight::Number(*r1),
            (LineHeight::Percent(r1), LineHeight::Percent(r2)) => LineHeight::Percent(r1 + r2),
            (LineHeight::Percent(r1), _) => LineHeight::Percent(*r1),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            LineHeight::Normal => LineHeight::Normal,
            LineHeight::Length(r1) => LineHeight::Length(r1 * other),
            LineHeight::Number(r1) => LineHeight::Length(r1 * other),
            LineHeight::Percent(r1) => LineHeight::Length(r1 * other),
        }
    }
}

impl AnimatableValue for Color {
    fn add(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Color::RGBA(r1), Color::RGBA(r2)) => Color::RGBA(r1.add(r2)),
            (Color::RGBA(r1), Color::LinearGradient(_)) => Color::RGBA(r1.clone()),
            (Color::LinearGradient(r1), Color::RGBA(_)) => Color::LinearGradient(r1.clone()),
            (Color::LinearGradient(r1), Color::LinearGradient(_)) => Color::LinearGradient(r1.clone()),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            Color::RGBA(r1) => Color::RGBA(r1.scale(other)),
            Color::LinearGradient(r) => Color::LinearGradient(r.clone()),
        }
    }
}

impl AnimatableValue for NotNanRect {
    fn add(&self, rhs: &Self) -> Self { NotNanRect::new(self.left + rhs.left, self.right + rhs.right, self.top + rhs.top, self.bottom + rhs.bottom) }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        let other = NotNan::new(other).unwrap();
        NotNanRect::new(self.left * other, self.right * other, self.top * other, self.bottom * other)
    }
}

impl AnimatableValue for Rect<Dimension> {
    fn add(&self, rhs: &Self) -> Self {
        Self {
            left: self.left.add(&rhs.left),
            right: self.right.add(&rhs.right),
            top: self.top.add(&rhs.top),
            bottom: self.bottom.add(&rhs.bottom),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        Self {
            left: self.left.scale(other),
            right: self.right.scale(other),
            top: self.top.scale(other),
            bottom: self.bottom.scale(other),
        }
    }
}

impl AnimatableValue for BorderImageSlice {
    fn add(&self, rhs: &Self) -> Self {
        Self {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
            fill: self.fill,
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        let other = NotNan::new(other).unwrap();
        Self {
            left: self.left * other,
            right: self.right * other,
            top: self.top * other,
            bottom: self.bottom * other,
            fill: self.fill,
        }
    }
}

impl AnimatableValue for BoxShadow {
    fn add(&self, rhs: &Self) -> Self {
        Self {
            h: self.h + rhs.h,
            v: self.v + rhs.v,
            blur: self.blur + rhs.blur,
            spread: self.spread + rhs.spread,
            color: self.color.add(&rhs.color),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        Self {
            h: self.h * other,
            v: self.v * other,
            blur: self.blur * other,
            spread: self.spread * other,
            color: self.color.scale(other),
        }
    }
}

impl AnimatableValue for BorderRadius {
    fn add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.add(&rhs.x),
            y: self.y.add(&rhs.y),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        Self {
            x: self.x.scale(other),
            y: self.y.scale(other),
        }
    }
}

impl AnimatableValue for LengthUnit {
    fn add(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (LengthUnit::Pixel(r1), LengthUnit::Pixel(r2)) => LengthUnit::Pixel(r1 + r2),
            (LengthUnit::Pixel(r1), LengthUnit::Percent(_)) => LengthUnit::Pixel(*r1),
            (LengthUnit::Percent(r1), LengthUnit::Pixel(_)) => LengthUnit::Percent(*r1),
            (LengthUnit::Percent(r1), LengthUnit::Percent(r2)) => LengthUnit::Percent(r1 + r2),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            LengthUnit::Pixel(r1) => LengthUnit::Pixel(r1 * other),
            LengthUnit::Percent(r1) => LengthUnit::Percent(r1 * other),
        }
    }
}

impl AnimatableValue for TransformOrigin {
    fn add(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (TransformOrigin::Center, _) => TransformOrigin::Center,
            (TransformOrigin::XY(x1, y1), TransformOrigin::Center) => TransformOrigin::XY(x1.clone(), y1.clone()),
            (TransformOrigin::XY(x1, y1), TransformOrigin::XY(x2, y2)) => TransformOrigin::XY(x1.add(x2), y1.add(y2)),
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        match self {
            TransformOrigin::Center => TransformOrigin::Center,
            TransformOrigin::XY(x1, y1) => TransformOrigin::XY(x1.scale(other), y1.scale(other)),
        }
    }
}

impl AnimatableValue for TransformFuncs {
    fn add(&self, rhs: &Self) -> Self {
        if self.len() != rhs.len() {
            return self.clone();
        }

        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let (t1, t2) = (&self[i], &rhs[i]);
            match (t1, t2) {
                (TransformFunc::TranslateX(t1), TransformFunc::TranslateX(t2)) => vec.push(TransformFunc::TranslateX(t1 + t2)),
                (TransformFunc::TranslateY(t1), TransformFunc::TranslateY(t2)) => vec.push(TransformFunc::TranslateY(t1 + t2)),
                (TransformFunc::Translate(x1, y1), TransformFunc::Translate(x2, y2)) => vec.push(TransformFunc::Translate(x1 + x2, y1 + y2)),
                (TransformFunc::TranslateXPercent(t1), TransformFunc::TranslateXPercent(t2)) => vec.push(TransformFunc::TranslateXPercent(t1 + t2)),
                (TransformFunc::TranslateYPercent(t1), TransformFunc::TranslateYPercent(t2)) => vec.push(TransformFunc::TranslateYPercent(t1 + t2)),
                (TransformFunc::TranslatePercent(x1, y1), TransformFunc::TranslatePercent(x2, y2)) => {
                    vec.push(TransformFunc::TranslatePercent(x1 + x2, y1 + y2))
                }
                (TransformFunc::ScaleX(t1), TransformFunc::ScaleX(t2)) => vec.push(TransformFunc::ScaleX(t1 + t2)),
                (TransformFunc::ScaleY(t1), TransformFunc::ScaleY(t2)) => vec.push(TransformFunc::ScaleY(t1 + t2)),
                (TransformFunc::Scale(x1, y1), TransformFunc::Scale(x2, y2)) => vec.push(TransformFunc::Scale(x1 + x2, y1 + y2)),
                (TransformFunc::RotateX(t1), TransformFunc::RotateX(t2)) => vec.push(TransformFunc::RotateX(t1 + t2)),
                (TransformFunc::RotateY(t1), TransformFunc::RotateY(t2)) => vec.push(TransformFunc::RotateY(t1 + t2)),
                (TransformFunc::RotateZ(t1), TransformFunc::RotateZ(t2)) => vec.push(TransformFunc::RotateZ(t1 + t2)),
                (TransformFunc::SkewX(t1), TransformFunc::SkewX(t2)) => vec.push(TransformFunc::SkewX(t1 + t2)),
                (TransformFunc::SkewY(t1), TransformFunc::SkewY(t2)) => vec.push(TransformFunc::SkewY(t1 + t2)),
                _ => return self.clone(), // 其他情况无法插值，则返回原值
            }
        }
        vec
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let t1 = &self[i];
            match t1 {
                TransformFunc::TranslateX(t1) => vec.push(TransformFunc::TranslateX(t1 * other)),
                TransformFunc::TranslateY(t1) => vec.push(TransformFunc::TranslateY(t1 * other)),
                TransformFunc::Translate(t1, t2) => vec.push(TransformFunc::Translate(t1 * other, t2 * other)),
                TransformFunc::TranslateXPercent(t1) => vec.push(TransformFunc::TranslateXPercent(t1 * other)),
                TransformFunc::TranslateYPercent(t1) => vec.push(TransformFunc::TranslateYPercent(t1 * other)),
                TransformFunc::TranslatePercent(t1, t2) => vec.push(TransformFunc::TranslatePercent(t1 * other, t2 * other)),
                TransformFunc::ScaleX(t1) => vec.push(TransformFunc::ScaleX(t1 * other)),
                TransformFunc::ScaleY(t1) => vec.push(TransformFunc::ScaleY(t1 * other)),
                TransformFunc::Scale(t1, t2) => vec.push(TransformFunc::Scale(t1 * other, t2 * other)),
                TransformFunc::RotateX(t1) => vec.push(TransformFunc::RotateX(t1 * other)),
                TransformFunc::RotateY(t1) => vec.push(TransformFunc::RotateY(t1 * other)),
                TransformFunc::RotateZ(t1) => vec.push(TransformFunc::RotateZ(t1 * other)),
                TransformFunc::SkewX(t1) => vec.push(TransformFunc::SkewX(t1 * other)),
                TransformFunc::SkewY(t1) => vec.push(TransformFunc::SkewY(t1 * other)),
            }
        }
        vec
    }
}
