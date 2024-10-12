// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::{swap, transmute};

// use pi_world::prelude::Entities;
use pi_world::prelude::Entity;
use pi_world::prelude::World;
use framework::{Param, Example};
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

    },
    resource::{NodeCmd, UserCommands, fragment::NodeTag},
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
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
	// entitys: Option<&'static mut Entities>,
    // world: Option<&'static mut World>,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));
		// self.world = Some(unsafe { transmute( world ) });
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
		world.user_cmd.init_node(root, NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), root));
		self.root = EntityKey(root);

        world.user_cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root, EntityKey::null().0);

        // 添加一个玫红色div到根节点， 并添加overflow属性
		let mut index = 0;
		let mut offsety = 0.0;
		let mut offsetx = 0.0;
		for _ in 0..1 {
			for _ in 0..100 {
				let c = if index % 3 == 0 {
					CgColor::new(1.0, 0.0, 0.0, 0.99)
				} else if index % 3 == 1 {
					CgColor::new(0.0, 1.0, 0.0, 0.99)
				} else {
					CgColor::new(0.0, 0.0, 1.0, 0.99)
				};
				// let c = CgColor::new(0.0, 0.0, 1.0, 1.0);
				let div1 = world.spawn(NodeTag::Div);
				world.user_cmd.set_style(div1, WidthType(Dimension::Points(size.0 as f32)));
				world.user_cmd.set_style(div1, HeightType(Dimension::Points(size.1 as f32)));
				world.user_cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
				world.user_cmd.set_style(div1, PositionLeftType(Dimension::Points(offsetx)));
				world.user_cmd.set_style(div1, PositionTopType(Dimension::Points(offsety)));
				world.user_cmd
						.set_style(div1, BackgroundColorType(Color::RGBA(c.clone())));
				world.user_cmd.append(div1, root);

				
				
				index += 1;
				offsety += 0.5;
				// if offset > 100.0 {
				//     offset = 0.0;
				// }
			}
			offsetx += 10.0;
			offsety = 0.0;
		}
		
    }

    fn render(&mut self, cmd: &mut UserCommands) { 
		// world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
		// self.index += 1;

		// if self.index == 500 {
		// 	let div1 = self.entitys.as_mut().unwrap().reserve_entity();
		// 	world.user_cmd.init_node(div1, NodeTag::Div);
		// 	world.user_cmd.set_style(div1, WidthType(Dimension::Points(400.0 as f32)));
		// 	world.user_cmd.set_style(div1, HeightType(Dimension::Points(400.0 as f32)));
		// 	world.user_cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
		// 	world.user_cmd
		// 			.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
		// 	world.user_cmd.append(div1, self.root.0);
		// }
		 
	}
}

// fn main() {}
