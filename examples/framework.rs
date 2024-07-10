
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{sync::Arc, time::Instant};

use pi_bevy_ecs_extend::system_param::res::OrInitSingleResMut;
use pi_ui_render::resource::fragment::NodeTag;
use pi_ui_render::resource::ShareFontSheet;
use pi_world::prelude::{Entity, SystemSet, Local, App, SingleResMut, World, WorldPluginExtent, IntoSystemConfigs, First, Insert};
use bevy_window::{Window, WindowResolution};

use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_asset::{AssetConfig, PiAssetPlugin};
// use pi_pi_world_extend::prelude::Root;
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, PiRenderOptions};
use pi_flex_layout::prelude::Size;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME, Arg};
use pi_share::{Share, ShareMutex};
use pi_hal::font::font::FontType;
use pi_ui_render::system::RunState;
// use pi_ui_render::components::user::AsImage;
// use pi_ui_render::system::draw_obj::calc_text::IsRun;
use pi_ui_render::system::system_set::UiSystemSet;
use pi_ui_render::{prelude::{UiPlugin, UiStage}, resource::UserCommands};

#[cfg(feature = "debug")]
use pi_ui_render::system::cmd_play::{CmdNodeCreate, PlayState, Records};
use pi_winit::event::{Event, WindowEvent};
use pi_winit::event_loop::{EventLoop, ControlFlow};
#[cfg(target_arch = "wasm32")]
use pi_async_rt::rt::serial_local_compatible_wasm_runtime::{LocalTaskRunner, LocalTaskRuntime};
use pi_world::single_res::SingleRes;
use pi_world::system_params::SystemParam;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub trait Example: 'static + Sized {
    fn setting(&mut self, _app: &mut App) {}
    fn init(&mut self, param: Param, size: (usize, usize));
    fn render(&mut self, commands: &mut UserCommands);

    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        None
    }
	fn font_type(&self) -> FontType {
        FontType::Sdf2
    }
    #[cfg(feature = "debug")]
    fn record_option(&self) -> pi_ui_render::system::cmd_play::TraceOption { pi_ui_render::system::cmd_play::TraceOption::None }
    fn play_option(&self) -> Option<PlayOption> { None }
}

