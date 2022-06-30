/// 用户操作的组件
/// 遵循css对应属性的语意

use std::default::Default;
use std::{
    hash::{Hash, Hasher},
    mem::transmute,
};

use ordered_float::NotNan;
use pi_flex_layout::style::{AlignContent, Dimension, AlignSelf, PositionType, Direction, AlignItems, FlexWrap, FlexDirection, JustifyContent, Display};
use smallvec::SmallVec;

use pi_atom::Atom;
use pi_flex_layout::prelude::{Size as FlexSize, Number, Rect};

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;
#[derive(Default, Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct CgColor(nalgebra::Vector4<f32>);
pub type Aabb2 = ncollide2d::bounding_volume::AABB<f32>;
pub type NotNanRect = Rect<NotNan<f32>>;

impl Hash for CgColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
		unsafe{
			NotNan::new_unchecked(self.0.x).hash(state);
			NotNan::new_unchecked(self.0.y).hash(state);
			NotNan::new_unchecked(self.0.z).hash(state);
			NotNan::new_unchecked(self.0.w).hash(state);
		}
	}
}

impl CgColor {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self(nalgebra::Vector4::new(x, y, z, w))
	}
}

#[derive(Default)]
pub struct Node;

/// 布局大小
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Size(pub FlexSize<Dimension>);

/// 布局外边距
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Margin(pub Rect<Dimension>);

/// 布局内边距
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Padding(pub Rect<Dimension>);

/// 布局边框尺寸
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Border(pub Rect<Dimension>);

#[derive(Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Position(pub Rect<Dimension>);

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct MinMax{
	pub min: FlexSize<Dimension>,
	pub max: FlexSize<Dimension>,
}

// 描述子节点行为的flex布局属性
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FlexContainer {
	pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
	pub direction: Direction,
}

// 描述节点自身行为的flex布局属性
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FlexNormal {
	pub order: isize,
    pub flex_basis: Dimension,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_self: AlignSelf,
	pub position_type: PositionType,
	pub aspect_ratio: Number,
}

impl Default for Position {
	fn default() -> Self {
		Position(Rect{
			left: Dimension::Undefined,
			right: Dimension::Undefined,
			top: Dimension::Undefined,
			bottom: Dimension::Undefined,
		})
	}
}

impl Default for FlexContainer {
	fn default() -> Self {
		FlexContainer { 
			flex_direction: Default::default(),
			flex_wrap: Default::default(),
			justify_content: Default::default(),
			align_items: Default::default(),
			align_content: AlignContent::FlexStart,
			direction: Default::default(),
		}
	}
}

impl Default for FlexNormal {
	fn default() -> Self {
		Self { 
			order: 0,
			flex_basis: Dimension::Auto,
			flex_grow: Default::default(),
			flex_shrink: Default::default(),
			align_self: Default::default(),
			position_type: Default::default(),
			aspect_ratio: Default::default(),
		}
	}
}

//================================== 组件
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
pub struct ZIndex(pub isize);

//超出部分的裁剪方式
#[derive(Deref, DerefMut, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Overflow(pub bool);
//不透明度
#[derive(Deref, DerefMut, Clone, Debug, Serialize, Deserialize)]
pub struct Opacity(pub f32);

/// 渲染模式
#[derive(Clone, Debug, Serialize, Deserialize, EnumDefault)]
pub enum BlendMode {
	Normal,
	AlphaAdd,
	Subtract,
	Multiply,
	OneOne,
}

// 将display、visibility、enable合并为show组件
#[derive(Deref, DerefMut, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show(pub usize);

// 变换
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Transform {
    pub funcs: Vec<TransformFunc>,
    pub origin: TransformOrigin,
}

pub type TransformFuncs = Vec<TransformFunc>;
// 背景色和class
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref)]
pub struct BackgroundColor(pub Color);

// class名称， 支持多个class， 当只有一个或两个class时， 有优化
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct ClassName(SmallVec<[usize; 1]>);

// 边框颜色
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct BorderColor(pub CgColor);

// 图片路劲及纹理
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default, Hash)]
pub struct BackgroundImage(pub Atom);

