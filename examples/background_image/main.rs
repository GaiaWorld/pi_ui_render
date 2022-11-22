// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::{style::{Dimension, FlexWrap, PositionType}, prelude::Rect};
use pi_null::Null;
use pi_style::{
    style::{ImageRepeat, ImageRepeatOption, NotNanRect, Point2, Aabb2},
    style_type::{
        BackgroundImageType, BackgroundRepeatType, BorderRadiusType, FlexWrapType, HeightType, MarginLeftType, MarginTopType, PositionLeftType,
        PositionTopType, PositionTypeType, WidthType, BackgroundImageClipType,
    },
};
use pi_ui_render::{
    components::user::{BorderRadius, CgColor, LengthUnit, ClearColor, Viewport},
    export::Engine, utils::cmd::NodeCmd,
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Engine, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true));

        // 添加根节点
        let root = gui.gui.create_node();
		gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root));
		gui.gui.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))), root));

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
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
        );
        gui.gui.append(div1, root);

        // Repeat
        let div2 = gui.gui.create_node();
        gui.gui.set_style(div2, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div2, HeightType(Dimension::Points(50.0)));
        gui.gui.set_style(div2, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div2,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
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
        gui.gui.set_style(div3, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div3, HeightType(Dimension::Points(50.0)));
        gui.gui.set_style(div3, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div3,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
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
        gui.gui.set_style(div4, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div4, HeightType(Dimension::Points(50.0)));
        gui.gui.set_style(div4, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div4,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
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
        gui.gui.set_style(div5, WidthType(Dimension::Points(100.0)));
        gui.gui.set_style(div5, HeightType(Dimension::Points(100.0)));
        gui.gui.set_style(div5, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div5,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
        );
        gui.gui.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        gui.gui.append(div5, root);

		// imageclip
		let div6 = gui.gui.create_node();
        gui.gui.set_style(div6, WidthType(Dimension::Points(50.0)));
        gui.gui.set_style(div6, HeightType(Dimension::Points(50.0)));
        gui.gui.set_style(div6, PositionTypeType(PositionType::Relative));
        gui.gui.set_style(
            div6,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
        );
		gui.gui.set_style(
            div6,
            BackgroundImageClipType(NotNanRect(unsafe {Rect {
                top: NotNan::new_unchecked(0.0),
                right: NotNan::new_unchecked(0.5),
                bottom: NotNan::new_unchecked(0.5),
                left: NotNan::new_unchecked(0.0),
            }})),
        );
        gui.gui.append(div6, root);

		// 圆角
		let div7 = gui.gui.create_node();
        gui.gui.set_style(div7, WidthType(Dimension::Points(100.0)));
        gui.gui.set_style(div7, HeightType(Dimension::Points(100.0)));
        gui.gui.set_style(div7, PositionTypeType(PositionType::Relative));
		gui.gui.set_style(div7, BorderRadiusType(BorderRadius {
			x: [
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0)], 
			y: [
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0), 
				LengthUnit::Pixel(50.0)]}));
        gui.gui.set_style(
            div7,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
        );
        gui.gui.append(div7, root);


		// 圆角
		let div8 = gui.gui.create_node();
        gui.gui.set_style(div8, WidthType(Dimension::Points(200.0)));
        gui.gui.set_style(div8, HeightType(Dimension::Points(300.0)));
        gui.gui.set_style(div8, PositionTypeType(PositionType::Relative));
		gui.gui.set_style(
            div8,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
		gui.gui.set_style(div8, BorderRadiusType(BorderRadius {
			x: [
				LengthUnit::Pixel(180.0),
				LengthUnit::Pixel(30.0),
				LengthUnit::Pixel(180.0),
				LengthUnit::Pixel(30.0)],
			y: [
				LengthUnit::Pixel(280.0),
				LengthUnit::Pixel(20.0),
				LengthUnit::Pixel(280.0),
				LengthUnit::Pixel(20.0)]}));
        gui.gui.set_style(
            div8,
            BackgroundImageType(Atom::from("examples/background_image/source/bg_jianzhu_fszx.jpg")),
        );
        gui.gui.append(div8, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
