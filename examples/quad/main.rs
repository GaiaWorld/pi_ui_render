
#[path ="../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use pi_async::rt::{
	AsyncRuntime, 
	single_thread::SingleTaskPool
};
/// 渲染四边形 demo
use pi_ecs::prelude::{Id, SingleDispatcher, Dispatcher};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_render::RenderStage;
use pi_ui_render::{
	gui::Gui, 
	components::user::{BackgroundColor, Color, CgColor, Point2, Aabb2}, 
	utils::style::style_sheet::{WidthType, HeightType, BackgroundColorType, PositionTypeType, PositionLeftType, PositionTopType, MarginLeftType, MarginTopType}, 
	resource::Viewport
};

fn main() {
	framework::start(QuadExample::default())
}

#[derive(Default)]
pub struct QuadExample{
	dispather: Option<SingleDispatcher<SingleTaskPool<()>>>,
}

#[async_trait]
impl Example for QuadExample {
    async fn init(
		&mut self, 
		gui: &mut Gui, 
		render_stage: RenderStage,
		_rt: AsyncRuntime<(), SingleTaskPool<()>>,
		size: (usize, usize),
	) {
		gui.init(render_stage);
		
		gui.world_mut().insert_resource(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))));
		// 添加根节点
		let root = gui.create_node();
		gui.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
		gui.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
		
		gui.set_style(root, PositionTypeType(PositionType::Absolute));
		gui.set_style(root, PositionLeftType(Dimension::Points(0.0)));
		gui.set_style(root, PositionTopType(Dimension::Points(0.0)));
		gui.set_style(root, MarginLeftType(Dimension::Points(0.0)));
		gui.set_style(root, MarginTopType(Dimension::Points(0.0)));


		// if style.position_left().is_points()
		// 	&& style.position_top().is_points()
		// 	&& style.margin_left().is_points()
		// 	&& style.margin_top().is_points()
		// 	&& style.width().is_points()
		// 	&& style.height().is_points()
		gui.append(root, Id::null());

		// 添加一个绿色div
		let div1 = gui.create_node();
		gui.set_style(div1, WidthType(Dimension::Points(50.0)));
		gui.set_style(div1, HeightType(Dimension::Points(100.0)));
		gui.set_style(div1, BackgroundColorType (BackgroundColor(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0)) )));
		gui.append(div1, root);
	}
	
	fn render(&mut self, gui: &mut Gui) {
		gui.run();
	}
}




