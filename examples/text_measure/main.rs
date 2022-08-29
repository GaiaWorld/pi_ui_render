// 一个简单的四边形渲染

#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use font_kit::font::{new_face_by_path};
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_render::font::{FontSheet, Font};
use pi_share::{Share, ShareCell};
use pi_ui_render::export::Engine;

fn main() {
	framework::start(QuadExample::default())
}

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(
		&mut self, 
		gui: &mut Engine, 
		_size: (usize, usize),
	) {
		let mut dir = std::env::current_dir().unwrap();
		log::info!("dir: {:?}", dir);
		dir.push("examples\\text\\source\\hwkt.ttf");
		new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
		let name = Atom::from("hwkt");
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
		
		let font_sheet = gui.gui.world_mut().get_resource::<Share<ShareCell<FontSheet>>>().unwrap();
		let mut font_sheet = font_sheet.borrow_mut();
		
		
		let font_id = font_sheet.font_id(Font::new(
			name,
			30,
			500,
			unsafe { NotNan::new_unchecked(0.0) },
		));
		
		// let list = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
		// let mut r = None;

		// r = Some(font_sheet.measure_width(font_id, 'w'));
		// let mut r = None;
		// let time = std::time::Instant::now();
		// for z in 0..1000 {
		// 	for i in 0..10 {
		// 		// face.set_pixel_sizes(30);
		// 		r = Some(font_sheet.measure_width(font_id, list[i]));
		// 	}
		// }

		let list = vec!['获', '取', '验', '证', '码', '登', '录', '注', '册', '手','机', '号', '证', '码', '登', '录', '注', '册', '手', '账', '我', '是'];
		let mut r = None;

		// r = Some(font_sheet.measure_width(font_id, 'w'));
		let time = std::time::Instant::now();
		for z in 0..2 {
			for i in 0..list.len() {
				// face.set_pixel_sizes(30);
				r = Some(font_sheet.measure_width(font_id, list[i]));
			}
		}
		println!("measure_width time, 10000 times: {:?}, {:?}", std::time::Instant::now() - time, r);
		
	}
	
	fn render(&mut self, gui: &mut Engine) {
		gui.gui.run();
	}
}




