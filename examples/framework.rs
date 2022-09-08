use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use async_trait::async_trait;
use log::info;
use pi_async::rt::{AsyncRuntime, AsyncRuntimeBuilder};
use pi_ecs::prelude::{Dispatcher, IntoSystem, Local, Res, ResMut, SingleDispatcher, StageBuilder, World};
use pi_flex_layout::prelude::Size;
use pi_hal::{
    image::{init_image_cb, on_load},
    runtime::MULTI_MEDIA_RUNTIME,
};
use pi_render::{
    components::view::{
        render_window::{RenderWindow, RenderWindows},
        target_alloc::ShareTargetView,
    },
    init_render,
    rhi::options::RenderOptions,
};
use pi_share::ShareRefCell;
use pi_ui_render::{export::Engine, gui::Gui, resource::TimeInfo};
use wgpu::PresentMode;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
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

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
    env_logger::Builder::default()
        // .filter(Some("wgpu_core"), log::LevelFilter::Warn)
        // .filter(Some("wgpu_hal"), log::LevelFilter::Warn)
        // .filter(Some("pi_graph"), log::LevelFilter::Warn)
        .filter(None, log::LevelFilter::Warn)
        // .filter(Some("pi_ui_render"), log::LevelFilter::Trace)
        // .filter(Some("pi_animation"), log::LevelFilter::Trace)
        // .filter(Some("pi_curves"), log::LevelFilter::Trace)
        // .filter(Some("pi_style"), log::LevelFilter::Trace)
        .filter(Some("pi_ui_render"), log::LevelFilter::Info)
        .init();
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // 单线程运行时
    let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);
    let rt = runtime.clone();
    // let runtime: MultiTaskRuntime<()> = {
    //     let count = match env::var("_ver") {
    //         Ok(r) => usize::from_str_radix(r.as_str(), 10).unwrap(),
    //         _ => num_cpus::get()
    //     };
    //     let pool = StealableTaskPool::with(count, count);
    //     // 线程池：每个线程1M的栈空间，10ms 休眠，10毫秒的定时器间隔
    //     let builder = MultiTaskRuntimeBuilder::new(pool).init_worker_size(count).set_worker_limit(count, count);
    //     builder.build()
    // };

    let event_loop = EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());


    let dispatcher = SingleDispatcher::new(runtime.clone());
    let mut world = World::new();
    let gui = Gui::new(&mut world);

    let engine = ShareRefCell::new(Engine {
        win: window.clone(),
        dispatcher: dispatcher,
        world,
        rt: runtime,
        gui,
    });


    init_image_cb(Arc::new(|path: String| {
        MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                if let Ok(dynamic_image) = image::open(path.clone()) {
                    on_load(path.as_str(), dynamic_image);
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

    // let gui = ShareRefCell::new(Gui::new(&mut world));

    // let rt = runtime.clone();
    // let win = window.clone();
    // let mut g = gui.clone();
    // let e1 = e.clone();

    let mut e = ShareRefCell::new(example);

    let engine1 = engine.clone();

    std::thread::spawn(move || {
        let example = e.clone();
        let _ = rt.spawn(rt.alloc(), async move {
            let engine = engine1.clone();

            let mut engine_ref = engine1.0.borrow_mut();
            let (win, rt) = (engine_ref.win.clone(), engine_ref.rt.clone());

            let options = RenderOptions::default();
            let render_stages = init_render::<Option<ShareTargetView>, _>(&mut engine_ref.world, options, win.clone(), rt.clone()).await;

            init_data(&mut engine_ref.world, win.clone());

            let mut stages = Vec::new();

            let mut first_stage = StageBuilder::new();
            let first_run = move |mut frame_start_time: ResMut<FrameStartTime>, mut cur_time: ResMut<TimeInfo>| {
                let now = Instant::now();
                *cur_time = TimeInfo {
                    cur_time: now,
                    delta: (now - frame_start_time.0).as_millis() as u64,
                };
                frame_start_time.0 = now;
            };
            first_stage.add_node(IntoSystem::system(first_run, &mut engine_ref.world));
            stages.push(Arc::new(first_stage.build(&mut engine_ref.world)));

            // 初始化gui stage
            let gui_stages = engine_ref.gui.init(0, 0, size.width, size.height);
            for stage in gui_stages.into_iter() {
                stages.push(Arc::new(stage.build(&mut engine_ref.world)));
            }
            stages.push(Arc::new(render_stages.extract_stage.build(&mut engine_ref.world)));
            stages.push(Arc::new(render_stages.prepare_stage.build(&mut engine_ref.world)));
            stages.push(Arc::new(render_stages.render_stage.build(&mut engine_ref.world)));

            let mut last_stage = StageBuilder::new();

            let last_run = move |pre_frame_time: Local<PreFrameTime>, frame_start_time: Res<FrameStartTime>| {
                let _use_time = Instant::now() - frame_start_time.0;
                let pre_frame_time1 = pre_frame_time.0.clone();

                let mut example = example.clone();
                let engine1 = engine.clone();

                let engine_ref = engine.borrow();
                engine_ref
                    .rt
                    .spawn(engine_ref.rt.alloc(), async move {
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
                        let mut engine_ref = engine1.0.borrow_mut();
                        example.render(&mut engine_ref);
                        engine_ref.dispatcher.run();
                    })
                    .unwrap();
            };
            last_stage.add_node(IntoSystem::system(last_run, &mut engine_ref.world));
            stages.push(Arc::new(last_stage.build(&mut engine_ref.world)));

            let mut world = engine_ref.world.clone();
            engine_ref.dispatcher.init(stages, &mut world);

            e.init(&mut engine_ref, (size.width as usize, size.height as usize)).await;

            // 首次运行
            e.render(&mut engine_ref);
            engine_ref.dispatcher.run();
        });
    });

    run_window_loop(window, event_loop);
}

pub struct PreFrameTime(pub Arc<Mutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self { Self(Instant::now()) }
}

impl Default for PreFrameTime {
    fn default() -> Self { Self(Arc::new(Mutex::new(Instant::now()))) }
}

fn init_data(world: &mut World, win: Arc<Window>) {
    // 创建 RenderWindow
    let render_window = RenderWindow::new(win, PresentMode::Mailbox);
    let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
    render_windows.insert(render_window);
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
