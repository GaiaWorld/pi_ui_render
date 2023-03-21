// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy::ecs::system::Commands;
use framework::Example;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};

use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        BorderBottomType, BorderColorType, BorderLeftType, BorderRightType, BorderTopType, HeightType, MarginLeftType, MarginTopType,
        PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut command: Commands, _gui: &mut Gui, size: (usize, usize)) {
        // 添加根节点
        let root = self.cmd.spawn(NodeBundle::default()).id();
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

        // 添加一个红色div
        let div1 = self.cmd.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(110.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(144.0)));
        self.cmd.set_style(div1, BorderColorType(CgColor::new(1.0, 1.0, 0.0, 1.0)));
        self.cmd.set_style(div1, BorderTopType(Dimension::Points(10.0)));
        self.cmd.set_style(div1, BorderRightType(Dimension::Points(10.0)));
        self.cmd.set_style(div1, BorderBottomType(Dimension::Points(10.0)));
        self.cmd.set_style(div1, BorderLeftType(Dimension::Points(10.0)));
        self.cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
