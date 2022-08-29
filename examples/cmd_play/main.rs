
#[path ="../framework.rs"]
mod framework;

#[macro_use]
pub extern crate lazy_static;

use font_kit::font::new_face_by_path;
use framework::Example;
use pi_map::vecmap::VecMap;
use std::{fs::{DirEntry, read}, collections::hash_map::Entry, path::Path};

use async_trait::async_trait;
use json::{JsonValue, number::Number, object::Object};
use pi_atom::Atom;
use pi_flex_layout::prelude::Size;
use pi_hash::XHashMap;
use pi_idtree::IdTree;
use pi_ui_render::{export::{style::*, Engine}, export::json_parse::as_value};

use pi_style::{style_type::ClassSheet, style_parse::parse_class_map_from_string};

// 
fn main() {
	framework::start(ExampleCommonPlay::default())
}


pub struct ExampleCommonPlay {
	play_context: PlayContext,
	list_index: usize,
	file_index: usize,
	play_version: String,
	json_arr: JsonValue,

	width: usize,
	height: usize,
	scale: f32,
}

impl Default for ExampleCommonPlay {
    fn default() -> Self {
        Self { 
			play_context: PlayContext {
				nodes: VecMap::new(),
				idtree: IdTree::default(),
				atoms: XHashMap::default(),
			},
			list_index: 0, 
			file_index: 0,
			play_version: "test".to_string(), 
			json_arr: JsonValue::Array(Vec::default()),
			// width: 400,
			// height: 750,
			// scale: 1.0
			width: 781,
			height: 1334,
			scale: 0.703
		}
    }
}

