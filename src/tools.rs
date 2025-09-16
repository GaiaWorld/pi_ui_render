
use std::mem::transmute;

use pi_flex_layout::style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension};
use pi_null::Null;
use pi_style::{style::{AnimationDirection, AnimationTimingFunction, BorderRadius, Color, FitType, FontStyle, ImageRepeatOption, LengthUnit, LineHeight, Point2, TextAlign, VerticalAlign, WhiteSpace}, style_parse::Attribute};
use pi_style::style::TransformOrigin;
use pi_curves::steps::EStepMode;

use pi_world::{
    prelude::{Entity}, 
    world::World,
};
use crate::{components::{calc::{InPassId, IsShow, ZRange}, pass_2d::ParentPassId, user::Overflow}, devtools::{get_global_info, node_info}, resource::{draw_obj::{create_common_pipeline_state, LastGraphNode}, IsRun, QuadTree}};
use crate::devtools::{get_style, get_class_names, get_class};
use crate::components::calc::Quad;
use crate::components::calc::EntityKey;
use crate::components::user::Aabb2;
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};
use pi_bevy_ecs_extend::prelude::{Down, EntityTag, Layer, Up};
use pi_bevy_render_plugin::{node::Node, PiRenderDevice, PiScreenTexture, RenderContext, PiRenderGraph};

use pi_hal::font::sdf_gpu::create_indices;
use pi_share::ShareRefCell;
use pi_spatial::quad_helper::intersects;
use pi_world::{
    single_res::{SingleRes, SingleResMut}, query::Query
};
use pi_futures::BoxFuture;


use wgpu::util::DeviceExt;
use wgpu::CommandEncoder;

use pi_bevy_render_plugin::{Cmd};


