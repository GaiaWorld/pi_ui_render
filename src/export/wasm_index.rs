use crate::{
    components::{
        calc::{IsEnable, QuadTree, Visibility},
        user::{Aabb2, CgColor, Node, Point2, ClearColor, Viewport},
    },
    utils::cmd::SingleCmd,
};
use pi_animation::{animation_group::AnimationGroupID, animation_listener::EAnimationEvent};
use pi_async::prelude::AsyncRuntime;
use pi_hash::XHashMap;
use pi_idtree::InsertType;
use pi_slotmap::SecondaryMap;
use pi_style::{
    style_parse::{parse_class_map_from_string, ClassMap, StyleParse, KeyFrameList, ValueParseErrorKind, parse_comma_separated, parse_animation, parse_text_shadow},
    style_type::*,
	style::*
};
use smallvec::SmallVec;

pub use super::Engine as Gui;
use super::{style::PlayContext, json_parse::as_value};
// pub use crate::gui::Gui;
use js_proxy_gen_macro::pi_js_export;
pub use super::Atom;
use pi_ecs::{entity::Id, storage::{LocalVersion, Offset}, prelude::DispatcherMgr};
use pi_null::Null;
use pi_spatialtree::quad_helper::intersects;
use std::{intrinsics::transmute, sync::{Arc, atomic::{AtomicBool, Ordering}}};
pub use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;
use winit::dpi::PhysicalSize;
pub use winit::platform::web::WindowBuilderExtWebSys;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use cssparser::ParseError;
use web_sys::HtmlCanvasElement;
use js_sys::{Function, Array};
use pi_hal::runtime::{RUNNER_MULTI, RUNNER_RENDER};
use pi_async::prelude::SingleTaskRunner;

/// width、height为physical_size
#[wasm_bindgen]
pub fn create_engine(win: HtmlCanvasElement, r: f64, width: u32, height: u32) -> Gui {
	let event_loop = EventLoop::new();
	let r = crate::export::create_engine_wasm(&Arc::new(WindowBuilder::new().with_canvas(Some(win)).with_inner_size(PhysicalSize {width, height}).build(&event_loop).unwrap()), r);
	r
}

#[wasm_bindgen]
pub fn create_gui(context: JsValue, gui: &mut Gui, width: f32, height: f32, load_image_fun: Option<Function>, class_sheet: u32, font_sheet: u32, cur_time: u32, animation_event_fun: Function) {
	/// 设置动画的监听器
	let a_callback = Box::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (Id<Node>, pi_atom::Atom)>| {
		let mut arr = Array::new();
		for (group_id, ty, count) in list.iter() {
			match map.get(*group_id) {
				Some(r) => {
					arr.push(&JsValue::from_f64(unsafe { transmute(r.0) }));
					arr.push(&JsValue::from_f64(r.1.get_hash() as f64));
				},
				None => continue,
			};
			arr.push(&JsValue::from_f64(unsafe {transmute::<_, u8>(*ty)}  as f64));
			arr.push(&JsValue::from_f64(*count as f64));
		}
		animation_event_fun.call1(&context, &JsValue::from(arr))
                        .expect("call animation event fail!!!");
    });
	gui.gui.set_event_listener(a_callback);
}

#[wasm_bindgen]
pub fn set_animation_name_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32,) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);
	let value = if let Ok(value) = parse_comma_separated::<_, _, ParseError<ValueParseErrorKind>>(&mut parse, |input| Ok(pi_atom::Atom::from(input.expect_ident()?.as_ref()))) {
		value
	} else {
		Default::default()
	};

	gui.gui.set_style(node_id, AnimationNameType(AnimationName {scope_hash: scope_hash as usize, value}));
}

#[wasm_bindgen]
pub fn reset_animation_name_str(gui: &mut Gui, node_id: f64) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetAnimationNameType);
}

#[wasm_bindgen]
pub fn set_animation_timing_function_str(gui: &mut Gui, node_id: f64, value: &str) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);

	let value = if let Ok(value) = parse_comma_separated(&mut parse, <AnimationTimingFunction as StyleParse>::parse) {
		value
	} else {
		Default::default()
	};
	gui.gui.set_style(node_id, AnimationTimingFunctionType(value));
}

