//! 定义支持的样式属性，每个枚举值，都对应一个属性设置接口
//! 将支持的属性按照样式的尺寸划分为三类。


use smallvec::SmallVec;

use pi_flex_layout::style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension};
use crate::components::user::{ObjectFit, TextAlign, VerticalAlign, WhiteSpace, FontStyle, Enable, Blur, LineHeight, FontSize, Opacity, BorderImageRepeat, MaskImage, BlendMode, BackgroundColor, BorderColor, BoxShadow, BackgroundImageClip, BorderImageClip, BorderImageSlice, Color, Hsi, TransformOrigin, TransformFunc, BorderRadius, Stroke, TextShadow, BackgroundImage, BorderImage, MaskImageClip};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Attribute {
    PositionType(PositionType),
    FlexWrap(FlexWrap),
    FlexDirection(FlexDirection),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    JustifyContent(JustifyContent),

    ObjectFit(ObjectFit),
    TextAlign(TextAlign),
    VerticalAlign(VerticalAlign),
    WhiteSpace(WhiteSpace),
    FontStyle(FontStyle),
    Enable(Enable),
    Display(Display),

    Visibility(bool),
    Overflow(bool),
    LetterSpacing(f32),
    TextIndent(f32),
    WordSpacing(f32),
    FontWeight(usize),
    FontFamily(usize),
    ZIndex(isize),
	BackgroundImage(BackgroundImage),
    BorderImage(BorderImage),
	FlexShrink(f32),
    FlexGrow(f32),

    Opacity(Opacity),
    BorderImageRepeat(BorderImageRepeat),
	FontSize(FontSize),
	Blur(Blur),
	LineHeight(LineHeight),
	
    Width(Dimension),
    Height(Dimension),
    MarginLeft(Dimension),
    MarginTop(Dimension),
    MarginBottom(Dimension),
    MarginRight(Dimension),
    Margin(Dimension),
    PaddingLeft(Dimension),
    PaddingTop(Dimension),
    PaddingBottom(Dimension),
    PaddingRight(Dimension),
    Padding(Dimension),
    BorderLeft(Dimension),
    BorderTop(Dimension),
    BorderBottom(Dimension),
    BorderRight(Dimension),
    Border(Dimension),
    MinWidth(Dimension),
    MinHeight(Dimension),
    MaxHeight(Dimension),
    MaxWidth(Dimension),
    FlexBasis(Dimension),
    PositionLeft(Dimension),
    PositionTop(Dimension),
    PositionRight(Dimension),
    PositionBottom(Dimension),

	MaskImage(MaskImage),
	BlendMode(BlendMode),

	BackgroundColor(BackgroundColor),
    BorderColor(BorderColor),
    BoxShadow(BoxShadow),

    ImageClip(BackgroundImageClip),

    BorderImageClip(BorderImageClip),
    BorderImageSlice(BorderImageSlice),

    Color(Color),
    TextShadow(SmallVec<[TextShadow;1]>),
    TextStroke(Stroke),

    BorderRadius(BorderRadius),
    Transform(Vec<TransformFunc>),
    TransformOrigin(TransformOrigin),
    Hsi(Hsi),

	MaskImageClip(MaskImageClip),
}