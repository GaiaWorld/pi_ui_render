//! 将设置布局属性的接口导出到js


use std::{
    mem::transmute,
};

// use crate::components::user::{
//     BorderImageSlice, BorderRadius, CgColor, ClassName, Color, ColorAndPosition, FontSize, Hsi, ImageRepeat, LengthUnit, LineHeight,
//     LinearGradientColor, MaskImage, Stroke, TextContent, TransformFunc, TransformOrigin,
// };
use crate::components::{calc::EntityKey, NodeBundle};
use crate::resource::NodeCmd;
use pi_null::Null;
use crate::components::user::ClassName;
use crate::export::json_parse::as_value;
use bevy::ecs::prelude::Entity;
use ordered_float::NotNan;
use pi_flex_layout::prelude::*;
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_style::style::*;
use pi_style::{
    style_type::*,
};
use pi_style::style_parse::{parse_comma_separated, parse_text_shadow, StyleParse};
use smallvec::SmallVec;
pub use crate::export::{Gui, Atom};
pub use super::Engine;
use crate::system::RunState;
use pi_bevy_render_plugin::FrameState;

#[cfg(feature="wasm_bindgen")]
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
	($func_name:ident, $context: ident, $node: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="wasm_bindgen")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($context: &mut Gui, $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}

		$crate::paste::item! {
			pub fn [<play_ $func_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = -1;
				i += 1;
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap()))}.index();
				$(i += 1; let $name_ref = super::json_parse::as_value::<$ty_ref>(json, i as usize).unwrap(); let $name_ref = &$name_ref;)*
				$(i += 1; let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				$func_name(gui, node,  $($name_ref,)* $($name,)*);
			}
		}
	};

	($func_name:ident, [$($context: ident: $context_ty: ty,)*], $expr:expr, [$($name_ref: ident: &$ty_ref: ty,)*], [$($name: ident: $ty: ty,)*]) => {
		#[cfg(feature="wasm_bindgen")]
		#[wasm_bindgen]
		pub fn $func_name($($context: $context_ty,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($($context: $context_ty,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		$crate::paste::item! {
			pub fn [<play_ $func_name>]($($context: $context_ty,)* _context: &mut PlayContext, _json: &Vec<json::JsonValue>) {
				let mut _i = -1;
				$(_i += 1; let $name_ref = super::json_parse::as_value::<$ty_ref>(_json, _i as usize).unwrap(); let $name_ref = &$name_ref;)*
				$(_i += 1; let $name = super::json_parse::as_value::<$ty>(_json, _i as usize).unwrap();)*
				$func_name($($context,)* $($name_ref,)* $($name,)*);
			}
		}
	};

	($func_name:ident, $context: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="wasm_bindgen")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($context: &mut Gui, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		$crate::paste::item! {
			#[allow(unused_variables)]
			pub fn [<play_ $func_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let i = -1;
				$(let i = i + 1; let $name_ref = super::json_parse::as_value::<$ty_ref>(json, i as usize).unwrap();let $name_ref = &$name_ref;)*
				$(let i = i + 1; let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
				$func_name(gui, $($name_ref,)* $($name,)*);
			}
		}
	};

	// 带返回值的接口
	(@with_return_node, $func_name:ident, $context: ident: $context_ty: ty, $node: ident, $return_ty: ty, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="wasm_bindgen")]
		#[wasm_bindgen]
		pub fn $func_name($context: $context_ty, $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($context: $context_ty, $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}
	};

	// 带返回值的接口
	(@with_return, $func_name:ident, $return_ty: ty, $expr:expr, $($mut_name_ref: ident: &mut $mut_ty_ref: ty,)*; $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="wasm_bindgen")]
		#[wasm_bindgen]
		pub fn $func_name($($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			$expr
		}

		#[cfg(not(target_arch = "wasm32"))]
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			$expr
		}
	};
}

#[macro_export]
macro_rules! style_out_export {
	(@dimension_box $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@dimension_inner  [<$attr_name _percent>], $last_ty, Dimension::Percent(v),; v: f32, );
			style_out_export!(@dimension_inner $attr_name, $last_ty, Dimension::Points(v),; v: f32, );
			style_out_export!(@dimension_inner  [<$attr_name _auto>], $last_ty, Dimension::Auto,; );
		}
	};

	(@dimension $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@expr  [<$attr_name _percent>], $last_ty, Dimension::Percent(v),; v: f32, );
			style_out_export!(@expr $attr_name, $last_ty, Dimension::Points(v),; v: f32, );
			style_out_export!(@expr  [<$attr_name _auto>], $last_ty, Dimension::Auto,; );
		}
	};

	(@cenum $attr_name:ident, $last_ty: ident) => {
		style_out_export!(@expr $attr_name, $last_ty, unsafe {transmute(v as u8)},; v: f64,);
	};

	(@expr $attr_name:ident, $last_ty: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = 0;
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap()))}.index();
				i += 1;
				$(let $name_ref = super::json_parse::as_value::<$ty_ref>(json, i).unwrap(); i += 1;let $name_ref = &$name_ref;)*
				$(let $name = super::json_parse::as_value::<$ty>(json, i).unwrap(); i += 1;)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				[<set_ $attr_name>](gui, node, $($name_ref,)* $($name,)*);
			}
		}
    };

	(@dimension_inner $attr_name:ident, $last_ty: ident, $expr: expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
			#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.commands.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.commands.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.commands.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.commands.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.commands.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.commands.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.commands.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.commands.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
			#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.commands.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.commands.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.commands.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.commands.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.commands.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.commands.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.commands.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.commands.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
				let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node, edge);
			}

			#[allow(unused_variables)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let mut i = -1;
				i += 1;
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap()))}.index();
				i += 1;
				let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
				$(i += 1;let $name_ref = super::json_parse::as_value::<$ty_ref>(json, i as usize).unwrap();let $name_ref = &$name_ref;)*
				$(i += 1;let $name = super::json_parse::as_value::<$ty>(json, i as usize).unwrap();)*
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				[<set_ $attr_name>](gui, node, edge, $($name_ref,)* $($name,)*);
			}
		}
	};

	(@atom $attr_name:ident, $last_ty: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(feature="wasm_bindgen")]
			#[allow(unused_attributes)]
       		#[wasm_bindgen]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(feature="pi_js_export")]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_reset_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
				let node = match context.nodes.get(node as usize) {
					Some(r) => r.clone(),
					None => return,
				};
				[<reset_ $attr_name>](gui, node);
			}

			#[allow(unused_variables)]
			#[allow(unused_assignments)]
			pub fn [<play_ $attr_name>](gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
				let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
				let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
				// let node = context.nodes.get(node).unwrap().clone();
				let node = match context.nodes.get(node as usize) {
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

style_out_export!(@expr flex_grow, FlexGrowType, v, ; v: f32,);
style_out_export!(@expr flex_shrink, FlexGrowType, v, ; v: f32,);

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

style_out_export!(@expr background_rgba_color, BackgroundColorType, Color::RGBA(CgColor::new(r, g, b, a)), ; r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	background_linear_color, 
	BackgroundColorType, 
	Color::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )), ;
	direction: f32, color_and_positions: Vec<f32>,);

