// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use async_trait::async_trait;
use bevy::prelude::Commands;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, AnimationDirection, AnimationName, IterationCount, Time},
    style_parse::parse_class_map_from_string,
    style_type::{
        AnimationDirectionType, AnimationDurationType, AnimationIterationCountType, AnimationNameType, BackgroundColorType, HeightType,
        MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::{user::{CgColor, ClearColor, Color, Point2, Viewport, RenderDirty}, calc::EntityKey, NodeBundle},
    resource::{UserCommands, NodeCmd, ExtendCssCmd},
};
use pi_export_gui::Gui;
use smallvec::smallvec;

fn main() { framework::start(AnimationExample::default()) }

#[derive(Default)]
pub struct AnimationExample {cmd: UserCommands}
#[async_trait]
impl Example for AnimationExample {
    fn init(&mut self, mut command: Commands, _gui: &mut Gui, size: (usize, usize)) {
        // 添加keyframes
        let css = "@keyframes test-animation {
			0% {transform: scale(1.0, 1.0);}
			50% {transform: scale(2.0, 2.0);}
			100% {transform: scale(1.0, 1.0);}
		}";
        let class_map = parse_class_map_from_string(css, 0).unwrap();
		self.cmd.push_cmd(ExtendCssCmd(vec![class_map]));

        // 设置清屏颜色为绿色
        // self.cmd.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

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

		let div1 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        self.cmd.set_style(
            div1,
            AnimationNameType(AnimationName {
                scope_hash: 0,
                value: smallvec![Atom::from("test-animation")],
            }),
        );
        self.cmd.set_style(div1, AnimationIterationCountType(smallvec![IterationCount(10000000.0)]));
        self.cmd.set_style(div1, AnimationDirectionType(smallvec![AnimationDirection::Reverse]));
        self.cmd.set_style(div1, AnimationDurationType(smallvec![Time(3000)]));
        self.cmd.append(div1, root);

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div2 = command.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        self.cmd.append(div2, div1);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
