//！ 定义用户设置的组件

use std::mem::{transmute, forget};
use std::ptr::read_unaligned;
use std::{collections::VecDeque, fmt::Debug};

// use pi_world::event::Event;
use pi_world::prelude::{Entity, Component, Mut};
use bitvec::prelude::BitArray;
use pi_ui_render_macros::enum_type;
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_flex_layout::prelude::INode;
pub use pi_flex_layout::prelude::{Dimension, Number, Rect, Size as FlexSize};
use pi_flex_layout::style::{AlignContent, AlignItems, AlignSelf, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, OverflowWrap};
use pi_null::Null;
use pi_slotmap::DefaultKey;
use pi_style::style::{TextOverflow, StrokeDasharray, Shadow, OuterGlow};
pub use pi_style::style::{
    Aabb2, AnimationDirection, AnimationFillMode, AnimationName, AnimationPlayState, AnimationTimingFunction, CgColor, Color, ColorAndPosition,
    Enable, FitType, FontSize, FontStyle, ImageRepeat, IterationCount, LengthUnit, LineHeight, LinearGradientColor, NotNanRect, ShowType, Stroke,
    StyleType, TextAlign, TextShadow as TextShadow1, Time, TransformFunc, TransformFuncs, TransformOrigin, VerticalAlign, WhiteSpace,
};
use pi_style::style_parse::style_to_buffer;
use pi_style::{
    style::{
        AllTransform, AsImage as AsImage1, BaseShape, BlendMode as BlendMode1, BorderImageSlice as BorderImageSlice1, BorderRadius as BorderRadius1,
        BoxShadow as BoxShadow1, Hsi as Hsi1, MaskImage as MaskImage1, TextContent as TextContent1,
    },
    style_parse::Attribute,
    style_type::{ClassMeta, Attr, STYLE_COUNT},
};

use pi_world::world::World;
use crate::resource::animation_sheet::TransitionData;
// use pi_hal::pi_sdf::shape::PathVerb;

use super::calc::{NeedMark, EntityKey, StyleMarkType};
pub use super::root::{ClearColor, RenderDirty, RenderTargetType, Viewport};
use smallvec::SmallVec;

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

pub const SVG_COUNT: u8 = 50;

// type Rectf32 = NotNanRect;

// pub struct RadialWave {
//     /// 是否应用纵横比 - 应用则为 圆形， 否则随纵横比形变
//     pub aspect_ratio: bool,
//     /// 扭曲半径起点 - 渲染范围 [-1, 1]
//     pub start: f32,
//     /// 扭曲半径终点 - 渲染范围 [-1, 1]
//     pub end: f32,
//     /// 扭曲中心点坐标 x - 渲染范围 [-1, 1]
//     pub center_x: f32,
//     /// 扭曲中心点坐标 y - 渲染范围 [-1, 1]
//     pub center_y: f32,
//     /// 波纹周期数
//     pub cycle: u8,
//     /// 扭曲强度
//     pub weight: f32,
// }

// #[derive(Deref, Clone, Debug, Event)]
// pub struct ComponentRemove<T: Component> {
// 	#[deref]
// 	pub id: Entity,
// 	mark: PhantomData<T>,
// }


#[derive(Deref, Clone, Debug, Component)]
pub struct RadialWave(pub pi_postprocess::prelude::RadialWave);

impl Default for RadialWave {
    fn default() -> Self { 
        Self(
            pi_postprocess::prelude::RadialWave {
                aspect_ratio: false,
                start: 0.0,
                end: 0.0,
                center_x: 0.0,
                center_y: 0.0,
                cycle: 0,
                weight: 0.0,
            }
        )
    }
}

impl NeedMark for RadialWave {
    #[inline]
    fn need_mark(&self) -> bool {
		// 不在扭曲范围内， 则不需要扭曲
        if (self.start >= 1.0 || self.start <= -1.0) && (self.end >= 1.0 || self.end <= -1.0) {
			return false;
		}
		true
    }
}

#[derive(Deref, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
// #[component(storage = "SparseSet")]
pub struct ZIndex(pub isize);

/// 当post_process不为null时， 节点需要通过post_process对应的图节点进行处理，输出结果再渲染到gui上(注意，当前节点问根节点时，设置post_process，将不能把结果再渲染回gui)
#[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
pub struct AsImage {
	pub level: AsImage1, 
	pub post_process: EntityKey, // 通过post_process， 需要能查询到一个GraphId组件，此GraphId对应的图节点负责后处理效果
}

impl NeedMark for AsImage {
    #[inline]
    fn need_mark(&self) -> bool {
        if self.level != AsImage1::None || !self.post_process.is_null() {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
pub struct NodeState(pub INode);

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize)]
pub struct BoxShadow(pub BoxShadow1);

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize)]
pub struct Hsi(pub Hsi1);

impl NeedMark for Hsi {
    #[inline]
    fn need_mark(&self) -> bool {
        if self.saturate != 0.0 || self.hue_rotate != 0.0 || self.bright_ness != 0.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize)]
pub struct MaskImage(pub MaskImage1);

impl NeedMark for MaskImage {
    fn need_mark(&self) -> bool { true }
}

impl From<Atom> for MaskImage {
    fn from(value: Atom) -> Self { Self(MaskImage1::Path(value)) }
}

// 仅支持Atom的比较， 如果是渐变颜色，一律不相等
impl PartialEq for MaskImage {
    fn eq(&self, other: &Self) -> bool {
        if let MaskImage1::Path(r1) = &self.0 {
            if let MaskImage1::Path(r2) = &other.0 {
                if r1 == r2 {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize)]
pub struct ClipPath(pub BaseShape);
impl NeedMark for ClipPath {
    fn need_mark(&self) -> bool { true }
}

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize)]
pub struct BlendMode(pub BlendMode1);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: AnimationName,                                     // 指定要绑定到选择器的关键帧的名称
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub property: SmallVec<[usize; 1]>, // 指定过度影响的属性
	pub duration: SmallVec<[Time; 1]>,                           // 指定需要多少毫秒完成过度
	pub delay: SmallVec<[Time; 1]>,                    // 启动过度前的延迟间隔。
    pub timing_function: SmallVec<[AnimationTimingFunction; 1]>, // 插值函数

	// 计算数据
	pub mark: StyleMarkType,
	pub data: SmallVec<[TransitionData; 1]>,
	pub is_all: usize,
}

impl Transition {
    pub fn get_attr<T: Default + Clone>(i: usize, vec: &SmallVec<[T; 1]>) -> T {
        Animation::get_attr(i, vec)
    }
}


//ObjectFit
#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct BackgroundImageMod {
    pub object_fit: FitType,
    pub repeat: ImageRepeat,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref)]
pub struct Blur(pub f32);

impl NeedMark for Blur {
    #[inline]
    fn need_mark(&self) -> bool {
        if self.0 > 0.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref)]
pub struct BorderRadius(pub BorderRadius1);

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref, Hash)]
pub struct BorderImageSlice(pi_style::style::BorderImageSlice);

/// 布局大小
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug, Component)]
pub struct Size(pub FlexSize<Dimension>);

//超出部分的裁剪方式
#[derive(Deref, Clone, Default, Serialize, Deserialize, Debug, Component)]
pub struct Overflow(pub bool);

impl NeedMark for Overflow {
    #[inline]
    fn need_mark(&self) -> bool {
        if self.0 == true {
            true
        } else {
            false
        }
    }
}

//不透明度
#[derive(Deref, Clone, Debug, Serialize, Deserialize, Component)]
pub struct Opacity(pub f32);

impl NeedMark for Opacity {
    #[inline]
    fn need_mark(&self) -> bool {
        if self.0 < 1.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Deref, Clone, Debug, Serialize, Deserialize, Default, Component)]
pub struct TextContent(pub TextContent1);


#[derive(Default, Component, Clone)]
pub struct SvgInnerContent {
    pub shape: Shape,
    pub style: SvgStyle,
    pub hash: u64,
}

#[derive(Default, Component, Clone)]
pub struct SvgContent {
    pub width: f32,
    pub height: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(Debug, Default, Component, Clone)]
pub struct SvgGradient {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub id: Vec<Entity>
}

#[derive(Debug, Default, Component, Clone)]
pub struct SvgStop {
    pub offset: f32,
    pub color: CgColor,
}

#[derive(Debug, Default, Clone)]
pub struct SvgFilterBlurLevel {
    pub level: f32,
}

#[derive(Debug, Default, Clone)]
pub struct SvgFilterOffset {
    pub offset_x: f32,
    pub offset_y: f32,
    pub color: f32,
}

#[derive(Debug, Default, Clone)]
pub struct SvgFilter(pub Vec<Entity>);


// impl Default for SvgColor{
//     fn default() -> Self {
//         todo!()
//     }
// }





// 将display、visibility、enable合并为show组件
#[derive(Deref, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show(pub usize);

// 变换
#[derive(Debug, Clone, Default, Serialize, Deserialize, Component)]
// #[component(storage = "SparseSet")]
pub struct Transform {
    pub all_transform: AllTransform,
    pub origin: TransformOrigin,
}

impl Transform {
    pub fn add_func(&mut self, f: TransformFunc) { self.all_transform.transform.push(f); }
    pub fn set_origin(&mut self, o: TransformOrigin) { self.origin = o; }
}

// 背景色和class
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref)]
pub struct BackgroundColor(pub Color);

// class名称， 支持多个class， 当只有一个或两个class时， 有优化
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, Component)]
pub struct ClassName(pub SmallVec<[usize; 1]>);

// 边框颜色
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref)]
pub struct BorderColor(pub CgColor);

// 图片路劲及纹理
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq)]
pub struct BackgroundImage(pub Atom);

impl From<Atom> for BackgroundImage {
    fn from(value: Atom) -> Self { BackgroundImage(value) }
}

impl BackgroundImage {
    pub fn set_url() {}
}

#[derive(Debug, Deref, Clone, Serialize, Deserialize)]
pub struct MaskImageClip(pub NotNanRect);

impl Default for MaskImageClip {
    fn default() -> Self {
        unsafe {
            MaskImageClip(NotNanRect(Rect {
                left: NotNan::new_unchecked(0.0),
                right: NotNan::new_unchecked(1.0),
                top: NotNan::new_unchecked(0.0),
                bottom: NotNan::new_unchecked(1.0),
            }))
        }
    }
}

// image图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, Clone, Serialize, Deserialize)]
pub struct BackgroundImageClip(pub NotNanRect);

impl Default for BackgroundImageClip {
    fn default() -> Self {
        unsafe {
            BackgroundImageClip(NotNanRect(Rect {
                left: NotNan::new_unchecked(0.0),
                right: NotNan::new_unchecked(1.0),
                top: NotNan::new_unchecked(0.0),
                bottom: NotNan::new_unchecked(1.0),
            }))
        }
    }
}

// 边框图片
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq)]
pub struct BorderImage(pub Atom);

impl From<Atom> for BorderImage {
    fn from(value: Atom) -> Self { Self(value) }
}

// borderImage图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Hash)]
pub struct BorderImageClip(pub NotNanRect);

impl Default for BorderImageClip {
    fn default() -> Self {
        unsafe {
            BorderImageClip(NotNanRect(Rect {
                left: NotNan::new_unchecked(0.0),
                right: NotNan::new_unchecked(1.0),
                top: NotNan::new_unchecked(0.0),
                bottom: NotNan::new_unchecked(1.0),
            }))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, Deref)]
pub struct BorderImageRepeat(pub ImageRepeat);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
	pub font_style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    pub font_weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    pub font_size: FontSize,   //
    pub font_family: Atom,     //	规定字体系列。参阅：font-family 中可能的值。

    pub line_height: LineHeight,  //设置行高
	pub letter_spacing: f32,      //字符间距， 单位：像素
    pub word_spacing: f32,        //字符间距， 单位：像素
    pub white_space: WhiteSpace,  //空白处理

    pub text_indent: f32,
	pub text_stroke: Stroke,
	pub vertical_align: VerticalAlign,
	pub text_align: TextAlign,

	pub color: Color, //颜色

    // pub color: Color, //颜色
    // pub text_indent: f32,
    // pub text_stroke: Stroke,
    // pub text_align: TextAlign,
    // pub letter_spacing: f32,     //字符间距， 单位：像素
    // pub word_spacing: f32,       //字符间距， 单位：像素
    // pub white_space: WhiteSpace, //空白处理
    // pub line_height: LineHeight, //设置行高
    // pub vertical_align: VerticalAlign,

