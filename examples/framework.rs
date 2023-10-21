use std::{sync::Arc, time::Instant};

use bevy_ecs::prelude::{IntoSystemConfigs, Entity, SystemSet, Local};
use bevy_app::prelude::{ App, Update, Startup };
#[cfg(feature = "debug")]
use bevy_ecs::prelude::{Commands, ResMut, World};
use bevy_ecs::system::SystemState;

use bevy_ecs::system::Resource;
use bevy_window::{Window, WindowResolution};

use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_asset::{AssetConfig, PiAssetPlugin};
// use pi_bevy_ecs_extend::prelude::Root;
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, PiRenderOptions};
use pi_flex_layout::prelude::Size;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_share::{Share, ShareMutex};
// use pi_ui_render::components::user::AsImage;
// use pi_ui_render::system::draw_obj::calc_text::IsRun;
use pi_ui_render::system::{system_set::UiSystemSet, RunState};
use pi_ui_render::{prelude::UiPlugin, resource::UserCommands};

#[cfg(feature = "debug")]
use pi_ui_render::system::cmd_play::{CmdNodeCreate, PlayState, Records};
use pi_winit::event::{Event, WindowEvent};
use pi_winit::event_loop::{EventLoop, ControlFlow};
#[cfg(target_arch = "wasm32")]
use pi_async_rt::rt::serial_local_compatible_wasm_runtime::{LocalTaskRunner, LocalTaskRuntime};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub trait Example: 'static + Sized {
    fn setting(&mut self, _app: &mut App) {}
    fn init(&mut self, world: &mut World, size: (usize, usize));
    fn render(&mut self, commands: &mut UserCommands, cmd1: &mut Commands);

    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        None
    }
    #[cfg(feature = "debug")]
    fn record_option(&self) -> pi_ui_render::system::cmd_play::TraceOption { pi_ui_render::system::cmd_play::TraceOption::None }
    fn play_option(&self) -> Option<PlayOption> { None }
}

