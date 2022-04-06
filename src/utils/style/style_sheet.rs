//! 定义样式表

use bitvec::array::BitArray;
use pi_ecs::{entity::Entity, prelude::{Query, Write}};
use pi_flex_layout::{style::{Dimension, Direction, JustifyContent, FlexDirection, AlignItems, AlignContent, FlexWrap, AlignSelf, PositionType as PositionType1, Display}, prelude::Number};
// use pi_flex_layout::style::AlignItems;
use pi_hash::XHashMap;

use crate::{components::{user::{
	Node, Size, Margin, Padding, Position, Border, MinMax, FlexContainer, FlexNormal, ZIndex, Overflow, Opacity, BlendMode, Transform, Show, BackgroundColor, BorderColor, BackgroundImage, MaskImage, MaskImageClip, Hsi, Blur, ObjectFit, BackgroundImageClip, BorderImage, BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderRadius, BoxShadow, TextStyle, TransformOrigin, FontSize, FontStyle, LineHeight, TextAlign, VerticalAlign, Color, Stroke, TextShadows, TransformFuncs, WhiteSpace, Enable
}, calc::StyleType}};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Class {
	// pub attrs: Vec<Attribute>,
	// pub attrs: Vec<u8>, // Vec<(StyleType, Attr)>
	pub start: usize,
	pub end: usize,
    pub class_style_mark: BitArray<[u32;3]>,  // 标记class中的有效属性
}

// 全局Class表
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClassSheet {
	pub style_buffer: Vec<u8>,
    pub class_map: XHashMap<usize, Class>,
}

pub struct StyleReader<'a> {
	buffer: &'a Vec<u8>,
	start: usize,
	end: usize,
}

impl<'a> StyleReader<'a> {
	pub fn default(buffer: &Vec<u8>) -> StyleReader {
		StyleReader {
			buffer,
			start: 0,
			end: buffer.len()
		}
	}

	/// 
	pub fn new(buffer: &Vec<u8>, start: usize, end: usize) -> StyleReader {
		StyleReader {
			buffer,
			start,
			end,
		}
	}
	// 下一个类型
	pub fn next_type(&mut self) -> Option<StyleType> {
		if self.start >= self.end {
			return None;
		}

		let ty_size = std::mem::size_of::<StyleType>();
		let ty = unsafe {Some(self.buffer.as_ptr().add(self.start).cast::<StyleType>().read_unaligned())};
		self.start += ty_size;
		ty
	}

	// 设置属性
	pub fn set(&mut self, style_type: StyleType, query: &mut StyleQuery, entity: Entity){
		StyleAttr::set(style_type, &self.buffer, self.start, query, entity)
	}
}

pub struct TextType;
pub struct FontStyleType;
pub struct FontWeightType;
pub struct FontSizeType;
pub struct FontFamilyType;
pub struct LetterSpacingType;
pub struct WordSpacingType;
pub struct LineHeightType;
pub struct TextIndentType;
pub struct WhiteSpaceType;
pub struct TextAlignType;
pub struct VerticalAlignType;
pub struct ColorType;
pub struct TextStrokeType;
pub struct TextShadowType;

pub struct BackGroundImageType;
pub struct BackGroundImageClipType;
pub struct ObjectFitType;

pub struct BorderImageType;
pub struct BorderImageClipType;
pub struct BorderImageSliceType;
pub struct BorderImageRepeatType;

pub struct BorderColorType;

pub struct BackgroundColorType;

pub struct BoxShadowType;

pub struct MatrixType;
pub struct OpacityType;
pub struct LayoutType;
pub struct BorderRadiusType;
pub struct ByOverflowType;
pub struct HsiType;
pub struct OctType;
pub struct BlurType;
pub struct BorderImageTextureType;
pub struct ImageTextureType;
pub struct TransformOriginType;
pub struct ContentBoxType;
pub struct DirectionType;
pub struct AspectRatioType;
pub struct OrderType;
pub struct FlexBasisType;

pub struct DisplayType;
pub struct VisibilityType;
pub struct EnableType;
pub struct ZIndexType;
pub struct TransformType;
pub struct TransformWillChangeType;
pub struct OverflowType;

