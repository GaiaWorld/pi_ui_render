//! 定义样式表

use std::mem::forget;

use bitvec::array::BitArray;
use pi_ecs::{entity::Id, prelude::{Query, Write}};
use pi_flex_layout::{style::{Dimension, Direction, JustifyContent, FlexDirection, AlignItems, AlignContent, FlexWrap, AlignSelf, PositionType as PositionType1, Display}, prelude::Number};
use pi_hash::XHashMap;

use crate::components::{user::{
	Node, Size, Margin, Padding, Position, Border, MinMax, FlexContainer, FlexNormal, ZIndex, Overflow, Opacity, BlendMode, Transform, Show, BackgroundColor, BorderColor, BackgroundImage, MaskImage, MaskImageClip, Hsi, Blur, ObjectFit, BackgroundImageClip, BorderImage, BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderRadius, BoxShadow, TextStyle, TransformOrigin, FontSize, FontStyle, LineHeight, TextAlign, VerticalAlign, Color, Stroke, TextShadows, TransformFuncs, WhiteSpace, Enable
}, calc::StyleType};

// 全局Class样式表
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClassSheet {
	pub style_buffer: Vec<u8>, // 所有class样式的buffer集合
    pub class_map: XHashMap<usize, ClassMeta>, // 每个class的元信息描述
}

/// class样式
/// 该类型单独存在没有意义，它与ClassSheet结合起来使用，用于描述该class的有效属性类型以及属性在classSheet中的位置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClassMeta {
	pub start: usize, // 在某个buffer中的开始偏移
	pub end: usize, // 在某个buffer中的结束偏移
    pub class_style_mark: BitArray<[u32;3]>,  // 标记class中的有效属性
}

/// 从Buffer中读取StyleType
pub struct StyleTypeReader<'a> {
	buffer: &'a Vec<u8>,
	cursor: usize,
	end: usize,
}

impl<'a> StyleTypeReader<'a> {
	pub fn default(buffer: &Vec<u8>) -> StyleTypeReader {
		StyleTypeReader {
			buffer,
			cursor: 0,
			end: buffer.len()
		}
	}

	pub fn new(buffer: &Vec<u8>, start: usize, end: usize) -> StyleTypeReader {
		StyleTypeReader {
			buffer,
			cursor: start,
			end,
		}
	}

	// 将当前style写入组件
	pub fn write_to_component(&mut self, entity: Id<Node>, query: &mut StyleQuery) -> Option<StyleType>{
		let next_type = self.next_type();
		// log::info!("write_to_component ty: {:?}, cursor:{}, buffer_len:{}", next_type, self.cursor, self.buffer.len());
		if let Some(style_type) = next_type {
			StyleAttr::set(style_type, &self.buffer, self.cursor, query, entity);
			let size = StyleAttr::size(style_type);
			self.cursor += size;
		}
		
		next_type
	}

	// f函数返回true，则写入到组件，否则不写入,跳过该属性
	pub fn or_write_to_component<F: Fn(StyleType) -> bool>(&mut self, entity: Id<Node>, query: &mut StyleQuery, f: F) -> Option<StyleType>{
		let next_type = self.next_type();
		if let Some(style_type) = next_type {
			if f(style_type) {
				StyleAttr::set(style_type, &self.buffer, self.cursor, query, entity)
			}
			let size = StyleAttr::size(style_type);
			self.cursor += size;
		}
		next_type
	}

	// 读下一个样式类型
	fn next_type(&mut self) -> Option<StyleType> {
		if self.cursor >= self.end {
			return None;
		}

		let ty_size = std::mem::size_of::<StyleType>();
		let ty = unsafe {Some(self.buffer.as_ptr().add(self.cursor).cast::<StyleType>().read_unaligned())};

		// log::info!("next_type ty: {:?}, type_size:{:?}", ty, ty_size);
		self.cursor += ty_size;
		ty
	}
}

pub trait Attr: 'static + Sync + Send {
	fn get_type(&self) -> StyleType;
	fn size(&self) -> usize;
	/// 序列化自身到buffer中
	unsafe fn write(self, buffer: &mut Vec<u8>);
	/// 安全： entity必须存在
	fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>);
	/// 安全： entity必须存在
	fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>);
}

