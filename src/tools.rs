
use pi_flex_layout::style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension};
use pi_style::{style::{AnimationDirection, AnimationTimingFunction, BorderRadius, Color, FitType, FontStyle, ImageRepeatOption, LengthUnit, LineHeight, TextAlign, VerticalAlign, WhiteSpace}, style_parse::Attribute};
use pi_style::style::TransformOrigin;
use pi_curves::steps::EStepMode;
use tracing_subscriber::fmt::format;

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