//! 将设置布局属性的接口导出到js

use std::{
    mem::transmute,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::components::user::{
    BorderImageSlice, BorderRadius, BoxShadow, CgColor, ClassName, Color, ColorAndPosition, FontSize, Hsi, ImageRepeat, LengthUnit, LineHeight,
    LinearGradientColor, MaskImage, Node, NotNanRect, Stroke, TextContent, TransformFunc, TransformOrigin,
};
use crate::utils::cmd::SingleCmd;
use ordered_float::NotNan;
use pi_async::prelude::AsyncRuntime;
use pi_ecs::prelude::{DispatcherMgr, Id, LocalVersion, Offset};
use pi_flex_layout::prelude::*;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_style::style::{AnimationTimingFunction, AnimationName};
use pi_style::{
    style_parse::{parse_class_map_from_string, parse_comma_separated, parse_text_shadow, StyleParse},
    style_type::*,
};
use smallvec::SmallVec;
pub use {super::Atom, crate::export::Engine as Gui};

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

#[macro_export]
macro_rules! other_out_export {
	($func_name:ident, $context: ident, $node: ident, $expr:expr, $($name: ident: $ty: ty,)*) => {
		#[cfg(target_arch = "wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $node: f64, $($name: $ty,)*) {
			let $node = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>($node)))};
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($context: &mut Gui, $node: f64, $($name: $ty,)*) {
			let $node = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>($node)))};
			$expr
		}

		$crate::paste::item! {
			pub fn [<play_ $func_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = -1;
				i += 1;
				let node = unsafe{ transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, i as usize).unwrap())}.offset();
				$(i += 1; let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				$func_name(gui, node, $($name,)*);
			}
		}
	};

	($func_name:ident, $context: ident, $expr:expr, $($name: ident: $ty: ty,)*) => {
		#[cfg(target_arch = "wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $($name: $ty,)*) {
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($context: &mut Gui, $($name: $ty,)*) {
			$expr
		}

		$crate::paste::item! {
			#[allow(unused_variables)]
			pub fn [<play_ $func_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let i = -1;
				$(let i = i + 1; let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
				$func_name(gui, $($name,)*);
			}
		}
	};
}

#[macro_export]
macro_rules! style_out_export {
	(@dimension_box $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@dimension_inner  [<$attr_name _percent>], $last_ty, Dimension::Percent(v), v: f32, );
			style_out_export!(@dimension_inner $attr_name, $last_ty, Dimension::Points(v), v: f32, );
			style_out_export!(@dimension_inner  [<$attr_name _auto>], $last_ty, Dimension::Auto, );
		}
	};

	(@dimension $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@expr  [<$attr_name _percent>], $last_ty, Dimension::Percent(v), v: f32, );
			style_out_export!(@expr $attr_name, $last_ty, Dimension::Points(v), v: f32, );
			style_out_export!(@expr  [<$attr_name _auto>], $last_ty, Dimension::Auto, );
		}
	};

	(@cenum $attr_name:ident, $last_ty: ident) => {
		style_out_export!(@expr $attr_name, $last_ty, unsafe {transmute(v as u8)}, v: f64,);
	};

	(@expr $attr_name:ident, $last_ty: ident, $expr:expr, $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe { transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, 0).unwrap())}.offset();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = 0;
				let node = unsafe{ transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, i).unwrap())}.offset();
				i += 1;
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
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.gui.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.gui.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.gui.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.gui.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.gui.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.gui.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.gui.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.gui.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.gui.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.gui.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
			#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.gui.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.gui.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.gui.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.gui.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.gui.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.gui.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.gui.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.gui.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.gui.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.gui.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe { transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, 0).unwrap())}.offset();
				let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node, edge);
			}

			#[allow(unused_variables)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = -1;
				i += 1;
				let node = unsafe{ transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, i as usize).unwrap())}.offset();
				i += 1;
				let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
				$(i += 1;let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
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
				gui.gui.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name: $ty,)*) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch = "wasm32")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
				gui.gui.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe { transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, 0).unwrap())}.offset();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe { transmute::<_, Id<Node>>(super::json_parse::as_value::<f64>(json, 0).unwrap())}.offset();
				let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node) {
					Some(r) => r.clone(),
					None => return,
				};
				let atom_hash = match context.atoms.get(&hash) {
					Some(r) => r.get_hash(),
					None => panic!("can not find atom, hash: {}", hash),
				};
				[<set_ $attr_name>](gui, node, &Atom(pi_atom::Atom::get(atom_hash as usize).unwrap()) );
			}
		}
    };
}

style_out_export!(@cenum align_content, AlignContentType);
style_out_export!(@cenum align_items, AlignItemsType);
style_out_export!(@cenum justify_content, JustifyContentType);
style_out_export!(@cenum flex_direction, FlexDirectionType);
style_out_export!(@cenum flex_wrap, FlexWrapType);
style_out_export!(@cenum align_self, AlignSelfType);
style_out_export!(@cenum position_type, PositionTypeType);

style_out_export!(@expr flex_grow, FlexGrowType, v, v: f32,);
style_out_export!(@expr flex_shrink, FlexGrowType, v, v: f32,);

