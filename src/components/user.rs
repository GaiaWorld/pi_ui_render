use bitvec::prelude::BitArray;
use pi_ecs::prelude::Id;
use pi_render::graph::NodeId as GraphId;
pub use pi_style::style::*;
pub use super::root::{ClearColor, Viewport, RenderDirty, RenderTargetType};

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

/// 绘制canvas的图节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas(pub GraphId);

/// 显示改变（一般是指canvas，gui不能感知除了style属性以外的属性改变，如果canvas内容发生改变，应该通过style设置，以便gui能感知，从而设置脏区域）
pub struct ShowChange;

pub fn get_size(s: &FontSize) -> usize {
    match s {
        &FontSize::None => {
            // size
            32 // 默认32px
        }
        &FontSize::Length(r) => r,
        &FontSize::Percent(_r) => {
            // (r * size as f32).round() as usize;
            panic!()
        }
    }
}


pub mod serialize {
    use std::mem::forget;

    use crate::components::user::*;
    use pi_atom::Atom;
    use pi_ecs::{
        prelude::{Query, ResMut},
        query::{DefaultComponent, Write},
    };
    use pi_flex_layout::{
        prelude::Number,
        style::{
            AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent,
            PositionType as PositionType1,
        },
    };
    use pi_style::{style_type::*, style_parse::Attribute};
    use smallvec::SmallVec;


    /// 定义trait ConvertToComponent， 可将buffer转化到ecs组件上
    pub trait ConvertToComponent: Attr {
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

		fn to_attr(ptr: *const u8) -> Attribute
		where
			Self: Sized;
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

