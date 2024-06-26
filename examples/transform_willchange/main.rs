// 半透明渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use framework::{Param, Example};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        AsImageType, BackgroundColorType, HeightType, MarginLeftType, MarginTopType, OpacityType, PositionLeftType, PositionTopType, PositionTypeType, TransformType, TransformWillChangeType, WidthType
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, RenderDirty, TransformFunc, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    root: EntityKey,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        self.root = EntityKey(root);
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

        // 添加一个玫红色div到根节点， 并添加TransformWillChange属性
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        // let mut transform_willchange = Vec::default();
        // transform_willchange.push(TransformFunc::TranslateX(pi_style::style::LengthUnit::Pixel(50.0)));
        // world.user_cmd.set_style(div1, TransformWillChangeType(true));
        // world.user_cmd.set_style(div1, TransformType(transform_willchange));

        world.user_cmd.append(div1, root);

        // 添加一个红色div到玫红节点
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        world.user_cmd.append(div2, div1);

        // 添加一个容器节点，其下有一个绿色节点，一个黄色节点， 对本节点添加TransformWillchange
        let div3 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div3, PositionTopType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div3, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div3, HeightType(Dimension::Points(200.0)));
        // 设置TransformWillChange，向右平移100个像素
        let mut transform_willchange = Vec::default();
        transform_willchange.push(TransformFunc::TranslateX(pi_style::style::LengthUnit::Pixel(50.0)));
        world.user_cmd.set_style(div3, TransformWillChangeType(true));
        world.user_cmd.set_style(div3, TransformType(transform_willchange));

        // 添加一个绿色div
        let div4 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div4, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div4, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div4, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        world.user_cmd.append(div4, div3);

        // 添加一个黄色
        let div5 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div5, PositionTopType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div5, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div5, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div5, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        // 设置opacity，测试Pass2d在父上存在TransformWillChange的情况下能否正确渲染
        world.user_cmd.set_style(div5, OpacityType(0.5));
        world.user_cmd.append(div5, div3);

        world.user_cmd.append(div3, div1);

        println!("div1: {:?}", (div3, div5));
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        // world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
        
    }
}
