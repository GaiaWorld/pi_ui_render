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
        BackgroundColorType, HeightType, MarginLeftType, MarginTopType, OpacityType, OverflowType, PositionLeftType, PositionTopType,
        PositionTypeType, TransformType, WidthType,
    },
};
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
		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        self.cmd.append(root, EntityKey::null().0);

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div1 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div1, WidthType(Dimension::Points(300.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(300.0)));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        // self.cmd.set_style(div1, OverflowType(Overflow(true)));
        let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::RotateZ(45.0)); // 旋转45度
        self.cmd.set_style(div1, TransformType(transform.all_transform.transform));
        // self.cmd.set_style(div1, TransformWillChangeType(transform.funcs));
        self.cmd.append(div1, root);

        // 添加一个红色div到红节点
        let div2 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        self.cmd.append(div2, div1);

        // 添加一个容器节点，设置overflow
        let div3 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div3, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div3, WidthType(Dimension::Points(250.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(150.0)));
        self.cmd
            .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.5, 1.0))));
        self.cmd.set_style(div3, OverflowType(true));
        self.cmd.append(div3, div1);

        // 添加一个绿色div
        let div4 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div4, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div4, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div4, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.append(div4, div3);

        // 添加一个黄色
        let div5 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div5, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div5, PositionLeftType(Dimension::Points(50.0)));
        self.cmd.set_style(div5, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div5, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div5, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        // 设置opacity，测试Pass2d在父上存在TransformWillChange的情况下能否正确渲染
        self.cmd.set_style(div5, OpacityType(0.5));
        self.cmd.append(div5, div3);

        // 添加一个灰色四边形，并设置半透明、旋转， 测试一个渲染上下文在父上下文旋转、自身也旋转的情况下，渲染是否正确
        let div6 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div6, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div6, PositionLeftType(Dimension::Points(100.0)));
        self.cmd.set_style(div6, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div6, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div6, HeightType(Dimension::Points(100.0)));
        self.cmd
            .set_style(div6, BackgroundColorType(Color::RGBA(CgColor::new(0.7, 0.7, 0.7, 1.0))));
        // 设置opacity，测试Pass2d在父上存在TransformWillChange的情况下能否正确渲染
        self.cmd.set_style(div6, OpacityType(0.5));
        let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::RotateZ(30.0)); // 旋转30度
        self.cmd.set_style(div6, TransformType(transform.all_transform.transform));
        self.cmd.append(div6, div3);

        // 添加一个绿色四边形，设置oveflow为true，
        let div7 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div7, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div7, PositionLeftType(Dimension::Points(150.0)));
        self.cmd.set_style(div7, PositionTopType(Dimension::Points(100.0)));
        self.cmd.set_style(div7, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div7, HeightType(Dimension::Points(50.0)));
        self.cmd
            .set_style(div7, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        self.cmd.set_style(div7, OverflowType(true));
        let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::RotateZ(30.0)); // 旋转30度
        self.cmd.set_style(div7, TransformType(transform.all_transform.transform));
        self.cmd.append(div7, div3);

        // 添加一个黄色四边形，设置旋转，
        let div8 = world.spawn(NodeTag::Div);
        self.cmd.set_style(div8, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(div8, PositionLeftType(Dimension::Points(15.0)));
        self.cmd.set_style(div8, WidthType(Dimension::Points(50.0)));
        self.cmd.set_style(div8, HeightType(Dimension::Points(50.0)));
        self.cmd
            .set_style(div8, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 0.0, 1.0))));
        let mut transform = Transform::default();
        transform.all_transform.transform.push(TransformFunc::RotateZ(45.0)); // 旋转45度
        self.cmd.set_style(div8, TransformType(transform.all_transform.transform));
        self.cmd.append(div8, div7);
    }

    fn render(&mut self, cmd: &mut UserCommands) { swap(&mut self.cmd, cmd); }
}