style_out_export!(@expr 
	border_color,
	BorderColorType,
	CgColor::new(r, g, b, a),;
	r: f32, g: f32, b: f32, a: f32,);

style_out_export!(@expr
	border_radius,
	BorderRadiusType,
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let border_radius = pi_style::style_parse::parse_border_radius(&mut parse);
		if let Ok(value) = border_radius {
			value
		} else {
			Default::default()
		}
	},
	s: &str,; );

style_out_export!(@expr 
	box_shadow,
	BoxShadowType,
	BoxShadow {
		h: h,
		v: v,
		blur: blur,
		spread: spread,
		color: CgColor::new(r, g, b, a)
	},;
	h: f32, v: f32, blur: f32, spread: f32, r: f32, g: f32 ,b: f32, a: f32,);
style_out_export!(@cenum object_fit, ObjectFitType);

style_out_export!(@expr background_repeat, BackgroundRepeatType, ImageRepeat {
		x: unsafe { transmute(x as u8) },
		y: unsafe { transmute(y as u8) },
	},;
	x: u8, y: u8, );

style_out_export!(@expr 
	mask_image_linenear,
	MaskImageType,
	MaskImage::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )),;
	direction: f32, color_and_positions: Vec<f32>, );

style_out_export!(@expr 
	image_clip,
	BackgroundImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	mask_image_clip,
	MaskImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	border_image_clip,
	BorderImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
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
	},;
	top: f32, right: f32, bottom: f32, left: f32, fill: bool,);

