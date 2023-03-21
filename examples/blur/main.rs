// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::style_type::{
    BackgroundImageType, BlurType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
};
use pi_ui_render::components::user::{CgColor, ClearColor};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, mut command: Commands, _gui: &mut Gui, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        self.cmd.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true));

        // 添加根节点
        let root = self.cmd.spawn(NodeBundle::default()).id();
        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));

        self.cmd.append(root, Id::null());

        // 添加一个div
        let div1 = self.cmd.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(110.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(110.0)));
        self.cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/blur/source/dialog_bg.png")));
        self.cmd.set_style(div1, BlurType(1.0));

        self.cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, cmd1: &mut Commands) { swap(&mut self.cmd, cmd); }
}
