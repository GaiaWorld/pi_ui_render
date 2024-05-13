// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;


use framework::{Param, Example};
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::{
    prelude::Rect,
    style::{Dimension, FlexWrap, PositionType},
};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, BorderRadius, CgColor, ImageRepeat, ImageRepeatOption, NotNanRect, Point2, Color},
    style_type::{
        BackgroundImageClipType, BackgroundImageType, BackgroundRepeatType, BorderRadiusType, FlexWrapType, HeightType, MarginLeftType,
        MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType, BackgroundColorType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{ClearColor, LengthUnit, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
	web_sys::console::log_1(&"background_image===========".into());
	framework::start(QuadExample::default());
}

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));
        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, FlexWrapType(FlexWrap::Wrap));
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd.append(root, EntityKey::null().0);

        let div1 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        self.cmd.set_style(
            div1,
            BorderRadiusType(BorderRadius {
                x: [
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                ],
                y: [
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                ],
            }),
        );
        self.cmd.append(div1, root);

        // Repeat x轴空间超过一倍但小于两倍， y轴空间不足一倍
        let div2 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div2, WidthType(Dimension::Points(150.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(80.0)));
        self.cmd.set_style(div2, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div2, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
		self.cmd
			.set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.set_style(
            div2,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div2, root);

		// Repeat, x轴空间超过两倍， 但是偶数倍
		let div5 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div5, WidthType(Dimension::Points(250.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(80.0)));
        self.cmd.set_style(div5, PositionTypeType(PositionType::Relative));
		self.cmd.set_style(div5, PositionTopType(Dimension::Points(10.0)));
        self.cmd
            .set_style(div5, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
		self.cmd
            .set_style(div5, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div5, root);

		// Repeat, x轴空间超过两倍， 但是奇数数倍
		let div5 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div5, WidthType(Dimension::Points(350.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(80.0)));
        self.cmd.set_style(div5, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div5, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
		self.cmd
            .set_style(div5, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div5, root);

        // Round TODO
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, WidthType(Dimension::Points(190.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(80.0)));
        self.cmd.set_style(div3, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div3, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        self.cmd.set_style(
            div3,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        self.cmd.append(div3, root);

        // space
        let div4 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div4, WidthType(Dimension::Points(190.0)));
        self.cmd.set_style(div4, HeightType(Dimension::Points(160.0)));
        self.cmd.set_style(div4, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div4, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        self.cmd.set_style(
            div4,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        self.cmd.append(div4, root);

        // space， x, y空间超过两倍
        let div5 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div5, WidthType(Dimension::Points(300.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(300.0)));
        self.cmd.set_style(div5, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div5, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        self.cmd.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        self.cmd.append(div5, root);

        // imageclip
        let div6 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div6, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div6, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div6, PositionTypeType(PositionType::Relative));
        self.cmd
            .set_style(div6, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        self.cmd.set_style(
            div6,
            BackgroundImageClipType(NotNanRect(unsafe {
                Rect {
                    top: NotNan::new_unchecked(0.0),
                    right: NotNan::new_unchecked(0.5),
                    bottom: NotNan::new_unchecked(0.5),
                    left: NotNan::new_unchecked(0.0),
                }
            })),
        );
        self.cmd.append(div6, root);

        // 圆角
        let div7 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div7, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div7, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div7, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(
            div7,
            BorderRadiusType(BorderRadius {
                x: [
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                ],
                y: [
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                    LengthUnit::Pixel(50.0),
                ],
            }),
        );
        self.cmd
            .set_style(div7, BackgroundImageType(Atom::from("examples/z_source/3675173.jpg")));
        self.cmd.append(div7, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) { swap(&mut self.cmd, cmd); }
}