style_out_export!(@expr 
	border_image_repeat,
	BorderImageRepeatType,
	ImageRepeat {
		x: unsafe { transmute(vertical as u8) },
		y: unsafe { transmute(horizontal as u8) },
	},;
	vertical: u8, horizontal: u8, );

style_out_export!(@expr overflow, OverflowType, v,; v: bool,);
style_out_export!(@expr opacity, OpacityType, v,; v: f32,);
style_out_export!(@cenum display, DisplayType);
style_out_export!(@expr visibility, VisibilityType, v,; v: bool,);
style_out_export!(@cenum enable, EnableType);
style_out_export!(@cenum blend_mode, BlendModeType);
style_out_export!(@expr zindex, ZIndexType, v as isize,; v: i32,);
style_out_export!(@expr filter_blur, BlurType, v,; v: f32,);

// hsi, 效果与ps一致,  h: -180 ~ 180, s: -100 ~ 100, i: -100 ~ 100
style_out_export!(@expr 
	filter_hsi,
	HsiType,
	{
		let (mut h, mut s, mut _i) = (h, s, _i);
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
		if _i > 100.0 {
			_i = 100.0;
		} else if _i < -100.0 {
			_i = -100.0
		}
		Hsi {
			hue_rotate: h / 360.0,
			saturate: s / 100.0,
			bright_ness: _i / 100.0,
		}
	},;
	h: f32, s: f32, _i: f32, );
/************************************ Transform **************************************/
style_out_export!(@expr 
	transform_translate, 
	TransformFuncType, 
	TransformFunc::Translate(x, y),;
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_translate_percent, 
	TransformFuncType, 
	TransformFunc::TranslatePercent(x, y),;
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_translate_x, 
	TransformFuncType, 
	TransformFunc::TranslateX(x),;
	x: f32,);
style_out_export!(@expr 
	transform_translate_x_percent, 
	TransformFuncType, 
	TransformFunc::TranslateXPercent(x),;
	x: f32,);
style_out_export!(@expr 
	transform_translate_y, 
	TransformFuncType, 
	TransformFunc::TranslateY(y),;
	y: f32,);
style_out_export!(@expr 
	transform_translate_y_percent, 
	TransformFuncType, 
	TransformFunc::TranslateYPercent(y),;
	y: f32,);
style_out_export!(@expr 
	transform_scale, 
	TransformFuncType, 
	TransformFunc::Scale(x, y),;
	x: f32, y: f32,);
style_out_export!(@expr 
	transform_scale_x, 
	TransformFuncType, 
	TransformFunc::ScaleX(x),;
	x: f32,);
style_out_export!(@expr 
	transform_scale_y, 
	TransformFuncType, 
	TransformFunc::ScaleY(y),;
	y: f32,);
style_out_export!(@expr 
	transform_rotate_x, 
	TransformFuncType, 
	TransformFunc::RotateX(x),;
	x: f32,);
style_out_export!(@expr 
	transform_rotate_y, 
	TransformFuncType, 
	TransformFunc::RotateY(y),;
	y: f32,);
style_out_export!(@expr 
	transform_rotate_z, 
	TransformFuncType, 
	TransformFunc::RotateZ(z),;
	z: f32,);
style_out_export!(@expr 
	transform_skew_x, 
	TransformFuncType, 
	TransformFunc::SkewX(x),;
	x: f32,);
style_out_export!(@expr 
	transform_skew_y, 
	TransformFuncType, 
	TransformFunc::SkewY(y),;
	y: f32,);
style_out_export!(@expr 
	clear_transform, 
	TransformType, 
	Vec::new(),;);
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
	},;
	x_ty: f64, x: f32, y_ty: f64, y: f32,);

// 设置transform为None TODO

style_out_export!(@expr letter_spacing, LetterSpacingType, v,; v: f32,);
style_out_export!(@expr word_spacing, WordSpacingType, v,; v: f32,);

style_out_export!(@expr text_rgba_color, ColorType, Color::RGBA(CgColor::new(r, g, b, a)),; r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	text_linear_gradient_color, 
	ColorType, 
	Color::LinearGradient(to_linear_gradient_color(
		color_and_positions.as_slice(),
		direction,
	)),; direction: f32, color_and_positions: Vec<f32>, );