#[cfg(target_arch = "wasm32")]
pub static mut RUNNER: std::cell::OnceCell<LocalTaskRunner<()>> = std::cell::OnceCell::new();

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
	let play_option = example.play_option();
    let play_option1 = play_option.clone().unwrap_or_default();
   
    
    match (play_option1.play_path, play_option1.play_url, play_option1.play_way.as_str()) {
        (_, Some(url), "url") => {
            // println!("本机IP地址: {}", ip);

             //构建客户端
            let httpc = pi_async_httpc::AsyncHttpcBuilder::new()
                .bind_address("0.0.0.0") // 访问(localhost之外)外网用明确的本地ip（自身ip）
                .build().unwrap();
            init_load_cb(Arc::new(move |_: String, _:String,  hash:String, path:Vec<Arg>| {
                let httpc = httpc.clone();
                let url = url.clone();
                let path = match &path[0] {
                    Arg::String(r) => r.clone(),
                    _ => return,
                };
                MULTI_MEDIA_RUNTIME
                    .spawn(async move {
                        let mut result = Vec::new();
                        let pp: String = url + "/" + path.as_str();
                        match httpc
                            .build_request(pp.as_str(), pi_async_httpc::AsyncHttpRequestMethod::Get)
                            // .set_pairs(&[("login_type", "2"), ("user", "1694151132349ldxNJ")]) // 设置参数
                            .send().await 
                        {
                            Err(e) => {
                                log::warn!("not find file, url: {:?}, {:?}", path, e);
                            },
                            Ok(mut resp) => {
                                // println!("!!!!!!request time: {:?}", now.elapsed());

                                loop {
                                    match resp.get_body().await 
                                    {
                                        Err(e) => {
                                            log::warn!("not find file, url: {:?}, {:?}", path, e);
                                            break;
                                        },
                                        Ok(Some(_body)) => {
                                            result.extend_from_slice(_body.as_ref());
                                            continue;
                                        },
                                        Ok(None) => {
                                            if resp.get_status() == 200 {
                                                on_load(hash.parse::<u64>().unwrap(), Ok(result));
                                                log::debug!("load file success,path: {:?}", path);
                                                // on_load(path.as_str(), result);
                                            } else {
                                                on_load(hash.parse::<u64>().unwrap(), Err(format!("not find file, url: {:?}, {:?}", path, resp.get_status())));
                                                log::warn!("not find file, url: {:?}, {:?}", path, resp.get_status());
                                            }
                                            
                                            // println!("!!!!!!response time: {:?}", now.elapsed());
                                            // println!("!!!!!!peer address: {:?}", resp.get_peer_addr());
                                            // println!("!!!!!!url: {}", resp.get_url());
                                            // println!("!!!!!!status: {}", resp.get_status());
                                            // println!("!!!!!!version: {}", resp.get_version());
                                            // println!("!!!!!!headers: {:#?}", resp.to_headers());
                                            // println!("!!!!!!body len: {:?}", resp.get_headers("content-length"));
                                            break;
                                        },
                                    }
                                }
                            },
                        }
                    })
                    .unwrap();
            }));
        },
        (Some(dir), _, _) => {
            init_load_cb(Arc::new(move |_: String, _:String,  hash:String, path:Vec<Arg>| {
                let dir = dir.clone();
                let path = match &path[0] {
                    Arg::String(r) => r.clone(),
                    _ => return,
                };
                MULTI_MEDIA_RUNTIME
                    .spawn(async move {
                        if let Ok(file) = std::fs::read(Path::new(dir.as_str()).join(&path)) {
                            on_load(hash.parse::<u64>().unwrap(), Ok(file));
                            log::debug!("load file success,path: {:?}", path);
                            // on_load(path.as_str(), file);
                        } else {
                            on_load(hash.parse::<u64>().unwrap(), Err(format!("not find file,path: {:?}", path)));
                            log::warn!("not find file,path: {:?}", path);
                        }
                    })
                    .unwrap();
            }));
        },
        _ => {
            init_load_cb(Arc::new(move |module: String, _:String, hash:String, path:Vec<Arg>| {
                // println!("=========== module: {}, {}", module, hash);
                if module.ends_with("file"){
                    let path = match &path[0] {
                        Arg::String(r) => r.clone(),
                        _ => panic!(""),
                    };
                    MULTI_MEDIA_RUNTIME
                        .spawn(async move {
                            if let Ok(file) = std::fs::read(path.as_str()) {
                                on_load(hash.parse::<u64>().unwrap(), Ok(file));
                                // on_load(path.as_str(), file);
                                log::debug!("load file success,path: {:?}", path);
                            } else {
                                on_load(hash.parse::<u64>().unwrap(), Err(format!("not find file,path: {:?}", path)));
                                log::warn!("not find file,path: {:?}", path);
                            }
                        })
                        .unwrap();
                }else{
                    on_load(hash.parse::<u64>().unwrap(), Ok(vec![]));
                }
                
            }));
        },
    }
	// // let aa = pi_async_rt::rt::startup_global_time_loop(10);
	// // let current_dir = std::env::current_dir().unwrap();
	// #[cfg(not(target_arch = "wasm32"))]
	// init_load_cb(Arc::new(|path: String| {
    //     MULTI_MEDIA_RUNTIME
    //         .spawn(async move {
    //             if let Ok(dynamic_image) = std::fs::read(path.clone()) {
    //                 on_load(path.as_str(), dynamic_image);
    //             } else {
    //                 log::warn!("not find image,path: {:?}", path);
    //             }
    //         })
    //         .unwrap();
    // }));

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
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/dialog_bg.png")[..]));
				} else if path.as_str() == "examples/z_source/6.png" {
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/6.png")[..]));
				} else if path.as_str() == "examples/z_source/chouka_shitou_1.png" {
					on_load(path.as_str(), Vec::from(&include_bytes!("./z_source/chouka_shitou_1.png")[..]));
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
	let font_type = example.font_type();
    let exmple = Share::new(ShareMutex::new(example));
    let exmple1 = exmple.clone();

    let exmple_run = move |mut commands: SingleResMut<UserCommands>| {
        // log::warn!("zzzzzzzzzzzzzzzzzzzzzzzzbbbbbb");
        exmple.lock().unwrap().render(&mut commands);
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

	// // 初始化sdf的加载方法
	// if use_sdf {
	// 	log::warn!("init_load_cb1===========" );
	// 	pi_hal::font::sdf_brush::init_load_cb(Arc::new(move |key: DefaultKey, font_family: usize, chars: &[char]| {
	// 		let current_dir = current_dir.clone();
	// 		log::warn!("init_load_cb==========={:?}, {:?}", key, chars);
	// 		let chars = Vec::from(chars);
	// 		MULTI_MEDIA_RUNTIME.spawn(async move { // 这里必须异步，否则会造成死锁
	// 			let font_name = Atom::get(font_family).unwrap();
	// 			let mut result: Vec<Vec<u8>> = Vec::with_capacity(chars.len());
	// 			for char in chars.iter() {
	// 				let unicode = unsafe{transmute::<_, u32>(*char)};
	// 				let path = current_dir.join(format!("D://0_js/cdqxz_new_mult_gui_exe/dst_font/{}/_{}.bin", font_name.as_str(), unicode));
	// 				if let Ok(buffer) = std::fs::read(path.clone()) {
	// 					result.push(buffer);
	// 				} else {
	// 					panic!("not find sdf font,path: {:?}", path);
	// 				}
	// 			}
	// 			log::warn!("onload==========={:?}, {:?}, {:?}", key, chars, result.len());
	// 			pi_hal::font::sdf_brush::on_load(key, result);
	// 		}).unwrap();
	// 	}));
	// }

    app.world.insert_single_res(RunState::MATRIX);
	#[cfg(feature = "debug")]
	if let Some(play_option) = play_option {
		app.world.insert_single_res(play_option);
	}

    #[cfg(feature = "debug")]
    app.add_plugins(UiPlugin { cmd_trace: record_option, font_type });
    #[cfg(not(feature = "debug"))]
    app.add_plugins(UiPlugin::default());
	exmple1.lock().unwrap().setting(&mut app);

	#[cfg(feature = "debug")]
    match record_option {
        pi_ui_render::system::cmd_play::TraceOption::None => (),
        pi_ui_render::system::cmd_play::TraceOption::Record => {
            app.add_system(UiStage, record_cmd_to_file.in_set(UiSystemSet::NextSetting));
        }
        pi_ui_render::system::cmd_play::TraceOption::Play => {
            app.add_system(First, setting_next_record);
        }
    }

    app.add_system(First, exmple_run)
        .add_startup_system(First, move |param: Param| {
            exmple1.lock().unwrap().init(param, (width as usize, height as usize));
        });

	event_loop.run(move |event, _, control_flow| {

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
			Event::RedrawRequested(_) => {
				#[cfg(not(target_arch = "wasm32"))]
                app.run();

				#[cfg(target_arch = "wasm32")] 
				{
					// 资源运行时
					let rt = unsafe{RUNNER.get().unwrap()};
					while pi_hal::runtime::RENDER_RUNTIME.len() > 0 {
						rt.poll();
						rt.run_once();
					}
					app.run();
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
    let mut app = App::new();

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

    match std::env::var("GL") {
        Ok(r) if r == "opengl" => {
            o.present_mode = wgpu::PresentMode::Fifo;
            o.backends = wgpu::Backends::GL;
        },
		_ => {
            o.present_mode = wgpu::PresentMode::Mailbox;
            o.backends = wgpu::Backends::VULKAN;
        },
    };
	// o.present_mode = wgpu::PresentMode::Fifo;
	// o.backends = wgpu::Backends::GL;

	// o.present_mode = wgpu::PresentMode::Mailbox;
	// o.backends = wgpu::Backends::VULKAN;

	app.world.insert_single_res(o);

	// app.world.insert_single_res(IsRun(true));

	let filter = match std::env::var("RUST_LOG") {
		Ok(r) => r,
		Err(_) => "info,wgpu=warn,naga_warn".to_string(),
	};
    println!("filter========={:?}", filter);

	// let level = match std::env::var("RUST_LOG") {
	// 	Ok(r) => match r.as_str() {
	// 		"trace" => bevy_log::Level::TRACE,
	// 		"info" => bevy_log::Level::INFO,
	// 		"warn" => bevy_log::Level::WARN,
	// 		"error" => bevy_log::Level::ERROR,
	// 		_ => bevy_log::Level::INFO
	// 	},
	// 	Err(_) => bevy_log::Level::INFO,
	// };

    app.add_plugins(pi_bevy_log::LogPlugin::<Vec<u8>> {
        filter,
        level: LOG_LEVEL.clone(),
		chrome_write: None,
    })
    // .add_plugins(bevy_a11y::AccessibilityPlugin)
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

	


    // let h = app.world.get_single_res_mut::<pi_bevy_log::LogFilterHandle>().unwrap();
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
    is_end: bool,
}

#[cfg(feature = "debug")]
impl Default for NextState {
    fn default() -> Self {
        NextState {
            file_index: 0,
            // play_version: "performance",
            // play_version: "test",
            // play_path: "D://0_js/cdqxz_new_mult_gui_exe/dst",
            // play_path: "D://0_js/cdqxz_new_gui_exe/dst",
            // cmd_path: Some("D://0_rust/pi_export/crates/gui/examples/cmd_play/source/cmds"),
            is_end: false,
        }
    }
}

// 将record写入文件
#[cfg(feature = "debug")]
pub fn record_cmd_to_file(mut records: SingleResMut<Records>) {
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
    if local_state.is_end {
        return;
    }
	let play_option  = (*world.get_single_res::<PlayOption>().unwrap()).clone();
    let local_state = &mut *local_state;
    setting(&mut local_state.file_index, world, &mut local_state.is_end, &play_option)
}


#[cfg(feature = "debug")]
fn setting(file_index1: &mut usize, world: &mut World, is_end: &mut bool, play_option: &PlayOption) {
    use std::mem::transmute;

    use pi_world::system::TypeInfo;

    let mut file_index = *file_index1;
    let play_state = world.get_single_res::<PlayState>();
    if let Some(r) = play_state {
        if r.is_running {
            return;
        } else {
            let path = play_option.cmd_path.clone() + "/cmd_" + play_option.play_version.as_str() + "_" + file_index.to_string().as_str() + ".gui_cmd";
            // let path = Path::new(play_option.cmd_path.as_str()).join(("cmd_".to_string() + play_option.play_version.as_str() + "_" + file_index.to_string().as_str() + ".gui_cmd").as_str());
            if file_index > play_option.max_index {
                if !*is_end {
                    log::warn!("play end, {:?}", path);
                    // world.insert_single_res(IsRun(true)); // 屏蔽所有节点运行
                }
                *is_end = true;
                return;
            }
            let world: &'static mut World = unsafe {transmute(world)};
            let file_index1: &'static mut usize = unsafe {transmute(file_index1)}; 
            let is_end: &'static mut bool = unsafe {transmute(is_end)}; 
            let speed = play_option.speed;
            let path1 = path.clone();

            use pi_async_rt::prelude::AsyncRuntimeExt;
            let _ = pi_hal::runtime::MULTI_MEDIA_RUNTIME.block_on(async move {
                match pi_hal::file::load_from_url(&pi_atom::Atom::from(path)).await {
                    Ok(bin) => {
                        match postcard::from_bytes::<Records>(&bin) {
                            Ok(r) => {
                                log::debug!("cmd!!!!!!!!!================{:?}", r.len());
                                world.or_register_single_res(TypeInfo::of::<Records>());
                                **world.get_single_res_mut::<Records>().unwrap() = r;
                                // 重设播放状态
                                let play_state = world.get_single_res_mut::<PlayState>().unwrap();
                                play_state.is_running = true;
                                play_state.next_reord_index = 0;
                                play_state.next_state_index = 0;
                                play_state.cur_frame_count = 0;
                                play_state.speed = speed;
                            }
                            Err(e) => {
                                *is_end = true;
                                log::warn!("parse fail================{:?}, {:?}", e, bin.len());
                            }
                        }
                        file_index += 1;
                        *file_index1 = file_index;
                    }
                    Err(_e) => {
                        if !*is_end {
                            log::warn!("play end, {:?}", path1);
                            // world.insert_single_res(IsRun(true)); // 屏蔽所有节点运行
                        }
                        *is_end = true;
                        return;
                    }
                }
            });
            
        }
    }
    return;
}

#[derive(SystemParam)]
pub struct Param<'w> {
    pub insert: Insert<'w, ()>,
    pub creates: OrInitSingleResMut<'w, CmdNodeCreate>,
    pub user_cmd: OrInitSingleResMut<'w, UserCommands>,
    pub font_sheet: Option<SingleRes<'w, ShareFontSheet>>,
    pub play_option: Option<SingleRes<'w, PlayOption>>,
}

impl Deref for Param<'_> {
    type Target = UserCommands;

    fn deref(&self) -> &Self::Target {
        &**self.user_cmd
    }
}

impl DerefMut for Param<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut **self.user_cmd
    }
}

impl Param<'_> {
    pub fn spawn(&mut self, tag: NodeTag) -> Entity {
        let r = self.insert.insert(());
        self.user_cmd.init_node(r, tag);
        #[cfg(feature = "debug")]
        {
            self.creates.0.push(r);
        }

        r
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlayOption {
	pub play_path: Option<String>,
    pub play_url: Option<String>,
    pub play_way: String, // "path" or "url"
	pub play_version: String,
	pub cmd_path: String,
    pub max_index: usize,
    pub speed: f32,
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
// pub const FILTER: &'static str = "wgpu=warn,naga=warn,pi_ui_render::system::draw_obj::life_drawobj=trace";// pi_ui_render::system::draw_obj::calc_text::text_sdf2=trace
// pub const FILTER: &'static str = "wgpu=warn,naga=warn,pi_ui_render::components::user=trace";//pi_ui_render::resource::animation_sheet=trace
// pub const FILTER: &'static str = "wgpu=warn,naga=trace";
// pub const FILTER: &'static str = "wgpu=warn,pi_ui_render::system::pass::pass_graph_node=trace,pi_ui_render::system::pass_effect::radial_wave=trace,pi_ui_render::system::pass::pass_life=trace";
// pub const FILTER: &'static str = "wgpu=warn,pi_ui_render::system::pass_effect::radial_wave=trace,pi_ui_render::system::pass::pass_life=trace,pi_ui_render::system::pass::update_graph=trace";
// pi_bevy_render_plugin=error
// pub const FILTER: &'static str = "wgpu=error,naga=warn,bevy_app=warn,pi_world::schedule::executor::single_threaded=warn,pi_world::system::commands=warn,pi_bevy_render_plugin=error";
// pub const FILTER: &'static str = "wgpu=warn,naga=warn,pi_wgpu=warn,pi_ui_render::system::draw_obj::life_drawobj=trace,pi_ui_render::system::pass::pass_graph_node=trace";
// pub const FILTER: &'static str = "";
// pub const LOG_LEVEL: bevy_log::Level = bevy_log::Level::INFO;
pub const LOG_LEVEL: tracing::Level = tracing::Level::INFO;
