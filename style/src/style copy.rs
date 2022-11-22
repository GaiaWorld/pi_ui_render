/// 用户操作的组件定义
/// 遵循css对应属性的语意
use std::default::Default;
use std::{
    hash::{Hash, Hasher},
    mem::transmute,
};

use ordered_float::NotNan;
use pi_curves::easing::EEasingMode;
use pi_curves::steps::EStepMode;
use pi_flex_layout::style::{
    AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType,
};
use smallvec::SmallVec;

use pi_atom::Atom;
use pi_flex_layout::prelude::{INode, Number, Rect, Size as FlexSize};

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct CgColor(nalgebra::Vector4<f32>);
pub type Aabb2 = ncollide2d::bounding_volume::AABB<f32>;
pub type NotNanRect = Rect<NotNan<f32>>;

impl Hash for CgColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            NotNan::new_unchecked(self.0.x).hash(state);
            NotNan::new_unchecked(self.0.y).hash(state);
            NotNan::new_unchecked(self.0.z).hash(state);
            NotNan::new_unchecked(self.0.w).hash(state);
        }
    }
}

impl CgColor {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self(nalgebra::Vector4::new(x, y, z, w)) }
}

impl Default for CgColor {
    fn default() -> Self { Self(nalgebra::Vector4::new(1.0, 1.0, 1.0, 1.0)) }
}

#[derive(Default)]
pub struct Node;


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: SmallVec<[Atom; 1]>,                               // 指定要绑定到选择器的关键帧的名称
    pub duration: SmallVec<[Time; 1]>,                           // 动画指定需要多少毫秒完成
    pub timing_function: SmallVec<[AnimationTimingFunction; 1]>, // 设置动画将如何完成一个周期(插值函数)
    pub iteration_count: SmallVec<[IterationCount; 1]>,
    pub delay: SmallVec<[Time; 1]>,                    // 设置动画在启动前的延迟间隔。
    pub direction: SmallVec<[AnimationDirection; 1]>,  // 指定是否应该轮流反向播放动画。
    pub fill_mode: SmallVec<[AnimationFillMode; 1]>,   // 规定当动画不播放时（当动画完成时，或当动画有一个延迟未开始播放时），要应用到元素的样式。
    pub play_state: SmallVec<[AnimationPlayState; 1]>, // 指定动画是否正在运行或已暂停
}

impl Animation {
    pub fn get_attr<T: Default + Clone>(i: usize, vec: &SmallVec<[T; 1]>) -> T {
        if vec.len() > 0 {
            let i = i % vec.len();
            vec[i].clone()
        } else {
            T::default()
        }
    }
}

/// 动画循环次数
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Deref, DerefMut)]
pub struct IterationCount(pub f32);

// 动画默认播放一次
impl Default for IterationCount {
    fn default() -> Self { Self(1.0) }
}

/// 时间 ，单位 ms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Deref, DerefMut, Default)]
pub struct Time(pub usize);

/// 动画循环方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumDefault)]
pub enum AnimationDirection {
    /// 每个循环内动画向前循环，换言之，每个动画循环结束，动画重置到起点重新开始，这是默认属性。
    Normal,
    /// 动画交替反向运行，反向运行时，动画按步后退，同时，带时间功能的函数也反向，比如，ease-in 在反向时成为 ease-out。计数取决于开始时是奇数迭代还是偶数迭代
    Reverse,
    /// 反向运行动画，每周期结束动画由尾到头运行。
    Alternate,
    /// 反向交替，反向开始交替。动画第一次运行时是反向的，然后下一次是正向，后面依次循环。决定奇数次或偶数次的计数从 1 开始。
    AlternateReverse,
}

/// 动画播放状态
#[derive(Debug, Clone, Serialize, Deserialize, EnumDefault)]
pub enum AnimationPlayState {
    /// 正在播放
    Running,
    /// 暂停
    Paused,
}

/// 设置 CSS 动画在执行之前和之后如何将样式应用于其目标
#[derive(Debug, Clone, Serialize, Deserialize, EnumDefault)]
pub enum AnimationFillMode {
    /// 当动画未执行时，动画将不会将任何样式应用于目标，而是已经赋予给该元素的 CSS 规则来显示该元素。这是默认值
    None,
    /// 目标将保留由执行期间遇到的最后一个关键帧 (en-US)计算值。最后一个关键帧取决于animation-direction和animation-iteration-count的值
    Forwards,
    /// 动画将在应用于目标时立即应用第一个关键帧中定义的值，并在animation-delay期间保留此值。第一个关键帧取决于animation-direction的值
    Backwards,
    /// 动画将遵循forwards和backwards的规则，从而在两个方向上扩展动画属性
    Both,
}