style_out_export!(@expr line_height_normal, LineHeightType, LineHeight::Normal,;);
style_out_export!(@expr line_height, LineHeightType, LineHeight::Length(value),; value: f32,);
style_out_export!(@expr line_height_percent, LineHeightType, LineHeight::Percent(value), ;value: f32,);
style_out_export!(@expr text_indent, TextIndentType, v,; v: f32,);
style_out_export!(@cenum text_align, TextAlignType);
style_out_export!(@expr text_stroke, TextStrokeType, Stroke {
	width: NotNan::new(width).expect("stroke width is nan"),
	color: CgColor::new(r, g, b, a),
},; width: f32, r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@cenum white_space, WhiteSpaceType);
style_out_export!(@cenum font_style, FontStyleType);
style_out_export!(@expr font_weight, FontWeightType, v as usize,; v: f64,);
style_out_export!(@expr font_size_none, FontSizeType, FontSize::None,;);
style_out_export!(@expr font_size, FontSizeType, FontSize::Length(value as usize),; value: f64,);
style_out_export!(@expr font_size_percent, FontSizeType, FontSize::Percent(value),; value: f32,);
style_out_export!(@expr text_content_utf8, TextContentType, {
	let content = unsafe{String::from_utf8_unchecked(content)};
	TextContent(content, pi_atom::Atom::from(""))
},; content: Vec<u8>,);
style_out_export!(@expr clip_path_str, ClipPathType, {
	let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);

    match BaseShape::parse(&mut parse) {
        Ok(r) => r,
        Err(e) => {
            log::error!("set_animation_str fail, animation: {}, err: {:?}", value, e);
            return;
        }
    }
}, value: &str,;);



// animation
style_out_export!(@expr animation_duration, AnimationDurationType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) },; name: Vec<usize>,);
style_out_export!(@expr animation_delay, AnimationDelayType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) },; name: Vec<usize>,);
style_out_export!(@expr animation_iteration_count, AnimationIterationCountType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>()) },; name: Vec<f32>,);
style_out_export!(@expr animation_direction, AnimationDirectionType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_fill_mode, AnimationFillModeType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_play_state, AnimationPlayStateType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_name_str, AnimationNameType, {
	let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);
    let value = if let Ok(value) =
        parse_comma_separated::<_, _, cssparser::ParseError<pi_style::style_parse::ValueParseErrorKind>>(&mut parse, |input| Ok(pi_atom::Atom::from(input.expect_ident()?.as_ref())))
    {
        value
    } else {
        Default::default()
    };
	AnimationName {
		scope_hash: scope_hash as usize,
		value,
	}
},value: &str,; scope_hash: u32,);

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature="pi_js_export")]
pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    set_animation_str_inner(gui, node_id, value, scope_hash);
}

#[cfg(feature="wasm_bindgen")]
#[wasm_bindgen]
pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    set_animation_str_inner(gui, node_id, value, scope_hash);
}



#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature="pi_js_export")]
pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
    reset_animation_str_inner(gui, node_id);
}

#[cfg(feature="wasm_bindgen")]
#[wasm_bindgen]
pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
    reset_animation_str_inner(gui, node_id);
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn play_reset_animation_str(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
	let node = match context.nodes.get(node as usize) {
		Some(r) => r.clone(),
		None => return,
	};
	reset_animation_str(gui, node);
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn play_animation_str(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let node = unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))}.index();
	// let node = context.nodes.get(node).unwrap().clone();
	let node = match context.nodes.get(node as usize) {
		Some(r) => r.clone(),
		None => return,
	};
	let value = super::json_parse::as_value::<String>(json, 1).unwrap();
	let scope_hash = super::json_parse::as_value::<u32>(json, 2).unwrap();
	set_animation_str(gui, node, &value, scope_hash);
}


// #[cfg(feature="pi_js_export")]
// pub fn set_animation_name(_gui: &mut Gui, _scope_hash: u32, _name: &Vec<String>) {

// }
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

style_out_export!(@atom 
	mask_image,
	MaskImageType,
	MaskImage::Path(image_hash.0.clone()),
	image_hash: &Atom,; );

style_out_export!(@atom 
	background_image,
	BackgroundImageType,
	image_hash.0.clone(),
	image_hash: &Atom,; );
style_out_export!(@atom 
	border_image,
	BorderImageType,
	image_hash.0.clone(),
	image_hash: &Atom,; );
