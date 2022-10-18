use std::{
    sync::{Arc},
};
use pi_time::Instant;

use pi_async::prelude::{WorkerRuntime, AsyncRuntime, AsyncRuntimeBuilder};
#[cfg(target_arch = "wasm32")]
use pi_async::prelude::{SingleTaskRuntime, SingleTaskRunner};
use pi_ecs::{
    prelude::{DispatcherMgr, IntoSystem, ResMut, SingleDispatcher, StageBuilder},
    world::World,
};
use pi_render::{
    components::view::render_window::{RenderWindow, RenderWindows},
    init_render,
    rhi::options::RenderOptions,
};
use pi_share::{Share, ShareRwLock, ShareMutex};
use pi_slotmap::DefaultKey;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use wgpu::PresentMode;
use winit::window::Window;

use crate::gui::Gui;

pub mod json_parse;
#[cfg(not(target_arch = "wasm32"))]
mod native_index;
#[cfg(target_arch = "wasm32")]
mod wasm_index;
#[cfg(target_arch = "wasm32")]
mod wasm_debug;
mod rr;

mod style;


#[cfg(not(target_arch = "wasm32"))]
pub use native_index::*;
pub use style::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_index::*;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Deref, DerefMut)]
#[wasm_bindgen]
pub struct Atom(pub(crate) pi_atom::Atom);

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Atom(pub(crate) pi_atom::Atom);

impl Atom {
    pub fn new(value: pi_atom::Atom) -> Self { Self(value) }
}
// pub mod layout;

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub struct Gui(pub crate::gui::Gui);

