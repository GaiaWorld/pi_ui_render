// 一个简单BorderImage

#[path = "../framework.rs"]
mod framework;

use async_trait::async_trait;
use framework::Example;
use ordered_float::NotNan;
use pi_atom::Atom;
/// 渲染四边形 demo
use pi_ecs::prelude::Id;
use pi_flex_layout::style::{Dimension, FlexWrap, PositionType};
use pi_null::Null;
use pi_style::{
    style_type::{
        BorderBottomType, BorderImageRepeatType, BorderImageSliceType, BorderImageType, BorderLeftType, BorderRightType, BorderTopType, FlexWrapType,
        HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType,
    },
};
use pi_ui_render::{
    components::user::{BorderImageSlice, CgColor, ImageRepeat, ImageRepeatOption, ClearColor},
    export::Engine,
};

fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

#[async_trait]
impl Example for QuadExample {
    async fn init(&mut self, gui: &mut Engine, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0), true));

        // 添加根节点
        let root = gui.gui.create_node();
        gui.gui.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        gui.gui.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        gui.gui.set_style(root, PositionTypeType(PositionType::Absolute));
        gui.gui.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root, PositionTopType(Dimension::Points(0.0)));
        gui.gui.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        gui.gui.set_style(root, MarginTopType(Dimension::Points(0.0)));
        gui.gui.set_style(root, FlexWrapType(FlexWrap::Wrap));
        gui.gui.append(root, Id::null());

        // repeat 整数倍数
        let div2 = gui.gui.create_node();
        gui.gui.set_style(div2, WidthType(Dimension::Points(200.0)));
        gui.gui.set_style(div2, HeightType(Dimension::Points(200.0)));
        gui.gui.set_style(div2, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div2, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div2,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div2, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div2, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div2, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div2, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div2,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        gui.gui.append(div2, root);

        // repeat 非整数倍数
        let div3 = gui.gui.create_node();
        gui.gui.set_style(div3, WidthType(Dimension::Points(220.0)));
        gui.gui.set_style(div3, HeightType(Dimension::Points(220.0)));
        gui.gui.set_style(div3, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div3, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div3,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div3, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div3, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div3, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div3, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div3,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        gui.gui.append(div3, root);

        // space 非整数倍数
        let div4 = gui.gui.create_node();
        gui.gui.set_style(div4, WidthType(Dimension::Points(220.0)));
        gui.gui.set_style(div4, HeightType(Dimension::Points(220.0)));
        gui.gui.set_style(div4, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div4, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div4,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div4, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div4, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div4, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div4, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div4,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        gui.gui.append(div4, root);

        // round 非整数倍数
        let div5 = gui.gui.create_node();
        gui.gui.set_style(div5, WidthType(Dimension::Points(220.0)));
        gui.gui.set_style(div5, HeightType(Dimension::Points(220.0)));
        gui.gui.set_style(div5, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div5, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div5,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div5, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div5, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div5, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div5, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div5,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        gui.gui.append(div5, root);

        // 测试中间不足一倍的情况 repeat
        let div6 = gui.gui.create_node();
        gui.gui.set_style(div6, WidthType(Dimension::Points(95.0)));
        gui.gui.set_style(div6, HeightType(Dimension::Points(95.0)));
        gui.gui.set_style(div6, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div6, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div6,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div6, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div6, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div6, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div6, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div6,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Repeat,
            }),
        );
        gui.gui.append(div6, root);

        // 测试中间不足一倍的情况 round
        let div7 = gui.gui.create_node();
        gui.gui.set_style(div7, WidthType(Dimension::Points(95.0)));
        gui.gui.set_style(div7, HeightType(Dimension::Points(95.0)));
        gui.gui.set_style(div7, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div7, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div7,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div7, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div7, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div7, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div7, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div7,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Round,
                y: ImageRepeatOption::Round,
            }),
        );
        gui.gui.append(div7, root);

        // 测试中间不足一倍的情况 space
        let div8 = gui.gui.create_node();
        gui.gui.set_style(div8, WidthType(Dimension::Points(95.0)));
        gui.gui.set_style(div8, HeightType(Dimension::Points(95.0)));
        gui.gui.set_style(div8, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div8, BorderImageType(Atom::from("examples/border_image/source/border.png")));
        gui.gui.set_style(
            div8,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.33333) },
                right: unsafe { NotNan::new_unchecked(0.33333) },
                bottom: unsafe { NotNan::new_unchecked(0.33333) },
                left: unsafe { NotNan::new_unchecked(0.33333) },
                fill: true,
            }),
        );
        gui.gui.set_style(div8, BorderTopType(Dimension::Points(40.0)));
        gui.gui.set_style(div8, BorderRightType(Dimension::Points(40.0)));
        gui.gui.set_style(div8, BorderBottomType(Dimension::Points(40.0)));
        gui.gui.set_style(div8, BorderLeftType(Dimension::Points(40.0)));
        gui.gui.set_style(
            div8,
            BorderImageRepeatType(ImageRepeat {
                x: ImageRepeatOption::Space,
                y: ImageRepeatOption::Space,
            }),
        );
        gui.gui.append(div8, root);


		// 测试top\bottom为0，并且为Stretch的情况
        let div9 = gui.gui.create_node();
        gui.gui.set_style(div9, WidthType(Dimension::Points(448.0)));
        gui.gui.set_style(div9, HeightType(Dimension::Points(62.0)));
        gui.gui.set_style(div9, PositionTypeType(PositionType::Relative));
        gui.gui
            .set_style(div9, BorderImageType(Atom::from("examples/border_image/source/chuangjianjuese_shuxingbg.png")));
        gui.gui.set_style(
            div9,
            BorderImageSliceType(BorderImageSlice {
                top: unsafe { NotNan::new_unchecked(0.0) },
                right: unsafe { NotNan::new_unchecked(0.4464) },
                bottom: unsafe { NotNan::new_unchecked(0.0) },
                left: unsafe { NotNan::new_unchecked(0.4464) },
                fill: true,
            }),
        );
        gui.gui.set_style(div9, BorderTopType(Dimension::Points(0.0)));
        gui.gui.set_style(div9, BorderRightType(Dimension::Points(200.0)));
        gui.gui.set_style(div9, BorderBottomType(Dimension::Points(0.0)));
        gui.gui.set_style(div9, BorderLeftType(Dimension::Points(200.0)));
        gui.gui.append(div9, root);
    }

    fn render(&mut self, gui: &mut Engine) { gui.gui.run(); }
}