macro_rules! get_type {
    ($key: ident) => {
        #[inline]
		fn get_type(&self) -> StyleType {
			StyleType::$key
		}
    };
}

macro_rules! size {
    ($value_ty: ident) => {
        #[inline]
		fn size(&self) -> usize {
			std::mem::size_of::<$value_ty>()
		}
    };
}

macro_rules! write_buffer {
    () => {
        unsafe fn write(self, buffer: &mut Vec<u8>) {
			let ty_size = std::mem::size_of::<StyleType>();
			let value_size = self.size();
			let len = buffer.len();
			buffer.reserve(ty_size + value_size);
			buffer.set_len(len + ty_size + value_size);
	
			let ty = self.get_type();
			std::ptr::copy_nonoverlapping(
				&ty as *const StyleType as *const u8,
				buffer.as_mut_ptr().add(len),
				ty_size,
			);
			
			std::ptr::copy_nonoverlapping(
				&self as *const Self as usize as *const u8,
				buffer.as_mut_ptr().add(len + ty_size),
				value_size,
			);
			forget(self)
		}
	};
}

macro_rules! set {
	// 整体插入
    ($name: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			item.write(v);
		}
	};
	// 属性修改
	($name: ident, $feild: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			let component = item.get_mut_or_default();
			component.$feild = v;
			item.notify_modify();
		}
	};
	// 属性修改
	(@func $name: ident, $set_func: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			let component = item.get_mut_or_default();
			component.$set_func(v);
			item.notify_modify();
		}
	};

	// 属性修改
	($name: ident, $feild1: ident, $feild2: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			let component = item.get_mut_or_default();
			component.$feild1.$feild2 = v;
			item.notify_modify();
		}
	};
	
	// 盒模属性（上右下左）
	(@box_model $name: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			let component = item.get_mut_or_default();
			component.top = v.top;
			component.right = v.right;
			component.bottom = v.bottom;
			component.left = v.left;
			item.notify_modify();
		}
	};
}

macro_rules! reset {
	($name: ident, $value_ty: ident) => {
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = item.get_default().clone();
			item.write(v);
		}
	};
	// 属性修改
	($name: ident, $feild: ident) => {
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
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
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
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
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = item.get_default().$feild1.$feild2.clone();
			let component = item.get_mut_or_default();
			component.$feild1.$feild2 = v;
			item.notify_modify();
		}
	};
	// 属性修改
	(@box_model_single $name: ident, $feild: ident, $ty_all: ident) => {
        fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
			// 单个盒模型属性重置
			// 如：重置MarginLeft，只有在没有设置Margin属性的时候才能够重置
			if cur_style_mark[StyleType::$ty_all as usize] {
				return;
			}
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = item.get_default().$feild.clone();
			let component = item.get_mut_or_default();
			component.$feild = v;
			item.notify_modify();
		}
	};

	(@box_model $name: ident, $ty: ident) => {
        fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
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
		pub struct $struct_name(pub $ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($ty);
			write_buffer!();
			set!($name, $ty);
			reset!($name, $ty);
		}
	};
	($struct_name: ident, $name: ident, $ty: ident, $value_ty: ident) => {
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $value_ty);
			reset!($name);
		}
	};
	($struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			reset!($name, $feild);
		}
	};
	($struct_name: ident, $name: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild1, $feild2, $value_ty);
			reset!($name, $feild1, $feild2);
		}
	};
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!(@func $name, $set_func, $value_ty);
			reset!(@func $name, $set_func, $get_func);
		}
	};
	(@box_model_single $struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident, $ty_all: ident) => {
		pub struct $struct_name(pub $value_ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			reset!(@box_model_single $name, $feild, $ty_all);
		}
	};
	(@box_model $struct_name: ident, $name: ident, $ty: ident) => {
		pub struct $struct_name(pub $ty);
		impl Attr for $struct_name {
			get_type!($ty);
			size!($ty);
			write_buffer!();
			set!(@box_model $name, $ty);
			reset!(@box_model $name, $ty);
		}
	};
}

