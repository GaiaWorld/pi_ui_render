// 一个简单的四边形渲染

#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use font_kit::font::new_face_by_path;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
	components::user::{Color, CgColor, TextContent, FontSize, BackgroundColor}, 
	resource::ClearColor, export::Engine
};

fn main() {
	framework::start(QuadExample::default())
}
use pi_style::style_type::{WidthType, HeightType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType, TextContentType, FontFamilyType, ColorType, FontSizeType, BackgroundColorType};

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(
		&mut self, 
		gui: &mut Engine, 
		size: (usize, usize),
	) {
		let mut dir = std::env::current_dir().unwrap();
		log::info!("dir: {:?}", dir);
		dir.push("examples/text/source/hwkt.ttf");
		// new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
		new_face_by_path("hwkt".to_string(), "examples/common_play/source/SOURCEHANSANSK-MEDIUM.TTF");

		// 设置清屏颜色为绿色
		gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));
		
		// 添加根节点
		let root = gui.gui.create_node();
		gui.gui.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
		gui.gui.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
		
		gui.gui.set_style(root, PositionTypeType(PositionType::Absolute));
		gui.gui.set_style(root, PositionLeftType(Dimension::Points(0.0)));
		gui.gui.set_style(root, PositionTopType(Dimension::Points(0.0)));
		gui.gui.set_style(root, MarginLeftType(Dimension::Points(0.0)));
		gui.gui.set_style(root, MarginTopType(Dimension::Points(0.0)));
		gui.gui.set_style(root, BackgroundColorType (BackgroundColor(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0)) )));


		// if style.position_left().is_points()
		// 	&& style.position_top().is_points()
		// 	&& style.margin_left().is_points()
		// 	&& style.margin_top().is_points()
		// 	&& style.width().is_points()
		// 	&& style.height().is_points()
		gui.gui.append(root, Id::null());

		// 添加一个红色div
		let div1 = gui.gui.create_node();
		gui.gui.set_style(div1, WidthType(Dimension::Points(50.0)));
		gui.gui.set_style(div1, HeightType(Dimension::Points(100.0)));
		gui.gui.set_style(div1, TextContentType (TextContent("baseo".to_string(), Atom::from("base"))));
		gui.gui.set_style(div1, FontFamilyType (Atom::from("hwkt")));
		gui.gui.set_style(div1, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
		gui.gui.set_style(div1, FontSizeType(FontSize::Length(17)));
		// gui.gui.set_style(div1, TextStrokeType(Stroke {
		// 	width: unsafe {NotNan::new_unchecked(2.0)}, 
		// 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
		gui.gui.append(div1, root);
	}
	
	fn render(&mut self, gui: &mut Engine) {
		gui.gui.run();
	}
}




