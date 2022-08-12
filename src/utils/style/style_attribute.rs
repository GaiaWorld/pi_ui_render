//! 定义支持的样式属性，每个枚举值，都对应一个属性设置接口
//! 将支持的属性按照样式的尺寸划分为三类。


use smallvec::SmallVec;

use pi_flex_layout::style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension};
use crate::components::user::{ObjectFit, TextAlign, VerticalAlign, WhiteSpace, FontStyle, Enable, Blur, LineHeight, FontSize, Opacity, BorderImageRepeat, MaskImage, BlendMode, BackgroundColor, BorderColor, BoxShadow, BackgroundImageClip, BorderImageClip, BorderImageSlice, Color, Hsi, TransformOrigin, TransformFunc, BorderRadius, Stroke, TextShadow, BackgroundImage, BorderImage, MaskImageClip};

use super::style_sheet::{PositionTypeType, FlexWrapType, VisibilityType, LetterSpacingType, TextIndentType, WordSpacingType, FontWeightType, FontFamilyType, ZIndexType, FlexShrinkType, FlexGrowType, OpacityType, BorderImageRepeatType, FontSizeType, BlurType, LineHeightType, MarginLeftType, MarginTopType, MarginBottomType, MarginRightType, PaddingLeftType, PaddingTopType, PaddingBottomType, PaddingRightType, PaddingType, BorderLeftType, BorderTopType, BorderBottomType, BorderRightType, BorderType, MinWidthType, MinHeightType, MaxHeightType, MaxWidthType, FlexBasisType, PositionLeftType, PositionTopType, PositionRightType, PositionBottomType, MaskImageType, BlendModeType, BackgroundColorType, BorderColorType, BoxShadowType, BackgroundImageClipType, BorderImageClipType, BorderImageSliceType, ColorType, TextStrokeType, TextShadowType, TransformType, TransformOriginType, MaskImageClipType, FlexDirectionType, AlignContentType, AlignItemsType, AlignSelfType, JustifyContentType, ObjectFitType, VerticalAlignType, WhiteSpaceType, EnableType, FontStyleType, DisplayType, OverflowType, TextAlignType, BackgroundImageType, BorderImageType, WidthType, HeightType, MarginType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Attribute {
    PositionType(PositionTypeType),
    FlexWrap(FlexWrapType),
    FlexDirection(FlexDirectionType),
    AlignContent(AlignContentType),
    AlignItems(AlignItemsType),
    AlignSelf(AlignSelfType),
    JustifyContent(JustifyContentType),

    ObjectFit(ObjectFitType),
    TextAlign(TextAlignType),
    VerticalAlign(VerticalAlignType),
    WhiteSpace(WhiteSpaceType),
    FontStyle(FontStyleType),
    Enable(EnableType),
    Display(DisplayType),

    Visibility(VisibilityType),
    Overflow(OverflowType),
    LetterSpacing(LetterSpacingType),
    TextIndent(TextIndentType),
    WordSpacing(WordSpacingType),
    FontWeight(FontWeightType),
    FontFamily(FontFamilyType),
    ZIndex(ZIndexType),
	BackgroundImage(BackgroundImageType),
    BorderImage(BorderImageType),
	FlexShrink(FlexShrinkType),
    FlexGrow(FlexGrowType),

    Opacity(OpacityType),
    BorderImageRepeat(BorderImageRepeatType),
	FontSize(FontSizeType),
	Blur(BlurType),
	LineHeight(LineHeightType),
	
    Width(WidthType),
    Height(HeightType),
    MarginLeft(MarginLeftType),
    MarginTop(MarginTopType),
    MarginBottom(MarginBottomType),
    MarginRight(MarginRightType),
    Margin(MarginType),
    PaddingLeft(PaddingLeftType),
    PaddingTop(PaddingTopType),
    PaddingBottom(PaddingBottomType),
    PaddingRight(PaddingRightType),
    Padding(PaddingType),
    BorderLeft(BorderLeftType),
    BorderTop(BorderTopType),
    BorderBottom(BorderBottomType),
    BorderRight(BorderRightType),
    Border(BorderType),
    MinWidth(MinWidthType),
    MinHeight(MinHeightType),
    MaxHeight(MaxHeightType),
    MaxWidth(MaxWidthType),
    FlexBasis(FlexBasisType),
    PositionLeft(PositionLeftType),
    PositionTop(PositionTopType),
    PositionRight(PositionRightType),
    PositionBottom(PositionBottomType),

	MaskImage(MaskImageType),
	BlendMode(BlendModeType),

	BackgroundColor(BackgroundColorType),
    BorderColor(BorderColorType),
    BoxShadow(BoxShadowType),

    ImageClip(BackgroundImageClipType),

    BorderImageClip(BorderImageClipType),
    BorderImageSlice(BorderImageSliceType),

    Color(ColorType),
    TextShadow(TextShadowType),
    TextStroke(TextStrokeType),

    BorderRadius(BorderRadius),
    Transform(TransformType),
    TransformOrigin(TransformOriginType),
    Hsi(Hsi),

	MaskImageClip(MaskImageClipType),
}