// 遮罩图片是图片路径或线性渐变色
#[derive(Clone, Debug, Serialize, Deserialize, EnumDefault)]
pub enum MaskImage {
	Path(usize),
	LinearGradient(LinearGradientColor),
}


#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct MaskImageClip (pub Aabb2);

impl Default for MaskImageClip {
	fn default() -> Self {
		MaskImageClip (Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
	}
}

// 滤镜， 与CSS的Filter不同， 该滤镜不依赖Filter 函数的先后顺序， 且同种滤镜设置多次，会覆盖前面的设置（css是一种叠加效果）
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Hsi {
    pub hue_rotate: f32,  //色相转换  -0.5 ~ 0.5 , 对应ps的-180 ~180
    pub saturate: f32,    // 饱和度  -1。0 ~1.0 ， 对应ps的 -100 ~ 100
    pub bright_ness: f32, //亮度 -1。0 ~1.0 ， 对应ps的 -100 ~ 100
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct Blur(pub f32);

//ObjectFit
#[derive(Debug, Deref, DerefMut, Clone, Default, Serialize, Deserialize)]
pub struct ObjectFit(pub FitType);

// image图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct BackgroundImageClip(pub Aabb2);

// 边框图片
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default, Hash)]
pub struct BorderImage(pub Atom);

// borderImage图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Hash)]
pub struct BorderImageClip(pub NotNanRect);

