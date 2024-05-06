// // 一个简单的四边形渲染

// #[path = "../framework.rs"]
// mod framework;

// use std::mem::swap;

// use pi_world::prelude::World;

// use framework::{Param, Example};
// use ordered_float::NotNan;
// use pi_flex_layout::style::{Dimension, PositionType};
// use pi_hal::pi_sdf::shape::PathVerb;
// use pi_null::Null;
// use pi_ui_render::{
//     components::{
//         calc::EntityKey,
//         user::{CgColor, ClearColor, Color, RenderDirty, Viewport, SvgGradient},
// 
//     },
//     resource::{NodeCmd, Shape, SvgColorCmd, SvgShapeCmd, UserCommands, StrokeDasharrayCmd, SvgStrokeColorCmd, SvgStrokeWidthCmd, SvgShapeWidthCmd, SvgShapeHeightCmd, SvgShapeXCmd, SvgShapeYCmd, SvgWidthCmd, SvgHeightCmd, SvgShadowColorCmd, SvgShadowOffsetXCmd, SvgShadowOffsetYCmd, SvgShadowBlurLevelCmd, SvgFilterOffsetXCmd, SvgFilterOffsetYCmd, SvgFilterBlurLevelCmd, SvgFilterColorCmd, SvgFilterCmd, SvgGradientCmd, SvgGradientX1Cmd, SvgGradientY1Cmd, SvgGradientY2Cmd, SvgGradientX2Cmd, SvgStopOffsetCmd, SvgStopColorCmd},
// };
// use pi_style::{
//     style::{Aabb2, Point2, Stroke, StrokeDasharray, LinearGradientColor, ColorAndPosition},
//     style_type::{
//         AsImageType, BackgroundColorType, HeightType,
//         MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType,
//         WidthType,
//     },
// };

// fn main() { framework::start(QuadExample::default()) }


// #[derive(Default)]
// pub struct QuadExample {
//     cmd: UserCommands,
//     root: EntityKey,
// }

// impl Example for QuadExample {
//     #[rustfmt::skip]
//     fn init(&mut self, mut world: Param, size: (usize, usize)) {
//         // 添加根节点
//         let root = world.spawn();
//         self.root = EntityKey(root);
//         self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
//         self.cmd.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))), root));
//         self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));

//         self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
//         self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

//         self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
//         self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
//         self.cmd.set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 1.0, 1.0))));
//         self.cmd.append(root, EntityKey::null().0);


//         // // 矩形
//         let div1 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div1, Shape::from(0.0)));
       
//         self.cmd.push_cmd(SvgStrokeColorCmd(div1, CgColor::new(0., 1., 0., 1.)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div1, NotNan::new(1.0).unwrap()));
//         self.cmd.push_cmd(SvgShapeWidthCmd(div1, 100.0));
//         self.cmd.push_cmd(SvgShapeHeightCmd(div1, 50.0));
//         self.cmd.push_cmd(SvgShapeXCmd(div1, 20.0));
//         self.cmd.push_cmd(SvgShapeYCmd(div1, 20.0));
//         self.cmd.append(div1, root);
        
//         // 圆
//         let div2 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div2, Shape::Circle {cx: 200.0, cy: 60.0, radius: 40.0},));
//         self.cmd.push_cmd(SvgColorCmd(div2, Color::LinearGradient(LinearGradientColor {
//             direction: 0.5 * 3.14,
//             list: vec![
//                 ColorAndPosition {
//                     position: 0.0,
//                     rgba: CgColor::new(0.0, 0.0, 0.0, 1.0),
//                 },
//                 ColorAndPosition {
//                     position: 0.5,
//                     rgba: CgColor::new(0.3, 0.0, 0.0, 1.0),
//                 },
//                 ColorAndPosition {
//                     position: 1.0,
//                     rgba: CgColor::new(0.6, 0.0, 0.0, 1.0),
//                 },
//                 ColorAndPosition {
//                     position: 1.0,
//                     rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
//                 },
//             ],
//         })));
//         self.cmd.push_cmd(SvgStrokeColorCmd(div2, CgColor::new(0., 0., 0., 1.)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div2, NotNan::new(2.0).unwrap()));
//         self.cmd.append(div2, div1);

//         //椭圆
//         let div3 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div3, Shape::Ellipse {cx: 320.0, cy: 60.0, rx: 50.0, ry: 25.0 },));
//         self.cmd.push_cmd(SvgColorCmd(div3, Color::RGBA(CgColor::new(1., 0., 0., 1.)) ));
//         self.cmd.push_cmd(SvgStrokeColorCmd(div3,  CgColor::new(0., 0., 0., 1.0)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div3, NotNan::new(2.0).unwrap()));
//         self.cmd.append(div3, root);

//         // // 线段
//         let div4 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div4, Shape::Segment {ax: 220.0, ay: 300.0, bx: 20.0, by: 100.0 },));
//         self.cmd.push_cmd(SvgColorCmd(div4, Color::RGBA(CgColor::new(0., 0., 0., 0.))));
//         self.cmd.push_cmd(SvgStrokeColorCmd(div4,  CgColor::new(0., 0., 0., 1.0)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div4, NotNan::new(3.0).unwrap()));
//         self.cmd.append(div4, root);

