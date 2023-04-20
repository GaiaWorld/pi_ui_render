// use bevy::ecs::{
//     entity::Entities,
//     query::QueryState,
//     system::{Query, Res, SystemState},
// };
// use pi_bevy_ecs_extend::prelude::{Down, Layer, OrDefault, Up};
// use pi_null::Null;
// use pi_style::style::Aabb2;
// use pi_time::Instant;
// use std::mem::transmute;

// #[cfg(target_arch = "wasm32")]
// use pi_async::prelude::{SingleTaskRunner, SingleTaskRuntime};
// // use pi_ecs::{
// //     prelude::{DispatcherMgr, IntoSystem, ResMut, SingleDispatcher, StageBuilder},
// //     world::World,
// // };
// use pi_sparialtree::quad_helper::intersects;
// use pi_share::{Share, ShareMutex};
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::wasm_bindgen;

// use bevy::app::prelude::App;

// pub mod json_parse;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod native_index;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod native_debug;
// pub mod rr;
// #[cfg(target_arch = "wasm32")]
// pub mod wasm_debug;
// #[cfg(target_arch = "wasm32")]
// pub mod wasm_index;

// pub mod style;




// #[cfg(not(target_arch = "wasm32"))]
// pub use native_index::*;
// #[cfg(not(target_arch = "wasm32"))]
// pub use native_debug::*;
// #[cfg(not(target_arch = "wasm32"))]
// pub use style::*;
// #[cfg(target_arch = "wasm32")]
// pub use wasm_index::*;

// use crate::{
//     components::{
//         calc::{InPassId, IsShow, LayoutResult, Quad, WorldMatrix, ZRange, EntityKey},
//         pass_2d::{GraphId, ParentPassId},
//         user::{Overflow, Point2},
//     },
//     prelude::UserCommands,
//     resource::QuadTree,
// };

// #[cfg(target_arch = "wasm32")]
// #[derive(Debug, Clone, Deref, DerefMut)]
// #[wasm_bindgen]
// pub struct Atom(pub(crate) pi_atom::Atom);

// #[cfg(not(target_arch = "wasm32"))]
// #[derive(Debug, Clone, Deref, DerefMut)]
// pub struct Atom(pub(crate) pi_atom::Atom);

// impl Atom {
//     pub fn new(value: pi_atom::Atom) -> Self { Self(value) }
// }
// // pub mod layout;

// // #[cfg(target_arch = "wasm32")]
// // #[wasm_bindgen]
// // pub struct Gui(pub crate::gui::Gui);

// // #[cfg(not(target_arch = "wasm32"))]
// // pub struct Gui(pub crate::gui::Gui);

// // #[cfg(target_arch = "wasm32")]
// // #[wasm_bindgen]
// // pub struct Engine {
// //     pub(crate) win: Arc<Window>,
// //     // pub dispatcher: SingleDispatcher<WorkerRuntime>,
// //     pub(crate) render_dispatcher: DefaultKey,
// //     pub(crate) dispatcher_mgr: DispatcherMgr,
// //     pub(crate) world: World,
// //     pub(crate) rt: SingleTaskRuntime,
// //     pub(crate) gui: crate::gui::Gui,
// // 	pub(crate) runner: SingleTaskRunner<()>,
// // }

// // #[cfg(not(target_arch = "wasm32"))]
// // pub struct Engine {
// //     pub win: Arc<Window>,
// //     // pub dispatcher: SingleDispatcher<WorkerRuntime>,
// //     // pub render_dispatcher: DefaultKey,
// //     // pub dispatcher_mgr: DispatcherMgr,
// //     // pub world: World,
// //     // pub rt: WorkerRuntime,
// //     pub app: App,

// // }

// #[cfg(not(target_arch = "wasm32"))]
// pub struct Gui {
//     pub entitys: &'static Entities,
//     pub commands: UserCommands,
//     pub down_query: QueryState<&'static Down>,
//     pub up_query: QueryState<&'static Up>,
//     pub layer_query: QueryState<&'static Layer>,
//     pub enable_query: QueryState<&'static IsShow>,
//     pub depth_query: QueryState<&'static ZRange>,
//     pub layout_query: QueryState<&'static LayoutResult>,
//     pub quad_query: QueryState<&'static Quad>,
//     pub matrix_query: QueryState<&'static WorldMatrix>,
//     pub overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
//     pub in_pass2d_query: QueryState<&'static InPassId>,
//     pub graph_id: QueryState<&'static GraphId>,
//     pub query_state: SystemState<(
//         Res<'static, QuadTree>,
//         Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
//         Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
//     )>,
// }

// #[cfg(not(target_arch = "wasm32"))]
// #[derive(Debug, Deref, DerefMut)]
// pub struct Engine(pub App);

