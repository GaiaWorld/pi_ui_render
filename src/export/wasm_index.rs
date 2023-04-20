use crate::components::user::Overflow;
use crate::components::user::{RenderDirty, TextContent as TextContent1};
use crate::components::NodeBundle;
use crate::{
    components::{
        calc::{EntityKey, InPassId, IsShow, Quad, ZRange},
        pass_2d::ParentPassId,
        user::{Aabb2, CgColor, ClearColor, Point2, Viewport},
    },
    prelude::{UiPlugin, UserCommands},
    resource::{ExtendCssCmd, NodeCmd, QuadTree},
    system::node::user_setting::user_setting,
    utils::cmd::SingleCmd,
};
use bevy::ecs::{
    prelude::Entity,
    system::{CommandQueue, Query, SystemState},
    world::WorldCell,
};
use bevy::window::WindowId;
use pi_animation::{animation_group::AnimationGroupID, animation_listener::EAnimationEvent};
use pi_async::prelude::AsyncRuntime;
use pi_bevy_ecs_extend::prelude::{Layer, OrDefault};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_hash::XHashMap;
use pi_idtree::InsertType;
use pi_slotmap::SecondaryMap;
use pi_style::{
    style::*,
    style_parse::{
        parse_animation, parse_class_map_from_string, parse_comma_separated, parse_text_shadow, ClassMap, KeyFrameList, StyleParse,
        ValueParseErrorKind,
    },
    style_type::*,
};
use smallvec::SmallVec;