// 淡入淡出方式
#[derive(EnumDefault, Debug, Clone, Serialize, Deserialize)]
pub enum EaseFunction {
    Back,
    Circle,
    Cubic,
    Sine,
    Quad,
    Quart,
    Quint,
    Expo,
    Elastic,
    Bounce,
}

/// 插值函数
#[derive(EnumDefault, Debug, Clone, Serialize, Deserialize)]
pub enum AnimationTimingFunction {
    /// 匀速
    Linear,
	/// 淡入淡出
    Ease(EEasingMode),
    /// 跳跃
    Step(usize, EStepMode),
    /// 贝塞尔曲线
    CubicBezier(f32, f32, f32, f32),
}


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
pub struct MinMax {
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
        Position(Rect {
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

impl Transform {
    pub fn add_func(&mut self, f: TransformFunc) { self.funcs.push(f); }
    pub fn set_origin(&mut self, o: TransformOrigin) { self.origin = o; }
}

pub type TransformFuncs = Vec<TransformFunc>;
// 背景色和class
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref)]
pub struct BackgroundColor(pub Color);

// class名称， 支持多个class， 当只有一个或两个class时， 有优化
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct ClassName(pub SmallVec<[usize; 1]>);

// 边框颜色
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct BorderColor(pub CgColor);

// 图片路劲及纹理
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default, Hash)]
pub struct BackgroundImage(pub Atom);

// 遮罩图片是图片路径或线性渐变色
#[derive(Clone, Debug, Serialize, Deserialize, EnumDefault)]
pub enum MaskImage {
    Path(Atom),
    LinearGradient(LinearGradientColor),
}


#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct MaskImageClip(pub Aabb2);

impl Default for MaskImageClip {
    fn default() -> Self { MaskImageClip(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))) }
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundImageMod {
    pub object_fit: FitType,
    pub repeat: ImageRepeat,
}

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
        Self(NotNanRect {
            left: unsafe { NotNan::new_unchecked(0.0) },
            top: unsafe { NotNan::new_unchecked(0.0) },
            right: unsafe { NotNan::new_unchecked(1.0) },
            bottom: unsafe { NotNan::new_unchecked(1.0) },
        })
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, Deref, DerefMut)]
pub struct BorderImageRepeat(pub ImageRepeat);

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct ImageRepeat {
    pub x: ImageRepeatOption,
    pub y: ImageRepeatOption,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub color: Color, //颜色
    pub text_indent: f32,
    pub text_stroke: Stroke,
    pub text_align: TextAlign,
    pub text_shadow: TextShadows, // 缩进， 单位： 像素
    pub letter_spacing: f32,      //字符间距， 单位：像素
    pub word_spacing: f32,        //字符间距， 单位：像素
    pub white_space: WhiteSpace,  //空白处理
    pub line_height: LineHeight,  //设置行高
    pub vertical_align: VerticalAlign,

    pub font_style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    pub font_weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    pub font_size: FontSize,   //
    pub font_family: Atom,     //	规定字体系列。参阅：font-family 中可能的值。
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Default::default(),
            text_indent: Default::default(),
            text_stroke: Default::default(),
            text_align: Default::default(),
            text_shadow: Default::default(),
            letter_spacing: Default::default(),
            word_spacing: Default::default(),
            white_space: Default::default(),
            line_height: Default::default(),
            vertical_align: Default::default(),
            font_style: Default::default(),
            font_weight: 500,
            font_size: Default::default(),
            font_family: Default::default(),
        }
    }
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
    pub family: usize,    //	规定字体系列。参阅：font-family 中可能的值。
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
pub fn to_linear_gradient_color(color_and_positions: &[f32], direction: f32) -> LinearGradientColor {
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
pub fn to_radial_gradient_color(color_and_positions: &[f32], center_x: f32, center_y: f32, shape: u8, size: u8) -> RadialGradientColor {
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
    pub width: NotNan<f32>, //	描边宽度
    pub color: CgColor,     //	描边颜色
}

// 图像填充的方式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FitType {
    None,
    Fill,
    Contain,
    Cover,
    ScaleDown,
    // Repeat,
    // RepeatX,
    // RepeatY,
}

