//! * 定义样式类型
//! * 为所有的样式类型实现Attr这个tarit
//! * 为所有的样式类型实现Add和Scale trait，用于动画插值

use std::mem::forget;
use std::ops::Add;

use bitvec::array::BitArray;
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_curves::curve::frame::{FrameValueScale, KeyFrameCurveValue};
use pi_ecs::prelude::{DefaultComponent, Id, Query, ResMut, Write};
use pi_flex_layout::{
    prelude::{Number, Rect},
    style::{
        AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType as PositionType1,
    },
};
use pi_hash::XHashMap;
use pi_print_any::out_any;
use smallvec::SmallVec;

use crate::style::{
    Aabb2, Animation, AnimationDirection, AnimationFillMode, AnimationPlayState, AnimationTimingFunction, BackgroundColor, BackgroundImage,
    BackgroundImageClip, BackgroundImageMod, BlendMode, Blur, Border, BorderColor, BorderImage, BorderImageClip, BorderImageRepeat, BorderImageSlice,
    BorderRadius, BoxShadow, CgColor, Color, Enable, FitType, FlexContainer, FlexNormal, FontSize, FontStyle, Hsi, ImageRepeat, IterationCount,
    LengthUnit, LineHeight, Margin, MaskImage, MaskImageClip, MinMax, Node, NodeState, NotNanRect, Opacity, Overflow, Padding, Point2, Position,
    Show, Size, Stroke, StyleType, TextAlign, TextContent, TextShadows, TextStyle, Time, Transform, TransformFunc, TransformFuncs, TransformOrigin,
    TransformWillChange, VerticalAlign, WhiteSpace, ZIndex,
};

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

    /// 将样式属性设置到组件上
    /// ptr为样式属性的指针
    /// 安全： entity必须存在
    fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
    where
        Self: Sized;

    /// 为样式设置默认值
    fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>)
    where
        Self: Sized;
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

/// 从Buffer中读取StyleType
pub struct StyleTypeReader<'a> {
    buffer: &'a Vec<u8>,
    cursor: usize,
    end: usize,
}

pub enum StyleSet {
    Set,
    Cancel,
}

impl<'a> StyleTypeReader<'a> {
    pub fn default(buffer: &Vec<u8>) -> StyleTypeReader {
        StyleTypeReader {
            buffer,
            cursor: 0,
            end: buffer.len(),
        }
    }

    pub fn new(buffer: &Vec<u8>, start: usize, end: usize) -> StyleTypeReader { StyleTypeReader { buffer, cursor: start, end } }

    // 将当前style写入组件
    pub fn write_to_component(&mut self, cur_style_mark: &mut BitArray<[u32; 3]>, entity: Id<Node>, query: &mut StyleQuery) -> bool {
        let next_type = self.next_type();
        // log::info!("write_to_component ty: {:?}, cursor:{}, buffer_len:{}", next_type, self.cursor, self.buffer.len());
        if let Some(style_type) = next_type {
            StyleAttr::set(cur_style_mark, style_type, &self.buffer, self.cursor, query, entity);
            let size = StyleAttr::size(style_type);
            self.cursor += size;
            return true;
            // return Some(StyleAttr::get_type(style_type));
        }
        false
    }

    // 将当前style写入默认组件
    pub fn write_to_default(&mut self, query: &mut DefaultStyle<'a>) -> Option<StyleType> {
        let next_type = self.next_type();
        // log::info!("write_to_component ty: {:?}, cursor:{}, buffer_len:{}", next_type, self.cursor, self.buffer.len());
        if let Some(style_type) = next_type {
            StyleAttr::set_default(style_type, &self.buffer, self.cursor, query);
            let size = StyleAttr::size(style_type);
            self.cursor += size;
            return Some(StyleAttr::get_type(style_type));
        }

        None
    }

    // f函数返回true，则写入到组件，否则不写入,跳过该属性
    pub fn or_write_to_component<F: Fn(StyleType) -> bool>(
        &mut self,
        cur_style_mark: &mut BitArray<[u32; 3]>,
        entity: Id<Node>,
        query: &mut StyleQuery,
        f: F,
    ) -> Option<StyleType> {
        let next_type = self.next_type();
        if let Some(style_type) = next_type {
            let ty = StyleAttr::get_type(style_type);
            if f(ty) {
                StyleAttr::set(cur_style_mark, style_type, &self.buffer, self.cursor, query, entity);
            }
            let size = StyleAttr::size(style_type);
            self.cursor += size;
            return Some(ty);
        }
        None
    }

