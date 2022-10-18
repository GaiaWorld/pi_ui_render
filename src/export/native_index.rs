use std::sync::atomic::{AtomicBool, Ordering};
use crate::{
    components::{
        calc::{IsEnable, QuadTree},
        user::{Aabb2, CgColor, Node, Point2, ClearColor, Viewport},
    },
    utils::cmd::{SingleCmd, NodeCmd},
};
use pi_idtree::InsertType;
use pi_style::{
    style_parse::{parse_class_map_from_string, parse_comma_separated, parse_text_shadow, StyleParse, ClassMap, KeyFrameList},
    style_type::*,
	style::*
};
use smallvec::SmallVec;

use pi_ecs::prelude::{DispatcherMgr, Id, LocalVersion, Offset};
pub use super::Engine as Gui;
use super::{json_parse::as_value, style::PlayContext, Atom};
use pi_async::prelude::AsyncRuntime;
// pub use crate::gui::Gui;
use js_proxy_gen_macro::pi_js_export;
use pi_null::Null;
use pi_spatialtree::quad_helper::intersects;
use std::{
    intrinsics::transmute,
    sync::{
        Arc,
    },
};
pub use winit::window::Window;
use crate::{style_out_export, other_out_export};

style_out_export!(@atom 
	mask_image,
	MaskImageType,
	MaskImage::Path(image_hash.0.clone()),
	image_hash: &Atom, );

style_out_export!(@atom 
	background_image,
	BackgroundImageType,
	image_hash.0.clone(),
	image_hash: &Atom, );
style_out_export!(@atom 
	border_image,
	BorderImageType,
	image_hash.0.clone(),
	image_hash: &Atom, );
style_out_export!(@expr text_shadow, TextShadowType, {
	let mut input = cssparser::ParserInput::new(s);
	let mut parse = cssparser::Parser::new(&mut input);

	let shadows = parse_text_shadow(&mut parse);
	if let Ok(value) = shadows {
		value
	} else {
		Default::default()
	}
}, s: &str,);
style_out_export!(@atom font_family, FontFamilyType, name.0.clone(), name: &Atom,);
style_out_export!(@expr text_content, TextContentType,  TextContent(content.to_string(), pi_atom::Atom::from("")), content: &str,);
style_out_export!(@expr animation_name, AnimationNameType, AnimationName{scope_hash: scope_hash as usize,value: name.into_iter().map(|s| {pi_atom::Atom::from(s)}).collect::<SmallVec<[pi_atom::Atom; 1]>>()}, scope_hash: u32, name: Vec<&str>,);
style_out_export!(@expr animation_timing_function_str, AnimationTimingFunctionType, { 
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);

	if let Ok(value) = parse_comma_separated(&mut parse, <AnimationTimingFunction as StyleParse>::parse) {
		value
	} else {
		Default::default()
	}
}, value: &str,);

other_out_export!(set_default_style, gui, gui.gui.set_default_style_by_str(value, 0), value: &str,);

other_out_export!(
    create_class,
    gui,
    {
        let mut class_sheet = ClassSheet::default();
        match parse_class_map_from_string(css, scope_hash as usize) {
            Ok(r) => r.to_class_sheet(&mut class_sheet),
            Err(e) => {
                log::warn!("{:?}", e);
                return;
            }
        };
        gui.gui.push_cmd(SingleCmd(class_sheet));
    },
	scope_hash: u32,
    css: &str,
);

