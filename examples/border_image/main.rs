// 一个简单BorderImage

#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::{style::{Dimension, PositionType}, prelude::Rect};
use pi_null::Null;
use pi_ui_render::{
	gui::Gui, 
	components::user::{ CgColor, BorderImage, BorderImageSlice, Border, BorderImageRepeat, BorderImageRepeatOption}, 
	resource::ClearColor
};
use pi_style::style_type::{WidthType, HeightType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType, BorderImageType, BorderImageSliceType, BorderType, BorderImageRepeatType};

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
		gui.set_style(div1, BorderImageType (BorderImage(Atom::from("examples/border_image/source/dialog_bg.png") )));
		gui.set_style(div1, BorderImageSliceType(BorderImageSlice {
			top: unsafe { NotNan::new_unchecked(0.3333334) },
			right: unsafe { NotNan::new_unchecked(0.4272727) },
			bottom: unsafe { NotNan::new_unchecked(0.5625) },
			left: unsafe { NotNan::new_unchecked(0.4272727) },
			fill: true,
		}));
		gui.set_style(div1, BorderType (Border(Rect {
			left: Dimension::Points(48.0),
			top: Dimension::Points(48.0),
			right: Dimension::Points(48.0),
			bottom: Dimension::Points(81.0),
		} )));
		gui.set_style(div1, BorderImageRepeatType (BorderImageRepeat (BorderImageRepeatOption::Repeat, BorderImageRepeatOption::Repeat)));
		gui.append(div1, root);
	}
	
	fn render(&mut self, gui: &mut Gui) {
		gui.run();
	}
}