#[wasm_bindgen]
pub fn reset_animation_timing_function_str(gui: &mut Gui, node_id: f64) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetAnimationTimingFunctionType);
}

#[wasm_bindgen]
pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
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
		gui.gui.set_style(node_id, AnimationNameType(animations.name));
		gui.gui.set_style(node_id,AnimationDurationType(animations.duration));
		gui.gui.set_style(node_id,AnimationTimingFunctionType(
			animations.timing_function,
		));
		gui.gui.set_style(node_id,AnimationIterationCountType(
			animations.iteration_count,
		));
		gui.gui.set_style(node_id,AnimationDelayType(animations.delay));
		gui.gui.set_style(node_id,AnimationDirectionType(animations.direction));
		gui.gui.set_style(node_id,AnimationFillModeType(animations.fill_mode));
		gui.gui.set_style(node_id,AnimationPlayStateType(animations.play_state));
	}
}

#[wasm_bindgen]
pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetAnimationNameType);
	gui.gui.set_style(node_id, ResetAnimationDurationType);
	gui.gui.set_style(node_id, ResetAnimationIterationCountType);
	gui.gui.set_style(node_id, ResetAnimationDelayType);
	gui.gui.set_style(node_id, ResetAnimationDirectionType);
	gui.gui.set_style(node_id, ResetAnimationFillModeType);
	gui.gui.set_style(node_id, ResetAnimationPlayStateType);
}

#[wasm_bindgen]
pub fn set_text_shadow(gui: &mut Gui, node_id: f64, value: &str) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);

	let shadows = parse_text_shadow(&mut parse);
	let value = if let Ok(value) = shadows {
		value
	} else {
		Default::default()
	};
	gui.gui.set_style(node_id, TextShadowType(value));
}

#[wasm_bindgen]
pub fn reset_text_shadow(gui: &mut Gui, node_id: f64) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetTextShadowType);
}

#[wasm_bindgen]
pub fn set_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, MaskImageType(MaskImage::Path(image_hash.0.clone())));
}

#[wasm_bindgen]
pub fn reset_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetMaskImageType);
}

#[wasm_bindgen]
pub fn set_border_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, BorderImageType(image_hash.0.clone()));
}

#[wasm_bindgen]
pub fn reset_border_image(gui: &mut Gui, node_id: f64) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetBorderImageType);
}

#[wasm_bindgen]
pub fn set_font_family(gui: &mut Gui, node_id: f64, name: &Atom) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, FontFamilyType(name.0.clone()));
}

#[wasm_bindgen]
pub fn reset_font_family(gui: &mut Gui, node_id: f64) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetFontFamilyType);
}

// #[wasm_bindgen]
// pub fn set_animation_name(gui: &mut Gui, node_id: f64, name: Vec<&str>) { 
// 	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
// 	gui.gui.set_style(node_id, AnimationNameType(name.into_iter().map(|s| {pi_atom::Atom::from(s)}).collect::<SmallVec<[pi_atom::Atom; 1]>>()));
// }

#[wasm_bindgen]
pub fn set_background_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, BackgroundImageType(image_hash.0.clone()));
}

#[wasm_bindgen]
pub fn reset_background_image(gui: &mut Gui, node_id: f64) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetBackgroundImageType);
}

#[wasm_bindgen]
pub fn set_text_content(gui: &mut Gui, node_id: f64, content: &str) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, TextContentType(TextContent(content.to_string(), pi_atom::Atom::from(""))));
}

#[wasm_bindgen]
pub fn reset_text_content(gui: &mut Gui, node_id: f64) { 
	let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
	gui.gui.set_style(node_id, ResetTextContentType);
}

#[wasm_bindgen]
pub fn destroy_node(gui: &mut Gui, node_id: f64) { gui.gui.destroy_node(unsafe { transmute(node_id) }); }

