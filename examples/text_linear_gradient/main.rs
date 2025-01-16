// 一个简单的四边形渲染

#[path = "../framework.rs"]
mod framework;

use framework::{Param, Example};
use pi_atom::Atom;
use pi_flex_layout::style::{Dimension, PositionType};
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, FontSize, RenderDirty, Viewport, LinearGradientColor, ColorAndPosition},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }
use pi_style::{
    style::{Aabb2, Point2, TextContent},
    style_type::{
        BackgroundColorType, ColorType, FontFamilyType, FontSizeType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
        PositionTypeType, TextContentType, WidthType, AsImageType,
    },
};

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
        world.user_cmd
            .set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 1.0, 1.0))));

        world.user_cmd.append(root, EntityKey::null().0);

        // 添加一个渐变颜色的文字
        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, PositionTopType(Dimension::Points(20.0)));
        world.user_cmd.set_style(div1, PositionLeftType(Dimension::Points(20.0)));
        world.user_cmd
            .set_style(div1, TextContentType(TextContent("base02".to_string(), Atom::from("base02"))));
        // rgb(255,0,0) 0px 0px 5px, rgb(255,0,0) 0px 0px 3px, rgb(255,255,255) 0px 0px 1px;
        world.user_cmd.set_style(div1, FontFamilyType(Atom::from("hwkt")));
        world.user_cmd.set_style(div1, ColorType(Color::LinearGradient(LinearGradientColor {
			direction: 0.0,
			list: vec![
				ColorAndPosition {
					position: 0.0,
					rgba: CgColor::new(1.0, 0.0, 0.0, 1.0),
				},
				ColorAndPosition {
					position: 1.0,
					rgba: CgColor::new(0.0, 1.0, 0.0, 1.0),
				},
			],
		})));
        world.user_cmd.set_style(div1, FontSizeType(FontSize::Length(17)));
        // world.user_cmd.set_style(div1, TextStrokeType(Stroke {
        // 	width: unsafe {NotNan::new_unchecked(2.0)},
        // 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
        world.user_cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {
        // world.user_cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
    }
}
