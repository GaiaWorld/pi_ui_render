use std::borrow::BorrowMut;

use async_trait::async_trait;
use log::{info, debug};
use pi_async::rt::{single_thread::{SingleTaskRunner, SingleTaskPool}, AsyncRuntime};
use pi_ecs::prelude::{World};
use pi_render::{components::view::{render_window::{RenderWindow, RenderWindows}, target::{TextureViews, RenderTarget, RenderTargets}, target_alloc::ShareTargetView}, rhi::options::RenderOptions, init_render, RenderStage};
use pi_share::ShareRefCell;
use pi_ui_render::{gui::Gui, resource::draw_obj::RenderInfo, system::pass::pass_graph_node::Pass2DNode};
use wgpu::PresentMode;
use winit::{event_loop::{EventLoop, ControlFlow}, window::Window, event::{WindowEvent, Event}};

#[async_trait]
pub trait Example: 'static + Sized {
    async fn init(
		&mut self, 
		gui: &mut Gui, 
		render_stage: RenderStage,
		rt: AsyncRuntime<(), SingleTaskPool<()>>,
		size: (usize, usize),
	);
	fn render(&mut self, gui: &mut Gui);
}

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

	// 初始化运行时
	let runner = SingleTaskRunner::<()>::default();
    let runtime = AsyncRuntime::Local(runner.startup().unwrap());

    let event_loop = EventLoop::new();
    let window = ShareRefCell::new(winit::window::Window::new(&event_loop).unwrap());
	let size = window.inner_size();

    let gui = ShareRefCell::new(Gui::new(runtime.clone()));


	let mut e = ShareRefCell::new(example);
    let rt = runtime.clone();
    let win = window.clone();
	let mut g = gui.clone();
	let e1 = e.clone();

    std::thread::spawn(move || {
        let example = e.clone();
		let gui = g.clone();

        let runtime = runtime.clone();

        let rt = runtime.clone();
        let _ = runtime.spawn(runtime.alloc(), async move {
			let world = g.world_mut();

            let options = RenderOptions::default();
			let render_stages = init_render::<Option<ShareTargetView>, _>(world, options, win.clone(), rt.clone()).await;

			init_data(world, win);
			e.init(g.borrow_mut(), render_stages, rt, (size.width as usize, size.height as usize)).await;
			
        });

		let mut frame = 0;
        loop {
            frame += 1;
            debug!("=================== frame = {}", frame);

            let mut e = example.clone();
			let mut g = gui.clone();
			e.render(g.borrow_mut());

            loop {
                let count = runner.run().unwrap();
                if count == 0 {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    run_window_loop(window, event_loop, e1, rt, gui);
}

fn init_data(world: &mut World, win: ShareRefCell<Window>) {
	// 取 TextureView
	let texture_views = match world.get_resource_mut::<TextureViews>(){
		Some(r) => r,
		None => {
			world.insert_resource(TextureViews::default());
			world.get_resource_mut::<TextureViews>().unwrap()
		}
	};
	let view = texture_views.insert(None);

	// 创建 RenderWindow
	let render_window = RenderWindow::new(win, PresentMode::Mailbox, view);
	let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
	render_windows.insert(render_window);

	// 创建 RenderTarget
	world.insert_resource(RenderTargets::default());
	let mut rt = RenderTarget::default();
	rt.add_color(view);
	let render_targets = world.get_resource_mut::<RenderTargets>().unwrap();
	let rt_key = render_targets.insert(rt);

	world.insert_resource(RenderInfo {
		rt_key
	});
}


fn run_window_loop<T: Example + Sync + Send + 'static>(
    window: ShareRefCell<winit::window::Window>,
    event_loop: EventLoop<()>,
    _example: ShareRefCell<T>,
    rt: AsyncRuntime<(), SingleTaskPool<()>>,
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