// impl TextType {
// 	impl_style!(text, );
// }

impl_style!(FontStyleType, text_style, font_style, FontStyle, FontStyle);

impl_style!(FontWeightType, text_style, font_weight, FontWeight, usize);
impl_style!(FontSizeType, text_style, font_size, FontSize, FontSize);
impl_style!(FontFamilyType, text_style, font_family, FontFamily, usize);
impl_style!(LetterSpacingType, text_style, letter_spacing, LetterSpacing, f32);
impl_style!(WordSpacingType, text_style, word_spacing, WordSpacing, f32);
impl_style!(LineHeightType, text_style, line_height, LineHeight, LineHeight);
impl_style!(TextIndentType, text_style, text_indent, TextIndent, f32);
impl_style!(WhiteSpaceType, text_style, white_space, WhiteSpace, WhiteSpace);

impl_style!(TextAlignType, text_style, text_align, TextAlign, TextAlign);
impl_style!(VerticalAlignType, text_style, vertical_align, VerticalAlign, VerticalAlign);
impl_style!(ColorType, text_style, color, Color, Color);
impl_style!(TextStrokeType, text_style, text_stroke, TextStroke, Stroke);
impl_style!(TextShadowType, text_style, text_shadow, TextShadow, TextShadows);

impl_style!(BackgroundImageType, background_image, BackgroundImage);
impl_style!(BackgroundImageClipType, background_image_clip, BackgroundImageClip);
impl_style!(ObjectFitType, object_fit, ObjectFit);

impl_style!(BorderImageType, border_image, BorderImage);
impl_style!(BorderImageClipType, border_image_clip, BorderImageClip);
impl_style!(BorderImageSliceType, border_image_slice, BorderImageSlice);
impl_style!(BorderImageRepeatType, border_image_repeat, BorderImageRepeat);

impl_style!(BorderColorType, border_color, BorderColor);

impl_style!(BackgroundColorType, background_color, BackgroundColor);

impl_style!(BoxShadowType, box_shadow, BoxShadow);

// impl_style!(MatrixType, text_style, font_style, Matrix);
impl_style!(OpacityType, opacity, Opacity);
// impl_style!(LayoutType, text_style, font_style, Layout);
impl_style!(BorderRadiusType, border_radius, BorderRadius);
// impl_style!(ByOverflowType, text_style, font_style, ByOverflow);
impl_style!(HsiType, hsi, Hsi);
// impl_style!(OctType, text_style, font_style, Oct);
impl_style!(BlurType, blur, Blur);
// impl_style!(BorderImageTextureType, text_style, font_style, BorderImageTexture);
// impl_style!(ImageTextureType, text_style, font_style, ImageTexture);
impl_style!(TransformOriginType, transform, origin, TransformOrigin, TransformOrigin);
impl_style!(TransformType, transform, funcs, Transform, TransformFuncs);
// impl_style!(ContentBoxType, contente, font_style, ContentBox);
impl_style!(DirectionType, flex_container, direction, Direction, Direction);
impl_style!(AspectRatioType, flex_normal, aspect_ratio, AspectRatio, Number);
impl_style!(OrderType, flex_normal, order, Order, isize);
impl_style!(FlexBasisType, flex_normal, flex_basis, FlexBasis, Dimension);

// pub flex_direction: FlexDirection,
//     pub flex_wrap: FlexWrap,
//     pub justify_content: JustifyContent,
//     pub align_items: AlignItems,
//     pub align_content: AlignContent,
// 	pub direction: Direction,

// impl_style!(CreateType, text_style, font_style, Create);
// impl_style!(DeleteType, text_style, font_style, Delete);

impl_style!(@func DisplayType, show, set_display, get_display, Display, Display);
impl_style!(@func VisibilityType, show, set_visibility, get_visibility, Visibility, bool);
impl_style!(@func EnableType, show, set_enable, get_enable, Enable, Enable);

// TODO TransformWillChangeType
// impl_style!(TransformWillChangeType, transform_will_change, font_style, TransformWillChange);