    // 读下一个样式类型
    fn next_type(&mut self) -> Option<u8> {
        if self.cursor >= self.end {
            return None;
        }

        // let ty_size = std::mem::size_of::<u8>();
        let ty = unsafe { Some(self.buffer.as_ptr().add(self.cursor).cast::<u8>().read_unaligned()) };

        // log::info!("next_type ty: {:?}, type_size:{:?}", ty, ty_size);
        // self.cursor += ty_size;
        self.cursor += 1;
        ty
    }
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

            // println_any!("write, value: {:?}, start: {:?}, end: {:?}", Self, len, len + ty_size + value_size);

            let ty = Self::get_style_index();
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

macro_rules! set {
    // 整体插入
    ($name: ident, $value_ty: ty) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);

            let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };

            cur_style_mark.set(Self::get_type() as usize, true);
            item.write(v);
        }
    };
    // 属性修改
    ($name: ident, $feild: ident, $value_ty: ty) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
            out_any!(log::trace, "set: {:?}", &v);
            let component = item.get_mut_or_default();
            component.$feild = v;
            cur_style_mark.set(Self::get_type() as usize, true);
            item.notify_modify();
        }
    };
    // 属性修改
    (@func $name: ident, $set_func: ident, $value_ty: ty) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
            let component = item.get_mut_or_default();
            component.$set_func(v);
            cur_style_mark.set(Self::get_type() as usize, true);
            item.notify_modify();
        }
    };

    // 属性修改
    ($name: ident, $feild1: ident, $feild2: ident, $value_ty: ty) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
            let component = item.get_mut_or_default();
            component.$feild1.$feild2 = v;
            cur_style_mark.set(Self::get_type() as usize, true);
            item.notify_modify();
        }
    };

    // 盒模属性（上右下左）
    (@box_model $name: ident, $value_ty: ty) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
            let component = item.get_mut_or_default();
            component.top = v.top;
            component.right = v.right;
            component.bottom = v.bottom;
            component.left = v.left;
            cur_style_mark.set(Self::get_type() as usize, true);
            item.notify_modify();
        }
    };
}

// 设置默认值
macro_rules! set_default {
    (@empty) => {
        fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _query: &mut DefaultStyle<'a>) {}
    };
    // 整体插入
    ($name: ident, $value_ty: ty) => {
        fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>) {
            *(query.$name) = DefaultComponent(unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() });
        }
    };
    // 属性修改
    ($name: ident, $feild: ident, $value_ty: ty) => {
        fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>) {
            query.$name.$feild = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
        }
    };
    // 属性修改
    (@func $name: ident, $set_func: ident, $value_ty: ty) => {
        fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>) {
            query
                .$name
                .$set_func(unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() });
        }
    };

    // 属性修改
    ($name: ident, $feild1: ident, $feild2: ident, $value_ty: ty) => {
        fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>) {
            query.$name.$feild1.$feild2 = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
        }
    };

    // 盒模属性（上右下左）
    (@box_model $name: ident, $value_ty: ty) => {
        fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle<'a>) {
            let v = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
            let c = &mut query.$name;
            c.top = v.top;
            c.right = v.right;
            c.bottom = v.bottom;
            c.left = v.left;
        }
    };
}

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
            fn scale(&self, rhs: KeyFrameCurveValue) -> Self { Self(self.0.scale(rhs)) }
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

macro_rules! reset {
    // 空实现
    (@empty) => {
        fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, _query: &mut StyleQuery, _entity: Id<Node>) {}
    };
    ($name: ident, $value_ty: ident) => {
        fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = item.get_default().clone();
            item.write(v);
        }
    };
    // 属性修改
    ($name: ident, $feild: ident) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = item.get_default().$feild.clone();
            let component = item.get_mut_or_default();
            component.$feild = v;
            item.notify_modify();
        }
    };
    // 属性修改
    (@func $name: ident, $set_func: ident, $get_func: ident) => {
        fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = item.get_default().$get_func();
            let component = item.get_mut_or_default();
            component.$set_func(v);
            item.notify_modify();
        }
    };
    // 属性修改
    ($name: ident, $feild1: ident, $feild2: ident) => {
        fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = item.get_default().$feild1.$feild2.clone();
            let component = item.get_mut_or_default();
            component.$feild1.$feild2 = v;
            item.notify_modify();
        }
    };
    // 属性修改
    (@box_model_single $name: ident, $feild: ident) => {
        fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 取不到说明实体已经销毁
            let mut item = query.$name.get_unchecked_mut(entity);
            let v = item.get_default().$feild.clone();
            let component = item.get_mut_or_default();
            component.$feild = v;
            item.notify_modify();
        }
    };

    (@box_model $name: ident, $ty: ident) => {
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
            // 设置为默认值 TODO
            let mut item = query.$name.get_unchecked_mut(entity);
            let default_value = item.get_default().clone();
            if let Some(component) = item.get_mut() {
                let mut is_changed = false;
                $crate::paste::item! {
                if !cur_style_mark[StyleType::[<$ty Top>] as usize] {
                    is_changed = true;
                    component.top = default_value.top;
                }
                if !cur_style_mark[StyleType::[<$ty Right>] as usize] {
                    is_changed = true;
                    component.right = default_value.right;
                }
                if !cur_style_mark[StyleType::[<$ty Bottom>] as usize] {
                    is_changed = true;
                    component.bottom = default_value.bottom;
                }
                if !cur_style_mark[StyleType::[<$ty Left>] as usize] {
                    is_changed = true;
                    component.left = default_value.left;
                }
                }

                // 通知padding修改
                if is_changed {
                    item.notify_modify();
                }
            }
        }
    };
}

