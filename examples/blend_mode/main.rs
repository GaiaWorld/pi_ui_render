// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy::{ecs::system::Commands, prelude::World};
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::{
    prelude::{Rect, Size},
    style::{Dimension, FlexWrap, PositionType, AlignContent},
};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, BorderRadius, CgColor, ImageRepeat, ImageRepeatOption, NotNanRect, Point2, BlendMode},
    style_type::{
        BackgroundImageClipType, BackgroundImageType, BackgroundRepeatType, BorderRadiusType, FlexWrapType, HeightType, MarginLeftType,
        MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, BlendModeType, JustifyContentType, AlignItemsType, AlignContentType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{ClearColor, LengthUnit, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
	fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        Some(Size{ width: 1020, height: 959})
    }

    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
		self.cmd.set_style(root, JustifyContentType(pi_flex_layout::style::JustifyContent::Center));
		self.cmd.set_style(root, AlignItemsType(pi_flex_layout::style::AlignItems::Center));
		self.cmd.set_style(root, AlignContentType(pi_flex_layout::style::AlignContent::Center));
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, FlexWrapType(FlexWrap::Wrap));
        self.cmd.append(root, EntityKey::null().0);

        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(510.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(480.0)));
        self.cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
        self.cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/blend_mode/source/chouka_shitou_1.png")));
        self.cmd.append(div1, root);

		let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(450.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(600.0)));
		self.cmd.set_style(div2, BlendModeType(BlendMode::AlphaAdd));
        self.cmd.set_style(div2, PositionTypeType(PositionType::Absolute));
        self.cmd
            .set_style(div2, BackgroundImageType(Atom::from("examples/blend_mode/source/6.png")));
        self.cmd.append(div2, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}