pub struct CreateType;
pub struct DeleteType;

pub struct MaskImageType;
pub struct MaskImageClipType;
pub struct MaskTextureType;

pub struct WidthType;
pub struct HeightType;

pub struct MarginTopType;
pub struct MarginRightType;
pub struct MarginBottomType;
pub struct MarginLeftType;

pub struct PaddingTopType;
pub struct PaddingRightType;
pub struct PaddingBottomType;
pub struct PaddingLeftType;

pub struct BorderTopType;
pub struct BorderRightType;
pub struct BorderBottomType;
pub struct BorderLeftType;

pub struct PositionTopType;
pub struct PositionRightType;
pub struct PositionBottomType;
pub struct PositionLeftType;

pub struct MinWidthType;
pub struct MinHeightType;
pub struct MaxHeightType;
pub struct MaxWidthType;
pub struct JustifyContentType;
pub struct FlexShrinkType;
pub struct FlexGrowType;
pub struct PositionTypeType;
pub struct FlexWrapType;
pub struct FlexDirectionType;
pub struct AlignContentType;
pub struct AlignItemsType;
pub struct AlignSelfType;
pub struct BlendModeType;

// 设置Position、Border、Margin、Padding的优先级比单独设置上右下左的优先级要低，所以有单独的标识，
// 假定Position属性的设置，作用到上由下左上，可能会覆盖单独设置的上右下左属性
pub struct PositionType;
pub struct BorderType;
pub struct MarginType;
pub struct PaddingType;

pub trait Attr: 'static + Sync + Send {
	fn get_type(&self) -> StyleType;
	fn size(&self) -> usize;
	unsafe fn write(&self, ptr: *mut u8, buffer: &mut Vec<u8>);
	/// 安全： entity必须存在
	fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity);
	/// 安全： entity必须存在
	fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity);
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
        unsafe fn write(&self, ptr: *mut u8, buffer: &mut Vec<u8>) {
			let ty_size = std::mem::size_of::<StyleType>();
			let value_size = self.size();
			let len = buffer.len();
			buffer.reserve(ty_size + value_size);
	
			let ty = self.get_type();
			std::ptr::copy_nonoverlapping(
				&ty as *const StyleType as *const u8,
				buffer.as_mut_ptr().add(len),
				ty_size,
			);
	
			std::ptr::copy_nonoverlapping(
				ptr,
				buffer.as_mut_ptr().add(len + ty_size),
				value_size,
			);
		}
	};
}