#[cfg(target_arch = "wasm32")]
pub static mut RUNNER: std::cell::OnceCell<LocalTaskRunner<()>> = std::cell::OnceCell::new();

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
	#[cfg(not(target_arch = "wasm32"))]
	init_load_cb(Arc::new(|path: String| {
        MULTI_MEDIA_RUNTIME
            .spawn(async move {
                if let Ok(dynamic_image) = std::fs::read(path.clone()) {
                    on_load(path.as_str(), dynamic_image);
                } else {
                    log::warn!("not find image,path: {:?}", path);
                }
            })
            .unwrap();
    }));

	#[cfg(target_arch = "wasm32")]
    init_load_cb(Arc::new(|path: String| {
		log::warn!("load==============={:?}", path);
        MULTI_MEDIA_RUNTIME
            .spawn(async move {
				log::warn!("load1==============={:?}", path);
				// wasm暂时只允许加载这几张资源
				if path.as_str() == "examples/z_source/bx_lanseguanbi.s3tc.ktx" {
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/bx_lanseguanbi.s3tc.ktx")[..]));
				} else if path.as_str() == "examples/z_source/3675173.jpg" {
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/3675173.jpg")[..]));
				} else if path.as_str() == "examples/z_source/bx_lanseguanbi.png" {
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/bx_lanseguanbi.png")[..]));
				} else if path.as_str() == "examples/z_source/dialog_bg.png" {
					log::warn!("dialog_bg.png is load success");
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/dialog_bg.png")[..]));
				}
            })
            .unwrap();
    }));

    let size = example.get_init_size();
    // let mut window_plugin = bevy_window::WindowPlugin::default();
    let (width, height) = if let Some(size) = size {
        // window_plugin.window.width = size.width as f32;
        // window_plugin.window.height = size.height as f32;
        (size.width, size.height)
    } else {
        (450, 720)
    };

    #[cfg(feature = "debug")]
    let record_option = example.record_option();
	#[cfg(feature = "debug")]
	let play_option = example.play_option();
    let exmple = Share::new(ShareMutex::new(example));
    let exmple1 = exmple.clone();
    let exmple_run = move |world: &mut World, commands: &mut SystemState<(ResMut<UserCommands>, Commands)>| {
        // log::warn!("zzzzzzzzzzzzzzzzzzzzzzzzbbbbbb");
        let mut commands = commands.get_mut(world);
        exmple.lock().render(&mut commands.0, &mut commands.1);
    };

	let event_loop = EventLoop::new();
	#[cfg(not(target_arch = "wasm32"))]
	let window = Arc::new(pi_winit::window::Window::new(&event_loop).unwrap());
	#[cfg(target_arch = "wasm32")]
	let (window, canvas) = {
		use pi_winit::platform::web::WindowBuilderExtWebSys;
		use wasm_bindgen::JsCast;
		let canvas: wasm_bindgen::JsValue = web_sys::window().unwrap().document().unwrap().create_element("canvas").unwrap().into();
		let canvas: web_sys::HtmlCanvasElement = canvas.into();
		(
			Arc::new(
				pi_winit::window::WindowBuilder::new()
					.with_canvas(Some(canvas.clone()))
					.build(&event_loop)
					.unwrap(),
			),
			canvas
		)
	};

	#[cfg(target_arch = "wasm32")]
    {
		// 将window中的canvas添加到dom树中
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(canvas))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

			// 初始化运行时（全局localRuntime需要初始化）
			let runner = LocalTaskRunner::new();
			let rt = runner.get_runtime();
			//非线程安全，外部保证同一时间只有一个线程在多读或单写变量
			unsafe {
				RUNNER.set(runner);
				pi_hal::runtime::MULTI_MEDIA_RUNTIME.0.set(rt.clone());
				pi_hal::runtime::RENDER_RUNTIME.0.set(rt);
			}
    }

    let mut app = init(width, height, &event_loop, window.clone());

    app.world.insert_resource(RunState::RENDER);
	#[cfg(feature = "debug")]
	if let Some(play_option) = play_option {
		app.world.insert_resource(play_option);
	}

    #[cfg(feature = "debug")]
    app.add_plugins(UiPlugin { cmd_trace: record_option });
    #[cfg(not(feature = "debug"))]
    app.add_plugins(UiPlugin::default());
	exmple1.lock().setting(&mut app);

    app.add_systems(Update, exmple_run.before(UiSystemSet::Setting).in_set(ExampleSet))
        .add_systems(Startup, move |world: &mut World| {
            exmple1.lock().init(world, (width as usize, height as usize));
        });

    #[cfg(feature = "debug")]
    match record_option {
        pi_ui_render::system::cmd_play::TraceOption::None => (),
        pi_ui_render::system::cmd_play::TraceOption::Record => {
            app.add_systems(Update, record_cmd_to_file.after(UiSystemSet::Setting));
        }
        pi_ui_render::system::cmd_play::TraceOption::Play => {
            app.add_systems(Update, setting_next_record.before(UiSystemSet::Setting));
        }
    }

	event_loop.run(move |event, _, control_flow| {

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
			Event::RedrawRequested(_) => {
				#[cfg(not(target_arch = "wasm32"))]
                app.update();

				#[cfg(target_arch = "wasm32")] 
				{
					// 资源运行时
					let rt = unsafe{RUNNER.get().unwrap()};
					while pi_hal::runtime::RENDER_RUNTIME.len() > 0 {
						rt.poll();
						rt.run_once();
					}
					app.update();
				}
				
            }
			Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });

	// let mut i = 0;
	// loop {
	// 	// event_loop
	// 	app.update();
	// 	i += 1;
		
	// 	std::thread::sleep(std::time::Duration::from_millis(16));
	// 	// if i == 1 {
	// 	// 	break;
	// 	// }
	// }
	// loop {
		
	// }
	// app.update();
	// let mut v = Vec::with_capacity(10);
	// for _i in 0..10 {
	// 	let t = std::time::Instant::now();
	// 	// log::warn!("zzz================");
	// 	app.update();
	// 	v.push(std::time::Instant::now() - t);
	// }
	// log::warn!("time: {:?}", v);


	// let mut criterion = criterion::Criterion::default();
	// criterion::__warn_about_html_reports_feature();
	// criterion::__warn_about_cargo_bench_support_feature();

	// let mut group = criterion.benchmark_group("app_update");
	// group.warm_up_time(std::time::Duration::from_millis(500));
	// group.measurement_time(std::time::Duration::from_secs(3));
	// group.bench_function("update", |bencher| {
	// 	bencher.iter(|| {
	// 		app.update();
	// 	});
	// });
	// group.finish();

	// criterion::Criterion::default()
	// 	.configure_from_args()
	// 	.final_summary();


	// log::warn!("end==================");
	// criterion::criterion_group!(
	// 	benchmarks,
	// 	// bench_simple_insert,
	// 	// bench_simple_iter,
	// 	// bench_frag_iter_bc,
	// 	// bench_event_deal,
	// 	bench_login_setting,
	// 	// bench_schedule,
	// 	// bench_heavy_compute,
	// 	// bench_add_remove,
	// 	// bench_serialize_text,
	// 	// bench_serialize_binary,
	// );
	// criterion::criterion_main!(benchmarks);


    // app.run();

    // let system_schedule = bevy_mod_debugdump::get_schedule(&mut app);
    // let mut file = File::create("system_schedule.dot").unwrap();
    // file.write_all(system_schedule.as_bytes()).unwrap();

    // bevy_mod_debugdump::print_schedule(&mut app);

    // run_window_loop(window, event_loop);
}
#[derive(Debug, Clone, Hash, SystemSet, PartialEq, Eq)]
pub struct ExampleSet;