    // pub font_style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    // pub font_weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    // pub font_size: FontSize,   //
    // pub font_family: Atom,     //	规定字体系列。参阅：font-family 中可能的值。
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgStyle {
    pub stroke: Stroke,
    pub fill_color: Color, //颜色
    pub stroke_dasharray: StrokeDasharray,
    pub shadow: Shadow,
    pub outer_glow_color_and_dist: [f32; 4],
    pub filter :Entity
}

impl Default for SvgStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke{ width: unsafe { NotNan::new_unchecked(0.0) }, color: CgColor::new(0.0, 0.0, 0.0, 0.0) },
            fill_color: Color::RGBA(CgColor::new(0.0, 0.0, 0.0, 0.0)),
            stroke_dasharray: Default::default(),
            shadow: Shadow::default(),
            outer_glow_color_and_dist: [0.0,0.0,0.0,f32::INFINITY],
            filter: Entity::null()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextOverflowData {
	pub text_overflow: TextOverflow,
	pub text_overflow_char: SmallVec<[TextOverflowChar;1]>, // 通常是...
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOverflowChar {
	pub width: f32,
	pub ch: char,
	pub ch_id: DefaultKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Deref)]
pub struct TextShadow(pub TextShadowList);

#[derive(Debug, Clone, Serialize, Deserialize, Default, Deref)]
pub struct TextOuterGlow(pub OuterGlow);

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Default::default(),
            text_indent: Default::default(),
            text_stroke: Default::default(),
            text_align: Default::default(),
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

impl TextStyle {
    pub fn font_weight(&self) -> f32 {
        if self.font_weight == 500 {
            0.0
        } else if self.font_weight < 500 {
            -1.0
        } else {
            1.0
        }
    }
}


pub type TextShadowList = SmallVec<[TextShadow1; 1]>;

// TransformWillChange， 用于优化频繁变化的Transform
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
// #[component(storage = "SparseSet")]
pub struct TransformWillChange(pub Option<Transform>); //

impl NeedMark for TransformWillChange {
    #[inline]
    fn need_mark(&self) -> bool { self.0.is_some() }
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
            // Display::Grid => self.0 &= !(ShowType::Display as usize),
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

/// 布局外边距
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug)]
pub struct Margin(pub Rect<Dimension>);

/// 布局内边距
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug)]
pub struct Padding(pub Rect<Dimension>);

/// 布局边框尺寸
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug)]
pub struct Border(pub Rect<Dimension>);

#[derive(Deref, Clone, Serialize, Deserialize, Debug)]
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
	pub overflow_wrap: OverflowWrap,
    pub row_gap: f32,
    pub column_gap: f32,
    pub auto_reduce: bool,
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
			overflow_wrap: Default::default(),
            row_gap: 0.0,
            column_gap: 0.0,
            auto_reduce: false,
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

/// 绘制canvas的图节点
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Canvas {
	pub id: Entity,
    pub pre_graph_id: pi_render::depend_graph::NodeId,
	pub by_draw_list: bool,
}

/// 显示改变（一般是指canvas，gui不能感知除了style属性以外的属性改变，如果canvas内容发生改变，应该通过style设置，以便gui能感知，从而设置脏区域）
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ShowChange;

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

pub mod serialize {
    use std::mem::forget;

    use crate::components::{
        SettingComponentIds,
        calc::{BackgroundImageTexture, BorderImageTexture}, user::*
    };
    use pi_atom::Atom;
    // use pi_print_any::{out_any, println_any};
    // use pi_ecs::{
    //     prelude::{Query, SingleResMut},
    //     query::{DefaultComponent, Write},
    // };
    use pi_world::{
        // component::ColumnIndex,
        prelude::Entity, world::{ComponentIndex, FromWorld}
    };
    use pi_flex_layout::{
        prelude::Number,
        style::{
            AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent,
            PositionType as PositionType1,
        },
    };
    use pi_style::{
        style::{NotNanRect, StyleType},
        style_parse::Attribute,
        style_type::*,
    };
    use smallvec::SmallVec;


    /// 定义trait ConvertToComponent， 可将buffer转化到ecs组件上
    pub trait AttrSet: Attr {
        /// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized;