macro_rules! set {
	// 整体插入
    ($name: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = unsafe {buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned()};
			item.write(v);
		}
	};
	// 属性修改
	($name: ident, $feild: ident, $value_ty: ident) => {
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
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
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
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
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
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
        fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
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
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
			// 取不到说明实体已经销毁
			let mut item = query.$name.get_unchecked_mut(entity);
			let v = item.get_default().clone();
			item.write(v);
		}
	};
	// 属性修改
	($name: ident, $feild: ident) => {
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
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
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
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
        fn reset(&self, _cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
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
        fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
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
        fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
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
		impl Attr for $struct_name {
			get_type!($ty);
			size!($ty);
			write_buffer!();
			set!($name, $ty);
			reset!($name, $ty);
		}
	};
	($struct_name: ident, $name: ident, $ty: ident, $value_ty: ident) => {
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $value_ty);
			reset!($name);
		}
	};
	($struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			reset!($name, $feild);
		}
	};
	($struct_name: ident, $name: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild1, $feild2, $value_ty);
			reset!($name, $feild1, $feild2);
		}
	};
	(@func $struct_name: ident,  $name: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!(@func $name, $set_func, $value_ty);
			reset!(@func $name, $set_func, $get_func);
		}
	};
	(@box_model_single $struct_name: ident, $name: ident, $feild: ident, $ty: ident, $value_ty: ident, $ty_all: ident) => {
		impl Attr for $struct_name {
			get_type!($ty);
			size!($value_ty);
			write_buffer!();
			set!($name, $feild, $value_ty);
			reset!(@box_model_single $name, $feild, $ty_all);
		}
	};
	(@box_model $struct_name: ident, $name: ident, $ty: ident) => {
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

impl_style!(BackGroundImageType, background_image, BackgroundImage);
impl_style!(BackGroundImageClipType, background_image_clip, BackgroundImageClip);
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
impl_style!(HeightType, size, width, Height, Dimension);

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

impl_style!(FlexShrinkType, flex_normal, flex_shrink, FlexShrink, f32);
impl_style!(FlexGrowType, flex_normal, flex_grow, FlexGrow, f32);
impl_style!(PositionTypeType, flex_normal, position_type, PositionType, PositionType1);
impl_style!(AlignSelfType, flex_normal, align_self, AlignSelf, AlignSelf);

impl_style!(BlendModeType, blend_mode, BlendMode);

// 设置Position、Border、Margin、Padding的优先级比单独设置上右下左的优先级要低，所以有单独的标识，
// 假定Position属性的设置，作用到上由下左上，可能会覆盖单独设置的上右下左属性
impl_style!(@box_model PositionType, position, Position);
impl_style!(@box_model BorderType, border, Border);
impl_style!(@box_model MarginType, margin, Margin);
impl_style!(@box_model PaddingType, padding, Padding);

// impl Attr for PaddingType {
// 	#[inline]
// 	fn get_type(&self) -> StyleType {
// 		StyleType::Padding
// 	}

// 	#[inline]
// 	fn size(&self) -> usize {
// 		std::mem::size_of::<Dimension>()
// 	}
	
// 	unsafe fn write(&self, ptr: *mut u8, buffer: &mut Vec<u8>) {
// 		let ty_size = std::mem::size_of::<StyleType>();
// 		let value_size = self.size();
// 		let len = buffer.len();
// 		buffer.reserve(ty_size + value_size);

// 		let ty = StyleType::Padding;
// 		std::ptr::copy_nonoverlapping(
// 			&ty as *const StyleType as *const u8,
// 			buffer.as_mut_ptr().add(len),
// 			ty_size,
// 		);

// 		std::ptr::copy_nonoverlapping(
// 			ptr,
// 			buffer.as_mut_ptr().add(len + ty_size),
// 			value_size,
// 		);
// 	}

// 	fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
// 		// 取不到说明实体已经销毁
// 		let mut item = query.padding.get_unchecked_mut(entity);
// 		let v = unsafe {buffer.as_ptr().add(offset).cast::<Dimension>().read_unaligned()};
// 		let component = item.get_mut_or_default();
// 		component.top = v;
// 		component.right = v;
// 		component.bottom = v;
// 		component.left = v;
// 		item.notify_modify();
// 	}

// 	fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
// 		// 设置为默认值 TODO
// 		let mut item = query.padding.get_unchecked_mut(entity);
// 		if let Some(component) = item.get_mut() {
// 			let mut is_changed = false;
// 			if !cur_style_mark[StyleType::PaddingTop as usize] {
// 				is_changed = true;
// 			}
// 			if !cur_style_mark[StyleType::PaddingRight as usize] {
// 				is_changed = true;
// 			}
// 			if !cur_style_mark[StyleType::PaddingBottom as usize] {
// 				is_changed = true;
// 			}
// 			if !cur_style_mark[StyleType::PaddingLeft as usize] {
// 				is_changed = true;
// 			}

// 			// 通知padding修改
// 			if is_changed {
// 				item.notify_modify();
// 			}
// 		}
// 	}
// }


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

lazy_static! {
	static ref STYLE_ATTR: [Box<dyn Attr>; 1] = [Box::new(PaddingType)];
}

impl StyleAttr {
	pub fn get_attr(style_type: StyleType) -> &'static Box<dyn Attr> {
		&STYLE_ATTR[style_type as usize]
	}

	pub unsafe fn write<T>(style_type: StyleType, value: T, buffer: &mut Vec<u8>) {
		STYLE_ATTR[style_type as usize].write(&value as *const T as usize as *mut u8, buffer);
	}

	pub fn set(style_type: StyleType, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity) {
		STYLE_ATTR[style_type as usize].set(buffer, offset, query, entity);
	}

	pub fn reset(style_type: StyleType, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity) {
		STYLE_ATTR[style_type as usize].reset(cur_style_mark, query, entity);
	}
}



