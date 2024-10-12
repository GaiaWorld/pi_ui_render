// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;


use framework::{Param, Example};
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_flex_layout::{
    prelude::Rect,
    style::{Dimension, FlexWrap, PositionType},
};
use pi_null::Null;
use pi_style::{
    style::{Aabb2, CgColor, ImageRepeat, ImageRepeatOption, NotNanRect, Point2, Color},
    style_type::{
        BackgroundImageClipType, BackgroundImageType, BackgroundRepeatType, FlexWrapType, HeightType, MarginLeftType,
        MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType, AsImageType, BackgroundColorType,
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
	web_sys::console::log_1(&"background_image===========".into());
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
        world.user_cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div1, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div1, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        world.user_cmd.append(div1, root);

        // Repeat x轴空间超过一倍但小于两倍， y轴空间不足一倍
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(150.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(80.0)));
        world.user_cmd.set_style(div2, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div2, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
		world.user_cmd
			.set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        world.user_cmd.set_style(
            div2,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        world.user_cmd.append(div2, root);

        // Repeat x轴空间超过一倍但小于两倍， y轴超过一倍但小于两倍
        let div2 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div2, WidthType(Dimension::Points(150.0)));
        world.user_cmd.set_style(div2, HeightType(Dimension::Points(190.0)));
        world.user_cmd.set_style(div2, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div2, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
		world.user_cmd
			.set_style(div2, BackgroundColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
        world.user_cmd.set_style(
            div2,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        world.user_cmd.append(div2, root);


        // Round 
        let div3 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div3, WidthType(Dimension::Points(190.0)));
        world.user_cmd.set_style(div3, HeightType(Dimension::Points(80.0)));
        world.user_cmd.set_style(div3, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div3, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        world.user_cmd.set_style(
            div3,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        world.user_cmd.append(div3, root);

        // space
        let div4 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div4, WidthType(Dimension::Points(190.0)));
        world.user_cmd.set_style(div4, HeightType(Dimension::Points(160.0)));
        world.user_cmd.set_style(div4, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div4, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        world.user_cmd.set_style(
            div4,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        world.user_cmd.append(div4, root);

        // space， x, y空间超过两倍
        let div5 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div5, WidthType(Dimension::Points(300.0)));
        world.user_cmd.set_style(div5, HeightType(Dimension::Points(300.0)));
        world.user_cmd.set_style(div5, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div5, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        world.user_cmd.set_style(
            div5,
            BackgroundRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        world.user_cmd.append(div5, root);

        // imageclip
        let div6 = world.spawn(NodeTag::Div);
        world.user_cmd.set_style(div6, WidthType(Dimension::Points(50.0)));
        world.user_cmd.set_style(div6, HeightType(Dimension::Points(100.0)));
        world.user_cmd.set_style(div6, PositionTypeType(PositionType::Relative));
        world.user_cmd
            .set_style(div6, BackgroundImageType(Atom::from("examples/z_source/dialog_bg.png")));
        world.user_cmd.set_style(
            div6,
            BackgroundImageClipType(NotNanRect(unsafe {
                Rect {
                    top: NotNan::new_unchecked(0.0),
                    right: NotNan::new_unchecked(0.5),
                    bottom: NotNan::new_unchecked(0.5),
                    left: NotNan::new_unchecked(0.0),
                }
            })),
        );
        world.user_cmd.append(div6, root);

    }

    fn render(&mut self, cmd: &mut UserCommands) {  }
}