#[wasm_bindgen]
pub fn append_child(gui: &mut Gui, node_id: f64, parent_id: f64) {
    let parent_id = if parent_id == 0.0 {
        unsafe { transmute(Id::<Node>::null()) }
    } else {
        unsafe { transmute(parent_id) }
    };
	let node_id = unsafe { transmute(node_id) };
    gui.gui.append(node_id, parent_id);
	// if parent_id.is_null() {
	// 	init_root(gui, node_id);
	// }
}

#[wasm_bindgen]
pub fn insert_before(gui: &mut Gui, node_id: f64, borther: f64) {
    gui.gui.insert_before(unsafe { transmute(node_id) }, unsafe { transmute(borther) });
}


/// 创建容器节点， 容器节点可设置背景颜色
#[wasm_bindgen]
pub fn create_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }

/// 创建虚拟节点

#[wasm_bindgen]
pub fn create_vnode(gui: &mut Gui) -> f64 {
    use pi_style::style_type::VNodeType;

    let node = gui.gui.create_node();
    gui.gui.set_style(node, VNodeType(true));
    unsafe { transmute(node) }
}

/// 创建文本节点
#[wasm_bindgen]
pub fn create_text_node(gui: &mut Gui) -> f64 {
    use pi_style::{style_type::TextContentType, style::TextContent};

    let node = gui.gui.create_node();
    gui.gui.set_style(node, TextContentType(TextContent("".to_string(), pi_atom::Atom::from(""))));
    unsafe { transmute(node) }
}

/// 创建图片节点
#[wasm_bindgen]
pub fn create_image_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }

/// 创建图片节点
#[wasm_bindgen]
pub fn create_canvas_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }

/// 移除节点
#[wasm_bindgen]
pub fn remove_node(gui: &mut Gui, node_id: f64) { gui.gui.remove_node(unsafe { transmute(node_id) }); }

#[wasm_bindgen]
pub fn get_font_sheet(gui: &mut Gui) -> u32 { 0 }

#[wasm_bindgen]
pub fn get_class_sheet(gui: &mut Gui) -> u32 { 0 }

#[wasm_bindgen]
pub fn create_render_target(gui: &mut Gui, fbo: f64) -> u32 { 0 }


#[wasm_bindgen]
pub fn destroy_render_target(gui: &mut Gui, fbo: f64) -> u32 { 0 }

#[wasm_bindgen]
pub fn bind_render_target(gui: &mut Gui) {}

#[wasm_bindgen]
pub fn clone_engine(engine: &Gui) -> Gui { todo!() }

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[wasm_bindgen]
pub fn get_text_texture_width(gui: &mut Gui) -> u32 { 0 }

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[wasm_bindgen]
pub fn get_text_texture_height(gui: &mut Gui) -> u32 { 0 }

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[wasm_bindgen]
pub fn set_pixel_ratio(gui: &mut Gui, pixel_ratio: f32) {}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[wasm_bindgen]
pub fn nullify_clear_color(gui: &mut Gui) {}

#[wasm_bindgen]
pub fn set_view_port(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32, root: f64) {
    let root = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(root)))};
    gui.gui.push_cmd(RootCmd(Viewport(Aabb2::new(
        Point2::new(x as f32, y as f32),
        Point2::new(width as f32, height as f32),
    )), root));
}

#[wasm_bindgen]
pub fn set_clear_color(gui: &mut Gui, r: f32, g: f32, b: f32, a: f32, root: f64, is_clear_window: bool) {
	let root = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(root)))};
	gui.gui.push_cmd(RootCmd(ClearColor(CgColor::new(r, g, b, a), is_clear_window), root)); 
}

/// 设置视口
#[wasm_bindgen]
pub fn set_scissor(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32) {}


#[wasm_bindgen]
pub fn set_project_transfrom(gui: &mut Gui, scale_x: f32, scale_y: f32, translate_x: f32, translate_y: f32, rotate: f32, width: f64, height: f64) {}

#[wasm_bindgen]
pub fn force_update_text(gui: &mut Gui, node_id: f64) {}


#[inline]
fn run_all(rt: &SingleTaskRunner<()>) {
	while let Ok(r) = rt.run() {
		if r == 0 {
			break;
		}
	}
}