style_out_export!(@dimension flex_basis, FlexBasisType);
style_out_export!(@dimension width, WidthType);
style_out_export!(@dimension height, HeightType);
style_out_export!(@dimension min_width, MinWidthType);
style_out_export!(@dimension min_height, MinHeightType);
style_out_export!(@dimension max_width, MaxWidthType);
style_out_export!(@dimension max_height, MaxHeightType);

style_out_export!(@dimension_box padding, Padding);
style_out_export!(@dimension_box margin, Margin);
style_out_export!(@dimension_box border, Border);
style_out_export!(@dimension_box position, Position);

style_out_export!(@expr background_rgba_color, BackgroundColorType, Color::RGBA(CgColor::new(r, g, b, a)), r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	background_linear_color, 
	BackgroundColorType, 
	Color::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )), 
	direction: f32, color_and_positions: Vec<f32>,);

style_out_export!(@expr 
	border_color,
	BorderColorType,
	CgColor::new(r, g, b, a),
	r: f32, g: f32, b: f32, a: f32,);

style_out_export!(@expr 
	border_radius,
	BorderRadiusType,
	BorderRadius {
		x: LengthUnit::Pixel(x),
		y: LengthUnit::Pixel(y)
	},
	x: f32, y: f32,);

style_out_export!(@expr 
	border_radius_percent,
	BorderRadiusType,
	BorderRadius {
		x: LengthUnit::Percent(x),
		y: LengthUnit::Percent(y)
	},
	x: f32, y: f32,);

style_out_export!(@expr 
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
style_out_export!(@cenum object_fit, ObjectFitType);

style_out_export!(@expr background_repeat, BackgroundRepeatType, ImageRepeat {
	x: unsafe { transmute(x as u8) },
	y: unsafe { transmute(y as u8) },
}, x: u8, y: u8, );

style_out_export!(@expr 
	mask_image_linenear,
	MaskImageType,
	MaskImage::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )),
	direction: f32, color_and_positions: Vec<f32>, );

style_out_export!(@expr 
	image_clip,
	BackgroundImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	mask_image_clip,
	MaskImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	border_image_clip,
	BorderImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
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

style_out_export!(@expr 
	border_image_repeat,
	BorderImageRepeatType,
	ImageRepeat {
		x: unsafe { transmute(vertical as u8) },
		y: unsafe { transmute(horizontal as u8) },
	},
	vertical: u8, horizontal: u8, );

style_out_export!(@expr overflow, OverflowType, v, v: bool,);
style_out_export!(@expr opacity, OpacityType, v, v: f32,);
style_out_export!(@cenum display, DisplayType);
style_out_export!(@expr visibility, VisibilityType, v, v: bool,);
style_out_export!(@cenum enable, EnableType);
style_out_export!(@cenum blend_mode, BlendModeType);
style_out_export!(@expr zindex, ZIndexType, v as isize, v: i32,);
style_out_export!(@expr filter_blur, BlurType, v, v: f32,);

// hsi, 效果与ps一致,  h: -180 ~ 180, s: -100 ~ 100, i: -100 ~ 100
style_out_export!(@expr 
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
style_out_export!(@expr 
	transform_translate, 
	TransformFuncType, 
	TransformFunc::Translate(x, y),
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_translate_percent, 
	TransformFuncType, 
	TransformFunc::TranslatePercent(x, y),
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_translate_x, 
	TransformFuncType, 
	TransformFunc::TranslateX(x),
	x: f32,);
style_out_export!(@expr 
	transform_translate_x_percent, 
	TransformFuncType, 
	TransformFunc::TranslateXPercent(x),
	x: f32,);
style_out_export!(@expr 
	transform_translate_y, 
	TransformFuncType, 
	TransformFunc::TranslateY(y),
	y: f32,);
style_out_export!(@expr 
	transform_translate_y_percent, 
	TransformFuncType, 
	TransformFunc::TranslateYPercent(y),
	y: f32,);
style_out_export!(@expr 
	transform_scale, 
	TransformFuncType, 
	TransformFunc::Scale(x, y),
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_scale_x, 
	TransformFuncType, 
	TransformFunc::ScaleX(x),
	x: f32,);
style_out_export!(@expr 
	transform_scale_y, 
	TransformFuncType, 
	TransformFunc::ScaleY(y),
	y: f32,);
style_out_export!(@expr 
	transform_rotate_x, 
	TransformFuncType, 
	TransformFunc::RotateX(x),
	x: f32,);
style_out_export!(@expr 
	transform_rotate_y, 
	TransformFuncType, 
	TransformFunc::RotateY(y),
	y: f32,);
style_out_export!(@expr 
	transform_rotate_z, 
	TransformFuncType, 
	TransformFunc::RotateZ(z),
	z: f32,);
style_out_export!(@expr 
	transform_skew_x, 
	TransformFuncType, 
	TransformFunc::SkewX(x),
	x: f32,);
style_out_export!(@expr 
	transform_skew_y, 
	TransformFuncType, 
	TransformFunc::SkewY(y),
	y: f32,);
style_out_export!(@expr 
	clear_transform, 
	TransformType, 
	vec![],);
style_out_export!(@expr 
	transform_origin, 
	TransformOriginType, 
	{
		let x_ty = unsafe { transmute(x_ty as u8) };
		let y_ty = unsafe { transmute(y_ty as u8) };
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
	x_ty: f64, x: f32, y_ty: f64, y: f32,);

// 设置transform为None TODO

style_out_export!(@expr letter_spacing, LetterSpacingType, v, v: f32,);
style_out_export!(@expr word_spacing, WordSpacingType, v, v: f32,);

style_out_export!(@expr text_rgba_color, ColorType, Color::RGBA(CgColor::new(r, g, b, a)), r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	text_linear_gradient_color, 
	ColorType, 
	Color::LinearGradient(to_linear_gradient_color(
		color_and_positions.as_slice(),
		direction,
	)), direction: f32, color_and_positions: Vec<f32>, );
style_out_export!(@expr line_height_normal, LineHeightType, LineHeight::Normal,);
style_out_export!(@expr line_height, LineHeightType, LineHeight::Length(value), value: f32,);
style_out_export!(@expr line_height_percent, LineHeightType, LineHeight::Percent(value), value: f32,);
style_out_export!(@expr text_indent, TextIndentType, v, v: f32,);
style_out_export!(@cenum text_align, TextAlignType);
style_out_export!(@expr text_stroke, TextStrokeType, Stroke {
	width: NotNan::new(width).expect("stroke width is nan"),
	color: CgColor::new(r, g, b, a),
}, width: f32, r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@cenum white_space, WhiteSpaceType);
style_out_export!(@cenum font_style, FontStyleType);
style_out_export!(@expr font_weight, FontWeightType, v as usize, v: f64,);
style_out_export!(@expr font_size_none, FontSizeType, FontSize::None,);
style_out_export!(@expr font_size, FontSizeType, FontSize::Length(value as usize), value: f64,);
style_out_export!(@expr font_size_percent, FontSizeType, FontSize::Percent(value), value: f32,);
style_out_export!(@expr text_content_utf8, TextContentType, {
	let content = unsafe{String::from_utf8_unchecked(content)};
	TextContent(content, pi_atom::Atom::from(""))
}, content: Vec<u8>,);
style_out_export!(@expr animation_duration, AnimationDurationType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) }, name: Vec<usize>,);
style_out_export!(@expr animation_delay, AnimationDelayType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) }, name: Vec<usize>,);
style_out_export!(@expr animation_iteration_count, AnimationIterationCountType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>()) }, name: Vec<f32>,);
style_out_export!(@expr animation_direction, AnimationDirectionType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) }, name: Vec<u8>,);
style_out_export!(@expr animation_fill_mode, AnimationFillModeType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) }, name: Vec<u8>,);
style_out_export!(@expr animation_play_state, AnimationPlayStateType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) }, name: Vec<u8>,);