// #[wasm_bindgen]
// #[cfg(target_arch = "wasm32")]
// pub struct Gui {
//     pub(crate) entitys: &'static Entities,
//     pub(crate) commands: UserCommands,
//     pub(crate) down_query: QueryState<&'static Down>,
//     pub(crate) up_query: QueryState<&'static Up>,
//     pub(crate) layer_query: QueryState<&'static Layer>,
//     pub(crate) enable_query: QueryState<&'static IsShow>,
//     pub(crate) depth_query: QueryState<&'static ZRange>,
//     pub(crate) layout_query: QueryState<&'static LayoutResult>,
//     pub(crate) quad_query: QueryState<&'static Quad>,
//     pub(crate) matrix_query: QueryState<&'static WorldMatrix>,
//     pub(crate) overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
//     pub(crate) in_pass2d_query: QueryState<&'static InPassId>,
//     pub(crate) graph_id: QueryState<&'static GraphId>,
//     pub(crate) query_state: SystemState<(
//         Res<'static, QuadTree>,
//         Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
//         Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
//     )>,
// }

// #[wasm_bindgen]
// #[cfg(target_arch = "wasm32")]
// #[derive(Debug, Deref, DerefMut)]
// pub struct Engine(App);

// // #[cfg(not(target_arch = "wasm32"))]
// // pub fn create_engine(win: &Arc<Window>, _r: f64) -> Engine {
// // 	let event_loop = winit::event_loop::EventLoop::new();
// // 	Arc::new(winit::window::WindowBuilder::new().build(&event_loop));

// //     let size = win.inner_size();
// //     let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

// //     let world = World::new();

// //     let mut world1 = world.clone();
// //     let win1 = win.clone();
// //     let rt = runtime.clone();

// //     let mut result: Share<ShareRwLock<Option<(Gui, DispatcherMgr, DefaultKey)>>> = Share::new(ShareRwLock::new(None));
// //     let result1 = result.clone();

// //     let _ = runtime.spawn(runtime.alloc(), async move {
// //         let world = &mut world1;
// //         let options = RenderOptions::default();
// //         let mut dispatcher_mgr = DispatcherMgr::default();
// //         let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

// //         init_data(world, win1);

// //         let mut stages = Vec::new();

// //         let mut first_stage = StageBuilder::new();
// //         let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
// //             frame_start_time.0 = Instant::now();
// //         };
// //         first_stage.add_node(IntoSystem::system(first_run, world));
// //         stages.push(Share::new(first_stage.build(world)));

// //         // 初始化gui stage
// //         let mut gui = Gui::new(world);
// //         let gui_stages = gui.init(0, 0, size.width, size.height, rt.clone(), &mut dispatcher_mgr);
// //         stages.push(Share::new(gui_stages.node_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.post_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.draw_obj_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.pass_2d_stage.build(world)));

// //         stages.push(Share::new(render_stages.prepare_stage.build(world)));
// //         stages.push(Share::new(render_stages.render_stage.build(world)));

// // 		stages.push(Share::new(gui_stages.clear_stage.build(world)));

// //         let mut dispatcher = SingleDispatcher::new(rt);
// //         dispatcher.init(stages, world);
// //         let render_diapatcher = dispatcher_mgr.insert(Box::new(dispatcher));

// //         *result1.write() = Some((gui, dispatcher_mgr, render_diapatcher));
// //     });
// //     loop {
// //         if result.read().is_some() {
// //             match Share::try_unwrap(result) {
// //                 Ok(r) => {
// //                     let r = r.into_inner().unwrap();
// //                     let engine = Box::new(Engine {
// //                         win: win.clone(),
// //                         // dispatcher_mgr: r.1,
// //                         // render_dispatcher: r.2,
// //                         // world: World::new(),
// //                         app: App::default(),
// //                         // rt: runtime.clone(),
// //                     });
// //                     return Box::into_inner(engine);
// //                 }
// //                 Err(r) => result = r,
// //             }
// //         }
// //     }
// // }

// // #[cfg(target_arch = "wasm32")]
// // pub fn create_engine_wasm(win: &Arc<Window>, _r: f64) -> Engine {
// //     let size = win.inner_size();
// // 	let mut runner = SingleTaskRunner::default();
// // 	let runtime = runner.startup().unwrap();

// //     let world = World::new();

// //     let mut world1 = world.clone();
// //     let win1 = win.clone();
// //     let rt = runtime.clone();

// //     let mut result: Share<ShareRwLock<Option<(Gui, DispatcherMgr, DefaultKey)>>> = Share::new(ShareRwLock::new(None));
// //     let result1 = result.clone();

// //     let _ = runtime.spawn(runtime.alloc(), async move {
// //         let world = &mut world1;
// // 		let mut options = RenderOptions::default();
// // 		options.backends = wgpu::Backends::GL;
// //         let mut dispatcher_mgr = DispatcherMgr::default();
// // 		log::info!("init_render, options: {:?}", options);
// //         let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

