// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::{swap, transmute};

use bevy_ecs::entity::Entities;
use bevy_ecs::{system::Commands, entity::Entity};
use bevy_ecs::prelude::World;
use framework::Example;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType},
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands, fragment::NodeTag},
};

fn main() { framework::start(QuadExample::default()) }

#[cfg(predicate)]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
	framework::start(QuadExample::default());
}

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
	root: EntityKey,
	index: usize,
	entitys: Option<&'static mut Entities>,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));
		self.entitys = Some(unsafe { transmute( world.entities_mut() ) });
        // 添加根节点
        let root = world.spawn(()).id();
		self.cmd.init_node(root, NodeTag::Div);
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));
		self.root = EntityKey(root);

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd.append(root, EntityKey::null().0);

        // 添加一个玫红色div到根节点， 并添加overflow属性
		let mut index = 0;
		let mut offset = 0.0;
		for _ in 0..48 {
			let div1 = world.spawn(NodeBundle::default()).id();
			self.cmd.set_style(div1, WidthType(Dimension::Points(size.0 as f32)));
			self.cmd.set_style(div1, HeightType(Dimension::Points(size.1 as f32)));
			self.cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
			self.cmd.set_style(div1, PositionLeftType(Dimension::Points(offset)));
			self.cmd.set_style(div1, PositionTopType(Dimension::Points(offset)));
			self.cmd.append(div1, root);

			let c = if index % 3 == 0 {
				CgColor::new(1.0, 0.0, 0.0, 1.0)
			} else if index % 3 == 1 {
				CgColor::new(0.0, 1.0, 0.0, 1.0)
			} else {
				CgColor::new(0.0, 0.0, 1.0, 1.0)
			};
			index += 1;
			offset += 3.0;

			for i in 0..2500 {
				let div2 = world.spawn(NodeBundle::default()).id();
				self.cmd.set_style(div2, WidthType(Dimension::Points(9.0)));
				self.cmd.set_style(div2, HeightType(Dimension::Points(9.0)));
				self.cmd.set_style(div2, MarginLeftType(Dimension::Points(1.0)));
				self.cmd.set_style(div2, MarginTopType(Dimension::Points(1.0)));
				self.cmd
					.set_style(div2, BackgroundColorType(Color::RGBA(c.clone())));
				self.cmd.append(div2, div1);
			} 
		}
		
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { 
		self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
		self.index += 1;

		if self.index == 500 {
			let div1 = self.entitys.as_mut().unwrap().reserve_entity();
			self.cmd.init_node(div1, NodeTag::Div);
			self.cmd.set_style(div1, WidthType(Dimension::Points(400.0 as f32)));
			self.cmd.set_style(div1, HeightType(Dimension::Points(400.0 as f32)));
			self.cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
			self.cmd
					.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
			self.cmd.append(div1, self.root.0);
		}
		swap(&mut self.cmd, cmd); 
	}
}
