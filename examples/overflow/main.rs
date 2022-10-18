// 半透明渲染

#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
	components::user::{Color, CgColor, ClearColor}, 
	export::Engine
};
use pi_style::style_type::{WidthType, HeightType, BackgroundColorType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType, OpacityType, OverflowType};

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
		gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true));
		
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

		// 添加一个玫红色div到根节点， 并添加overflow属性
		let div1 = gui.gui.create_node();
		gui.gui.set_style(div1, WidthType(Dimension::Points(300.0)));
		gui.gui.set_style(div1, HeightType(Dimension::Points(300.0)));
		gui.gui.set_style(div1, BackgroundColorType (Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0)) ));
		// gui.gui.set_style(div1, OverflowType(Overflow(true)));
		gui.gui.append(div1, root);

		// 添加一个红色div到玫红节点
		let div2 = gui.gui.create_node();
		gui.gui.set_style(div2, WidthType(Dimension::Points(50.0)));
		gui.gui.set_style(div2, HeightType(Dimension::Points(100.0)));
		gui.gui.set_style(div2, BackgroundColorType (Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0)) ));
		gui.gui.append(div2, div1);

		// 添加一个容器节点，其下有一个绿色节点，一个黄色节点， 对本节点添加TransformWillchange
		let div3 = gui.gui.create_node();
		gui.gui.set_style(div3, PositionTopType(Dimension::Points(100.0)));
		gui.gui.set_style(div3, WidthType(Dimension::Points(90.0)));
		gui.gui.set_style(div3, HeightType(Dimension::Points(150.0)));
		// 设置TransformWillChange，向右平移100个像素
		gui.gui.set_style(div3, OverflowType(true));

		// 添加一个绿色div
		let div4 = gui.gui.create_node();
		gui.gui.set_style(div4, WidthType(Dimension::Points(50.0)));
		gui.gui.set_style(div4, HeightType(Dimension::Points(100.0)));
		gui.gui.set_style(div4, BackgroundColorType (Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0)) ));
		gui.gui.append(div4, div3);

		// 添加一个黄色
		let div5 = gui.gui.create_node();
		gui.gui.set_style(div5, PositionTopType(Dimension::Points(100.0)));
		gui.gui.set_style(div5, WidthType(Dimension::Points(50.0)));
		gui.gui.set_style(div5, HeightType(Dimension::Points(100.0)));
		gui.gui.set_style(div5, BackgroundColorType (Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0)) ));
		// 设置opacity，测试Pass2d在父上存在TransformWillChange的情况下能否正确渲染
		gui.gui.set_style(div5, OpacityType(0.5));

		gui.gui.append(div5, div3);

		gui.gui.append(div3, div1);

	}
	
	fn render(&mut self, gui: &mut Engine) {
		gui.gui.run();
	}
}