pub fn to_css_str(attr: &Attribute) -> (&'static str, String) {
    match attr {
        Attribute::ClipPath(r) => ("clip-path", match &r.0 {
            pi_style::style::BaseShape::Circle { radius, center } => format!("circle({})", len_to_str(radius) + " at " + len_to_str(&center.x).as_str() + " " + len_to_str(&center.y).as_str()),
            pi_style::style::BaseShape::Ellipse { rx, ry, center } => format!("ellipse({})", len_to_str(rx) + " " + len_to_str(ry).as_str() + " at " + len_to_str(&center.x).as_str() + " " + len_to_str(&center.y).as_str()),
            pi_style::style::BaseShape::Inset { rect_box, border_radius } => format!("inset({})", len_to_str(&rect_box[0]) + " " + len_to_str(&rect_box[1]).as_str()+ " " + len_to_str(&rect_box[2]).as_str()+ " " + len_to_str(&rect_box[3]).as_str() + " round " + border_radius_to_str(border_radius).as_str()),
            pi_style::style::BaseShape::Sector { rotate, angle, radius, center } =>  format!("sector({} {} {} at {} {})", rotate, angle, len_to_str(radius), len_to_str(&center.x), len_to_str(&center.y) ),
        }),
		Attribute::AsImage(r) => ("as-image", match r.0 {
			pi_style::style::AsImage::None => "none".to_string(),
			pi_style::style::AsImage::Advise => "advise".to_string(),
			pi_style::style::AsImage::Force => "force".to_string(),
		}),
        Attribute::PositionType(r) => ("position", match r.0 {
            PositionType::Relative => "relative".to_string(),
            PositionType::Absolute => "absolute".to_string(),
            // PositionType::Fixed => "fixed".to_string(),
        }),
        Attribute::FlexWrap(r) => ("flex-wrap", match r.0 {
            FlexWrap::NoWrap => "nowrap".to_string(),
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse => "wrapreverse".to_string(),
        }),
        Attribute::FlexDirection(r) => ("flex-direction", match r.0 {
            FlexDirection::Column => "column".to_string(),
            FlexDirection::ColumnReverse => "columnreverse".to_string(),
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "rowreverse".to_string(),
        }),
        Attribute::AlignContent(r) => ("align-content", match r.0 {
            // AlignContent::Auto => "auto".to_string(),
            AlignContent::FlexStart => "flex-start".to_string(),
            AlignContent::Center => "center".to_string(),
            AlignContent::FlexEnd => "flex-end".to_string(),
            AlignContent::Stretch => "stretch".to_string(),
            // AlignContent::Baseline => "baseline".to_string(),
            AlignContent::SpaceBetween => "space-between".to_string(),
            AlignContent::SpaceAround => "space-around".to_string(),
        }),
        Attribute::AlignItems(r) => ("align-items", match r.0 {
            // AlignItems::Auto => "auto".to_string(),
            AlignItems::FlexStart => "flex-start".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::FlexEnd => "flex-end".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            // AlignItems::SpaceBetween => "space-between".to_string(),
            // AlignItems::SpaceAround => "space-around".to_string(),
        }),
        Attribute::AlignSelf(r) => ("align-self", match r.0 {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::FlexStart => "flex-start".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::FlexEnd => "flex-end".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            // AlignSelf::SpaceBetween => "space-between".to_string(),
            // AlignSelf::SpaceAround => "space-around".to_string(),
        }),
        Attribute::JustifyContent(r) => ("justify-content", match r.0 {
            JustifyContent::FlexStart => "flex-start".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::FlexEnd => "flex-end".to_string(),
            JustifyContent::SpaceBetween => "space-between".to_string(),
            JustifyContent::SpaceAround => "space-around".to_string(),
            JustifyContent::SpaceEvenly => "space-evenly".to_string(),
        }),

        Attribute::ObjectFit(r) => ("object-fit", match r.0 {
            FitType::None => "none".to_string(),
            FitType::Fill => "fill".to_string(),
            FitType::Contain => "contain".to_string(),
            FitType::Cover => "cover".to_string(),
            FitType::ScaleDown => "scale-down".to_string(),
            // FitType::Repeat => "repeat".to_string(),
            // FitType::RepeatX => "repeat-x".to_string(),
            // FitType::RepeatY => "repeat-y".to_string(),
        }),

        Attribute::BackgroundRepeat(r) => ("background-repeat", {
                match r.x {
                    ImageRepeatOption::Stretch => "stretch ",
                    ImageRepeatOption::Repeat => "repeat ",
                    ImageRepeatOption::Round => "round ",
                    ImageRepeatOption::Space => "space ",
                }.to_string()
                + match r.y {
                    ImageRepeatOption::Stretch => "stretch",
                    ImageRepeatOption::Repeat => "repeat",
                    ImageRepeatOption::Round => "round",
                    ImageRepeatOption::Space => "space",
                }
        }),
        Attribute::TextAlign(r) =>("text-align", match r.0 {
            TextAlign::Left => "left".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Justify => "justify".to_string(),
        }),
        Attribute::VerticalAlign(r) => ("vertical-align", match r.0 {
            VerticalAlign::Top => "top".to_string(),
            VerticalAlign::Middle => "middle".to_string(),
            VerticalAlign::Bottom => "bottom".to_string(),
        }),
        Attribute::WhiteSpace(r) => ("white-space", match r.0 {
            WhiteSpace::Normal => "normal".to_string(),
            WhiteSpace::Nowrap => "nowrap".to_string(),
            WhiteSpace::PreWrap => "pre-wrap".to_string(),
            WhiteSpace::Pre => "pre".to_string(),
            WhiteSpace::PreLine => "pre-line".to_string(),
        }),
        Attribute::FontStyle(r) => ("font-style", match r.0 {
            FontStyle::Normal => "normal".to_string(),
            FontStyle::Ttalic => "ttalic".to_string(),
            FontStyle::Oblique => "oblique".to_string(),
        }),
        Attribute::Enable(r) => ("enable", match r.0 {
            pi_style::style::Enable::Auto => "auto".to_string(),
            pi_style::style::Enable::None => "none".to_string(),
            pi_style::style::Enable::Visible => "visible".to_string(),
        }),
        Attribute::Display(r) => ("display", match r.0 {
            Display::Flex => "flex".to_string(),
            Display::None => "none".to_string(),
            // Display::Grid => "Grid".to_string(),
        }),
        Attribute::Visibility(r) => ("visibility", match r.0 {
            true => "visible".to_string(),
            false => "hidden".to_string(),
        }),
        Attribute::Overflow(r) => ("overflow", match r.0 {
            true => "hidden".to_string(),
            false => "visible".to_string(),
        }),
		// "[-a-zA-Z]*:
        Attribute::LetterSpacing(r) => ("letter-spacing", r.to_string()),
        Attribute::LineHeight(r) => ("line-height",match r.0 {
            LineHeight::Normal => "normal".to_string(),
            LineHeight::Length(r) => r.to_string() + "px",
            LineHeight::Number(r) => r.to_string(),
            LineHeight::Percent(r) => (r * 100.0).to_string() + "%",
        }),
        Attribute::TextIndent(r) => ("text-indent", r.to_string() + "px"),
        Attribute::WordSpacing(r) => ("word-space", r.to_string() + "px"),
        Attribute::FontWeight(r) => ("font-weight", r.to_string()),
        Attribute::FontSize(r) => ("font-size", match &r.0 {
            pi_style::style::FontSize::None => "normal".to_string(),
            pi_style::style::FontSize::Length(r) => format!("{}px", r),
            pi_style::style::FontSize::Percent(r) => format!("{}%", r),
        }),
        Attribute::FontFamily(r) => ("font-family", r.to_string()),
        Attribute::ZIndex(r) => ("z-index", r.to_string()),
        Attribute::Opacity(r) => ("opacity", r.0.to_string()),
        // Attribute::BorderImageRepeat(BorderImageRepeat)(x, y) => "" + r.to_string() + " " +,
        Attribute::BackgroundImage(r) => ("baskground-image-source", r.to_string()),
        Attribute::BorderImage(r) => ("border-image-source", r.to_string()),

        Attribute::FlexShrink(r) => ("flex-shrink", r.to_string()),
        Attribute::FlexGrow(r) => ("flex-grow", r.to_string()),
        Attribute::Width(r) => ("width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::Height(r) => ("height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginLeft(r) => ("margin-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginTop(r) => ("margin-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginBottom(r) => ("margin-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginRight(r) => ("margin-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingLeft(r) => ("padding-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingTop(r) => ("padding-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingBottom(r) => ("padding-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingRight(r) => ("padding-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderLeft(r) => ("border-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderTop(r) => ("border-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderBottom(r) => ("border-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderRight(r) => ("border-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        // Attribute::Border(r) => ("visibility",match r.0 {
        //     Dimension::Undefined => "".to_string(),
        //     Dimension::Auto => "auto".to_string(),
        //     Dimension::Points(r) =>  r.to_string() + "px",
        //     Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        // },
        Attribute::MinWidth(r) => ("min-width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MinHeight(r) => ("min-height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MaxHeight(r) => ("max-height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MaxWidth(r) => ("max-width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::FlexBasis(r) => ("flex-basis",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionLeft(r) => ("left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionTop(r) => ("top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionRight(r) => ("right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionBottom(r) => ("bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BackgroundColor(color) => ("background-color", match &color.0 {
            Color::RGBA(r) => {
                "rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "linear-gradient".to_string(),
        }),

        Attribute::BorderColor(r) => ("border-color",{
            let r = &r.0;
            "rgba(".to_string()
                + r.x.to_string().as_str()
                + ","
                + r.y.to_string().as_str()
                + ","
                + r.z.to_string().as_str()
                + ","
                + r.w.to_string().as_str()
                + ")"
        }),
        Attribute::BoxShadow(r) => ("box-shadow",{
            r.h.to_string()
                + " "
                + r.v.to_string().as_str()
                + " "
                + r.blur.to_string().as_str()
                + " "
                + r.spread.to_string().as_str()
                + " rgba("
                + r.color.x.to_string().as_str()
                + ","
                + r.color.y.to_string().as_str()
                + ","
                + r.color.z.to_string().as_str()
                + ","
                + r.color.w.to_string().as_str()
                + ")"
            // pub h: f32,         // 水平偏移，正右负左
            // pub v: f32,         // 垂直偏移，正下负上
            // pub blur: f32,      // 模糊半径，0代表不模糊，
            // pub spread: f32,    // 阴影扩展，上下左右各加上这个值
            // pub color: CgColor, // 阴影颜色
        }),

        Attribute::BackgroundImageClip(r) => ("image-clip",{
           	(r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),
        Attribute::MaskImageClip(r) => ("mask-image-clip",{
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),

        Attribute::BorderImageClip(r) => ("border-image-clip",{
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),
        Attribute::BorderImageSlice(r) => ("border-image-slice",{
            let mut f = "";
            if r.fill {
                f = " fill";
            }
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
                + f
        }),

        Attribute::Color(r) => ("color",match &r.0 {
            Color::RGBA(r) => {
                "rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "linear-gradient".to_string(),
        }),
        Attribute::TextShadow(r) => ("text-shadow",{
            let mut rr = "".to_string();
            for shadow in r.iter() {
                rr = rr
                    + shadow.h.to_string().as_str()
                    + " "
                    + shadow.v.to_string().as_str()
                    + " "
                    + shadow.blur.to_string().as_str()
                    + " rgba("
                    + shadow.color.x.to_string().as_str()
                    + ","
                    + shadow.color.y.to_string().as_str()
                    + ","
                    + shadow.color.z.to_string().as_str()
                    + ","
                    + shadow.color.w.to_string().as_str()
                    + ","
                    + ")";
            }
            rr
        }),
        Attribute::TextStroke(r) => ("text-stroke",{
            "rgba(".to_string()
                + r.0.color.x.to_string().as_str()
                + ","
                + r.0.color.y.to_string().as_str()
                + ","
                + r.0.color.z.to_string().as_str()
                + ","
                + r.0.color.w.to_string().as_str()
                + ")"
        }),

        Attribute::BorderRadius(_r) => ("border-radius", border_radius_to_str(&_r.0)),
        Attribute::TransformOrigin(_r) => ("transform-origin", match &_r.0 {
            TransformOrigin::Center => "center".to_string(),
            TransformOrigin::XY(x, y) => format!("{} {}", len_to_str(x), len_to_str(y)),
        }),
        Attribute::Hsi(_r) => ("filter", format!("hsi({} {}, {})", _r.hue_rotate,  _r.saturate,  _r.bright_ness)),
        Attribute::BorderImageRepeat(r) => ("border-image-repeat", format!("{:?}", r.x) + " " + format!("{:?}", r.y).as_str()),
        Attribute::Blur(r) => ("blur", r.0.to_string() + "px"),
        Attribute::MaskImage(r) => ("mask-image",format!("{:?}", r.0)),
        Attribute::Transform(_r) => ("transform", "".to_string()),               // TODO
		Attribute::Translate(_r) => ("", "".to_string()),               // TODO
		Attribute::Scale(_r) => ("", "".to_string()),               // TODO
		Attribute::Rotate(_r) => ("", "".to_string()),               // TODO
        Attribute::TransformWillChange(_r) => ("", "".to_string()),     // TODO
        Attribute::BlendMode(_r) => ("", "".to_string()),               // TODO
        Attribute::Direction(_r) => ("", "".to_string()),               // TODO
        Attribute::AspectRatio(_r) => ("", "".to_string()),             // TODO
        Attribute::Order(_r) => ("", "".to_string()),                   // TODO
        Attribute::TextContent(_r) => ("text-content", _r.0.0.to_string()),
        Attribute::VNode(_r) => ("", "".to_string()),                   // TODO
        Attribute::AnimationName(_r) => ("animation", _r.0.value.iter().map(|r| r.as_str().to_string()).collect::<Vec<String>>().join(",")),
        Attribute::AnimationDuration(_r) => ("animation-duration", _r.0.iter().map(|r| format!("{}ms", r.0)).collect::<Vec<String>>().join(",")),
        Attribute::AnimationTimingFunction(_r) => ("animation-timing-function", _r.0.iter().map(|r| match r {
            AnimationTimingFunction::Linear => "linear".to_string(),
            AnimationTimingFunction::Ease(_) => "ease".to_string(),
            AnimationTimingFunction::CubicBezier(0.42, 0.0, 1.0, 1.0) => "ease-in".to_string(),
            AnimationTimingFunction::CubicBezier(0.0, 0.0, 0.58, 1.0) => "ease-out".to_string(),
            AnimationTimingFunction::CubicBezier(r0, r1, r2, r3) => format!("cubic-bezier({}, {}, {}, {})", r0, r1, r2, r3),
            AnimationTimingFunction::Step(1, EStepMode::JumpStart) => "step-start".to_string(),
            AnimationTimingFunction::Step(1, EStepMode::JumpEnd) => "step-end".to_string(),
            AnimationTimingFunction::Step(r, r1) => format!("step({}, {})", r, match r1 {
                EStepMode::JumpStart => "jump-start",
                EStepMode::JumpEnd => "jump-end",
                EStepMode::JumpNone => "jump-none",
                EStepMode::JumpBoth => "jump-both",
            }.to_string()),
        }).collect::<Vec<String>>().join(",")),
        Attribute::AnimationDelay(_r) => ("animation-delay", _r.0.iter().map(|r| format!("{}ms", r.0)).collect::<Vec<String>>().join(",")),
        Attribute::AnimationIterationCount(_r) => ("animation-iteration-count", _r.0.iter().map(|r| format!("{}", r.0)).collect::<Vec<String>>().join(",")),
        Attribute::AnimationDirection(_r) => ("animation-direction", _r.0.iter().map(|r| match r {
            AnimationDirection::Normal => "normal".to_string(),
            AnimationDirection::Reverse => "reverse".to_string(),
            AnimationDirection::Alternate => "alternate".to_string(),
            AnimationDirection::AlternateReverse => "alternate-reverse".to_string(),
        }).collect::<Vec<String>>().join(",")),
        Attribute::AnimationFillMode(_r) => ("animation-fill-mode", _r.0.iter().map(|r| match r {
            pi_style::style::AnimationFillMode::None =>  "none".to_string(),
            pi_style::style::AnimationFillMode::Forwards =>  "forwards".to_string(),
            pi_style::style::AnimationFillMode::Backwards =>  "backwards".to_string(),
            pi_style::style::AnimationFillMode::Both =>  "both".to_string(),
        }).collect::<Vec<String>>().join(",")),
        Attribute::AnimationPlayState(_r) => ("animation-play-state", _r.0.iter().map(|r| match r {
            pi_style::style::AnimationPlayState::Running => "running".to_string(),
            pi_style::style::AnimationPlayState::Paused => "paused".to_string(),
        }).collect::<Vec<String>>().join(",")), 
        Attribute::TextOverflow(r) =>  ("text-overflow", match &r.0 {
            pi_style::style::TextOverflow::None => "none".to_string(),
            pi_style::style::TextOverflow::Clip => "clip".to_string(),
            pi_style::style::TextOverflow::Ellipsis => "ellipsis".to_string(),
            pi_style::style::TextOverflow::Custom(r) => r.clone(),
        }),
        Attribute::OverflowWrap(r) => ("overflow-wrap", match &r.0 {
            pi_flex_layout::style::OverflowWrap::Normal => "normal".to_string(),
            pi_flex_layout::style::OverflowWrap::Anywhere => "anywhere".to_string(),
            pi_flex_layout::style::OverflowWrap::BreakWord => "break-word".to_string(),
        }),
        Attribute::TransitionProperty(_) => ("", "".to_string()), // TODO
        Attribute::TransitionDuration(_) => ("", "".to_string()), // TODO
        Attribute::TransitionTimingFunction(_) => ("", "".to_string()), // TODO
        Attribute::TransitionDelay(_) => ("", "".to_string()), // TODO

        Attribute::TextOuterGlow(r) => ("text-outer-glow", {
            "rgba(".to_string()
                + r.color.x.to_string().as_str()
                + ","
                + r.color.y.to_string().as_str()
                + ","
                + r.color.z.to_string().as_str()
                + ","
                + r.color.w.to_string().as_str()
                + ") "
                + r.distance.to_string().as_str()
                + "px "
                + r.intensity.to_string().as_str()
        }),
        // Attribute::RowGap(r) => ("row-gap", r.0.to_string() + "px"),
        // Attribute::ColumnGap(r) => ("column-gap", r.0.to_string() + "px"),
        // Attribute::AutoReduce(r) => ("auto-reduce", r.0.to_string()),
    }
}


fn len_to_str(value: &LengthUnit) -> String {
    match value {
        LengthUnit::Pixel(r) => r.to_string() + "px",
        LengthUnit::Percent(r) => r.to_string() + "%",
        // LengthUnit::Rem(r) => r.to_string() + "rem",
        // LengthUnit::Em(r) => r.to_string() + "em",
    }
}


fn border_radius_to_str(value: &BorderRadius) -> String {
    format!("{} {} {} {}/{} {} {} {}", 
        len_to_str(&value.x[0]), 
        len_to_str(&value.x[1]), 
        len_to_str(&value.x[2]), 
        len_to_str(&value.x[3]), 
        len_to_str(&value.y[0]), 
        len_to_str(&value.y[1]), 
        len_to_str(&value.y[2]), 
        len_to_str(&value.y[3]), 
    )
}

// pub struct BorderRadius {
//     pub x: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的x半径
// 	pub y: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的y半径
// }

pub fn _request_computed(world: &mut World, nodeid: Entity) -> String {
    let select_node_id: Entity = nodeid;
    if let Some(msg) = node_info(world, select_node_id) {
        let cmd = Cmd {
            cmd: "computed-data".to_string(),
            payload: msg,
        };
        return serde_json::to_string(&cmd).unwrap();
    }
    return String::from("{}");
}

pub fn _request_style(world: &mut World, nodeid: Entity) -> String {
    let _select_node_id: Entity = nodeid;
    let style = get_style(world, _select_node_id);

    let c = style.split(";").collect::<Vec<&str>>();
    let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    for  i in 0..c.len() - 1 {
        let arr = c[i].split(":").collect::<Vec<&str>>();
        style[arr[0]] = arr[1].into();
    }

    let class_name = get_class_names(world, _select_node_id);
    let class_name = serde_json::from_str::<serde_json::Value>(&class_name).unwrap();
    // log::error!("======= class_name: {:?}", class_name);
    let mut classs = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    for class_name in class_name.as_array().unwrap() {
        let class_name = class_name.as_u64().unwrap() as u32;
        let mut r = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let c = get_class(&world, class_name);
        
            let c = c.split(";");
            for a in c {
                let arr = a.split(":").collect::<Vec<&str>>();
                // log::error!("========= arr: {:?}", (&a, &arr));
                if arr.len() > 1 {
                    r[arr[0]] = arr[1].into();
                }
            }
            classs[class_name.to_string()] = r;
        
    }
    format!("{{\"cmd\": \"style-data\", \"payload\": {{\"style\": {}, \"classs\": {} }} }}", style.to_string(), classs)
}

pub fn _request_showbox(world: &mut World, nodeid: Entity) {
    let select_node_id: Entity = nodeid;

    let info = world.get_single_res_mut::<ShowboxInfo>().unwrap();
    if info.id != select_node_id{
        info.id = select_node_id;
    }
}

pub fn _request_right_key_element(world: &mut World, x: f32, y: f32) -> String {
    if let Some((id, root_id)) = lookup_ele_by_pointer(world, x, y){
        let msg = format!("{{\"cmd\": \"right-key-element\" , \"payload\": {{\"uniqueID\": {}, \"documentUniqueID\": {} }}}}", id, root_id);
        // log::error!("========= msg: {}", msg);
        return msg;
    }
    return String::from("{}");
}

pub fn _request_global_interface(world: &mut World) -> String {
    let msg = "{\"cmd\": \"global-info-interface\" , \"payload\": [[\"ExecutionGraph\",\"graph\"],[\"ToopGraph\",\"graph\"],[\"GlobalInfo\",\"json\"]]}";
    return String::from(msg); 
}

pub fn _request_global_info(world: &mut World, request_cmd: &str) -> String {
    let info = match request_cmd {
        "ExecutionGraph" => {
            let g = world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
            g.dump_graphviz()
        },
        "ToopGraph" => {
            let g = world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
            g.dump_toop_graphviz()
        },
        "GlobalInfo" => {
            let info = get_global_info(&world);
            serde_json::to_string(&info).unwrap()
        },
        _ => "".to_string(),
    };
    // let info = "digraph Render {\"\"}";
    // let j = serde_json::from_str::<serde_json::Value>(&info).unwrap();
    // println!("============ request-global-info msg: {:?}", j);
    let msg = format!("{{\"cmd\": \"global-info-data\", \"payload\": {{\"name\":\"{}\", \"data\": {:?}}}}}", request_cmd, info);
    println!("============ request-global-info msg: {}", msg);
    // let j = serde_json::from_str::<serde_json::Value>(&msg).unwrap();
    
    // println!("============ request-global-info msg: {}", j);
    
    return msg;
}

// 初始化渲染图的根节点
pub fn init_show_box_node(
    last_graph_id: OrInitSingleResMut<LastGraphNode>,
    mut rg: SingleResMut<PiRenderGraph>,
	r: OrInitSingleRes<IsRun>
) {
    if r.0 {
		return;
	}
    
    match rg.add_node("show_box".to_string(), ShowBoxNode, Null::null(), Null::null()) {
        Ok(r) => {
            rg.add_depend(last_graph_id.0, r).unwrap();
            let _ = rg.set_finish(r, true);
        },
        Err(e) => log::error!("node: {:?}, {:?}", "show_box".to_string(), e),
    };
    // rg
}

pub struct ShowBoxNode;

impl Node for ShowBoxNode {
    type RunParam = (SingleRes<'static, PiScreenTexture>, SingleResMut<'static, ShowboxInfo>, Query<'static, &'static Quad, ()> );
    type BuildParam = ();
    type ResetParam = ();
    // // 释放纹理占用
    // fn reset<'a>(&'a mut self) {
    //     // self.out_put_target = None;
    //     // self.target = None;
    // }

    /// 用于给pass2d分配fbo
    fn build<'a>(
        &'a mut self,
        // world: &'a mut pi_world::world::World,
        _param: &'a mut Self::BuildParam,
        _context: pi_bevy_render_plugin::RenderContext,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> std::result::Result<(), String> {
        Ok(())
    }

    fn run<'a>(
        &'a mut self,
        param: &'a Self::RunParam,
        _context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> BoxFuture<'a, std::result::Result<(), String>> {
        // println!("=========== run");
        Box::pin(async move {
            if let Ok(quad) = param.2.get(param.1.id) {
                let mut rpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("debug showbox"),
                        color_attachments: &[
                            Some(wgpu::RenderPassColorAttachment {
                                view: param.0.as_ref().unwrap().view().as_ref().unwrap(),
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                            })
                        ],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    }
                );
    
                rpass.set_pipeline(&param.1.pipeline);
                // println!("view: {:?}", (quad.mins.x , quad.mins.y, quad.maxs.x - quad.mins.x, quad.maxs.y - quad.mins.y,));
                rpass.set_viewport(quad.mins.x , quad.mins.y, quad.maxs.x - quad.mins.x, quad.maxs.y - quad.mins.y, 0.0, 1.0);
                // rpass.set_bind_group(0, &bind_group1, &[]);
                // rpass.set_bind_group(1, &bind_group2, &[]);
    
                rpass.set_index_buffer(param.1.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                rpass.set_vertex_buffer(0, param.1.vertex_buffer.slice(..));
    
                rpass.draw_indexed(0..6, 0, 0..1 as u32);
            }
            
            Ok(())
        })
    }
    
    fn reset<'a>(
        &'a mut self,
        param: &'a mut Self::ResetParam,
        context: RenderContext,
        id: Entity,
    ) {

    }
}
pub struct ShowboxInfo{
    pipeline: pi_render::rhi::pipeline::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    id: Entity,
}
pub fn init_showbox_pipeline(world: &mut World){
    let device = world.get_single_res::<PiRenderDevice>().unwrap();
    
    let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Glsl {
            shader: include_str!("./devtools/showbox.vert").into(),
            stage: naga::ShaderStage::Vertex,
            defines: Default::default(),
        },
    });

    // Load the shaders from disk
    let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Glsl {
            shader: include_str!("./devtools/showbox.frag").into(),
            stage: naga::ShaderStage::Fragment,
            defines: Default::default(),
        },
    });

    let vertexs = [
        -1.0f32, -1.0, 
         -1.0, 1.0, 
         1.0, -1.0,
         1.0, 1.0
    ]; // 获取网格数据
    println!("vertexs: {:?}", vertexs);


    // 创建网格数据
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&vertexs),
        usage: wgpu::BufferUsages::VERTEX,
    });


    let index_data = create_indices();
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&index_data),
        usage: wgpu::BufferUsages::INDEX,
    });

    let primitive = wgpu::PrimitiveState::default();

    // primitive.
    // let mut tt: ColorTargetState = swapchain_format.into();
    // tt.blend = Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING);
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let r = create_common_pipeline_state();
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs,
            entry_point: Some("main"),
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
            ],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs,
            entry_point: Some("main"),
            targets: &r.targets,
            compilation_options: Default::default(),
        }),
        primitive,
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });
    world.insert_single_res(ShowboxInfo{
        pipeline: render_pipeline,
        vertex_buffer,
        index_buffer,
        id: Entity::null(),
        // bbox: Aabb2::new(Point2::new(0., 0.), Point2::new(0., 0.))
    });
}