//         // 多边形
//         let div5 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div5, Shape::Polygon {points: vec![
//             [270.0, 110.0],
//             [350.0, 170.0],
//             [320.0, 220.0],
//             [220.0, 210.0],
//             [200.0, 160.0]
//         ]}));
//         self.cmd.push_cmd(SvgColorCmd(div5, Color::RGBA(CgColor::new(0., 1., 0., 1.))) );
//         self.cmd.push_cmd(SvgStrokeColorCmd(div5,  CgColor::new(0., 0., 0., 1.0)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div5, NotNan::new(2.0).unwrap()));
//         self.cmd.append(div5, root);

//         // 多段线
//         let div6 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div6, Shape::Polyline { 
//             points: vec![
//             [20., 220.],
//             [40., 225.],
//             [60., 240.],
//             [80., 320.],
//             [120., 340.],
//             [180., 320.],
//         ]}));
//         self.cmd.push_cmd(SvgColorCmd(div6, Color::RGBA(CgColor::new(0., 0., 0., 0.)) ));
//         // self.cmd.push_cmd(SvgStrokeCmd(div5, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
//         self.cmd.push_cmd(SvgStrokeColorCmd(div6,  CgColor::new(0., 0., 0., 1.0)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div6, NotNan::new(2.0).unwrap()));
//         self.cmd.append(div6, root);

//         // 贝塞尔曲线
//         let div7 = world.spawn();
//         self.cmd.push_cmd(SvgShapeCmd(div7, Shape::Path { 
//             points: vec![
//                 [210., 30.],
//                 [210., 250.],
//                 [25., 190.],
//                 [110., 150.],
//             ], 
//             verb: vec![
//                 PathVerb::MoveTo, 
//                 PathVerb::CubicTo
//             ]
//         }));
//         self.cmd.push_cmd(SvgColorCmd(div7, Color::RGBA(CgColor::new(0., 1., 0., 0.))));
//         self.cmd.push_cmd(SvgStrokeColorCmd(div7, CgColor::new(0., 1., 1., 1.)));
//         self.cmd.push_cmd(SvgStrokeWidthCmd(div7, NotNan::new(2.0).unwrap()));
//         self.cmd.append(div7, root);

//         // 虚线(仅支持直线)
//         // let div8 = world.spawn();
//         // self.cmd.push_cmd(SvgShapeCmd(div8, Shape::Path { 
//         //     points: vec![
//         //         [10., 390.],
//         //         [215., 0.],
//         //     ], 
//         //     verb: vec![
//         //         PathVerb::MoveTo, 
//         //         PathVerb::LineToRelative
//         //     ]
//         // }));
//         // self.cmd.push_cmd(SvgColorCmd(div8, Color::RGBA(CgColor::new(0., 0., 0., 0.)) ));
//         // // self.cmd.push_cmd(SvgStrokeCmd(div6, Stroke {width: NotNan::new(2.0).unwrap(), color: CgColor::new(0., 0., 0., 1.)}));
//         // self.cmd.push_cmd(SvgStrokeColorCmd(div8, CgColor::new(0., 0., 0., 1.)));
//         // self.cmd.push_cmd(SvgStrokeWidthCmd(div8, NotNan::new(2.0).unwrap()));
//         // self.cmd.push_cmd(StrokeDasharrayCmd(div8,  StrokeDasharray{start_x: 10.0,  start_y: 390.0, /* 路径的第一个点 */ real: 20.0, empty: 10.0 }));
//         // self.cmd.append(div8, root);

//         let div9 = world.spawn();
//         // self.cmd.push_cmd(SvgFilterCmd(div9, div4));
//         self.cmd.push_cmd(SvgFilterCmd(div9, div1));
//         self.cmd.append(div9, root);

//         let div10 = world.spawn();
//         self.cmd.push_cmd(SvgFilterOffsetXCmd(div10, 20.0));
//         self.cmd.push_cmd(SvgFilterOffsetYCmd(div10, 40.0));
//         self.cmd.push_cmd(SvgFilterColorCmd(div10, 0.0));
//         self.cmd.append(div10, div9);

//         let div11 = world.spawn();
//         self.cmd.push_cmd(SvgFilterBlurLevelCmd(div11, 3.0));
//         self.cmd.append(div11, div9);

//         let div12 = world.spawn();
//         self.cmd.push_cmd(SvgGradientCmd(div12, div1));
//         self.cmd.push_cmd(SvgGradientX1Cmd(div12, 0.0));
//         self.cmd.push_cmd(SvgGradientY1Cmd(div12, 0.0));
//         self.cmd.push_cmd(SvgGradientX2Cmd(div12, 100.0));
//         self.cmd.push_cmd(SvgGradientY2Cmd(div12, 0.0));
//         self.cmd.append(div12, root);

//         let div13 = world.spawn();
//         self.cmd.push_cmd(SvgStopOffsetCmd(div13, 0.0));
//         self.cmd.push_cmd(SvgStopColorCmd(div13, CgColor::new(1., 1., 0., 1.)));
//         self.cmd.append(div13, div12);

//         let div14 = world.spawn();
//         self.cmd.push_cmd(SvgStopOffsetCmd(div14, 100.0));
//         self.cmd.push_cmd(SvgStopColorCmd(div14, CgColor::new(1., 0., 0., 1.)));
//         self.cmd.append(div14, div12);
//     }

//     fn render(&mut self, cmd: &mut UserCommands) {
//         self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
//         swap(&mut self.cmd, cmd);
//     }
// }


fn main() {}