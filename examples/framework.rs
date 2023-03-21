use std::{sync::Arc, time::Instant, mem::transmute};

use async_trait::async_trait;
use bevy::app::CoreStage;
use bevy::ecs::{
    schedule::{IntoSystemDescriptor, ShouldRun, StageLabel, SystemStage},
    system::{Commands, ResMut, SystemState},
    world::World,
};
use bevy::prelude::App;
use bevy::winit::WinitPlugin;
use pi_async::prelude::AsyncRuntime;
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin};
use pi_flex_layout::prelude::Size;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_share::{Share, ShareMutex};
use pi_export_gui::{Engine, Gui};
use pi_ui_render::system::RunState;
use pi_ui_render::{prelude::UiPlugin, resource::UserCommands, system::node::user_setting::user_setting};

#[async_trait]
pub trait Example: 'static + Sized {
    fn init(&mut self, app: Commands, gui: &mut Gui, size: (usize, usize));
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

	let mut engine = create_engine(width, height);

	engine.world.insert_resource(RunState::RENDER);
    engine.add_plugin(UiPlugin);

    engine.app_mut()
		.add_system_to_stage(CoreStage::First, exmple_run.before(user_setting))
		.add_stage_before(
			CoreStage::First,
			InitStartupStage::Startup,
			SystemStage::parallel().with_run_criteria(ShouldRun::once),
		)
		.add_system_to_stage(InitStartupStage::Startup, move |world: &mut World, commands_state: &mut SystemState<Commands>| {
			let mut gui = Gui::new(
				unsafe { transmute(world.entities()) },
				UserCommands::default(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				world.query(),
				SystemState::new(world),
				
			);
			let commands = commands_state.get_mut(world);
			exmple1.lock().init(commands, &mut gui, (500, 500));
			commands_state.apply(world);
		})
		.run();
	
    // let system_schedule = bevy_mod_debugdump::get_schedule(&mut app);
    // let mut file = File::create("system_schedule.dot").unwrap();
    // file.write_all(system_schedule.as_bytes()).unwrap();

    // bevy_mod_debugdump::print_schedule(&mut app);

    // run_window_loop(window, event_loop);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum InitStartupStage {
    /// The [`Stage`](bevy::ecs::schedule::Stage) that runs once before [`StartupStage::Startup`].
    Startup,
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

pub fn create_engine(width: u32, height: u32) -> Engine {
    let mut app = App::default();

	let mut window_plugin = bevy::window::WindowPlugin::default();
	window_plugin.window.width = width as f32;
	window_plugin.window.height = height as f32;
	
	app
		.add_plugin(bevy::log::LogPlugin {
			filter: "wgpu=info,pi_ui_render::components::user=debug".to_string(),
			level: bevy::log::Level::INFO,
		})
		.add_plugin(bevy::input::InputPlugin::default())
		.add_plugin(window_plugin)
		.add_plugin(WinitPlugin::default())
		// .add_plugin(WorldInspectorPlugin::new())
		.add_plugin(PiRenderPlugin::default())
		.add_plugin(PiPostProcessPlugin);
    Engine::new(app)
}