#[async_trait]
impl Example for ExampleCommonPlay {
	async fn init(
		&mut self, 
		gui: &mut Engine, 
		size: (usize, usize),
	) {
		let mut dir = std::env::current_dir().unwrap();
		dir.push("examples/cmd_play/source/");
		std::env::set_current_dir(dir).unwrap();

		// 设置默认字体
		new_face_by_path("default".to_string(), "SOURCEHANSANSK-MEDIUM.TTF");

		println!("view_port:{:?}", size);
		// 设置class
		let mut class_sheet = ClassSheet::default();
		let mut cb = |dwcss: &DirEntry| {
			let file = read(dwcss.path());
			if let Ok(r) = file {
				let file = String::from_utf8(r).unwrap();
				parse_class_map_from_string(file.as_str()).unwrap().to_class_sheet(&mut class_sheet);
			}
		};
		visit_dirs(&Path::new("dwcss/"), &mut cb).unwrap();


		let full_screen_class = format!(".c3165071837 {{position : absolute ;left : 0px ;top : 0px ;width : {:?}px ;height : {:?}px ;}}", self.width, self.height);
		parse_class_map_from_string(full_screen_class.as_str()).unwrap().to_class_sheet(&mut class_sheet);
		

		match gui.gui.world_mut().get_resource_mut::<ClassSheet>() {
			Some(r) => r.extend_from_class_sheet(class_sheet),
			None => {gui.gui.world_mut().insert_resource(class_sheet);},
		};
		
		// let gui = &mut gui.gui;
		// let gui = unsafe { &mut *(gui as *mut Gui as usize as *mut pi_ui_render::export::Gui)};
		let context = &mut self.play_context;
		context.atoms.insert(3781626326, Atom::from("_$text"));
		context.atoms.insert(11, Atom::from(""));

		let mut json = Object::new();
		json.insert("ret", JsonValue::Number(1.into()));
		let _root = play_create_node(gui, context, &vec![JsonValue::Object(json.clone())]);
		play_width(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(self.width))]);
		play_height(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(self.height))]);
		play_transform_scale(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(self.scale)), JsonValue::Number(Number::from(self.scale))]);
		play_transform_origin(gui, context, &vec![
			JsonValue::Number(Number::from(1)), 
			JsonValue::Number(Number::from(0)), 
			JsonValue::Number(Number::from(0.0)), 
			JsonValue::Number(Number::from(0)),
			JsonValue::Number(Number::from(0.0))]);
		play_position(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(0)),JsonValue::Number(Number::from(0.))]);
		play_position(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(1)),JsonValue::Number(Number::from(0.))]);
		play_margin(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(0)),JsonValue::Number(Number::from(0.))]);
		play_margin(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(1)),JsonValue::Number(Number::from(0.))]);
		play_position_type(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(1))]);
		play_append_child(gui, context, &vec![JsonValue::Number(Number::from(1)), JsonValue::Number(Number::from(0))]);


		for i in 2..22 {
			json.insert("ret", JsonValue::Number(i.into()));
			let _notch_bar = play_create_node(gui, context, &vec![JsonValue::Object(json.clone())]);
			play_width_percent(gui, context, &vec![JsonValue::Number(Number::from(i)), JsonValue::Number(Number::from(1.0))]);
			play_height_percent(gui, context, &vec![JsonValue::Number(Number::from(i)), JsonValue::Number(Number::from(1.0))]);
			play_position_type(gui, context, &vec![JsonValue::Number(Number::from(i)), JsonValue::Number(Number::from(1))]);
			play_append_child(gui, context, &vec![JsonValue::Number(Number::from(i)), JsonValue::Number(Number::from(1))]);
		}

		json.insert("ret", JsonValue::Number(21.into()));
		let _body = play_create_node(gui, context, &vec![JsonValue::Object(json.clone())]);
		play_width_percent(gui, context, &vec![JsonValue::Number(Number::from(21)), JsonValue::Number(Number::from(1.0))]);
		play_height_percent(gui, context, &vec![JsonValue::Number(Number::from(21)), JsonValue::Number(Number::from(1.0))]);
		play_position_type(gui, context, &vec![JsonValue::Number(Number::from(21)), JsonValue::Number(Number::from(1))]);
		play_append_child(gui, context, &vec![JsonValue::Number(Number::from(21)), JsonValue::Number(Number::from(1))]);
	}
	
	fn render(&mut self, gui: &mut Engine) {
		// let gui1 = unsafe { &mut *(gui as *mut Engine as usize as  *mut Gui)};
		// let gui = unsafe { &mut *(gui as *mut Gui as usize as *mut pi_ui_render::export::Gui)};
		let (mut list_index, mut file_index, json_arr) = (self.list_index, self.file_index, &mut self.json_arr);

		if list_index >= json_arr.len() {
			if list_index == json_arr.len() {
				let path = "cmd/cmd_".to_string() + self.play_version.as_str() + "_" + file_index.to_string().as_str() + ".gui_cmd.json";
				match std::fs::read(path.clone()) {
					Ok(r) => {
						*json_arr = json::parse(String::from_utf8(r).unwrap().as_str()).unwrap();
						list_index = 0;
						file_index += 1;
						self.list_index = list_index;
						self.file_index = file_index;
					},
					Err(_) => println!("play end, {:?}", path)
				};
			}
			
		}

		if list_index < json_arr.len() {
			let cur_play = &json_arr[list_index];
			if let JsonValue::Array(cur_play) = cur_play {
				for play_item in cur_play.iter() {
					if let JsonValue::Object(r) = play_item {
						let ty = r.get("type").unwrap().as_usize().unwrap();
						let param = r.get("param").unwrap();
						let ret = match r.get("ret") {
							Some(r) => match r.as_usize() {
								Some(r) => r,
								None => 0,
							},
							None => 0
						};

						if ret == 0 {
							if let JsonValue::Array(param) = param {
								if let Some(cmd) = CMD_LIST.get(ty) {
									cmd(gui, &mut self.play_context, param);
								}
							}
						} else {
							if let Some(cmd) = CMD_LIST.get(ty) {
								cmd(gui, &mut self.play_context, &vec![play_item.clone()]);
							}
						}
					}
				}
			}
		}
		self.list_index += 1;

		
		gui.gui.run();
	}

	fn get_init_size(&self) -> Option<Size<u32>> {
		Some(Size {
			width: (self.width as f32 * self.scale).floor() as u32,
			height: (self.height as f32 * self.scale).floor() as u32,
		})
	}
}

pub fn visit_dirs<F: FnMut(&DirEntry)>(path: &Path, cb: &mut F) -> std::io::Result<()> {
	if path.is_dir() {
		for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
	}
	Ok(())
}

