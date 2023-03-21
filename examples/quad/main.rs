// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy::ecs::system::Commands;
use framework::Example;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType},
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};
use pi_export_gui::Gui;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut command: Commands, _gui: &mut Gui, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root = command.spawn(NodeBundle::default()).id();
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

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div1 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));

        self.cmd.append(div1, root);

        // 添加一个红色div到红节点
        let div2 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        self.cmd.append(div2, div1);

        // 添加一个容器节点，其下有一个绿色节点，一个黄色节点， 对本节点添加TransformWillchange
        let div3 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div3, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, WidthType(Dimension::Points(90.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(150.0)));

        // 添加一个绿色div
        let div4 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div4, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div4, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div4, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.append(div4, div3);

        // 添加一个黄色
        let div5 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div5, PositionLeftType(Dimension::Points(50.0)));
        self.cmd.set_style(div5, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div5, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        // 设置opacity，测试Pass2d在父上存在TransformWillChange的情况下能否正确渲染
        // self.cmd.set_style(div5, OpacityType(Opacity(0.5)));

        self.cmd.append(div5, div3);

        self.cmd.append(div3, div1);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
