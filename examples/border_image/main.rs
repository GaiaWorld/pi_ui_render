// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use bevy::{ecs::system::Commands, prelude::World};
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_flex_layout::style::{Dimension, FlexWrap, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, BorderImageSlice, ImageRepeatOption, Point2},
    style_type::{
        BorderBottomType, BorderImageRepeatType, BorderImageSliceType, BorderImageType, BorderLeftType, BorderRightType, BorderTopType, FlexWrapType,
        HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, ImageRepeat, RenderDirty, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};
use std::mem::swap;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, world: &mut World, size: (usize, usize)) {
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
        self.cmd.set_style(root, FlexWrapType(FlexWrap::Wrap));
        self.cmd.append(root, EntityKey::null().0);

        // repeat 整数倍数
        let div2 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div2, WidthType(Dimension::Points(200.0)));
        self.cmd.set_style(div2, HeightType(Dimension::Points(200.0)));
        self.cmd.set_style(div2, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div2, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div2,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div2, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div2, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div2, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div2, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div2,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div2, root);

        // repeat 非整数倍数
        let div3 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div3, WidthType(Dimension::Points(220.0)));
        self.cmd.set_style(div3, HeightType(Dimension::Points(220.0)));
        self.cmd.set_style(div3, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div3, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div3,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div3, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div3, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div3, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div3, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div3,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div3, root);

        // space 非整数倍数
        let div4 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div4, WidthType(Dimension::Points(220.0)));
        self.cmd.set_style(div4, HeightType(Dimension::Points(220.0)));
        self.cmd.set_style(div4, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div4, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div4,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div4, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div4, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div4, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div4, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div4,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        self.cmd.append(div4, root);

        // round 非整数倍数
        let div5 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div5, WidthType(Dimension::Points(220.0)));
        self.cmd.set_style(div5, HeightType(Dimension::Points(220.0)));
        self.cmd.set_style(div5, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div5, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div5,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div5, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div5, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div5, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div5, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div5,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        self.cmd.append(div5, root);

        // 测试中间不足一倍的情况 repeat
        let div6 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div6, WidthType(Dimension::Points(95.0)));
        self.cmd.set_style(div6, HeightType(Dimension::Points(95.0)));
        self.cmd.set_style(div6, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div6, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div6,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div6, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div6, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div6, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div6, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div6,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        self.cmd.append(div6, root);

        // 测试中间不足一倍的情况 round
        let div7 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div7, WidthType(Dimension::Points(95.0)));
        self.cmd.set_style(div7, HeightType(Dimension::Points(95.0)));
        self.cmd.set_style(div7, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div7, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div7,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div7, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div7, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div7, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div7, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div7,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        self.cmd.append(div7, root);

        // 测试中间不足一倍的情况 space
        let div8 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div8, WidthType(Dimension::Points(95.0)));
        self.cmd.set_style(div8, HeightType(Dimension::Points(95.0)));
        self.cmd.set_style(div8, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(div8, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        self.cmd.set_style(
            div8,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        self.cmd.set_style(div8, BorderTopType(Dimension::Points(40.0)));
        self.cmd.set_style(div8, BorderRightType(Dimension::Points(40.0)));
        self.cmd.set_style(div8, BorderBottomType(Dimension::Points(40.0)));
        self.cmd.set_style(div8, BorderLeftType(Dimension::Points(40.0)));
        self.cmd.set_style(
            div8,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        self.cmd.append(div8, root);


        // 测试top\bottom为0，并且为Stretch的情况
        let div9 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div9, WidthType(Dimension::Points(448.0)));
        self.cmd.set_style(div9, HeightType(Dimension::Points(62.0)));
        self.cmd.set_style(div9, PositionTypeType(PositionType::Relative));
        self.cmd.set_style(
            div9,
            BorderImageType(Atom::from("examples/border_image/source/chuangjianjuese_shuxingbg.png")),
        );
        self.cmd.set_style(
            div9,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.0) },
                right: unsafe { NotNan::new_unchecked(0.4464) },
                bottom: unsafe { NotNan::new_unchecked(0.0) },
                left: unsafe { NotNan::new_unchecked(0.4464) },
                fill: true,
            }),
        );
        self.cmd.set_style(div9, BorderTopType(Dimension::Points(0.0)));
        self.cmd.set_style(div9, BorderRightType(Dimension::Points(200.0)));
        self.cmd.set_style(div9, BorderBottomType(Dimension::Points(0.0)));
        self.cmd.set_style(div9, BorderLeftType(Dimension::Points(200.0)));
        self.cmd.append(div9, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
