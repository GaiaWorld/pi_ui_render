// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::user::{CgColor, ClearColor, Viewport},
    export::Engine,
	utils::cmd::NodeCmd,
};

use pi_style::{style_type::{
    BorderBottomType, BorderColorType, BorderLeftType, BorderRightType, BorderTopType, HeightType, MarginLeftType, MarginTopType, PositionLeftType,
    PositionTopType, PositionTypeType, WidthType, BorderRadiusType,
}, style::{Aabb2, Point2, BorderRadius, LengthUnit}};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Engine, size: (usize, usize)) {
        // // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true));

        // 添加根节点
        let root = gui.gui.create_node();
		gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(0.7, 0.7, 0.7, 1.0), true), root));
		gui.gui.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))), root));

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
        gui.gui.set_style(div1, BorderColorType(CgColor::new(0.0, 1.0, 0.0, 1.0)));
        gui.gui.set_style(div1, BorderTopType(Dimension::Points(10.0)));
        gui.gui.set_style(div1, BorderRightType(Dimension::Points(15.0)));
        gui.gui.set_style(div1, BorderBottomType(Dimension::Points(10.0)));
        gui.gui.set_style(div1, BorderLeftType(Dimension::Points(15.0)));
		gui.gui.set_style(div1, MarginLeftType(Dimension::Points(10.0)));
        gui.gui.append(div1, root);

		let div2 = gui.gui.create_node();
        gui.gui.set_style(div2, WidthType(Dimension::Points(110.0)));
        gui.gui.set_style(div2, HeightType(Dimension::Points(144.0)));
        gui.gui.set_style(div2, BorderColorType(CgColor::new(0.0, 1.0, 0.0, 1.0)));
        gui.gui.set_style(div2, BorderTopType(Dimension::Points(10.0)));
        gui.gui.set_style(div2, BorderRightType(Dimension::Points(15.0)));
        gui.gui.set_style(div2, BorderBottomType(Dimension::Points(10.0)));
        gui.gui.set_style(div2, BorderLeftType(Dimension::Points(15.0)));
		gui.gui.set_style(div2, MarginLeftType(Dimension::Points(10.0)));
		gui.gui.set_style(div2, MarginTopType(Dimension::Points(10.0)));
		gui.gui.set_style(div2, BorderRadiusType(BorderRadius {
			x: [
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0)],
			y: [
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0),
				LengthUnit::Pixel(40.0)]}));
        gui.gui.append(div2, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
