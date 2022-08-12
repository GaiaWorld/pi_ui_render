//! 将设置布局属性的接口导出到js

use std::mem::transmute;

use ordered_float::NotNan;
use pi_async::rt::worker_thread::WorkerRuntime;
use pi_atom::Atom;
use pi_hash::XHashMap;
use pi_idtree::InsertType;
use pi_map::vecmap::VecMap;
use pi_null::Null;
use pi_share::ShareRefCell;
use pi_slab::Slab;
use smallvec::SmallVec;
use crate::{
	components::user::{Node, BackgroundColor, Color, CgColor, LinearGradientColor, ColorAndPosition, BorderColor, BorderRadius, LengthUnit, BoxShadow, BackgroundImageClip, Aabb2, Point2, MaskImageClip, BorderImageClip, NotNanRect, BorderImageSlice, BorderImageRepeat, Overflow, Opacity, ZIndex, Blur, Hsi, BorderImage, MaskImage, BackgroundImage, TransformFunc, TransformOrigin, LineHeight, FontSize, TextContent, Stroke, TextShadows, Position, Border, Margin, Padding, ClassName, Transform},
};
use pi_style::{style_parse::parse_text_shadow, style_type::*};
use pi_flex_layout::prelude::*;
use pi_ecs::{prelude::{Id, LocalVersion, SingleDispatcher, Dispatcher}, storage::Offset};
use super::json_parse::as_value;
pub use crate::export::Gui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

pub struct PlayContext {
	pub nodes: VecMap<f64>,
	pub atoms: XHashMap<usize, Atom>,
	pub idtree: pi_idtree::IdTree<()>,
}

macro_rules! out_export {
	(@dimension_box $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			out_export!(@dimension_inner  [<$attr_name _percent>], $last_ty, Dimension::Percent(v), v: f32, );
			out_export!(@dimension_inner $attr_name, $last_ty, Dimension::Points(v), v: f32, );
			out_export!(@dimension_inner  [<$attr_name _auto>], $last_ty, Dimension::Auto, );
		}
	};

	(@dimension $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			out_export!(@expr  [<$attr_name _percent>], $last_ty, Dimension::Percent(v), v: f32, );
			out_export!(@expr $attr_name, $last_ty, Dimension::Points(v), v: f32, );
			out_export!(@expr  [<$attr_name _auto>], $last_ty, Dimension::Auto, );
		}
	};

	(@cenum $attr_name:ident, $last_ty: ident) => {
		out_export!(@expr $attr_name, $last_ty, unsafe {transmute(v)}, v: u8,);
	};

	(@expr $attr_name:ident, $last_ty: ident, $expr:expr, $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = 1;
				let node = super::json_parse::as_value::<usize>(json, 0).unwrap();
				$(let $name = super::json_parse::as_value::<$ty>(json, i).unwrap(); i += 1;)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				[<set_ $attr_name>](gui, node, $($name,)*);
			}
		}
    };

	(@dimension_inner $attr_name:ident, $last_ty: ident, $expr: expr, $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
			#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge:u8, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge)} {
					Edge::All => gui.0.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
						top: $expr,
						right: $expr,
						bottom: $expr,
						left: $expr,
					}))),
					Edge::Top => gui.0.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.0.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.0.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.0.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge:u8, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge)} {
					Edge::All => gui.0.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
						top: $expr,
						right: $expr,
						bottom: $expr,
						left: $expr,
					}))),
					Edge::Top => gui.0.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.0.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.0.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.0.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
			#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge:u8) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge)} {
					Edge::All => gui.0.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.0.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.0.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.0.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.0.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge:u8) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge)} {
					Edge::All => gui.0.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.0.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.0.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.0.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.0.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[allow(unused_variables)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = 2;
				let node = super::json_parse::as_value::<usize>(json, 0).unwrap();
				let edge = super::json_parse::as_value::<u8>(json, 1).unwrap();
				$(let $name = super::json_parse::as_value::<$ty>(json, i).unwrap(); i += 1;)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				[<set_ $attr_name>](gui, node, edge, $($name,)*);
			}
		}
	};

	(@atom $attr_name:ident, $last_ty: ident, $expr:expr, $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.0.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = super::json_parse::as_value::<usize>(json, 0).unwrap();
				let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				let atom_hash = context.atoms.get(&hash).unwrap().get_hash();
				[<set_ $attr_name>](gui, node, atom_hash);
			}
		}
    };
}