        // push取组件操作
        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>);
    }

    // svg属性
    pub trait SvgAttr {
        /// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(ptr: *const u8, query: &mut Setting, entity: Entity)
        where
            Self: Sized;
        
        fn get_index() -> u8 where Self: Sized;
    }

    pub trait SetDefault {
        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle)
        where
            Self: Sized;
 
    }

	pub trait ConvertToComponent<T: 'static>: AttrSet {
		/// 获取属性
		fn get(
            world: &World,
			query: &SettingComponentIds,
			entity: Entity,
		) -> Option<T>;

        fn to_attr(ptr: *const u8) -> T
        where
            Self: Sized; 
    }

    // // 初始化节点
    // pub struct InitNodeType;

    // impl Attr for InitNodeType {
    //      /// 获取样式属性类型
    //     fn get_type() -> StyleType
    //     where
    //         Self: Sized {
    //             StyleType::Width
    //         }
    //     /// 获取样式属性索引（对应StyleAttrs的索引）
    //     fn get_style_index() -> u8
    //     where
    //         Self: Sized {
    //         STYLE_COUNT * 2
    //     }
    //     /// 样式属性的牛内存大小
    //     fn size() -> usize
    //     where
    //         Self: Sized;
    //     /// 序列化自身到buffer中
    //     unsafe fn write(&self, buffer: &mut Vec<u8>);
    // }

    /// 从Buffer中读取StyleType
    pub struct StyleTypeReader<'a> {
        buffer: &'a Vec<u8>,
        cursor: usize,
        end: usize,
    }

    pub enum StyleSet {
        Set,
        Cancel,
    }

    pub fn set_style_attr<V: Debug, C: Clone + 'static, F: FnMut(Mut<C>, V)>(
        world: &mut World,
        component_id: ComponentIndex,
        entity: Entity,
        // component_id: ColumnIndex,
        // default_component_id: ColumnIndex,
		// dirty_list: ColumnIndex,
        v: V,
        mut f: F,
    ) {
		// log::debug!("type: {:?}, entity: {:?}", std::any::type_name::<C>(), entity);
        
        log::debug!(
            "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
            std::any::type_name::<C>(),
            v,
            entity
        );

        // if std::any::type_name::<C>().contains("TextContent") {
        //     log::warn!(
        //         "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
        //         std::any::type_name::<C>(),
        //         v,
        //         entity
        //     );
        // }
        

       
        // pi_print_any::out_any!(println,
        //     "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
        //     std::any::type_name::<C>(),
        //     &v,
        //     &entity
        // );
        // pi_print_any::out_any!(println, "set_default_style_attr==={:?}", (entity, std::any::type_name::<C>(), &v));
        match world.get_component_mut_by_index::<C>(entity, component_id) {
            Ok(component) => {
                f(component, v);
            }
            _ => {
                log::error!("set style fail, component is not exist: {:?}", (entity, v, component_id, std::any::type_name::<C>()));
                // let default_value = world.get_resource_by_id(default_component_id).unwrap();
                // let mut r = unsafe { default_value.deref::<DefaultComponent<C>>() }.0.clone();
                // let default_value = if let Some(default_value) = world.get_single_res::<C>() {
                //     default_value.clone()  
                // } else {
                //     // C::default()
                //     panic!()
                // };

                // let mut default_value = default_value.clone();
                // f(&mut default_value, v);
                
                // let _ = alter.alter(entity, (default_value, ));
                // f(&mut r, v);
                // world.entity_mut(entity).insert(r);
            }
        };
		// unsafe { world.get_resource_mut_by_id(dirty_list).unwrap().into_inner().deref_mut::<DirtyList>() }.push(entity);
		// unsafe { dirty_list.into_inner().deref_mut::<DirtyList>() }
		// dirty_list
    }

	// 在设置Class、styleMark时调用， 不需要进脏列表
    pub fn set_style_attr_or_default<V, C: Clone + Default + 'static, F: FnMut(&mut C, V)>(
        world: &mut World,
        component_id: ComponentIndex,
        entity: Entity,
        
        v: V,
        mut f: F,
    ) {
        match world.get_component_mut_by_index::<C>(entity, component_id) {
            Ok(mut component) => {
                // SAFETY: `test_component` has unique access of the `EntityMut` and is not used afterwards
                f(&mut component, v);
            }
            _ => {
                // let mut default_value = default_value.clone();
                // f(&mut default_value, v);
                // let _ = alter.alter(entity, (default_value, ));
            }
        };
    }

    // pub unsafe fn get_component_mut_by_index<C: Clone + Default>(world: &mut World, entity: Entity, component_id: ColumnIndex) -> &mut C {
    //     match world.get_mut_by_id(entity, component_id) {
    //         Some(component) => unsafe { component.into_inner().deref_mut::<C>() },
    //         None => panic!("get_component fail, get_component is not exist: {:?}, entity: {:?}", component_id, entity),
    //     }
    // }


    // fn set_default_style_attr<V, C: Clone, F: FnMut(&mut C, V)>(default_value: , v: V, mut f: F) {
	// 	pi_print_any::out_any!(log::trace, "set_default_style_attr==={:?}", &v);
    //     match world.get_single_res_mut::<C>() {
    //         Some(mut component) => {
    //             component.set_changed();
    //             // SAFETY: `test_component` has unique access of the `EntityMut` and is not used afterwards
    //             // f(&mut *unsafe { component.into_inner().deref_mut::<DefaultComponent<C>>() }, v);
    //             f(component, v);
    //         }
    //         None => {
    //             log::error!(
    //                 "set_default_style_attr fail, default value is not exist, {:?}",
    //                 std::any::type_name::<C>()
    //             );
    //         }
    //     };
    // }

    #[inline]
    fn reset_style_attr<C: Clone + 'static, F: FnMut(&mut C, &C)>(
        world: &mut World,
        component_id: ComponentIndex,
        entity: Entity,
        default_id: u32,
        // component_id: ColumnIndex,
        // default_component_id: ColumnIndex,
		// dirty_list: ColumnIndex,
        mut f: F,
    ) {
        let default_value = match world.index_single_res::<C>(default_id as usize) {
            Some(r) => unsafe {transmute(&*r)},
            None => return,
        };
        if let Ok(mut component) = world.get_component_mut_by_index::<C>(entity, component_id) {
            f(&mut component, default_value);
        };
    }

    impl<'a> StyleTypeReader<'a> {
        pub fn default(buffer: &Vec<u8>) -> StyleTypeReader {
            StyleTypeReader {
                buffer,
                cursor: 0,
                end: buffer.len(),
            }
        }

        pub fn new(buffer: &Vec<u8>, start: usize, end: usize) -> StyleTypeReader { StyleTypeReader { buffer, cursor: start, end } }

        // 将当前style写入组件
        // 小心使用该方法， 保证self.buffer中的内存只被使用一次
        pub fn write_to_component(&mut self, cur_style_mark: &mut StyleMarkType, entity: Entity, query: &mut Setting, is_clone: bool) -> bool {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
				// pi_print_any::out_any!(println, "write_to_component==={:?}, cursor:{:?}, next_type: {:?}", style_type, self.cursor, next_type);
                StyleAttr::set(cur_style_mark, style_type, &self.buffer, self.cursor, query, entity, is_clone);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return true;
                // return Some(StyleAttr::get_type(style_type));
            }
            false
        }

        // 将当前style写入默认组件
        pub fn write_to_default(&mut self, world: &mut World, query: &DefaultStyle) -> Option<StyleType> {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
                StyleAttr::set_default(style_type, &self.buffer, self.cursor, world, query);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return Some(StyleAttr::get_type(style_type));
            }

            None
        }

        // 将当前style写入组件
        pub fn to_attr(&mut self) -> Option<StyleAttribute> {
			// let c = self.cursor;
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
                let r = if style_type < STYLE_COUNT {
                    let r = StyleAttr::to_attr(style_type, &self.buffer, self.cursor);
                    StyleAttribute::Set(r)
                } else {
					
                    // reset
                    StyleAttribute::Reset(style_type)
                };

                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return Some(r);
                // return Some(StyleAttr::get_type(style_type));
            }
            None
        }

        // f函数返回true，则写入到组件，否则不写入,跳过该属性
        // 同时，使用该函数， 属性将被clone后，放入world中 （设置class时使用， 因为class的buffer会被共享， 如果属性中存在堆属性， 堆被共享为多个所有权， 将会出现未知错误）
        pub fn or_write_to_component<F: Fn(StyleType) -> bool>(
            &mut self,
            cur_style_mark: &mut StyleMarkType,
            entity: Entity,
            query: &mut Setting,
            f: F,
        ) -> Option<StyleType> {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
                let ty = StyleAttr::get_type(style_type);
                if f(ty) {
                    StyleAttr::set(cur_style_mark, style_type, &self.buffer, self.cursor, query, entity, true);
                }
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return Some(ty);
            }
            None
        }

        pub fn push_component_ops(
            &mut self,
            query: &SettingComponentIds,
            arr: &mut Vec<(ComponentIndex, bool)>,
        ) -> bool {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
				// pi_print_any::out_any!(println, "write_to_component==={:?}, cursor:{:?}, next_type: {:?}", style_type, self.cursor, next_type);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                StyleAttr::push_component_ops(style_type, query, arr);
                return true;
            }
            false
        }

        // 读下一个样式类型
        fn next_type(&mut self) -> Option<u8> {
            if self.cursor >= self.end {
                return None;
            }

            // let ty_size = std::mem::size_of::<u8>();
            let ty = unsafe { Some(self.buffer.as_ptr().add(self.cursor).cast::<u8>().read_unaligned()) };

            // self.cursor += ty_size;
            self.cursor += 1;
            ty
        }
    }


    macro_rules! set {
        (@expr $component_name: ident, $component_ty: ty, $value_name: ident, $value_ty: ty, $set: expr $(,$component_name1: ident)*) => {
            fn set(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                set_style_attr(
                    &mut query.world,
                    query.style.$component_name,
                    entity,
					// query.style.dirty_list,
                    v,
                    |mut $component_name: Mut<$component_ty>, $value_name: $value_ty| $set,
                );
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$component_name, true));
                $(arr.push((ids.$component_name1, true));)*
            }
        };

        (@expr2 $component_name: ident, $component_ty: ty, $component_name2: ident, $component_ty2: ty, $value_name: ident, $value_ty: ty, $set: expr $(,$component_name1: ident)*) => {
            fn set(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let $value_name = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };

                pi_print_any::out_any!(log::debug,
                    "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                    std::any::type_name::<$component_ty>(),
                    &$value_name,
                    &entity
                );
                // pi_print_any::out_any!(println,
                //     "set_style_attr, type: {:?}, value_type: {:?}, value: {:?}, entity: {:?}",
                //     std::any::type_name::<$component_ty>(),
                //     std::any::type_name::<$value_ty>(),
                //     &$value_name,
                //     &entity
                // );

                if let Ok(mut component) = query.world.get_component_mut_by_index::<$component_ty2>(entity, query.style.$component_name2) {
                    if component.0.is_some() {
                        // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                        if let Some($component_name) = &mut component.0 {
                            // log::debug!("set t========{:?}, {:?}, {:?}", entity, query.style.$component_name2, &v);
                            $set;
                            return
                        }
                    }
                };
    
                // 不存在component_ty2， 则设置在component_ty上
                match query.world.get_component_mut_by_index::<$component_ty>(entity, query.style.$component_name){
                    Ok(mut $component_name)  => {
                        $set;
                    }
                    _ => ()
                }
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$component_name, true));
                $(arr.push((ids.$component_name1, true));)*
            }
        };
    }

	macro_rules! get {
        (@expr $attr: ident, $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $component_ty: ty, $get: expr) => {
            fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<$attr> {
				match world.get_component_by_index::<$component_ty>(entity, query.$component_name) {
					Ok($component_name) => Some($attr::$attr_name($attr_ty_name($get))),
					_ => None
				}
            }
        };

		(@empty $attr: ident) => {
			fn get(_world: &World, _query: &SettingComponentIds, _entity: Entity) -> Option<$attr> {
				None
			}
		};
    }

    // 设置默认值
    macro_rules! set_default {
        (@expr $attr_ty_name: ident, $component_name: ident, $component_ty: ty, $value_name: ident, $value_ty: ty, $set: expr) => {
            impl SetDefault for $attr_ty_name {
                fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                    if let Some($component_name) = world.index_single_res_mut::<$component_ty>(query.$component_name as usize) {
                        let $value_name = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
                        $set;
                    }
                }
            }
        };

        (@empty $attr_ty_name: ident) => {
            impl SetDefault for $attr_ty_name {
                fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle) {}
            }
        };
    }

    macro_rules! reset {
        (@expr $attr_ty_name:ident, $component_name: ident, $component_ty: ty, $value_name: ident, $value_ty: ty, $set: expr) => {
            $crate::paste::item! {
                impl AttrSet for [<Reset $attr_ty_name>] {
                    fn set(_ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                        reset_style_attr(
                            &mut query.world,
                            query.style.$component_name,

                            entity,
                            query.default_value.$component_name,
                            
                            |$component_name: &mut $component_ty, $value_name: &$component_ty| {
                                $set;
                            },
                        );
                    }
                    fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                        
                    }
                }
            }
        };

        (@expr2 $attr_ty_name:ident, $component_name: ident, $component_ty: ty, $component_name2: ident, $component_ty2: ty, $value_name: ident, $value_ty: ty, $set: expr) => {
            $crate::paste::item! {
                impl AttrSet for [<Reset $attr_ty_name>] {
                    fn set(_ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                        
                        if let Ok(mut component) = query.world.get_component_mut_by_index::<$component_ty2>(entity, query.style.$component_name2) {
                            if component.0.is_some() {
                                if let Some($component_name) = &mut component.0 {
                                    let $value_name = Default::default();
                                    $set;
                                }
                            }
                        };
            
                        match query.world.get_component_mut_by_index::<$component_ty>(entity, query.style.$component_name) {
                            Ok(mut $component_name)  => {
                                let $value_name = Default::default();
                                $set;
                            }
                            _ => (),
                        }
                    }
                    fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                        
                    }
                }
            }
        };

        // 空实现
        (@empty, $attr_ty_name: ident) => {
            impl AttrSet for $attr_ty_name {
                fn set(_ptr: *const u8, _query: &mut Setting, _entity: Entity, _is_clone: bool) {}
                fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {   
                }
            }
        };
        
    }

    macro_rules! impl_style {
        (@expr $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $component_ty: ty, $value_name: ident, $value_ty: ty, $set: expr, $get: expr, $reset: expr $(,$component_name1: ident)*) => {
            impl AttrSet for $attr_ty_name {
                set!(@expr $component_name, $component_ty, $value_name, $value_ty, $set $(,$component_name1)*);
            }

            set_default!(@expr $attr_ty_name, $component_name, $component_ty, $value_name, $value_ty, $set);
            impl ConvertToComponent<Attribute> for $attr_ty_name {
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$attr_name($attr_ty_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@expr Attribute, $attr_ty_name, $attr_name, $component_name, $component_ty, $get);
            }
            reset!(@expr $attr_ty_name, $component_name, $component_ty, $value_name, $value_ty, $reset);
        };

        (@expr2 $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $component_ty: ty, $component_name2: ident, $component_ty2: ty, $value_name: ident, $value_ty: ty, $set: expr, $get: expr, $reset: expr $(,$component_name1: ident)*) => {
            impl AttrSet for $attr_ty_name {
                set!(@expr2 $component_name, $component_ty, $component_name2, $component_ty2, $value_name, $value_ty, $set $(,$component_name1)*);
            }

            set_default!(@expr $attr_ty_name, $component_name, $component_ty, $value_name, $value_ty, $set);
            impl ConvertToComponent<Attribute> for $attr_ty_name {
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$attr_name($attr_ty_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@expr Attribute, $attr_ty_name, $attr_name, $component_name, $component_ty, $get);
            }

            reset!(@expr2 $attr_ty_name, $component_name, $component_ty, $component_name2, $component_ty2, $value_name, $value_ty, $reset);
        };

        (@svg $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $component_ty: ty, $value_name: ident, $value_ty: ty, $set: expr, $get: expr, $reset: expr $(,$component_name1: ident)*) => {
            impl AttrSet for $attr_ty_name {
                set!(@expr $component_name, $component_ty, $value_name, $value_ty, $set $(,$component_name1)*);
            }

            impl ConvertToComponent<SvgTypeAttr> for $attr_ty_name {
                fn to_attr(ptr: *const u8) -> SvgTypeAttr
                {
                    SvgTypeAttr::$attr_name($attr_ty_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@expr SvgTypeAttr, $attr_ty_name, $attr_name, $component_name, $component_ty, $get);
            }

            reset!(@expr $attr_ty_name, $component_name, $component_ty, $value_name, $value_ty, $reset);
        };

        ($attr_ty_name: ident) => {
            reset!(@empty, $attr_ty_name); 
            set_default!(@empty $attr_ty_name);
            impl ConvertToComponent<Attribute> for $attr_ty_name {
                fn to_attr(_ptr: *const u8) -> Attribute
                {
                    todo!()
                }
                get!(@empty Attribute);
            }
        };
        ($attr_ty_name: ident, $component_name: ident, $ty: ident) => {
            impl_style!(@expr $attr_ty_name, $ty, $component_name, $ty, v, $ty, *$component_name = v, $component_name);
        };

        (@pack $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $attr_name, $component_name, $attr_name, v, $value_ty, $component_name.0 = v, $component_name.0.clone(), $component_name.0 = v.0.clone());
        };
        (@pack_compare $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $attr_name, $component_name, $attr_name, v, $value_ty, if $component_name.0 != v {$component_name.0 = v}, $component_name.0, $component_name.0 = v.0);
        };
        (@pack_send $attr_ty_name: ident, $component_name: ident, $component_ty: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $component_ty, $component_name, $component_ty, v, $value_ty, *$component_name = $component_ty(v), $component_name.0);
        };
        ($attr_ty_name: ident, $component_name: ident, $component_ty: ident, $attr_name: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $attr_name, $component_name, $component_ty, v, $value_ty, *$component_name = $component_ty(v), $component_name.0);
        };
        ($attr_ty_name: ident, $attr_ty: ident, $component_name: ident, $component_ty: ident, $feild: ident, $value_ty: ty) => {
            impl_style!(@expr $attr_ty_name, $attr_ty, $component_name, $component_ty, v, $value_ty, $component_name.$feild = v, $component_name.$feild.clone(), $component_name.$feild = v.$feild.clone());
        };
        ($attr_ty_name: ident, $attr_ty: ident, $component_name: ident, $component_ty: ident, $feild1: ident, $feild2: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $attr_ty, $component_name, $component_ty, v, $value_ty, $component_name.$feild1.$feild2 = v, $component_name.$feild1.$feild2, $component_name.$feild1.$feild2 = v.$feild1.$feild2);
        };
        (@func $attr_ty_name: ident, $attr_name: ident, $component_name: ident, $component_ty: ident, $set_func: ident, $get_func: ident, $value_ty: ident) => {
            impl_style!(@expr $attr_ty_name, $attr_name, $component_name, $component_ty, v, $value_ty, $component_name.$set_func(v), $component_name.$get_func(), $component_name.$set_func(v.$get_func())); 
        };
    }

    impl_style!(EmptyType);

    impl_style!(FontStyleType, FontStyle, text_style, TextStyle, font_style, FontStyle);

    impl_style!(FontWeightType, FontWeight, text_style, TextStyle, font_weight, usize);
    impl_style!(FontSizeType, FontSize, text_style, TextStyle, font_size, FontSize);
    impl_style!(FontFamilyType, FontFamily, text_style, TextStyle, font_family, Atom);
    impl_style!(LetterSpacingType, LetterSpacing, text_style, TextStyle, letter_spacing, f32);
    impl_style!(WordSpacingType, WordSpacing, text_style, TextStyle, word_spacing, f32);
    impl_style!(LineHeightType, LineHeight, text_style, TextStyle, line_height, LineHeight);
    impl_style!(TextIndentType, TextIndent, text_style, TextStyle, text_indent, f32);
    impl_style!(WhiteSpaceType, WhiteSpace, text_style, TextStyle, white_space, WhiteSpace);
	impl_style!(TextOverflowType, TextOverflow, text_overflow, TextOverflowData, text_overflow, TextOverflow);
    impl_style!(TextAlignType, TextAlign, text_style, TextStyle, text_align, TextAlign);
    impl_style!(@expr TextContentType, TextContent, text_content, TextContent, v, TextContent1, text_content.0 = v, text_content.0.clone(), text_content.0 = v.0.clone(), flex_container);
    impl_style!(VerticalAlignType, VerticalAlign, text_style, TextStyle, vertical_align, VerticalAlign);
    impl_style!(ColorType, Color, text_style, TextStyle, color, Color);
    impl_style!(TextStrokeType, TextStroke, text_style, TextStyle, text_stroke, Stroke);
    impl_style!(@pack TextShadowType, TextShadow, text_shadow, TextShadowList);
    impl_style!(@pack TextOuterGlowType, TextOuterGlow, text_outer_glow, OuterGlow);
    impl_style!(@expr BackgroundImageType, BackgroundImage, background_image, BackgroundImage, v, Atom, {
        background_image.0 = v;
    }, background_image.0.clone(), background_image.0 = v.0.clone(), background_image_texture);
    impl_style!(@expr BorderRadiusType, BorderRadius, border_radius, BorderRadius, v, BorderRadius1,{
        border_radius.0 = v;
    }, border_radius.0.clone(), border_radius.0 = v.0.clone(), sdf_slice, sdf_uv);

    impl_style!(@pack BackgroundImageClipType, BackgroundImageClip, background_image_clip, NotNanRect);
    impl_style!(ObjectFitType, ObjectFit, background_image_mod, BackgroundImageMod, object_fit, FitType);
    impl_style!(
        BackgroundRepeatType,
        BackgroundRepeat,
        background_image_mod,
        BackgroundImageMod,
        repeat,
        ImageRepeat
    );
    impl_style!(@expr BorderImageType, BorderImage, border_image, BorderImage, v, Atom, border_image.0 = v, border_image.0.clone(), border_image.0 = v.0.clone(), border_image_texture);
    impl_style!(@pack BorderImageClipType, BorderImageClip, border_image_clip, NotNanRect);
    impl_style!(@pack BorderImageSliceType, BorderImageSlice, border_image_slice, BorderImageSlice1);
    impl_style!(@pack BorderImageRepeatType, BorderImageRepeat, border_image_repeat, ImageRepeat);

    impl_style!(@pack BorderColorType, BorderColor, border_color, CgColor);

    impl_style!(@pack BackgroundColorType, BackgroundColor, background_color, Color);

    impl_style!(@pack BoxShadowType, BoxShadow, box_shadow, BoxShadow1);

    impl_style!(@pack_compare OpacityType, Opacity, opacity, f32);
    impl_style!(@pack HsiType, Hsi, hsi, Hsi1);
    impl_style!(@pack_compare BlurType, Blur, blur, f32);
    impl_style!(TransformOriginType, TransformOrigin, transform, Transform, origin, TransformOrigin);
    impl_style!(DirectionType, Direction, flex_container, FlexContainer, direction, Direction);
    impl_style!(AspectRatioType, AspectRatio, flex_normal, FlexNormal, aspect_ratio, Number);
    impl_style!(OrderType, Order, flex_normal, FlexNormal, order, isize);
    impl_style!(FlexBasisType, FlexBasis, flex_normal, FlexNormal, flex_basis, Dimension);


    impl_style!(@func DisplayType, Display, show, Show, set_display, get_display, Display);
    impl_style!(@func VisibilityType, Visibility, show, Show, set_visibility, get_visibility, bool);
    impl_style!(@func EnableType, Enable, show, Show, set_enable, get_enable, Enable);

    impl_style!(@func VNodeType, VNode, node_state, NodeState, set_vnode, is_vnode, bool);
    // impl_style!(@func VNodeType, node_state, set_vnode, NodeState, bool);

    impl_style!(@pack_compare ZIndexType, ZIndex, z_index, isize);
    impl_style!(@pack_compare OverflowType, Overflow, overflow, bool);

    impl_style!(@pack MaskImageType, MaskImage, mask_image, MaskImage1);
    impl_style!(@pack MaskImageClipType, MaskImageClip, mask_image_clip, NotNanRect);
    impl_style!(@pack ClipPathType, ClipPath, clip_path, BaseShape);

	impl_style!(AsImageType, AsImage, as_image, AsImage, level, AsImage1);
	// impl_style!(AsImageType, as_image, AsImage, level, AsImage, AsImage1);

    impl_style!(WidthType, Width, size, Size, width, Dimension);
    impl_style!(HeightType, Height, size, Size, height, Dimension);

    impl_style!(MarginTopType, MarginTop, margin, Margin, top, Dimension);
    impl_style!(MarginRightType, MarginRight, margin, Margin, right, Dimension);
    impl_style!(MarginBottomType, MarginBottom, margin, Margin, bottom, Dimension);
    impl_style!(MarginLeftType, MarginLeft, margin, Margin, left, Dimension);

    impl_style!(PaddingTopType, PaddingTop, padding, Padding, top, Dimension);
    impl_style!(PaddingRightType, PaddingRight, padding, Padding, right, Dimension);
    impl_style!(PaddingBottomType, PaddingBottom, padding, Padding, bottom, Dimension);
    impl_style!(PaddingLeftType, PaddingLeft, padding, Padding, left, Dimension);

    impl_style!(BorderTopType, BorderTop, border, Border, top, Dimension);
    impl_style!(BorderRightType, BorderRight, border, Border, right, Dimension);
    impl_style!(BorderBottomType, BorderBottom, border, Border, bottom, Dimension);
    impl_style!(BorderLeftType, BorderLeft, border, Border, left, Dimension);

    impl_style!(PositionTopType, PositionTop, position, Position, top, Dimension);
    impl_style!(PositionRightType, PositionRight, position, Position, right, Dimension);
    impl_style!(PositionBottomType, PositionBottom, position, Position, bottom, Dimension);
    impl_style!(PositionLeftType, PositionLeft, position, Position, left, Dimension);

    // impl_style!(RowGapType, RowGap, flex_container, FlexContainer, row_gap, f32);
    // impl_style!(ColumnGapType, ColumnGap, flex_container, FlexContainer, column_gap, f32);
    // impl_style!(AutoReduceType, AutoReduce, flex_container, FlexContainer, auto_reduce, bool);
    impl_style!(MinWidthType, MinWidth, min_max, MinMax, min, width, Dimension);
    impl_style!(MinHeightType, MinHeight, min_max, MinMax, min, height, Dimension);
    impl_style!(MaxHeightType, MaxHeight, min_max, MinMax, max, height, Dimension);
    impl_style!(MaxWidthType, MaxWidth, min_max, MinMax, max, width, Dimension);
    impl_style!(
        JustifyContentType,
        JustifyContent,
        flex_container,
        FlexContainer,
        justify_content,
        JustifyContent
    );
    impl_style!(
        FlexDirectionType,
        FlexDirection,
        flex_container,
        FlexContainer,
        flex_direction,
        FlexDirection
    );
    impl_style!(AlignContentType, AlignContent, flex_container, FlexContainer, align_content, AlignContent);
    impl_style!(AlignItemsType, AlignItems, flex_container, FlexContainer, align_items, AlignItems);
    impl_style!(FlexWrapType, FlexWrap, flex_container, FlexContainer, flex_wrap, FlexWrap);
	impl_style!(OverflowWrapType, OverflowWrap, flex_container, FlexContainer, overflow_wrap, OverflowWrap);

    impl_style!(FlexShrinkType, FlexShrink, flex_normal, FlexNormal, flex_shrink, f32);
    impl_style!(FlexGrowType, FlexGrow, flex_normal, FlexNormal, flex_grow, f32);
    impl_style!(PositionTypeType, PositionType, flex_normal, FlexNormal, position_type, PositionType1);
    impl_style!(AlignSelfType, AlignSelf, flex_normal, FlexNormal, align_self, AlignSelf);

    impl_style!(@pack BlendModeType, BlendMode, blend_mode, BlendMode1);
    impl_style!(AnimationNameType, AnimationName, animation, Animation, name, AnimationName);
    impl_style!(
        AnimationDurationType,
        AnimationDuration,
        animation,
        Animation,
        duration,
        SmallVec<[Time; 1]>
    );
    impl_style!(
        AnimationTimingFunctionType,
        AnimationTimingFunction,
        animation,
        Animation,
        timing_function,
        SmallVec<[AnimationTimingFunction; 1]>
    );
    impl_style!(AnimationDelayType, AnimationDelay, animation, Animation, delay, SmallVec<[Time; 1]>);
    impl_style!(
        AnimationIterationCountType,
        AnimationIterationCount,
        animation,
        Animation,
        iteration_count,
        SmallVec<[IterationCount; 1]>
    );
    impl_style!(
        AnimationDirectionType,
        AnimationDirection,
        animation,
        Animation,
        direction,
        SmallVec<[AnimationDirection; 1]>
    );
    impl_style!(
        AnimationFillModeType,
        AnimationFillMode,
        animation,
        Animation,
        fill_mode,
        SmallVec<[AnimationFillMode; 1]>
    );
    impl_style!(
        AnimationPlayStateType,
        AnimationPlayState,
        animation,
        Animation,
        play_state,
        SmallVec<[AnimationPlayState; 1]>
    );

	// transition
	impl_style!(TransitionPropertyType, TransitionProperty, transition, Transition, property, SmallVec<[usize; 1]>);
    impl_style!(
        TransitionDurationType,
        TransitionDuration,
        transition,
        Transition,
        duration,
        SmallVec<[Time; 1]>
    );
    impl_style!(
        TransitionTimingFunctionType,
        TransitionTimingFunction,
        transition,
        Transition,
        timing_function,
        SmallVec<[AnimationTimingFunction; 1]>
    );
    impl_style!(TransitionDelayType, TransitionDelay, transition, Transition, delay, SmallVec<[Time; 1]>);

    impl_style!(@expr2 TransformType, Transform, transform, Transform, transform_will_change, TransformWillChange, v, TransformFuncs, 
        transform.all_transform.transform = v, 
        transform.all_transform.transform.clone(), 
        transform.all_transform.transform = v
    );
	impl_style!(@expr2 TranslateType, Translate, transform, Transform, transform_will_change, TransformWillChange, v, [LengthUnit; 2], 
        transform.all_transform.translate = Some(v), 
        match &transform.all_transform.translate {
            Some(r) => r.clone(),
            None => Default::default(),
        }, 
        transform.all_transform.translate = v
    );
    impl_style!(@expr2 ScaleType, Scale, transform, Transform, transform_will_change, TransformWillChange, v, [f32; 2], 
        transform.all_transform.scale = Some(v), 
        match &transform.all_transform.scale  {
            Some(r) => r.clone(),
            None => Default::default(),
        }, 
        transform.all_transform.scale = v
    );
    impl_style!(@expr2 RotateType, Rotate, transform, Transform, transform_will_change, TransformWillChange, v, f32, 
        transform.all_transform.rotate = Some(v), 
        match &transform.all_transform.rotate {
            Some(r) => r.clone(),
            None => Default::default(),
        }, 
        transform.all_transform.rotate = v
    );

	impl AttrSet for TransformWillChangeType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<bool>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<TransformWillChange>(),
                v,
                entity
            );

            use pi_key_alloter::Key;
            if entity.index() == 90 ||  entity.index() == 91 {
                log::debug!(
                    "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                    std::any::type_name::<TransformWillChange>(),
                    v,
                    entity
                );
            }

			if !v {
				if let Ok(mut component) = query.world.get_component_mut_by_index::<TransformWillChange>(entity, query.style.transform_will_change) {
                    if let Some(c) = &component.0 {
                        // log::debug!("set2========{:?}, {:?}", entity, c);
                        let c1 = c.clone();
                        component.0 = None;
                        match query.world.get_component_mut_by_index::<Transform>(entity, query.style.transform) {
                            Ok(mut component)  => {
                                // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                                component.all_transform = c1.all_transform;
                            }
                            _ => {
                                // let _ = query.style.transform.alter(entity, (Transform {
                                //     all_transform: c.clone(),
                                //     ..Default::default()
                                // },));
                            }
                        }
                        
                        return;
                    }
				
					
				}
			} else {
				// 不存在transform_willChange， 则设置在Transfrom上
				match query.world.get_component_mut_by_index::<Transform>(entity, query.style.transform) {
					Ok(component)  => {
                        let c = component.clone();
                        if let Ok(mut component_will_change) = query.world.get_component_mut_by_index::<TransformWillChange>(entity, query.style.transform_will_change) {
                            if component_will_change.0.is_none() { 
                                // log::debug!("set1========{:?}, {:?}", entity, c);
                                // TransformWillChange不存在才初始化
                                *component_will_change = TransformWillChange(Some(c));
                            } 
                        }
					}
					_ => {
						// let _ = query.style.transform_will_change.alter(entity, (TransformWillChange::default(), ));
					}
				}
			}
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform_will_change, true))
        }
	}
    impl SetDefault for TransformWillChangeType {
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }
    }
    impl ConvertToComponent<Attribute> for TransformWillChangeType {

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::TransformWillChange(unsafe { TransformWillChangeType(ptr.cast::<bool>().read_unaligned()) })
        }

		fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
			match world.get_component_by_index::<TransformWillChange>(entity, query.transform_will_change) {
				Ok(component)  => if component.0.is_some() {
					Some(Attribute::TransformWillChange(TransformWillChangeType(true)))
				} else {
					None
				},
				_ => None
			}
		}
    }

	impl AttrSet for ResetTransformWillChangeType {
		fn set<'w, 's>(_ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            log::debug!("reset_style_attr, type: TransformWillChange, entity: {:?}", entity);
            if let Ok(mut component) = query.world.get_component_mut_by_index::<TransformWillChange>(entity, query.style.transform_will_change) {
                // 删除TransformWillChange, 设置Transform
                if let Some(c) = &component.0 {
                    let c = c.clone();
                    component.0 = None;
                    // 设置transform
                    match query.world.get_component_mut_by_index::<Transform>(entity, query.style.transform) {
                        Ok(mut component)  => {
                            // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                            component.all_transform = c.all_transform;
                        }
                        _ => {
                            // let _ = query.style.transform.alter(entity, (Transform {
                            //     all_transform: c.clone(),
                            //     ..Default::default()
                            // }, ));
                        }
                    }
                }
				
            }
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform, true));
        }
	}


    pub struct StyleFunc {
        get_type: fn() -> u8,
		get: fn(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute>,
        // get_style_index: fn() -> u8,
        size: fn() -> usize,
        // /// 安全： entity必须存在
        // fn set(&self, cur_style_mark: &mut StyleMarkType, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity);
        /// 安全： entity必须存在
        set: fn(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),

        /// 设置默认值
        set_default: fn(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle),
        to_attr: fn(ptr: *const u8) -> Attribute,
        push_component_ops: fn (ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>),
    }

    impl StyleFunc {
        fn new<T: ConvertToComponent<Attribute> + SetDefault>() -> StyleFunc {
            StyleFunc {
                get_type: T::get_type,
                // get_style_index: T::get_style_index,
                size: T::size,
				get: T::get,
                set: T::set,
                set_default: T::set_default,
                to_attr: T::to_attr,
                push_component_ops: T::push_component_ops,
                // add: T::add,
                // scale: T::scale,
            }
        }
    }

	pub struct ResetStyleFunc {
		set: fn(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),
        push_component_ops: fn (ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>),
	}

	impl ResetStyleFunc {
        const fn new<T: AttrSet>() -> ResetStyleFunc {
            ResetStyleFunc {
                set: T::set,
                push_component_ops: T::push_component_ops,
            }
        }
    }

    pub struct SvgFunc {
        get_type: fn() -> u8,
		get: fn(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<SvgTypeAttr>,
        // get_style_index: fn() -> u8,
        size: fn() -> usize,
        // /// 安全： entity必须存在
        // fn set(&self, cur_style_mark: &mut StyleMarkType, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity);
        /// 安全： entity必须存在
        set: fn(ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),

        to_attr: fn(ptr: *const u8) -> SvgTypeAttr,
        push_component_ops: fn (ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>),
    }

    impl SvgFunc {
        const fn new<T: ConvertToComponent<SvgTypeAttr>>() -> SvgFunc {
            SvgFunc {
                get_type: T::get_type,
                // get_style_index: T::get_style_index,
                size: T::size,
				get: T::get,
                set: T::set,
                to_attr: T::to_attr,
                push_component_ops: T::push_component_ops,
                // add: T::add,
                // scale: T::scale,
            }
        }
    }

    lazy_static::lazy_static! {

        static ref STYLE_ATTR: [StyleFunc; 97] = [
            StyleFunc::new::<BackgroundRepeatType>(), // 0
            StyleFunc::new::<FontStyleType>(), // 1
            StyleFunc::new::<FontWeightType>(), // 2
            StyleFunc::new::<FontSizeType>(), // 3
            StyleFunc::new::<FontFamilyType>(), // 4
            StyleFunc::new::<LetterSpacingType>(), // 5
            StyleFunc::new::<WordSpacingType>(), // 6
            StyleFunc::new::<LineHeightType>(), // 7
            StyleFunc::new::<TextIndentType>(), // 8
            StyleFunc::new::<WhiteSpaceType>(), // 9

            StyleFunc::new::<TextAlignType>(), // 10
            StyleFunc::new::<VerticalAlignType>(), // 11
            StyleFunc::new::<ColorType>(), // 12
            StyleFunc::new::<TextStrokeType>(), // 13
            StyleFunc::new::<TextShadowType>(), // 14

            StyleFunc::new::<BackgroundImageType>(), // 15
            StyleFunc::new::<BackgroundImageClipType>(), // 16
            StyleFunc::new::<ObjectFitType>(), // 17
            StyleFunc::new::<BackgroundColorType>(), // 18
            StyleFunc::new::<BoxShadowType>(), // 19
            StyleFunc::new::<BorderImageType>(), // 20
            StyleFunc::new::<BorderImageClipType>(), // 21
            StyleFunc::new::<BorderImageSliceType>(), // 22
            StyleFunc::new::<BorderImageRepeatType>(), // 23

            StyleFunc::new::<BorderColorType>(), // 24


            StyleFunc::new::<HsiType>(), // 25
            StyleFunc::new::<BlurType>(), // 26
            StyleFunc::new::<MaskImageType>(), // 27
            StyleFunc::new::<MaskImageClipType>(), // 28
            StyleFunc::new::<TransformType>(), // 29
            StyleFunc::new::<TransformOriginType>(), // 30
            StyleFunc::new::<TransformWillChangeType>(), // 31
            StyleFunc::new::<BorderRadiusType>(), // 32
            StyleFunc::new::<ZIndexType>(), // 33
            StyleFunc::new::<OverflowType>(), // 34


            StyleFunc::new::<BlendModeType>(), // 35
            StyleFunc::new::<DisplayType>(), // 36
            StyleFunc::new::<VisibilityType>(), // 37
            StyleFunc::new::<EnableType>(), // 38


            StyleFunc::new::<WidthType>(), // 39
            StyleFunc::new::<HeightType>(), // 40

            StyleFunc::new::<MarginTopType>(), // 41
            StyleFunc::new::<MarginRightType>(), // 42
            StyleFunc::new::<MarginBottomType>(), // 43
            StyleFunc::new::<MarginLeftType>(), // 44

            StyleFunc::new::<PaddingTopType>(), // 45
            StyleFunc::new::<PaddingRightType>(), // 46
            StyleFunc::new::<PaddingBottomType>(), // 47
            StyleFunc::new::<PaddingLeftType>(), // 48

            StyleFunc::new::<BorderTopType>(), // 49
            StyleFunc::new::<BorderRightType>(), // 50
            StyleFunc::new::<BorderBottomType>(), // 51
            StyleFunc::new::<BorderLeftType>(), // 52

            StyleFunc::new::<PositionTopType>(), // 53
            StyleFunc::new::<PositionRightType>(), // 54
            StyleFunc::new::<PositionBottomType>(), // 55
            StyleFunc::new::<PositionLeftType>(), // 56

            StyleFunc::new::<MinWidthType>(), // 57
            StyleFunc::new::<MinHeightType>(), // 58
            StyleFunc::new::<MaxHeightType>(), // 59
            StyleFunc::new::<MaxWidthType>(), // 60
            StyleFunc::new::<DirectionType>(), // 61
            StyleFunc::new::<FlexDirectionType>(), // 62
            StyleFunc::new::<FlexWrapType>(), // 63
            StyleFunc::new::<JustifyContentType>(), // 64
            StyleFunc::new::<AlignContentType>(), // 65
            StyleFunc::new::<AlignItemsType>(), // 66


            StyleFunc::new::<PositionTypeType>(), // 67
            StyleFunc::new::<AlignSelfType>(), // 68
            StyleFunc::new::<FlexShrinkType>(), // 79
            StyleFunc::new::<FlexGrowType>(), // 70
            StyleFunc::new::<AspectRatioType>(), // 71
            StyleFunc::new::<OrderType>(), // 72
            StyleFunc::new::<FlexBasisType>(), // 73
            StyleFunc::new::<OpacityType>(), // 74

            StyleFunc::new::<TextContentType>(), // 75

            StyleFunc::new::<VNodeType>(), // 76

            StyleFunc::new::<AnimationNameType>(), // 77
            StyleFunc::new::<AnimationDurationType>(), // 78
            StyleFunc::new::<AnimationTimingFunctionType>(), // 79
            StyleFunc::new::<AnimationDelayType>(), // 80
            StyleFunc::new::<AnimationIterationCountType>(), // 81
            StyleFunc::new::<AnimationDirectionType>(), // 82
            StyleFunc::new::<AnimationFillModeType>(), // 83
            StyleFunc::new::<AnimationPlayStateType>(), // 84
            StyleFunc::new::<ClipPathType>(), // 85
            StyleFunc::new::<TranslateType>(), // 86
            StyleFunc::new::<ScaleType>(), // 87
            StyleFunc::new::<RotateType>(), // 88
            StyleFunc::new::<AsImageType>(), // 89
			StyleFunc::new::<TextOverflowType>(), // 90
			StyleFunc::new::<OverflowWrapType>(), // 91

			StyleFunc::new::<TransitionPropertyType>(), // 92
			StyleFunc::new::<TransitionDurationType>(), // 93
			StyleFunc::new::<TransitionTimingFunctionType>(), // 94
			StyleFunc::new::<TransitionDelayType>(), // 95

            StyleFunc::new::<TextOuterGlowType>(), // 96
            // StyleFunc::new::<RowGapType>(), // 97
            // StyleFunc::new::<ColumnGapType>(), // 98
            // StyleFunc::new::<AutoReduceType>(), // 99
		];

		
		static ref RESET_STYLE_ATTR: [ResetStyleFunc; 97] = [
        /******************************* reset ******************************************************/
            ResetStyleFunc::new::<ResetBackgroundRepeatType>(), // 0
            ResetStyleFunc::new::<ResetFontStyleType>(), // 1
            ResetStyleFunc::new::<ResetFontWeightType>(), // 2
            ResetStyleFunc::new::<ResetFontSizeType>(), // 3
            ResetStyleFunc::new::<ResetFontFamilyType>(), // 4
            ResetStyleFunc::new::<ResetLetterSpacingType>(), // 5
            ResetStyleFunc::new::<ResetWordSpacingType>(), // 6
            ResetStyleFunc::new::<ResetLineHeightType>(), // 7
            ResetStyleFunc::new::<ResetTextIndentType>(), // 8
            ResetStyleFunc::new::<ResetWhiteSpaceType>(), // 9

            ResetStyleFunc::new::<ResetTextAlignType>(), // 10
            ResetStyleFunc::new::<ResetVerticalAlignType>(), // 11
            ResetStyleFunc::new::<ResetColorType>(), // 12
            ResetStyleFunc::new::<ResetTextStrokeType>(), // 13
            ResetStyleFunc::new::<ResetTextShadowType>(), // 14

            ResetStyleFunc::new::<ResetBackgroundImageType>(), // 15
            ResetStyleFunc::new::<ResetBackgroundImageClipType>(), // 16
            ResetStyleFunc::new::<ResetObjectFitType>(), // 17
            ResetStyleFunc::new::<ResetBackgroundColorType>(), // 18
            ResetStyleFunc::new::<ResetBoxShadowType>(), // 19
            ResetStyleFunc::new::<ResetBorderImageType>(), // 20
            ResetStyleFunc::new::<ResetBorderImageClipType>(), // 21
            ResetStyleFunc::new::<ResetBorderImageSliceType>(), // 22
            ResetStyleFunc::new::<ResetBorderImageRepeatType>(), // 23

            ResetStyleFunc::new::<ResetBorderColorType>(), // 24


            ResetStyleFunc::new::<ResetHsiType>(), // 25
            ResetStyleFunc::new::<ResetBlurType>(), // 26
            ResetStyleFunc::new::<ResetMaskImageType>(), // 27
            ResetStyleFunc::new::<ResetMaskImageClipType>(), // 28
            ResetStyleFunc::new::<ResetTransformType>(), // 29
            ResetStyleFunc::new::<ResetTransformOriginType>(), // 30
            ResetStyleFunc::new::<ResetTransformWillChangeType>(), // 31
            ResetStyleFunc::new::<ResetBorderRadiusType>(), // 32
            ResetStyleFunc::new::<ResetZIndexType>(), // 32
            ResetStyleFunc::new::<ResetOverflowType>(), // 34


            ResetStyleFunc::new::<ResetBlendModeType>(), // 35
            ResetStyleFunc::new::<ResetDisplayType>(), // 36
            ResetStyleFunc::new::<ResetVisibilityType>(), // 37
            ResetStyleFunc::new::<ResetEnableType>(), // 38


            ResetStyleFunc::new::<ResetWidthType>(), // 39
            ResetStyleFunc::new::<ResetHeightType>(), // 40

            ResetStyleFunc::new::<ResetMarginTopType>(), // 41
            ResetStyleFunc::new::<ResetMarginRightType>(), // 42
            ResetStyleFunc::new::<ResetMarginBottomType>(), // 43
            ResetStyleFunc::new::<ResetMarginLeftType>(), // 44

            ResetStyleFunc::new::<ResetPaddingTopType>(), // 45
            ResetStyleFunc::new::<ResetPaddingRightType>(), // 46
            ResetStyleFunc::new::<ResetPaddingBottomType>(), // 47
            ResetStyleFunc::new::<ResetPaddingLeftType>(), // 48

            ResetStyleFunc::new::<ResetBorderTopType>(), // 49
            ResetStyleFunc::new::<ResetBorderRightType>(), // 50
            ResetStyleFunc::new::<ResetBorderBottomType>(), // 51
            ResetStyleFunc::new::<ResetBorderLeftType>(), // 52

            ResetStyleFunc::new::<ResetPositionTopType>(), // 53
            ResetStyleFunc::new::<ResetPositionRightType>(), // 54
            ResetStyleFunc::new::<ResetPositionBottomType>(), // 55
            ResetStyleFunc::new::<ResetPositionLeftType>(), // 56

            ResetStyleFunc::new::<ResetMinWidthType>(), // 57
            ResetStyleFunc::new::<ResetMinHeightType>(), // 58
            ResetStyleFunc::new::<ResetMaxHeightType>(), // 59
            ResetStyleFunc::new::<ResetMaxWidthType>(), // 60
            ResetStyleFunc::new::<ResetDirectionType>(), // 61
            ResetStyleFunc::new::<ResetFlexDirectionType>(), // 62
            ResetStyleFunc::new::<ResetFlexWrapType>(), // 63
            ResetStyleFunc::new::<ResetJustifyContentType>(), // 64
            ResetStyleFunc::new::<ResetAlignContentType>(), // 65
            ResetStyleFunc::new::<ResetAlignItemsType>(), // 66


            ResetStyleFunc::new::<ResetPositionTypeType>(), // 67
            ResetStyleFunc::new::<ResetAlignSelfType>(), // 68
            ResetStyleFunc::new::<ResetFlexShrinkType>(), // 69
            ResetStyleFunc::new::<ResetFlexGrowType>(), // 70
            ResetStyleFunc::new::<ResetAspectRatioType>(), // 71
            ResetStyleFunc::new::<ResetOrderType>(), // 72
            ResetStyleFunc::new::<ResetFlexBasisType>(), // 73
            ResetStyleFunc::new::<ResetOpacityType>(), // 74

            ResetStyleFunc::new::<ResetTextContentType>(), // 75

            ResetStyleFunc::new::<ResetVNodeType>(), // 76

            ResetStyleFunc::new::<ResetAnimationNameType>(), // 77
            ResetStyleFunc::new::<ResetAnimationDurationType>(), // 78
            ResetStyleFunc::new::<ResetAnimationTimingFunctionType>(), // 79
            ResetStyleFunc::new::<ResetAnimationDelayType>(), // 80
            ResetStyleFunc::new::<ResetAnimationIterationCountType>(), // 81
            ResetStyleFunc::new::<ResetAnimationDirectionType>(), // 82
            ResetStyleFunc::new::<ResetAnimationFillModeType>(), // 83
            ResetStyleFunc::new::<ResetAnimationPlayStateType>(), // 84

            ResetStyleFunc::new::<ResetClipPathType>(), // 85
            ResetStyleFunc::new::<ResetTranslateType>(), // 86
            ResetStyleFunc::new::<ResetScaleType>(), // 87
            ResetStyleFunc::new::<ResetRotateType>(), // 88
            ResetStyleFunc::new::<ResetAsImageType>(), // 89
			ResetStyleFunc::new::<ResetTextOverflowType>(), // 90
			ResetStyleFunc::new::<ResetOverflowWrapType>(), // 91

			ResetStyleFunc::new::<ResetTransitionPropertyType>(), // 92
			ResetStyleFunc::new::<ResetTransitionDurationType>(), // 93
			ResetStyleFunc::new::<ResetTransitionTimingFunctionType>(), // 94
			ResetStyleFunc::new::<ResetTransitionDelayType>(), // 95

            ResetStyleFunc::new::<ResetTextOuterGlowType>(), // 96
            // ResetStyleFunc::new::<ResetRowGapType>(), // 97
            // ResetStyleFunc::new::<ResetColumnGapType>(), // 98
            // ResetStyleFunc::new::<ResetAutoReduceType>(), // 99
        ];
    }

    pub struct Setting<'w> {
        // pub style: &'s mut StyleQuery<'w>,
        pub default_value: &'w DefaultStyle,
        pub style: &'w SettingComponentIds,
        pub world: &'w mut World,
    }
    // 默认值
    pub struct DefaultStyle {
        pub size: u32,
        pub margin: u32,
        pub padding: u32,
        pub border: u32,
        pub position: u32,
        pub min_max: u32,
        pub flex_container: u32,
        pub flex_normal: u32,
        pub z_index: u32,
        pub overflow: u32,
        pub opacity: u32,
        pub blend_mode: u32,
        pub show: u32,
        pub transform: u32,
        pub background_color: u32,
        pub border_color: u32,
        pub background_image: u32,
        pub background_image_texture: u32,
        pub background_image_clip: u32,
        pub mask_image: u32,
        pub mask_image_clip: u32,
        pub hsi: u32,
        pub blur: u32,
        pub clip_path: u32,
        pub background_image_mod: u32,
        pub border_image: u32,
        pub border_image_texture: u32,
        pub border_image_clip: u32,
        pub border_image_slice: u32,
        pub border_image_repeat: u32,
        pub border_radius: u32,
        pub box_shadow: u32,
        pub text_style: u32,
        pub text_shadow: u32,
        pub text_outer_glow: u32,
        pub transform_will_change: u32,
        pub text_content: u32,
        pub node_state: u32,
        pub animation: u32,
		pub transition: u32,
        pub class_name: u32,
        pub as_image: u32,
		pub text_overflow: u32,

        pub svg: u32,
    }

    impl FromWorld for DefaultStyle {
        fn from_world(world: &mut World) -> Self {
            Self {
                size: world.init_single_res::<Size>() as u32,
                margin: world.init_single_res::<Margin>() as u32,
                padding: world.init_single_res::<Padding>() as u32,
                border: world.init_single_res::<Border>() as u32,
                position: world.init_single_res::<Position>() as u32,
                min_max: world.init_single_res::<MinMax>() as u32,
                flex_container: world.init_single_res::<FlexContainer>() as u32,
                flex_normal: world.init_single_res::<FlexNormal>() as u32,
                z_index: world.init_single_res::<ZIndex>() as u32,
                overflow: world.init_single_res::<Overflow>() as u32,
                opacity: world.init_single_res::<Opacity>() as u32,
                blend_mode: world.init_single_res::<BlendMode>() as u32,
                show: world.init_single_res::<Show>() as u32,
                transform: world.init_single_res::<Transform>() as u32,
                background_color: world.init_single_res::<BackgroundColor>() as u32,
                border_color: world.init_single_res::<BorderColor>() as u32,
                background_image: world.init_single_res::<BackgroundImage>() as u32,
                background_image_texture: world.init_single_res::<BackgroundImageTexture>() as u32,
                background_image_clip: world.init_single_res::<BackgroundImageClip>() as u32,
                mask_image: world.init_single_res::<MaskImage>() as u32,
                mask_image_clip: world.init_single_res::<MaskImageClip>() as u32,
                hsi: world.init_single_res::<Hsi>() as u32,
                blur: world.init_single_res::<Blur>() as u32,
                clip_path: world.init_single_res::<ClipPath>() as u32,
                background_image_mod: world.init_single_res::<BackgroundImageMod>() as u32,
                border_image: world.init_single_res::<BorderImage>() as u32,
                border_image_texture: world.init_single_res::<BorderImageTexture>() as u32,
                border_image_clip: world.init_single_res::<BorderImageClip>() as u32,
                border_image_slice: world.init_single_res::<BorderImageSlice>() as u32,
                border_image_repeat: world.init_single_res::<BorderImageRepeat>() as u32,
                border_radius: world.init_single_res::<BorderRadius>() as u32,
                box_shadow: world.init_single_res::<BoxShadow>() as u32,
                text_style: world.init_single_res::<TextStyle>() as u32,
                text_shadow: world.init_single_res::<TextShadow>() as u32,
                text_outer_glow: world.init_single_res::<TextOuterGlow>() as u32,
                transform_will_change: world.init_single_res::<TransformWillChange>() as u32,
                text_content: world.init_single_res::<TextContent>() as u32,
                node_state: world.init_single_res::<NodeState>() as u32,
                animation: world.init_single_res::<Animation>() as u32,
                transition: world.init_single_res::<Transition>() as u32,
                class_name: world.init_single_res::<ClassName>() as u32,
                as_image: world.init_single_res::<AsImage>() as u32,
                text_overflow: world.init_single_res::<TextOverflow>() as u32,

                svg: world.init_single_res::<SvgInnerContent>() as u32,
            }
        }
    }

    pub struct StyleAttr;

    impl StyleAttr {
        #[inline]
        pub fn get_type(style_type: u8) -> StyleType { unsafe { transmute((STYLE_ATTR[style_type as usize].get_type)()) } }

        #[inline]
        pub unsafe fn write<T: Attr>(value: T, buffer: &mut Vec<u8>) {
            value.write(buffer);
            forget(value);
        }

        #[inline]
        pub fn set(
            cur_style_mark: &mut StyleMarkType,
            style_index: u8,
            buffer: &Vec<u8>,
            offset: usize,
            query: &mut Setting,
            entity: Entity,
            is_clone: bool,
        ) {
			if style_index > STYLE_COUNT {
				(RESET_STYLE_ATTR[style_index as usize - STYLE_COUNT as usize].set)(unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone)
			} else if style_index < STYLE_COUNT * 2  {
				(STYLE_ATTR[style_index as usize].set)(unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone);
                cur_style_mark.set(style_index as usize, true);
			} 
            // else if style_index < STYLE_COUNT * 2 + SVG_COUNT {
            //     // set svg
            //     (SVGTYPE_ATTR[style_index as usize].set)(unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone);
            // } else if style_index < STYLE_COUNT * 2 + SVG_COUNT * 2 {
            //     // reset svg
            //     (RESET_SVGTYPE_ATTR[style_index as usize].set)(unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone);
            // }
            
        }

        pub fn push_component_ops(
            style_index: u8,
            components: &SettingComponentIds,
            arr: &mut Vec<(ComponentIndex, bool)>,){
            
            if style_index > STYLE_COUNT {
                (RESET_STYLE_ATTR[style_index as usize - STYLE_COUNT as usize].push_component_ops)(components, arr)
            } else {
                (STYLE_ATTR[style_index as usize].push_component_ops)(components, arr)
            }

        }

		pub fn get(
            style_index: u8,
            world: &World,
            query: &SettingComponentIds,
            entity: Entity,
        ) -> Option<Attribute> {
            (STYLE_ATTR[style_index as usize].get)(world, query, entity)
        }

        #[inline]
        pub fn to_attr(style_index: u8, buffer: &Vec<u8>, offset: usize) -> Attribute {
            (STYLE_ATTR[style_index as usize].to_attr)(unsafe { buffer.as_ptr().add(offset) })
        }

        #[inline]
        pub fn size(style_index: u8) -> usize { 
			if style_index > STYLE_COUNT {
				0
			} else {
				(STYLE_ATTR[style_index as usize].size)()
			}
		 }

        #[inline]
        pub fn reset(_cur_style_mark: &mut StyleMarkType, style_index: u8, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity) {
            (RESET_STYLE_ATTR[style_index as usize].set)(unsafe { buffer.as_ptr().add(offset) }, query, entity, false);
        }

        #[inline]
        pub fn set_default(style_index: u8, buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
            (STYLE_ATTR[style_index as usize].set_default)(buffer, offset, world, query);
        }
    }

    /// svg属性类型
    #[enum_type]
    #[index_start(0)]
    #[func(SvgFunc)]
    #[reset_func(ResetStyleFunc)]
    pub enum SvgType {
        #[v(f32)]
        SvgWidth, // 0,
        #[v(f32)]
        SvgHeight, // 1,
        #[v(Color)]
        SvgColor, // 2,
        #[v(CgColor)]
        SvgStrokeColor, // 3,
        #[v(NotNan<f32>)]
        SvgStrokeWidth, // 4,
        #[v(StrokeDasharray)]
        StrokeDasharray, // 5,
        #[v(SvgShapeEnum)]
        SvgShape, // 6,
        #[v(f32)]
        SvgShapeWidth, // 7,
        #[v(f32)]
        SvgShapeHeight, // 8,
        #[v(f32)]
        SvgShapeX, // 9,
        #[v(f32)]
        SvgShapeY, // 10,
        #[v(f32)]
        SvgShapeCX, // 11,
        #[v(f32)]
        SvgShapeCY, // 12,
        #[v(f32)]
        SvgShapeRadius, // 13,
        #[v(f32)]
        SvgShapeRadiusX, // 14,
        #[v(f32)]
        SvgShapeRadiusY, // 15,
        #[v(f32)]
        SvgShapeAX, // 16,
        #[v(f32)]
        SvgShapeAY, // 17,
        #[v(f32)]
        SvgShapeBX, // 18,
        #[v(f32)]
        SvgShapeBY, // 19,
        #[v(Vec<f32>)]
        SvgShapePoints, // 20,
        #[v((Vec<f32>, Vec<u8>))]
        SvgShapePath, // 21,
        #[v(CgColor)]
        SvgShadowColor, // 22,
        #[v(f32)]
        SvgShadowOffsetX, // 23,
        #[v(f32)]
        SvgShadowOffsetY, // 24,
        #[v(f32)]
        SvgShadowBlurLevel, // 25,
        #[v(f32)]
        SvgFilterOffsetX, // 26,
        #[v(f32)]
        SvgFilterOffsetY, // 27,
        #[v(f32)]
        SvgFilterColor, // 28,
        #[v(f32)]
        SvgFilterBlurLevel, // 29,
        #[v(f32)]
        SvgGradientX1, // 30,
        #[v(f32)]
        SvgGradientY1, // 31,
        #[v(f32)]
        SvgGradientX2, // 32,
        #[v(f32)]
        SvgGradientY2, // 33,
        #[v(f32)]
        SvgStopOffset, // 34,
        #[v(CgColor)]
        SvgStopColor, // 35,
        #[v(Entity)]
        SvgGradient, // 36,
        #[v(Entity)]
        SvgFilter, // 37,
    }

    impl_style!(@svg SvgWidthType, SvgWidth, svg, SvgContent, v, f32,
        svg.width = v,
        svg.width,
        svg.width = v.width
    );

    impl_style!(@svg SvgHeightType, SvgHeight, svg, SvgContent, v, f32,
        svg.height = v,
        svg.height,
        svg.height = v.height
    );

    impl_style!(@svg SvgShapeType, SvgShape, svg, SvgInnerContent, v, SvgShapeEnum,
        match v {
            SvgShapeEnum::Rect => svg.shape = Shape::Rect{width: 0.0, height: 0.0, x: 0.0, y: 0.0},
            SvgShapeEnum::Circle => svg.shape = Shape::Circle{cx: 0.0, cy: 0.0, radius: 0.0},
            SvgShapeEnum::Ellipse => svg.shape = Shape::Ellipse{cy: 0.0, cx: 0.0, rx: 0.0, ry: 0.0},
            SvgShapeEnum::Segment => svg.shape = Shape::Segment{ax: 0.0, ay: 0.0, bx: 0.0, by: 0.0},
            SvgShapeEnum::Polygon => svg.shape = Shape::Polygon{points: Vec::default()},
            SvgShapeEnum::Polyline => svg.shape = Shape::Polyline{points: Vec::default()},
            SvgShapeEnum::Path => svg.shape = Shape::Path{ points: Vec::default(), verb: Vec::default()},
        },
        match &svg.shape {
            Shape::Rect {..} => SvgShapeEnum::Rect,
            Shape::Circle {..} => SvgShapeEnum::Circle,
            Shape::Ellipse {..} => SvgShapeEnum::Ellipse,
            Shape::Segment {..} => SvgShapeEnum::Segment,
            Shape::Polygon {..} => SvgShapeEnum::Polygon,
            Shape::Polyline {..} => SvgShapeEnum::Polyline,
            Shape::Path {..} => SvgShapeEnum::Path,
        },
        ()
    );
    impl_style!(@svg SvgShapeWidthType, SvgShapeWidth, svg, SvgInnerContent, v, f32,
        if let Shape::Rect {width, ..} = &mut svg.shape{ *width = v; },
        if let Shape::Rect {width, ..} = &svg.shape{ *width } else { Default::default() },
        if let (Shape::Rect {width, ..}, Shape::Rect {width: v, ..}) = (&mut svg.shape, &v.shape){ *width = *v; }
    );
    impl_style!(@svg SvgShapeHeightType, SvgShapeHeight, svg, SvgInnerContent, v, f32,
        if let Shape::Rect {height, ..} = &mut svg.shape{ *height = v; },
        if let Shape::Rect {height, ..} = &svg.shape{ *height } else { Default::default() },
        if let (Shape::Rect {height, ..}, Shape::Rect {height: v, ..}) = (&mut svg.shape, &v.shape){ *height = *v; }
    );
    impl_style!(@svg SvgShapeXType, SvgShapeX, svg, SvgInnerContent, v, f32,
        if let Shape::Rect {x, ..} = &mut svg.shape{ *x = v; },
        if let Shape::Rect {x, ..} = &svg.shape{ *x } else { Default::default() },
        if let (Shape::Rect {x, ..}, Shape::Rect {x: v, ..}) = (&mut svg.shape, &v.shape){ *x = *v; }
    );
    impl_style!(@svg SvgShapeYType, SvgShapeY, svg, SvgInnerContent, v, f32,
        if let Shape::Rect {y, ..} = &mut svg.shape{ *y = v; },
        if let Shape::Rect {y, ..} = &svg.shape{ *y } else { Default::default() },
        if let (Shape::Rect {y, ..}, Shape::Rect {y: v, ..}) = (&mut svg.shape, &v.shape){ *y = *v; }
    );
    impl_style!(@svg SvgShapeCXType, SvgShapeCX, svg, SvgInnerContent, v, f32,
        if let Shape::Circle { cx, .. } | Shape::Ellipse { cx, ..} = &mut svg.shape{ *cx = v; },
        if let Shape::Circle { cx, .. } | Shape::Ellipse { cx, ..} = &svg.shape{ *cx } else { Default::default() },
        if let (Shape::Circle { cx, .. } | Shape::Ellipse { cx, ..}, Shape::Circle { cx: v, .. } | Shape::Ellipse { cx: v, ..}) = (&mut svg.shape, &v.shape){ *cx = *v; }
    );
    impl_style!(@svg SvgShapeCYType, SvgShapeCY, svg, SvgInnerContent, v, f32,
        if let Shape::Circle { cy, .. } | Shape::Ellipse { cy, ..} = &mut svg.shape{ *cy = v; },
        if let Shape::Circle { cy, .. } | Shape::Ellipse { cy, ..} = &svg.shape{ *cy } else { Default::default() },
        if let (Shape::Circle { cy, .. } | Shape::Ellipse { cy, ..}, Shape::Circle { cy: v, .. } | Shape::Ellipse { cy: v, ..}) = (&mut svg.shape, &v.shape){ *cy = *v; }
    );
    impl_style!(@svg SvgShapeRadiusType, SvgShapeRadius, svg, SvgInnerContent, v, f32,
        if let Shape::Circle { radius, .. } = &mut svg.shape{ *radius = v; },
        if let Shape::Circle { radius, .. } = &svg.shape{ *radius } else { Default::default() },
        if let (Shape::Circle { radius, .. }, Shape::Circle { radius: v, .. }) = (&mut svg.shape, &v.shape){ *radius = *v; }
    );
    impl_style!(@svg SvgShapeRadiusXType, SvgShapeRadiusX, svg, SvgInnerContent, v, f32,
        if let Shape::Ellipse { rx, .. } = &mut svg.shape{ *rx = v; },
        if let Shape::Ellipse { rx, .. } = &svg.shape{ *rx } else { Default::default() },
        if let (Shape::Ellipse { rx, .. }, Shape::Ellipse { rx: v, .. }) = (&mut svg.shape, &v.shape){ *rx = *v; }
    );
    impl_style!(@svg SvgShapeRadiusYType, SvgShapeRadiusY, svg, SvgInnerContent, v, f32,
        if let Shape::Ellipse { ry, .. } = &mut svg.shape{ *ry = v; },
        if let Shape::Ellipse { ry, .. } = &svg.shape{ *ry } else { Default::default() },
        if let (Shape::Ellipse { ry, .. }, Shape::Ellipse { ry: v, .. }) = (&mut svg.shape, &v.shape){ *ry = *v; }
    );
    impl_style!(@svg SvgShapeAXType, SvgShapeAX, svg, SvgInnerContent, v, f32,
        if let Shape::Segment { ax, .. } = &mut svg.shape{ *ax = v; },
        if let Shape::Segment { ax, .. } = &svg.shape{ *ax } else { Default::default() },
        if let (Shape::Segment { ax, .. }, Shape::Segment { ax: v, .. }) = (&mut svg.shape, &v.shape){ *ax = *v; }
    );
    impl_style!(@svg SvgShapeAYType, SvgShapeAY, svg, SvgInnerContent, v, f32,
        if let Shape::Segment { ay, .. } = &mut svg.shape{ *ay = v; },
        if let Shape::Segment { ay, .. } = &svg.shape{ *ay } else { Default::default() },
        if let (Shape::Segment { ay, .. }, Shape::Segment { ay: v, .. }) = (&mut svg.shape, &v.shape){ *ay = *v; }
    );
    impl_style!(@svg SvgShapeBXType, SvgShapeBX, svg, SvgInnerContent, v, f32,
        if let Shape::Segment { bx, .. } = &mut svg.shape{ *bx = v; },
        if let Shape::Segment { bx, .. } = &svg.shape{ *bx } else { Default::default() },
        if let (Shape::Segment { bx, .. }, Shape::Segment { bx: v, .. }) = (&mut svg.shape, &v.shape){ *bx = *v; }
    );
    impl_style!(@svg SvgShapeBYType, SvgShapeBY, svg, SvgInnerContent, v, f32,
        if let Shape::Segment { by, .. } = &mut svg.shape{ *by = v; },
        if let Shape::Segment { by, .. } = &svg.shape{ *by } else { Default::default() },
        if let (Shape::Segment { by, .. }, Shape::Segment { by: v, .. }) = (&mut svg.shape, &v.shape){ *by = *v; }
    );
    impl_style!(@svg SvgShapePointsType, SvgShapePoints, svg, SvgInnerContent, v, Vec<f32>,
        if let Shape::Polygon { points } = &mut svg.shape{ *points = v; },
        if let Shape::Polygon { points } = &svg.shape{ points.clone() } else { Default::default() },
        if let (Shape::Polygon { points }, Shape::Polygon { points: v }) = (&mut svg.shape, &v.shape){ *points = v.clone(); }
    );
    impl_style!(@svg SvgShapePathType, SvgShapePath, svg, SvgInnerContent, v, (Vec<f32>, Vec<u8>),
        if let Shape::Path { points, verb } = &mut svg.shape{ *points = v.0; *verb = v.1; },
        if let Shape::Path { points, verb } = &svg.shape{ (points.clone(), verb.clone()) } else { Default::default() },
        if let (Shape::Path { points, verb }, Shape::Path { points: v, verb: v1 }) = (&mut svg.shape, &v.shape){ *points = v.clone(); *verb = v1.clone(); }
    );

    impl_style!(@svg SvgColorType, SvgColor, svg, SvgInnerContent, v, Color,
        svg.style.fill_color = v,
        svg.style.fill_color.clone(),
        svg.style.fill_color = v.style.fill_color.clone()
    );
    impl_style!(@svg SvgStrokeColorType, SvgStrokeColor, svg, SvgInnerContent, v, CgColor,
        svg.style.stroke.color = v,
        svg.style.stroke.color.clone(),
        svg.style.stroke.color = v.style.stroke.color.clone()
    );
    impl_style!(@svg SvgStrokeWidthType, SvgStrokeWidth, svg, SvgInnerContent, v, NotNan<f32>,
        svg.style.stroke.width = v,
        svg.style.stroke.width.clone(),
        svg.style.stroke.width = v.style.stroke.width.clone()
    );
    impl_style!(@svg StrokeDasharrayType, StrokeDasharray, svg, SvgInnerContent, v, StrokeDasharray,
        svg.style.stroke_dasharray = v,
        svg.style.stroke_dasharray.clone(),
        svg.style.stroke_dasharray = v.style.stroke_dasharray.clone()
    );
    impl_style!(@svg SvgShadowColorType, SvgShadowColor, svg, SvgInnerContent, v, CgColor,
        svg.style.shadow.color = v,
        svg.style.shadow.color.clone(),
        svg.style.shadow.color = v.style.shadow.color.clone()
    );
    impl_style!(@svg SvgShadowOffsetXType, SvgShadowOffsetX, svg, SvgInnerContent, v, f32,
        svg.style.shadow.offset_x = v,
        svg.style.shadow.offset_x.clone(),
        svg.style.shadow.offset_x = v.style.shadow.offset_x.clone()
    );
    impl_style!(@svg SvgShadowOffsetYType, SvgShadowOffsetY, svg, SvgInnerContent, v, f32,
        svg.style.shadow.offset_y = v,
        svg.style.shadow.offset_y.clone(),
        svg.style.shadow.offset_y = v.style.shadow.offset_y.clone()
    );
    impl_style!(@svg SvgShadowBlurLevelType, SvgShadowBlurLevel, svg, SvgInnerContent, v, f32,
        svg.style.shadow.blur_level = v,
        svg.style.shadow.blur_level.clone(),
        svg.style.shadow.blur_level = v.style.shadow.blur_level.clone()
    );
    impl_style!(@svg SvgFilterType, SvgFilter, svg, SvgInnerContent, v, Entity,
        svg.style.filter = v,
        svg.style.filter.clone(),
        svg.style.filter = v.style.filter.clone()
    );
    // TODO
    impl_style!(@svg SvgGradientType, SvgGradient, svg, SvgGradient, v, Entity,
        svg.id.push(v),
        svg.id.last().map_or(Entity::null(), |r| {r.clone()}),
        svg.id = v.id.clone()
    );

    impl_style!(@svg SvgFilterOffsetXType, SvgFilterOffsetX, svg, SvgFilterOffset, v, f32,
        svg.offset_x = v,
        svg.offset_x.clone(),
        svg.offset_x = v.offset_x.clone()
    );
    impl_style!(@svg SvgFilterOffsetYType, SvgFilterOffsetY, svg, SvgFilterOffset, v, f32,
        svg.offset_y = v,
        svg.offset_y.clone(),
        svg.offset_y = v.offset_y.clone()
    );
    impl_style!(@svg SvgFilterColorType, SvgFilterColor, svg, SvgFilterOffset, v, f32,
        svg.color = v,
        svg.color.clone(),
        svg.color = v.color.clone()
    );
    impl_style!(@svg SvgFilterBlurLevelType, SvgFilterBlurLevel, svg, SvgFilterBlurLevel, v, f32,
        svg.level = v,
        svg.level.clone(),
        svg.level = v.level.clone()
    );
    impl_style!(@svg SvgGradientX1Type, SvgGradientX1, svg, SvgGradient, v, f32,
        svg.x1 = v,
        svg.x1.clone(),
        svg.x1 = v.x1.clone()
    );
    impl_style!(@svg SvgGradientY1Type, SvgGradientY1, svg, SvgGradient, v, f32,
        svg.y1 = v,
        svg.y1.clone(),
        svg.y1 = v.y1.clone()
    );
    impl_style!(@svg SvgGradientX2Type, SvgGradientX2, svg, SvgGradient, v, f32,
        svg.x2 = v,
        svg.x2.clone(),
        svg.x2 = v.x2.clone()
    );
    impl_style!(@svg SvgGradientY2Type, SvgGradientY2, svg, SvgGradient, v, f32,
        svg.y2 = v,
        svg.y2.clone(),
        svg.y2 = v.y2.clone()
    );
    impl_style!(@svg SvgStopOffsetType, SvgStopOffset, svg, SvgStop, v, f32,
        svg.offset = v,
        svg.offset.clone(),
        svg.offset = v.offset.clone()
    );
    impl_style!(@svg SvgStopColorType, SvgStopColor, svg, SvgStop, v, CgColor,
        svg.color = v,
        svg.color.clone(),
        svg.color = v.color.clone()
    );
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StyleAttribute {
    Reset(u8),
    Set(Attribute),
}

pub fn style_attr_list_to_buffer(style_buffer: &mut Vec<u8>, style_list: &mut VecDeque<StyleAttribute>, mut count: usize) -> ClassMeta {
    let start = style_buffer.len();
    let mut class_meta = ClassMeta {
        start,
        end: start,
        class_style_mark: BitArray::default(),
    };

    loop {
        if count == 0 {
            break;
        }
        let r = style_list.pop_front().unwrap();
        match r {
            StyleAttribute::Reset(r) => style_buffer.push(r),
            StyleAttribute::Set(r) => {
                match &r {
                    // pi_style::style_parse::Attribute::AnimationName(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDuration(_) => (),
                    // pi_style::style_parse::Attribute::AnimationTimingFunction(_) => (),
                    // pi_style::style_parse::Attribute::AnimationIterationCount(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDirection(_) => (),
                    // pi_style::style_parse::Attribute::AnimationFillMode(_) => (),
                    // pi_style::style_parse::Attribute::AnimationPlayState(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDelay(_) => (),
                    // pi_style::style_parse::Attribute::BackgroundImage(_) => (),
                    // pi_style::style_parse::Attribute::BackgroundImageClip(_) => (),
                    _ => {
                        style_to_buffer(style_buffer, r, &mut class_meta);
                    }
                };
                
            },
        }

        count -= 1;
    }
    class_meta.end = style_buffer.len();

    class_meta
}




// clone指针指向的对象（可能未对齐）
fn clone_unaligned<T: Clone>(src: *const T) -> T {
	let r = unsafe {read_unaligned(src)};
	let ret = r.clone();
	forget(r); // 这里忘记r， 是因为read_unaligned对src进行逐位读取，如果不忘记r， src指向的对象会被释放、而此函数仅仅是想拷贝src
	ret
}















/******************************************************************svg属性*********************************************************************/

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SvgShapeEnum {
	Rect,
    Circle,
    Ellipse,
    Segment,
    Polygon,
    Polyline,
    Path,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rect { x: f32, y: f32, width: f32, height: f32 },
    Circle { cx: f32, cy: f32, radius: f32 },
    Ellipse { cx: f32, cy: f32, rx: f32, ry: f32 },
    Segment { ax: f32, ay: f32, bx: f32, by: f32},
    Polygon { points: Vec<f32> },
    Polyline { points: Vec<f32> },
    Path { points: Vec<f32>, verb: Vec<u8>}
}

impl Shape {
    pub fn is_ready(&self) -> bool {
        match self {
            Shape::Rect { x, y, width, height } => !(x.is_infinite() || y.is_infinite() || width.is_infinite() || height.is_infinite()),
            Shape::Circle { cx, cy, radius } => !(cx.is_infinite() || cy.is_infinite() || radius.is_infinite()),
            Shape::Ellipse { cx, cy, rx, ry } => !(cx.is_infinite() || cy.is_infinite() || rx.is_infinite() || ry.is_infinite()),
            Shape::Segment { ax, ay, bx, by } => !(ax.is_infinite() || ay.is_infinite() || bx.is_infinite() || by.is_infinite()),
            Shape::Polygon { points } => !points.is_empty(),
            Shape::Polyline { points } => !points.is_empty(),
            Shape::Path { points, verb } => !(points.is_empty() || verb.is_empty()),
        }
    }

    pub fn hash(&self) -> u64 {
        use std::hash::Hasher;

        let mut hasher = pi_hash::DefaultHasher::default();
        let data = match self {
            Shape::Rect { x, y, width, height } => vec![*x, *y, *width, *height, 1.0],
            Shape::Circle { cx, cy, radius } => vec![*cx, *cy, *radius, 2.0],
            Shape::Ellipse { cx, cy, rx, ry } => vec![*cx, *cy, *rx, *ry, 3.0],
            Shape::Segment { ax, ay, bx, by } => vec![*ax, *ay, *bx, *by, 4.0],
            Shape::Polygon { points } => {
                let mut p = points.clone();
                p.push(5.0);
                p
            }
            Shape::Polyline { points } => {
                let mut p = points.clone();
                p.push(6.0);
                p
            }
            Shape::Path { points, verb } => {
                let mut p = points.clone();
                let mut v = verb.iter().map(|v| (*v).into()).collect::<Vec<f32>>();
                p.append(&mut v);
                p.push(7.0);
                p
            }
        };
        println!("hash data: {:?}", bytemuck::cast_slice::<_, u8>(data.as_slice()));
        hasher.write(bytemuck::cast_slice::<_, u8>(data.as_slice()));
        let hash = hasher.finish() as u64;
        println!("hash: {}", hash);
        hash
    }
}

#[rustfmt::skip]
impl From<(f64, &[f32], &[f32])> for Shape {
    fn from(value: (f64, &[f32], &[f32])) -> Self {
        match value.0 as u8 {
            1 => Self::Rect { x: value.2[0], y: value.2[1], width: value.2[2], height: value.2[3]},
            2 => Self::Circle { cx: value.2[0], cy: value.2[1], radius: value.2[2]},
            3 => Self::Ellipse { cx: value.2[0], cy: value.2[1], rx: value.2[2], ry: value.2[3]},
            4 => Self::Segment { ax: value.2[0], ay: value.2[1], bx: value.2[2], by: value.2[3]},
            5 => Self::Polygon { points: value.2.to_vec()},
            6 => Self::Polyline { points: value.2.to_vec()},
            7 => Self::Path { 
                points: value.2.to_vec(), 
                verb: value.1.iter().map(|v|*v as u8 ).collect::<Vec<u8>>()
            },
            _ => panic!("svg not surpport shape; ty = {}", value.0 )
        }
    }
}

#[rustfmt::skip]
impl From<f32> for Shape {
    fn from(value: f32) -> Self {
        match value as u8 {
            0 => Self::Rect { x: f32::INFINITY, y: f32::INFINITY, width: f32::INFINITY, height: f32::INFINITY},
            1 => Self::Circle { cx: f32::INFINITY, cy:  f32::INFINITY, radius:  f32::INFINITY},
            2 => Self::Ellipse { cx: f32::INFINITY, cy:  f32::INFINITY, rx:  f32::INFINITY, ry:  f32::INFINITY},
            3 => Self::Segment { ax: f32::INFINITY, ay:  f32::INFINITY, bx: f32::INFINITY, by:  f32::INFINITY},
            4 => Self::Polygon { points: vec![]},
            5 => Self::Polyline { points: vec![]},
            6 => Self::Path { points: vec![], verb: vec![]},
            _ => panic!("svg not surpport shape; ty = {}", value )
        }
    }
}

impl Default for Shape {
    fn default() -> Self { Self::from(0.0) }
}