macro_rules! impl_style {
	($struct_name: ident, $name: ident, $ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $ty);

		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($ty);
			write_buffer!();
			set!($name, $ty);
			// reset!($name, $ty);
			set_default!($name, $ty);
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
				reset!($name, $ty);
				set_default!($name, $ty);
			}
		}
	};
	($struct_name: ident, $name: ident, $ty: ident, $value_ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $value_ty);
			// reset!($name);
			set_default!($name, $value_ty);
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
				reset!($name);
				set_default!($name, $value_ty);
			}
		}
	};
	($struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ty) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			// reset!($name, $feild);
			set_default!($name, $feild, $value_ty);
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
				reset!($name, $feild);
				set_default!($name, $feild, $value_ty);
			}
		}
	};
	($struct_name: ident, $name: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild1, $feild2, $value_ty);
			// reset!($name, $feild1, $feild2);
			set_default!($name, $feild1, $feild2, $value_ty);
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
				reset!($name, $feild1, $feild2);
				set_default!($name, $feild1, $feild2, $value_ty);
			}
		}
	};
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!(@func $name, $set_func, $value_ty);
			// reset!(@func $name, $set_func, $get_func);
			set_default!(@func $name, $set_func, $value_ty);
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
				reset!(@func $name, $set_func, $get_func);
				set_default!(@func $name, $set_func, $value_ty);
			}
		}
	};
	// 方法设置，并且不实现set_default和reset
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $ty: ident, $value_ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!(@func $name, $set_func, $value_ty);
			// reset!(@empty);
			set_default!(@empty);
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
				reset!(@empty);
				set_default!(@empty);
			}
		}
	};

	(@box_model_single $struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			// reset!(@box_model_single $name, $feild, $ty_all);
			set_default!($name, $feild, $value_ty);
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
				reset!(@box_model_single $name, $feild);
				set_default!($name, $feild, $value_ty);
			}
		}
	};
	(@box_model $struct_name: ident, $name: ident, $ty: ident) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		pub struct $struct_name(pub $ty);
		impl Attr for $struct_name {
			fn get_style_index() -> u8 {
				Self::get_type() as u8
			}
			get_type!(StyleType::$ty);
			size!($ty);
			write_buffer!();
			set!(@box_model $name, $ty);
			// reset!(@box_model $name, $ty);
			set_default!(@box_model $name, $ty);
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
				reset!(@box_model $name, $ty);
				set_default!(@box_model $name, $ty);
			}
		}
	};
}

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
    fn add(&self, rhs: &Self) -> Self {
        NotNanRect {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
        }
    }
    #[inline]
    fn scale(&self, other: f32) -> Self {
        let other = NotNan::new(other).unwrap();
        NotNanRect {
            left: self.left * other,
            right: self.right * other,
            top: self.top * other,
            bottom: self.bottom * other,
        }
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
impl_interpolation!(@animatable_value_next, BackgroundImageClipType, BackgroundImageClip);
impl_interpolation!(@keep, ObjectFitType);
impl_interpolation!(@keep, BackgroundRepeatType);

impl_interpolation!(@keep, BorderImageType);
impl_interpolation!(@animatable_value_next, BorderImageClipType, BorderImageClip);
impl_interpolation!(@animatable_value, BorderImageSliceType);
impl_interpolation!(@keep, BorderImageRepeatType);

impl_interpolation!(@animatable_value_next, BorderColorType, BorderColor);

impl_interpolation!(@animatable_value_next, BackgroundColorType, BackgroundColor);

impl_interpolation!(@animatable_value, BoxShadowType);

impl_interpolation!(@animatable_value_next, OpacityType, Opacity);
impl_interpolation!(@animatable_value, BorderRadiusType);
impl_interpolation!(@animatable_value, HsiType);
impl_interpolation!(@animatable_value_next, BlurType, Blur);
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

impl_interpolation!(@animatable_value_next, ZIndexType, ZIndex);
impl_interpolation!(@keep, OverflowType);

impl_interpolation!(@keep, MaskImageType);
impl_interpolation!(@animatable_value_next, MaskImageClipType, MaskImageClip);

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

// // 设置Position、Border、Margin、Padding的优先级比单独设置上右下左的优先级要低，所以有单独的标识，
// // 假定Position属性的设置，直接作用到上由下左上，可能会覆盖单独设置的上右下左属性
// impl_interpolation!(@animatable_value_next, PositionType, Position);
// impl_interpolation!(@animatable_value_next, BorderType, Border);
// impl_interpolation!(@animatable_value_next, MarginType, Margin);
// impl_interpolation!(@animatable_value_next, PaddingType, Padding);


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
impl_style!(TextShadowType, text_style, text_shadow, TextShadow, TextShadows);

impl_style!(BackgroundImageType, background_image, BackgroundImage);
impl_style!(BackgroundImageClipType, background_image_clip, BackgroundImageClip);
impl_style!(ObjectFitType, background_image_mod, object_fit, ObjectFit, FitType);
impl_style!(BackgroundRepeatType, background_image_mod, repeat, BackgroundRepeat, ImageRepeat);

impl_style!(BorderImageType, border_image, BorderImage);
impl_style!(BorderImageClipType, border_image_clip, BorderImageClip);
impl_style!(BorderImageSliceType, border_image_slice, BorderImageSlice);
impl_style!(BorderImageRepeatType, border_image_repeat, BorderImageRepeat);

impl_style!(BorderColorType, border_color, BorderColor);

impl_style!(BackgroundColorType, background_color, BackgroundColor);

impl_style!(BoxShadowType, box_shadow, BoxShadow);

impl_style!(OpacityType, opacity, Opacity);
impl_style!(BorderRadiusType, border_radius, BorderRadius);
impl_style!(HsiType, hsi, Hsi);
impl_style!(BlurType, blur, Blur);
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

impl_style!(TransformWillChangeType, transform_will_change, TransformWillChange);

impl_style!(ZIndexType, z_index, ZIndex);
impl_style!(OverflowType, overflow, Overflow);

impl_style!(MaskImageType, mask_image, MaskImage);
impl_style!(MaskImageClipType, mask_image_clip, MaskImageClip);

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
impl_style!(AnimationNameType, animation, name, AnimationName, SmallVec<[Atom; 1]>);
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

// AnimationName = 79,
// AnimationDuration = 80,
// AnimationTimingFunction = 81,
// AnimationDelay = 82,
// AnimationIterationCount = 83,
// AnimationDirection = 84,
// AnimationFillMode = 85,
// AnimationPlayState = 86,

// // 设置Position、Border、Margin、Padding的优先级比单独设置上右下左的优先级要低，所以有单独的标识，
// // 假定Position属性的设置，直接作用到上由下左上，可能会覆盖单独设置的上右下左属性
// impl_style!(@box_model PositionType, position, Position);
// impl_style!(@box_model BorderType, border, Border);
// impl_style!(@box_model MarginType, margin, Margin);
// impl_style!(@box_model PaddingType, padding, Padding);

// impl_style!(AnimationType, animation, Animation);

pub struct StyleFunc {
    get_type: fn() -> StyleType,
    // get_style_index: fn() -> u8,
    size: fn() -> usize,
    // /// 安全： entity必须存在
    // fn set(&self, cur_style_mark: &mut BitArray<[u32;3]>, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>);
    /// 安全： entity必须存在
    set: fn(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>),

    /// 设置默认值
    set_default: fn(buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle),
}

impl StyleFunc {
    fn new<T: Attr>() -> StyleFunc {
        StyleFunc {
            get_type: T::get_type,
            // get_style_index: T::get_style_index,
            size: T::size,
            set: T::set,
            set_default: T::set_default,
            // add: T::add,
            // scale: T::scale,
        }
    }
}

lazy_static::lazy_static! {
    static ref STYLE_ATTR: [StyleFunc; 173] = [
        StyleFunc::new::<PaddingTopType>(), // 0 empty 占位， 无实际作用
        StyleFunc::new::<BackgroundRepeatType>(), // 1 text
        StyleFunc::new::<FontStyleType>(), // 2
        StyleFunc::new::<FontWeightType>(), // 3
        StyleFunc::new::<FontSizeType>(), // 4
        StyleFunc::new::<FontFamilyType>(), // 5
        StyleFunc::new::<LetterSpacingType>(), // 6
        StyleFunc::new::<WordSpacingType>(), // 7
        StyleFunc::new::<LineHeightType>(), // 8
        StyleFunc::new::<TextIndentType>(), // 9
        StyleFunc::new::<WhiteSpaceType>(), // 10

        StyleFunc::new::<TextAlignType>(), // 11
        StyleFunc::new::<VerticalAlignType>(), // 12
        StyleFunc::new::<ColorType>(), // 13
        StyleFunc::new::<TextStrokeType>(), // 14
        StyleFunc::new::<TextShadowType>(), // 15

        StyleFunc::new::<BackgroundImageType>(), // 16
        StyleFunc::new::<BackgroundImageClipType>(), // 17
        StyleFunc::new::<ObjectFitType>(), // 18
        StyleFunc::new::<BackgroundColorType>(), // 19
        StyleFunc::new::<BoxShadowType>(), // 20
        StyleFunc::new::<BorderImageType>(), // 21
        StyleFunc::new::<BorderImageClipType>(), // 22
        StyleFunc::new::<BorderImageSliceType>(), // 23
        StyleFunc::new::<BorderImageRepeatType>(), // 24

        StyleFunc::new::<BorderColorType>(), // 25


        StyleFunc::new::<HsiType>(), // 26
        StyleFunc::new::<BlurType>(), // 27
        StyleFunc::new::<MaskImageType>(), // 28
        StyleFunc::new::<MaskImageClipType>(), // 29
        StyleFunc::new::<TransformType>(), // 30
        StyleFunc::new::<TransformOriginType>(), // 31
        StyleFunc::new::<TransformWillChangeType>(), // 32
        StyleFunc::new::<BorderRadiusType>(), // 33
        StyleFunc::new::<ZIndexType>(), // 34
        StyleFunc::new::<OverflowType>(), // 35


        StyleFunc::new::<BlendModeType>(), // 36
        StyleFunc::new::<DisplayType>(), // 37
        StyleFunc::new::<VisibilityType>(), // 38
        StyleFunc::new::<EnableType>(), // 30


        StyleFunc::new::<WidthType>(), // 40
        StyleFunc::new::<HeightType>(), // 41

        StyleFunc::new::<MarginTopType>(), // 42
        StyleFunc::new::<MarginRightType>(), // 43
        StyleFunc::new::<MarginBottomType>(), // 44
        StyleFunc::new::<MarginLeftType>(), // 45

        StyleFunc::new::<PaddingTopType>(), // 46
        StyleFunc::new::<PaddingRightType>(), // 47
        StyleFunc::new::<PaddingBottomType>(), // 48
        StyleFunc::new::<PaddingLeftType>(), // 49

        StyleFunc::new::<BorderTopType>(), // 50
        StyleFunc::new::<BorderRightType>(), // 51
        StyleFunc::new::<BorderBottomType>(), // 52
        StyleFunc::new::<BorderLeftType>(), // 53

        StyleFunc::new::<PositionTopType>(), // 54
        StyleFunc::new::<PositionRightType>(), // 55
        StyleFunc::new::<PositionBottomType>(), // 56
        StyleFunc::new::<PositionLeftType>(), // 57

        StyleFunc::new::<MinWidthType>(), // 58
        StyleFunc::new::<MinHeightType>(), // 59
        StyleFunc::new::<MaxHeightType>(), // 60
        StyleFunc::new::<MaxWidthType>(), // 61
        StyleFunc::new::<DirectionType>(), // 62
        StyleFunc::new::<FlexDirectionType>(), // 63
        StyleFunc::new::<FlexWrapType>(), // 64
        StyleFunc::new::<JustifyContentType>(), // 65
        StyleFunc::new::<AlignContentType>(), // 66
        StyleFunc::new::<AlignItemsType>(), // 67


        StyleFunc::new::<PositionTypeType>(), // 68
        StyleFunc::new::<AlignSelfType>(), // 69
        StyleFunc::new::<FlexShrinkType>(), // 70
        StyleFunc::new::<FlexGrowType>(), // 71
        StyleFunc::new::<AspectRatioType>(), // 72
        StyleFunc::new::<OrderType>(), // 73
        StyleFunc::new::<FlexBasisType>(), // 74
        StyleFunc::new::<OpacityType>(), // 75

        StyleFunc::new::<TextContentType>(), // 76

        StyleFunc::new::<VNodeType>(), // 77

        StyleFunc::new::<TransformFuncType>(), // 78

        StyleFunc::new::<AnimationNameType>(), // 79
        StyleFunc::new::<AnimationDurationType>(), // 80
        StyleFunc::new::<AnimationTimingFunctionType>(), // 81
        StyleFunc::new::<AnimationDelayType>(), // 82
        StyleFunc::new::<AnimationIterationCountType>(), // 83
        StyleFunc::new::<AnimationDirectionType>(), // 84
        StyleFunc::new::<AnimationFillModeType>(), // 85
        StyleFunc::new::<AnimationPlayStateType>(), // 86

    /******************************* reset ******************************************************/
        StyleFunc::new::<ResetBackgroundRepeatType>(), // 1 text
        StyleFunc::new::<ResetFontStyleType>(), // 2
        StyleFunc::new::<ResetFontWeightType>(), // 3
        StyleFunc::new::<ResetFontSizeType>(), // 4
        StyleFunc::new::<FontFamilyType>(), // 5
        StyleFunc::new::<LetterSpacingType>(), // 6
        StyleFunc::new::<WordSpacingType>(), // 7
        StyleFunc::new::<ResetLineHeightType>(), // 8
        StyleFunc::new::<TextIndentType>(), // 9
        StyleFunc::new::<ResetWhiteSpaceType>(), // 10

        StyleFunc::new::<ResetTextAlignType>(), // 11
        StyleFunc::new::<ResetVerticalAlignType>(), // 12
        StyleFunc::new::<ResetColorType>(), // 13
        StyleFunc::new::<ResetTextStrokeType>(), // 14
        StyleFunc::new::<ResetTextShadowType>(), // 15

        StyleFunc::new::<ResetBackgroundImageType>(), // 16
        StyleFunc::new::<ResetBackgroundImageClipType>(), // 17
        StyleFunc::new::<ResetObjectFitType>(), // 18
        StyleFunc::new::<ResetBackgroundColorType>(), // 19
        StyleFunc::new::<ResetBoxShadowType>(), // 20
        StyleFunc::new::<ResetBorderImageType>(), // 21
        StyleFunc::new::<ResetBorderImageClipType>(), // 22
        StyleFunc::new::<ResetBorderImageSliceType>(), // 23
        StyleFunc::new::<ResetBorderImageRepeatType>(), // 24

        StyleFunc::new::<ResetBorderColorType>(), // 25


        StyleFunc::new::<ResetHsiType>(), // 26
        StyleFunc::new::<ResetBlurType>(), // 27
        StyleFunc::new::<ResetMaskImageType>(), // 28
        StyleFunc::new::<ResetMaskImageClipType>(), // 29
        StyleFunc::new::<ResetTransformType>(), // 30
        StyleFunc::new::<ResetTransformOriginType>(), // 31
        StyleFunc::new::<ResetTransformWillChangeType>(), // 32
        StyleFunc::new::<ResetBorderRadiusType>(), // 33
        StyleFunc::new::<ResetZIndexType>(), // 34
        StyleFunc::new::<ResetOverflowType>(), // 35


        StyleFunc::new::<ResetBlendModeType>(), // 36
        StyleFunc::new::<ResetDisplayType>(), // 37
        StyleFunc::new::<ResetVisibilityType>(), // 38
        StyleFunc::new::<ResetEnableType>(), // 39


        StyleFunc::new::<ResetWidthType>(), // 40
        StyleFunc::new::<ResetHeightType>(), // 41

        StyleFunc::new::<ResetMarginTopType>(), // 42
        StyleFunc::new::<ResetMarginRightType>(), // 43
        StyleFunc::new::<ResetMarginBottomType>(), // 44
        StyleFunc::new::<ResetMarginLeftType>(), // 45

        StyleFunc::new::<ResetPaddingTopType>(), // 46
        StyleFunc::new::<ResetPaddingRightType>(), // 47
        StyleFunc::new::<ResetPaddingBottomType>(), // 48
        StyleFunc::new::<ResetPaddingLeftType>(), // 49

        StyleFunc::new::<ResetBorderTopType>(), // 50
        StyleFunc::new::<ResetBorderRightType>(), // 51
        StyleFunc::new::<ResetBorderBottomType>(), // 52
        StyleFunc::new::<ResetBorderLeftType>(), // 53

        StyleFunc::new::<ResetPositionTopType>(), // 54
        StyleFunc::new::<ResetPositionRightType>(), // 55
        StyleFunc::new::<ResetPositionBottomType>(), // 56
        StyleFunc::new::<ResetPositionLeftType>(), // 57

        StyleFunc::new::<ResetMinWidthType>(), // 58
        StyleFunc::new::<ResetMinHeightType>(), // 59
        StyleFunc::new::<ResetMaxHeightType>(), // 60
        StyleFunc::new::<ResetMaxWidthType>(), // 61
        StyleFunc::new::<ResetDirectionType>(), // 62
        StyleFunc::new::<ResetFlexDirectionType>(), // 63
        StyleFunc::new::<ResetFlexWrapType>(), // 64
        StyleFunc::new::<ResetJustifyContentType>(), // 65
        StyleFunc::new::<ResetAlignContentType>(), // 66
        StyleFunc::new::<ResetAlignItemsType>(), // 67


        StyleFunc::new::<ResetPositionTypeType>(), // 68
        StyleFunc::new::<ResetAlignSelfType>(), // 69
        StyleFunc::new::<FlexShrinkType>(), // 70
        StyleFunc::new::<FlexGrowType>(), // 71
        StyleFunc::new::<ResetAspectRatioType>(), // 72
        StyleFunc::new::<ResetOrderType>(), // 73
        StyleFunc::new::<ResetFlexBasisType>(), // 74
        StyleFunc::new::<ResetOpacityType>(), // 75

        StyleFunc::new::<ResetTextContentType>(), // 76

        StyleFunc::new::<ResetVNodeType>(), // 77

        StyleFunc::new::<ResetTransformFuncType>(), // 78

        StyleFunc::new::<ResetAnimationNameType>(), // 79
        StyleFunc::new::<ResetAnimationDurationType>(), // 80
        StyleFunc::new::<ResetAnimationTimingFunctionType>(), // 81
        StyleFunc::new::<ResetAnimationDelayType>(), // 82
        StyleFunc::new::<ResetAnimationIterationCountType>(), // 83
        StyleFunc::new::<ResetAnimationDirectionType>(), // 84
        StyleFunc::new::<ResetAnimationFillModeType>(), // 85
        StyleFunc::new::<ResetAnimationPlayStateType>(), // 86

    ];
}

pub struct StyleQuery<'a> {
    pub size: Query<'static, 'static, Node, Write<Size>>,
    pub margin: Query<'static, 'static, Node, Write<Margin>>,
    pub padding: Query<'static, 'static, Node, Write<Padding>>,
    pub border: Query<'static, 'static, Node, Write<Border>>,
    pub position: Query<'static, 'static, Node, Write<Position>>,
    pub min_max: Query<'static, 'static, Node, Write<MinMax>>,
    pub flex_container: Query<'static, 'static, Node, Write<FlexContainer>>,
    pub flex_normal: Query<'static, 'static, Node, Write<FlexNormal>>,
    pub z_index: Query<'static, 'static, Node, Write<ZIndex>>,
    pub overflow: Query<'static, 'static, Node, Write<Overflow>>,
    pub opacity: Query<'static, 'static, Node, Write<Opacity>>,
    pub blend_mode: Query<'static, 'static, Node, Write<BlendMode>>,
    pub show: Query<'static, 'static, Node, Write<Show>>,
    pub transform: Query<'static, 'static, Node, Write<Transform>>,
    pub background_color: Query<'static, 'static, Node, Write<BackgroundColor>>,
    pub border_color: Query<'static, 'static, Node, Write<BorderColor>>,
    pub background_image: Query<'static, 'static, Node, Write<BackgroundImage>>,
    pub background_image_clip: Query<'static, 'static, Node, Write<BackgroundImageClip>>,
    pub mask_image: Query<'static, 'static, Node, Write<MaskImage>>,
    pub mask_image_clip: Query<'static, 'static, Node, Write<MaskImageClip>>,
    pub hsi: Query<'static, 'static, Node, Write<Hsi>>,
    pub blur: Query<'static, 'static, Node, Write<Blur>>,
    pub background_image_mod: Query<'static, 'static, Node, Write<BackgroundImageMod>>,
    pub border_image: Query<'static, 'static, Node, Write<BorderImage>>,
    pub border_image_clip: Query<'static, 'static, Node, Write<BorderImageClip>>,
    pub border_image_slice: Query<'static, 'static, Node, Write<BorderImageSlice>>,
    pub border_image_repeat: Query<'static, 'static, Node, Write<BorderImageRepeat>>,
    pub border_radius: Query<'static, 'static, Node, Write<BorderRadius>>,
    pub box_shadow: Query<'static, 'static, Node, Write<BoxShadow>>,
    pub text_style: Query<'static, 'static, Node, Write<TextStyle>>,
    pub transform_will_change: Query<'static, 'static, Node, Write<TransformWillChange>>,
    pub text_content: Query<'static, 'static, Node, Write<TextContent>>,
    pub node_state: Query<'static, 'static, Node, Write<NodeState>>,
    pub animation: &'a mut Query<'static, 'static, Node, Write<Animation>>,
}

