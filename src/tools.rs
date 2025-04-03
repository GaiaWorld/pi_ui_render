
use pi_flex_layout::style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension};
use pi_style::{style::{BorderRadius, Color, FitType, FontStyle, ImageRepeatOption, LengthUnit, LineHeight, TextAlign, VerticalAlign, WhiteSpace}, style_parse::Attribute};

pub fn to_css_str(attr: &Attribute) -> (&'static str, String) {
    match attr {
        Attribute::ClipPath(r) => ("clip-path", match &r.0 {
            pi_style::style::BaseShape::Circle { radius, center } => format!("circle({})", len_to_str(radius) + " at " + len_to_str(&center.x).as_str() + " " + len_to_str(&center.y).as_str()),
            pi_style::style::BaseShape::Ellipse { rx, ry, center } => format!("ellipse({})", len_to_str(rx) + " " + len_to_str(ry).as_str() + " at " + len_to_str(&center.x).as_str() + " " + len_to_str(&center.y).as_str()),
            pi_style::style::BaseShape::Inset { rect_box, border_radius } => todo!(),
            pi_style::style::BaseShape::Sector { rotate, angle, radius, center } => todo!(),
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
        Attribute::FontSize(_r) => ("","".to_string()), // TODO
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

        Attribute::BorderRadius(_r) => ("", "".to_string()),    // TODO
        Attribute::TransformOrigin(_r) => ("", "".to_string()), // TODO
        Attribute::Hsi(_r) => ("", "".to_string()),
        Attribute::BorderImageRepeat(r) => ("border-image-repeat", format!("{:?}", r.x) + " " + format!("{:?}", r.y).as_str()),
        Attribute::Blur(r) => ("blur", r.0.to_string() + "px"),
        Attribute::MaskImage(r) => ("mask-image",format!("{:?}", r.0)),
        Attribute::Transform(_r) => ("", "".to_string()),               // TODO
		Attribute::Translate(_r) => ("", "".to_string()),               // TODO
		Attribute::Scale(_r) => ("", "".to_string()),               // TODO
		Attribute::Rotate(_r) => ("", "".to_string()),               // TODO
        Attribute::TransformWillChange(_r) => ("", "".to_string()),     // TODO
        Attribute::BlendMode(_r) => ("", "".to_string()),               // TODO
        Attribute::Direction(_r) => ("", "".to_string()),               // TODO
        Attribute::AspectRatio(_r) => ("", "".to_string()),             // TODO
        Attribute::Order(_r) => ("", "".to_string()),                   // TODO
        Attribute::TextContent(_r) => ("", "".to_string()),             // TODO
        Attribute::VNode(_r) => ("", "".to_string()),                   // TODO
        Attribute::AnimationName(_r) => ("", "".to_string()),           // TODO
        Attribute::AnimationDuration(_r) => ("", "".to_string()),       // TODO
        Attribute::AnimationTimingFunction(_r) => ("", "".to_string()), // TODO
        Attribute::AnimationDelay(_r) => ("", "".to_string()),          // TODO
        Attribute::AnimationIterationCount(_r) => ("", "".to_string()), // TODO
        Attribute::AnimationDirection(_r) => ("", "".to_string()),      // TODO
        Attribute::AnimationFillMode(_r) => ("", "".to_string()),       // TODO
        Attribute::AnimationPlayState(_r) => ("", "".to_string()),
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

// fn border_radius_to_str(value: &BorderRadius) -> String {
//     format!();
// }

// pub struct BorderRadius {
//     pub x: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的x半径
// 	pub y: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的y半径
// }