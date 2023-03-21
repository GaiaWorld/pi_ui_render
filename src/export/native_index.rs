use crate::{prelude::{UserCommands, UiPlugin}};
use bevy::{window::WindowId};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_idtree::InsertType;
use crate::resource::animation_sheet::KeyFramesSheet;

// use pi_ecs::prelude::{DispatcherMgr, Id, LocalVersion, Offset};

pub use super::Gui;
pub use super::{Atom, Engine};
use super::{json_parse::as_value, style::PlayContext, remove_node, append_child, destroy_node, insert_before, create_node, create_text_node, create_image_node, create_canvas_node, create_vnode};
use crate::components::calc::EntityKey;
use bevy::app::prelude::App;
use bevy::ecs::{prelude::Entity, system::SystemState};
// pub use crate::gui::Gui;
use pi_null::Null;
use std::{intrinsics::transmute, sync::Arc};
pub use winit::window::Window;

#[cfg(feature="pi_js_export")]
pub fn create_engine(window: &Arc<Window>, _r: f64, width: u32, height: u32) -> Engine {
    let mut app = App::default();

	let mut window_plugin = bevy::window::WindowPlugin::default();
    window_plugin.add_primary_window = false;
	// window_plugin.window.width = width as f32;
    // window_plugin.window.height = height as f32;
	// window_plugin.add_primary_window = false;
	
    app
		// .add_plugin(bevy::log::LogPlugin {
		// 	filter: "wgpu=debug".to_string(),
		// 	level: bevy::log::Level::DEBUG,
		// })
		// .add_plugin(bevy::input::InputPlugin::default())
		.add_plugin(window_plugin)
		.add_plugin(pi_bevy_winit_window::WinitPlugin::new(window.clone(), WindowId::primary()).with_size(width, height))
		// .add_plugin(WorldInspectorPlugin::new())
		.add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugin(PiPostProcessPlugin);
    Engine(app)
}

#[allow(unused_variables)]
#[cfg(feature="pi_js_export")]
pub fn create_gui(
    context: u32,
    engine: &mut Engine,
    width: f32,
    height: f32,
    load_image_fun: u32,
    class_sheet: u32,
    font_sheet: u32,
    cur_time: u32,
    animation_event_fun: u32,
) -> Gui {
	println!("width====================!!!!!, {:?}", width);
    let gui = Gui {
        down_query: engine.world.query(),
        up_query: engine.world.query(),
        layer_query: engine.world.query(),
        enable_query: engine.world.query(),
        depth_query: engine.world.query(),
        layout_query: engine.world.query(),
        quad_query: engine.world.query(),
        matrix_query: engine.world.query(),
        overflow_query: engine.world.query(),
        in_pass2d_query: engine.world.query(),
        graph_id: engine.world.query(),
        query_state: SystemState::new(&mut engine.world),
        // 这里使用非安全的方法，将entities转为静态声明周期，外部需要保证entities使用期间， app的指针不能更改（如将App放入堆中就不可行）
        entitys: unsafe { transmute(engine.world.entities()) },
        commands: UserCommands::default(),
    };

    engine.add_plugin(UiPlugin);

	// // 设置动画的监听器
    // let a_callback = Share::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (ObjKey, pi_atom::Atom)>| {
    // 	let mut arr: Vec<f64> = Vec::new();
    // 	for (group_id, ty, count) in list.iter() {
    // 		match map.get(*group_id) {
    // 			Some(r) => {
	// 				arr.push(unsafe { transmute::<_, f64>(r.0.to_bits()) }); // entity
	// 				arr.push(r.1.get_hash() as f64); // name hash
    // 			},
    // 			None => continue,
    // 		};
    // 		arr.push(unsafe {transmute::<_, u8>(*ty)}  as f64); // event type
    // 		arr.push(*count as f64); // cur iter count
    // 	}
    // 	animation_event_fun(context, arr, None);
    // });
    // gui.commands.push_cmd(AnimationListenCmd(a_callback));

    gui
}

// 取出动画事件
#[cfg(feature="pi_js_export")]
pub fn get_animation_events(
    engine: &Engine,
) -> Option<Vec<u8>> {
	let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();

	let events = key_frames.get_animation_events();
	let map = key_frames.get_group_bind();
	let mut arr: Vec<u8> = Vec::new();
	for (group_id, ty, count) in events.iter() {
		match map.get(*group_id) {
			Some(r) => {
				arr.extend_from_slice(r.0.index().to_be_bytes().as_slice()); // entity
				arr.extend_from_slice(r.0.generation().to_be_bytes().as_slice());
				arr.extend_from_slice((r.1.get_hash() as u32).to_be_bytes().as_slice()); // name hash
			},
			None => continue,
		};
		arr.extend_from_slice((unsafe {transmute::<_, u8>(*ty)}  as u32).to_be_bytes().as_slice()); // event type
		arr.extend_from_slice((*count).to_be_bytes().as_slice()); // cur iter count
	}
	if arr.len() > 0 {
		Some(arr)
	} else {
		None
	}
	
	// arr
}

