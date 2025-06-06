// 测试有两个根的情况

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use async_trait::async_trait;
use pi_world::prelude::Component;
use derive_deref_rs::Deref;
use framework::{Param, Example};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType},
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{Canvas, CgColor, ClearColor, Color, RenderTargetType, Viewport},

    },
    resource::{ComponentCmd, NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

pub struct QuadExample {
    root_one: EntityKey,
    root_tow: EntityKey,
    cmd: UserCommands,
}

impl Default for QuadExample {
    fn default() -> Self {
        Self {
            root_one: EntityKey::null(),
            root_tow: EntityKey::null(),
            cmd: UserCommands::default(),
        }
    }
}

#[derive(Debug, Component, Deref)]
pub struct RootTarget(EntityKey);

// pub fn add_root_depend(
// 	query: Query<(&RootTarget, &GraphId), Changed<RootTarget>>,
// 	commands: Commands,
// 	query_graphid: Query<&GraphId>,
// ) {
// 	for root in query.iter() {
// 		if let Err(e) = rg.add_depend(**graph_id, **parent_graph_id) {
// 			log::error!("{:?}", e);
// 		}
// 	}
// }

#[async_trait]
impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // world.user_cmd.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root_one = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), false), root_one));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root_one,
        ));

        world.user_cmd.set_style(root_one, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root_one, HeightType(Dimension::Points(size.1 as f32)));

        world.user_cmd.set_style(root_one, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root_one, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_one, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root_one, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root_one, EntityKey::null().0);

        // 添加一个红色div到根节点
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));

        world.user_cmd.append(div1, root_one);
        self.root_one = EntityKey(root_one);

        // 添加根节点
        let root_tow = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 0.0), true), root_tow));
        world.user_cmd
            .push_cmd(NodeCmd(Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(200.0, 200.0))), root_tow));
        world.user_cmd.push_cmd(NodeCmd(RenderTargetType::OffScreen, root_tow));

        world.user_cmd.set_style(root_tow, WidthType(Dimension::Points(200.0)));
        world.user_cmd.set_style(root_tow, HeightType(Dimension::Points(200.0)));

        world.user_cmd.set_style(root_tow, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root_tow, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root_tow, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root_tow, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root_tow, EntityKey::null().0);

        // 添加一个绿色div到根节点
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(200.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        world.user_cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));

        world.user_cmd.append(div1, root_tow);

        self.root_tow = EntityKey(root_tow);

        // 创建一个canvas节点
        // 将200 * 200的gui渲染到300*300的canvas上
        let canvas = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(canvas, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(canvas, WidthType(Dimension::Points(300.0)));
        world.user_cmd.set_style(canvas, HeightType(Dimension::Points(300.0)));
        world.user_cmd.set_style(canvas, PositionLeftType(Dimension::Points(100.0)));
        world.user_cmd.set_style(canvas, PositionTopType(Dimension::Points(100.0)));
        world.user_cmd.push_cmd(pi_ui_render::resource::CanvasCmd(root_tow, false, canvas));
        world.user_cmd.append(canvas, self.root_one.0);

    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