impl Default for BorderImageClip {
    fn default() -> Self {
        Self(
			NotNanRect {
				left: unsafe { NotNan::new_unchecked(0.0) }, 
				top: unsafe { NotNan::new_unchecked(0.0) },
				right: unsafe { NotNan::new_unchecked(1.0) } ,
				bottom: unsafe { NotNan::new_unchecked(1.0) },
			}
		)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct BorderImageSlice {
    pub top: NotNan<f32>,
    pub right: NotNan<f32>,
    pub bottom: NotNan<f32>,
    pub left: NotNan<f32>,
    pub fill: bool,
}

impl Default for BorderImageSlice {
    fn default() -> Self {
        Self {
			left: unsafe { NotNan::new_unchecked(0.0) }, 
			top: unsafe { NotNan::new_unchecked(0.0) },
			right: unsafe { NotNan::new_unchecked(0.0) },
			bottom: unsafe { NotNan::new_unchecked(0.0) },
			fill: true,
		}
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct BorderImageRepeat(pub BorderImageRepeatOption, pub BorderImageRepeatOption);

// 圆角， 目前仅支持x分量
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BorderRadius {
    pub x: LengthUnit,
    pub y: LengthUnit,
}

// 参考CSS的box-shadow的语法
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoxShadow {
    pub h: f32,         // 水平偏移，正右负左
    pub v: f32,         // 垂直偏移，正下负上
    pub blur: f32,      // 模糊半径，0代表不模糊，
    pub spread: f32,    // 阴影扩展，上下左右各加上这个值
    pub color: CgColor, // 阴影颜色
}

// 文字样式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    pub letter_spacing: f32,     //字符间距， 单位：像素
    pub word_spacing: f32,       //字符间距， 单位：像素
    pub line_height: LineHeight, //设置行高
    pub indent: f32,             // 缩进， 单位： 像素
    pub white_space: WhiteSpace, //空白处理
    pub color: Color,            //颜色
    pub stroke: Stroke,
    pub text_align: TextAlign,
    pub vertical_align: VerticalAlign,
}

// 文本内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextContent(pub String, pub Atom);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextStyle {
	pub color: Color,            //颜色
	pub text_indent: f32,  
	pub text_stroke: Stroke,
    pub text_align: TextAlign,
	pub text_shadow: TextShadows,           // 缩进， 单位： 像素
	pub letter_spacing: f32,     //字符间距， 单位：像素
    pub word_spacing: f32,       //字符间距， 单位：像素
    pub white_space: WhiteSpace, //空白处理
	pub line_height: LineHeight, //设置行高
	pub vertical_align: VerticalAlign,

	pub font_style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    pub font_weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    pub font_size: FontSize,   //
    pub font_family: Atom,     //	规定字体系列。参阅：font-family 中可能的值。
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct TextShadows(pub SmallVec<[TextShadow; 1]>);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextShadow {
    pub h: f32,         //	必需。水平阴影的位置。允许负值。	测试
    pub v: f32,         //	必需。垂直阴影的位置。允许负值。	测试
    pub blur: f32,      //	可选。模糊的距离。	测试
    pub color: CgColor, //	可选。阴影的颜色。参阅 CSS 颜色值。
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    pub weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    pub size: FontSize,   //
    pub family: usize,     //	规定字体系列。参阅：font-family 中可能的值。
}

// TransformWillChange， 用于优化频繁变化的Transform
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransformWillChange(pub Transform);

impl Default for Font {
    fn default() -> Self {
        Self {
            style: FontStyle::default(),
            weight: 500,
            size: FontSize::default(),
            family: 0,
        }
    }
}

// #[derive(Debug)]
// pub struct Quad(pub Point2, pub Point2, pub Point2, pub Point2);

pub enum LengthUnitType {
    Pixel,
    Percent,
}
#[derive(Clone, Copy, Debug, EnumDefault, Serialize, Deserialize)]
pub enum LengthUnit {
    Pixel(f32),
    Percent(f32),
}

// #[derive(Clone, Copy, Debug, EnumDefault, Serialize, Deserialize)]
// pub enum Display {
//     Flex,
//     None,
// }

#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum Color {
    // RGB(CgColor),
    RGBA(CgColor),
    LinearGradient(LinearGradientColor),
    // RadialGradient(RadialGradientColor),
}

impl Color {
    #[inline]
    pub fn is_opaque(&self) -> bool {
        match self {
            Color::RGBA(c) => c.w >= 1.0,
            Color::LinearGradient(l) => {
                for c in l.list.iter() {
                    if c.rgba.w < 1.0 {
                        return false;
                    }
                }
                true
            } // Color::RadialGradient(g) => {
              //     for c in g.list.iter() {
              //         if c.rgba.a < 1.0 {
              //             return false
              //         }
              //     }
              //     return true;
              // }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinearGradientColor {
    pub direction: f32,
    pub list: Vec<ColorAndPosition>,
}

impl Hash for LinearGradientColor {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        NotNan::new(self.direction).unwrap().hash(hasher);
        for l in self.list.iter() {
            NotNan::new(l.position).unwrap().hash(hasher);
            NotNan::new(l.rgba.x).unwrap().hash(hasher);
            NotNan::new(l.rgba.y).unwrap().hash(hasher);
            NotNan::new(l.rgba.z).unwrap().hash(hasher);
            NotNan::new(l.rgba.w).unwrap().hash(hasher);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialGradientColor {
    pub center: (f32, f32),
    pub shape: RadialGradientShape,
    pub size: RadialGradientSize,
    pub list: Vec<ColorAndPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorAndPosition {
	// 位置百分比0~1
    pub position: f32,
    pub rgba: CgColor,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum RadialGradientSize {
    ClosestSide,
    FarthesSide,
    ClosestCorner,
    Farthescorner,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum RadialGradientShape {
    Ellipse,
    Circle,
}
pub type Polygon = Vec<f32>;

// color_and_positions: [r, g, b, a, pos,   r, g, b, a, pos], direction: 0-360度
pub fn to_linear_gradient_color(
    color_and_positions: &[f32],
    direction: f32,
) -> LinearGradientColor {
    let arr = color_and_positions;
    let len = arr.len();
    let count = len / 5;
    let mut list = Vec::with_capacity(count);
    for i in 0..count {
        let start = i * 5;
        let color_pos = ColorAndPosition {
            rgba: CgColor::new(arr[start], arr[start + 1], arr[start + 2], arr[start + 3]),
            position: arr[start + 4],
        };
        list.push(color_pos);
    }
    LinearGradientColor {
        direction: direction,
        list: list,
    }
}

// color_and_positions: [r, g, b, a, pos,   r, g, b, a, pos], center_x: 0~1, center_y: 0~1, shape: RadialGradientShape, size: RadialGradientSize
pub fn to_radial_gradient_color(
    color_and_positions: &[f32],
    center_x: f32,
    center_y: f32,
    shape: u8,
    size: u8,
) -> RadialGradientColor {
    let arr = color_and_positions;
    let len = arr.len();
    let count = len / 5;
    let mut list = Vec::with_capacity(count);
    for i in 0..count {
        let start = i * 5;
        let color_pos = ColorAndPosition {
            rgba: CgColor::new(arr[start], arr[start + 1], arr[start + 2], arr[start + 3]),
            position: arr[start + 4],
        };
        list.push(color_pos);
    }
    RadialGradientColor {
        center: (center_x, center_y),
        shape: unsafe { transmute(shape) },
        size: unsafe { transmute(size) },
        list: list,
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub width: NotNan<f32>,     //	描边宽度
    pub color: CgColor, //	描边颜色
}

// 图像填充的方式
#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum FitType {
    None,
    Fill,
    Contain,
    Cover,
    ScaleDown,
    Repeat,
    RepeatX,
    RepeatY,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize, Hash)]
pub enum BorderImageRepeatOption {
    Stretch, // 拉伸源图像的边缘区域以填充每个边界之间的间隙。
    Repeat, // 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以修剪瓷砖以实现适当的配合。
    Round, // 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以拉伸瓷砖以实现适当的配合。
    Space, // 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以缩小瓷砖以实现适当的配合。
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum FontSize {
    None,         // 默认尺寸。
    Length(usize),  //把 font-size 设置为一个固定的值。
    Percent(f32), //把 font-size 设置为基于父元素的一个百分比值。
}

//设置行高
#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum LineHeight {
    Normal,       //设置合理的行间距（等于font-size）
    Length(f32),  //固定像素
    Number(f32),  //设置数字，此数字会与当前的字体尺寸相乘来设置行间距。
    Percent(f32), //	基于当前字体尺寸的百分比行间距.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformFunc {
    TranslateX(f32),
    TranslateY(f32),
    Translate(f32, f32),

    //平移， 单位： %
    TranslateXPercent(f32),
    TranslateYPercent(f32),
    TranslatePercent(f32, f32),

    ScaleX(f32),
    ScaleY(f32),
    Scale(f32, f32),

	RotateX(f32),
	RotateY(f32),
    RotateZ(f32),

	SkewX(f32),
	SkewY(f32),
}

#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum TransformOrigin {
    Center,
    XY(LengthUnit, LengthUnit),
}

impl TransformOrigin {
    pub fn to_value(&self, width: f32, height: f32) -> Point2 {
        match self {
            TransformOrigin::Center => Point2::new(0.5 * width, 0.5 * height),
            TransformOrigin::XY(x, y) => Point2::new(
                match x {
                    LengthUnit::Pixel(v) => v.clone(),
                    LengthUnit::Percent(v) => v * width,
                },
                match y {
                    LengthUnit::Pixel(v) => v.clone(),
                    LengthUnit::Percent(v) => v * height,
                },
            ),
        }
    }
}

#[derive(Debug)]
enum ShowType {
    Display = 1,    // 0表示 Flex
    Visibility = 2, // 0表示no Visible
    Enable = 12,    // 0表示no Enable
}

#[derive(Debug, Clone, EnumDefault, Copy, Serialize, Deserialize)]
pub enum Enable {
    Auto = 0,
    None = 1,
    Visible = 2,
}

// pub fn tanslate_to_matrix(x: f32, y: f32, z: f32) -> Matrix4{
// 	Matrix4::new(
// 		1.0, 0.0, 0.0, 0.0,
// 		0.0, 1.0, 0.0, 0.0,
// 		0.0, 0.0, 1.0, 0.0,
// 		x, y, z, 1.0,
// 	)
// }

// pub fn sacle_to_matrix(x: f32, y: f32, z: f32) -> Matrix4{
// 	Matrix4::new(
// 		x, 0.0, 0.0, 0.0,
// 		0.0, y, 0.0, 0.0,
// 		0.0, 0.0, z, 0.0,
// 		0.0, 0.0, 0.0, 1.0,
// 	)
// }

// pub fn angle_z_to_matrix(theta: f32) -> Matrix4 {
// 	let r = theta/180.0*(std::f32::consts::PI);
// 	let (s, c) = r.sin_cos();
// 	// http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
// 	// let (s, c) = Rad::sin_cos(theta.into());

// 	Matrix4::new(
// 		c, s, 0.0, 0.0,
// 		-s, c, 0.0, 0.0,
// 		0.0, 0.0, 1.0, 0.0,
// 		0.0, 0.0, 0.0, 1.0,
// 	)
// }

//对齐元素中的文本
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    Left,    //把文本排列到左边。默认值：由浏览器决定。
    Right,   //把文本排列到右边。
    Center,  //把文本排列到中间。
    Justify, //实现两端对齐文本效果。
}

//设置元素中空白的处理方式
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum WhiteSpace {
    Normal,  //	默认。空白会被浏览器忽略(其实是所有的空白被合并成一个空格), 超出范围会换行。
    Nowrap, //	空白会被浏览器忽略(其实是所有的空白被合并成一个空格), 超出范围文本也不会换行，文本会在在同一行上继续，直到遇到 <br> 标签为止。
    PreWrap, //	保留所有空白符序列，超出范围会换行。
    Pre,    //	保留空白符，超出范围不会换行(利用yoga无法支持， 暂不支持)
    PreLine, //	合并空白符序列，如果存在换行符，优先保留换行符， 超出范围会换行。
}

impl WhiteSpace {
    pub fn allow_wrap(&self) -> bool {
        // match *self {
        //     WhiteSpace::Nowrap | WhiteSpace::Pre => false,
        //     WhiteSpace::Normal | WhiteSpace::PreWrap | WhiteSpace::PreLine => true,
		// }
		match *self {
            WhiteSpace::Nowrap => false,
            _ => true,
        }
    }

    pub fn preserve_newlines(&self) -> bool {
        match *self {
            WhiteSpace::Normal | WhiteSpace::Nowrap => false,
            WhiteSpace::Pre | WhiteSpace::PreWrap | WhiteSpace::PreLine => true,
        }
    }

    pub fn preserve_spaces(&self) -> bool {
        match *self {
            WhiteSpace::Normal | WhiteSpace::Nowrap | WhiteSpace::PreLine => true,
            WhiteSpace::Pre | WhiteSpace::PreWrap => false,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,  //	默认值。标准的字体样式。
    Ttalic,  //	斜体的字体样式。
    Oblique, //	倾斜的字体样式。
}
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for Opacity {
    fn default() -> Opacity {
        Opacity(1.0)
    }
}

impl Show {
    #[inline]
    pub fn get_display(&self) -> Display {
        unsafe { transmute((self.0 & (ShowType::Display as usize)) as u8) }
    }

    #[inline]
    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::Flex => self.0 &= !(ShowType::Display as usize),
            Display::None => self.0 |= ShowType::Display as usize,
        }
    }

    #[inline]
    pub fn get_visibility(&self) -> bool {
        (self.0 & (ShowType::Visibility as usize)) != 0
    }

    #[inline]
    pub fn set_visibility(&mut self, visibility: bool) {
        if visibility {
            self.0 |= ShowType::Visibility as usize;
        } else {
            self.0 &= !(ShowType::Visibility as usize);
        }
    }

    #[inline]
    pub fn get_enable(&self) -> Enable {
        let r = unsafe { transmute(((self.0 & (ShowType::Enable as usize)) >> 2) as u8) };
        r
    }

    #[inline]
    pub fn set_enable(&mut self, enable: Enable) {
        self.0 = self.0 & !(ShowType::Enable as usize) | ((enable as usize) << 2);
    }
}

impl Default for Show {
    fn default() -> Show {
        Show(ShowType::Visibility as usize)
    }
}
impl Default for BackgroundImageClip {
    fn default() -> BackgroundImageClip {
        BackgroundImageClip(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)))
    }
}

pub fn get_size(s: &FontSize) -> usize {
    match s {
        &FontSize::None => {
			// size
			panic!()
		},
        &FontSize::Length(r) => r,
        &FontSize::Percent(_r) => {
			// (r * size as f32).round() as usize;
			panic!()
		},
    }
}

