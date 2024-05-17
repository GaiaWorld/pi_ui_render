// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use async_trait::async_trait;
use framework::{Param, Example};
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::Aabb2,
    style_type::{
        BackgroundImageType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Point2, RenderDirty, Viewport, RadialWave},
    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

#[async_trait]
impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), root));

        world.user_cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));

        world.user_cmd.append(root, EntityKey::null().0);

        // 添加一个div
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(600.0)));
        world.user_cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/z_source/3675173.jpg")));
        world.user_cmd.push_cmd(NodeCmd(RadialWave(pi_postprocess::prelude::RadialWave { // 添加水波纹效果
			aspect_ratio: false,
			start: 0.0,
			end: 1.0,
			center_x: 0.0,
			center_y: 0.0,
			cycle: 2,
			weight: 2.0,
		}), div1));

        world.user_cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
