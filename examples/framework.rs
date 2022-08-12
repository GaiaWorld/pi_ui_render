use std::{sync::{Arc, Mutex}, time::{Instant, Duration}, env};

use async_trait::async_trait;
use log::info;
use pi_async::rt::{AsyncRuntime, multi_thread::{MultiTaskRuntime, StealableTaskPool, MultiTaskRuntimeBuilder}, AsyncRuntimeBuilder};
use pi_ecs::prelude::{World, SingleDispatcher, Dispatcher, Local, StageBuilder, IntoSystem, ResMut, Res};
use pi_flex_layout::prelude::Size;
use pi_hal::{image::{init_image_cb, on_load}, runtime::MULTI_MEDIA_RUNTIME};
use pi_render::{
	components::view::{
		render_window::{RenderWindow, RenderWindows}, 
		target_alloc::ShareTargetView
	}, 
	rhi::options::RenderOptions, init_render
};
use pi_share::ShareRefCell;
use pi_ui_render::gui::Gui;
use wgpu::PresentMode;
use winit::{
	event_loop::{EventLoop, ControlFlow}, 
	window::Window, 
	event::{WindowEvent, Event}, dpi::PhysicalSize
};

#[async_trait]
pub trait Example: 'static + Sized {
    async fn init(
		&mut self, 
		gui: &mut Gui, 
		size: (usize, usize),
	);
	fn render(&mut self, gui: &mut Gui);

	fn get_init_size(&self) -> Option<Size<u32>> {
		// None表示使用默认值
		None
	}
}

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
	// env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

	// 单线程运行时
	// let runtime = AsyncRuntimeBuilder::default_worker_thread(
	// 	None,
	// 	None,
	// 	None,
	// 	None,
	// );

	let runtime: MultiTaskRuntime<()> = {
        let count = match env::var("_ver") {
            Ok(r) => usize::from_str_radix(r.as_str(), 10).unwrap(),
            _ => num_cpus::get()
        };
        let pool = StealableTaskPool::with(count, count);
        // 线程池：每个线程1M的栈空间，10ms 休眠，10毫秒的定时器间隔
        let builder = MultiTaskRuntimeBuilder::new(pool).init_worker_size(count).set_worker_limit(count, count);
        builder.build()
    };

	init_image_cb(Arc::new(|path: String| {
		MULTI_MEDIA_RUNTIME.spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
			if let Ok(dynamic_image) = image::open(path.clone()) {
				on_load(path.as_str(), dynamic_image);
			}
		}).unwrap();
	}));

    let event_loop = EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

	if let Some(init_size) = example.get_init_size() {
		window.set_inner_size(PhysicalSize {
			width: init_size.width,
			height: init_size.height,
		});
	}
	
	let size = window.inner_size();

	let mut world = World::new();
    let gui = ShareRefCell::new(Gui::new(&mut world));

	let dispatcher = ShareRefCell::new(SingleDispatcher::new(runtime.clone()));

	let mut e = ShareRefCell::new(example);
    let rt = runtime.clone();
    let win = window.clone();
	let mut g = gui.clone();
	let d = dispatcher.clone();
	let e1 = e.clone();

    std::thread::spawn(move || {
        let example = e.clone();
		let gui = g.clone();
		let gui1 = g.clone();

        let runtime = runtime.clone();

        let rt = runtime.clone();
        let _ = runtime.spawn(runtime.alloc(), async move {
			let world = g.world_mut();

            let options = RenderOptions::default();
			let render_stages = init_render::<Option<ShareTargetView>, _>(world, options, win.clone(), rt.clone()).await;

			init_data(world, win);

			let mut stages = Vec::new();

			let mut first_stage = StageBuilder::new();
			let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
				frame_start_time.0 = Instant::now();
			};
			first_stage.add_node(IntoSystem::system(first_run, world));
			stages.push(Arc::new(first_stage.build(world)));

			// 初始化gui stage
			let gui_stages = gui.0.borrow_mut().init(0, 0, size.width, size.height);
			for stage in gui_stages.into_iter() {
				stages.push(Arc::new(stage.build(world)));
			}
			stages.push(Arc::new(render_stages.extract_stage.build(world)));
			stages.push(Arc::new(render_stages.prepare_stage.build(world)));
			stages.push(Arc::new(render_stages.render_stage.build(world)));

			let mut last_stage = StageBuilder::new();
			let rt = rt.clone();
			let last_run = move |pre_frame_time: Local<PreFrameTime>, frame_start_time: Res<FrameStartTime>| {
				let use_time = Instant::now() - frame_start_time.0;
				let pre_frame_time1 = pre_frame_time.0.clone();
				
				let mut example = example.clone();
				let dispatcher = dispatcher.clone();
				let gui1 = gui1.clone();
				
				rt.spawn(rt.alloc(), async move {
					let duration = {
						let time = Instant::now();
						let time1 = pre_frame_time1.lock().unwrap();

						if time > *time1 {
							let d = time - *time1;
							if d > Duration::from_millis(16) {
								Duration::from_millis(0)
							} else {
								Duration::from_millis(16) - d
							}
						} else {
							Duration::from_millis(0)
						}
					};
					spin_sleep::sleep(duration);
					// log::warn!("frame time=============duration: {:?}, preframe_use: {:?},  sleep: {:?}", Instant::now() - *pre_frame_time1.lock().unwrap(), use_time, duration);
					*pre_frame_time1.lock().unwrap() = Instant::now();
					example.render(&mut gui1.0.borrow_mut());
					dispatcher.0.borrow().run();
				}).unwrap();
			};
			last_stage.add_node(IntoSystem::system(last_run, world));
			stages.push(Arc::new(last_stage.build(world)));

			d.0.borrow_mut().init(stages, world);

			e.init(&mut g.0.borrow_mut(), (size.width as usize, size.height as usize)).await;

			// 首次运行
			e.render(&mut g.0.borrow_mut());
			d.0.borrow_mut().run();
        });
    });

    run_window_loop(window, event_loop, e1, rt, gui);

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

fn init_data(world: &mut World, win: Arc<Window>) {
	// 创建 RenderWindow
	let render_window = RenderWindow::new(win, PresentMode::Mailbox);
	let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
	render_windows.insert(render_window);
}

fn run_window_loop<T: Example + Sync + Send + 'static, A: AsyncRuntime>(
    window: Arc<winit::window::Window>,
    event_loop: EventLoop<()>,
    _example: ShareRefCell<T>,
    rt: A,
	_gui: ShareRefCell<Gui>,
) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // let w = size.width;
                // let h = size.height;
                // let e = example.clone();
                let _ = rt.spawn(rt.alloc(), async move {
                    info!("RenderExample::resize, size = {:?}", size);
                    // e.resize(w, h);
                });
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
				// let mut e = example.clone();
				// let mut g = gui.clone();
				// e.render(g.borrow_mut());
	
				// loop {
				// 	let count = runner.run().unwrap();
				// 	if count == 0 {
				// 		break;
				// 	}
				// }
				// std::thread::sleep(std::time::Duration::from_millis(16));
            }
            Event::WindowEvent {
                // 窗口 关闭，退出 循环
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("RenderExample::clean");
                // let e = example.clone();
                let _ = rt.spawn(rt.alloc(), async move {
                    // e.clean();
                });

                *control_flow = ControlFlow::Exit
            }
            _ => {}
        }
    });
}

#[allow(dead_code)]
fn main() {

}