impl_style!(ZIndexType, z_index, ZIndex);
impl_style!(OverflowType, overflow, Overflow);

impl_style!(MaskImageType, mask_image, MaskImage);
impl_style!(MaskImageClipType, mask_image_clip, MaskImageClip);
// impl_style!(MaskTextureType, text_style, font_style, MaskTexture);

impl_style!(WidthType, size, width, Width, Dimension);
impl_style!(HeightType, size, height, Height, Dimension);



impl_style!(@box_model_single MarginTopType, margin, top, MarginTop, Dimension, Margin);
impl_style!(@box_model_single MarginRightType, margin, right, MarginRight, Dimension, Margin);
impl_style!(@box_model_single MarginBottomType, margin, bottom, MarginBottom, Dimension, Margin);
impl_style!(@box_model_single MarginLeftType, margin, left, MarginLeft, Dimension, Margin);

impl_style!(@box_model_single PaddingTopType, padding, top, PaddingTop, Dimension, Padding);
impl_style!(@box_model_single PaddingRightType, padding, right, PaddingRight, Dimension, Padding);
impl_style!(@box_model_single PaddingBottomType, padding, bottom, PaddingBottom, Dimension, Padding);
impl_style!(@box_model_single PaddingLeftType, padding, left, PaddingLeft, Dimension, Padding);

impl_style!(@box_model_single BorderTopType, border, top, BorderTop, Dimension, Border);
impl_style!(@box_model_single BorderRightType, border, right, BorderRight, Dimension, Border);
impl_style!(@box_model_single BorderBottomType, border, bottom, BorderBottom, Dimension, Border);
impl_style!(@box_model_single BorderLeftType, border, left, BorderLeft, Dimension, Border);

impl_style!(@box_model_single PositionTopType, position, top, PositionTop, Dimension, Position);
impl_style!(@box_model_single PositionRightType, position, right, PositionRight, Dimension, Position);
impl_style!(@box_model_single PositionBottomType, position, bottom, PositionBottom, Dimension, Position);
impl_style!(@box_model_single PositionLeftType, position, left, PositionLeft, Dimension, Position);

impl_style!(MinWidthType, min_max, min, width, MinWidth, Dimension);
impl_style!(MinHeightType, min_max, min, height, MinHeight, Dimension);
impl_style!(MaxHeightType, min_max, max, height, MaxHeight, Dimension);
impl_style!(MaxWidthType, min_max, max, width, MaxWidth, Dimension);
impl_style!(JustifyContentType, flex_container, justify_content, JustifyContent, JustifyContent);
impl_style!(FlexDirectionType, flex_container, flex_direction, FlexDirection, FlexDirection);
impl_style!(AlignContentType, flex_container, align_content, AlignContent, AlignContent);
impl_style!(AlignItemsType, flex_container, align_items, AlignItems, AlignItems);
impl_style!(FlexWrapType, flex_container, flex_wrap, FlexWrap, FlexWrap);



