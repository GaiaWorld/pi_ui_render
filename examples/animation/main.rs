// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{AnimationDirection, IterationCount, Time, AnimationName, Aabb2},
    style_parse::parse_class_map_from_string,
    style_type::{
        AnimationDirectionType, AnimationDurationType, AnimationIterationCountType, AnimationNameType, BackgroundColorType, HeightType,
        MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::user::{CgColor, Color, ClearColor, Viewport, Point2},
    export::Engine,
    utils::cmd::{SingleCmd, NodeCmd},
};
use smallvec::smallvec;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;
#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Engine, size: (usize, usize)) {
        // 添加keyframes
        let css = "@keyframes test-animation {
			0% {left: 0px;}
			100% {left: 300px}
		}";
        let class_map = parse_class_map_from_string(css, 0).unwrap();
        gui.gui.push_cmd(SingleCmd(class_map.key_frames));


        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root = gui.gui.create_node();
		gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true), root));
		gui.gui.push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))), root));
        gui.gui.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        gui.gui.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        gui.gui.set_style(root, PositionTypeType(PositionType::Absolute));
        gui.gui.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root, PositionTopType(Dimension::Points(0.0)));
        gui.gui.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root, MarginTopType(Dimension::Points(0.0)));
        gui.gui.append(root, Id::null());

        // 添加一个玫红色div到根节点， 并添加overflow属性
        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, WidthType(Dimension::Points(100.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(100.0)));
        gui.gui
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
        gui.gui.set_style(div1, AnimationNameType(AnimationName{scope_hash: 0, value: smallvec![Atom::from("test-animation")]} ));
        gui.gui
            .set_style(div1, AnimationIterationCountType(smallvec![IterationCount(1.0)]));
        gui.gui.set_style(div1, AnimationDirectionType(smallvec![AnimationDirection::Reverse]));
        gui.gui.set_style(div1, AnimationDurationType(smallvec![Time(3000)]));

        gui.gui.append(div1, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
