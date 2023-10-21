// 测试AsImage, 上下文中的节点发生改变时， 上下文节点使用原来的fbo进行脏更渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::system::Commands;
use bevy_ecs::prelude::World;
use framework::Example;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        AsImageType, BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, RotateType,
        WidthType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};

fn main() { framework::start(QuadExample::default()) }


#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    change_node: EntityKey,
    rotate: f32,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
        self.cmd.append(root, EntityKey::null().0);

        // 添加一个红色div到根节点， 并设置AsImage为Force
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(200.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        self.cmd.set_style(div1, AsImageType(pi_style::style::AsImage::Force));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        self.cmd.set_style(div1, RotateType(30.0));
        self.cmd.append(div1, root);

        // 添加一个绿色div到div2
        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.append(div2, div1);
        self.change_node = EntityKey(div2);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) {
        // 不停的转动change_node
        self.rotate += 1.0;
        self.cmd.set_style(self.change_node.0, RotateType(self.rotate));
        swap(&mut self.cmd, cmd);
    }
}