lazy_static! {
    pub static ref CMD_LIST: Vec<fn (&mut Engine, &mut PlayContext, &Vec<json::JsonValue>) > = vec![
		// 布局
		play_position_type, // 1
		play_display, // 1
	
		play_width, // 1
		play_width_auto, // 1
		play_width_percent, // 1
		play_min_width, // 1
		play_min_width_percent, // 1
		play_max_width, // 1
		play_max_width_percent, // 1
		
		play_height, // 1
		play_height_auto, // 1
		play_height_percent, // 1
		play_min_height, // 1
		play_min_height_percent, // 1
		play_max_height, // 1
		play_max_height_percent, // 1
	
		play_position, // 1
		play_position_percent, // 1
	
		play_margin, // 1
		play_margin_auto, // 1
		play_margin_percent, // 1
	
		play_padding, // 1
		play_padding_percent, // 1
	
		play_border, // 1
		
		play_flex_direction, // 1
		play_align_content, // 1
		play_align_items, // 1
		play_align_self, // 1
		play_justify_content, // 1
		
		play_flex_wrap, // 1
		play_flex_grow, // 1
		play_flex_shrink, // 1
		play_flex_basis, // 1
		play_flex_basis_auto, // 1
		play_todo, //play_align_content, // 不存在，暂时占位
	
		// offset，会导致布局，也录制下来
		play_todo, //"offset_top",
		play_todo, //"offset_left",
		play_todo, //"offset_width",
		play_todo, //"offset_height",
		play_todo, //"offset_ducument",
	
		// transform
		play_clear_transform, //"clear_transform",
		play_clear_transform, //"reset_transform",
	
		play_transform_translate, // 1
		play_transform_translate_x, // 1
		play_transform_translate_y, // 1
	
		play_transform_translate_percent, // 1
		play_transform_translate_x_percent, // 1
		play_transform_translate_y_percent, // 1
	
		play_transform_scale, // 1
		play_transform_scale_x, // 1
		play_transform_scale_y, // 1
	
		play_transform_rotate_x, // 1
		play_transform_rotate_y, // 1
		play_transform_rotate_z, // 1
	
		play_transform_skew_x, // 1
		play_transform_skew_y, // 1
	
		play_transform_origin, // 1
	
		// "create_engine",
		// "create_gui",
	
		// play_view_port,
		// play_project_transfrom,
		// play_gui_size,
	
		play_create_node, // 57
		play_create_vnode, // 1
		play_create_text_node, // 1
		play_create_image_node, // 1
		play_create_canvas_node, // 1
	
		play_remove_node, // 62
		play_destroy_node, // 1
	
		play_todo, //"update_canvas",
		play_todo, //play_canvas_size,
	
		play_todo, //"force_update_text",
		play_todo, //play_render_dirty,
		play_todo, //"render",
		play_todo, //"calc",
		play_todo, //"calc_geo",
		play_todo, //"cal_layout",
		// "create_render_target",
		// "bind_render_target",
		
		play_todo, //"add_canvas_font",
		play_todo, //"add_sdf_font_res",
		play_todo, //"add_font_face",
		
		play_todo, //play_transform_will_change,
	
		play_set_class, //play_class,
		play_todo, //"add_class_start",
		play_todo, //"add_class",
		play_todo, //"add_class_end",
		play_set_class, //play_class_name,
		play_todo, //play_default_style_by_bin,
		
		play_filter_hsi, // 1
		play_enable, // 1
	
		play_append_child, // 84
		play_insert_before, // 85
		play_todo, // "insert_after", // 86
	
		play_todo, // "first_child",
		play_todo, // "last_child",
		play_todo, // "next_sibling",
		play_todo, // "previous_sibling",
		play_todo, // "node_is_exist",
	
		play_background_rgba_color,
		play_todo, //play_background_radial_gradient_color,
		play_background_linear_color,
	
		play_background_image, //play_src,
		play_image_clip, // 1
		play_object_fit, // 1
	
		play_mask_image, // 1
		play_mask_image_clip, // 1
		play_mask_image_linenear, // 1
	
		play_border_color, // 1
		play_border_radius, // 1
		play_border_image, // 1
		play_border_image_slice, // 1
		play_border_image_clip, // 1
		play_border_image_repeat, // 1
	
		play_blend_mode, // 1
	
		play_overflow, // 1
		play_opacity, // 1
		play_zindex, // 1
		play_visibility, // 1
	
		play_todo, //"text",
		play_text_content,
		play_todo, //play_clip_path_geometry_box,
		play_todo, //play_clip_path_basic_shape,
		play_todo, //"text_align",
		play_text_align,
		play_todo, //"letter_spacing",
		play_letter_spacing,
		play_todo, //"line_height",
		play_line_height,
		play_todo, //"text_indent",
		play_text_indent,
		play_todo, //"white_space",
		play_white_space, // 1
		play_text_stroke, // 1
		play_text_linear_gradient_color, // 1
		play_text_shadow, // 1
		play_text_rgba_color, // 1
		play_todo, //"font",
		play_todo, //"font_style",
		play_font_style,
		play_todo, //"font_weight",
		play_font_weight,
		play_todo, //"font_size",
		play_font_size,
		play_todo, //"font_family",
		play_font_family, // 1
	
		play_box_shadow, // 1
		play_todo, //play_box_shadow_color,
		play_todo, //play_box_shadow_h,
		play_todo, //play_box_shadow_v,
		play_todo, //play_box_shadow_blur,
	
		play_reset_text_content, //"reset_text_content",
		play_reset_font_style, //"reset_font_style",
		play_reset_font_weight, //"reset_font_weight",
		play_reset_font_size, //"reset_font_size",
		play_reset_font_family, //"reset_font_family",
		play_reset_letter_spacing, //"reset_letter_spacing",
		play_reset_word_spacing, //"reset_word_spacing",
		play_reset_line_height, //"reset_line_height",
		play_reset_text_indent, //"reset_indent",
		play_reset_white_space, //"reset_white_space",
		play_reset_text_align, //"reset_text_align",
		play_todo, //"reset_vertical_align",
		play_reset_text_rgba_color, //"reset_color",
		play_reset_text_stroke, //"reset_stroke",
		play_reset_text_shadow, //"reset_text_shadow",
		play_reset_background_image, //"reset_image",
		play_reset_image_clip, //"reset_image_clip",
		play_reset_object_fit, //"reset_object_fit",
		play_reset_border_image, //"reset_border_image",
		play_reset_border_image_clip, //"reset_border_image_clip",
		play_reset_border_image_slice, //"reset_border_image_slice",
		play_reset_border_image_repeat, //"reset_border_image_repeat",
		play_reset_border_color, //"reset_border_color",
		play_reset_border_radius, //"reset_border_radius",
		play_reset_background_rgba_color, //"reset_background_color",
		play_reset_box_shadow, //"reset_box_shadow",
		play_todo, //"reset_filter",
		play_reset_opacity, //"reset_opacity",
		play_reset_flex_direction, //"reset_direction",
		play_todo, //"reset_order",
		play_reset_flex_basis, //"reset_flex_basis",
		play_reset_zindex, //"reset_z_index",
		play_todo, //"reset_transform",
		play_todo, //"reset_transform_will_change",
		play_reset_overflow, //"reset_overflow",
		play_reset_mask_image, //"reset_mask_image",
		play_reset_mask_image_clip, //"reset_mask_image_clip",
		play_reset_width, //"reset_width",
		play_reset_height, //"reset_height",
		play_todo, //"reset_margin_top",
		play_todo, //"reset_margin_right",
		play_todo, //"reset_margin_bottom",
		play_todo, //"reset_margin_left",
		play_todo, //"reset_top",
		play_todo, //"reset_right",
		play_todo, //"reset_bottom",
		play_todo, //"reset_left",
		play_todo, //"reset_padding_top",
		play_todo, //"reset_padding_right",
		play_todo, //"reset_padding_bottom",
		play_todo, //"reset_padding_left",
		play_todo, //"reset_border_top",
		play_todo, //"reset_border_right",
		play_todo, //"reset_border_bottom",
		play_todo, //"reset_border_left",
		play_reset_min_width, //"reset_min_width",
		play_reset_min_height, //"reset_min_height",
		play_reset_max_width, //"reset_max_width",
		play_reset_max_height, //"reset_max_height",
		play_reset_justify_content, //"reset_justify_content",
		play_reset_flex_shrink, //"reset_flex_shrink",
		play_reset_flex_grow, //"reset_flex_grow",
		play_reset_position_type, //"reset_position_type",
		play_reset_flex_wrap, //"reset_flex_wrap",
		play_reset_flex_direction, //"reset_flex_direction",
		play_reset_align_content, //"reset_align_content",
		play_reset_align_items, //"reset_align_items",
		play_reset_align_self, //"reset_align_self",
		play_reset_blend_mode, //"reset_blend_mode",
		play_reset_display, //"reset_display",
		play_reset_visibility, //"reset_visibility",
		play_reset_enable, //"reset_enable",
	
	
		set_atom, //"__$set_atom",
	];
}

