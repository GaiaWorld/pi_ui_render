// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::prelude::World;
use bevy_ecs::system::Commands;
use framework::Example;
use ordered_float::NotNan;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_hal::pi_sdf::shape::PathVerb;
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, Shape, SvgColorCmd, SvgShapeCmd, SvgStrokeCmd, UserCommands, StrokeDasharrayCmd},
};
use pi_style::{
    style::{Aabb2, Point2, Stroke, StrokeDasharray},
    style_type::{
        AsImageType, BackgroundColorType, HeightType,
        MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType,
        WidthType,
    },
};

fn main() { framework::start(QuadExample::default()) }


#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    root: EntityKey,
}

impl Example for QuadExample {
    #[rustfmt::skip]
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
        self.root = EntityKey(root);
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))), root));
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd.set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 1.0, 1.0))));
        self.cmd.append(root, EntityKey::null().0);

        // 矩形
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div1, Shape::Rect {x: 120.0, y: 70.0, width: -100.0, height: -50.0}));
        self.cmd.push_cmd(SvgColorCmd(div1, CgColor::new(0., 0., 1., 1.)));
        self.cmd.push_cmd(SvgStrokeCmd(div1, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div1, root);

        // 圆
        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div2, Shape::Circle {cx: 200.0, cy: 60.0, radius: 40.0},));
        self.cmd.push_cmd(SvgColorCmd(div2, CgColor::new(0., 0., 1., 1.)));
        self.cmd.push_cmd(SvgStrokeCmd(div2, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div2, root);

         // 椭圆
        let div3 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div3, Shape::Ellipse {cx: 320.0, cy: 60.0, rx: 50.0, ry: 25.0 },));
        self.cmd.push_cmd(SvgColorCmd(div3, CgColor::new(1., 0., 0., 1.)));
        self.cmd.push_cmd(SvgStrokeCmd(div3, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div3, root);

        // 线段
        let div4 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div4, Shape::Segment {ax: 20.0, ay: 100.0, bx: 120.0, by: 180.0 },));
        self.cmd.push_cmd(SvgColorCmd(div4, CgColor::new(0., 0., 0., 0.)));
        self.cmd.push_cmd(SvgStrokeCmd(div4, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div4, root);

         // 多边形
        let div4 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div4, Shape::Polygon {points: vec![
            [270.0, 110.0],
            [350.0, 170.0],
            [320.0, 220.0],
            [220.0, 210.0],
            [200.0, 160.0]
        ]}));
        self.cmd.push_cmd(SvgColorCmd(div4, CgColor::new(0., 1., 0., 1.)));
        self.cmd.push_cmd(SvgStrokeCmd(div4, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div4, root);

        // 多段线
        let div5 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div5, Shape::Polyline { 
            points: vec![
            [20., 220.],
            [40., 225.],
            [60., 240.],
            [80., 320.],
            [120., 340.],
            [180., 320.],
        ]}));
        self.cmd.push_cmd(SvgColorCmd(div5, CgColor::new(0., 0., 0., 0.)));
        self.cmd.push_cmd(SvgStrokeCmd(div5, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.append(div5, root);

        // 贝塞尔曲线
        let div6 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div6, Shape::Path { 
            points: vec![
                [210., 30.],
                [210., 250.],
                [25., 190.],
                [110., 150.],
            ], 
            verb: vec![
                PathVerb::MoveTo, 
                PathVerb::CubicTo
            ]
        }));
        self.cmd.push_cmd(SvgColorCmd(div6, CgColor::new(0., 1., 0., 0.)));
        self.cmd.push_cmd(SvgStrokeCmd(div6, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 1., 1., 1.)}));
        self.cmd.append(div6, root);

        // 虚线(仅支持直线)
        let div6 = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(SvgShapeCmd(div6, Shape::Path { 
            points: vec![
                [10., 390.],
                [215., 0.],
            ], 
            verb: vec![
                PathVerb::MoveTo, 
                PathVerb::LineToRelative
            ]
        }));
        self.cmd.push_cmd(SvgColorCmd(div6, CgColor::new(0., 0., 0., 0.)));
        self.cmd.push_cmd(SvgStrokeCmd(div6, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
        self.cmd.push_cmd(StrokeDasharrayCmd(div6,  StrokeDasharray{start_x: 10.0,  start_y: 390.0, /* 路径的第一个点 */ real: 20.0, empty: 10.0 }));
        self.cmd.append(div6, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) {
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        swap(&mut self.cmd, cmd);
    }
}