out_export!(@cenum align_content, AlignContentType);
out_export!(@cenum align_items, AlignItemsType);
out_export!(@cenum justify_content, JustifyContentType);
out_export!(@cenum flex_direction, FlexDirectionType);
out_export!(@cenum flex_wrap, FlexWrapType);
out_export!(@cenum align_self, AlignSelfType);
out_export!(@cenum position_type, PositionTypeType);

out_export!(@expr flex_grow, FlexGrowType, v, v: f32,);
out_export!(@expr flex_shrink, FlexGrowType, v, v: f32,);

out_export!(@dimension flex_basis, FlexBasisType);
out_export!(@dimension width, WidthType);
out_export!(@dimension height, HeightType);
out_export!(@dimension min_width, MinWidthType);
out_export!(@dimension min_height, MinHeightType);
out_export!(@dimension max_width, MaxWidthType);
out_export!(@dimension max_height, MaxHeightType);

out_export!(@dimension_box padding, Padding);
out_export!(@dimension_box margin, Margin);
out_export!(@dimension_box border, Border);
out_export!(@dimension_box position, Position);

out_export!(@expr background_rgba_color, BackgroundColorType, BackgroundColor(Color::RGBA(CgColor::new(r, g, b, a))), r: f32, g: f32, b: f32, a: f32,);
out_export!(@expr 
	background_linear_color, 
	BackgroundColorType, 
	BackgroundColor(Color::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    ))), 
	direction: f32, color_and_positions: Vec<f32>,);

out_export!(@expr 
	border_color,
	BorderColorType,
	BorderColor(CgColor::new(r, g, b, a)),
	r: f32, g: f32, b: f32, a: f32,);

out_export!(@expr 
	border_radius,
	BorderRadiusType,
	BorderRadius {
		x: LengthUnit::Pixel(x),
		y: LengthUnit::Pixel(y)
	},
	x: f32, y: f32,);

out_export!(@expr 
	border_radius_percent,
	BorderRadiusType,
	BorderRadius {
		x: LengthUnit::Percent(x),
		y: LengthUnit::Percent(y)
	},
	x: f32, y: f32,);

out_export!(@expr 
	box_shadow,
	BoxShadowType,
	BoxShadow {
		h: h,
		v: v,
		blur: blur,
		spread: spread,
		color: CgColor::new(r, g, b, a)
	},
	h: f32, v: f32, blur: f32, spread: f32, r: f32, g: f32 ,b: f32, a: f32,);
out_export!(@cenum object_fit, ObjectFitType);

out_export!(@atom 
	mask_image,
	MaskImageType,
	MaskImage::Path(Atom::get(image_hash).unwrap()),
	image_hash: usize, );

out_export!(@atom 
	background_image,
	BackgroundImageType,
	{
		// println!("set_image=============={:?}", image_hash);
		BackgroundImage(Atom::get(image_hash).unwrap())
	},
	image_hash: usize, );

out_export!(@expr 
	mask_image_linenear,
	MaskImageType,
	MaskImage::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )),
	direction: f32, color_and_positions: Vec<f32>, );

out_export!(@atom 
	border_image,
	BorderImageType,
	BorderImage (Atom::get(image_hash).unwrap()),
	image_hash: usize, );

out_export!(@expr 
	image_clip,
	BackgroundImageClipType,
	BackgroundImageClip(Aabb2::new(Point2::new(u1, v1), Point2::new(u2, v2))),
	u1: f32, v1: f32, u2: f32, v2: f32,);

