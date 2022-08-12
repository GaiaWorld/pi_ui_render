use crate::{utils::cmd::SingleCmd, resource::{ClearColor, Viewport}, components::{user::{CgColor, Aabb2, Point2, Node}, calc::{QuadTree, IsEnable}, pass_2d::Pass2D}};
use pi_style::{style_parse::parse_class_map_from_string, style_type::ClassSheet};

pub use crate::export::Gui;
use js_proxy_gen_macro::pi_js_export;
use pi_cg2d::include_quad2;
use pi_null::Null;
use pi_share::ShareRefCell;
use pi_spatialtree::quad_helper::intersects;
use wgpu::PresentMode;
use winit::window::Window;
use pi_async::rt::{AsyncRuntimeBuilder, worker_thread::WorkerRuntime};
use pi_ecs::{prelude::{World, SingleDispatcher, Dispatcher}, entity::Id, storage::LocalVersion};
use pi_render::{init_render, components::view::{target_alloc::ShareTargetView, render_window::{RenderWindow, RenderWindows}}, rhi::options::RenderOptions};
use std::{sync::Arc, intrinsics::transmute};

use pi_ecs::{prelude::StageBuilder, storage::Local};

#[pi_js_export]
pub fn create_engine(win: &Arc<Window>, r: usize) -> Engine {
	let runtime = AsyncRuntimeBuilder::default_worker_thread(
		None,
		None,
		None,
		None,
	);
	Engine {
		win: win.clone(),
		dispatcher: ShareRefCell::new(SingleDispatcher::new(runtime.clone())),
		world: World::new(),
		rt: runtime.clone(),
	}
}

#[pi_js_export]
pub fn get_font_sheet(gui: &mut Gui) -> u32 {
	0
}

#[pi_js_export]
pub fn get_class_sheet(gui: &mut Gui) -> u32 {
	0
}

#[pi_js_export]
pub fn create_render_target(gui: &mut Gui, fbo: u32) -> u32 {
	0
}


#[pi_js_export]
pub fn destroy_render_target(gui: &mut Gui, fbo: u32) -> u32 {
	0
}

#[pi_js_export]
pub fn bind_render_target(gui: &mut Gui) {
}

#[pi_js_export]
pub fn clone_engine(engine: &Engine) -> Engine {
	Engine {
		win: engine.win.clone(),
		dispatcher: engine.dispatcher.clone(),
		world: engine.world.clone(),
		rt: engine.rt.clone(),
	}
}

