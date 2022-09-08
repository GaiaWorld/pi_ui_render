// 半透明渲染

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::Hsi,
    style_type::{
        BackgroundColorType, HeightType, HsiType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::user::{BackgroundColor, CgColor, Color},
    export::Engine,
    resource::ClearColor,
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Engine, size: (usize, usize)) {
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
        gui.gui
            .set_style(div1, BackgroundColorType(BackgroundColor(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0)))));
        gui.gui.append(div1, root);


        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, PositionTopType(Dimension::Points(100.0)));
        gui.gui.set_style(div1, WidthType(Dimension::Points(100.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(200.0)));
        gui.gui.set_style(
            div1,
            HsiType(Hsi {
                hue_rotate: 0.0,
                saturate: -1.0,
                bright_ness: 0.0,
            }),
        );
        // gui.gui.set_style(div1, OpacityType(Opacity(0.5)));

        // 添加一个绿色div
        let div2 = gui.gui.create_node();
        gui.gui.set_style(div2, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div2, HeightType(Dimension::Points(100.0)));
        gui.gui
            .set_style(div2, BackgroundColorType(BackgroundColor(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0)))));
        gui.gui.append(div2, div1);

        // 添加一个黄色
        let div3 = gui.gui.create_node();
        gui.gui.set_style(div3, PositionTopType(Dimension::Points(100.0)));
        gui.gui.set_style(div3, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div3, HeightType(Dimension::Points(100.0)));
        gui.gui
            .set_style(div3, BackgroundColorType(BackgroundColor(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0)))));
        gui.gui.append(div3, div1);

        gui.gui.append(div1, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