// pub enum AnimationTimingFunction {
//     /// 匀速
//     Linear,
//     /// 淡入淡出
//     Ease(EEasingMode),
//     /// 跳跃
//     Step(usize, EStepMode),
//     /// 贝塞尔曲线
//     CubicBezier(f32, f32, f32, f32),
// }

// impl_style!(
//     AnimationTimingFunctionType,
//     animation,
//     timing_function,
//     AnimationTimingFunction,
//     SmallVec<[AnimationTimingFunction; 1]>
// );
// impl_style!(AnimationDelayType, animation, delay, AnimationDelay, SmallVec<[Time; 1]>);
// impl_style!(
//     AnimationIterationCountType,
//     animation,
//     iteration_count,
//     AnimationIterationCount,
//     SmallVec<[IterationCount; 1]>
// );
// impl_style!(
//     AnimationDirectionType,
//     animation,
//     direction,
//     AnimationDirection,
//     SmallVec<[AnimationDirection; 1]>
// );
// impl_style!(
//     AnimationFillModeType,
//     animation,
//     fill_mode,
//     AnimationFillMode,
//     SmallVec<[AnimationFillMode; 1]>
// );
// impl_style!(
//     AnimationPlayStateType,
//     animation,
//     play_state,
//     AnimationPlayState,
//     SmallVec<[AnimationPlayState; 1]>
// );

other_out_export!(
    set_class,
    gui,
    node,
    {
        let mut s = SmallVec::with_capacity(class_name.len());
        for i in class_name.iter() {
            s.push(*i as usize);
        }
        gui.gui.set_class(node, ClassName(s));
    },
    class_name: Vec<f64>,
);


// #[cfg(target_arch = "wasm32")]
// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn set_default_style_by_bin(gui: &mut Gui, bin: &[u8]) { gui.gui.set_default_style_by_bin(bin); }

// #[cfg(target_arch = "wasm32")]
// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn set_default_style_by_str(gui: &mut Gui, bin: &str) { gui.gui.set_default_style_by_str(bin); }

pub fn to_linear_gradient_color(color_and_positions: &[f32], direction: f32) -> LinearGradientColor {
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
