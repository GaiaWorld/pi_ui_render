// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::user::{
         BackgroundImage, BorderRadius, CgColor, LengthUnit,
    },
    gui::Gui,
    resource::ClearColor,
    utils::style::style_sheet::{BackgroundImageType, BorderRadiusType, HeightType, MarginLeftType, MarginTopType,
        PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};

fn main() {
    framework::start(QuadExample::default())
}

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Gui, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        gui.world_mut()
            .insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

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
        gui.set_style(div1, WidthType(Dimension::Points(50.0)));
        gui.set_style(div1, HeightType(Dimension::Points(100.0)));
        // gui.set_style(div1, BackgroundColorType (BackgroundColor(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0)) )));

        gui.set_style(
            div1,
            BackgroundImageType(BackgroundImage(Atom::from(
                "examples/background_image/source/dialog_bg.png",
            ))),
        );
        gui.set_style(
            div1,
            BorderRadiusType(BorderRadius {
                x: LengthUnit::Pixel(10.0),
                y: LengthUnit::Pixel(10.0),
            }),
        );

        gui.append(div1, root);
    }

    fn render(&mut self, gui: &mut Gui) {
        gui.run();
    }
}
