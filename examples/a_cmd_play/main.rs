#[path = "../framework.rs"]
mod framework;

use std::path::PathBuf;

// use font_kit::font::new_face_by_path;
use framework::{Param, Example};
use nalgebra::Point2;
use pi_flex_layout::prelude::Size;
use pi_style::style::Aabb2;
use pi_ui_render::resource::UserCommands;
//
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
fn main() { framework::start(ExampleCommonPlay::new()) }

pub struct ExampleCommonPlay {
	current_dir: PathBuf,
}

impl ExampleCommonPlay {
	fn new() -> Self {
		Self {
			current_dir: std::env::current_dir().unwrap(),
		}
	}
}

// pub const FILTER: &'static str = "wgpu=error,naga=warn,bevy_app=warn,pi_world::schedule::executor::single_threaded=warn,pi_world::system::commands=warn,pi_bevy_render_plugin=error";

// pub struct Commands1 {
//     queue: &'static mut CommandQueue,
//     entities: &'static Entities,
// }

impl Example for ExampleCommonPlay {
    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        // Some(Size { width: 1080, height: 2160 })
        #[cfg(not(target_os = "android"))]
		let r = Some(Size { width: 522, height: 919 });
        #[cfg(target_os = "android")]
        let r = None;
        r
        // None
    }

    fn init(&mut self, mut _world: Param, size: (usize, usize)) {   
        _world.play_state.view_port = Some(Aabb2::new(
            Point2::new(0.0, 0.0),
            Point2::new(size.0 as f32, size.1 as f32),
        ));
        
        println!("view_port:{:?}", size);
    }

    fn render(&mut self, _cmd: &mut UserCommands) {  }

    #[cfg(feature = "debug")]
    fn record_option(&self) -> pi_ui_render::system::base::node::cmd_play::TraceOption { pi_ui_render::system::base::node::cmd_play::TraceOption::Play }

    fn play_option(&self) -> Option<framework::PlayOption> {
		let mut option = framework::PlayOption {
			play_path: None,
			play_version: "".to_string(),
    		cmd_path: "".to_string(),
            max_index: std::usize::MAX,
            speed: 1.0,
            play_url: None,
            play_way: "path".to_string(),
		};
        #[cfg(target_os = "android")]
        let config = include_str!("source/run_config.txt");
        #[cfg(not(target_os = "android"))]
        let config = std::fs::read_to_string(self.current_dir.join("examples/a_cmd_play/source/run_config.txt")).unwrap();
		let r = config.split(";");
        for i in r {
            let mut r = i.split("=");
            if let (Some(key), Some(value)) = (r.next(), r.next()) {
                let key = key.trim();
                if key == "play_path" {
                    option.play_path = Some(value.trim().to_string());
                } else if key == "play_url" {
                    option.play_url = Some(value.trim().to_string());
                } else if key == "play_way" {
                    option.play_way = value.trim().to_string();
                } else if key == "play_version" {
                    option.play_version = value.trim().to_string();
                } else if key == "cmd_path" {
                    option.cmd_path = value.trim().to_string();
                } else if key == "max_index" {
                    option.max_index = value.trim().parse().unwrap();
                } else if key == "speed" {
                    option.speed = value.trim().parse().unwrap();
                }
            }
        }

        if option.play_way.as_str() == "url" {
            option.cmd_path = "gui_cmd".to_string();
        } else if option.play_way.as_str() == "path" {
            option.cmd_path = self.current_dir.join("examples/a_cmd_play/source/cmds").to_string_lossy().to_string(); 
        }

        
		println!("play_option==={:?}", option);
		Some(option)
	}
}


