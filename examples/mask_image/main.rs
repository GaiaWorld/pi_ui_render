// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::system::Commands;
use bevy_ecs::prelude::World;
use font_kit::font::new_face_by_path;
use framework::Example;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};

fn main() { framework::start(QuadExample::default()) }
use pi_style::{
    style::{Aabb2, ColorAndPosition, LinearGradientColor, MaskImage, Point2},
    style_type::{
        BackgroundColorType, HeightType, MarginLeftType, MarginTopType, MaskImageType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    root: EntityKey,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        let mut dir = std::env::current_dir().unwrap();
        log::info!("dir: {:?}", dir);
        dir.push("examples/text/source/hwkt.ttf");
        // new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
        new_face_by_path("hwkt".to_string(), "examples/text/source/SOURCEHANSANSK-MEDIUM.TTF");

        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
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

        self.cmd.append(root, EntityKey::null().0);

        // 添加div, 设置渐变遮罩
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(
            div1,
            MaskImageType(MaskImage::LinearGradient(LinearGradientColor {
                direction: 0.0,
                list: vec![
                    ColorAndPosition {
                        position: 0.0,
                        rgba: CgColor::new(0.0, 0.0, 0.0, 1.0),
                    },
                    ColorAndPosition {
                        position: 1.0,
                        rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
                    },
                ],
            })),
        );
        self.cmd.append(div1, root);

        // 为遮罩节点添加子节点，并设置颜色
        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.append(div2, div1);

        // 为遮罩节点添加子节点，并设置颜色
        let div3 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div3, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 1.0, 1.0))));
        self.cmd.set_style(
            div3,
            MaskImageType(MaskImage::LinearGradient(LinearGradientColor {
                direction: 0.5 * 3.14,
                list: vec![
                    ColorAndPosition {
                        position: 0.0,
                        rgba: CgColor::new(0.0, 0.0, 0.0, 1.0),
                    },
                    ColorAndPosition {
                        position: 0.5,
                        rgba: CgColor::new(0.5, 0.0, 0.0, 1.0),
                    },
                    ColorAndPosition {
                        position: 1.0,
                        rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
                    },
                ],
            })),
        );
        self.cmd.set_style(
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
        self.cmd.append(div3, root);

        // background:linear-gradient(0deg,#ff0000,#00ff00);mask-image-source:linear-gradient(90deg, #000000, #777777 50%, #ffffff);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) {
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        swap(&mut self.cmd, cmd);
    }
}