pub struct PreFrameTime(pub Arc<ShareMutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self { Self(Instant::now()) }
}

impl Default for PreFrameTime {
    fn default() -> Self { Self(Arc::new(ShareMutex::new(Instant::now()))) }
}

#[allow(dead_code)]
fn main() {}

pub fn init(width: u32, height: u32, _event_loop: &EventLoop<()>, w: Arc<pi_winit::window::Window>) -> App {
    let mut app = App::default();

    // let event_loop =  EventLoopBuilder::new().with_any_thread(true).build();
    // let window = winit::window::Window::new(&event_loop).unwrap();
    // window.set_inner_size(PhysicalSize {width, height});
    let mut window = Window::default();
    window.resolution = WindowResolution::new(width as f32, height as f32);
	println!("window========={:?}, {:?}", width, height);
    let mut window_plugin = bevy_window::WindowPlugin::default();
    // window_plugin.primary_window = Some(window);
	window_plugin.primary_window = None;

	let mut o = PiRenderOptions::default();
	o.present_mode = wgpu::PresentMode::Mailbox;
	app.world.insert_resource(o);

	// app.world.insert_resource(IsRun(true));


    app.add_plugins(pi_bevy_log::LogPlugin::<Vec<u8>> {
        filter: FILTER.to_string(),
        level: LOG_LEVEL,
		chrome_write: None,
    })
    .add_plugins(bevy_a11y::AccessibilityPlugin)
    // .add_plugins(bevy_input::InputPlugin::default())
    .add_plugins(window_plugin)
    // .add_plugins(WinitPlugin::default())
	.add_plugins(pi_bevy_winit_window::WinitPlugin::new(w).with_size(width, height))
    .add_plugins(PiAssetPlugin {
        total_capacity: 1024 * 1024 * 1024,
        asset_config: AssetConfig::default(),
    })
    // .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(PiRenderPlugin::default())
    .add_plugins(PiPostProcessPlugin);

	


    // let h = app.world.get_resource_mut::<pi_bevy_log::LogFilterHandle>().unwrap();
    // let default_filter = { format!("{},my_target=info", bevy_log::Level::WARN) };
    // let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
    // 	.or_else(|_| tracing_subscriber::EnvFilter::try_new(&default_filter))
    // 	.unwrap();
    // h.0.modify(|filter| *filter = filter_layer);
    // log::info!("aaa=============");
    // log::info!(target: "my_target", "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    // log::log!(target: "xxxx", log::Level::Info, a="bbbbbbbb====",);
    app
}

// // 创建root时间，设置AsImage为force
// // 实际运行时不需要这样做，应该按需设置AsImage
// pub fn root_calc(mut q: Query<&mut AsImage, Added<Root>>) {
//     for mut i in q.iter_mut() {
//         i.0 = pi_style::style::AsImage::Force;
//     }
// }

#[cfg(feature = "debug")]
pub struct NextState {
    file_index: usize,
    // play_path: &'static str,
    cmd_path: Option<&'static str>,
    is_end: bool,
}

#[cfg(feature = "debug")]
impl Default for NextState {
    fn default() -> Self {
        NextState {
            file_index: 0,
            // play_version: "performance",
            // play_version: "test",
            cmd_path: Some("D://0_rust/pi_ui_render_new/examples/a_cmd_play/source/cmds"),
            // play_path: "D://0_js/cdqxz_new_mult_gui_exe/dst",
            // play_path: "D://0_js/cdqxz_new_gui_exe/dst",
            // cmd_path: Some("D://0_rust/pi_export/crates/gui/examples/cmd_play/source/cmds"),
            is_end: false,
        }
    }
}

// 将record写入文件
#[cfg(feature = "debug")]
pub fn record_cmd_to_file(mut records: ResMut<Records>) {
    use std::path::Path;
    if records.list.len() == 0 && records.run_state.len() == 0 {
        return;
    }
    let r = match postcard::to_stdvec(&*records) {
        Ok(bin) => bin,
        Err(r) => {
            log::error!("serialize fail!!, {:?}", r);
            Vec::<u8>::default()
        }
    };
    // log::warn!("record============={:?}", &*records);
    std::fs::write(Path::new("examples/a_cmd_play/source/cmds/").join("cmd_local_0.gui_cmd"), r).unwrap();
    records.clear()
}

