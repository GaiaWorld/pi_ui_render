// 测试有两个根的情况

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Node, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType},
};
use pi_ui_render::{
    components::user::{Canvas, CgColor, ClearColor, Color, RenderTargetType, Viewport},
    export::Engine,
    utils::cmd::NodeCmd,
};

fn main() { framework::start(QuadExample::default()) }

pub struct QuadExample {
    root_one: Id<Node>,
    root_tow: Id<Node>,
	cmd: UserCommands,
}

impl Default for QuadExample {
    fn default() -> Self {
        Self {
            root_one: Id::null(),
            root_tow: Id::null(),
			cmd: UserCommands::default(),
        }
    }
}

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, mut command: Commands, _gui: &mut Gui, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root_one = gui.gui.create_node();
        gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), false), root_one));
        gui.gui.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root_one,
        ));

        gui.gui.set_style(root_one, WidthType(Dimension::Points(size.0 as f32)));
        gui.gui.set_style(root_one, HeightType(Dimension::Points(size.1 as f32)));

        gui.gui.set_style(root_one, PositionTypeType(PositionType::Absolute));
        gui.gui.set_style(root_one, PositionLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root_one, PositionTopType(Dimension::Points(0.0)));
        gui.gui.set_style(root_one, MarginLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root_one, MarginTopType(Dimension::Points(0.0)));
        gui.gui.append(root_one, Id::null());

        // 添加一个红色div到根节点
        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, WidthType(Dimension::Points(100.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(100.0)));
        gui.gui
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));

        gui.gui.append(div1, root_one);
        self.root_one = root_one;

        // 添加根节点
        let root_tow = gui.gui.create_node();
        gui.gui.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root_tow));
        gui.gui
            .push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(300.0, 300.0))), root_tow));
        gui.gui.push_cmd(NodeCmd(RenderTargetType::OffScreen, root_tow));

        gui.gui.set_style(root_tow, WidthType(Dimension::Points(300.0)));
        gui.gui.set_style(root_tow, HeightType(Dimension::Points(300.0)));

        gui.gui.set_style(root_tow, PositionTypeType(PositionType::Absolute));
        gui.gui.set_style(root_tow, PositionLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root_tow, PositionTopType(Dimension::Points(0.0)));
        gui.gui.set_style(root_tow, MarginLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root_tow, MarginTopType(Dimension::Points(0.0)));
        gui.gui.append(root_tow, Id::null());

        // 添加一个绿红色div到根节点
        let div1 = gui.gui.create_node();
        gui.gui.set_style(div1, WidthType(Dimension::Points(300.0)));
        gui.gui.set_style(div1, HeightType(Dimension::Points(300.0)));
        gui.gui
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));

        gui.gui.append(div1, root_tow);

        self.root_tow = root_tow;
    }

    fn render(&mut self, gui: &mut Engine) {
        gui.gui.run();

        if !self.root_tow.is_null() {
            if let Some(r) = gui.gui.get_graph_node_id(self.root_tow.clone()) {
                let canvas = gui.gui.create_node();
                gui.gui.set_style(canvas, PositionTypeType(PositionType::Absolute));
                gui.gui.set_style(canvas, WidthType(Dimension::Points(300.0)));
                gui.gui.set_style(canvas, HeightType(Dimension::Points(300.0)));
                gui.gui.set_style(canvas, PositionLeftType(Dimension::Points(100.0)));
                gui.gui.set_style(canvas, PositionTopType(Dimension::Points(100.0)));

                gui.gui.push_cmd(NodeCmd(Canvas(r), canvas));

                gui.gui.append(canvas, self.root_one);
                self.root_tow = Id::null();
            }
        }
    }
}
