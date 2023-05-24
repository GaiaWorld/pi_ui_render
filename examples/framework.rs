use std::{sync::Arc, time::Instant};

use bevy::prelude::App;
use bevy::winit::WinitPlugin;
use bevy::{
    ecs::{
        system::{Commands, ResMut, SystemState},
        world::World,
    },
    prelude::IntoSystemConfig,
    window::{Window, WindowResolution},
};
use pi_async::prelude::AsyncRuntime;
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_flex_layout::prelude::Size;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_share::{Share, ShareMutex};
use pi_ui_render::system::{system_set::UiSystemSet, RunState};
use pi_ui_render::{prelude::UiPlugin, resource::UserCommands, system::node::user_setting::user_setting};

pub trait Example: 'static + Sized {
    // fn setting(world: &mut App) {}
    fn init(&mut self, world: &mut World, size: (usize, usize));
    fn render(&mut self, commands: &mut UserCommands, cmd1: &mut Commands);

    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        None
    }
}

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
    init_load_cb(Arc::new(|path: String| {
        MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                if let Ok(dynamic_image) = std::fs::read(path.clone()) {
                    on_load(path.as_str(), dynamic_image);
                } else {
                    log::warn!("not find image,path: {:?}", path);
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

    let exmple = Share::new(ShareMutex::new(example));
    let exmple1 = exmple.clone();
    let exmple_run = move |world: &mut World, commands: &mut SystemState<(ResMut<UserCommands>, Commands)>| {
        // log::warn!("zzzzzzzzzzzzzzzzzzzzzzzzbbbbbb");
        let mut commands = commands.get_mut(world);
        exmple.lock().render(&mut commands.0, &mut commands.1);
    };

    let mut app = init(width, height);

    app.world.insert_resource(RunState::RENDER);
    app.add_plugin(UiPlugin);


    app.add_system(exmple_run.before(user_setting).in_set(UiSystemSet::Setting))
        .add_startup_system(move |world: &mut World| {
            exmple1.lock().init(world, (500, 500));
        });
    app.run();

    // let system_schedule = bevy_mod_debugdump::get_schedule(&mut app);
    // let mut file = File::create("system_schedule.dot").unwrap();
    // file.write_all(system_schedule.as_bytes()).unwrap();

    // bevy_mod_debugdump::print_schedule(&mut app);

    // run_window_loop(window, event_loop);
}

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

pub fn init(width: u32, height: u32) -> App {
    let mut app = App::default();

    // let event_loop =  EventLoopBuilder::new().with_any_thread(true).build();
    // let window = winit::window::Window::new(&event_loop).unwrap();
    // window.set_inner_size(PhysicalSize {width, height});
    let mut window = Window::default();
    window.resolution = WindowResolution::new(width as f32, height as f32);
    let mut window_plugin = bevy::window::WindowPlugin::default();
    window_plugin.primary_window = Some(window);

    app.add_plugin(bevy::log::LogPlugin {
        filter: "wgpu=warn,pi_ui_render::components::user=debug".to_string(),
        level: bevy::log::Level::INFO,
    })
    .add_plugin(bevy::a11y::AccessibilityPlugin)
    .add_plugin(bevy::input::InputPlugin::default())
    .add_plugin(window_plugin)
    .add_plugin(WinitPlugin::default())
    // .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(PiRenderPlugin::default())
    .add_plugin(PiPostProcessPlugin);
    app
}