#[wasm_bindgen]
pub fn render(gui: &mut Gui, cur_time: u32) {
	run_all(&RUNNER_MULTI.lock());
	run_all(&RUNNER_RENDER.lock());
	// RUNNER_MULTI.lock().run();
	// RUNNER_RENDER.lock().run();

	let rt = gui.rt.clone();
	let state = AtomicBool::default();
	let state1: &'static AtomicBool = unsafe{ transmute(&state)};
	let render_dispatcher = gui.render_dispatcher;
	let gui1: &'static mut crate::gui::Gui = unsafe{ transmute(&mut gui.gui)};
	let dispatcher_mgr: &'static mut DispatcherMgr = unsafe{ transmute(&mut gui.dispatcher_mgr)};
	rt.spawn(rt.alloc(), async move {
		gui1.run();
		dispatcher_mgr.run(render_dispatcher, false).await;
		state1.store(true, Ordering::Relaxed);
	}).unwrap();

	run_all(&gui.runner);

	run_all(&RUNNER_MULTI.lock());
	run_all(&RUNNER_RENDER.lock());
}

#[wasm_bindgen]
pub fn calc(gui: &mut Gui) {
	let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc(dispatcher_mgr, false).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

	run_all(&gui.runner);
}

#[wasm_bindgen]
pub fn calc_layout(gui: &mut Gui) {
	let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc_layout(dispatcher_mgr, false).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

	run_all(&gui.runner);
}

#[wasm_bindgen]
pub fn calc_geo(gui: &mut Gui) {
	let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc_geo(dispatcher_mgr, false).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

	gui.runner.run();
}

//设置shader
#[wasm_bindgen]
pub fn set_shader(engine: &mut Gui, shader_name: String, shader_code: String) {}

// 创建class
#[wasm_bindgen]
pub fn create_class(gui: &mut Gui, css: &str, scope_hash: u32) {
    let mut class_sheet = ClassSheet::default();
    match parse_class_map_from_string(css, scope_hash as usize) {
        Ok(r) => r.to_class_sheet(&mut class_sheet),
        Err(e) => {
            log::warn!("{:?}", e);
            return;
        }
    };
    gui.gui.push_cmd(SingleCmd(class_sheet));
}

/// 添加二进制格式的css表
#[wasm_bindgen]
pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
    match bincode::deserialize::<Vec<ClassMap>>(bin) {
        Ok(mut r) => {
			for mut item in r.into_iter() {
				if item.key_frames.frames.len() > 0 {
					log::info!("create keyframs, count: {}", item.key_frames.frames.len());
					gui.gui.push_cmd(SingleCmd(std::mem::replace(&mut item.key_frames, KeyFrameList::default())));
				}
				let mut class_sheet = ClassSheet::default();
				item.to_class_sheet(&mut class_sheet);
				gui.gui.push_cmd(SingleCmd(class_sheet));
			}
        }
        Err(e) => {
            log::warn!("deserialize_class_map error: {:?}, {:?}", e, bin);
            return;
        }
    };
}

#[wasm_bindgen]
pub fn node_is_exist(gui: &mut Gui, node: f64) -> bool { return true }

#[wasm_bindgen]
pub fn node_is_visibility(gui: &mut Gui, node: f64) -> bool { true }

#[wasm_bindgen]
pub fn first_child(gui: &mut Gui, parent: f64) -> Option<f64> {
    let node: Id<Node> = unsafe { transmute(parent) };
    match gui.gui.down_query.get(&gui.gui.world, node) {
        Some(r) => {
            if r.head.is_null() {
                None
            } else {
                Some(unsafe { transmute(r.head) })
            }
        }
        None => None,
    }
}

#[wasm_bindgen]
pub fn last_child(gui: &mut Gui, parent: f64) -> Option<f64> {
    let node: Id<Node> = unsafe { transmute(parent) };
    match gui.gui.down_query.get(&gui.gui.world, node) {
        Some(r) => {
            if r.tail.is_null() {
                None
            } else {
                Some(unsafe { transmute(r.tail) })
            }
        }
        None => None,
    }
}


