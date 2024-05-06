// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use framework::{Example, Param};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, AnimationTimingFunction, Time, StyleType},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
        PositionTypeType, WidthType, TransitionPropertyType, TransitionTimingFunctionType, TransitionDurationType, TransitionDelayType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, Point2, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use smallvec::smallvec;

fn main() { framework::start(TransitionExample::default()) }

#[derive(Default)]
pub struct TransitionExample {
    cmd: UserCommands,
	frame: usize,
	transition_node1: EntityKey,
	transition_node2: EntityKey,
}

impl Example for TransitionExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // self.cmd.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root = world.spawn();
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
        self.cmd.append(root, EntityKey::null().0);

        let div1 = world.spawn();
		self.transition_node1 = EntityKey(div1);
		self.cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
		self.cmd.set_style(div1, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(
            div1,
            TransitionPropertyType(smallvec![StyleType::PositionLeft as usize]),
        );
        self.cmd.set_style(
            div1,
            TransitionTimingFunctionType(smallvec![AnimationTimingFunction::Linear]),
        );
        self.cmd.set_style(div1, TransitionDurationType(smallvec![Time(3000)]));
		self.cmd.set_style(div1, TransitionDelayType(smallvec![Time(0)]));
		self.cmd
		.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        self.cmd.append(div1, root);

        // 添加一个红色div2到根节点
        let div2 = world.spawn();
		self.transition_node2 = EntityKey(div2);
		self.cmd.set_style(div2, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
		self.cmd.set_style(div2, PositionLeftType(Dimension::Points(0.0)));
		self.cmd.set_style(div2, PositionTopType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
		self.cmd.set_style(
            div2,
            TransitionPropertyType(smallvec![std::usize::MAX]),
        );
        self.cmd.set_style(
            div2,
            TransitionTimingFunctionType(smallvec![AnimationTimingFunction::Linear]),
        );
        self.cmd.set_style(div2, TransitionDurationType(smallvec![Time(3000)]));
		self.cmd.set_style(div2, TransitionDelayType(smallvec![Time(0)]));
        self.cmd.append(div2, root);

    }

    fn render(&mut self, cmd: &mut UserCommands) {
		self.frame += 1;
		if self.frame == 60 * 3 {
			self.cmd.set_style(*self.transition_node1, PositionLeftType(Dimension::Points(300.0)));

			self.cmd.set_style(*self.transition_node2, PositionLeftType(Dimension::Points(300.0)));
			self.cmd.set_style(*self.transition_node2, WidthType(Dimension::Points(150.0)));
		}
		swap(&mut self.cmd, cmd);
	}
}
