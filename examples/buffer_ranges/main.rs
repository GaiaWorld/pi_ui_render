// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use pi_atom::Atom;
use smallvec::smallvec;
use framework::{Param, Example};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

use pi_style::{
    style::{Aabb2, AnimationFillMode, AnimationName, BorderRadius, Color, IterationCount, LengthUnit, Point2, Time}, style_parse::parse_class_map_from_string, style_type::{
        AnimationDurationType, AnimationFillModeType, AnimationIterationCountType, AnimationNameType, AsImageType, BackgroundColorType, BorderBottomType, BorderColorType, BorderLeftType, BorderRadiusType, BorderRightType, BorderTopType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType
    }
};
use pi_ui_render::resource::ExtendCssCmd;
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {

        let css = "@keyframes test-animation {
			0% {left: 0px; }
			100% {left: 50px; }
		}";
        let class_map = parse_class_map_from_string(css, 0).unwrap();
        world.user_cmd.push_cmd(ExtendCssCmd(vec![class_map]));
        
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
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(20.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(20.0)));
        world.user_cmd.set_style(
            div1,
            AnimationNameType(AnimationName {
                scope_hash: 0,
                value: smallvec![Atom::from("test-animation")],
            }),
        );
        world.user_cmd.set_style(div1, AnimationIterationCountType(smallvec![IterationCount(10000000.0)]));
        world.user_cmd.set_style(div1, AnimationDurationType(smallvec![Time(3000)]));
        world.user_cmd.set_style(div1, AnimationFillModeType(smallvec![AnimationFillMode::Forwards]));
        world.user_cmd.append(div1, root);


        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(10.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(10.0)));
        world.user_cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        world.user_cmd.append(div2, div1);

        for _ in 0..1000 {
            // 添加一个红色div
            let div = world.spawn(NodeTag::Div);
            world.user_cmd.set_style(div, WidthType(Dimension::Points(10.0)));
            world.user_cmd.set_style(div, HeightType(Dimension::Points(10.0)));
            world.user_cmd.set_style(div, BorderColorType(CgColor::new(1.0, 0.0, 0.0, 1.0)));
            world.user_cmd.set_style(div, BorderTopType(Dimension::Points(2.0)));
            world.user_cmd.set_style(div, BorderRightType(Dimension::Points(2.0)));
            world.user_cmd.set_style(div, BorderBottomType(Dimension::Points(2.0)));
            world.user_cmd.set_style(div, BorderLeftType(Dimension::Points(2.0)));
            world.user_cmd.append(div, root);
        }

        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(20.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(20.0)));
        world.user_cmd.set_style(
            div1,
            AnimationNameType(AnimationName {
                scope_hash: 0,
                value: smallvec![Atom::from("test-animation")],
            }),
        );
        world.user_cmd.set_style(div1, AnimationIterationCountType(smallvec![IterationCount(10000000.0)]));
        world.user_cmd.set_style(div1, AnimationDurationType(smallvec![Time(3000)]));
        world.user_cmd.set_style(div1, AnimationFillModeType(smallvec![AnimationFillMode::Forwards]));
        world.user_cmd.append(div1, root);

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(10.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(10.0)));
        world.user_cmd
            .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        world.user_cmd.append(div2, div1);

       
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
