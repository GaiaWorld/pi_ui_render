// 子节点超出容器节点范围， 并且， 容器节点变为fbo的情况测试

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use framework::{Param, Example};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        BackgroundColorType, HeightType, MarginLeftType, MarginTopType, OpacityType, PositionLeftType, PositionBottomType, PositionTopType, PositionTypeType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    div0: EntityKey,
    count: usize,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        // self.root = EntityKey(root);
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

        // 添加一个红色div
        let div0 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div0, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div0, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div0, PositionLeftType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div0, PositionTopType(Dimension::Points(200.0)));
        world.user_cmd
            .set_style(div0, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        world.user_cmd.append(div0, root);
        self.div0 = EntityKey(div0);

        // 添加一个绿色div
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        world.user_cmd.set_style(div1, PositionBottomType(Dimension::Points(0.0)));
        world.user_cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        world.user_cmd.append(div1, div0);
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        self.count += 1;
        if self.count == 500 {
            cmd.set_style(self.div0.0, OpacityType(0.5));
        }
       
        // world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        
    }
}
