use std::{
    sync::{Arc},
    time::{Duration, Instant}
};

use async_trait::async_trait;
use log::info;
use pi_async::prelude::AsyncRuntime;
use pi_flex_layout::prelude::Size;
use pi_hal::{
    image::{init_image_cb, on_load},
    runtime::MULTI_MEDIA_RUNTIME,
};
use pi_share::ShareMutex;
use pi_ui_render::{export::{Engine, create_engine}};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

#[async_trait]
pub trait Example: 'static + Sized {
    async fn init(&mut self, engine: &mut Engine, size: (usize, usize));
    fn render(&mut self, gui: &mut Engine);

    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        None
    }
}

pub fn start<T: Example + Sync + Send + 'static>(mut example: T) {
    env_logger::Builder::default()
        // .filter(Some("wgpu_core"), log::LevelFilter::Warn)
        // .filter(Some("wgpu_hal"), log::LevelFilter::Warn)
        // .filter(Some("pi_graph"), log::LevelFilter::Warn)
        .filter(None, log::LevelFilter::Warn)
        // .filter(Some("pi_ui_render"), log::LevelFilter::Trace)
        // .filter(Some("pi_animation"), log::LevelFilter::Trace)
        // .filter(Some("pi_curves"), log::LevelFilter::Trace)
		// .filter(Some("pi_flex_layout"), log::LevelFilter::Trace)
		// .filter(Some("pi_style::style_type"), log::LevelFilter::Trace)
		// .filter(Some("pi_ui_render::components::user"), log::LevelFilter::Trace)
		.filter(Some("pi_hal"), log::LevelFilter::Trace)
		.filter(Some("pi_render"), log::LevelFilter::Trace)
        .filter(Some("pi_ui_render"), log::LevelFilter::Info)
		.filter(Some("pi_ui_ecs"), log::LevelFilter::Info)
        .init();

    let event_loop = EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

    init_image_cb(Arc::new(|path: String| {
        MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                if let Ok(dynamic_image) = image::open(path.clone()) {
                    on_load(path.as_str(), dynamic_image);
                } else {
					log::warn!("not find image,path: {:?}", path);
				}
            })
            .unwrap();
    }));


    if let Some(init_size) = example.get_init_size() {
        window.set_inner_size(PhysicalSize {
            width: init_size.width,
            height: init_size.height,
        });
    }

    let size = window.inner_size();

	// let dispatcher_mgr = DispatcherMgr::default();
    // let mut world = World::new();
    // let gui = Gui::new(&mut world);

    // let engine = ShareRefCell::new(Engine {
    //     win: window.clone(),
	// 	dispatcher_mgr: dispatcher_mgr,
    //     render_dispatcher: DefaultKey::null(),
    //     world,
    //     rt: runtime,
    //     gui,
    // });
	let mut engine = create_engine(&window, 1.0);

    // let engine1 = engine.clone();
	MULTI_MEDIA_RUNTIME.spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
		example.init(&mut engine, (size.width as usize, size.height as usize)).await;

		let mut pre_frame_time = Instant::now();
		loop {
			
			// 运行
			let render_key = engine.render_dispatcher;
			example.render(&mut engine);
			engine.dispatcher_mgr.run(render_key, true).await;

			let time = Instant::now();
			// let _use_time = Instant::now() - pre_frame_time;
			let time1 = pre_frame_time.clone();

			if time > time1 {
				let d = time - time1;
				let duration = if d > Duration::from_millis(16) {
					Duration::from_millis(0)
				} else {
					Duration::from_millis(16) - d
				};
				spin_sleep::sleep(duration);
			}
			pre_frame_time = time;
		}
	}).unwrap();

    run_window_loop(window, event_loop);
}

pub struct PreFrameTime(pub Arc<ShareMutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self { Self(Instant::now()) }
}

impl Default for PreFrameTime {
    fn default() -> Self { Self(Arc::new(ShareMutex::new(Instant::now()))) }
}

fn run_window_loop(window: Arc<winit::window::Window>, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // let w = size.width;
                // let h = size.height;
                // let e = example.clone();
                // let _ = rt.spawn(rt.alloc(), async move {
                //     info!("RenderExample::resize, size = {:?}", size);
                //     // e.resize(w, h);
                // });
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
                // let _ = rt.spawn(rt.alloc(), async move {
                //     // e.clean();
                // });

                *control_flow = ControlFlow::Exit
            }
            _ => {}
        }
    });
}

#[allow(dead_code)]
fn main() {}