pub struct AbQueryArgs<'a> {
    // query: Query<'s, 'w, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
    // query_parent: Query<'s, 'w, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    world: &'a World,
    aabb: Aabb2,
    result: EntityKey,
    root_id: Entity,
    max_z: usize,
}

fn lookup_ele_by_pointer(world: &mut World, x: f32, y: f32) -> Option<(f64, f64)>{

    let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
    let mut args = AbQueryArgs {
        world: world,
        aabb,
        result: EntityKey::null(),
        root_id:Entity::null(),
        max_z: usize::MIN,
    };
    world.get_single_res::<QuadTree>().unwrap().query(&aabb, intersects, &mut args, ab_query_func);
    if args.result.is_null() {
        None
    } else {
        Some(unsafe {( transmute(args.result), transmute(args.root_id)) })
    }
   
}

/// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, aabb: &Aabb2, _bind: &()) {
	// log::warn!("ab_query_func======={:?}", id);
    let (layer, z_range, inpass) = match (
        arg.world.get_component::<Layer>(*id), 
        arg.world.get_component::<IsShow>(*id), 
        arg.world.get_component::<ZRange>(*id), 
        arg.world.get_component::<InPassId>(*id),
    ) {
        // 如果enable false 表示不接收事件, visibility为false， 也无法接收事件、不在树上也不能接收事件
        (Ok(r0), Ok(r1), Ok(r2), Ok(r3)) if (r0.layer() != 0 && r1.get_enable() && r1.get_visibility() && r1.get_display()) => (r0, r2, r3),
        _ => return,
    };
    if intersects(&arg.aabb, aabb) {
        // 取最大z的node
        if z_range.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
            let mut inpass = *(inpass.0);
            while !inpass.is_null() {
                // log::warn!("inpass======={:?}", (inpass, id));
                if let (Ok(parent), Ok(quad)) = (
                    arg.world.get_component::<ParentPassId>(inpass),
                    arg.world.get_component::<Quad>(inpass),
                ){
                    inpass = parent.0;
                    if let Ok(oveflow) = arg.world.get_component::<Overflow>(inpass) {
                        if oveflow.0 {
                            if !intersects(&arg.aabb, quad) {
                                return; // 如果不想交，直接返回，该点不能命中该节点
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            arg.root_id = layer.root();
            arg.result = id;
            arg.max_z = z_range.start;
        }
    }
}
// /**
//  * 定义公共的数据结构
//  */
// pub enum Cmd {
// 	AddChild = "add-child", // 添加节点
// 	RemoveChild = "remove-child", // 移除节点
// 	updateChild = "update-child", // 添加节点
// 	DocumentData = "document-data", // document的json数据
// 	InitDocument = "init-document", // document初始化成功时，需要发送的指令
// 	StyleData = "style-data", // Style数据
// 	ComputedData = "computed-data",
// 	RenderGraphData = "render-graph-data", // 渲染图数据
// 	SystemGraphData = "system-graph-data", // 系统图数据
// 	RightKeyElement = "right-key-element", // 右键元素id

// 	ShowGuiDevpanel = "show-gui-devpanel",

// 	RequestAll = "request-document", // devpanel被打开，请求显示document
// 	RequestStyle = "request-style", // 请求style数据
// 	RequestComputed = "request-computed", // 请求计算数据
// 	RequestShowbox = "request-showbox", // 请求显示包围盒
// 	RequestRightKeyElement = "request-right-key-element", // 请求右键元素id
// 	RequestRenderGraph = "request-render-graph", // 请求渲染图
// 	RequestSystemGraph = "request-system-graph", // 请求系统图

// 	RequestModifyStyle = "request-modify-style", // 请求修改元素的style

// 	ShowDocument = "show-document", // 显示的document

	
// }

