
extern crate paste;
pub mod style_macro;

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