// //         init_data_wasm(world, win1);

// //         let mut stages = Vec::new();

// //         let mut first_stage = StageBuilder::new();
// //         let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
// //             frame_start_time.0 = Instant::now();
// //         };
// //         first_stage.add_node(IntoSystem::system(first_run, world));
// //         stages.push(Share::new(first_stage.build(world)));

// //         // 初始化gui stage
// //         let mut gui = Gui::new(world);
// //         let gui_stages = gui.init(0, 0, size.width, size.height, rt.clone(), &mut dispatcher_mgr);

// // 		stages.push(Share::new(gui_stages.node_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.post_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.draw_obj_stage.build(world)));
// // 		stages.push(Share::new(gui_stages.pass_2d_stage.build(world)));

// //         stages.push(Share::new(render_stages.prepare_stage.build(world)));
// //         stages.push(Share::new(render_stages.render_stage.build(world)));

// // 		stages.push(Share::new(gui_stages.clear_stage.build(world)));

// //         let mut dispatcher = SingleDispatcher::new(rt);
// //         dispatcher.init(stages, world);
// //         let render_diapatcher = dispatcher_mgr.insert(Box::new(dispatcher));

// //         *result1.write() = Some((gui, dispatcher_mgr, render_diapatcher));
// //     });
// // 	runner.run();
// //     loop {
// //         if result.read().is_some() {
// //             match Share::try_unwrap(result) {
// //                 Ok(r) => {
// //                     let r = r.into_inner().unwrap();
// //                     let engine = Box::new(Engine {
// //                         win: win.clone(),
// //                         dispatcher_mgr: r.1,
// //                         render_dispatcher: r.2,
// //                         world: World::new(),
// //                         gui: r.0,
// //                         rt: runtime.clone(),
// // 						runner,
// //                     });
// //                     return Box::into_inner(engine);
// //                 }
// //                 Err(r) => result = r,
// //             }
// //         }
// //     }
// // }

// // fn init_data(world: &mut World, win: Arc<Window>) {
// //     // 创建 RenderWindow
// //     let render_window = RenderWindow::new(win, PresentMode::Mailbox);
// //     let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
// //     render_windows.insert(render_window);
// // }

// // #[cfg(target_arch = "wasm32")]
// // fn init_data_wasm(world: &mut World, win: Arc<Window>) {
// //     // 创建 RenderWindow
// //     let render_window = RenderWindow::new(win, PresentMode::Fifo);
// //     let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
// //     render_windows.insert(render_window);
// // }

// pub struct PreFrameTime(pub Share<ShareMutex<Instant>>);
// pub struct FrameStartTime(pub Instant);
// impl Default for FrameStartTime {
//     fn default() -> Self { Self(Instant::now()) }
// }

// impl Default for PreFrameTime {
//     fn default() -> Self { Self(Share::new(ShareMutex::new(Instant::now()))) }
// }

// /// 用点命中一个节点
// #[allow(unused_attributes)]
// pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
//     let query = gui.query_state.get(&mut engine.world);

//     let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
//     let mut args = AbQueryArgs {
//         query: query.1,
//         query_parent: query.2,
//         aabb,
//         result: EntityKey::null(),
//         max_z: usize::MIN,
//     };
//     query.0.query(&aabb, intersects, &mut args, ab_query_func);
//     if args.result.is_null() {
//         None
//     } else {
//         Some(unsafe { transmute(args.result.to_bits()) })
//     }
// }

// /// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
// fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, aabb: &Aabb2, _bind: &()) {
//     let (_layer, _is_show, z_range, inpass) = match arg.query.get(*id) {
//         // 如果enable false 表示不接收事件, visibility为false， 也无法接收事件、不在树上也不能接收事件
//         Ok(r) if (r.0.layer() != 0 && r.1.get_enable() && r.1.get_visibility()) => r,
//         _ => return,
//     };

//     if intersects(&arg.aabb, aabb) {
//         // 取最大z的node
//         if z_range.start > arg.max_z {
//             // 检查是否有裁剪，及是否在裁剪范围内
//             let mut inpass = inpass.0;
//             while !inpass.is_null() {
//                 if let Ok((parent, quad, oveflow)) = arg.query_parent.get(*id) {
//                     inpass = parent.0;
//                     if oveflow.0 {
//                         if !intersects(&arg.aabb, quad) {
//                             return; // 如果不想交，直接返回，该点不能命中该节点
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//             arg.result = id;
//             arg.max_z = z_range.start;
//         }
//     }
// }

// pub struct AbQueryArgs<'s, 'w> {
//     query: Query<'s, 'w, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
//     query_parent: Query<'s, 'w, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
//     aabb: Aabb2,
//     result: EntityKey,
//     max_z: usize,
// }
