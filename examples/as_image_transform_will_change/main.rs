// 测试AsImage, 在AsImage节点上不断设置TransformWillChange， AsImage节点应该不会每帧渲染，只是将缓冲的fbo渲染到父目标上

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use framework::{Param, Example};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2, TransformFunc},
    style_type::{
        AsImageType, BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType,
        TransformType, TransformWillChangeType, WidthType,
    },
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
    change_node: EntityKey,
    rotate: f32,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

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

        // 添加一个红色div到根节点， 并设置AsImage为Force
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(200.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        world.user_cmd.set_style(div1, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        // world.user_cmd.set_style(self.change_node.0, RotateType(30.0));
        world.user_cmd.append(div1, root);
        self.change_node = EntityKey(div1);

        // 添加一个绿色div到div2
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        world.user_cmd.append(div2, div1);
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        if self.rotate == 0.0 {
            cmd.set_style(self.change_node.0, TransformWillChangeType(true));
        }
        // 不停的转动change_node
        // if self.rotate < 45.0 {
        self.rotate += 1.0;
        // }


        let mut transform_willchange = Vec::default();
        transform_willchange.push(TransformFunc::RotateZ(self.rotate));
        cmd.set_style(self.change_node.0, TransformType(transform_willchange));

        
    }
}