		// 将当前style写入组件
        pub fn to_attr(&mut self) -> Option<Attribute> {
            let next_type = self.next_type();
            // log::info!("write_to_component ty: {:?}, cursor:{}, buffer_len:{}", next_type, self.cursor, self.buffer.len());
            if let Some(style_type) = next_type {
                let r = StyleAttr::to_attr(style_type, &self.buffer, self.cursor);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return Some(r);
                // return Some(StyleAttr::get_type(style_type));
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

    macro_rules! set {
        // 整体插入
        ($name: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
                // 取不到说明实体已经销毁
                let mut item = query.$name.get_unchecked_mut(entity);

                let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);

                cur_style_mark.set(Self::get_type() as usize, true);
                item.write(v);
            }
        };
        // 属性修改
        (@pack $name: ident, $pack_ty: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
                // 取不到说明实体已经销毁
                let mut item = query.$name.get_unchecked_mut(entity);

                let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);
                cur_style_mark.set(Self::get_type() as usize, true);
                item.write($pack_ty(v));
            }
        };
        // 属性修改
        ($name: ident, $feild: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>) {
                // 取不到说明实体已经销毁
                let mut item = query.$name.get_unchecked_mut(entity);
                let v = unsafe { ptr.cast::<$value_ty>().read_unaligned() };
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);
                // out_any!(log::trace, "set: {:?}", &v);
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
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);
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
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);
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
				log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);
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
	($struct_name: ident) => {
		impl ConvertToComponent for $struct_name {
			reset!(@empty);
			// reset!($name, $ty);
			fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _query: &mut DefaultStyle<'a>) {

			}
			fn to_attr(_ptr: *const u8) -> Attribute
			{
				todo!()
			}
		}
	};
	($struct_name: ident, $name: ident, $ty: ident) => {

		impl ConvertToComponent for $struct_name {
			set!($name, $ty);
			// reset!($name, $ty);
			set_default!($name, $ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!($name, $ty);
				set_default!($name, $ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty($struct_name(Default::default()))
				}
			}
			
		}
	};
	(@pack $struct_name: ident, $name: ident, $pack_ty: ident, $value_ty: ident) => {

		impl ConvertToComponent for $struct_name {
			set!(@pack $name, $pack_ty, $value_ty);
			// reset!($name, $ty);
			set_default!($name, $pack_ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$pack_ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!($name, $pack_ty);
				set_default!($name, $pack_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$pack_ty( $struct_name(Default::default()) )
				}
			}
		}
	};
	($struct_name: ident, $name: ident, $ty: ident, $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!($name, $value_ty);
			// reset!($name);
			set_default!($name, $value_ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!($name);
				set_default!($name, $value_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty( $struct_name(Default::default()) )
				}
			}
		}
	};
	($struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ty) => {
		impl ConvertToComponent for $struct_name {
			set!($name, $feild, $value_ty);
			// reset!($name, $feild);
			set_default!($name, $feild, $value_ty);
			fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized
			{
				Attribute::$ty(unsafe{$struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!($name, $feild);
				set_default!($name, $feild, $value_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty($struct_name(Default::default()) )
				}
			}
		}
	};
	($struct_name: ident, $name: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!($name, $feild1, $feild2, $value_ty);
			// reset!($name, $feild1, $feild2);
			set_default!($name, $feild1, $feild2, $value_ty);
			fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!($name, $feild1, $feild2);
				set_default!($name, $feild1, $feild2, $value_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty( $struct_name(Default::default()) )
				}
			}
		}
	};
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!(@func $name, $set_func, $value_ty);
			// reset!(@func $name, $set_func, $get_func);
			set_default!(@func $name, $set_func, $value_ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!(@func $name, $set_func, $get_func);
				set_default!(@func $name, $set_func, $value_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty( $struct_name(Default::default()) )
				}
			}
		}
	};
	// 方法设置，并且不实现set_default和reset
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $ty: ident, $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!(@func $name, $set_func, $value_ty);
			// reset!(@empty);
			set_default!(@empty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!(@empty);
				set_default!(@empty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty( $struct_name(Default::default()) )
				}
			}
		}
	};

	(@func1 $struct_name: ident,  $name: ident, $set_func: ident, $ty: ident, $attr_ty: ident,  $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!(@func $name, $set_func, $value_ty);
			// reset!(@empty);
			set_default!(@empty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$attr_ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!(@empty);
				set_default!(@empty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$attr_ty( $struct_name(Default::default()) )
				}
			}
		}
	};

	(@box_model_single $struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!($name, $feild, $value_ty);
			// reset!(@box_model_single $name, $feild, $ty_all);
			set_default!($name, $feild, $value_ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$value_ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {

			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!(@box_model_single $name, $feild);
				set_default!($name, $feild, $value_ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty($struct_name(Default::default()) )
				}
			}
		}
	};
	(@box_model $struct_name: ident, $name: ident, $ty: ident) => {
		impl ConvertToComponent for $struct_name {
			set!(@box_model $name, $ty);
			// reset!(@box_model $name, $ty);
			set_default!(@box_model $name, $ty);
			fn to_attr(ptr: *const u8) -> Attribute
			{
				Attribute::$ty(unsafe { $struct_name(ptr.cast::<$ty>().read_unaligned()) })
			}
		}

		$crate::paste::item! {
			impl ConvertToComponent for [<Reset $struct_name>] {
				reset!(@box_model $name, $ty);
				set_default!(@box_model $name, $ty);
				fn to_attr(_ptr: *const u8) -> Attribute
				{
					Attribute::$ty( $struct_name(Default::default()) )
				}
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
    // impl_style!(WhiteSpaceType, text_style, white_space, WhiteSpace, WhiteSpace);
	impl ConvertToComponent for WhiteSpaceType {
		// 设置white_space,需要同时设置flex_wrap
		fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			// 取不到说明实体已经销毁
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);

			let v = unsafe { ptr.cast::<WhiteSpace>().read_unaligned() };
			log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);

			cur_style_mark.set(Self::get_type() as usize, true);
			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

			let component = text_style_item.get_mut_or_default();
			component.white_space = v;
			text_style_item.notify_modify();
				
			let component = flex_container_item.get_mut_or_default();
			component.flex_wrap = if v.allow_wrap() {
                FlexWrap::Wrap
            } else {
                FlexWrap::NoWrap
            };
			flex_container_item.notify_modify();
		}

		set_default!(text_style, white_space, WhiteSpace);
		fn to_attr(ptr: *const u8) -> Attribute{
			Attribute::WhiteSpace(unsafe { WhiteSpaceType(ptr.cast::<WhiteSpace>().read_unaligned()) })
		}
	}

	impl ConvertToComponent for ResetWhiteSpaceType {
		fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);
			let v = text_style_item.get_default().white_space.clone();
			let component = text_style_item.get_mut_or_default();
			component.white_space = v;
			text_style_item.notify_modify();

			let component = flex_container_item.get_mut_or_default();
			component.flex_wrap = if v.allow_wrap() {
                FlexWrap::Wrap
            } else {
                FlexWrap::NoWrap
            };
			flex_container_item.notify_modify();
		}

		set_default!(text_style, white_space, WhiteSpace);
		fn to_attr(_ptr: *const u8) -> Attribute{
			todo!()
			// Attribute::WhiteSpace(unsafe { WhiteSpaceType(ptr.cast::<WhiteSpace>().read_unaligned()) })
		}
	}

    impl_style!(TextContentType, text_content, TextContent);
    // impl_style!(TextAlignType, text_style, text_align, TextAlign, TextAlign);

	impl ConvertToComponent for TextAlignType {
		// 设置text_align,需要同时设置justify_content
		fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			// 取不到说明实体已经销毁
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);

			let v = unsafe { ptr.cast::<TextAlign>().read_unaligned() };
			log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);

			cur_style_mark.set(Self::get_type() as usize, true);
			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

			let component = text_style_item.get_mut_or_default();
			component.text_align = v;
			text_style_item.notify_modify();
				
			let component = flex_container_item.get_mut_or_default();
			component.justify_content = match v {
				TextAlign::Center => JustifyContent::Center,
                TextAlign::Right => JustifyContent::FlexEnd,
                TextAlign::Left => JustifyContent::FlexStart,
                TextAlign::Justify => JustifyContent::SpaceBetween,
			};
			flex_container_item.notify_modify();
		}

		set_default!(text_style, text_align, TextAlign);
		fn to_attr(ptr: *const u8) -> Attribute{
			Attribute::TextAlign(unsafe { TextAlignType(ptr.cast::<TextAlign>().read_unaligned()) })
		}
	}

	impl ConvertToComponent for ResetTextAlignType {
		fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);
			let v = text_style_item.get_default().text_align.clone();
			let component = text_style_item.get_mut_or_default();
			component.text_align = v;
			text_style_item.notify_modify();

			let component = flex_container_item.get_mut_or_default();
			component.justify_content = match v {
				TextAlign::Center => JustifyContent::Center,
                TextAlign::Right => JustifyContent::FlexEnd,
                TextAlign::Left => JustifyContent::FlexStart,
                TextAlign::Justify => JustifyContent::SpaceBetween,
			};
			flex_container_item.notify_modify();
		}

		set_default!(text_style, text_align, TextAlign);
		fn to_attr(_ptr: *const u8) -> Attribute{
			todo!()
		}
	}

    // impl_style!(VerticalAlignType, text_style, vertical_align, VerticalAlign, VerticalAlign);
	impl ConvertToComponent for VerticalAlignType {
		// 设置vertical_align,需要同时设置jalign_items, align_content
		fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			// 取不到说明实体已经销毁
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);

			let v = unsafe { ptr.cast::<VerticalAlign>().read_unaligned() };
			log::info!("set_style, type: {:?}, value: {:?}, entity: {:?}", std::any::type_name::<Self>(), v, entity);

			cur_style_mark.set(Self::get_type() as usize, true);
			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

			let component = text_style_item.get_mut_or_default();
			component.vertical_align = v;
			text_style_item.notify_modify();
				
			let component = flex_container_item.get_mut_or_default();
			component.align_content = match v {
				VerticalAlign::Middle => AlignContent::Center,
				VerticalAlign::Bottom => AlignContent::FlexEnd,
				VerticalAlign::Top => AlignContent::FlexStart,
			};
			component.align_items = match v {
				VerticalAlign::Middle => AlignItems::Center,
				VerticalAlign::Bottom => AlignItems::FlexEnd,
				VerticalAlign::Top => AlignItems::FlexStart,
			};
			flex_container_item.notify_modify();
		}

		set_default!(text_style, vertical_align, VerticalAlign);
		fn to_attr(ptr: *const u8) -> Attribute{
			Attribute::VerticalAlign(unsafe { VerticalAlignType(ptr.cast::<VerticalAlign>().read_unaligned()) })
		}
	}

	impl ConvertToComponent for ResetVerticalAlignType {
		fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut StyleQuery, entity: Id<Node>)
			where
				Self: Sized {
			let mut text_style_item = query.text_style.get_unchecked_mut(entity);
			let mut flex_container_item = query.flex_container.get_unchecked_mut(entity);
			let v = text_style_item.get_default().vertical_align.clone();
			let component = text_style_item.get_mut_or_default();
			component.vertical_align = v;
			text_style_item.notify_modify();

			let component = flex_container_item.get_mut_or_default();
			component.align_content = match v {
				VerticalAlign::Middle => AlignContent::Center,
				VerticalAlign::Bottom => AlignContent::FlexEnd,
				VerticalAlign::Top => AlignContent::FlexStart,
			};
			component.align_items = match v {
				VerticalAlign::Middle => AlignItems::Center,
				VerticalAlign::Bottom => AlignItems::FlexEnd,
				VerticalAlign::Top => AlignItems::FlexStart,
			};
			flex_container_item.notify_modify();
		}

		set_default!(text_style, vertical_align, VerticalAlign);
		fn to_attr(_ptr: *const u8) -> Attribute{
			todo!()
		}
	}
	
    impl_style!(ColorType, text_style, color, Color, Color);
    impl_style!(TextStrokeType, text_style, text_stroke, TextStroke, Stroke);
    impl_style!(TextShadowType, text_style, text_shadow, TextShadow, SmallVec<[TextShadow; 1]>);

    impl_style!(@pack BackgroundImageType, background_image, BackgroundImage, Atom);
    impl_style!(@pack BackgroundImageClipType, background_image_clip, BackgroundImageClip, NotNanRect);
    impl_style!(ObjectFitType, background_image_mod, object_fit, ObjectFit, FitType);
    impl_style!(BackgroundRepeatType, background_image_mod, repeat, BackgroundRepeat, ImageRepeat);

    impl_style!(@pack BorderImageType, border_image, BorderImage, Atom);
    impl_style!(@pack BorderImageClipType, border_image_clip, BorderImageClip, NotNanRect);
    impl_style!(BorderImageSliceType, border_image_slice, BorderImageSlice);
    impl_style!(@pack BorderImageRepeatType, border_image_repeat, BorderImageRepeat, ImageRepeat);

    impl_style!(@pack BorderColorType, border_color, BorderColor, CgColor);

    impl_style!(@pack BackgroundColorType, background_color, BackgroundColor, Color);

    impl_style!(BoxShadowType, box_shadow, BoxShadow);

    impl_style!(@pack OpacityType, opacity, Opacity, f32);
    impl_style!(BorderRadiusType, border_radius, BorderRadius);
    impl_style!(HsiType, hsi, Hsi);
    impl_style!(@pack BlurType, blur, Blur, f32);
    impl_style!(TransformOriginType, transform, origin, TransformOrigin, TransformOrigin);
    impl_style!(TransformType, transform, funcs, Transform, TransformFuncs);
    impl_style!(DirectionType, flex_container, direction, Direction, Direction);
    impl_style!(AspectRatioType, flex_normal, aspect_ratio, AspectRatio, Number);
    impl_style!(OrderType, flex_normal, order, Order, isize);
    impl_style!(FlexBasisType, flex_normal, flex_basis, FlexBasis, Dimension);


    impl_style!(@func DisplayType, show, set_display, get_display, Display, Display);
    impl_style!(@func VisibilityType, show, set_visibility, get_visibility, Visibility, bool);
    impl_style!(@func EnableType, show, set_enable, get_enable, Enable, Enable);

    impl_style!(@func1 TransformFuncType, transform, add_func, TransformFunc, TransformFunc, TransformFunc);
	impl_style!(@func1 VNodeType, node_state, set_vnode, NodeState, VNode, bool);
    // impl_style!(@func VNodeType, node_state, set_vnode, NodeState, bool);

    impl_style!(@pack TransformWillChangeType, transform_will_change, TransformWillChange, TransformFuncs);

    impl_style!(@pack ZIndexType, z_index, ZIndex, isize);
    impl_style!(@pack OverflowType, overflow, Overflow, bool);

    impl_style!(MaskImageType, mask_image, MaskImage);
    impl_style!(@pack MaskImageClipType, mask_image_clip, MaskImageClip, NotNanRect);

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
		to_attr: fn(ptr: *const u8) -> Attribute,
    }

    impl StyleFunc {
        fn new<T: ConvertToComponent>() -> StyleFunc {
            StyleFunc {
                get_type: T::get_type,
                // get_style_index: T::get_style_index,
                size: T::size,
                set: T::set,
                set_default: T::set_default,
				to_attr: T::to_attr,
                // add: T::add,
                // scale: T::scale,
            }
        }
    }

    lazy_static::lazy_static! {

        static ref STYLE_ATTR: [StyleFunc; 173] = [
            StyleFunc::new::<EmptyType>(), // 0 empty 占位， 无实际作用
            StyleFunc::new::<BackgroundRepeatType>(), // 1
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
        pub fn set(
            cur_style_mark: &mut BitArray<[u32; 3]>,
            style_index: u8,
            buffer: &Vec<u8>,
            offset: usize,
            query: &mut StyleQuery,
            entity: Id<Node>,
        ) {
            (STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity)
        }

		#[inline]
        pub fn to_attr(style_index: u8, buffer: &Vec<u8>, offset: usize) -> Attribute {
            (STYLE_ATTR[style_index as usize].to_attr)(unsafe { buffer.as_ptr().add(offset) })
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
}