#[wasm_bindgen]
pub fn next_sibling(gui: &mut Gui, node: f64) -> Option<f64> {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.up_query.get(&gui.gui.world, node) {
        Some(r) => {
            if r.next().is_null() {
                None
            } else {
                Some(unsafe { transmute(r.next()) })
            }
        }
        None => None,
    }
}

#[wasm_bindgen]
pub fn previous_sibling(gui: &mut Gui, node: f64) -> Option<f64> {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.up_query.get(&gui.gui.world, node) {
        Some(r) => {
            if r.prev().is_null() {
                None
            } else {
                Some(unsafe { transmute(r.prev()) })
            }
        }
        None => None,
    }
}

#[wasm_bindgen]
pub fn get_layer(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layer_query.get(&gui.gui.world, node) {
        Some(r) => r.layer,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的上边界的距离
#[wasm_bindgen]
pub fn offset_top(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => r.rect.top as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的左边界的距离
#[wasm_bindgen]
pub fn offset_left(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => r.rect.left as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点的布局宽度
#[wasm_bindgen]
pub fn offset_width(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => (r.rect.right - r.rect.left) as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点布局高度
#[wasm_bindgen]
pub fn offset_height(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => (r.rect.bottom - r.rect.top) as usize,
        None => 0,
    }
}

#[wasm_bindgen]
pub struct OffsetDocument {
	pub left: f32,
	pub top: f32, 
	pub width: f32, 
	pub height: f32
}

// impl OffsetDocument {
// 	#[wasm_bindgen]
// 	pub fn left(&self) -> f32 {
// 		self.left
// 	}
// 	#[wasm_bindgen]
// 	pub fn top(&self) -> f32 {
// 		self.top
// 	}
// 	#[wasm_bindgen]
// 	pub fn width(&self) -> f32 {
// 		self.width
// 	}
// 	#[wasm_bindgen]
// 	pub fn height(&self) -> f32 {
// 		self.height
// 	}
// }

/// 等同于html的getBoundingClientRect TODO
/// left top width height
#[wasm_bindgen]
pub fn offset_document(gui: &mut Gui, node: f64) -> OffsetDocument {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.quad_query.get(&gui.gui.world, node) {
        Some(quad) => OffsetDocument{ left: quad.mins.x, top: quad.mins.y, width: quad.maxs.x - quad.mins.x, height: quad.maxs.y - quad.mins.y },
        None => OffsetDocument{ left: 0.0, top: 0.0, width: 0.0, height: 0.0 },
    }
}

#[wasm_bindgen]
pub fn set_default_style(gui: &mut Gui, class: &str) {
	gui.gui.set_default_style_by_str(class, 0);
}

#[wasm_bindgen]
pub fn get_atom(s: &str) -> Atom { Atom(pi_atom::Atom::from(s)) }

#[wasm_bindgen]
pub fn get_atom_hash(s: &Atom) -> f64 { s.get_hash() as f64 }

#[wasm_bindgen]
pub fn get_string_by_hash(s: u32) -> Option<String> {
	match pi_atom::Atom::get(s as usize) {
		Some(r) => Some(r.as_ref().to_string()),
		None => None,
	}
}

#[wasm_bindgen]
pub fn get_entity_offset(v: f64) -> u32 {
	unsafe {transmute::<_, Id<Node>>(v)}.offset() as u32
}

#[wasm_bindgen]
pub fn get_entity_version(v: f64) -> u32 {
	(unsafe {transmute::<_, u64>(v)} >> 32 | 1) as u32
}

/**
 * 获取canvas资源
 */
#[wasm_bindgen]
pub fn get_canvas_source(
    gui: &mut Gui,
    soruce: u32, // 是否缓存
) -> i32 {
    -1
}

/**
 * canvas宽高改变时调用(分配纹理成功，返回对应索引，否则返回-1)
 * @return __jsObj 纹理
*/
#[wasm_bindgen]
pub fn set_canvas_size(
    gui: &mut Gui,
    node: f64,
    width: u32,
    height: u32,
    soruce: u32, // 是否缓存
    need_depth: bool, // 是否需要深度缓冲区
                 // avail_width: u32,
                 // avail_height: u32,
) -> i32 {
    1
}

#[wasm_bindgen]
pub fn get_canvas_target(gui: &mut Gui, index: usize) -> Option<usize> {
    Some(1)
}

#[wasm_bindgen]
pub fn get_canvas_rect(gui: &mut Gui, index: usize) -> JsValue {
    JsValue::from_serde(&CanvasRect(0, 0, 0, 0)).unwrap()
}

/**
 * canvas内容发生改变时，应该调用此方法更新gui渲染
*/
#[wasm_bindgen]
pub fn update_canvas(gui: &mut Gui, _node: u32) {
}

// /// content宽高的累加值
// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn content_box(gui: &mut Gui, node: f64) -> JsValue {
//     let node_id = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(node_id)))};
//     let world = &mut gui.gui;
//     let layout = world.layout.lend();
//     let idtree = world.idtree.borrow();
//     let (mut left, mut right, mut top, mut bottom) = (FMAX, 0.0, FMAX, 0.0);
//     for (id, _) in idtree.iter(idtree[node as usize].children().head) {
//         let l = &layout[id];
//         let r = l.rect.end;
//         let b = l.rect.bottom;
//         if l.rect.start < left {
//             left = l.rect.start;
//         }
//         if r > right {
//             right = r;
//         }
//         if b > bottom {
//             bottom = b;
//         }
//         if l.rect.top < top {
//             top = l.rect.top;
//         }
// 	}
// 	JsValue::from_serde(&Size{
// 		width: right - left,
// 		height: bottom - top,
// 	}).unwrap()
// }

/// 用点命中一个节点
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn query(gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
    let quad = gui.gui.quad_component_comtainer.clone();
    let quad = quad.borrow();
    let quad: &QuadTree = quad.get_storage();

    let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
    let mut args = AbQueryArgs {
        gui,
        aabb,
        result: Id::<Node>::null(),
        max_z: usize::MIN,
    };
    quad.query(&aabb, intersects, &mut args, ab_query_func);
    if args.result.is_null() {
        None
    } else {
        Some(unsafe { transmute(args.result) })
    }
}

/// 调试使用， 设置渲染脏， 使渲染系统在下一帧进行渲染
#[wasm_bindgen]
pub fn set_render_dirty(gui: &mut Gui) {}

/// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
fn ab_query_func(arg: &mut AbQueryArgs, id: LocalVersion, aabb: &Aabb2, bind: &()) {
    match arg.gui.gui.layer_query.get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) }) {
        Some(layer) => {
            if layer.layer() == 0 {
                return;
            }
        }
        None => return,
    };
    if intersects(&arg.aabb, aabb) {
        let enable = arg.gui.gui.enable_query.get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) });
		let visibility = arg.gui.gui.visibility_query.get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) });
        let z_depth = arg.gui.gui.depth_query.get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) }).unwrap();
        // log::info!("enable----------id: {}, enable: {}, z_depth: {}, max_z: {}", bind, enable, z_depth,  arg.max_z);
        //如果enable false 表示不接收事件, visibility为false， 也无法接收时间
        if !enable.unwrap_or(&IsEnable(false)).0 || !visibility.unwrap_or(&Visibility(false)).0 {
            return;
        }

        // 取最大z的node
        if z_depth.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
            if let Some( inpass) = arg
                .gui
                .gui
                .in_pass2d_query
                .get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) }) {

				let mut inpass = inpass.0;
				while !inpass.is_null() {
					if let Some((parent, (quad, oveflow))) = arg.gui.gui.overflow_query.get(&arg.gui.gui.world, inpass) {
						inpass = parent.0;
						if oveflow.0 {
							if !intersects(&arg.aabb, quad) {
								return; // 如果不想交，直接返回，该点不能命中该节点
							}
						}
					} else {
						break;
					}
				}

				arg.result = unsafe { Id::<Node>::new(id) };
				arg.max_z = z_depth.start;
			}
			
        }
    }
}

pub struct AbQueryArgs<'a> {
    gui: &'a mut Gui,
    aabb: Aabb2,
    result: Id<Node>,
    max_z: usize,
}

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

#[derive(Serialize)]
pub struct CanvasRect(u32, u32, u32, u32);
