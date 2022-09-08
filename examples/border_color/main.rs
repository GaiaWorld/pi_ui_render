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
    components::user::{BorderColor, CgColor},
    export::Engine,
    resource::ClearColor,
};

use pi_style::style_type::{
    BorderBottomType, BorderColorType, BorderLeftType, BorderRightType, BorderTopType, HeightType, MarginLeftType, MarginTopType, PositionLeftType,
    PositionTopType, PositionTypeType, WidthType,
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

        gui.gui.append(root, Id::null());

        // 添加一个红色div
        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, WidthType(Dimension::Points(110.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(144.0)));
        gui.gui.set_style(div1, BorderColorType(BorderColor(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        gui.gui.set_style(div1, BorderTopType(Dimension::Points(10.0)));
        gui.gui.set_style(div1, BorderRightType(Dimension::Points(10.0)));
        gui.gui.set_style(div1, BorderBottomType(Dimension::Points(10.0)));
        gui.gui.set_style(div1, BorderLeftType(Dimension::Points(10.0)));
        gui.gui.append(div1, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