impl Default for FitType {
    fn default() -> Self { FitType::Fill }
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ImageRepeatOption {
    /// 拉伸源图像的边缘区域以填充每个边界之间的间隙。
    Stretch,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以修剪瓷砖以实现适当的配合。
    Repeat,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以拉伸瓷砖以实现适当的配合。
    Round,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以缩小瓷砖以实现适当的配合。
    Space,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum FontSize {
    None,          // 默认尺寸。
    Length(usize), //把 font-size 设置为一个固定的值。
    Percent(f32),  //把 font-size 设置为基于父元素的一个百分比值。
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
    Nowrap,  //	空白会被浏览器忽略(其实是所有的空白被合并成一个空格), 超出范围文本也不会换行，文本会在在同一行上继续，直到遇到 <br> 标签为止。
    PreWrap, //	保留所有空白符序列，超出范围会换行。
    Pre,     //	保留空白符，超出范围不会换行(利用yoga无法支持， 暂不支持)
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
    fn default() -> Opacity { Opacity(1.0) }
}

impl Show {
    #[inline]
    pub fn get_display(&self) -> Display { unsafe { transmute((self.0 & (ShowType::Display as usize)) as u8) } }

    #[inline]
    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::Flex => self.0 &= !(ShowType::Display as usize),
            Display::None => self.0 |= ShowType::Display as usize,
        }
    }

    #[inline]
    pub fn get_visibility(&self) -> bool { (self.0 & (ShowType::Visibility as usize)) != 0 }

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
    pub fn set_enable(&mut self, enable: Enable) { self.0 = self.0 & !(ShowType::Enable as usize) | ((enable as usize) << 2); }
}

impl Default for Show {
    fn default() -> Show { Show(ShowType::Visibility as usize) }
}
impl Default for BackgroundImageClip {
    fn default() -> BackgroundImageClip { BackgroundImageClip(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))) }
}

pub fn get_size(s: &FontSize) -> usize {
    match s {
        &FontSize::None => {
            // size
            32 // 默认32px
        }
        &FontSize::Length(r) => r,
        &FontSize::Percent(_r) => {
            // (r * size as f32).round() as usize;
            panic!()
        }
    }
}

#[derive(Clone, Default, Deref, DerefMut, Debug, Serialize, Deserialize)]
pub struct NodeState(pub INode);

// 枚举样式的类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StyleType {
    BackgroundRepeat = 1,
    FontStyle = 2,
    FontWeight = 3,
    FontSize = 4,
    FontFamily = 5,
    LetterSpacing = 6,
    WordSpacing = 7,
    LineHeight = 8,
    TextIndent = 9,
    WhiteSpace = 10,
    TextAlign = 11,
    VerticalAlign = 12,
    Color = 13,
    TextStroke = 14,
    TextShadow = 15,

    BackgroundImage = 16,
    BackgroundImageClip = 17,
    ObjectFit = 18,

    BackgroundColor = 19,
    BoxShadow = 20,

    BorderImage = 21,
    BorderImageClip = 22,
    BorderImageSlice = 23,
    BorderImageRepeat = 24,

    BorderColor = 25,

    Hsi = 26,
    Blur = 27,
    MaskImage = 28,
    MaskImageClip = 29,

    Transform = 30,
    TransformOrigin = 31,
    TransformWillChange = 32,

    BorderRadius = 33,
    ZIndex = 34,
    Overflow = 35,
    BlendMode = 36,

    Display = 37,
    Visibility = 38,
    Enable = 39,

    Width = 40,
    Height = 41,

    MarginTop = 42,
    MarginRight = 43,
    MarginBottom = 44,
    MarginLeft = 45,

    PaddingTop = 46,
    PaddingRight = 47,
    PaddingBottom = 48,
    PaddingLeft = 49,

    BorderTop = 50,
    BorderRight = 51,
    BorderBottom = 52,
    BorderLeft = 53,

    PositionTop = 54,
    PositionRight = 55,
    PositionBottom = 56,
    PositionLeft = 57,

    MinWidth = 58,
    MinHeight = 59,
    MaxHeight = 60,
    MaxWidth = 61,

    Direction = 62,
    FlexDirection = 63,
    FlexWrap = 64,
    JustifyContent = 65,
    AlignContent = 66,
    AlignItems = 67,

    PositionType = 68,
    AlignSelf = 69,
    FlexShrink = 70,
    FlexGrow = 71,
    AspectRatio = 72,
    Order = 73,
    FlexBasis = 74,

    Opacity = 75,
    TextContent = 76,
    NodeState = 77,

    TransformFunc = 78,

    AnimationName = 79,
    AnimationDuration = 80,
    AnimationTimingFunction = 81,
    AnimationDelay = 82,
    AnimationIterationCount = 83,
    AnimationDirection = 84,
    AnimationFillMode = 85,
    AnimationPlayState = 86,
}
