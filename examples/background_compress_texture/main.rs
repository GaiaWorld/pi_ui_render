// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;


use framework::{Param, Example};
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, FlexWrap, PositionType};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, CgColor, Point2},
    style_type::{
        BackgroundImageType, FlexWrapType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType,
    },
};
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{ClearColor, RenderDirty, Viewport},

    },
    resource::{NodeCmd, UserCommands},
};
use pi_ui_render::resource::fragment::NodeTag;

fn main() { framework::start(QuadExample::default()) }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
	web_sys::console::log_1(&"background_compress_texture===========".into());
	framework::start(QuadExample::default());
}


#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true), root));
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
        world.user_cmd.set_style(root, FlexWrapType(FlexWrap::Wrap));
		world.user_cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root, EntityKey::null().0);

        let div1 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div1, PositionTypeType(PositionType::Relative));
        world.user_cmd.set_style(
            div1,
            BackgroundImageType(Atom::from("examples/z_source/bx_lanseguanbi.s3tc.ktx")),
        );
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(150.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(80.0)));
        world.user_cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