// 设置下一条记录
#[cfg(feature = "debug")]
pub fn setting_next_record(world: &mut World, mut local_state: Local<NextState>) {
	let play_option  = world.get_resource::<PlayOption>().unwrap().clone();
    let local_state = &mut *local_state;
    setting(local_state.cmd_path, &mut local_state.file_index, world, &mut local_state.is_end, &play_option)
}


#[cfg(feature = "debug")]
fn setting(cmd_path: Option<&str>, file_index1: &mut usize, world: &mut World, is_end: &mut bool, play_option: &PlayOption) {
    let mut file_index = *file_index1;
    let play_state = world.get_resource::<PlayState>();
    if let Some(r) = play_state {
        if r.is_running {
            return;
        } else {
            let dir = match cmd_path {
                Some(r) => r.to_string(),
                None => "examples/a_cmd_play/source".to_string(),
            };
            let path = dir + "/cmd_" + play_option.play_version + "_" + file_index.to_string().as_str() + ".gui_cmd";
            // log::warn!("r================{:?}", path);
            let _span = tracing::warn_span!("gui_cmd").entered();
            match std::fs::read(path.clone()) {
                Ok(bin) => {
                    match postcard::from_bytes::<Records>(&bin) {
                        Ok(r) => {
                            // log::warn!("r================{:?}", r);
                            world.insert_resource(r);
                            // 重设播放状态
                            let mut play_state = world.get_resource_mut::<PlayState>().unwrap();
                            play_state.is_running = true;
                            play_state.next_reord_index = 0;
                            play_state.next_state_index = 0;
                            play_state.cur_frame_count = 0;
                        }
                        Err(e) => {
                            *is_end = true;
                            log::warn!("parse fail================{:?}, {:?}", e, bin.len());
                        }
                    }
                    file_index += 1;
                    *file_index1 = file_index;
                }
                Err(_) => {
                    if !*is_end {
                        log::warn!("play end, {:?}", path);
                    }
                    *is_end = true;
                    return;
                }
            };
        }
    }
    return;
}

#[allow(dead_code)]
pub fn spawn(world: &mut World) -> Entity {
    let r = world.spawn_empty().id();
    #[cfg(feature = "debug")]
    {
        let creates = world.get_resource_mut::<CmdNodeCreate>();
        if let Some(mut creates) = creates {
            creates.0.push(r)
        } else {
            world.insert_resource(CmdNodeCreate(vec![r]));
        }
        // gui.node_cmd.0.push(entity);
    }
    r
}

#[derive(Resource, Clone)]
pub struct PlayOption {
	pub play_path: Option<&'static str>,
	pub play_version: &'static str,
}

// #[allow(dead_code)]
// #[cfg(feature = "debug")]
// // pub const PLAY_PATH: Option<&'static str> = None;
// pub const PLAY_PATH: Option<&'static str> = Some("D://0_js/cdqxz_new_mult_gui_exe/dst");
// // pub const PLAY_PATH: Option<&'static str> = Some("D://0_js/pi_demo_mult_gui/dst");
// #[cfg(feature = "debug")]
// // pub const PLAY_VERSION: &'static str = "local";
// pub const PLAY_VERSION: &'static str = "test";

// pi_flex_layout=trace
// pub const FILTER: &'static str = "wgpu=warn,naga=warn,pi_ui_render::components::user=debug";
// pub const FILTER: &'static str = "wgpu=warn,entity_3v0=trace";
// pub const FILTER: &'static str = "wgpu=warn,pi_ui_render::system::pass::pass_graph_node=trace,pi_ui_render::system::pass_effect::radial_wave=trace,pi_ui_render::system::pass::pass_life=trace";
// pub const FILTER: &'static str = "wgpu=warn,pi_ui_render::system::pass_effect::radial_wave=trace,pi_ui_render::system::pass::pass_life=trace,pi_ui_render::system::pass::update_graph=trace";
// pub const FILTER: &'static str = "wgpu=warn,naga=warn,bevy_app=warn";
pub const FILTER: &'static str = "wgpu=warn,naga=warn,pi_wgpu=trace";
// pub const FILTER: &'static str = "";
pub const LOG_LEVEL: bevy_log::Level = bevy_log::Level::WARN;
// pub const LOG_LEVEL: bevy_log::Level = bevy_log::Level::INFO;