out_export!(@expr 
	mask_image_clip,
	MaskImageClipType,
	MaskImageClip(Aabb2::new(Point2::new(u1, v1), Point2::new(u2, v2))),
	u1: f32, v1: f32, u2: f32, v2: f32,);

out_export!(@expr 
	border_image_clip,
	BorderImageClipType,
	BorderImageClip(NotNanRect {
		top: unsafe { NotNan::new_unchecked(v1) },
		right: unsafe { NotNan::new_unchecked(u2) },
		bottom: unsafe { NotNan::new_unchecked(v2) },
		left: unsafe { NotNan::new_unchecked(u1) },
	}),
	u1: f32, v1: f32, u2: f32, v2: f32,);

out_export!(@expr 
	border_image_slice,
	BorderImageSliceType,
	BorderImageSlice {
		top: unsafe { NotNan::new_unchecked(top) },
		right: unsafe { NotNan::new_unchecked(right) },
		bottom: unsafe { NotNan::new_unchecked(bottom) },
		left: unsafe { NotNan::new_unchecked(left) },
		fill,
	},
	top: f32, right: f32, bottom: f32, left: f32, fill: bool,);

out_export!(@expr 
	border_image_repeat,
	BorderImageRepeatType,
	BorderImageRepeat (
		unsafe { transmute(vertical) },
		unsafe { transmute(horizontal) },
	),
	vertical: u8, horizontal: u8, );

out_export!(@expr overflow, OverflowType, Overflow(v), v: bool,);
out_export!(@expr opacity, OpacityType, Opacity(v), v: f32,);
out_export!(@cenum display, DisplayType);
out_export!(@expr visibility, VisibilityType, v, v: bool,);
out_export!(@cenum enable, EnableType);
out_export!(@cenum blend_mode, BlendModeType);
out_export!(@expr zindex, ZIndexType, ZIndex(v), v: isize,);
out_export!(@expr filter_blur, BlurType, Blur(v), v: f32,);

// hsi, 效果与ps一致,  h: -180 ~ 180, s: -100 ~ 100, i: -100 ~ 100
out_export!(@expr 
	filter_hsi,
	HsiType,
	{
		let (mut h, mut s, mut i) = (h, s, i);
		if h > 180.0 {
			h = 180.0;
		} else if h < -180.0 {
			h = -180.0
		}
		if s > 100.0 {
			s = 100.0;
		} else if s < -100.0 {
			s = -100.0
		}
		if i > 100.0 {
			i = 100.0;
		} else if i < -100.0 {
			i = -100.0
		}
		Hsi {
			hue_rotate: h / 360.0,
			saturate: s / 100.0,
			bright_ness: i / 100.0,
		}
	},
	h: f32, s: f32, i: f32, );
/************************************ Transform **************************************/
out_export!(@expr 
	transform_translate, 
	TransformFuncType, 
	TransformFunc::Translate(x, y),
	x: f32, y: f32,);
out_export!(@expr 
	transform_translate_percent, 
	TransformFuncType, 
	TransformFunc::TranslatePercent(x, y),
	x: f32, y: f32,);
out_export!(@expr 
	transform_translate_x, 
	TransformFuncType, 
	TransformFunc::TranslateX(x),
	x: f32,);
out_export!(@expr 
	transform_translate_x_percent, 
	TransformFuncType, 
	TransformFunc::TranslateXPercent(x),
	x: f32,);
out_export!(@expr 
	transform_translate_y, 
	TransformFuncType, 
	TransformFunc::TranslateY(y),
	y: f32,);
out_export!(@expr 
	transform_translate_y_percent, 
	TransformFuncType, 
	TransformFunc::TranslateYPercent(y),
	y: f32,);
out_export!(@expr 
	transform_scale, 
	TransformFuncType, 
	TransformFunc::Scale(x, y),
	x: f32, y: f32,);
out_export!(@expr 
	transform_scale_x, 
	TransformFuncType, 
	TransformFunc::ScaleX(x),
	x: f32,);
