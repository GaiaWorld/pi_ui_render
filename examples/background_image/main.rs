// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, FlexWrap, PositionType};
use pi_null::Null;
use pi_style::{
    style::{ImageRepeat, ImageRepeatOption},
    style_type::{
        BackgroundImageType, BackgroundRepeatType, BorderRadiusType, FlexWrapType, HeightType, MarginLeftType, MarginTopType, PositionLeftType,
        PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::user::{BackgroundImage, BorderRadius, CgColor, LengthUnit},
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
        gui.gui.set_style(root, FlexWrapType(FlexWrap::Wrap));

        gui.gui.append(root, Id::null());

        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(100.0)));
        gui.gui.set_style(div1, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div1,
            BackgroundImageType(BackgroundImage(Atom::from("examples/background_image/source/dialog_bg.png"))),
        );
        gui.gui.set_style(
            div1,
            BorderRadiusType(BorderRadius {
                x: LengthUnit::Pixel(10.0),
                y: LengthUnit::Pixel(10.0),
            }),
        );
        gui.gui.append(div1, root);

        // Repeat
        let div2 = gui.gui.create_node();
        gui.gui.set_style(div2, WidthType(Dimension::Points(190.0)));
        gui.gui.set_style(div2, HeightType(Dimension::Points(160.0)));
        gui.gui.set_style(div2, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div2,
            BackgroundImageType(BackgroundImage(Atom::from("examples/background_image/source/dialog_bg1.png"))),
        );
        gui.gui.set_style(
            div2,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        gui.gui.append(div2, root);

        // Round
        let div3 = gui.gui.create_node();
        gui.gui.set_style(div3, WidthType(Dimension::Points(190.0)));
        gui.gui.set_style(div3, HeightType(Dimension::Points(160.0)));
        gui.gui.set_style(div3, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div3,
            BackgroundImageType(BackgroundImage(Atom::from("examples/background_image/source/dialog_bg1.png"))),
        );
        gui.gui.set_style(
            div3,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        gui.gui.append(div3, root);

        // space
        let div4 = gui.gui.create_node();
        gui.gui.set_style(div4, WidthType(Dimension::Points(190.0)));
        gui.gui.set_style(div4, HeightType(Dimension::Points(160.0)));
        gui.gui.set_style(div4, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div4,
            BackgroundImageType(BackgroundImage(Atom::from("examples/background_image/source/dialog_bg1.png"))),
        );
        gui.gui.set_style(
            div4,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        gui.gui.append(div4, root);

        // space
        let div5 = gui.gui.create_node();
        gui.gui.set_style(div5, WidthType(Dimension::Points(300.0)));
        gui.gui.set_style(div5, HeightType(Dimension::Points(300.0)));
        gui.gui.set_style(div5, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div5,
            BackgroundImageType(BackgroundImage(Atom::from("examples/background_image/source/dialog_bg1.png"))),
        );
        gui.gui.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        gui.gui.append(div5, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
