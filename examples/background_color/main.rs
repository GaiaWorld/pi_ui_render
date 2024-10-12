// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;


use framework::{Param, Example};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, BorderRadius, LengthUnit, Point2},
    style_type::{AsImageType, BackgroundColorType, BorderRadiusType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType},
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, ColorAndPosition, LinearGradientColor, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), root));

        world.user_cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));

        world.user_cmd.append(root, EntityKey::null().0);

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        world.user_cmd.append(div1, root);

        // 添加一个渐变颜色
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(
            div1,
            BackgroundColorType(Color::LinearGradient(LinearGradientColor {
                direction: 0.0,
                list: vec![
                    ColorAndPosition {
                        position: 0.0,
                        rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
                    },
                    ColorAndPosition {
                        position: 1.0,
                        rgba: CgColor::new(0.0, 1.0, 0.0, 1.0),
                    },
                ],
            })),
        );
        world.user_cmd.append(div1, root);

        // 添加一个红色div
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(110.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(144.0)));
        world.user_cmd.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        world.user_cmd.set_style(div1, BorderRadiusType(BorderRadius {
            x: [LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0)],
            y: [LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0), LengthUnit::Pixel(10.0)],
        }));
        world.user_cmd.append(div1, root);

        // 添加一个红色div
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(110.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(144.0)));
        world.user_cmd.set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        world.user_cmd.set_style(div2, BorderRadiusType(BorderRadius {
            x: [LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0)],
            y: [LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0)],
        }));
        world.user_cmd.append(div2, root);

        // 添加一个渐变颜色
        let div3 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(
            div3,
            BackgroundColorType(Color::LinearGradient(LinearGradientColor {
                direction: 0.0,
                list: vec![
                    ColorAndPosition {
                        position: 0.0,
                        rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
                    },
                    ColorAndPosition {
                        position: 1.0,
                        rgba: CgColor::new(0.0, 1.0, 0.0, 1.0),
                    },
                ],
            })),
        );
        world.user_cmd.set_style(div3, BorderRadiusType(BorderRadius {
            x: [LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0)],
            y: [LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0), LengthUnit::Pixel(20.0)],
        }));
        world.user_cmd.append(div3, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
