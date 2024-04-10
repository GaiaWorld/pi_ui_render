// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::time::Duration;

use async_trait::async_trait;
use bevy_ecs::prelude::{Commands, World};
use font_kit::font::{new_face_by_path, new_face};
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_render::font::Font;
use pi_ui_render::resource::{ShareFontSheet, UserCommands};
use pi_async_rt::prelude::AsyncRuntime;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    fn init(&mut self, world: &mut World, _size: (usize, usize)) {
        let mut dir = std::env::current_dir().unwrap();
        log::info!("dir: {:?}", dir);
        dir.push("examples\\text_measure\\source\\hwxw.ttf");
		
		let buffer = std::fs::read(dir).unwrap();

		let name = Atom::from("hwxw");
		// let font_sheet = world.get_resource::<ShareFontSheet>().unwrap();
		// let mut font_sheet = font_sheet.borrow_mut();
		// let font_id = font_sheet.font_id(Font::new(name.clone(), 30, 500, unsafe { NotNan::new_unchecked(0.0) }));
			

		// 	// let list = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
		// 	// let mut r = None;

		// 	// r = Some(font_sheet.measure_width(font_id, 'w'));
		// 	// let mut r = None;
		// 	// let time = std::time::Instant::now();
		// 	// for z in 0..1000 {
		// 	// 	for i in 0..10 {
		// 	// 		// face.set_pixel_sizes(30);
		// 	// 		r = Some(font_sheet.measure_width(font_id, list[i]));
		// 	// 	}
		// 	// }

		// 	let list = vec![
		// 		'获', '取', '验', '证', '码', '登', '录', '注', '册', '手', '机', '号', '证', '码', '登', '录', '注', '册', '手', '账', '我', '是',
		// 	];
		// 	let mut r = None;

		// 	// r = Some(font_sheet.measure_width(font_id, 'w'));
		// 	let time = std::time::Instant::now();
		// 	for _z in 0..2 {
		// 		for i in 0..list.len() {
		// 			// face.set_pixel_sizes(30);
		// 			r = Some(font_sheet.measure_width(font_id, list[i]));
		// 		}
		// 	}
		// 	println!("measure_width time, 10000 times: {:?}, {:?}", std::time::Instant::now() - time, r);

		pi_hal::runtime::MULTI_MEDIA_RUNTIME.spawn(async move {
			let time = std::time::Instant::now();
			new_face("hwxw".to_string(), buffer).await;
			log::warn!("set size, times: {:?}", std::time::Instant::now() - time);

			// new_face_by_path("hwkt".to_string(), "examples/common_play/source/SOURCEHANSANSK-MEDIUM.TTF");


			// let mut face = match Face::from_family_name(&name) {
			// 	Ok(r) => r,
			// 	Err(_) => Face::from_family_name("default").unwrap()
			// };
			// face.set_pixel_sizes(30);
			// let list = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
			// let mut index = Vec::new();
			// for i in 0..10 {
			// 	// face.set_pixel_sizes(30);
			// 	index.push(face.get_glyph_id(list[i]));
			// 	// r = Some(face.get_metrics(list[i]));
			// }
			// face.get_metrics('w');
			// let mut r = None;
			// let time = std::time::Instant::now();
			// for z in 0..1000 {
			// 	for i in 0..10 {
			// 		// face.set_pixel_sizes(30);
			// 		r = Some(face.get_metrics1(index[i]));
			// 	}
			// }
			// println!("measure_width time, 10000 times: {:?}, {:?}", std::time::Instant::now() - time, r);

			
			
			let mut v = Vec::new();
			for i in 0..10 {
				let time = std::time::Instant::now();
				let mut font_id = font_kit::font::Face::from_family_name(name.as_str(), 16 + i).unwrap();
				log::warn!("new face, times: {:?}", std::time::Instant::now() - time);
				let time = std::time::Instant::now();
				font_id.set_pixel_sizes(16 + 16 + i);
				log::warn!("set size, times: {:?}", std::time::Instant::now() - time);
				v.push(font_id);
			}
			log::warn!("{:?}", v);
			
		}).unwrap();

		std::thread::sleep(Duration::from_secs(10));
        
    }

    fn render(&mut self, _cmd: &mut UserCommands, _cmd1: &mut Commands) {
        //swap(&mut self.cmd, cmd);
    }
}