#[cfg(feature="pi_js_export")]
pub struct OffsetDocument {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl OffsetDocument {
    #[cfg(feature="pi_js_export")]
    pub fn left(&self) -> f32 { self.left }
    #[cfg(feature="pi_js_export")]
    pub fn top(&self) -> f32 { self.top }
    #[cfg(feature="pi_js_export")]
    pub fn width(&self) -> f32 { self.width }
    #[cfg(feature="pi_js_export")]
    pub fn height(&self) -> f32 { self.height }
}

/// 等同于html的getBoundingClientRect TODO
/// left top width height
#[cfg(feature="pi_js_export")]
pub fn offset_document(_gui: &mut Gui, _node: f64) -> OffsetDocument {
    // let node: Entity = unsafe { transmute(node) };
    // match gui.commands.quad_query.get(&gui.commands.world, node) {
    //     Some(quad) => OffsetDocument {
    //         left: quad.mins.x,
    //         top: quad.mins.y,
    //         width: quad.maxs.x - quad.mins.x,
    //         height: quad.maxs.y - quad.mins.y,
    //     },
    //     None => OffsetDocument {
    //         left: 0.0,
    //         top: 0.0,
    //         width: 0.0,
    //         height: 0.0,
    //     },
    // }
    OffsetDocument {
        left: 0.0,
        top: 0.0,
        width: 0.0,
        height: 0.0,
    }
}

pub fn play_destroy_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let id = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index() as usize;
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

pub fn play_append_child(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {

    // log::warn!("play_append_child================{:?}, {:?}, version0: {:?}, version1: {:?}, index0: {}, index1:{:?}, v1:{}, v2:{}, entity:{:?}", r0, r1, r0 >> 32 as u32, r1 >> 32 as u32, r0 as u32, r1 as u32, as_value::<f64>(json, 0).unwrap(), as_value::<f64>(json, 1).unwrap(), unsafe {Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap()))} );

    let node_id = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index() as usize;
    let parent_id = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 1).unwrap())) }.index() as usize;
    let node_id1 = context.nodes.get(node_id).unwrap().clone();
    let parent_id1 = match context.nodes.get(parent_id) {
        Some(r) => r.clone(),
        None => unsafe { transmute(EntityKey::null().to_bits()) },
    };
    append_child(gui, node_id1, parent_id1);

    if context.idtree.get(node_id).is_none() {
        context.idtree.create(node_id);
    }

    if EntityKey(Entity::from_bits(unsafe { transmute(parent_id1) })).is_null() {
        context.idtree.insert_child(node_id, usize::null(), 0);
    } else {
        if context.idtree.get(parent_id).is_none() {
            context.idtree.create(parent_id);
        }
        context.idtree.insert_child(node_id, parent_id, 0);
    }
}

pub fn play_insert_before(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let node_id = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index() as usize;
    let borther = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 1).unwrap())) }.index() as usize;
    let node_id1 = context.nodes.get(node_id).unwrap().clone();
    let borther1 = context.nodes.get(borther).unwrap().clone();
    insert_before(gui, node_id1, borther1);

    if context.idtree.get(node_id).is_none() {
        context.idtree.create(node_id);
    }

    context.idtree.insert_brother(node_id, borther, InsertType::Front);
}

pub fn play_create_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { Entity::from_bits(transmute(ret.as_f64().unwrap())) }.index() as usize;
    let r = create_node(gui);
    context.nodes.insert(ret, r);
}

pub fn play_create_vnode(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { Entity::from_bits(transmute(ret.as_f64().unwrap())) }.index() as usize;
    context.nodes.insert(ret, create_vnode(gui));
}

pub fn play_create_text_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { Entity::from_bits(transmute(ret.as_f64().unwrap())) }.index() as usize;
    context.nodes.insert(ret, create_text_node(gui));
}

pub fn play_create_image_node(app: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { Entity::from_bits(transmute(ret.as_f64().unwrap())) }.index() as usize;
    context.nodes.insert(ret, create_image_node(app));
}

pub fn play_create_canvas_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let json = &json[0];
    let ret = &json["ret"];
    let ret = unsafe { Entity::from_bits(transmute(ret.as_f64().unwrap())) }.index() as usize;
    context.nodes.insert(ret, create_canvas_node(gui));
}


pub fn play_remove_node(gui: &mut Gui, context: &mut PlayContext, json: &Vec<json::JsonValue>) {
    let node_id = unsafe { Entity::from_bits(transmute(as_value::<f64>(json, 0).unwrap())) }.index() as usize;
    let node_id = context.nodes.get(node_id).unwrap().clone();
    remove_node(gui, node_id);
}