style_out_export!(@expr text_shadow, TextShadowType, {
	let mut input = cssparser::ParserInput::new(s);
	let mut parse = cssparser::Parser::new(&mut input);

	let shadows = parse_text_shadow(&mut parse);
	if let Ok(value) = shadows {
		value
	} else {
		Default::default()
	}
}, s: &str,;);
style_out_export!(@atom font_family, FontFamilyType, name.0.clone(), name: &Atom,;);
style_out_export!(@expr text_content, TextContentType,  TextContent(content, pi_atom::Atom::from("")), ;content: String,);
style_out_export!(@expr animation_timing_function_str, AnimationTimingFunctionType, { 
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);

	if let Ok(value) = parse_comma_separated(&mut parse, <AnimationTimingFunction as StyleParse>::parse) {
		value
	} else {
		Default::default()
	}
}, value: &str,;);

other_out_export!(set_default_style, gui, gui.commands.set_default_style_by_str(value, 0), value: &str,;);

other_out_export!(
    create_class,
    gui,
    {
        gui.commands.add_css(css, scope_hash as usize);
    },
	css: &str,
	;
    scope_hash: u32,
);

other_out_export!(
    set_class,
    gui,
    node,
    {
        let mut s = SmallVec::with_capacity(class_name.len());
        for i in class_name.iter() {
            s.push(*i as usize);
        }
        gui.commands.set_class(node, ClassName(s));
    },;
    class_name: Vec<u32>,
);

// /// 创建容器节点， 容器节点可设置背景颜色
// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(feature = "pi_js_export")]
// pub fn create_node(gui: &mut Gui) -> f64 {
//     use crate::components::NodeBundle;

//     let entity = gui.entitys.reserve_entity();
//     gui.commands.push_cmd(NodeCmd(NodeBundle::default(), entity));
//     // log::warn!("entity :{:?}", entity);
//     unsafe { transmute(entity.to_bits()) }
// }

// 创建节点
other_out_export!(
	@with_return,
    create_node,
	f64,
	{
		let entity = gui.entitys.reserve_entity();
		gui.commands.push_cmd(NodeCmd(NodeBundle::default(), entity));
		// log::warn!("entity :{:?}", entity);
		unsafe { transmute(entity.to_bits()) }
	},
	gui: &mut Gui,;
	;
);

other_out_export!(
	@with_return,
    create_vnode,
	f64,
	create_node(gui),
	gui: &mut Gui,;
	;
);

other_out_export!(
	@with_return,
    create_text_node,
	f64,
	create_node(gui),
	gui: &mut Gui,;
	;
);

other_out_export!(
	@with_return,
    create_image_node,
	f64,
	create_node(gui),
	gui: &mut Gui,;
	;
);

other_out_export!(
	@with_return,
    create_canvas_node,
	f64,
	create_node(gui),
	gui: &mut Gui,;
	;
);

other_out_export!(
    destroy_node,
    gui,
    node,
    {gui.commands.destroy_node(Entity::from_bits(unsafe { transmute(node) }));},;
);

other_out_export!(
    insert_as_root,
    gui,
    node,
    {
		gui.commands.append(node, unsafe { transmute(EntityKey::null().to_bits())});
	},;
);

other_out_export!(
    append_child,
    gui,
    node,
    {	
		let parent = Entity::from_bits(unsafe { transmute(parent) });
		gui.commands.append(node, parent);
	},;
	parent: f64,
);

other_out_export!(
    insert_before,
    gui,
    node,
    {
		gui.commands.insert_before(
			node,
			Entity::from_bits(unsafe { transmute(borther) }),
		);
	},;
	borther: f64,
);

other_out_export!(
    remove_node,
    gui,
    node,
    {
		gui.commands.remove_node(node);
	},;
);

other_out_export!(
    set_view_port,
    gui,
    {
		let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
		gui.commands.push_cmd(NodeCmd(
			crate::components::user::Viewport(Aabb2::new(Point2::new(x as f32, y as f32), Point2::new(width as f32, height as f32))),
			root,
		));
	},;
	x: i32, y: i32, width: i32, height: i32, root: f64,
);