out_export!(@expr 
	transform_scale_y, 
	TransformFuncType, 
	TransformFunc::ScaleY(y),
	y: f32,);
out_export!(@expr 
	transform_rotate_x, 
	TransformFuncType, 
	TransformFunc::RotateX(x),
	x: f32,);
out_export!(@expr 
	transform_rotate_y, 
	TransformFuncType, 
	TransformFunc::RotateY(y),
	y: f32,);
out_export!(@expr 
	transform_rotate_z, 
	TransformFuncType, 
	TransformFunc::RotateZ(z),
	z: f32,);
out_export!(@expr 
	transform_skew_x, 
	TransformFuncType, 
	TransformFunc::SkewX(x),
	x: f32,);
out_export!(@expr 
	transform_skew_y, 
	TransformFuncType, 
	TransformFunc::SkewY(y),
	y: f32,);
out_export!(@expr 
	clear_transform, 
	TransformType, 
	vec![],);
out_export!(@expr 
	transform_origin, 
	TransformOriginType, 
	{
		let x_ty = unsafe { transmute(x_ty) };
		let y_ty = unsafe { transmute(y_ty) };
		let x_value = match x_ty {
			LengthUnitType::Pixel => LengthUnit::Pixel(x),
			LengthUnitType::Percent => LengthUnit::Percent(x),
		};
		let y_value = match y_ty {
			LengthUnitType::Pixel => LengthUnit::Pixel(y),
			LengthUnitType::Percent => LengthUnit::Percent(y),
		};
		TransformOrigin::XY(x_value, y_value)
	},
	x_ty: u8, x: f32, y_ty: u8, y: f32,);

// 设置transform为None TODO

out_export!(@expr letter_spacing, LetterSpacingType, v, v: f32,);
out_export!(@expr word_spacing, WordSpacingType, v, v: f32,);

out_export!(@expr text_rgba_color, ColorType, Color::RGBA(CgColor::new(r, g, b, a)), r: f32, g: f32, b: f32, a: f32,);
out_export!(@expr 
	text_linear_gradient_color, 
	ColorType, 
	Color::LinearGradient(to_linear_gradient_color(
		color_and_positions.as_slice(),
		direction,
	)), direction: f32, color_and_positions: Vec<f32>, );
out_export!(@expr line_height_normal, LineHeightType, LineHeight::Normal,);
out_export!(@expr line_height, LineHeightType, LineHeight::Length(value), value: f32,);
out_export!(@expr line_height_percent, LineHeightType, LineHeight::Percent(value), value: f32,);
out_export!(@expr text_indent, TextIndentType, v, v: f32,);
out_export!(@cenum text_align, TextAlignType);
out_export!(@expr text_stroke, TextStrokeType, Stroke {
	width: NotNan::new(width).expect("stroke width is nan"),
	color: CgColor::new(r, g, b, a),
}, width: f32, r: f32, g: f32, b: f32, a: f32,);
out_export!(@cenum white_space, WhiteSpaceType);
out_export!(@expr text_shadow, TextShadowType, {
	let mut input = cssparser::ParserInput::new(s.as_str());
	let mut parse = cssparser::Parser::new(&mut input);

	let shadows = parse_text_shadow(&mut parse);
	if let Ok(value) = shadows {
		value
	} else {
		Default::default()
	}
}, s: String,);
out_export!(@cenum font_style, FontStyleType);
out_export!(@expr font_weight, FontWeightType, v, v: usize,);
out_export!(@expr font_size_none, FontSizeType, FontSize::None,);
out_export!(@expr font_size, FontSizeType, FontSize::Length(value), value: usize,);
out_export!(@expr font_size_percent, FontSizeType, FontSize::Percent(value), value: f32,);
out_export!(@atom font_family, FontFamilyType, Atom::get(name).unwrap(), name: usize,);
out_export!(@expr text_content, TextContentType,  TextContent(content, Atom::from("")), content: String,);
out_export!(@expr text_content_utf8, TextContentType, {
	let content = unsafe{String::from_utf8_unchecked(content)};
	TextContent(content, Atom::from(""))
}, content: Vec<u8>,);