// #[cfg(not(target_arch = "wasm32"))]
// pub struct Gui(pub crate::gui::Gui);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Engine {
    pub(crate) win: Arc<Window>,
    // pub dispatcher: SingleDispatcher<WorkerRuntime>,
    pub(crate) render_dispatcher: DefaultKey,
    pub(crate) dispatcher_mgr: DispatcherMgr,
    pub(crate) world: World,
    pub(crate) rt: SingleTaskRuntime,
    pub(crate) gui: crate::gui::Gui,
	pub(crate) runner: SingleTaskRunner<()>,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Engine {
    pub win: Arc<Window>,
    // pub dispatcher: SingleDispatcher<WorkerRuntime>,
    pub render_dispatcher: DefaultKey,
    pub dispatcher_mgr: DispatcherMgr,
    pub world: World,
    pub rt: WorkerRuntime,
    pub gui: crate::gui::Gui,

}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_engine(win: &Arc<Window>, _r: f64) -> Engine {
	let event_loop = winit::event_loop::EventLoop::new();
	Arc::new(winit::window::WindowBuilder::new().build(&event_loop));

    let size = win.inner_size();
    let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

    let world = World::new();

    let mut world1 = world.clone();
    let win1 = win.clone();
    let rt = runtime.clone();

    let mut result: Share<ShareRwLock<Option<(Gui, DispatcherMgr, DefaultKey)>>> = Share::new(ShareRwLock::new(None));
    let result1 = result.clone();

    let _ = runtime.spawn(runtime.alloc(), async move {
        let world = &mut world1;
        let options = RenderOptions::default();
        let mut dispatcher_mgr = DispatcherMgr::default();
        let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

        init_data(world, win1);

        let mut stages = Vec::new();

        let mut first_stage = StageBuilder::new();
        let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
            frame_start_time.0 = Instant::now();
        };
        first_stage.add_node(IntoSystem::system(first_run, world));
        stages.push(Share::new(first_stage.build(world)));

        // 初始化gui stage
        let mut gui = Gui::new(world);
        let gui_stages = gui.init(0, 0, size.width, size.height, rt.clone(), &mut dispatcher_mgr);
        stages.push(Share::new(gui_stages.node_stage.build(world)));
		stages.push(Share::new(gui_stages.post_stage.build(world)));
		stages.push(Share::new(gui_stages.draw_obj_stage.build(world)));
		stages.push(Share::new(gui_stages.pass_2d_stage.build(world)));

        stages.push(Share::new(render_stages.prepare_stage.build(world)));
        stages.push(Share::new(render_stages.render_stage.build(world)));

		stages.push(Share::new(gui_stages.clear_stage.build(world)));

        let mut dispatcher = SingleDispatcher::new(rt);
        dispatcher.init(stages, world);
        let render_diapatcher = dispatcher_mgr.insert(Box::new(dispatcher));

        *result1.write() = Some((gui, dispatcher_mgr, render_diapatcher));
    });
    loop {
        if result.read().is_some() {
            match Share::try_unwrap(result) {
                Ok(r) => {
                    let r = r.into_inner().unwrap();
                    let engine = Box::new(Engine {
                        win: win.clone(),
                        dispatcher_mgr: r.1,
                        render_dispatcher: r.2,
                        world: World::new(),
                        gui: r.0,
                        rt: runtime.clone(),
                    });
                    return Box::into_inner(engine);
                }
                Err(r) => result = r,
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn create_engine_wasm(win: &Arc<Window>, _r: f64) -> Engine {
    let size = win.inner_size();
	let mut runner = SingleTaskRunner::default();
	let runtime = runner.startup().unwrap();

    let world = World::new();

    let mut world1 = world.clone();
    let win1 = win.clone();
    let rt = runtime.clone();

    let mut result: Share<ShareRwLock<Option<(Gui, DispatcherMgr, DefaultKey)>>> = Share::new(ShareRwLock::new(None));
    let result1 = result.clone();

    let _ = runtime.spawn(runtime.alloc(), async move {
        let world = &mut world1;
		let mut options = RenderOptions::default();
		options.backends = wgpu::Backends::GL;
        let mut dispatcher_mgr = DispatcherMgr::default();
		log::info!("init_render, options: {:?}", options);
        let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

        init_data_wasm(world, win1);

        let mut stages = Vec::new();

        let mut first_stage = StageBuilder::new();
        let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
            frame_start_time.0 = Instant::now();
        };
        first_stage.add_node(IntoSystem::system(first_run, world));
        stages.push(Share::new(first_stage.build(world)));

        // 初始化gui stage
        let mut gui = Gui::new(world);
        let gui_stages = gui.init(0, 0, size.width, size.height, rt.clone(), &mut dispatcher_mgr);
        
		stages.push(Share::new(gui_stages.node_stage.build(world)));
		stages.push(Share::new(gui_stages.post_stage.build(world)));
		stages.push(Share::new(gui_stages.draw_obj_stage.build(world)));
		stages.push(Share::new(gui_stages.pass_2d_stage.build(world)));

        stages.push(Share::new(render_stages.prepare_stage.build(world)));
        stages.push(Share::new(render_stages.render_stage.build(world)));

		stages.push(Share::new(gui_stages.clear_stage.build(world)));

        let mut dispatcher = SingleDispatcher::new(rt);
        dispatcher.init(stages, world);
        let render_diapatcher = dispatcher_mgr.insert(Box::new(dispatcher));

        *result1.write() = Some((gui, dispatcher_mgr, render_diapatcher));
    });
	runner.run();
    loop {
        if result.read().is_some() {
            match Share::try_unwrap(result) {
                Ok(r) => {
                    let r = r.into_inner().unwrap();
                    let engine = Box::new(Engine {
                        win: win.clone(),
                        dispatcher_mgr: r.1,
                        render_dispatcher: r.2,
                        world: World::new(),
                        gui: r.0,
                        rt: runtime.clone(),
						runner,
                    });
                    return Box::into_inner(engine);
                }
                Err(r) => result = r,
            }
        }
    }
}

fn init_data(world: &mut World, win: Arc<Window>) {
    // 创建 RenderWindow
    let render_window = RenderWindow::new(win, PresentMode::Mailbox);
    let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
    render_windows.insert(render_window);
}

#[cfg(target_arch = "wasm32")]
fn init_data_wasm(world: &mut World, win: Arc<Window>) {
    // 创建 RenderWindow
    let render_window = RenderWindow::new(win, PresentMode::Fifo);
    let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
    render_windows.insert(render_window);
}

pub struct PreFrameTime(pub Share<ShareMutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self { Self(Instant::now()) }
}

impl Default for PreFrameTime {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Instant::now()))) }
}


