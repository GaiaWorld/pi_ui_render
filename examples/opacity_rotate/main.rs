// overflow 旋转

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use async_trait::async_trait;
use framework::{Param, Example};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::style_type::AsImageType;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        BackgroundImageType, HeightType, MarginLeftType, MarginTopType, OpacityType, OverflowType, PositionLeftType, PositionTopType,
        PositionTypeType, TransformType, WidthType,
    },
};
use pi_atom::Atom;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Transform, TransformFunc, Viewport},

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
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, PositionTopType(Dimension::Points(300.0)));
        let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::RotateZ(180.0)); // 旋转45度
        world.user_cmd.set_style(div1, TransformType(transform.all_transform.transform));
        world.user_cmd.set_style(div1, OpacityType(0.5));
        world.user_cmd.set_style(div1, BackgroundImageType(Atom::from("examples/z_source/bx_lanseguanbi.png")));
        world.user_cmd.append(div1, root); 
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