pub use super::Gui;
use super::{json_parse::as_value, style::{PlayContext, Atom}};
// pub use crate::gui::Gui;
use super::Engine;
use bevy::app::App;
use cssparser::ParseError;
use js_proxy_gen_macro::pi_js_export;
use js_sys::{Array, Function};
use pi_async::prelude::SingleTaskRunner;
use pi_bevy_winit_window::WinitPlugin;
use pi_hal::runtime::{RENDER_RUNTIME, RUNNER_MULTI, RUNNER_RENDER};
use pi_null::Null;
use pi_sparialtree::quad_helper::intersects;
use std::{
    intrinsics::transmute,
    mem::swap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::HtmlCanvasElement;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
pub use winit::platform::web::WindowBuilderExtWebSys;
pub use winit::window::{Window, WindowBuilder};
use crate::resource::animation_sheet::KeyFramesSheet;



/// width、height为physical_size
#[wasm_bindgen]
pub fn create_engine(canvas: HtmlCanvasElement, r: f64, width: u32, height: u32) -> Engine {
    let mut app = App::default();

    let mut window_plugin = bevy::window::WindowPlugin::default();
    window_plugin.add_primary_window = false;
    app
        // .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(window_plugin)
        .add_plugin(pi_bevy_winit_window::WinitPlugin::new(canvas, WindowId::primary()).with_size(width, height))
        .add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
        .add_plugin(PiPostProcessPlugin)
		.add_plugin(RuntimePlugin); // 推动运行时
    Engine(app)
}

#[wasm_bindgen]
pub fn create_gui(
    context: JsValue,
    engine: &mut Engine,
    width: f32,
    height: f32,
    load_image_fun: Option<Function>,
    class_sheet: u32,
    font_sheet: u32,
    cur_time: u32,
    animation_event_fun: Function,
) -> Gui {
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

    // /// 设置动画的监听器
    // let a_callback = Box::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (Entity, pi_atom::Atom)>| {
    // 	let mut arr = Array::new();
    // 	for (group_id, ty, count) in list.iter() {
    // 		match map.get(*group_id) {
    // 			Some(r) => {
    // 				arr.push(&JsValue::from_f64(unsafe { transmute(r.0) }));
    // 				arr.push(&JsValue::from_f64(r.1.get_hash() as f64));
    // 			},
    // 			None => continue,
    // 		};
    // 		arr.push(&JsValue::from_f64(unsafe {transmute::<_, u8>(*ty)}  as f64));
    // 		arr.push(&JsValue::from_f64(*count as f64));
    // 	}
    // 	animation_event_fun.call1(&context, &JsValue::from(arr))
    //                     .expect("call animation event fail!!!");
    // });
    // gui.commands.set_event_listener(a_callback);

    gui
}

// 取出动画事件
#[wasm_bindgen]
pub fn get_animation_events(
    engine: &Engine,
) -> Option<js_sys::ArrayBuffer> {
	let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();

	let events = key_frames.get_animation_events();
	let map = key_frames.get_group_bind();
	let mut arr = js_sys::Uint32Array::new_with_length(events.len() as u32);
	let mut i = 0;
	for (group_id, ty, count) in events.iter() {
		match map.get(*group_id) {
			Some(r) => {
				arr.set_index(i, r.0.index()); // entity
				arr.set_index(i + 4, r.0.generation());
				arr.set_index(i + 8, r.1.get_hash() as u32); // name hash
			},
			None => continue,
		};
		arr.set_index(i + 12, unsafe {transmute::<_, u8>(*ty)}  as u32); // event type
		arr.set_index(i + 16, *count); // cur iter count
		i += 20;
	}
	if arr.byte_length() > 0 {
		Some(arr.buffer())
	} else {
		None
	}
	
	// arr
}

#[wasm_bindgen]
pub struct OffsetDocument {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

/// content宽高的累加值
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn content_box(gui: &mut Gui, engine: &mut Engine, node_id: f64) -> JsValue {
    let node: Entity = Entity::from_bits(unsafe { transmute(node_id) });
    let mut cur_child = match gui.down_query.get(&engine.world, node) {
        Ok(down) => down.head(),
        _ => return JsValue::from_serde(&Size { width: 0.0, height: 0.0 }).unwrap(),
    };

    let (mut left, mut right, mut top, mut bottom) = (std::f32::MAX, 0.0, std::f32::MAX, 0.0);
    while !EntityKey(cur_child).is_null() {
        let l = match gui.layout_query.get(&engine.world, cur_child) {
            Ok(r) => r,
            _ => break,
        };
        let r = l.rect.right;
        let b = l.rect.bottom;
        if l.rect.left < left {
            left = l.rect.left;
        }
        if r > right {
            right = r;
        }
        if b > bottom {
            bottom = b;
        }
        if l.rect.top < top {
            top = l.rect.top;
        }

        cur_child = match gui.up_query.get(&engine.world, cur_child) {
            Ok(r) => r.next(),
            _ => break,
        };
    }
    JsValue::from_serde(&Size {
        width: right - left,
        height: bottom - top,
    })
    .unwrap()
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
pub fn offset_document(gui: &mut Gui, engine: &mut Engine, node: f64) -> OffsetDocument {
    let node: Entity = Entity::from_bits(unsafe { transmute(node) });
    match gui.quad_query.get(&engine.world, node) {
        Ok(quad) => OffsetDocument {
            left: quad.mins.x,
            top: quad.mins.y,
            width: quad.maxs.x - quad.mins.x,
            height: quad.maxs.y - quad.mins.y,
        },
        _ => OffsetDocument {
            left: 0.0,
            top: 0.0,
            width: 0.0,
            height: 0.0,
        },
    }
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
pub fn get_canvas_target(gui: &mut Gui, index: usize) -> Option<usize> { Some(1) }

#[wasm_bindgen]
pub fn get_canvas_rect(gui: &mut Gui, index: usize) -> JsValue { JsValue::from_serde(&CanvasRect(0, 0, 0, 0)).unwrap() }

/**
 * canvas内容发生改变时，应该调用此方法更新gui渲染
*/
#[wasm_bindgen]
pub fn update_canvas(gui: &mut Gui, _node: u32) {}

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

#[inline]
fn run_all(rt: &SingleTaskRunner<()>) {
    while let Ok(r) = rt.run() {
        if r == 0 {
            break;
        }
    }
}

// wasm 使用单线程运行时，需要手动推
pub struct RuntimePlugin;

impl bevy::app::Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
			bevy::prelude::CoreStage::First,
			RuntimeStage::Start,
			bevy::prelude::SystemStage::single(|| {
				run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
				run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
			}),
		);

		let last_stage = app.schedule.iter_stages();
		let mut last = None;
		for i in last_stage {
			last = Some(i.0);
		}
		let last = last.unwrap();
		app.add_stage_after(
			last,
			RuntimeStage::End,
			bevy::prelude::SystemStage::single(|| {
				run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
				run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
			}),
		);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, bevy::ecs::schedule::StageLabel)]
pub enum RuntimeStage {
	Start,
	End
}