pub fn play_todo(_gui: &mut Engine, _context: &mut PlayContext, _json: &Vec<json::JsonValue>) {
	
}

// pub fn render(gui: &mut pi_ui_render::export::Gui, context: &mut PlayContext, _json: &Vec<json::JsonValue>) {
// 	{
// 		gui.0.run();
// 		context.dispatcher.0.borrow_mut().run();
// 	}
	
// 	// 睡眠16毫秒
// 	std::thread::sleep( Duration::from_millis(16));
// }

pub fn set_atom(_gui: &mut Engine, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
	let hash = as_value::<usize>(json, 0).unwrap();
	
	match context.atoms.entry(hash) {
		Entry::Occupied(_r) => (),
		Entry::Vacant(r) => {
			let v = as_value::<String>(json, 1).unwrap();
			let value = Atom::from(v);
			r.insert(value);
		}
	}
}

// pub fn play_add_class_start(gui: &mut pi_ui_render::export::Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
// 	let hash = as_value::<usize>(json, 0).unwrap();
	
// 	match context.atoms.entry(hash) {
// 		Entry::Occupied(_r) => (),
// 		Entry::Vacant(r) => {
// 			let v = as_value::<String>(json, 1).unwrap();
// 			let value = Atom::from(v);
// 			r.insert(value);
// 		}
// 	}
// }





















