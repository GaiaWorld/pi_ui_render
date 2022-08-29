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
	components::user::{ CgColor, BoxShadow}, 
	resource::ClearColor, export::Engine
};
use pi_style::style_type::{WidthType, HeightType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType, BoxShadowType};

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
		size: (usize, usize),
	) {

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

		gui.gui.append(root, Id::null());

		// 添加一个红色div
		let div1 = gui.gui.create_node();
		gui.gui.set_style(div1, WidthType(Dimension::Points(110.0)));
		gui.gui.set_style(div1, HeightType(Dimension::Points(144.0)));
		gui.gui.set_style(div1, BoxShadowType (BoxShadow {
			h: 50.0,
			v: 50.0,
			spread: 0.0,
			blur: 20.0,
			color: CgColor::new(0.5, 0.5, 0.5, 1.0),
		}));
		gui.gui.append(div1, root);
	}
	
	fn render(&mut self, gui: &mut Engine) {
		gui.gui.run();
	}
}




