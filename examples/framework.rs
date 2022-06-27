use std::sync::Arc;

use async_trait::async_trait;
use log::{info, debug};
use pi_async::rt::{AsyncRuntime, AsyncRuntimeBuilder};
use pi_ecs::prelude::{World, SingleDispatcher, Dispatcher};
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
	event::{WindowEvent, Event}
};

#[async_trait]
pub trait Example: 'static + Sized {
    async fn init(
		&mut self, 
		gui: &mut Gui, 
		size: (usize, usize),
	);
	fn render(&mut self, gui: &mut Gui);
}

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

	// 初始化运行时
	let runtime = AsyncRuntimeBuilder::default_worker_thread(
		None,
		None,
		None,
		None,
	);

    let event_loop = EventLoop::new();
    let window = ShareRefCell::new(winit::window::Window::new(&event_loop).unwrap());
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
		let rt1 = runtime.clone();
        let _ = runtime.spawn(runtime.alloc(), async move {
			let world = g.world_mut();

            let options = RenderOptions::default();
			let render_stages = init_render::<Option<ShareTargetView>, _>(world, options, win.clone(), rt.clone()).await;

			init_data(world, win);

			// 初始化gui stage
			let gui_stages = gui.0.borrow_mut().init(0, 0, size.width, size.height);
			let mut stages = Vec::new();
			for stage in gui_stages.into_iter() {
				stages.push(Arc::new(stage.build(world)));
			}
			stages.push(Arc::new(render_stages.extract_stage.build(world)));
			stages.push(Arc::new(render_stages.prepare_stage.build(world)));
			stages.push(Arc::new(render_stages.render_stage.build(world)));
			d.0.borrow_mut().init(stages, world);

			e.init(&mut g.0.borrow_mut(), (size.width as usize, size.height as usize)).await;

			std::thread::spawn(move || {
				let mut frame = 0;
				loop {
					frame += 1;
					debug!("=================== frame = {}", frame);

					let mut e = example.clone();
					e.render(&mut gui1.0.borrow_mut());

					// let mut dispatcher = dispatcher.clone();
					dispatcher.0.borrow().run();

					loop {
						let count = rt1.len();
						if count == 0 {
							break;
						}
					}
					std::thread::sleep(std::time::Duration::from_millis(16));
				}
			});

        });

		// loop {
		// 	let count = rt1.len();
		// 	if count == 0 {
		// 		break;
		// 	}
		// }

		// let mut frame = 0;
		// loop {
		// 	frame += 1;
		// 	debug!("=================== frame = {}", frame);

		// 	let mut e = example.clone();
		// 	e.render(&mut gui1.0.borrow_mut());

		// 	// let mut dispatcher = dispatcher.clone();
		// 	dispatcher.0.borrow().run();

		// 	loop {
		// 		let count = rt1.len();
		// 		if count == 0 {
		// 			break;
		// 		}
		// 	}
		// 	std::thread::sleep(std::time::Duration::from_millis(16));
		// }
    });

    run_window_loop(window, event_loop, e1, rt, gui);

}

fn init_data(world: &mut World, win: ShareRefCell<Window>) {
	// 创建 RenderWindow
	let render_window = RenderWindow::new(win, PresentMode::Mailbox);
	let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
	render_windows.insert(render_window);

}


fn run_window_loop<T: Example + Sync + Send + 'static, A: AsyncRuntime>(
    window: ShareRefCell<winit::window::Window>,
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