pub fn set_class(gui: &mut Gui, node_id: f64, class_name: Vec<usize> ) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.0.set_class(node_id, ClassName(SmallVec::from_slice(class_name.as_slice())) );
}

pub fn play_set_class(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node = super::json_parse::as_value::<usize>(json, 0).unwrap();
	let class = super::json_parse::as_value::<Vec<usize>>(json, 1).unwrap();
	// let node = context.nodes.get(node).unwrap().clone();
	let node = match context.nodes.get(node) {
		Some(r) => r.clone(),
		None => return,
	};

	set_class(gui, node, class);
}

/// 设置默认样式, 暂支持布局属性、 文本属性的设置
/// __jsObj: class样式的二进制描述， 如".0{color:red}"生成的二进制， class名称必须是“0”
pub fn set_default_style_by_bin(gui: &mut Gui, bin: &[u8]) {
	gui.0.set_default_style_by_bin(bin);
}

pub fn set_default_style_by_str(gui: &mut Gui, bin: &str) {
	gui.0.set_default_style_by_str(bin);
}

/// 创建容器节点， 容器节点可设置背景颜色
pub fn create_node(gui: &mut Gui) -> f64 {
	unsafe { transmute(gui.0.create_node()) }
}

pub fn play_create_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let json = &json[0];
	let ret = &json["ret"];
	let ret = ret.as_usize().unwrap();
	context.nodes.insert(ret, create_node(gui) );
}

/// 创建虚拟节点
pub fn create_vnode(gui: &mut Gui) -> f64 {
	let node = gui.0.create_node();
	gui.0.set_style(node, VNodeType(true));
	unsafe { transmute(node) }
}

pub fn play_create_vnode(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let json = &json[0];
	let ret = &json["ret"];
	let ret = ret.as_usize().unwrap();
	context.nodes.insert(ret, create_vnode(gui) );
}

/// 创建文本节点
pub fn create_text_node(gui: &mut Gui) -> f64 {
    let node = gui.0.create_node();
	gui.0.set_style(node, TextContentType(TextContent("".to_string(), Atom::from(""))));
	unsafe { transmute(node) }
}

pub fn play_create_text_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let json = &json[0];
	let ret = &json["ret"];
	let ret = ret.as_usize().unwrap();
	context.nodes.insert(ret, create_text_node(gui) );
}

/// 创建图片节点
pub fn create_image_node(gui: &mut Gui) -> f64 {
    unsafe { transmute(gui.0.create_node()) }
}

pub fn play_create_image_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let json = &json[0];
	let ret = &json["ret"];
	let ret = ret.as_usize().unwrap();
	context.nodes.insert(ret, create_image_node(gui) );
}

/// 创建图片节点
pub fn create_canvas_node(gui: &mut Gui) -> f64 {
    unsafe { transmute(gui.0.create_node()) }
}

pub fn play_create_canvas_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let json = &json[0];
	let ret = &json["ret"];
	let ret = ret.as_usize().unwrap();
	context.nodes.insert(ret, create_canvas_node(gui) );
}

/// 移除节点
pub fn remove_node(gui: &mut Gui, node_id: f64) {
	gui.0.remove_node(unsafe { transmute(node_id)});
}

pub fn play_remove_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node_id = as_value::<usize>(json, 0).unwrap();
	let node_id = context.nodes.get(node_id).unwrap().clone();
	remove_node(gui, node_id);
}

pub fn destroy_node(gui: &mut Gui, node_id: f64) {
	gui.0.destroy_node(unsafe { transmute(node_id)});
}

pub fn play_destroy_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let id = as_value::<usize>(json, 0).unwrap();
	let node_id = context.nodes.remove(id).unwrap();

	if let Some(r) = context.idtree.get(id) {
		let head = r.children().head;
		// 移除所有节点
		
		for (id, _n) in context.idtree.recursive_iter(head) {
			context.nodes.remove(id);
		}

		// 递归删除idtree
		let r = match context.idtree.get(id) {
			Some(n) => (n.parent(), n.layer(), n.count(), n.prev(), n.next(), n.children().head),
			_ => return,
		};
		context.idtree.destroy(id, r, true);
	}
	

	// 销毁节点
	destroy_node(gui, node_id);
}