// TODO
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[pi_js_export]
pub async fn create_gui(engine: &mut Engine, width: u32, height: u32, load_image_fun: u32, class_sheet: u32, font_sheet: u32) -> Gui {

    let mut gui = Gui(crate::gui::Gui::new(&mut engine.world));

	let world = &mut engine.world;

	let options = RenderOptions::default();
	let render_stages = init_render::<Option<ShareTargetView>, _>(world, options, engine.win.clone(), engine.rt.clone()).await;

	init_data(world, engine.win.clone());

	// 初始化gui stage
	let gui_stages = gui.0.init(0, 0, width, height);
	let mut stages = Vec::new();
	for stage in gui_stages.into_iter() {
		stages.push(Arc::new(stage.build(world)));
	}
	stages.push(Arc::new(render_stages.extract_stage.build(world)));
	stages.push(Arc::new(render_stages.prepare_stage.build(world)));
	stages.push(Arc::new(render_stages.render_stage.build(world)));

	engine.dispatcher.0.borrow_mut().init(stages, world); // 应该由外部添加 TODO

	gui
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn get_text_texture_width(gui: &mut Gui) -> u32 {
	0
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn get_text_texture_height(gui: &mut Gui) -> u32 {
	0
}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn set_pixel_ratio(gui: &mut Gui, pixel_ratio: f32) {
}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn set_clear_color(gui: &mut Gui, r: f32, g: f32, b: f32, a: f32) {
	gui.0.push_cmd(SingleCmd(ClearColor(CgColor::new(r, g, b, a))));
}

#[allow(unused_attributes)]
#[allow(unused_unsafe)]
#[pi_js_export]
pub fn nullify_clear_color(gui: &mut Gui) {
}

#[pi_js_export]
pub fn set_view_port(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32) {
	gui.0.push_cmd(SingleCmd(Viewport(Aabb2::new(Point2::new(x as f32, y as f32), Point2::new(width as f32, height as f32)))));
}

/// 设置视口
#[pi_js_export]
pub fn set_scissor(gui: &mut Gui, x: i32, y: i32, width: i32, height: i32) {
}

#[pi_js_export]
pub fn set_project_transfrom(
    gui: &mut Gui,
    scale_x: f32,
    scale_y: f32,
    translate_x: f32,
    translate_y: f32,
	rotate: f32,
	width: u32,
	height: u32,
) {
}

#[pi_js_export]
pub fn force_update_text(gui: &mut Gui, node_id: u32) {
}


#[pi_js_export]
pub fn render(gui: &mut Gui) {
	gui.0.run();
	// dispach run TODO
}

/// 强制计算一次
#[pi_js_export]
pub fn calc(gui: &mut Gui) {
}

#[pi_js_export]
pub fn calc_geo(gui: &mut Gui) {
}

/// 强制计算一次布局
#[pi_js_export]
pub fn cal_layout(gui: &mut Gui) {
}

//设置shader
#[pi_js_export]
pub fn set_shader(engine: &mut Engine, shader_name: String, shader_code: String) {
}

// 创建class
#[pi_js_export]
pub fn create_class(gui: &mut Gui, css: &str) {
	let mut class_sheet = ClassSheet::default();
   	match parse_class_map_from_string(css, &mut class_sheet) {
        Ok(r) => r,
        Err(e) => {
			log::warn!("{:?}", e);
            return;
        }
    };

	gui.0.push_cmd(SingleCmd(class_sheet));
}

/// 添加二进制格式的css表
#[pi_js_export]
pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
    let class_sheet: ClassSheet = match bincode::deserialize(bin) {
        Ok(r) => r,
        Err(e) => {
			log::warn!("deserialize_class_map error: {:?}", e);
            return;
        }
    };

	gui.0.push_cmd(SingleCmd(class_sheet));
}

#[pi_js_export]
pub fn node_is_exist(gui: &mut Gui, node: u32) -> bool {
	return true
}

#[pi_js_export]
pub fn node_is_visibility(gui: &mut Gui, node: u32) -> bool {
	true
}

#[pi_js_export]
pub fn first_child(gui: &mut Gui, parent: f64) -> Option<f64> {
	let node: Id<Node> = unsafe { transmute(parent) };
	match gui.0.down_query.get(&gui.0.world, node) {
		Some(r) => if r.head.is_null() {
			None
		} else {
			Some(unsafe { transmute(r.head)})
		},
		None => None
	}
}

#[pi_js_export]
pub fn last_child(gui: &mut Gui, parent: f64) -> Option<f64> {
	let node: Id<Node> = unsafe { transmute(parent) };
	match gui.0.down_query.get(&gui.0.world, node) {
		Some(r) => if r.tail.is_null() {
			None
		} else {
			Some(unsafe { transmute(r.tail)})
		},
		None => None
	}
}


#[pi_js_export]
pub fn next_sibling(gui: &mut Gui, node: f64) -> Option<f64> {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.up_query.get(&gui.0.world, node) {
		Some(r) => if r.next().is_null() {
			None
		} else {
			Some(unsafe { transmute(r.next())})
		},
		None => None
	}
}

#[pi_js_export]
pub fn previous_sibling(gui: &mut Gui, node: f64) -> Option<f64> {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.up_query.get(&gui.0.world, node) {
		Some(r) => if r.prev().is_null() {
			None
		} else {
			Some(unsafe { transmute(r.prev())})
		},
		None => None
	}
}

#[pi_js_export]
pub fn get_layer(gui: &mut Gui, node: f64) -> usize {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.layer_query.get(&gui.0.world, node) {
		Some(r) => **r,
		None => 0
	}
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的上边界的距离
#[pi_js_export]
pub fn offset_top(gui: &mut Gui, node: f64) -> usize {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.layout_query.get(&gui.0.world, node) {
		Some(r) => r.rect.top as usize,
		None => 0
	}
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点到gui的左边界的距离
#[pi_js_export]
pub fn offset_left(gui: &mut Gui, node: f64) -> usize {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.layout_query.get(&gui.0.world, node) {
		Some(r) => r.rect.left as usize,
		None => 0
	}
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点的布局宽度
#[pi_js_export]
pub fn offset_width(gui: &mut Gui, node: f64) -> usize {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.layout_query.get(&gui.0.world, node) {
		Some(r) => (r.rect.right - r.rect.left) as usize,
		None => 0
	}
}

/// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
/// 节点布局高度
#[pi_js_export]
pub fn offset_height(gui: &mut Gui, node: f64) -> usize {
	let node: Id<Node> = unsafe { transmute(node) };
	match gui.0.layout_query.get(&gui.0.world, node) {
		Some(r) => (r.rect.bottom - r.rect.top) as usize,
		None => 0
	}
}

// /// 等同于html的getBoundingClientRect TODO
// /// left top width height
// #[pi_js_export]
// pub fn offset_document(gui: &mut Gui, node_id: u32) -> JsValue {
//     let node_id = node_id as usize;
//     let world = unsafe { &mut *(world as usize as *mut GuiWorld) };
//     let world = &mut world.gui;
//     // let layouts = world.layout.lend();
//     // let world_matrixs = world.world_matrix.lend();
//     // let transforms = world.transform.lend();
//     let octs = world.oct.lend();
//     // debug_println!("oct====================={:?}, {:?}", node_id, oct);
//     match octs.get(node_id) {
//         Some((oct, _)) => JsValue::from_serde(&Rect{left: oct.mins.x, top: oct.mins.y, width: oct.maxs.x - oct.mins.x, height: oct.maxs.y - oct.mins.y}).unwrap() ,
//         None => JsValue::from_serde(&Rect{left: 0.0, top: 0.0, width: 0.0, height: 0.0}).unwrap(),
//     }

//     // let transform;
//     // let transform1;
//     // match transforms.get(node_id) {
//     //     Some(r) => transform = r,
//     //     None => {
//     //         transform1 = Transform::default();
//     //         transform = &transform1;
//     //     }
//     // };

//     // let layout = unsafe { layouts.get_unchecked(node_id) };
//     // let origin = transform.origin.to_value(layout.width, layout.height);

//     // let world_matrix = unsafe { world_matrixs.get_unchecked(node_id) };
//     // let point = Vector4::new(
//     //     -origin.x + layout.border_left + layout.padding_left,
//     //     -origin.y + layout.border_top + layout.padding_top,
//     //     1.0,
//     //     1.0,
//     // );
//     // let left_top = world_matrix.0 * point;

//     // js! {
//     //     __jsObj.left = @{layout.left};
//     //     __jsObj.top = @{layout.top};
//     //     __jsObj.width = @{layout.width};
//     //     __jsObj.height = @{layout.height};
//     // }
// }

// /// content宽高的累加值
// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn content_box(gui: &mut Gui, node: u32) -> JsValue {
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
	let quad = gui.0.quad_component_comtainer.clone();
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
		Some(unsafe{transmute(args.result)})
	}
}

/// 调试使用， 设置渲染脏， 使渲染系统在下一帧进行渲染
#[pi_js_export]
pub fn set_render_dirty(gui: &mut Gui) {
}

/// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
fn ab_query_func(arg: &mut AbQueryArgs, id: LocalVersion, aabb: &Aabb2, bind: &()) {
    match arg.gui.0.layer_query.get(&arg.gui.0.world, unsafe {Id::<Node>::new(id)}) {
        Some(layer) => {
            if **layer == 0 {
                return;
            }
        }
        None => return,
    };
    if intersects(&arg.aabb, aabb) {
		let enable = arg.gui.0.enable_query.get(&arg.gui.0.world, unsafe {Id::<Node>::new(id)});
		let z_depth = arg.gui.0.depth_query.get(&arg.gui.0.world, unsafe {Id::<Node>::new(id)}).unwrap();
        // log::info!("enable----------id: {}, enable: {}, z_depth: {}, max_z: {}", bind, enable, z_depth,  arg.max_z);
        //如果enable true 表示不接收事件
		if enable.unwrap_or(&IsEnable(false)).0 {
			return
		}

        // 取最大z的node
        if z_depth.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
			let mut inpass = arg.gui.0.in_pass2d_query.get(&arg.gui.0.world, unsafe {Id::<Node>::new(id)}).unwrap();

			while !inpass.0.is_null() {
				let (quad, oveflow, parent) = arg.gui.0.overflow_query.get(&arg.gui.0.world, inpass.0).unwrap();
				
				if oveflow.0 {
					if !intersects(&arg.aabb, quad) {
						return; // 如果不想交，直接返回，该点不能命中该节点
					}
				}
			}

			arg.result = unsafe {Id::<Node>::new(id)};
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

pub struct Engine {
	win: Arc<Window>,
	dispatcher: ShareRefCell<SingleDispatcher<WorkerRuntime>>,
	world: World,
	rt: WorkerRuntime,
}

fn init_data(world: &mut World, win: Arc<Window>) {
	// 创建 RenderWindow
	let render_window = RenderWindow::new(win, PresentMode::Mailbox);
	let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
	render_windows.insert(render_window);

}