pub struct DefaultStyle<'a> {
    pub size: ResMut<'a, DefaultComponent<Size>>,
    pub margin: ResMut<'a, DefaultComponent<Margin>>,
    pub padding: ResMut<'a, DefaultComponent<Padding>>,
    pub border: ResMut<'a, DefaultComponent<Border>>,
    pub position: ResMut<'a, DefaultComponent<Position>>,
    pub min_max: ResMut<'a, DefaultComponent<MinMax>>,
    pub flex_container: ResMut<'a, DefaultComponent<FlexContainer>>,
    pub flex_normal: ResMut<'a, DefaultComponent<FlexNormal>>,
    pub z_index: ResMut<'a, DefaultComponent<ZIndex>>,
    pub overflow: ResMut<'a, DefaultComponent<Overflow>>,
    pub opacity: ResMut<'a, DefaultComponent<Opacity>>,
    pub blend_mode: ResMut<'a, DefaultComponent<BlendMode>>,
    pub show: ResMut<'a, DefaultComponent<Show>>,
    pub transform: ResMut<'a, DefaultComponent<Transform>>,
    pub background_color: ResMut<'a, DefaultComponent<BackgroundColor>>,
    pub border_color: ResMut<'a, DefaultComponent<BorderColor>>,
    pub background_image: ResMut<'a, DefaultComponent<BackgroundImage>>,
    pub background_image_clip: ResMut<'a, DefaultComponent<BackgroundImageClip>>,
    pub mask_image: ResMut<'a, DefaultComponent<MaskImage>>,
    pub mask_image_clip: ResMut<'a, DefaultComponent<MaskImageClip>>,
    pub hsi: ResMut<'a, DefaultComponent<Hsi>>,
    pub blur: ResMut<'a, DefaultComponent<Blur>>,
    pub background_image_mod: ResMut<'a, DefaultComponent<BackgroundImageMod>>,
    pub border_image: ResMut<'a, DefaultComponent<BorderImage>>,
    pub border_image_clip: ResMut<'a, DefaultComponent<BorderImageClip>>,
    pub border_image_slice: ResMut<'a, DefaultComponent<BorderImageSlice>>,
    pub border_image_repeat: ResMut<'a, DefaultComponent<BorderImageRepeat>>,
    pub border_radius: ResMut<'a, DefaultComponent<BorderRadius>>,
    pub box_shadow: ResMut<'a, DefaultComponent<BoxShadow>>,
    pub text_style: ResMut<'a, DefaultComponent<TextStyle>>,
    pub transform_will_change: ResMut<'a, DefaultComponent<TransformWillChange>>,
    pub text_content: ResMut<'a, DefaultComponent<TextContent>>,
    pub animation: ResMut<'a, DefaultComponent<Animation>>,
}

pub struct StyleAttr;

impl StyleAttr {
    #[inline]
    pub fn get_type(style_type: u8) -> StyleType { (STYLE_ATTR[style_type as usize].get_type)() }

    #[inline]
    pub unsafe fn write<T: Attr>(value: T, buffer: &mut Vec<u8>) {
        value.write(buffer);
        forget(value);
    }

    #[inline]
    pub fn set(cur_style_mark: &mut BitArray<[u32; 3]>, style_index: u8, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>) {
        (STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity)
    }

    #[inline]
    pub fn size(style_index: u8) -> usize { (STYLE_ATTR[style_index as usize].size)() }

    #[inline]
    pub fn reset(
        cur_style_mark: &mut BitArray<[u32; 3]>,
        style_index: u8,
        buffer: &Vec<u8>,
        offset: usize,
        query: &mut StyleQuery,
        entity: Id<Node>,
    ) {
        (STYLE_ATTR[style_index as usize + 86].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity);
    }

    #[inline]
    pub fn set_default(style_index: u8, buffer: &Vec<u8>, offset: usize, query: &mut DefaultStyle) {
        (STYLE_ATTR[style_index as usize].set_default)(buffer, offset, query);
    }
}
