// 测试有两个根的情况

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use async_trait::async_trait;
/// 渲染四边形 demo

use framework::{Param, Example};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType},
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, Viewport},

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
        // 设置清屏颜色为绿色
        // world.user_cmd.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root_one = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), false), root_one));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root_one,
        ));

        world.user_cmd.set_style(root_one, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root_one, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root_one, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root_one, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root_one, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root_one, EntityKey::null().0);

        // 添加一个红色div到根节点
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));

        world.user_cmd.append(div1, root_one);

        // 添加根节点
        let root_tow = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root_tow));
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root_tow));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root_tow,
        ));

        world.user_cmd.set_style(root_tow, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root_tow, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root_tow, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root_tow, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root_tow, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root_tow, EntityKey::null().0);

        // 添加一个绿红色div到根节点
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));

        world.user_cmd.append(div1, root_tow);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
