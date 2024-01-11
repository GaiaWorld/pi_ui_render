// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::system::Commands;
use bevy_ecs::prelude::World;
use font_kit::font::new_face_by_path;
use framework::Example;
use pi_atom::Atom;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, FontSize, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};
use smallvec::smallvec;

fn main() { framework::start(QuadExample::default()) }
use pi_style::{
    style::{Aabb2, Point2, TextContent, TextShadow},
    style_type::{
        BackgroundColorType, ColorType, FontFamilyType, FontSizeType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
        PositionTypeType, TextContentType, TextShadowType, WidthType, AsImageType,
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
        dir.push("examples/z_source/hwkt.ttf");
        // new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
        new_face_by_path("hwkt".to_string(), "examples/z_source/SOURCEHANSANSK-MEDIUM.TTF");

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
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd
            .set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 1.0, 1.0))));

        self.cmd.append(root, EntityKey::null().0);

        // 添加一个红色div
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, PositionTopType(Dimension::Points(20.0)));
        self.cmd.set_style(div1, PositionLeftType(Dimension::Points(20.0)));
        self.cmd
            .set_style(div1, TextContentType(TextContent("base02".to_string(), Atom::from("base02"))));
        self.cmd.set_style(
            div1,
            TextShadowType(smallvec![
                TextShadow {
                    h: 0.0,
                    v: 0.0,
                    blur: 5.0,
                    color: CgColor::new(1.0, 0.0, 0.0, 1.0)
                },
                TextShadow {
                    h: 0.0,
                    v: 0.0,
                    blur: 3.0,
                    color: CgColor::new(1.0, 0.0, 0.0, 1.0)
                },
            ]),
        );
		self.cmd.set_style(
            div1,
            TextShadowType(smallvec![
                TextShadow {
                    h: 0.0,
                    v: 0.0,
                    blur: 5.0,
                    color: CgColor::new(1.0, 0.0, 0.0, 1.0)
                },
                TextShadow {
                    h: 0.0,
                    v: 0.0,
                    blur: 3.0,
                    color: CgColor::new(1.0, 0.0, 0.0, 1.0)
                },
            ]),
        );
        // rgb(255,0,0) 0px 0px 5px, rgb(255,0,0) 0px 0px 3px, rgb(255,255,255) 0px 0px 1px;
        self.cmd.set_style(div1, FontFamilyType(Atom::from("hwkt")));
        self.cmd.set_style(div1, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.set_style(div1, FontSizeType(FontSize::Length(17)));
        // self.cmd.set_style(div1, TextStrokeType(Stroke {
        // 	width: unsafe {NotNan::new_unchecked(2.0)},
        // 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
        self.cmd.append(div1, root);

        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, TextContentType(TextContent("base1".to_string(), Atom::from("base1"))));
        self.cmd.set_style(div2, FontFamilyType(Atom::from("hwkt")));
        self.cmd.set_style(div2, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.set_style(div2, FontSizeType(FontSize::Length(17)));
        // self.cmd.set_style(div1, TextStrokeType(Stroke {
        // 	width: unsafe {NotNan::new_unchecked(2.0)},
        // 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
        self.cmd.append(div2, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) {
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        swap(&mut self.cmd, cmd);
    }
}
