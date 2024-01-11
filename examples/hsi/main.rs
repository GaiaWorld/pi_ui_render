// 半透明渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::system::Commands;
use bevy_ecs::prelude::World;
use framework::{spawn, Example};
use pi_atom::Atom;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Hsi, Point2},
    style_type::{
        BackgroundColorType, BackgroundImageType, HeightType, HsiType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
        PositionTypeType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{UserCommands, NodeCmd},
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.set_view_port(
            root,
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
        );
        self.cmd.set_render_dirty(root, RenderDirty(true));

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd
            .set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 0.0, 0.0, 1.0))));
        self.cmd.append(root, EntityKey::null().0);

        // 添加一个红色div
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        self.cmd.append(div1, root);


        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        self.cmd.set_style(
            div1,
            HsiType(Hsi {
                hue_rotate: 0.0,
                saturate: 0.0,
                bright_ness: 0.2,
            }),
        );
        // self.cmd.set_style(div1, OpacityType(0.5));
        // self.cmd.set_style(div1, OpacityType(Opacity(0.5)));

        // 添加一个绿色div
        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.append(div2, div1);

        // 添加一个黄色
        let div3 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div3, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, BackgroundImageType(Atom::from("examples/hsi/source/men.png")));
        // self.cmd
        // .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        self.cmd.append(div3, div1);

        self.cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
