// 测试有两个根的情况

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use bevy::prelude::{World, Commands};
use framework::Example;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType},
};
use pi_ui_render::{
    components::{user::{CgColor, ClearColor, Color, RenderTargetType, Viewport}, calc::EntityKey, NodeBundle}, resource::{UserCommands, NodeCmd},
};

fn main() { framework::start(QuadExample::default()) }

pub struct QuadExample {
    root_one: EntityKey,
    root_tow: EntityKey,
	cmd: UserCommands,
}

impl Default for QuadExample {
    fn default() -> Self {
        Self {
            root_one: EntityKey::null(),
            root_tow: EntityKey::null(),
			cmd: UserCommands::default(),
        }
    }
}

#[async_trait]
impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // self.cmd.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root_one = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), false), root_one));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root_one,
        ));

        self.cmd.set_style(root_one, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root_one, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root_one, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root_one, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root_one, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root_one, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root_one, MarginTopType(Dimension::Points(0.0)));
        self.cmd.append(root_one, EntityKey::null().0);

        // 添加一个红色div到根节点
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));

        self.cmd.append(div1, root_one);
        self.root_one = EntityKey(root_one);

        // 添加根节点
        let root_tow = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root_tow));
        self.cmd.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(300.0, 300.0))), root_tow));
        self.cmd.push_cmd(NodeCmd(RenderTargetType::OffScreen, root_tow));

        self.cmd.set_style(root_tow, WidthType(Dimension::Points(300.0)));
        self.cmd.set_style(root_tow, HeightType(Dimension::Points(300.0)));

        self.cmd.set_style(root_tow, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root_tow, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root_tow, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root_tow, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root_tow, MarginTopType(Dimension::Points(0.0)));
        self.cmd.append(root_tow, EntityKey::null().0);

        // 添加一个绿红色div到根节点
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
		self.cmd.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));

        self.cmd.append(div1, root_tow);

        self.root_tow = EntityKey(root_tow);
    }

    fn render(&mut self, _cmd: &mut UserCommands, _cmd1: &mut Commands) {
        // if !self.root_tow.is_null() {
        //     if let Some(r) = self.cmd.get_graph_node_id(self.root_tow.clone()) {
        //         let canvas = world.spawn(NodeBundle::default()).id();
        //         self.cmd.set_style(canvas, PositionTypeType(PositionType::Absolute));
        //         self.cmd.set_style(canvas, WidthType(Dimension::Points(300.0)));
        //         self.cmd.set_style(canvas, HeightType(Dimension::Points(300.0)));
        //         self.cmd.set_style(canvas, PositionLeftType(Dimension::Points(100.0)));
        //         self.cmd.set_style(canvas, PositionTopType(Dimension::Points(100.0)));

        //         self.cmd.push_cmd(NodeCmd(Canvas(r), canvas));

        //         self.cmd.append(canvas, self.root_one);
        //         self.root_tow = Id::null();
        //     }
        // }
    }
}
