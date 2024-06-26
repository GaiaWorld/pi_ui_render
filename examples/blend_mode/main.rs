// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;


use framework::{Param, Example};
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::{
    prelude::Size,
    style::{Dimension, FlexWrap, PositionType},
};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, BlendMode, CgColor, Point2},
    style_type::{
        AlignContentType, AlignItemsType, BackgroundImageType, BlendModeType,
        FlexWrapType, HeightType, JustifyContentType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType,
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

#[derive(Default)]
pub struct QuadExample {
    cmd: UserCommands,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
	web_sys::console::log_1(&"blend_mode===========".into());
	framework::start(QuadExample::default());
}

impl Example for QuadExample {
    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        Some(Size { width: 1020, height: 960 })
    }

    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(0.0, 0.0, 0.0, 1.0), true), root));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        world.user_cmd
            .set_style(root, JustifyContentType(pi_flex_layout::style::JustifyContent::Center));
        world.user_cmd.set_style(root, AlignItemsType(pi_flex_layout::style::AlignItems::Center));
        world.user_cmd.set_style(root, AlignContentType(pi_flex_layout::style::AlignContent::Center));
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
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(510.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(480.0)));
        world.user_cmd.set_style(div1, PositionTypeType(PositionType::Absolute));
        world.user_cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/z_source/chouka_shitou_1.png")));
        world.user_cmd.append(div1, root);

        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(450.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(600.0)));
        world.user_cmd.set_style(div2, BlendModeType(BlendMode::AlphaAdd));
        world.user_cmd.set_style(div2, PositionTypeType(PositionType::Absolute));
        world.user_cmd
            .set_style(div2, BackgroundImageType(Atom::from("examples/z_source/6.png")));
        world.user_cmd.append(div2, root);
    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