other_out_export!(render, gui, {
	let rt = gui.rt.clone();
	let state = AtomicBool::default();
	let state1: &'static AtomicBool = unsafe{ transmute(&state)};
	let render_dispatcher = gui.render_dispatcher;
	let gui1: &'static mut crate::gui::Gui = unsafe{ transmute(&mut gui.gui)};
	let dispatcher_mgr: &'static mut DispatcherMgr = unsafe{ transmute(&mut gui.dispatcher_mgr)};
	rt.spawn(rt.alloc(), async move {
		dispatcher_mgr.run(render_dispatcher, true).await;
		state1.store(true, Ordering::Relaxed);
	}).unwrap();


    // println!("loop cal_layout start" );
    loop {
        // println!("loop cal_layout" );
        if state.load(Ordering::Relaxed) {
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
},);

other_out_export!(calc, gui, {
    let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc(dispatcher_mgr, true).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

    // println!("loop calc start" );
    loop {
        // println!("loop calc" );
        if state.load(Ordering::Relaxed) {
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
},);

other_out_export!(calc_geo, gui, {
    let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc_geo(dispatcher_mgr, true).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

    // println!("loop calc_geo start" );
    loop {
        // println!("loop calc_geo" );
        if state.load(Ordering::Relaxed) {
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
},);


other_out_export!(cal_layout, gui, {
    let rt = gui.rt.clone();
    let state = AtomicBool::default();
    let state1: &'static AtomicBool = unsafe { transmute(&state) };
    let gui1: &'static mut crate::gui::Gui = unsafe { transmute(&mut gui.gui) };
    let dispatcher_mgr: &'static mut DispatcherMgr = unsafe { transmute(&mut gui.dispatcher_mgr) };
    rt.spawn(rt.alloc(), async {
        gui1.calc_layout(dispatcher_mgr, true).await;
        state1.store(true, Ordering::Relaxed);
    })
    .unwrap();

    // println!("loop cal_layout start" );
    loop {
        // println!("loop cal_layout" );
        if state.load(Ordering::Relaxed) {
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
},);

#[cfg(feature = "pi_js_export")]
pub fn destroy_node(gui: &mut Gui, node_id: f64) { gui.gui.destroy_node(unsafe { transmute(node_id) }); }

pub fn play_destroy_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let id = unsafe { transmute::<_, Id<Node>>(as_value::<f64>(json, 0).unwrap())}.offset();
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

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
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

pub fn play_append_child(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {;
    let node_id = unsafe { transmute::<_, u64>(as_value::<f64>(json, 0).unwrap())} as u32 as usize;
    let parent_id = unsafe { transmute::<_, u64>(as_value::<f64>(json, 1).unwrap())} as u32 as usize;
    let node_id1 = context.nodes.get(node_id).unwrap().clone();
    let parent_id1 = match context.nodes.get(parent_id) {
        Some(r) => r.clone(),
        None => unsafe { transmute(Id::<Node>::null()) },
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

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn insert_before(gui: &mut Gui, node_id: f64, borther: f64) {
    gui.gui.insert_before(unsafe { transmute(node_id) }, unsafe { transmute(borther) });
}

pub fn play_insert_before(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let node_id = unsafe { transmute::<_, Id<Node>>(as_value::<f64>(json, 0).unwrap())}.offset();
    let borther = unsafe { transmute::<_, Id<Node>>(as_value::<f64>(json, 1).unwrap())}.offset();
    let node_id1 = context.nodes.get(node_id).unwrap().clone();
    let borther1 = context.nodes.get(borther).unwrap().clone();
    insert_before(gui, node_id1, borther1);

    if context.idtree.get(node_id).is_none() {
        context.idtree.create(node_id);
    }

    context.idtree.insert_brother(node_id, borther, InsertType::Front);
}


/// 创建容器节点， 容器节点可设置背景颜色
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn create_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }


pub fn play_create_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { transmute::<_, Id<Node>>(ret.as_f64().unwrap())}.offset();
	let r = create_node(gui);
    context.nodes.insert(ret, r);
}

/// 创建虚拟节点

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn create_vnode(gui: &mut Gui) -> f64 {
    use pi_style::style_type::VNodeType;

    let node = gui.gui.create_node();
    gui.gui.set_style(node, VNodeType(true));
    unsafe { transmute(node) }
}

pub fn play_create_vnode(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { transmute::<_, Id<Node>>(ret.as_f64().unwrap())}.offset();
    context.nodes.insert(ret, create_vnode(gui));
}

/// 创建文本节点
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn create_text_node(gui: &mut Gui) -> f64 {
    use pi_style::{style::TextContent, style_type::TextContentType};

    let node = gui.gui.create_node();
    gui.gui
        .set_style(node, TextContentType(TextContent("".to_string(), pi_atom::Atom::from(""))));
    unsafe { transmute(node) }
}

pub fn play_create_text_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
	let ret = unsafe { transmute::<_, Id<Node>>(ret.as_f64().unwrap())}.offset();
    context.nodes.insert(ret, create_text_node(gui));
}

/// 创建图片节点
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn create_image_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }

pub fn play_create_image_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { transmute::<_, Id<Node>>(ret.as_f64().unwrap())}.offset();
    context.nodes.insert(ret, create_image_node(gui));
}

/// 创建图片节点
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn create_canvas_node(gui: &mut Gui) -> f64 { unsafe { transmute(gui.gui.create_node()) } }

pub fn play_create_canvas_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { transmute::<_, Id<Node>>(ret.as_f64().unwrap())}.offset();
    context.nodes.insert(ret, create_canvas_node(gui));
}

/// 移除节点
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "pi_js_export")]
pub fn remove_node(gui: &mut Gui, node_id: f64) { gui.gui.remove_node(unsafe { transmute(node_id) }); }

pub fn play_remove_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let node_id = unsafe { transmute::<_, Id<Node>>(as_value::<f64>(json, 0).unwrap())}.offset();
    let node_id = context.nodes.get(node_id).unwrap().clone();
    remove_node(gui, node_id);
}


#[pi_js_export]
pub fn create_engine(win: &Arc<Window>, r: f64) -> Gui { crate::export::create_engine(win, r) }

#[pi_js_export]
pub fn get_font_sheet(gui: &mut Gui) -> u32 { 0 }

#[pi_js_export]
pub fn get_class_sheet(gui: &mut Gui) -> u32 { 0 }

#[pi_js_export]
pub fn create_render_target(gui: &mut Gui, fbo: f64) -> u32 { 0 }


#[pi_js_export]
pub fn destroy_render_target(gui: &mut Gui, fbo: f64) -> u32 { 0 }

#[pi_js_export]
pub fn bind_render_target(gui: &mut Gui) {}

#[pi_js_export]
pub fn clone_engine(engine: &Gui) -> Gui { todo!() }

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn get_text_texture_width(gui: &mut Gui) -> u32 { 0 }

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn get_text_texture_height(gui: &mut Gui) -> u32 { 0 }

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn set_pixel_ratio(gui: &mut Gui, pixel_ratio: f32) {}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn nullify_clear_color(gui: &mut Gui) {}

#[pi_js_export]
pub fn set_view_port(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32, root: f64) {
	let root = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(root)))};
    gui.gui.push_cmd(NodeCmd(Viewport(Aabb2::new(
        Point2::new(x as f32, y as f32),
        Point2::new(width as f32, height as f32),
    )), root));
}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn set_clear_color(gui: &mut Gui, r: f32, g: f32, b: f32, a: f32, root: f64, is_clear_window: bool) {
	let root = unsafe {Id::<Node>::new(LocalVersion::from_ffi(transmute::<f64, u64>(root)))};
	gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(r, g, b, a), is_clear_window), root)); 
}

