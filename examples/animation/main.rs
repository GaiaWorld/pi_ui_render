// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;


use framework::{Param, Example};
use pi_atom::Atom;
use pi_curves::steps::EStepMode;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, AnimationFillMode, AnimationName, AnimationTimingFunction, IterationCount, Time},
    style_parse::parse_class_map_from_string,
    style_type::{
        AnimationDurationType, AnimationFillModeType, AnimationIterationCountType, AnimationNameType,
        AnimationTimingFunctionType, BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
        PositionTypeType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, Point2, RenderDirty, Viewport},

    },
    resource::{ExtendCssCmd, NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;
use smallvec::smallvec;

fn main() { framework::start(AnimationExample::default()) }

#[derive(Default)]
pub struct AnimationExample {
    cmd: UserCommands,
}

impl Example for AnimationExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加keyframes
        // let css = "@keyframes test-animation {
		// 	0% {transform: scale(1.0, 1.0); }
		// 	33% {transform: scale(1.5, 1.5); }
		// 	66% {transform: scale(2.0, 2.0); }
		// }";
		let css = "@keyframes test-animation {
			0% {opacity: 0; }
			100% {opacity: 0.9; }
		}";
        let class_map = parse_class_map_from_string(css, 0).unwrap();
        world.user_cmd.push_cmd(ExtendCssCmd(vec![class_map]));

        // 设置清屏颜色为绿色
        // world.user_cmd.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

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

        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(
            div1,
            AnimationNameType(AnimationName {
                scope_hash: 0,
                value: smallvec![Atom::from("test-animation")],
            }),
        );
        world.user_cmd.set_style(div1, AnimationIterationCountType(smallvec![IterationCount(90.0)]));
        world.user_cmd.set_style(div1, AnimationDurationType(smallvec![Time(3000)]));
        world.user_cmd.set_style(div1, AnimationFillModeType(smallvec![AnimationFillMode::Forwards]));
        // world.user_cmd.set_style(
        //     div1,
        //     AnimationTimingFunctionType(smallvec![AnimationTimingFunction::Step(1, EStepMode::JumpEnd)]),
        // );
        // world.user_cmd.set_style(div1, AnimationDirectionType(smallvec![AnimationDirection::Reverse]));
        world.user_cmd.append(div1, root);

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        world.user_cmd.append(div2, div1);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
