// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use framework::{Param, Example};
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2,TransformFunc},
    style_type::{BackgroundImageType, HeightType, MarginLeftType, MarginTopType, OpacityType, PositionLeftType, PositionTopType,
        PositionTypeType, TransformType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
    // div1: EntityKey,
    // time: Option<std::time::Instant>,
    // flag: bool,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {


        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_single_res(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 0.0, 0.0, 1.0), true), root));
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


        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(200.0)));
        world.user_cmd.set_style(div2, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(div2, PositionLeftType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div2, PositionTopType(Dimension::Points(50.0)));
        world.user_cmd.append(div2, root);

        let div3 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div3, WidthType(Dimension::Points(61.0)));
        world.user_cmd.set_style(div3, HeightType(Dimension::Points(116.0)));
        world.user_cmd.set_style(div3, PositionTypeType(PositionType::Absolute));
        world.user_cmd
            .set_style(div3, BackgroundImageType(Atom::from("examples/test/source/icon_qieye.png")));
        // world.user_cmd
        // .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 255.0, 0.0, 1.0))));
        // world.user_cmd
        // .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(255.0, 255.0, 255.0, 0.1))));
        let mut transform = Vec::default();
        transform.push(TransformFunc::RotateZ(180.0));
        transform.push(TransformFunc::Scale(0.5, 0.5));
        world.user_cmd.set_style(div3, TransformType(transform));
        world.user_cmd.set_style(div3, OpacityType(0.99));
        world.user_cmd.append(div3, div2);

        let div3 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div3, WidthType(Dimension::Points(61.0)));
        world.user_cmd.set_style(div3, HeightType(Dimension::Points(116.0)));
        world.user_cmd.set_style(div3, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(div3, PositionTopType(Dimension::Points(-50.0)));
        world.user_cmd
            .set_style(div3, BackgroundImageType(Atom::from("examples/test/source/icon_qieye.png")));
        // world.user_cmd
        // .set_style(div3, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 255.0, 0.0, 1.0))));
        // world.user_cmd
        // .set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(255.0, 255.0, 255.0, 0.1))));
        let mut transform = Vec::default();
        transform.push(TransformFunc::RotateZ(180.0));
        transform.push(TransformFunc::Scale(0.5, 0.5));
        world.user_cmd.set_style(div3, TransformType(transform));
        world.user_cmd.append(div3, div2);

        log::warn!("end=====");
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        // if let Some(time) = &self.time {
        // 	if std::time::Instant::now() - *time > std::time::Duration::from_millis(1000) {
        // 		self.time = Some(std::time::Instant::now());
        // 		log::warn!("zzzz=====");
        // 		if !self.flag {
        // 			world.user_cmd.set_style(
        // 				self.div1.0,
        // 				HsiType(Hsi {
        // 					hue_rotate: 0.0,
        // 					saturate: 0.0,
        // 					bright_ness: 0.0,
        // 				}),
        // 			);
        // 		} else {
        // 			world.user_cmd.set_style(
        // 				self.div1.0,
        // 				HsiType(Hsi {
        // 					hue_rotate: 0.0,
        // 					saturate: 0.0,
        // 					bright_ness: 0.5,
        // 				}),
        // 			);
        // 		}
        // 		self.flag =!self.flag;

        // 	}
        // }

        
    }
}