other_out_export!(
    set_clear_color,
    gui,
    {
		let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
		gui.commands
			.push_cmd(NodeCmd(crate::components::user::ClearColor(CgColor::new(r, g, b, a), is_clear_window), root));
	},;
	r: f32, g: f32, b: f32, a: f32, root: f64, is_clear_window: bool,
);

other_out_export!(
    create_class_by_bin,
    gui,
    match bincode::deserialize::<Vec<pi_style::style_parse::ClassMap>>(bin) {
		Ok(r) => {
			gui.commands.push_cmd(crate::resource::ExtendCssCmd(r));
		}
		Err(e) => {
			log::warn!("deserialize_class_map error: {:?}, {:?}", e, bin);
			return;
		}
	},
	bin: &[u8],;
	
);


// 调试使用， 设置渲染脏， 使渲染系统在下一帧进行渲染
other_out_export!(
    set_render_dirty,
    gui,
    {
		let node: Entity = Entity::from_bits(unsafe { transmute(root) });
    	gui.commands.push_cmd(NodeCmd(crate::components::user::RenderDirty(true), node));
	},;
	root: f64,
);

other_out_export!(
    fram_call,
    [engine: &mut Engine,],
    {
		*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::RENDER;
		*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
		engine.update();
	},
	[],
	[_cur_time: u32,]
);

// 
other_out_export!(
    flush,
    [gui: &mut Gui, engine: &mut Engine,],
    {
		bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
		let mut com = engine.world.get_resource_mut::<crate::prelude::UserCommands>().unwrap();
		std::mem::swap(&mut gui.commands, &mut *com);
		*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::SETTING;
		*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
		engine.update();
	},
	[],
	[]
);

other_out_export!(
    calc,
    [gui: &mut Gui, engine: &mut Engine,],
    {	
		bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
		let mut com = engine.world.get_resource_mut::<crate::prelude::UserCommands>().unwrap();
		std::mem::swap(&mut gui.commands, &mut *com);
		*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
		*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
		engine.update();
	},
	[],
	[]
);

other_out_export!(
    calc_layout,
    [gui: &mut Gui, engine: &mut Engine,],
    {	
		log::error!("calc_layout!!!!!");
		bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
		let mut com = engine.world.get_resource_mut::<crate::prelude::UserCommands>().unwrap();
		std::mem::swap(&mut gui.commands, &mut *com);
		*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::LAYOUT;
		*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
		engine.update();
	},
	[],
	[]
);

other_out_export!(
    calc_geo,
    [gui: &mut Gui, engine: &mut Engine,],
    {
		log::error!("calc_geo!!!!!");
		bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
		let mut com = engine.world.get_resource_mut::<crate::prelude::UserCommands>().unwrap();
		std::mem::swap(&mut gui.commands, &mut *com);
		*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
		*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
		engine.update();
	},
	[],
	[]
);

// TODO
other_out_export!(
    bind_render_target,
    _gui,
    {},;
);

// TODO
other_out_export!(
    set_pixel_ratio,
    _gui,
    {
		
	},;
	_pixel_ratio: f32,
);

// TODO
other_out_export!(
    nullify_clear_color,
    _gui,
    {
		
	},;
);

// TODO
other_out_export!(
    set_scissor,
    _gui,
    {
		
	},;
	_x: i32, _y: i32, _width: i32, _height: i32,
);

// TODO
other_out_export!(
    set_project_transfrom,
    _gui,
    {
		
	},;
	_scale_x: f32, _scale_y: f32, _translate_x: f32, _translate_y: f32, _rotate: f32, _width: f64, _height: f64,
);

// TODO
other_out_export!(
    force_update_text,
    _gui,
	_node,
    {
		
	},;
);

// TODO
//设置shader
other_out_export!(
    set_shader,
    _gui,
    {
	
	},
	_shader_name: &str, _shader_code: &str,
	;
	
);

// TODO
other_out_export!(
	@with_return, 
    get_font_sheet,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_class_sheet,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    create_render_target,
	u32,
    0,
	_gui: &mut Gui,;
	;
	_fbo: f64,
);

// TODO
other_out_export!(
	@with_return, 
    destroy_render_target,
	u32,
    0,
	_gui: &mut Gui,;
	;
	_fbo: f64,
);

// TODO
other_out_export!(
	@with_return, 
    clone_engine,
	Gui,
    todo!(),
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_text_texture_width,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_text_texture_height,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    node_is_exist,
	bool,
    true,
	_gui: &mut Gui,;
	;
	_node: f64,
);