/// 设置视口
#[pi_js_export]
pub fn set_scissor(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32) {}

#[pi_js_export]
pub fn set_project_transfrom(gui: &mut Gui, scale_x: f32, scale_y: f32, translate_x: f32, translate_y: f32, rotate: f32, width: f64, height: f64) {}

#[pi_js_export]
pub fn force_update_text(gui: &mut Gui, node_id: f64) {}

//设置shader
#[pi_js_export]
pub fn set_shader(engine: &mut Gui, shader_name: &str, shader_code: &str) {}

/// 添加二进制格式的css表
#[pi_js_export]
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

#[pi_js_export]
pub fn node_is_exist(gui: &mut Gui, node: f64) -> bool { return true }

#[pi_js_export]
pub fn node_is_visibility(gui: &mut Gui, node: f64) -> bool { true }

#[pi_js_export]
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

#[pi_js_export]
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


#[pi_js_export]
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

#[pi_js_export]
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

#[pi_js_export]
pub fn get_layer(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layer_query.get(&gui.gui.world, node) {
        Some(r) => r.layer(),
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的上边界的距离
#[pi_js_export]
pub fn offset_top(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => r.rect.top as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的左边界的距离
#[pi_js_export]
pub fn offset_left(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => r.rect.left as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点的布局宽度
#[pi_js_export]
pub fn offset_width(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => (r.rect.right - r.rect.left) as usize,
        None => 0,
    }
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点布局高度
#[pi_js_export]
pub fn offset_height(gui: &mut Gui, node: f64) -> usize {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.layout_query.get(&gui.gui.world, node) {
        Some(r) => (r.rect.bottom - r.rect.top) as usize,
        None => 0,
    }
}

#[pi_js_export]
pub struct OffsetDocument {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl OffsetDocument {
    #[pi_js_export]
    pub fn left(&self) -> f32 { self.left }
    #[pi_js_export]
    pub fn top(&self) -> f32 { self.top }
    #[pi_js_export]
    pub fn width(&self) -> f32 { self.width }
    #[pi_js_export]
    pub fn height(&self) -> f32 { self.height }
}

/// 等同于html的getBoundingClientRect TODO
/// left top width height
#[pi_js_export]
pub fn offset_document(gui: &mut Gui, node: f64) -> OffsetDocument {
    let node: Id<Node> = unsafe { transmute(node) };
    match gui.gui.quad_query.get(&gui.gui.world, node) {
        Some(quad) => OffsetDocument {
            left: quad.mins.x,
            top: quad.mins.y,
            width: quad.maxs.x - quad.mins.x,
            height: quad.maxs.y - quad.mins.y,
        },
        None => OffsetDocument {
            left: 0.0,
            top: 0.0,
            width: 0.0,
            height: 0.0,
        },
    }
}

#[pi_js_export]
pub fn get_atom(s: &str) -> Atom { Atom(pi_atom::Atom::from(s)) }

#[pi_js_export]
pub fn get_atom_hash(s: &Atom) -> f64 { s.get_hash() as f64 }

#[pi_js_export]
pub fn get_entity_offset(v: f64) -> u32 {
    let r: Id<Node> = unsafe { transmute(v) };
    return r.offset() as u32;
}

// /// content宽高的累加值
// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn content_box(gui: &mut Gui, node: f64) -> JsValue {
//     let world = unsafe { &mut *(world as usize as *mut GuiWorld) };
//     let world = &mut world.gui;
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
#[pi_js_export]
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
#[pi_js_export]
pub fn set_render_dirty(_gui: &mut Gui) {}

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
        let z_depth = arg.gui.gui.depth_query.get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) }).unwrap();
        // log::info!("enable----------id: {}, enable: {}, z_depth: {}, max_z: {}", bind, enable, z_depth,  arg.max_z);
        //如果enable true 表示不接收事件
        if enable.unwrap_or(&IsEnable(false)).0 {
            return;
        }

        // 取最大z的node
        if z_depth.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
            let inpass = arg
                .gui
                .gui
                .in_pass2d_query
                .get(&arg.gui.gui.world, unsafe { Id::<Node>::new(id) })
                .unwrap();
			let mut inpass = inpass.0;
            while !inpass.is_null() {
                let (parent, (quad, oveflow)) = arg.gui.gui.overflow_query.get(&arg.gui.gui.world, inpass).unwrap();
				inpass = parent.0;
                if oveflow.0 {
                    if !intersects(&arg.aabb, quad) {
                        return; // 如果不想交，直接返回，该点不能命中该节点
                    }
                }
            }

            arg.result = unsafe { Id::<Node>::new(id) };
            arg.max_z = z_depth.start;
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
