// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy::ecs::system::Commands;
use bevy::prelude::World;
use font_kit::font::new_face_by_path;
use framework::Example;
use pi_atom::Atom;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, FontSize, RenderDirty, Viewport, Transform},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};

fn main() { framework::start(QuadExample::default()) }
use pi_style::{
    style::{Aabb2, Point2, TextContent, ColorAndPosition, LinearGradientColor, TransformFunc},
    style_type::{
        ColorType, FontFamilyType, FontSizeType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType,
        TextContentType, WidthType, BackgroundColorType, TransformFuncType, TransformType,
    },
};

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        let mut dir = std::env::current_dir().unwrap();
        log::info!("dir: {:?}", dir);
        dir.push("examples/text/source/hwkt.ttf");
        // new_face_by_path("hwkt".to_string(), dir.to_str().unwrap());
        new_face_by_path("hwkt".to_string(), "examples/text/source/SOURCEHANSANSK-MEDIUM.TTF");

        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
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
		let div1 = world.spawn(NodeBundle::default()).id();
		self.cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
		self.cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
		self.cmd
			.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
		self.cmd.set_style(div1, PositionLeftType(Dimension::Percent(0.5)));

		self.cmd.append(div1, root);

		// 添加一个红色div到红节点
		let div2 = world.spawn(NodeBundle::default()).id();
		self.cmd.set_style(div2, WidthType(Dimension::Points(200.0)));
		self.cmd.set_style(div2, HeightType(Dimension::Points(200.0)));
		self.cmd.set_style(div2, PositionLeftType(Dimension::Percent(0.5)));
		let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::TranslateX(pi_style::style::LengthUnit::Percent(-0.5))); // 旋转45度
		transform.all_transform.transform.push(TransformFunc::Scale(1.4, 1.4)); // 旋转45度
        self.cmd.set_style(div2, TransformType(transform.all_transform.transform));
		self.cmd
			.set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
		self.cmd.append(div2, div1);
 
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