// TODO
other_out_export!(
	@with_return, 
    node_is_visibility,
	bool,
    true,
	_gui: &mut Gui,;
	;
	_node: f64,
);

type ReturnNode = Option<f64>;

// TODO
other_out_export!(
	@with_return_node, 
    first_child,
    _gui: &Gui,
	_parent,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    last_child,
    _gui: &Gui,
	_parent,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    next_sibling,
    _gui: &Gui,
	_node,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    previous_sibling,
    _gui: &Gui,
	_node,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    get_layer,
    _gui: &Gui,
	_node,
	u32,
    0,;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点到gui的上边界的距离
// TODO
other_out_export!(
	@with_return_node, 
    offset_top,
    _gui: &Gui,
	_node,
	u32,
    0,;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点到gui的左边界的距离
// TODO
other_out_export!(
	@with_return_node, 
    offset_left,
    _gui: &Gui,
	_node,
	u32,
    0,;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点的布局宽度
// TODO
other_out_export!(
	@with_return_node, 
    offset_width,
    _gui: &Gui,
	_node,
	u32,
    0,;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点布局高度
// TODO
other_out_export!(
	@with_return_node, 
    offset_height,
    _gui: &Gui,
	_node,
	u32,
    0,;
);

other_out_export!(
	@with_return, 
    get_atom,
	Atom,
    Atom(pi_atom::Atom::from(value)),;;value: String,
);

other_out_export!(
	@with_return, 
    get_atom_hash,
	f64,
    value.get_hash() as f64,;value: &Atom,;
);

other_out_export!(
	@with_return, 
    get_string_by_hash,
	Option<String>,
    match pi_atom::Atom::get(value as usize) {
        Some(r) => Some(r.as_ref().to_string()),
        None => None,
    },;;value: u32,
);

other_out_export!(
	@with_return, 
    get_entity_offset,
	u32,
    {
		let r = unsafe {Entity::from_bits(transmute::<f64, u64>(value))};
    	r.index()
	},;;value: f64,
);

other_out_export!(
	@with_return, 
    get_entity_version,
	u32,
    {
		let r = unsafe {Entity::from_bits(transmute::<f64, u64>(value))};
    	r.generation()
	},;;value: f64,
);

other_out_export!(
	@with_return, 
    query,
	Option<f64>,
    super::query(engine, gui, x, y),
	engine: &mut Engine, gui: &mut Gui,;;x: f32, y: f32,
);

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

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

#[inline]
fn set_animation_str_inner(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    use pi_style::style_parse::parse_animation;

    let node_id = Entity::from_bits(unsafe { transmute(node_id) });
    let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);

    let mut animations = match parse_animation(&mut parse) {
        Ok(r) => r,
        Err(e) => {
            log::error!("set_animation_str fail, animation: {}, err: {:?}", value, e);
            return;
        }
    };
    animations.name.scope_hash = scope_hash as usize;
    log::debug!("set_animation_str: {:?}", animations);
    if animations.name.value.len() > 0 {
        gui.commands.set_style(node_id, AnimationNameType(animations.name));
        gui.commands.set_style(node_id, AnimationDurationType(animations.duration));
        gui.commands.set_style(node_id, AnimationTimingFunctionType(animations.timing_function));
        gui.commands.set_style(node_id, AnimationIterationCountType(animations.iteration_count));
        gui.commands.set_style(node_id, AnimationDelayType(animations.delay));
        gui.commands.set_style(node_id, AnimationDirectionType(animations.direction));
        gui.commands.set_style(node_id, AnimationFillModeType(animations.fill_mode));
        gui.commands.set_style(node_id, AnimationPlayStateType(animations.play_state));
    }
}

#[inline]
fn reset_animation_str_inner(gui: &mut Gui, node_id: f64) {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });
    gui.commands.set_style(node_id, ResetAnimationNameType);
    gui.commands.set_style(node_id, ResetAnimationDurationType);
    gui.commands.set_style(node_id, ResetAnimationIterationCountType);
    gui.commands.set_style(node_id, ResetAnimationDelayType);
    gui.commands.set_style(node_id, ResetAnimationDirectionType);
    gui.commands.set_style(node_id, ResetAnimationFillModeType);
    gui.commands.set_style(node_id, ResetAnimationPlayStateType);
}


pub mod debug {
	
}