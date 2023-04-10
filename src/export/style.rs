
    //! 将设置布局属性的接口导出到js
    use std::mem::transmute;
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
    use pi_style::style_type::*;
    use pi_style::style_parse::{parse_comma_separated, parse_text_shadow, StyleParse};
    use smallvec::SmallVec;
    pub use crate::export::{Gui, Atom};
    pub use super::Engine;
    use crate::system::RunState;
    use pi_bevy_render_plugin::FrameState;
    #[cfg(feature = "wasm_bindgen")]
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
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_align_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_align_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_align_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignContentType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_align_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignContentType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_align_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_align_content(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_align_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_align_content(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_align_items(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignItemsType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_align_items(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignItemsType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_align_items(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignItemsType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_align_items(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignItemsType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_align_items(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_align_items(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_align_items(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_align_items(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_justify_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, JustifyContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_justify_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, JustifyContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_justify_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetJustifyContentType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_justify_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetJustifyContentType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_justify_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_justify_content(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_justify_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_justify_content(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_direction(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexDirectionType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_direction(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexDirectionType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexDirectionType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexDirectionType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_direction(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_direction(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_direction(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_direction(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_wrap(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexWrapType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_wrap(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexWrapType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_wrap(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexWrapType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_wrap(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexWrapType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_wrap(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_wrap(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_wrap(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_wrap(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_align_self(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignSelfType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_align_self(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, AlignSelfType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_align_self(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignSelfType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_align_self(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignSelfType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_align_self(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_align_self(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_align_self(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_align_self(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_position_type(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, PositionTypeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_position_type(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, PositionTypeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_position_type(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetPositionTypeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_position_type(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetPositionTypeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_position_type(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_position_type(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_position_type(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_position_type(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_grow(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_grow(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_grow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_grow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_grow(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_grow(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_grow(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_grow(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_shrink(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_shrink(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_shrink(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_shrink(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_shrink(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_shrink(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_shrink(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_shrink(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_basis_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_basis_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_basis_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_basis_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_basis_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_basis_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_basis_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_basis_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_basis(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_basis(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_basis(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_basis(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_basis(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_basis(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_basis(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_basis(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexBasisType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_flex_basis_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_flex_basis_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_flex_basis_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_flex_basis_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WidthType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_width_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_width_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WidthType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_width(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_width(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_width(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_width(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_width_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_width_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_width_auto(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_width_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, HeightType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, HeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_height_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_height_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, HeightType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, HeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_height(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_height(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_height(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_height(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_height_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_height_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_height_auto(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_height_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_width_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_width_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_width(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_width(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_width(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_width(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinWidthType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_width_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_width_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_width_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_width_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_height_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_height_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_height(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_height(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_height(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_height(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MinHeightType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_min_height_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_min_height_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_min_height_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_min_height_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_width_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_width_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_width_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_width(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_width(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_width(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_width(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxWidthType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_width_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_width_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_width_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_width_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Percent(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_height_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_height_percent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Points(v)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_height(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_height(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_height(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_height(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Auto));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaxHeightType(Dimension::Auto));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_max_height_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_max_height_auto(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_max_height_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_max_height_auto(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_padding_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_padding_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_padding_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_padding_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_padding_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_padding_percent(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_padding_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_padding_percent(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_padding(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_padding(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_padding(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_padding(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_padding(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_padding(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_padding(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_padding(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PaddingTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, PaddingRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PaddingBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, PaddingLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_padding_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_padding_auto(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_padding_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_padding_auto(gui, node, edge);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_margin_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_margin_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_margin_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_margin_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_margin_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_margin_percent(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_margin_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_margin_percent(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_margin(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_margin(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_margin(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_margin(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_margin(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_margin(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_margin(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_margin(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, MarginTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, MarginRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, MarginBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, MarginLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_margin_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_margin_auto(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_margin_auto(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_margin_auto(gui, node, edge);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_percent(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_border_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_percent(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_border(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, BorderTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, BorderRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, BorderBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, BorderLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_auto(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_border_auto(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_auto(gui, node, edge);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_position_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_position_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Percent(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Percent(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Percent(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Percent(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_position_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_position_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_position_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_position_percent(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_position_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_position_percent(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_position(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_position(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Points(v))),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Points(v))),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Points(v))),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Points(v))),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_position(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_position(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_position(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_position(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_position(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_position(gui, node, edge, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui
                .commands
                .set_style(node_id, PositionTopType(Dimension::Auto)),
            Edge::Right => gui
                .commands
                .set_style(node_id, PositionRightType(Dimension::Auto)),
            Edge::Bottom => gui
                .commands
                .set_style(node_id, PositionBottomType(Dimension::Auto)),
            Edge::Left => gui
                .commands
                .set_style(node_id, PositionLeftType(Dimension::Auto)),
            _ => return,
        };
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_position_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let edge = super::json_parse::as_value::<f64>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_position_auto(gui, node, edge);
    }
    #[allow(unused_variables)]
    pub fn play_position_auto(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let edge = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_position_auto(gui, node, edge);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_background_rgba_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundColorType(Color::RGBA(CgColor::new(r, g, b, a))),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_background_rgba_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundColorType(Color::RGBA(CgColor::new(r, g, b, a))),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_background_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_background_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_background_rgba_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_background_rgba_color(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_background_rgba_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let r = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let g = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let b = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let a = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_background_rgba_color(gui, node, r, g, b, a);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_background_linear_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundColorType(Color::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_background_linear_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundColorType(Color::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_background_linear_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_background_linear_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_background_linear_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_background_linear_color(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_background_linear_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let direction = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let color_and_positions = super::json_parse::as_value::<Vec<f32>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_background_linear_color(gui, node, direction, color_and_positions);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BorderColorType(CgColor::new(r, g, b, a)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BorderColorType(CgColor::new(r, g, b, a)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderColorType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderColorType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_color(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let r = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let g = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let b = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let a = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_color(gui, node, r, g, b, a);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_radius(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderRadiusType({
                let mut input = cssparser::ParserInput::new(s);
                let mut parse = cssparser::Parser::new(&mut input);
                let border_radius = pi_style::style_parse::parse_border_radius(&mut parse);
                if let Ok(value) = border_radius {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_radius(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderRadiusType({
                let mut input = cssparser::ParserInput::new(s);
                let mut parse = cssparser::Parser::new(&mut input);
                let border_radius = pi_style::style_parse::parse_border_radius(&mut parse);
                if let Ok(value) = border_radius {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_radius(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderRadiusType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_radius(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderRadiusType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_radius(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_radius(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_radius(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let s = super::json_parse::as_value::<str>(json, i).unwrap();
        i += 1;
        let s = &s;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_radius(gui, node, s);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_box_shadow(
        gui: &mut Gui,
        node_id: f64,
        h: f32,
        v: f32,
        blur: f32,
        spread: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BoxShadowType(BoxShadow {
                h: h,
                v: v,
                blur: blur,
                spread: spread,
                color: CgColor::new(r, g, b, a),
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_box_shadow(
        gui: &mut Gui,
        node_id: f64,
        h: f32,
        v: f32,
        blur: f32,
        spread: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BoxShadowType(BoxShadow {
                h: h,
                v: v,
                blur: blur,
                spread: spread,
                color: CgColor::new(r, g, b, a),
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_box_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBoxShadowType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_box_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBoxShadowType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_box_shadow(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_box_shadow(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_box_shadow(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let h = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let blur = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let spread = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let r = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let g = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let b = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let a = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_box_shadow(gui, node, h, v, blur, spread, r, g, b, a);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_object_fit(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ObjectFitType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_object_fit(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ObjectFitType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_object_fit(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetObjectFitType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_object_fit(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetObjectFitType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_object_fit(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_object_fit(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_object_fit(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_object_fit(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_background_repeat(gui: &mut Gui, node_id: f64, x: u8, y: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundRepeatType(ImageRepeat {
                x: unsafe { transmute(x as u8) },
                y: unsafe { transmute(y as u8) },
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_background_repeat(gui: &mut Gui, node_id: f64, x: u8, y: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundRepeatType(ImageRepeat {
                x: unsafe { transmute(x as u8) },
                y: unsafe { transmute(y as u8) },
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_background_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundRepeatType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_background_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundRepeatType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_background_repeat(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_background_repeat(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_background_repeat(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<u8>(json, i).unwrap();
        i += 1;
        let y = super::json_parse::as_value::<u8>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_background_repeat(gui, node, x, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_mask_image_linenear(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageType(MaskImage::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_mask_image_linenear(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageType(MaskImage::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_mask_image_linenear(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_mask_image_linenear(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_mask_image_linenear(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_mask_image_linenear(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_mask_image_linenear(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let direction = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let color_and_positions = super::json_parse::as_value::<Vec<f32>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_mask_image_linenear(gui, node, direction, color_and_positions);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BackgroundImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetBackgroundImageClipType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetBackgroundImageClipType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_image_clip(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_image_clip(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_image_clip(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let u1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let u2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_image_clip(gui, node, u1, v1, u2, v2);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_mask_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_mask_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_mask_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageClipType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_mask_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageClipType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_mask_image_clip(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_mask_image_clip(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_mask_image_clip(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let u1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let u2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_mask_image_clip(gui, node, u1, v1, u2, v2);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_image_clip(gui: &mut Gui, node_id: f64, u1: f32, v1: f32, u2: f32, v2: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageClipType(NotNanRect::new(
                unsafe { NotNan::new_unchecked(v1) },
                unsafe { NotNan::new_unchecked(u2) },
                unsafe { NotNan::new_unchecked(v2) },
                unsafe { NotNan::new_unchecked(u1) },
            )),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageClipType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageClipType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_image_clip(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_image_clip(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_image_clip(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let u1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v1 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let u2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let v2 = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_image_clip(gui, node, u1, v1, u2, v2);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_image_slice(
        gui: &mut Gui,
        node_id: f64,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        fill: bool,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(top) },
                right: unsafe { NotNan::new_unchecked(right) },
                bottom: unsafe { NotNan::new_unchecked(bottom) },
                left: unsafe { NotNan::new_unchecked(left) },
                fill,
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_image_slice(
        gui: &mut Gui,
        node_id: f64,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        fill: bool,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(top) },
                right: unsafe { NotNan::new_unchecked(right) },
                bottom: unsafe { NotNan::new_unchecked(bottom) },
                left: unsafe { NotNan::new_unchecked(left) },
                fill,
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_image_slice(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageSliceType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_image_slice(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageSliceType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_image_slice(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_image_slice(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_image_slice(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let top = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let right = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let bottom = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let left = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let fill = super::json_parse::as_value::<bool>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_image_slice(gui, node, top, right, bottom, left, fill);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_image_repeat(gui: &mut Gui, node_id: f64, vertical: u8, horizontal: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageRepeatType(ImageRepeat {
                x: unsafe { transmute(vertical as u8) },
                y: unsafe { transmute(horizontal as u8) },
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_image_repeat(gui: &mut Gui, node_id: f64, vertical: u8, horizontal: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            BorderImageRepeatType(ImageRepeat {
                x: unsafe { transmute(vertical as u8) },
                y: unsafe { transmute(horizontal as u8) },
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_image_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageRepeatType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_image_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageRepeatType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_image_repeat(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_image_repeat(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_image_repeat(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let vertical = super::json_parse::as_value::<u8>(json, i).unwrap();
        i += 1;
        let horizontal = super::json_parse::as_value::<u8>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_border_image_repeat(gui, node, vertical, horizontal);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_overflow(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OverflowType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_overflow(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OverflowType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_overflow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOverflowType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_overflow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOverflowType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_overflow(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_overflow(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_overflow(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<bool>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_overflow(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_opacity(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OpacityType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_opacity(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OpacityType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_opacity(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOpacityType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_opacity(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOpacityType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_opacity(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_opacity(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_opacity(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_opacity(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_display(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, DisplayType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_display(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, DisplayType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_display(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetDisplayType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_display(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetDisplayType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_display(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_display(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_display(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_display(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_visibility(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, VisibilityType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_visibility(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, VisibilityType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_visibility(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetVisibilityType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_visibility(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetVisibilityType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_visibility(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_visibility(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_visibility(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<bool>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_visibility(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_enable(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, EnableType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_enable(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, EnableType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_enable(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetEnableType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_enable(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetEnableType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_enable(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_enable(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_enable(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_enable(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_blend_mode(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BlendModeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_blend_mode(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BlendModeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_blend_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlendModeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_blend_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlendModeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_blend_mode(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_blend_mode(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_blend_mode(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_blend_mode(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_zindex(gui: &mut Gui, node_id: f64, v: i32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ZIndexType(v as isize));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_zindex(gui: &mut Gui, node_id: f64, v: i32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ZIndexType(v as isize));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_zindex(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetZIndexType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_zindex(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetZIndexType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_zindex(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_zindex(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_zindex(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<i32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_zindex(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_filter_blur(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlurType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_filter_blur(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlurType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_filter_blur(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlurType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_filter_blur(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlurType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_filter_blur(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_filter_blur(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_filter_blur(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_filter_blur(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_filter_hsi(gui: &mut Gui, node_id: f64, h: f32, s: f32, _i: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            HsiType({
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
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_filter_hsi(gui: &mut Gui, node_id: f64, h: f32, s: f32, _i: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            HsiType({
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
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_filter_hsi(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHsiType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_filter_hsi(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHsiType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_filter_hsi(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_filter_hsi(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_filter_hsi(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let h = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let s = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let _i = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_filter_hsi(gui, node, h, s, _i);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::Translate(x, y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::Translate(x, y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate(gui, node, x, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate_percent(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslatePercent(x, y)),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate_percent(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslatePercent(x, y)),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate_percent(gui, node, x, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::TranslateX(x)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::TranslateX(x)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate_x(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate_x(gui, node, x);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate_x_percent(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslateXPercent(x)),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate_x_percent(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslateXPercent(x)),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate_x_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate_x_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate_x_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate_x_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate_x_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate_x_percent(gui, node, x);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::TranslateY(y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::TranslateY(y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate_y(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate_y(gui, node, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_translate_y_percent(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslateYPercent(y)),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_translate_y_percent(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformFuncType(TransformFunc::TranslateYPercent(y)),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_translate_y_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_translate_y_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_translate_y_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_translate_y_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_translate_y_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_translate_y_percent(gui, node, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_scale(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::Scale(x, y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_scale(gui: &mut Gui, node_id: f64, x: f32, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::Scale(x, y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_scale(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_scale(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_scale(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_scale(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_scale(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_scale(gui, node, x, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_scale_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::ScaleX(x)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_scale_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::ScaleX(x)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_scale_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_scale_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_scale_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_scale_x(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_scale_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_scale_x(gui, node, x);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_scale_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::ScaleY(y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_scale_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::ScaleY(y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_scale_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_scale_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_scale_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_scale_y(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_scale_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_scale_y(gui, node, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_rotate_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateX(x)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_rotate_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateX(x)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_rotate_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_rotate_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_rotate_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_rotate_x(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_rotate_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_rotate_x(gui, node, x);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_rotate_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateY(y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_rotate_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateY(y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_rotate_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_rotate_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_rotate_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_rotate_y(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_rotate_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_rotate_y(gui, node, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_rotate_z(gui: &mut Gui, node_id: f64, z: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateZ(z)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_rotate_z(gui: &mut Gui, node_id: f64, z: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::RotateZ(z)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_rotate_z(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_rotate_z(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_rotate_z(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_rotate_z(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_rotate_z(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let z = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_rotate_z(gui, node, z);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_skew_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::SkewX(x)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_skew_x(gui: &mut Gui, node_id: f64, x: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::SkewX(x)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_skew_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_skew_x(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_skew_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_skew_x(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_skew_x(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_skew_x(gui, node, x);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_skew_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::SkewY(y)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_skew_y(gui: &mut Gui, node_id: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TransformFuncType(TransformFunc::SkewY(y)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_skew_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_skew_y(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformFuncType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_skew_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_skew_y(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_skew_y(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_skew_y(gui, node, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_clear_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TransformType(Vec::new()));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_clear_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TransformType(Vec::new()));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_clear_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_clear_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_clear_transform(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_clear_transform(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_clear_transform(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_clear_transform(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_transform_origin(gui: &mut Gui, node_id: f64, x_ty: f64, x: f32, y_ty: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformOriginType({
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
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_transform_origin(gui: &mut Gui, node_id: f64, x_ty: f64, x: f32, y_ty: f64, y: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TransformOriginType({
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
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_transform_origin(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformOriginType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_transform_origin(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformOriginType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_transform_origin(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_transform_origin(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_transform_origin(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let x_ty = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let x = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let y_ty = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let y = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_transform_origin(gui, node, x_ty, x, y_ty, y);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_letter_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LetterSpacingType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_letter_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LetterSpacingType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_letter_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLetterSpacingType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_letter_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLetterSpacingType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_letter_spacing(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_letter_spacing(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_letter_spacing(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_letter_spacing(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_word_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WordSpacingType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_word_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WordSpacingType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_word_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWordSpacingType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_word_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWordSpacingType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_word_spacing(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_word_spacing(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_word_spacing(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_word_spacing(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_rgba_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ColorType(Color::RGBA(CgColor::new(r, g, b, a))));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_rgba_color(gui: &mut Gui, node_id: f64, r: f32, g: f32, b: f32, a: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ColorType(Color::RGBA(CgColor::new(r, g, b, a))));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_rgba_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_rgba_color(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_rgba_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let r = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let g = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let b = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let a = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_rgba_color(gui, node, r, g, b, a);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_linear_gradient_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            ColorType(Color::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_linear_gradient_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            ColorType(Color::LinearGradient(to_linear_gradient_color(
                color_and_positions.as_slice(),
                direction,
            ))),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_linear_gradient_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_linear_gradient_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_linear_gradient_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_linear_gradient_color(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_linear_gradient_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let direction = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let color_and_positions = super::json_parse::as_value::<Vec<f32>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_linear_gradient_color(gui, node, direction, color_and_positions);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Normal));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Normal));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_line_height_normal(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_line_height_normal(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_line_height_normal(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_line_height_normal(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_line_height(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Length(value)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_line_height(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Length(value)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_line_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_line_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_line_height(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_line_height(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_line_height(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_line_height(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_line_height_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Percent(value)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_line_height_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, LineHeightType(LineHeight::Percent(value)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_line_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_line_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_line_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_line_height_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_line_height_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_line_height_percent(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_indent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TextIndentType(v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_indent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TextIndentType(v));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_indent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextIndentType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_indent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextIndentType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_indent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_indent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_indent(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_indent(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TextAlignType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, TextAlignType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextAlignType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextAlignType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_align(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_align(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_align(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_align(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_stroke(
        gui: &mut Gui,
        node_id: f64,
        width: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextStrokeType(Stroke {
                width: NotNan::new(width).expect("stroke width is nan"),
                color: CgColor::new(r, g, b, a),
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_stroke(
        gui: &mut Gui,
        node_id: f64,
        width: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextStrokeType(Stroke {
                width: NotNan::new(width).expect("stroke width is nan"),
                color: CgColor::new(r, g, b, a),
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_stroke(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextStrokeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_stroke(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextStrokeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_stroke(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_stroke(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_stroke(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let width = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let r = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let g = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let b = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let a = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_stroke(gui, node, width, r, g, b, a);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_white_space(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WhiteSpaceType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_white_space(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, WhiteSpaceType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_white_space(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWhiteSpaceType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_white_space(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWhiteSpaceType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_white_space(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_white_space(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_white_space(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_white_space(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_style(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontStyleType(unsafe { transmute(v as u8) }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_style(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontStyleType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_style(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontStyleType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_style(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontStyleType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_style(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_style(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_style(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_font_style(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_weight(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontWeightType(v as usize));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_weight(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontWeightType(v as usize));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_weight(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontWeightType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_weight(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontWeightType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_weight(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_weight(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_weight(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let v = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_font_weight(gui, node, v);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::None));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::None));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_size_none(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_size_none(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_size_none(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_font_size_none(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_size(gui: &mut Gui, node_id: f64, value: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::Length(value as usize)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_size(gui: &mut Gui, node_id: f64, value: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::Length(value as usize)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_size(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_size(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_size(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_size(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_size(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<f64>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_font_size(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_size_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::Percent(value)));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_size_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontSizeType(FontSize::Percent(value)));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_size_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_size_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_size_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_size_percent(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_size_percent(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<f32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_font_size_percent(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_content_utf8(gui: &mut Gui, node_id: f64, content: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextContentType({
                let content = unsafe { String::from_utf8_unchecked(content) };
                TextContent(content, pi_atom::Atom::from(""))
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_content_utf8(gui: &mut Gui, node_id: f64, content: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextContentType({
                let content = unsafe { String::from_utf8_unchecked(content) };
                TextContent(content, pi_atom::Atom::from(""))
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_content_utf8(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_content_utf8(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_content_utf8(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_content_utf8(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_content_utf8(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let content = super::json_parse::as_value::<Vec<u8>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_content_utf8(gui, node, content);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_clip_path_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            ClipPathType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                match BaseShape::parse(&mut parse) {
                    Ok(r) => r,
                    Err(e) => {
                        ();
                        return;
                    }
                }
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_clip_path_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            ClipPathType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                match BaseShape::parse(&mut parse) {
                    Ok(r) => r,
                    Err(e) => {
                        ();
                        return;
                    }
                }
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_clip_path_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetClipPathType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_clip_path_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetClipPathType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_clip_path_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_clip_path_str(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_clip_path_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<str>(json, i).unwrap();
        i += 1;
        let value = &value;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_clip_path_str(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_duration(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDurationType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_duration(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDurationType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_duration(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDurationType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_duration(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDurationType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_duration(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_duration(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_duration(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<usize>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_duration(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_delay(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDelayType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_delay(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDelayType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_delay(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDelayType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_delay(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDelayType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_delay(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_delay(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_delay(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<usize>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_delay(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_iteration_count(gui: &mut Gui, node_id: f64, name: Vec<f32>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationIterationCountType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_iteration_count(gui: &mut Gui, node_id: f64, name: Vec<f32>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationIterationCountType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_iteration_count(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetAnimationIterationCountType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_iteration_count(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetAnimationIterationCountType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_iteration_count(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_iteration_count(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_iteration_count(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<f32>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_iteration_count(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_direction(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDirectionType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_direction(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationDirectionType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDirectionType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDirectionType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_direction(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_direction(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_direction(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<u8>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_direction(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_fill_mode(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationFillModeType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_fill_mode(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationFillModeType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_fill_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationFillModeType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_fill_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationFillModeType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_fill_mode(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_fill_mode(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_fill_mode(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<u8>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_fill_mode(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_play_state(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationPlayStateType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_play_state(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationPlayStateType(unsafe {
                transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_play_state(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationPlayStateType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_play_state(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationPlayStateType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_play_state(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_play_state(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_play_state(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let name = super::json_parse::as_value::<Vec<u8>>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_play_state(gui, node, name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_name_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationNameType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                let value = if let Ok(value) = parse_comma_separated::<
                    _,
                    _,
                    cssparser::ParseError<pi_style::style_parse::ValueParseErrorKind>,
                >(&mut parse, |input| {
                    Ok(pi_atom::Atom::from(input.expect_ident()?.as_ref()))
                }) {
                    value
                } else {
                    Default::default()
                };
                AnimationName {
                    scope_hash: scope_hash as usize,
                    value,
                }
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_name_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationNameType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                let value = if let Ok(value) = parse_comma_separated::<
                    _,
                    _,
                    cssparser::ParseError<pi_style::style_parse::ValueParseErrorKind>,
                >(&mut parse, |input| {
                    Ok(pi_atom::Atom::from(input.expect_ident()?.as_ref()))
                }) {
                    value
                } else {
                    Default::default()
                };
                AnimationName {
                    scope_hash: scope_hash as usize,
                    value,
                }
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_name_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationNameType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_name_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationNameType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_name_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_name_str(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_name_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<str>(json, i).unwrap();
        i += 1;
        let value = &value;
        let scope_hash = super::json_parse::as_value::<u32>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_name_str(gui, node, value, scope_hash);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        set_animation_str_inner(gui, node_id, value, scope_hash);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        set_animation_str_inner(gui, node_id, value, scope_hash);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
        reset_animation_str_inner(gui, node_id);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
        reset_animation_str_inner(gui, node_id);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_str(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        let value = super::json_parse::as_value::<String>(json, 1).unwrap();
        let scope_hash = super::json_parse::as_value::<u32>(json, 2).unwrap();
        set_animation_str(gui, node, &value, scope_hash);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageType(MaskImage::Path(image_hash.0.clone())),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            MaskImageType(MaskImage::Path(image_hash.0.clone())),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_mask_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_mask_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_mask_image(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_mask_image(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_mask_image(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        let atom_hash = match context.atoms.get(&hash) {
            Some(r) => r.get_hash(),
            None => {
                ::core::panicking::panic_fmt(format_args!("can not find atom, hash: {0}", hash))
            }
        };
        set_mask_image(
            gui,
            node,
            &Atom(pi_atom::Atom::get(atom_hash as usize).unwrap()),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_background_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BackgroundImageType(image_hash.0.clone()));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_background_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BackgroundImageType(image_hash.0.clone()));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_background_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_background_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_background_image(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_background_image(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_background_image(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        let atom_hash = match context.atoms.get(&hash) {
            Some(r) => r.get_hash(),
            None => {
                ::core::panicking::panic_fmt(format_args!("can not find atom, hash: {0}", hash))
            }
        };
        set_background_image(
            gui,
            node,
            &Atom(pi_atom::Atom::get(atom_hash as usize).unwrap()),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_border_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BorderImageType(image_hash.0.clone()));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, BorderImageType(image_hash.0.clone()));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_border_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_border_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_border_image(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_border_image(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_border_image(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        let atom_hash = match context.atoms.get(&hash) {
            Some(r) => r.get_hash(),
            None => {
                ::core::panicking::panic_fmt(format_args!("can not find atom, hash: {0}", hash))
            }
        };
        set_border_image(
            gui,
            node,
            &Atom(pi_atom::Atom::get(atom_hash as usize).unwrap()),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_shadow(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextShadowType({
                let mut input = cssparser::ParserInput::new(s);
                let mut parse = cssparser::Parser::new(&mut input);
                let shadows = parse_text_shadow(&mut parse);
                if let Ok(value) = shadows {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_shadow(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextShadowType({
                let mut input = cssparser::ParserInput::new(s);
                let mut parse = cssparser::Parser::new(&mut input);
                let shadows = parse_text_shadow(&mut parse);
                if let Ok(value) = shadows {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextShadowType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextShadowType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_shadow(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_shadow(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_shadow(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let s = super::json_parse::as_value::<str>(json, i).unwrap();
        i += 1;
        let s = &s;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_shadow(gui, node, s);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_font_family(gui: &mut Gui, node_id: f64, name: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontFamilyType(name.0.clone()));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_font_family(gui: &mut Gui, node_id: f64, name: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FontFamilyType(name.0.clone()));
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_font_family(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontFamilyType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_font_family(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontFamilyType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_font_family(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_font_family(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_font_family(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let hash = super::json_parse::as_value::<usize>(json, 1).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        let atom_hash = match context.atoms.get(&hash) {
            Some(r) => r.get_hash(),
            None => {
                ::core::panicking::panic_fmt(format_args!("can not find atom, hash: {0}", hash))
            }
        };
        set_font_family(
            gui,
            node,
            &Atom(pi_atom::Atom::get(atom_hash as usize).unwrap()),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_text_content(gui: &mut Gui, node_id: f64, content: String) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextContentType(TextContent(content, pi_atom::Atom::from(""))),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_text_content(gui: &mut Gui, node_id: f64, content: String) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            TextContentType(TextContent(content, pi_atom::Atom::from(""))),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_text_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_text_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_text_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_text_content(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_text_content(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let content = super::json_parse::as_value::<String>(json, i).unwrap();
        i += 1;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_text_content(gui, node, content);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn set_animation_timing_function_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationTimingFunctionType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                if let Ok(value) = parse_comma_separated(
                    &mut parse,
                    <AnimationTimingFunction as StyleParse>::parse,
                ) {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_timing_function_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(
            node_id,
            AnimationTimingFunctionType({
                let mut input = cssparser::ParserInput::new(value);
                let mut parse = cssparser::Parser::new(&mut input);
                if let Ok(value) = parse_comma_separated(
                    &mut parse,
                    <AnimationTimingFunction as StyleParse>::parse,
                ) {
                    value
                } else {
                    Default::default()
                }
            }),
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[allow(unused_attributes)]
    #[wasm_bindgen]
    pub fn reset_animation_timing_function_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetAnimationTimingFunctionType);
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_timing_function_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ResetAnimationTimingFunctionType);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_reset_animation_timing_function_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        reset_animation_timing_function_str(gui, node);
    }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn play_animation_timing_function_str(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = 0;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let value = super::json_parse::as_value::<str>(json, i).unwrap();
        i += 1;
        let value = &value;
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_animation_timing_function_str(gui, node, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_default_style(gui: &mut Gui, value: &str) {
        gui.commands.set_default_style_by_str(value, 0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_default_style(gui: &mut Gui, value: &str) {
        gui.commands.set_default_style_by_str(value, 0)
    }
    #[allow(unused_variables)]
    pub fn play_set_default_style(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let value = super::json_parse::as_value::<str>(json, i as usize).unwrap();
        let value = &value;
        set_default_style(gui, value);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_class(gui: &mut Gui, css: &str, scope_hash: u32) {
        {
            gui.commands.add_css(css, scope_hash as usize);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_class(gui: &mut Gui, css: &str, scope_hash: u32) {
        {
            gui.commands.add_css(css, scope_hash as usize);
        }
    }
    #[allow(unused_variables)]
    pub fn play_create_class(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let css = super::json_parse::as_value::<str>(json, i as usize).unwrap();
        let css = &css;
        let i = i + 1;
        let scope_hash = super::json_parse::as_value::<u32>(json, i as usize).unwrap();
        create_class(gui, css, scope_hash);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_class(gui: &mut Gui, node: f64, class_name: Vec<u32>) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut s = SmallVec::with_capacity(class_name.len());
            for i in class_name.iter() {
                s.push(*i as usize);
            }
            gui.commands.set_class(node, ClassName(s));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_class(gui: &mut Gui, node: f64, class_name: Vec<u32>) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut s = SmallVec::with_capacity(class_name.len());
            for i in class_name.iter() {
                s.push(*i as usize);
            }
            gui.commands.set_class(node, ClassName(s));
        }
    }
    pub fn play_set_class(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let class_name = super::json_parse::as_value::<Vec<u32>>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        set_class(gui, node, class_name);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_node(gui: &mut Gui) -> f64 {
        {
            let entity = gui.entitys.reserve_entity();
            gui.commands
                .push_cmd(NodeCmd(NodeBundle::default(), entity));
            unsafe { transmute(entity.to_bits()) }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_node(gui: &mut Gui) -> f64 {
        {
            let entity = gui.entitys.reserve_entity();
            gui.commands
                .push_cmd(NodeCmd(NodeBundle::default(), entity));
            unsafe { transmute(entity.to_bits()) }
        }
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_vnode(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_vnode(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_text_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_text_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_image_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_image_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_canvas_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_canvas_node(gui: &mut Gui) -> f64 {
        create_node(gui)
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn destroy_node(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .destroy_node(Entity::from_bits(unsafe { transmute(node) }));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn destroy_node(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .destroy_node(Entity::from_bits(unsafe { transmute(node) }));
        }
    }
    pub fn play_destroy_node(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        destroy_node(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn insert_as_root(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .append(node, unsafe { transmute(EntityKey::null().to_bits()) });
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn insert_as_root(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .append(node, unsafe { transmute(EntityKey::null().to_bits()) });
        }
    }
    pub fn play_insert_as_root(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        insert_as_root(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn append_child(gui: &mut Gui, node: f64, parent: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let parent = Entity::from_bits(unsafe { transmute(parent) });
            gui.commands.append(node, parent);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn append_child(gui: &mut Gui, node: f64, parent: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let parent = Entity::from_bits(unsafe { transmute(parent) });
            gui.commands.append(node, parent);
        }
    }
    pub fn play_append_child(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let parent = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        append_child(gui, node, parent);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn insert_before(gui: &mut Gui, node: f64, borther: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .insert_before(node, Entity::from_bits(unsafe { transmute(borther) }));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn insert_before(gui: &mut Gui, node: f64, borther: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands
                .insert_before(node, Entity::from_bits(unsafe { transmute(borther) }));
        }
    }
    pub fn play_insert_before(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        i += 1;
        let borther = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        insert_before(gui, node, borther);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn remove_node(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands.remove_node(node);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn remove_node(gui: &mut Gui, node: f64) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            gui.commands.remove_node(node);
        }
    }
    pub fn play_remove_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        remove_node(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_view_port(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32, root: f64) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands.push_cmd(NodeCmd(
                crate::components::user::Viewport(Aabb2::new(
                    Point2::new(x as f32, y as f32),
                    Point2::new(width as f32, height as f32),
                )),
                root,
            ));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_view_port(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32, root: f64) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands.push_cmd(NodeCmd(
                crate::components::user::Viewport(Aabb2::new(
                    Point2::new(x as f32, y as f32),
                    Point2::new(width as f32, height as f32),
                )),
                root,
            ));
        }
    }
    #[allow(unused_variables)]
    pub fn play_set_view_port(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let x = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let y = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let width = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let height = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let root = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        set_view_port(gui, x, y, width, height, root);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_clear_color(
        gui: &mut Gui,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        root: f64,
        is_clear_window: bool,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands.push_cmd(NodeCmd(
                crate::components::user::ClearColor(CgColor::new(r, g, b, a), is_clear_window),
                root,
            ));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_clear_color(
        gui: &mut Gui,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        root: f64,
        is_clear_window: bool,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands.push_cmd(NodeCmd(
                crate::components::user::ClearColor(CgColor::new(r, g, b, a), is_clear_window),
                root,
            ));
        }
    }
    #[allow(unused_variables)]
    pub fn play_set_clear_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let r = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let g = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let b = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let a = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let root = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let i = i + 1;
        let is_clear_window = super::json_parse::as_value::<bool>(json, i as usize).unwrap();
        set_clear_color(gui, r, g, b, a, root, is_clear_window);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
        match bincode::deserialize::<Vec<pi_style::style_parse::ClassMap>>(bin) {
            Ok(r) => {
                gui.commands.push_cmd(crate::resource::ExtendCssCmd(r));
            }
            Err(e) => {
                ();
                return;
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
        match bincode::deserialize::<Vec<pi_style::style_parse::ClassMap>>(bin) {
            Ok(r) => {
                gui.commands.push_cmd(crate::resource::ExtendCssCmd(r));
            }
            Err(e) => {
                ();
                return;
            }
        }
    }
    #[allow(unused_variables)]
    pub fn play_create_class_by_bin(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let bin = super::json_parse::as_value::<[u8]>(json, i as usize).unwrap();
        let bin = &bin;
        create_class_by_bin(gui, bin);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_render_dirty(gui: &mut Gui, root: f64) {
        {
            let node: Entity = Entity::from_bits(unsafe { transmute(root) });
            gui.commands
                .push_cmd(NodeCmd(crate::components::user::RenderDirty(true), node));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_render_dirty(gui: &mut Gui, root: f64) {
        {
            let node: Entity = Entity::from_bits(unsafe { transmute(root) });
            gui.commands
                .push_cmd(NodeCmd(crate::components::user::RenderDirty(true), node));
        }
    }
    #[allow(unused_variables)]
    pub fn play_set_render_dirty(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let root = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        set_render_dirty(gui, root);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn fram_call(engine: &mut Engine, _cur_time: u32) {
        {
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::RENDER;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
            engine.update();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn fram_call(engine: &mut Engine, _cur_time: u32) {
        {
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::RENDER;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
            engine.update();
        }
    }
    pub fn play_fram_call(
        engine: &mut Engine,
        _context: &mut PlayContext,
        _json: &Vec<json::JsonValue>,
    ) {
        let mut _i = -1;
        _i += 1;
        let _cur_time = super::json_parse::as_value::<u32>(_json, _i as usize).unwrap();
        fram_call(engine, _cur_time);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn flush(gui: &mut Gui, engine: &mut Engine) {
        {
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::SETTING;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn flush(gui: &mut Gui, engine: &mut Engine) {
        {
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::SETTING;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    pub fn play_flush(
        gui: &mut Gui,
        engine: &mut Engine,
        _context: &mut PlayContext,
        _json: &Vec<json::JsonValue>,
    ) {
        let mut _i = -1;
        flush(gui, engine);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn calc(gui: &mut Gui, engine: &mut Engine) {
        {
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
            engine.update();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn calc(gui: &mut Gui, engine: &mut Engine) {
        {
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
            engine.update();
        }
    }
    pub fn play_calc(
        gui: &mut Gui,
        engine: &mut Engine,
        _context: &mut PlayContext,
        _json: &Vec<json::JsonValue>,
    ) {
        let mut _i = -1;
        calc(gui, engine);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn calc_layout(gui: &mut Gui, engine: &mut Engine) {
        {
            ();
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::LAYOUT;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn calc_layout(gui: &mut Gui, engine: &mut Engine) {
        {
            ();
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::LAYOUT;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    pub fn play_calc_layout(
        gui: &mut Gui,
        engine: &mut Engine,
        _context: &mut PlayContext,
        _json: &Vec<json::JsonValue>,
    ) {
        let mut _i = -1;
        calc_layout(gui, engine);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn calc_geo(gui: &mut Gui, engine: &mut Engine) {
        {
            ();
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn calc_geo(gui: &mut Gui, engine: &mut Engine) {
        {
            ();
            bevy::ecs::system::CommandQueue::default().apply(&mut engine.world);
            let mut com = engine
                .world
                .get_resource_mut::<crate::prelude::UserCommands>()
                .unwrap();
            std::mem::swap(&mut gui.commands, &mut *com);
            *engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
            *engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
            engine.update();
        }
    }
    pub fn play_calc_geo(
        gui: &mut Gui,
        engine: &mut Engine,
        _context: &mut PlayContext,
        _json: &Vec<json::JsonValue>,
    ) {
        let mut _i = -1;
        calc_geo(gui, engine);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn bind_render_target(_gui: &mut Gui) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn bind_render_target(_gui: &mut Gui) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_bind_render_target(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        bind_render_target(gui);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_pixel_ratio(_gui: &mut Gui, _pixel_ratio: f32) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_pixel_ratio(_gui: &mut Gui, _pixel_ratio: f32) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_set_pixel_ratio(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let _pixel_ratio = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        set_pixel_ratio(gui, _pixel_ratio);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn nullify_clear_color(_gui: &mut Gui) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn nullify_clear_color(_gui: &mut Gui) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_nullify_clear_color(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        nullify_clear_color(gui);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_scissor(_gui: &mut Gui, _x: i32, _y: i32, _width: i32, _height: i32) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_scissor(_gui: &mut Gui, _x: i32, _y: i32, _width: i32, _height: i32) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_set_scissor(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let i = -1;
        let i = i + 1;
        let _x = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let _y = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let _width = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        let i = i + 1;
        let _height = super::json_parse::as_value::<i32>(json, i as usize).unwrap();
        set_scissor(gui, _x, _y, _width, _height);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_project_transfrom(
        _gui: &mut Gui,
        _scale_x: f32,
        _scale_y: f32,
        _translate_x: f32,
        _translate_y: f32,
        _rotate: f32,
        _width: f64,
        _height: f64,
    ) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_project_transfrom(
        _gui: &mut Gui,
        _scale_x: f32,
        _scale_y: f32,
        _translate_x: f32,
        _translate_y: f32,
        _rotate: f32,
        _width: f64,
        _height: f64,
    ) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_set_project_transfrom(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let i = -1;
        let i = i + 1;
        let _scale_x = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let _scale_y = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let _translate_x = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let _translate_y = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let _rotate = super::json_parse::as_value::<f32>(json, i as usize).unwrap();
        let i = i + 1;
        let _width = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        let i = i + 1;
        let _height = super::json_parse::as_value::<f64>(json, i as usize).unwrap();
        set_project_transfrom(
            gui,
            _scale_x,
            _scale_y,
            _translate_x,
            _translate_y,
            _rotate,
            _width,
            _height,
        );
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn force_update_text(_gui: &mut Gui, _node: f64) {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn force_update_text(_gui: &mut Gui, _node: f64) {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        {}
    }
    pub fn play_force_update_text(
        gui: &mut Gui,
        context: &mut PlayContext,
        json: &Vec<json::JsonValue>,
    ) {
        let mut i = -1;
        i += 1;
        let node =
            unsafe { Entity::from_bits(transmute(as_value::<f64>(json, i as usize).unwrap())) }
                .index();
        let node = match context.nodes.get(node as usize) {
            Some(r) => r.clone(),
            None => return,
        };
        force_update_text(gui, node);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn set_shader(_gui: &mut Gui, _shader_name: &str, _shader_code: &str) {
        {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn set_shader(_gui: &mut Gui, _shader_name: &str, _shader_code: &str) {
        {}
    }
    #[allow(unused_variables)]
    pub fn play_set_shader(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
        let i = -1;
        let i = i + 1;
        let _shader_name = super::json_parse::as_value::<str>(json, i as usize).unwrap();
        let _shader_name = &_shader_name;
        let i = i + 1;
        let _shader_code = super::json_parse::as_value::<str>(json, i as usize).unwrap();
        let _shader_code = &_shader_code;
        set_shader(gui, _shader_name, _shader_code);
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_font_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_font_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_class_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_class_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn create_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn create_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn destroy_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn destroy_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn clone_engine(_gui: &mut Gui) -> Gui {
        ::core::panicking::panic("not yet implemented")
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn clone_engine(_gui: &mut Gui) -> Gui {
        ::core::panicking::panic("not yet implemented")
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_text_texture_width(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_text_texture_width(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_text_texture_height(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_text_texture_height(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn node_is_exist(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn node_is_exist(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn node_is_visibility(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn node_is_visibility(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    type ReturnNode = Option<f64>;
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn first_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn first_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn last_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn last_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn next_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn next_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn previous_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn previous_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_layer(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_layer(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn offset_top(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn offset_top(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn offset_left(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn offset_left(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn offset_width(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn offset_width(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn offset_height(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn offset_height(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_atom(value: String) -> Atom {
        Atom(pi_atom::Atom::from(value))
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_atom(value: String) -> Atom {
        Atom(pi_atom::Atom::from(value))
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_atom_hash(value: &Atom) -> f64 {
        value.get_hash() as f64
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_atom_hash(value: &Atom) -> f64 {
        value.get_hash() as f64
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_string_by_hash(value: u32) -> Option<String> {
        match pi_atom::Atom::get(value as usize) {
            Some(r) => Some(r.as_ref().to_string()),
            None => None,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_string_by_hash(value: u32) -> Option<String> {
        match pi_atom::Atom::get(value as usize) {
            Some(r) => Some(r.as_ref().to_string()),
            None => None,
        }
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_entity_offset(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.index()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_entity_offset(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.index()
        }
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn get_entity_version(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.generation()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn get_entity_version(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.generation()
        }
    }
    #[cfg(feature = "wasm_bindgen")]
    #[wasm_bindgen]
    pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
        super::query(engine, gui, x, y)
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "pi_js_export")]
    pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
        super::query(engine, gui, x, y)
    }
    pub struct Rect {
        pub left: f32,
        pub top: f32,
        pub width: f32,
        pub height: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Rect {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Rect",
                "left",
                &self.left,
                "top",
                &self.top,
                "width",
                &self.width,
                "height",
                &&self.height,
            )
        }
    }
    pub struct Size {
        pub width: f32,
        pub height: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Size {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Size",
                "width",
                &self.width,
                "height",
                &&self.height,
            )
        }
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
    #[inline]
    fn set_animation_str_inner(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        use pi_style::style_parse::parse_animation;
        let node_id = Entity::from_bits(unsafe { transmute(node_id) });
        let mut input = cssparser::ParserInput::new(value);
        let mut parse = cssparser::Parser::new(&mut input);
        let mut animations = match parse_animation(&mut parse) {
            Ok(r) => r,
            Err(e) => {
                ();
                return;
            }
        };
        animations.name.scope_hash = scope_hash as usize;
        ();
        if animations.name.value.len() > 0 {
            gui.commands
                .set_style(node_id, AnimationNameType(animations.name));
            gui.commands
                .set_style(node_id, AnimationDurationType(animations.duration));
            gui.commands.set_style(
                node_id,
                AnimationTimingFunctionType(animations.timing_function),
            );
            gui.commands.set_style(
                node_id,
                AnimationIterationCountType(animations.iteration_count),
            );
            gui.commands
                .set_style(node_id, AnimationDelayType(animations.delay));
            gui.commands
                .set_style(node_id, AnimationDirectionType(animations.direction));
            gui.commands
                .set_style(node_id, AnimationFillModeType(animations.fill_mode));
            gui.commands
                .set_style(node_id, AnimationPlayStateType(animations.play_state));
        }
    }
    #[inline]
    fn reset_animation_str_inner(gui: &mut Gui, node_id: f64) {
        let node_id = Entity::from_bits(unsafe { transmute(node_id) });
        gui.commands.set_style(node_id, ResetAnimationNameType);
        gui.commands.set_style(node_id, ResetAnimationDurationType);
        gui.commands
            .set_style(node_id, ResetAnimationIterationCountType);
        gui.commands.set_style(node_id, ResetAnimationDelayType);
        gui.commands.set_style(node_id, ResetAnimationDirectionType);
        gui.commands.set_style(node_id, ResetAnimationFillModeType);
        gui.commands.set_style(node_id, ResetAnimationPlayStateType);
    }
    pub mod debug {}