lazy_static! {
	static ref STYLE_ATTR: [Box<dyn Attr>; 81] = [
		Box::new(PaddingTopType(Dimension::default())), // 0 empty
		Box::new(PaddingTopType(Dimension::default())), // 1 text
		Box::new(FontStyleType(FontStyle::default())), // 2
		Box::new(FontWeightType(usize::default())), // 3
		Box::new(FontSizeType(FontSize::default())), // 4
		Box::new(FontFamilyType(usize::default())), // 5
		Box::new(LetterSpacingType(f32::default())), // 6
		Box::new(WordSpacingType(f32::default())), // 7
		Box::new(LineHeightType(LineHeight::default())), // 8
		Box::new(TextIndentType(f32::default())), // 9
		Box::new(WhiteSpaceType(WhiteSpace::default())), // 10

		Box::new(TextAlignType(TextAlign::default())), // 11
		Box::new(VerticalAlignType(VerticalAlign::default())), // 12
		Box::new(ColorType(Color::default())), // 13
		Box::new(TextStrokeType(Stroke::default())), // 14
		Box::new(TextShadowType(TextShadows::default())), // 15
		
		Box::new(BackgroundImageType(BackgroundImage::default())), // 16
		Box::new(BackgroundImageClipType(BackgroundImageClip::default())), // 17
		Box::new(ObjectFitType(ObjectFit::default())), // 18
		Box::new(BackgroundColorType(BackgroundColor::default())), // 19
		Box::new(BoxShadowType(BoxShadow::default())), // 20
		Box::new(BorderImageType(BorderImage::default())), // 21
		Box::new(BorderImageClipType(BorderImageClip::default())), // 22
		Box::new(BorderImageSliceType(BorderImageSlice::default())), // 23
		Box::new(BorderImageRepeatType(BorderImageRepeat::default())), // 24

		Box::new(BorderColorType(BorderColor::default())), // 25
		

		Box::new(HsiType(Hsi::default())), // 26
		Box::new(BlurType(Blur::default())), // 27
		Box::new(MaskImageType(MaskImage::default())), // 28
		Box::new(MaskImageClipType(MaskImageClip::default())), // 29
		Box::new(MaskImageClipType(MaskImageClip::default())), // 30 MaskTexture
		Box::new(TransformType(TransformFuncs::default())), // 31
		Box::new(TransformOriginType(TransformOrigin::default())), // 32
		Box::new(BorderRadiusType(BorderRadius::default())), // 33 transformwillchange
		Box::new(BorderRadiusType(BorderRadius::default())), // 34
		Box::new(ZIndexType(ZIndex::default())), // 35
		Box::new(OverflowType(Overflow::default())), // 36
		
		
		Box::new(BlendModeType(BlendMode::default())), // 37
		Box::new(DisplayType(Display::default())), // 38
		Box::new(VisibilityType(bool::default())), // 39
		Box::new(EnableType(Enable::default())), // 40

		
		Box::new(WidthType(Dimension::default())), // 41
		Box::new(HeightType(Dimension::default())), // 42

		Box::new(MarginTopType(Dimension::default())), // 43
		Box::new(MarginRightType(Dimension::default())), // 44
		Box::new(MarginBottomType(Dimension::default())), // 45
		Box::new(MarginLeftType(Dimension::default())), // 46

		Box::new(PaddingTopType(Dimension::default())), // 47
		Box::new(PaddingRightType(Dimension::default())), // 48
		Box::new(PaddingBottomType(Dimension::default())), // 49
		Box::new(PaddingLeftType(Dimension::default())), // 50

		Box::new(BorderTopType(Dimension::default())), // 51
		Box::new(BorderRightType(Dimension::default())), // 52
		Box::new(BorderBottomType(Dimension::default())), // 53
		Box::new(BorderLeftType(Dimension::default())), // 54

		Box::new(PositionTopType(Dimension::default())), // 55
		Box::new(PositionRightType(Dimension::default())), // 56
		Box::new(PositionBottomType(Dimension::default())), // 57
		Box::new(PositionLeftType(Dimension::default())), // 58

		Box::new(MinWidthType(Dimension::default())), // 59
		Box::new(MinHeightType(Dimension::default())), // 60
		Box::new(MaxHeightType(Dimension::default())), // 61
		Box::new(MaxWidthType(Dimension::default())), // 62
		Box::new(DirectionType(Direction::default())), // 63
		Box::new(FlexDirectionType(FlexDirection::default())), // 64
		Box::new(FlexWrapType(FlexWrap::default())), // 65
		Box::new(JustifyContentType(JustifyContent::default())), // 66
		Box::new(AlignContentType(AlignContent::default())), // 67
		Box::new(AlignItemsType(AlignItems::default())), // 68
		

		Box::new(PositionTypeType(PositionType1::default())), // 69
		Box::new(AlignSelfType(AlignSelf::default())), // 70
		Box::new(FlexShrinkType(f32::default())), // 71
		Box::new(FlexGrowType(f32::default())), // 72
		Box::new(AspectRatioType(Number::default())), // 73
		Box::new(OrderType(isize::default())), // 74
		Box::new(FlexBasisType(Dimension::default())), // 75
		Box::new(PositionType(Position::default())), // 76
		Box::new(BorderType(Border::default())), // 77
		Box::new(MarginType(Margin::default())), // 78
		Box::new(PaddingType(Padding::default())), // 79
		Box::new(OpacityType(Opacity::default())), // 80
		
		

		
	];
}
impl_style!(FlexShrinkType, flex_normal, flex_shrink, FlexShrink, f32);
impl_style!(FlexGrowType, flex_normal, flex_grow, FlexGrow, f32);
impl_style!(PositionTypeType, flex_normal, position_type, PositionType, PositionType1);
impl_style!(AlignSelfType, flex_normal, align_self, AlignSelf, AlignSelf);