pub fn append_child(gui: &mut Gui, node_id: f64, parent_id: f64 ) {
	gui.0.append(unsafe { transmute(node_id)}, unsafe { transmute(parent_id)});
}

pub fn play_append_child(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node_id = as_value::<usize>(json, 0).unwrap();
	let parent_id = as_value::<usize>(json, 1).unwrap();
	let node_id1 = context.nodes.get(node_id).unwrap().clone();
	let parent_id1 = match context.nodes.get(parent_id) {
		Some(r) => r.clone(),
		None => unsafe { transmute( Id::<Node>::null())},
	};
	
	append_child(gui, node_id1, parent_id1);

	if context.idtree.get(node_id).is_none() {
		context.idtree.create(node_id);
	}

	if parent_id1.is_null() {
		context.idtree.insert_child(node_id, 0, 0);
	} else {
		if context.idtree.get(parent_id).is_none() {
			context.idtree.create(parent_id);
		}
		context.idtree.insert_child(node_id, parent_id, 0);
	}
}

pub fn insert_before(gui: &mut Gui, node_id: f64, borther: f64 ) {
	gui.0.insert_before(unsafe { transmute(node_id)}, unsafe { transmute(borther)});
}

pub fn play_insert_before(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node_id = as_value::<usize>(json, 0).unwrap();
	let borther = as_value::<usize>(json, 1).unwrap();
	let node_id1 = context.nodes.get(node_id).unwrap().clone();
	let borther1 = context.nodes.get(borther).unwrap().clone();
	insert_before(gui, node_id1, borther1);

	if context.idtree.get(node_id).is_none() {
		context.idtree.create(node_id);
	}

	context.idtree.insert_brother(node_id, borther, InsertType::Front);
	
}



#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn set_default_style_by_bin(gui: &mut Gui, bin: &[u8]) {
	gui.0.set_default_style_by_bin(bin);
}

#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn set_default_style_by_str(gui: &mut Gui, bin: &str) {
	gui.0.set_default_style_by_str(bin);
}


/// 创建容器节点， 容器节点可设置背景颜色
#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn create_node(gui: &mut Gui) -> f64 {
	unsafe { transmute(gui.0.create_node()) }
}

/// 创建虚拟节点
#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn create_vnode(gui: &mut Gui) -> f64 {
	let node = gui.0.create_node();
	gui.0.set_style(node, VNodeType(true));
	unsafe { transmute(node) }
}

/// 创建文本节点
#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn create_text_node(gui: &mut Gui) -> f64 {
    let node = gui.0.create_node();
	gui.0.set_style(node, TextContentType(TextContent("".to_string(), Atom::from(""))));
	unsafe { transmute(node) }
}

/// 创建图片节点
#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn create_image_node(gui: &mut Gui) -> f64 {
    unsafe { transmute(gui.0.create_node()) }
}

/// 创建图片节点
#[cfg(target_arch = "wasm32")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn create_canvas_node(gui: &mut Gui) -> f64 {
    unsafe { transmute(gui.0.create_node()) }
}

pub fn to_linear_gradient_color(
    color_and_positions: &[f32],
    direction: f32,
) -> LinearGradientColor {
    let arr = color_and_positions;
    let len = arr.len();
    let count = len / 5;
    let mut list = Vec::with_capacity(count);
    for i in 0..count {
        let start = i * 5;
        let color_pos = ColorAndPosition {
            rgba: CgColor::new(arr[start], arr[start + 1], arr[start + 2], arr[start + 3]),
            position: arr[start + 4],
        };
        list.push(color_pos);
    }
    LinearGradientColor {
        direction: direction,
        list: list,
    }
}


pub enum LengthUnitType {
    Pixel,
    Percent,
}
