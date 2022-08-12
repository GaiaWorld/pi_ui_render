
use std::{time::Instant, sync::{Arc, Mutex}};

use pi_async::rt::{AsyncRuntimeBuilder, worker_thread::WorkerRuntime, AsyncRuntime};
use pi_ecs::{prelude::{StageBuilder, SingleDispatcher, ResMut, IntoSystem}, world::World};
use pi_render::{rhi::options::RenderOptions, init_render, components::view::{target_alloc::ShareTargetView, render_window::{RenderWindow, RenderWindows}}};
use pi_share::{ShareRefCell, Share};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use wgpu::PresentMode;
use winit::window::Window;

use crate::gui::Gui;

pub mod json_parse;
pub mod native_index;

pub mod style;
// pub mod layout;

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub struct Gui(pub crate::gui::Gui);

// #[cfg(not(target_arch = "wasm32"))]
// pub struct Gui(pub crate::gui::Gui);

pub struct Engine {
	win: Arc<Window>,
	dispatcher: SingleDispatcher<WorkerRuntime>,
	world: World,
	rt: WorkerRuntime,
	gui: crate::gui::Gui,
}

pub fn create_engine(win: &Arc<Window>, r: u32) -> Engine {
	let size = win.inner_size();
	let runtime = AsyncRuntimeBuilder::default_worker_thread(
		None,
		None,
		None,
		None,
	);

	let mut world = World::new();

	let mut world1 = world.clone();
	let mut win1 = win.clone();
	let rt = runtime.clone();

	let mut result: ShareRefCell<Option<(Gui, SingleDispatcher<WorkerRuntime>)>> = ShareRefCell::new(None);
	let result1 = result.clone();

	let _ = runtime.spawn(runtime.alloc(), async move {
		let world = &mut world1;
		let options = RenderOptions::default();
		let render_stages = init_render::<Option<ShareTargetView>, _>(world, options, win1.clone(), rt.clone()).await;

		init_data(world, win1);

		let mut stages = Vec::new();

		let mut first_stage = StageBuilder::new();
		let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
			frame_start_time.0 = Instant::now();
		};
		first_stage.add_node(IntoSystem::system(first_run, world));
		stages.push(Arc::new(first_stage.build(world)));

		// 初始化gui stage
		let mut gui = Gui::new(world);
		let gui_stages = gui.init(0, 0, size.width, size.height);
		for stage in gui_stages.into_iter() {
			stages.push(Arc::new(stage.build(world)));
		}
		stages.push(Arc::new(render_stages.extract_stage.build(world)));
		stages.push(Arc::new(render_stages.prepare_stage.build(world)));
		stages.push(Arc::new(render_stages.render_stage.build(world)));

		let mut dispatcher = SingleDispatcher::new(rt);
		dispatcher.init(stages, world);

		*result1.0.borrow_mut() = Some((gui, dispatcher));
	});
	loop {
		if result.0.borrow().is_some() {
			match Share::try_unwrap(result.0) {
				Ok(r) => {
					let r = r.into_inner().unwrap();
					let engine = Box::new(Engine {
						win: win.clone(),
						dispatcher: r.1,
						world: World::new(),
						gui: r.0,
						rt: runtime.clone(),
					});
					return Box::into_inner(engine);
				}
				Err(r) => result = ShareRefCell(r),
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

pub struct PreFrameTime(pub Arc<Mutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Default for PreFrameTime {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Instant::now())))
    }
}