impl_style!(BlendModeType, blend_mode, BlendMode);

// 设置Position、Border、Margin、Padding的优先级比单独设置上右下左的优先级要低，所以有单独的标识，
// 假定Position属性的设置，直接作用到上由下左上，可能会覆盖单独设置的上右下左属性
impl_style!(@box_model PositionType, position, Position);
impl_style!(@box_model BorderType, border, Border);
impl_style!(@box_model MarginType, margin, Margin);
impl_style!(@box_model PaddingType, padding, Padding);

pub struct StyleQuery {
	pub size: Query<Node, Write<Size>>,
	pub margin: Query<Node, Write<Margin>>,
	pub padding: Query<Node, Write<Padding>>,
	pub border: Query<Node, Write<Border>>,
	pub position: Query<Node, Write<Position>>,
	pub min_max: Query<Node, Write<MinMax>>,
	pub flex_container: Query<Node, Write<FlexContainer>>,
	pub flex_normal: Query<Node, Write<FlexNormal>>,
	pub z_index: Query<Node, Write<ZIndex>>,
	pub overflow: Query<Node, Write<Overflow>>,
	pub opacity: Query<Node, Write<Opacity>>,
	pub blend_mode: Query<Node, Write<BlendMode>>,
	pub show: Query<Node, Write<Show>>,
	pub transform: Query<Node, Write<Transform>>,
	pub background_color: Query<Node, Write<BackgroundColor>>,
	pub border_color: Query<Node, Write<BorderColor>>,
	pub background_image: Query<Node, Write<BackgroundImage>>,
	pub background_image_clip: Query<Node, Write<BackgroundImageClip>>,
	pub mask_image: Query<Node, Write<MaskImage>>,
	pub mask_image_clip: Query<Node, Write<MaskImageClip>>,
	pub hsi: Query<Node, Write<Hsi>>,
	pub blur: Query<Node, Write<Blur>>,
	pub object_fit: Query<Node, Write<ObjectFit>>,
	pub border_image: Query<Node, Write<BorderImage>>,
	pub border_image_clip: Query<Node, Write<BorderImageClip>>,
	pub border_image_slice: Query<Node, Write<BorderImageSlice>>,
	pub border_image_repeat: Query<Node, Write<BorderImageRepeat>>,
	pub border_radius: Query<Node, Write<BorderRadius>>,
	pub box_shadow: Query<Node, Write<BoxShadow>>,
	pub text_style: Query<Node, Write<TextStyle>>,
}

pub struct StyleAttr;

impl StyleAttr {
	#[inline]
	pub fn get_attr(style_type: StyleType) -> &'static Box<dyn Attr> {
		&STYLE_ATTR[style_type as usize]
	}

	#[inline]
	pub unsafe fn write<T: Attr>(value: T, buffer: &mut Vec<u8>) {
		// let start = buffer.len();
		value.write(buffer);
		// log::info!("write style: {:?}, start:{:?}, end: {}", std::any::type_name::<T>(), start, buffer.len());
	}

	#[inline]
	pub fn set(style_type: StyleType, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Id<Node>) {
		STYLE_ATTR[style_type as usize].set(buffer, offset, query, entity);
	}

	#[inline]
	pub fn size(style_type: StyleType) -> usize {
		STYLE_ATTR[style_type as usize].size()
	}

	#[inline]
	pub fn reset(style_type: StyleType, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Id<Node>) {
		STYLE_ATTR[style_type as usize].reset(cur_style_mark, query, entity);
	}
}



