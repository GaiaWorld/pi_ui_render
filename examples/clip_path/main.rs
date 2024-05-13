// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use font_kit::font::new_face_by_path;
use framework::{Param, Example};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }
use pi_style::{
    style::{Aabb2, BaseShape, BorderRadius, Center, LengthUnit, Point2},
    style_type::{
        BackgroundColorType, ClipPathType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType,
    },
};

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    root: EntityKey,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        let mut dir = std::env::current_dir().unwrap();
        log::info!("dir: {:?}", dir);
        dir.push("examples/text/source/hwkt.ttf");
        // new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
        new_face_by_path("hwkt".to_string(), "examples/text/source/SOURCEHANSANSK-MEDIUM.TTF");

        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        self.root = EntityKey(root);
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
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
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));

        self.cmd.append(root, EntityKey::null().0);


        // 添加div, 设置圆形裁剪
        let div1 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div1,
            ClipPathType(BaseShape::Circle {
                radius: LengthUnit::Pixel(20.0),
                center: Center {
                    x: LengthUnit::Percent(0.5),
                    y: LengthUnit::Percent(0.5),
                },
            }),
        );
        self.cmd.append(div1, root);


        // 添加div, 设置圆角裁剪
        let div2 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div2,
            ClipPathType(BaseShape::Inset {
                rect_box: [
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                ],
                border_radius: BorderRadius {
                    x: [
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                    ],
                    y: [
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                        LengthUnit::Pixel(5.0),
                    ],
                },
            }),
        );
        self.cmd.append(div2, root);

        // 添加div, 设置矩形裁剪
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div3,
            ClipPathType(BaseShape::Inset {
                rect_box: [
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                    LengthUnit::Pixel(10.0),
                ],
                border_radius: BorderRadius {
                    x: [
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                    ],
                    y: [
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                        LengthUnit::Pixel(0.0),
                    ],
                },
            }),
        );
        self.cmd.append(div3, root);

        // 添加div, 设置椭圆裁剪
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div3,
            ClipPathType(BaseShape::Ellipse {
                rx: LengthUnit::Percent(0.5),
                ry: LengthUnit::Percent(0.5),
                center: Center {
                    x: LengthUnit::Percent(0.5),
                    y: LengthUnit::Percent(0.5),
                },
            }),
        );
        self.cmd.append(div3, root);

        // 添加div, 设置椭圆裁剪
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div3,
            ClipPathType(BaseShape::Ellipse {
                rx: LengthUnit::Pixel(30.0),
                ry: LengthUnit::Pixel(20.0),
                center: Center {
                    x: LengthUnit::Percent(0.5),
                    y: LengthUnit::Percent(0.5),
                },
            }),
        );
        self.cmd.append(div3, root);

        // 添加div, 设置扇形裁剪
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div3,
            ClipPathType(BaseShape::Sector {
                rotate: 0.0,
                angle: 1.0 / 2.0 * 3.14,
                radius: LengthUnit::Pixel(20.0),
                center: Center {
                    x: LengthUnit::Percent(0.5),
                    y: LengthUnit::Percent(0.5),
                },
            }),
        );
        self.cmd.append(div3, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        swap(&mut self.cmd, cmd);
    }
}
