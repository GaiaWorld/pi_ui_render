// 一个简单BorderImage

#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
	gui::Gui, 
	components::user::{ CgColor, BoxShadow}, 
	utils::style::style_sheet::{WidthType, HeightType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType, BoxShadowType}, resource::ClearColor
};

fn main() {
	framework::start(QuadExample::default())
}

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(
		&mut self, 
		gui: &mut Gui, 
		size: (usize, usize),
	) {

		// 设置清屏颜色为绿色
		gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));
		
		// 添加根节点
		let root = gui.create_node();
		gui.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
		gui.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
		
		gui.set_style(root, PositionTypeType(PositionType::Absolute));
		gui.set_style(root, PositionLeftType(Dimension::Points(0.0)));
		gui.set_style(root, PositionTopType(Dimension::Points(0.0)));
		gui.set_style(root, MarginLeftType(Dimension::Points(0.0)));
		gui.set_style(root, MarginTopType(Dimension::Points(0.0)));

		gui.append(root, Id::null());

		// 添加一个红色div
		let div1 = gui.create_node();
		gui.set_style(div1, WidthType(Dimension::Points(110.0)));
		gui.set_style(div1, HeightType(Dimension::Points(144.0)));
		gui.set_style(div1, BoxShadowType (BoxShadow {
			h: 10.0,
			v: 10.0,
			spread: 3.0,
			blur: 6.0,
			color: CgColor::new(0.5, 0.5, 0.5, 1.0),
		}));
		gui.append(div1, root);
	}
	
	fn render(&mut self, gui: &mut Gui) {
		gui.run();
	}
}




