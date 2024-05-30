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
    style_type::ClassMeta,
};

use pi_world::world::World;
use crate::resource::animation_sheet::TransitionData;

use super::calc::{NeedMark, EntityKey, StyleMarkType};
pub use super::root::{ClearColor, RenderDirty, RenderTargetType, Viewport};
use smallvec::SmallVec;

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

pub const STYLE_COUNT: u8 = 128;

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


#[derive(Default, Component)]
pub struct SvgInnerContent {
    pub shape: Shape,
    pub style: SvgStyle,
    pub hash: u64,
}

#[derive(Default, Component)]
pub struct SvgContent {
    pub width: f32,
    pub height: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(Debug, Default, Component)]
pub struct SvgGradient {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub id: Vec<Entity>
}

#[derive(Debug, Default, Component)]
pub struct SvgStop {
    pub offset: f32,
    pub color: CgColor,
}

#[derive(Debug, Default)]
pub struct SvgFilterBlurLevel {
    pub level: f32,
}

#[derive(Debug, Default)]
pub struct SvgFilterOffset {
    pub offset_x: f32,
    pub offset_y: f32,
    pub color: f32,
}

#[derive(Debug, Default)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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


pub type TextShadowList = SmallVec<[TextShadow1; 1]>;

// TransformWillChange， 用于优化频繁变化的Transform
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
// #[component(storage = "SparseSet")]
pub struct TransformWillChange(pub Option<AllTransform>); //

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
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

	pub trait ConvertToComponent: AttrSet {
		/// 获取属性
		fn get(
            world: &World,
			query: &SettingComponentIds,
			entity: Entity,
		) -> Option<Attribute>;

        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle)
        where
            Self: Sized;

        fn to_attr(ptr: *const u8) -> Attribute
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
        use pi_key_alloter::Key;
		// log::debug!("type: {:?}, entity: {:?}", std::any::type_name::<C>(), entity);
        log::debug!(
            "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
            std::any::type_name::<C>(),
            v,
            entity
        );
        
        // pi_print_any::out_any!(println, "set_default_style_attr==={:?}", (entity, std::any::type_name::<C>(), &v));
        match world.get_component_by_index_mut::<C>(entity, component_id) {
            Ok(mut component) => {
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
        match world.get_component_by_index_mut::<C>(entity, component_id) {
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

    // pub unsafe fn get_component_by_index_mut<C: Clone + Default>(world: &mut World, entity: Entity, component_id: ColumnIndex) -> &mut C {
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
            Some(r) => unsafe {transmute(r.0)},
            None => return,
        };
        if let Ok(mut component) = world.get_component_by_index_mut::<C>(entity, component_id) {
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
        // 整体插入
        ($name: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
					// query.style.dirty_list,
                    v,
                    |item: &mut $value_ty, v: $value_ty| *item = v,
                );
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };
        // 表达式
        (@fun $name: ident, $value_ty: ty, $f: expr) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(&mut query.world, query.style.$name, entity, v, $f);
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };

        (@fun_send $name: ident, $value_ty: ty, $c_ty: ty, $f: expr) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
					clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(&mut query.world, query.style.$name, entity, v, $f);
                // if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.$name) {
                //     unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<$c_ty>>>>() }
                //         .send(ComponentEvent::<Changed<$c_ty>>::new(entity));
                // };
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };
        // 属性修改
        (@pack $name: ident, $pack_ty: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                // 取不到说明实体已经销毁
                set_style_attr(&mut query.world, query.style.$name, entity, v, |item: &mut $pack_ty, v: $value_ty| *item = $pack_ty(v));
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                set_style_attr(&mut query.world, query.style.$name, entity, v, |mut item: Mut<$c_ty>, v: $value_ty| {
                    item.$feild = v;
                });
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };
        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(&mut query.world, query.style.$name, entity, v, |mut item: Mut<$c_ty>, v: $value_ty| {
                    item.$set_func(v);
                });
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };

        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                set_style_attr(&mut query.world, query.style.$name, entity, v, |mut item: Mut<$c_ty>, v: $value_ty| {
                    item.$feild1.$feild2 = v;
                },);
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };

        // 盒模属性（上右下左）
        (@box_model $name: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };

                set_style_attr(&mut query.world, query.style.$name, entity, v, |item: &mut $value_ty, v: $value_ty| {
                    *item = v;
                });

                cur_style_mark.set(Self::get_type() as usize, true);
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                arr.push((ids.$name, true))
            }
        };
    }

	macro_rules! get {
		(@empty) => {
			fn get(_world: &World, _query: &SettingComponentIds, _entity: Entity) -> Option<Attribute> {
				None
			}
		};
        // 整体插入
        ($name: ident, $ty: ident, $struct_name: ident, $value_ty: ty) => {
            fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
				match world.get_component_by_index::<$ty>(entity, query.$name) {
					Some(mut component) => Some(Attribute::$ty($struct_name(component.clone())))
					None => None
				}
            }
        };

		// 属性修改
        (@pack $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty) => {
			fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
                match world.get_component_by_index::<$component_ty>(entity, query.$name) {
					Ok(component) => Some(Attribute::$ty($struct_name(component.0.clone()))),
					_ => None
				}
            }
        };

		// 属性修改
        (@feild $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field: ident) => {
            fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
                match world.get_component_by_index::<$component_ty>(entity, query.$name) {
					Ok(component) => Some(Attribute::$ty($struct_name(component.$field.clone()))),
					_ => None
				}
            }
        };

        // 属性修改
        (@feild2 $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field1: ident, $field2: ident) => {
			fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
                match world.get_component_by_index::<$component_ty>(entity, query.$name) {
					Ok(component) => Some(Attribute::$ty($struct_name(component.$field1.$field2.clone()))),
					_ => None
				}
            }
        };

		// 属性修改
		(@feild3 $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field1: ident, $field2: ident, $field3: ident) => {
			fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
                match world.get_component_by_index::<$component_ty>(entity, query.$name) {
					Ok(component) => Some(Attribute::$ty($struct_name(component.$field1.$field2.$field3.clone()))),
					_ => None
				}
            }
        };

        // 表达式
        (@fun $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $f: ident) => {
            fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
                match world.get_component_by_index::<$component_ty>(entity, query.$name) {
					Ok(component) => Some(Attribute::$ty($struct_name(component.$f()))),
					_ => None
				}
            }
        };
    }

    // 设置默认值
    macro_rules! set_default {
        (@empty) => {
            fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle) {}
        };
        // 整体插入
        ($name: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                if let Some((component, _)) = world.index_single_res_mut::<$value_ty>(query.$name as usize) {
                    *component = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
                }
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                if let Some((component, _)) = world.index_single_res_mut::<$c_ty>(query.$name as usize) {
                    component.$feild = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
                }
            }
        };
        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                if let Some((component, _)) = world.index_single_res_mut::<$c_ty>(query.$name as usize) {
                    component.$set_func(unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() });
                }
            }
        };

        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                if let Some((component, _)) = world.index_single_res_mut::<$c_ty>(query.$name as usize) {
                    component.$feild1.$feild2 = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
                }
            }
        };

        // 盒模属性（上右下左）
        (@box_model $name: ident, $c_ty: ty, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
                if let Some((component, _)) = world.index_single_res_mut::<$c_ty>(query.$name as usize) {
                    let v = unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() };
                    component.top = v.top;
                    component.right = v.right;
                    component.bottom = v.bottom;
                    component.left = v.left;
                }
            }
        };
    }

    macro_rules! reset {
        // 空实现
        (@empty) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, _query: &mut Setting, _entity: Entity, _is_clone: bool) {}
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
        ($name: ident, $value_ty: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,

                    entity,
                    query.default_value.$name,
					
                    |item: &mut $value_ty, v: &$value_ty| {
                        *item = v.clone();
                    },
                );
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
		// 属性重置， 并发送事件
        (@func_send $name: ident, $value_ty: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    query.default_value.$name,
					
                    |item: &mut $value_ty, v: &$value_ty| {
                        *item = v.clone();
                    },
                );
				// if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.$name) {
				// 	unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<$value_ty>>>>() }
				// 		.send(ComponentEvent::<Changed<$value_ty>>::new(entity));
				// };
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    query.default_value.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild = v.$feild.clone();
                    },
                );
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };

        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $get_func: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    query.default_value.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$set_func(v.$get_func());
                    },
                );
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    query.default_value.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild1.$feild2 = v.$feild1.$feild2.clone();
                    },
                );
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
        // 属性修改
        (@box_model_single $name: ident, $c_ty: ty, $feild: ident) => {
            fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    query.default_value.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild = v.$feild;
                    },
                );
            }
            fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };

        (@box_model $name: ident, $ty: ident) => {
            fn set(cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    query.style.$name,
                    entity,
                    &**query.style.default.$name,
					
                    |item: &mut $value_ty, v: &$value_ty| {
                        let mut is_changed = false;
                        $crate::paste::item! {
                            if !cur_style_mark[StyleType::[<$ty Top>] as usize] {
                                is_changed = true;
                                item.top = v.top;
                            }
                            if !cur_style_mark[StyleType::[<$ty Right>] as usize] {
                                is_changed = true;
                                item.right = v.right;
                            }
                            if !cur_style_mark[StyleType::[<$ty Bottom>] as usize] {
                                is_changed = true;
                                item.bottom = v.bottom;
                            }
                            if !cur_style_mark[StyleType::[<$ty Left>] as usize] {
                                is_changed = true;
                                item.left = v.left;
                            }
                        }

                        // // 通知padding修改
                        // if is_changed {
                        //     item.notify_modify();
                        // }
                    },
                );
            }
            fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
                
            }
        };
    }

    macro_rules! impl_style {
        ($struct_name: ident) => {
            impl AttrSet for $struct_name {
                reset!(@empty);
            }
            
            impl ConvertToComponent for $struct_name {
                
                // reset!($name, $ty);
                #[allow(unused_variables)]
                fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle) {

                }
                fn to_attr(_ptr: *const u8) -> Attribute
                {
                    todo!()
                }
                get!(@empty);
            }
        };
        ($struct_name: ident, $name: ident, $ty: ident) => {

            impl AttrSet for $struct_name {
                set!($name, $ty);
            }

            impl ConvertToComponent for $struct_name {
                
                // reset!($name, $ty);
                set_default!($name, $ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$ty>())))
                }
                get!($name, $ty, $struct_name, $ty);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $ty);
                }
            }
        };

        (@pack $struct_name: ident, $name: ident, $pack_ty: ident, $value_ty: ident) => {

            impl AttrSet for $struct_name {
                set!(@fun $name, $value_ty, |mut item: Mut<$pack_ty>, v: $value_ty| *item = $pack_ty(v));
            }

            impl ConvertToComponent for $struct_name {
                // set!(@pack $name, $pack_ty, $value_ty);
                // reset!($name, $ty);
                set_default!($name, $pack_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$pack_ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@pack $name, $pack_ty, $struct_name, $pack_ty);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $pack_ty);
                }
            }
        };
        (@pack_compare $struct_name: ident, $name: ident, $pack_ty: ident, $value_ty: ident) => {

            impl AttrSet for $struct_name {
                set!(@fun $name, $value_ty, |mut item: Mut<$pack_ty>, v: $value_ty| {
                    if **item != v {
                        *item = $pack_ty(v)
                    }
                });
            }

            impl ConvertToComponent for $struct_name {
                // set!(@pack $name, $pack_ty, $value_ty);
                // reset!($name, $ty);
                set_default!($name, $pack_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$pack_ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@pack $name, $pack_ty, $struct_name, $pack_ty);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $pack_ty);
                }
            }
        };
        (@pack_send $struct_name: ident, $name: ident, $pack_ty: ident, $value_ty: ident) => {

            impl AttrSet for $struct_name {
                set!(@fun_send $name, $value_ty, $pack_ty, |mut item: Mut<$pack_ty>, v: $value_ty| *item = $pack_ty(v));
            }

            impl ConvertToComponent for $struct_name {
                
                // set!(@pack $name, $pack_ty, $value_ty);
                // reset!($name, $ty);
                set_default!($name, $pack_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$pack_ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }

                get!(@pack $name, $pack_ty, $struct_name, $pack_ty);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@func_send $name, $pack_ty);
                }
            }
        };
        ($struct_name: ident, $name: ident, $c_ty: ident, $ty: ident, $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!($name, $c_ty, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                set!($name, $c_ty, $value_ty);
                // reset!($name);
                set_default!($name, $c_ty, $value_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }

                get!(@pack $name, $ty, $struct_name, $c_ty);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $c_ty);
                }
            }
        };
        ($struct_name: ident, $name: ident, $c_ty: ident, $feild: ident, $ty: ident, $value_ty: ty) => {
            impl AttrSet for $struct_name {
                set!($name, $c_ty, $feild, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                
                // reset!($name, $feild);
                set_default!($name, $c_ty, $feild, $value_ty);
                fn to_attr(ptr: *const u8) -> Attribute
            where
                Self: Sized
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@feild $name, $ty, $struct_name, $c_ty, $feild);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $c_ty, $feild);
                }
            }
        };
        ($struct_name: ident, $name: ident, $c_ty: ident, $feild1: ident, $feild2: ident, $ty: ident, $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!($name, $c_ty, $feild1, $feild2, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                set_default!($name, $c_ty, $feild1, $feild2, $value_ty);
                fn to_attr(ptr: *const u8) -> Attribute
            where
                Self: Sized
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@feild2 $name, $ty, $struct_name, $c_ty, $feild1, $feild2);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!($name, $c_ty, $feild1, $feild2);
                }
            }
        };
        (@func $struct_name: ident, $name: ident, $c_ty: ident, $set_func: ident, $get_func: ident, $ty: ident, $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!(@func $name, $c_ty, $set_func, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                

                
                // reset!(@func $name, $set_func, $get_func);
                set_default!(@func $name, $c_ty, $set_func, $value_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }

                get!(@fun $name, $ty, $struct_name, $c_ty, $get_func);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@func $name, $c_ty, $set_func, $get_func);
                }
            }
        };
        // 方法设置，并且不实现set_default和reset
        (@func $struct_name: ident,  $name: ident, $c_ty: ty, $set_func: ident, $ty: ident, $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!(@func $name, $c_ty, $set_func, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                
                // reset!(@empty);
                set_default!(@empty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@fun $name, $ty, $struct_name, $c_ty, $get_func);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@empty);
                }
            }
        };

        (@func1 $struct_name: ident,  $name: ident, $c_ty: ty, $set_func: ident, $ty: ident, $attr_ty: ident,  $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!(@func $name, $c_ty, $set_func, $value_ty);
            }
            impl ConvertToComponent for $struct_name {
                

                // reset!(@empty);
                set_default!(@empty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$attr_ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }
                get!(@empty);
            }

            $crate::paste::item! {

                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@empty);
                }
            }
        };

        (@box_model_single $struct_name: ident, $name: ident, $c_ty: ident, $feild: ident, $ty: ident, $value_ty: ident) => {
            impl AttrSet for $struct_name {
                set!($name, $c_ty, $feild, $value_ty);
            }

            impl ConvertToComponent for $struct_name {
                
                // reset!(@box_model_single $name, $feild, $ty_all);
                set_default!($name, $c_ty, $feild, $value_ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$value_ty>())))
                }

                get!(@feild $name, $ty, $struct_name, $c_ty, $feild);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@box_model_single $name, $c_ty, $feild);
                }
            }
        };
        (@box_model $struct_name: ident, $name: ident, $ty: ident) => {
            impl AttrSet for $struct_name {
                set!(@box_model $name, $ty);
            }

            impl ConvertToComponent for $struct_name {
                
                // reset!(@box_model $name, $ty);
                set_default!(@box_model $name, $ty);
                fn to_attr(ptr: *const u8) -> Attribute
                {
                    Attribute::$ty($struct_name(clone_unaligned(ptr.cast::<$ty>())))
                }

                get!(@empty);
            }

            $crate::paste::item! {
                impl AttrSet for [<Reset $struct_name>] {
                    reset!(@box_model $name, $ty);
                }
            }
        };
    }

    impl_style!(EmptyType);

    impl_style!(FontStyleType, text_style, TextStyle, font_style, FontStyle, FontStyle);

    impl_style!(FontWeightType, text_style, TextStyle, font_weight, FontWeight, usize);
    impl_style!(FontSizeType, text_style, TextStyle, font_size, FontSize, FontSize);
    impl_style!(FontFamilyType, text_style, TextStyle, font_family, FontFamily, Atom);
    impl_style!(LetterSpacingType, text_style, TextStyle, letter_spacing, LetterSpacing, f32);
    impl_style!(WordSpacingType, text_style, TextStyle, word_spacing, WordSpacing, f32);
    impl_style!(LineHeightType, text_style, TextStyle, line_height, LineHeight, LineHeight);
    impl_style!(TextIndentType, text_style, TextStyle, text_indent, TextIndent, f32);
    impl_style!(WhiteSpaceType, text_style, TextStyle, white_space, WhiteSpace, WhiteSpace);
	impl_style!(TextOverflowType, text_overflow, TextOverflowData, text_overflow, TextOverflow, TextOverflow);
    // impl ConvertToComponent for WhiteSpaceType {
    // 	// 设置white_space,需要同时设置flex_wrap
    // 	fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {
    // 		// 取不到说明实体已经销毁
    // 		let white_space = query.style.default.text_style.white_space.clone();
    // 		// let flex_wrap = query.style.default.text_style.flex_container.flex_wrap.clone();

    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let v = unsafe { ptr.cast::<WhiteSpace>().read_unaligned() };

    // 			cur_style_mark.set(Self::get_type() as usize, true);
    // 			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

    // 			text_style_item.white_space = white_space;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.flex_wrap = if v.allow_wrap() {
    // 				FlexWrap::Wrap
    // 			} else {
    // 				FlexWrap::NoWrap
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}


    // 	}

    // 	set_default!(text_style, white_space, WhiteSpace);
    // 	fn to_attr(ptr: *const u8) -> Attribute{
    // 		Attribute::WhiteSpace(unsafe { WhiteSpaceType(ptr.cast::<WhiteSpace>().read_unaligned()) })
    // 	}
    // }

    // impl ConvertToComponent for ResetWhiteSpaceType {
    // 	fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {

    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let white_space = query.style.default.text_style.white_space.clone();
    // 			text_style_item.white_space = white_space;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.flex_wrap = if white_space.allow_wrap() {
    // 				FlexWrap::Wrap
    // 			} else {
    // 				FlexWrap::NoWrap
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}
    // 	}

    // 	set_default!(text_style, white_space, WhiteSpace);
    // 	fn to_attr(_ptr: *const u8) -> Attribute{
    // 		todo!()
    // 		// Attribute::WhiteSpace(unsafe { WhiteSpaceType(ptr.cast::<WhiteSpace>().read_unaligned()) })
    // 	}
    // }
    impl_style!(TextAlignType, text_style, TextStyle, text_align, TextAlign, TextAlign);

	impl AttrSet for TextContentType {
		// 设置text_align,需要同时设置justify_content
        fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<TextContent1>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);
            set_style_attr(
                &mut query.world,
                query.style.text_content,
                entity,
				// query.style.dirty_list,
                v,
                |mut item: Mut<TextContent>, v| {
                    item.0 = v;
                },
            );
            // 发送事件
            // if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.text_content) {
            //     unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<TextContent>>>>() }
            //         .send(ComponentEvent::<Changed<TextContent>>::new(entity));
            // };


            // 插入默认的FlexContainer组件 TODO
            // if query.world.get_component_by_index_mut(entity, query.style.flex_container).is_err() {
            //     // let default_value = query.world.get_resource_by_id(query.style.default.flex_container).unwrap();
            //     // let r = unsafe { default_value.deref::<DefaultComponent<FlexContainer>>() }.0.clone();
            //     // let query.style.flex_container
            //     // if let Some(default_value) = query.style.flex_container.get_single_res::<FlexContainer>() {
            //         // query.world.entity_mut(entity).insert(default_value.clone());
            //         let _ = query.style.flex_container.alter(entity, (query.default_value.flex_container.clone(),));
            //     // }
                
            // };
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.extend_from_slice(&[
                (ids.text_content, true),
                (ids.flex_container, true)
            ]);
        }
	}

    // impl_style!(@pack_send TextContentType, text_content, TextContent, TextContent1);
    impl ConvertToComponent for TextContentType {
        

        set_default!(text_content, TextContent);
        fn to_attr(ptr: *const u8) -> Attribute { 
			let r = Attribute::TextContent(TextContentType(clone_unaligned(ptr.cast::<TextContent1>())));
			r
		}
		get!(@pack text_content, TextContent, TextContentType, TextContent);
    }

	impl AttrSet for ResetTextContentType {
		reset!(text_content, TextContent);
	}


    // impl ConvertToComponent for TextAlignType {
    // 	// 设置text_align,需要同时设置justify_content
    // 	fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {
    // 		// 取不到说明实体已经销毁
    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let v = unsafe { ptr.cast::<TextAlign>().read_unaligned() };

    // 			cur_style_mark.set(Self::get_type() as usize, true);
    // 			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

    // 			text_style_item.text_align = v;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.justify_content = match v {
    // 				TextAlign::Center => JustifyContent::Center,
    // 				TextAlign::Right => JustifyContent::FlexEnd,
    // 				TextAlign::Left => JustifyContent::FlexStart,
    // 				TextAlign::Justify => JustifyContent::SpaceBetween,
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}


    // 	}

    // 	set_default!(text_style, text_align, TextAlign);
    // 	fn to_attr(ptr: *const u8) -> Attribute{
    // 		Attribute::TextAlign(unsafe { TextAlignType(ptr.cast::<TextAlign>().read_unaligned()) })
    // 	}
    // }

    // impl ConvertToComponent for ResetTextAlignType {
    // 	fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {
    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let v = query.style.default.text_style.text_align.clone();
    // 			text_style_item.text_align = v;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.justify_content = match v {
    // 				TextAlign::Center => JustifyContent::Center,
    // 				TextAlign::Right => JustifyContent::FlexEnd,
    // 				TextAlign::Left => JustifyContent::FlexStart,
    // 				TextAlign::Justify => JustifyContent::SpaceBetween,
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}

    // 	}

    // 	set_default!(text_style, text_align, TextAlign);
    // 	fn to_attr(_ptr: *const u8) -> Attribute{
    // 		todo!()
    // 	}
    // }

    impl_style!(VerticalAlignType, text_style, TextStyle, vertical_align, VerticalAlign, VerticalAlign);
    // impl ConvertToComponent for VerticalAlignType {
    // 	// 设置vertical_align,需要同时设置jalign_items, align_content
    // 	fn set(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {
    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let v = unsafe { ptr.cast::<VerticalAlign>().read_unaligned() };

    // 			cur_style_mark.set(Self::get_type() as usize, true);
    // 			cur_style_mark.set(JustifyContentType::get_type() as usize, true);

    // 			text_style_item.vertical_align = v;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.align_content = match v {
    // 				VerticalAlign::Middle => AlignContent::Center,
    // 				VerticalAlign::Bottom => AlignContent::FlexEnd,
    // 				VerticalAlign::Top => AlignContent::FlexStart,
    // 			};
    // 			flex_container_item.align_items = match v {
    // 				VerticalAlign::Middle => AlignItems::Center,
    // 				VerticalAlign::Bottom => AlignItems::FlexEnd,
    // 				VerticalAlign::Top => AlignItems::FlexStart,
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}
    // 	}

    // 	set_default!(text_style, vertical_align, VerticalAlign);
    // 	fn to_attr(ptr: *const u8) -> Attribute{
    // 		Attribute::VerticalAlign(unsafe { VerticalAlignType(ptr.cast::<VerticalAlign>().read_unaligned()) })
    // 	}
    // }

    // impl ConvertToComponent for ResetVerticalAlignType {
    // 	fn set(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity)
    // 		where
    // 			Self: Sized {
    // 		if let (Ok(mut text_style_item), Ok(mut flex_container_item)) = (query.style.text_style.get_mut(entity), query.style.flex_container.get_mut(entity)) {
    // 			let v = query.style.default.text_style.vertical_align.clone();
    // 			text_style_item.vertical_align = v;
    // 			// text_style_item.notify_modify();

    // 			flex_container_item.align_content = match v {
    // 				VerticalAlign::Middle => AlignContent::Center,
    // 				VerticalAlign::Bottom => AlignContent::FlexEnd,
    // 				VerticalAlign::Top => AlignContent::FlexStart,
    // 			};
    // 			flex_container_item.align_items = match v {
    // 				VerticalAlign::Middle => AlignItems::Center,
    // 				VerticalAlign::Bottom => AlignItems::FlexEnd,
    // 				VerticalAlign::Top => AlignItems::FlexStart,
    // 			};
    // 			// flex_container_item.notify_modify();
    // 		}
    // 	}

    // 	set_default!(text_style, vertical_align, VerticalAlign);
    // 	fn to_attr(_ptr: *const u8) -> Attribute{
    // 		todo!()
    // 	}
    // }

    impl_style!(ColorType, text_style, TextStyle, color, Color, Color);
    impl_style!(TextStrokeType, text_style, TextStyle, text_stroke, TextStroke, Stroke);
    impl_style!(@pack_send TextShadowType, text_shadow, TextShadow, TextShadowList);
    impl_style!(@pack TextOuterGlowType, text_outer_glow, TextOuterGlow, OuterGlow);

	impl AttrSet for BackgroundImageType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<Atom>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<BackgroundImage>(),
                v,
                entity
            );
            match query.world.get_component_by_index_mut::<BackgroundImage>(entity, query.style.background_image) {
                Ok(mut component) => {
                    component.0 = v;
                    // f(unsafe { component.into_inner().deref_mut::<Atom>() }, v);
                }
                _ => {
                    // let _ = query.style.background_image.alter(entity, (BackgroundImage(v), ));
                    // // 顺便插入默认的BackgroundImageTexture， 以免后续修改原型
                    // if query.style.background_image_texture.contains(entity) {
                    //     let _ = query.style.background_image_texture.alter(entity, (BackgroundImageTexture::default(), ));
                    // };
                }
            };
        }
        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.extend_from_slice(&[
                (ids.background_image, true),
                (ids.background_image_texture, true)
            ]);
        }
	}
    impl ConvertToComponent for BackgroundImageType {
        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle)
        where
            Self: Sized,
        {
            if let Some((r, _)) = world.index_single_res_mut::<BackgroundImage>(query.background_image as usize) {
                **r = unsafe { buffer.as_ptr().add(offset).cast::<Atom>().read_unaligned() };
            }
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::BackgroundImage(BackgroundImageType(clone_unaligned(ptr.cast::<Atom>())))
        }

		get!(@pack background_image, BackgroundImage, BackgroundImageType, BackgroundImage);
    }

	impl AttrSet for ResetBackgroundImageType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            reset_style_attr(
                &mut query.world,
                    query.style.background_image,
                entity,
                query.default_value.background_image,
				// query.style.dirty_list,
                |item: &mut BackgroundImage, v: &BackgroundImage| {
                    *item = v.clone();
                },
            );
            // 设置纹理， TODO
        }
        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.extend_from_slice(&[
                (ids.background_image, false),
                (ids.background_image_texture, false)
            ]);
        }
	}

    impl_style!(@pack BackgroundImageClipType, background_image_clip, BackgroundImageClip, NotNanRect);
    impl_style!(ObjectFitType, background_image_mod, BackgroundImageMod, object_fit, ObjectFit, FitType);
    impl_style!(
        BackgroundRepeatType,
        background_image_mod,
        BackgroundImageMod,
        repeat,
        BackgroundRepeat,
        ImageRepeat
    );

	impl AttrSet for BorderImageType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<Atom>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<BorderImage>(),
                v,
                entity
            );
            match query.world.get_component_by_index_mut::<BorderImage>(entity, query.style.border_image){
                Ok(mut component) => {
                    component.0 = v;
                    // f(unsafe { component.into_inner().deref_mut::<Atom>() }, v);
                }
                _ => {
                    // let _ = query.style.border_image.alter(entity, (BorderImage(v), ));
                    // // 顺便插入默认的BorderImageTexture， 以免后续修改原型
                    // if !query.style.border_image_texture.contains(entity) {
                    //     let _ = query.style.border_image_texture.alter(entity, (BorderImageTexture::default(), ));
                    // }
                }
            };
        }
        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.extend_from_slice(&[
                (ids.border_image, true),
                (ids.border_image_texture, true)
            ]);
        }

	}
    impl ConvertToComponent for BorderImageType {
        
        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle)
        where
            Self: Sized,
        {
            if let Some((r, _)) = world.index_single_res_mut::<BorderImage>(query.border_image as usize) {
                **r = unsafe { buffer.as_ptr().add(offset).cast::<Atom>().read_unaligned() };
            }
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::BorderImage(BorderImageType(clone_unaligned(ptr.cast::<Atom>())))
        }

		get!(@pack border_image, BorderImage, BorderImageType, BorderImage);
    }

	impl AttrSet for ResetBorderImageType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            reset_style_attr(
                &mut query.world,
                    query.style.border_image,
                entity,
                query.default_value.border_image,
				// query.style.dirty_list,
                |item: &mut BorderImage, v: &BorderImage| {
                    *item = v.clone();
                },
            );
            // 设置纹理， TODO
        }
        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.extend_from_slice(&[
                (ids.border_image, false),
                (ids.border_image_texture, false)
            ]);
        }
	}
    // impl_style!(@func1 TransformFuncType, transform, Transform, add_func, TransformFunc, TransformFunc, TransformFunc);

	impl AttrSet for TransformType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<TransformFuncs>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );

			if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
                if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.transform = v;
                        return
                    }
                }
            };
			
            // 不存在transform_willChange， 则设置在Transfrom上
			match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    component.all_transform.transform = v;
				}
				_ => {
					// let _ = query.style.transform.alter(entity, (Transform {
					// 	all_transform: AllTransform {
					// 		transform: v,
					// 		..Default::default()
					// 	},
					// 	..Default::default()
					// },));
				}
			}
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform, true));
        }

	}
    impl ConvertToComponent for TransformType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::Transform(TransformType(clone_unaligned(ptr.cast::<TransformFuncs>())))
        }

		get!(@feild2 transform, Transform, TransformType, Transform, all_transform, transform);
    }

	impl AttrSet for ResetTransformType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
                if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.transform = Default::default();
                        return
                    }
                }
            };

            match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
                    component.all_transform.transform = Default::default();
				}
				_ => (),
			}
        }


        fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
            
        }
	}


	impl AttrSet for TranslateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<[LengthUnit; 2]>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );
            if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.translate = Some(v);
                        return
                    }
                }
            };

            // 不存在transform_willChange， 则设置在Transfrom上
			match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform){
				Ok(mut component)  => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    component.all_transform.translate = Some(v);
				}
				_ => {
					// let _ = query.style.transform.alter(entity, (Transform {
					// 	all_transform: AllTransform {
					// 		translate: Some(v),
					// 		..Default::default()
					// 	},
					// 	..Default::default()
					// },));
				}
			}
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform, true));
        }
	}
    impl ConvertToComponent for TranslateType {
        

        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::Translate(TranslateType(clone_unaligned(ptr.cast::<[LengthUnit; 2]>())))
        }
		fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
			match world.get_component_by_index::<Transform>(entity, query.transform) {
				Ok(component)  => match component.all_transform.translate {
					Some(r) => Some(Attribute::Translate(TranslateType(r))),
					None => None,
				},
				_ => None
			}
		}
    }

	impl AttrSet for ResetTranslateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.translate = None;
                        return
                    }
                }
            };

            match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					component.all_transform.translate = None;
				}
				_ => (),
			}
        }
        fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
            
        }
	}

	impl AttrSet for ScaleType {
		 /// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<[f32; 2]>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );

            if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.scale = Some(v);
                        return
                    }
                }
            };

            // 不存在transform_willChange， 则设置在Transfrom上
			match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    component.all_transform.scale = Some(v);
				}
				_ => {
					// let _ = query.style.transform.alter(entity, (Transform {
					// 	all_transform: AllTransform {
					// 		scale: Some(v),
					// 		..Default::default()
					// 	},
					// 	..Default::default()
					// },));
				}
			}
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform, true));
        }
	}
    impl ConvertToComponent for ScaleType {
       

        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::Scale(ScaleType(clone_unaligned(ptr.cast::<[f32; 2]>())))
        }

		fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
			match world.get_component_by_index::<Transform>(entity, query.transform) {
				Ok(component)  => match component.all_transform.scale {
					Some(r) => Some(Attribute::Scale(ScaleType(r))),
					None => None,
				},
				_ => None
			}
		}
    }

	impl AttrSet for ResetScaleType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.scale = None;
                        return
                    }
                }
            };

            match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					component.all_transform.scale = None;
				}
				_ => (),
			}
        }

        fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
            
        }
	}

	impl AttrSet for RotateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<f32>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );
            if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.rotate = Some(v);
                        return
                    }
                }
            };

            // 不存在transform_willChange， 则设置在Transfrom上
			match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    component.all_transform.rotate = Some(v);
				}
				_ => {
					// let _ = query.style.transform.alter(entity, (Transform {
					// 	all_transform: AllTransform {
					// 		rotate: Some(v),
					// 		..Default::default()
					// 	},
					// 	..Default::default()
					// },));
				}
			}
        }

        fn push_component_ops(ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>) {
            arr.push((ids.transform, true));
        }
	}
    impl ConvertToComponent for RotateType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized,
        {
            Attribute::Rotate(unsafe { RotateType(ptr.cast::<f32>().read_unaligned()) })
        }

		fn get(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute> {
			match world.get_component_by_index::<Transform>(entity, query.transform) {
				Ok(component)  => match component.all_transform.rotate {
					Some(r) => Some(Attribute::Rotate(RotateType(r))),
					None => None,
				},
				_ => None
			}
		}
    }

	impl AttrSet for ResetRotateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
				if component.0.is_some() {
                    // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                    if let Some(r) = &mut component.0 {
                        r.rotate = None;
                        return
                    }
                }
            };

            match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
				Ok(mut component)  => {
					component.all_transform.rotate = None;
				}
				_ => (),
			}
        }

        fn push_component_ops(_ids: &SettingComponentIds, _arr: &mut Vec<(ComponentIndex, bool)>) {
            
        }

	}

	impl AttrSet for TransformWillChangeType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized,
        {
            let v = ptr.cast::<bool>();
            let v = if is_clone {
                clone_unaligned(v)
            } else {
                unsafe { v.read_unaligned() }
            };
            cur_style_mark.set(Self::get_type() as usize, true);

            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<TransformWillChange>(),
                v,
                entity
            );

			if !v {
				if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
					if let Some(c) = &component.0 {
                        let c1 = c.clone();
                        component.0 = None;
                        match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
                            Ok(mut component)  => {
                                // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                                component.all_transform = c1;
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
				match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
					Ok(mut component)  => {
                        let c = component.all_transform.clone();
                        if let Ok(mut component_will_change) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
                            *component_will_change = TransformWillChange(Some(c));
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
    impl ConvertToComponent for TransformWillChangeType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _world: &mut World, _query: &DefaultStyle)
        where
            Self: Sized,
        {
        }

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
		fn set<'w, 's>(_cur_style_mark: &mut StyleMarkType, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            log::debug!("reset_style_attr, type: TransformWillChange, entity: {:?}", entity);
            if let Ok(mut component) = query.world.get_component_by_index_mut::<TransformWillChange>(entity, query.style.transform_will_change) {
                // 删除TransformWillChange, 设置Transform
                if let Some(c) = &component.0 {
                    let c = c.clone();
                    component.0 = None;
                    // 设置transform
                    match query.world.get_component_by_index_mut::<Transform>(entity, query.style.transform) {
                        Ok(mut component)  => {
                            // 如果存在transform_willChange,则将Transform设置在TransformWillChange上
                            component.all_transform = c;
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

    // impl_style!(@pack TransformWillChangeType, transform_will_change, TransformWillChange, TransformFuncs);

    // impl_style!(TransformType, transform, Transform, funcs, Transform, TransformFuncs);
    impl_style!(@pack BorderImageClipType, border_image_clip, BorderImageClip, NotNanRect);
    impl_style!(@pack BorderImageSliceType, border_image_slice, BorderImageSlice, BorderImageSlice1);
    impl_style!(@pack BorderImageRepeatType, border_image_repeat, BorderImageRepeat, ImageRepeat);

    impl_style!(@pack_send BorderColorType, border_color, BorderColor, CgColor);

    impl_style!(@pack_send BackgroundColorType, background_color, BackgroundColor, Color);

    impl_style!(@pack_send BoxShadowType, box_shadow, BoxShadow, BoxShadow1);

    impl_style!(@pack_compare OpacityType, opacity, Opacity, f32);
    impl_style!(@pack BorderRadiusType, border_radius, BorderRadius, BorderRadius1);
    impl_style!(@pack HsiType, hsi, Hsi, Hsi1);
    impl_style!(@pack_compare BlurType, blur, Blur, f32);
    impl_style!(TransformOriginType, transform, Transform, origin, TransformOrigin, TransformOrigin);
    impl_style!(DirectionType, flex_container, FlexContainer, direction, Direction, Direction);
    impl_style!(AspectRatioType, flex_normal, FlexNormal, aspect_ratio, AspectRatio, Number);
    impl_style!(OrderType, flex_normal, FlexNormal, order, Order, isize);
    impl_style!(FlexBasisType, flex_normal, FlexNormal, flex_basis, FlexBasis, Dimension);


    impl_style!(@func DisplayType, show, Show, set_display, get_display, Display, Display);
    impl_style!(@func VisibilityType, show, Show, set_visibility, get_visibility, Visibility, bool);
    impl_style!(@func EnableType, show, Show, set_enable, get_enable, Enable, Enable);

    impl_style!(@func1 VNodeType, node_state, NodeState, set_vnode, NodeState, VNode, bool);
    // impl_style!(@func VNodeType, node_state, set_vnode, NodeState, bool);

    impl_style!(@pack_compare ZIndexType, z_index, ZIndex, isize);
    impl_style!(@pack_compare OverflowType, overflow, Overflow, bool);

    impl_style!(@pack MaskImageType, mask_image, MaskImage, MaskImage1);
    impl_style!(@pack MaskImageClipType, mask_image_clip, MaskImageClip, NotNanRect);
    impl_style!(@pack ClipPathType, clip_path, ClipPath, BaseShape);

	impl_style!(AsImageType, as_image, AsImage, level, AsImage, AsImage1);
	// impl_style!(AsImageType, as_image, AsImage, level, AsImage, AsImage1);

    impl_style!(WidthType, size, Size, width, Width, Dimension);
    impl_style!(HeightType, size, Size, height, Height, Dimension);

    impl_style!(@box_model_single MarginTopType, margin, Margin, top, MarginTop, Dimension);
    impl_style!(@box_model_single MarginRightType, margin, Margin, right, MarginRight, Dimension);
    impl_style!(@box_model_single MarginBottomType, margin, Margin, bottom, MarginBottom, Dimension);
    impl_style!(@box_model_single MarginLeftType, margin, Margin, left, MarginLeft, Dimension);

    impl_style!(@box_model_single PaddingTopType, padding, Padding, top, PaddingTop, Dimension);
    impl_style!(@box_model_single PaddingRightType, padding, Padding, right, PaddingRight, Dimension);
    impl_style!(@box_model_single PaddingBottomType, padding, Padding, bottom, PaddingBottom, Dimension);
    impl_style!(@box_model_single PaddingLeftType, padding, Padding, left, PaddingLeft, Dimension);

    impl_style!(@box_model_single BorderTopType, border, Border, top, BorderTop, Dimension);
    impl_style!(@box_model_single BorderRightType, border, Border, right, BorderRight, Dimension);
    impl_style!(@box_model_single BorderBottomType, border, Border, bottom, BorderBottom, Dimension);
    impl_style!(@box_model_single BorderLeftType, border, Border, left, BorderLeft, Dimension);

    impl_style!(@box_model_single PositionTopType, position, Position, top, PositionTop, Dimension);
    impl_style!(@box_model_single PositionRightType, position, Position, right, PositionRight, Dimension);
    impl_style!(@box_model_single PositionBottomType, position, Position, bottom, PositionBottom, Dimension);
    impl_style!(@box_model_single PositionLeftType, position, Position, left, PositionLeft, Dimension);

    impl_style!(MinWidthType, min_max, MinMax, min, width, MinWidth, Dimension);
    impl_style!(MinHeightType, min_max, MinMax, min, height, MinHeight, Dimension);
    impl_style!(MaxHeightType, min_max, MinMax, max, height, MaxHeight, Dimension);
    impl_style!(MaxWidthType, min_max, MinMax, max, width, MaxWidth, Dimension);
    impl_style!(
        JustifyContentType,
        flex_container,
        FlexContainer,
        justify_content,
        JustifyContent,
        JustifyContent
    );
    impl_style!(
        FlexDirectionType,
        flex_container,
        FlexContainer,
        flex_direction,
        FlexDirection,
        FlexDirection
    );
    impl_style!(AlignContentType, flex_container, FlexContainer, align_content, AlignContent, AlignContent);
    impl_style!(AlignItemsType, flex_container, FlexContainer, align_items, AlignItems, AlignItems);
    impl_style!(FlexWrapType, flex_container, FlexContainer, flex_wrap, FlexWrap, FlexWrap);
	impl_style!(OverflowWrapType, flex_container, FlexContainer, overflow_wrap, OverflowWrap, OverflowWrap);

    impl_style!(FlexShrinkType, flex_normal, FlexNormal, flex_shrink, FlexShrink, f32);
    impl_style!(FlexGrowType, flex_normal, FlexNormal, flex_grow, FlexGrow, f32);
    impl_style!(PositionTypeType, flex_normal, FlexNormal, position_type, PositionType, PositionType1);
    impl_style!(AlignSelfType, flex_normal, FlexNormal, align_self, AlignSelf, AlignSelf);

    impl_style!(@pack BlendModeType, blend_mode, BlendMode, BlendMode1);
    impl_style!(AnimationNameType, animation, Animation, name, AnimationName, AnimationName);
    impl_style!(
        AnimationDurationType,
        animation,
        Animation,
        duration,
        AnimationDuration,
        SmallVec<[Time; 1]>
    );
    impl_style!(
        AnimationTimingFunctionType,
        animation,
        Animation,
        timing_function,
        AnimationTimingFunction,
        SmallVec<[AnimationTimingFunction; 1]>
    );
    impl_style!(AnimationDelayType, animation, Animation, delay, AnimationDelay, SmallVec<[Time; 1]>);
    impl_style!(
        AnimationIterationCountType,
        animation,
        Animation,
        iteration_count,
        AnimationIterationCount,
        SmallVec<[IterationCount; 1]>
    );
    impl_style!(
        AnimationDirectionType,
        animation,
        Animation,
        direction,
        AnimationDirection,
        SmallVec<[AnimationDirection; 1]>
    );
    impl_style!(
        AnimationFillModeType,
        animation,
        Animation,
        fill_mode,
        AnimationFillMode,
        SmallVec<[AnimationFillMode; 1]>
    );
    impl_style!(
        AnimationPlayStateType,
        animation,
        Animation,
        play_state,
        AnimationPlayState,
        SmallVec<[AnimationPlayState; 1]>
    );

	// transition
	impl_style!(TransitionPropertyType, transition, Transition, property, TransitionProperty, SmallVec<[usize; 1]>);
    impl_style!(
        TransitionDurationType,
        transition,
        Transition,
        duration,
        TransitionDuration,
        SmallVec<[Time; 1]>
    );
    impl_style!(
        TransitionTimingFunctionType,
        transition,
        Transition,
        timing_function,
        TransitionTimingFunction,
        SmallVec<[AnimationTimingFunction; 1]>
    );
    impl_style!(TransitionDelayType, transition, Transition, delay, TransitionDelay, SmallVec<[Time; 1]>);


    pub struct StyleFunc {
        get_type: fn() -> StyleType,
		get: fn(world: &World, query: &SettingComponentIds, entity: Entity) -> Option<Attribute>,
        // get_style_index: fn() -> u8,
        size: fn() -> usize,
        // /// 安全： entity必须存在
        // fn set(&self, cur_style_mark: &mut StyleMarkType, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity);
        /// 安全： entity必须存在
        set: fn(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),

        /// 设置默认值
        set_default: fn(buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle),
        to_attr: fn(ptr: *const u8) -> Attribute,
        push_component_ops: fn (ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>),
    }

    impl StyleFunc {
        fn new<T: ConvertToComponent>() -> StyleFunc {
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
		set: fn(cur_style_mark: &mut StyleMarkType, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),
        push_component_ops: fn (ids: &SettingComponentIds, arr: &mut Vec<(ComponentIndex, bool)>),
	}

	impl ResetStyleFunc {
        fn new<T: AttrSet>() -> ResetStyleFunc {
            ResetStyleFunc {
                set: T::set,
                push_component_ops: T::push_component_ops,
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
		];

		
		static ref RESET_STYLE_ATTR: [ResetStyleFunc; 97] = [
        /******************************* reset ******************************************************/
            ResetStyleFunc::new::<ResetBackgroundRepeatType>(), // 0
            ResetStyleFunc::new::<ResetFontStyleType>(), // 1
            ResetStyleFunc::new::<ResetFontWeightType>(), // 2
            ResetStyleFunc::new::<ResetFontSizeType>(), // 3
            ResetStyleFunc::new::<FontFamilyType>(), // 4
            ResetStyleFunc::new::<LetterSpacingType>(), // 5
            ResetStyleFunc::new::<WordSpacingType>(), // 6
            ResetStyleFunc::new::<ResetLineHeightType>(), // 7
            ResetStyleFunc::new::<TextIndentType>(), // 8
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
            ResetStyleFunc::new::<FlexShrinkType>(), // 69
            ResetStyleFunc::new::<FlexGrowType>(), // 70
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

            ResetStyleFunc::new::<TextOuterGlowType>(), // 96

        ];
    }

    pub struct Setting<'w> {
        // pub style: &'s mut StyleQuery<'w>,
        pub default_value: &'w DefaultStyle,
        pub style: &'w SettingComponentIds,
        pub world: &'w mut World,
    }

    impl<'w> Setting<'w> {
        // #[inline]
        // pub fn style_mut(&mut self) -> &mut StyleQuery<'w, 's> {
        // 	&mut self.style
        // }

        // #[inline]
        // pub fn world_mut(&mut self) -> &mut World {
        // 	&mut self.world
        // }

        // pub fn new<'ww, 'ss, 'ss1>(style: &'ss mut StyleQuery<'ww>, default_value: &'ss mut DefaultStyle<'ss1>) -> Setting<'ww, 'ss, 'ss1> { Setting { style, default_value } }
    }

    // impl FromWorld for StyleQuery {
    //     fn from_world(world: &mut World) -> Self {
    //         Self {
    //             size: world.init_component::<Size>(),
    //             margin: world.init_component::<Margin>(),
    //             padding: world.init_component::<Padding>(),
    //             border: world.init_component::<Border>(),
    //             position: world.init_component::<Position>(),
    //             min_max: world.init_component::<MinMax>(),
    //             flex_container: world.init_component::<FlexContainer>(),
    //             flex_normal: world.init_component::<FlexNormal>(),
    //             z_index: world.init_component::<ZIndex>(),
    //             overflow: world.init_component::<Overflow>(),
    //             opacity: world.init_component::<Opacity>(),
    //             blend_mode: world.init_component::<BlendMode>(),
    //             show: world.init_component::<Show>(),
    //             transform: world.init_component::<Transform>(),
    //             background_color: world.init_component::<BackgroundColor>(),
    //             border_color: world.init_component::<BorderColor>(),
    //             background_image: world.init_component::<BackgroundImage>(),
    //             background_image_texture: world.init_component::<BackgroundImageTexture>(),
    //             background_image_clip: world.init_component::<BackgroundImageClip>(),
    //             mask_image: world.init_component::<MaskImage>(),
    //             mask_image_clip: world.init_component::<MaskImageClip>(),
    //             hsi: world.init_component::<Hsi>(),
    //             blur: world.init_component::<Blur>(),
    //             clip_path: world.init_component::<ClipPath>(),
    //             background_image_mod: world.init_component::<BackgroundImageMod>(),
    //             border_image: world.init_component::<BorderImage>(),
    //             border_image_texture: world.init_component::<BorderImageTexture>(),
    //             border_image_clip: world.init_component::<BorderImageClip>(),
    //             border_image_slice: world.init_component::<BorderImageSlice>(),
    //             border_image_repeat: world.init_component::<BorderImageRepeat>(),
    //             border_radius: world.init_component::<BorderRadius>(),
    //             box_shadow: world.init_component::<BoxShadow>(),
    //             text_style: world.init_component::<TextStyle>(),
    //             text_shadow: world.init_component::<TextShadow>(),
    //             transform_will_change: world.init_component::<TransformWillChange>(),
    //             text_content: world.init_component::<TextContent>(),
    //             node_state: world.init_component::<NodeState>(),
    //             animation: world.init_component::<Animation>(),
	// 			transition: world.init_component::<Transition>(),
    //             style_mark: world.init_component::<StyleMark>(),
    //             class_name: world.init_component::<ClassName>(),
    //             as_image: world.init_component::<AsImage>(),
    //             // default: DefaultStyle::from_world(world),
    //             // event: ChangeEvent::from_world(world),

	// 			// dirty_list: world.components().get_resource_id(std::any::TypeId::of::<DirtyList>()).unwrap(),

	// 			text_overflow: world.init_component::<TextOverflowData>(),
    //             text_outer_glow: world.init_component::<TextOuterGlow>(),
    //         }
    //     }
    // }


    // #[derive(SystemParam, ParamSetElement)]
    // pub struct StyleQuery<'w> {
    //     pub entitys: Query<'w, Entity>,

    //     pub size: Alter<'w, Option<&'static mut Size>, (), (Size, )>,
    //     pub margin: Alter<'w, Option<&'static mut Margin>, (), (Margin, )>,
    //     pub padding: Alter<'w, Option<&'static mut Padding>, (), (Padding, )>,
    //     pub border: Alter<'w, Option<&'static mut Border>, (), (Border, )>,
    //     pub position: Alter<'w, Option<&'static mut Position>, (), (Position, )>,
    //     pub min_max: Alter<'w, Option<&'static mut MinMax>, (), (MinMax, )>,
    //     pub flex_container: Alter<'w, Option<&'static mut FlexContainer>, (), (FlexContainer, )>,
    //     pub flex_normal: Alter<'w, Option<&'static mut FlexNormal>, (), (FlexNormal, )>,
    //     pub z_index: Alter<'w, Option<&'static mut ZIndex>, (), (ZIndex, )>,
    //     pub overflow: Alter<'w, Option<&'static mut Overflow>, (), (Overflow, )>,
    //     pub opacity: Alter<'w, Option<&'static mut Opacity>, (), (Opacity, )>,
    //     pub blend_mode: Alter<'w, Option<&'static mut BlendMode>, (), (BlendMode, )>,
    //     pub show: Alter<'w, Option<&'static mut Show>, (), (Show, )>,
    //     pub transform: Alter<'w, Option<&'static mut Transform>, (), (Transform, )>,
    //     pub background_color: Alter<'w, Option<&'static mut BackgroundColor>, (), (BackgroundColor, )>,
    //     pub border_color: Alter<'w, Option<&'static mut BorderColor>, (), (BorderColor, )>,
    //     pub background_image: Alter<'w, Option<&'static mut BackgroundImage>, (), (BackgroundImage, )>,
    //     pub background_image_texture: Alter<'w, Option<&'static mut BackgroundImageTexture>, (), (BackgroundImageTexture, )>,
    //     pub background_image_clip: Alter<'w, Option<&'static mut BackgroundImageClip>, (), (BackgroundImageClip, )>,
    //     pub mask_image: Alter<'w, Option<&'static mut MaskImage>, (), (MaskImage, )>,
    //     pub mask_image_clip: Alter<'w, Option<&'static mut MaskImageClip>, (), (MaskImageClip, )>,
    //     pub hsi: Alter<'w, Option<&'static mut Hsi>, (), (Hsi, )>,
    //     pub blur: Alter<'w, Option<&'static mut Blur>, (), (Blur, )>,
    //     pub clip_path: Alter<'w, Option<&'static mut ClipPath>, (), (ClipPath, )>,
    //     pub background_image_mod: Alter<'w, Option<&'static mut BackgroundImageMod>, (), (BackgroundImageMod, )>,
    //     pub border_image: Alter<'w, Option<&'static mut BorderImage>, (), (BorderImage, )>,
    //     pub border_image_texture: Alter<'w, Option<&'static mut BorderImageTexture>, (), (BorderImageTexture, )>,
    //     pub border_image_clip: Alter<'w, Option<&'static mut BorderImageClip>, (), (BorderImageClip, )>,
    //     pub border_image_slice: Alter<'w, Option<&'static mut BorderImageSlice>, (), (BorderImageSlice, )>,
    //     pub border_image_repeat: Alter<'w, Option<&'static mut BorderImageRepeat>, (), (BorderImageRepeat, )>,
    //     pub border_radius: Alter<'w, Option<&'static mut BorderRadius>, (), (BorderRadius, )>,
    //     pub box_shadow: Alter<'w, Option<&'static mut BoxShadow>, (), (BoxShadow, )>,
    //     pub text_style: Alter<'w, Option<&'static mut TextStyle>, (), (TextStyle, )>,
    //     pub text_shadow: Alter<'w, Option<&'static mut TextShadow>, (), (TextShadow, )>,
    //     pub text_outer_glow: Alter<'w, Option<&'static mut TextOuterGlow>, (), (TextOuterGlow, )>,
    //     pub transform_will_change: Alter<'w, Option<&'static mut TransformWillChange>, (), (TransformWillChange, )>,
    //     pub text_content: Alter<'w, Option<&'static mut TextContent>, (), (TextContent, )>,
    //     pub node_state: Alter<'w, Option<&'static mut NodeState>, (), (NodeState, )>,
    //     pub animation: Alter<'w, Option<&'static mut Animation>, (), (Animation, )>,
	// 	pub transition: Alter<'w, Option<&'static mut Transition>, (), (Transition, )>,
    //     pub class_name: Alter<'w, Option<&'static mut ClassName>, (), (ClassName, )>,
    //     pub as_image: Alter<'w, Option<&'static mut AsImage>, (), (AsImage, )>,
	// 	pub text_overflow: Alter<'w, Option<&'static mut TextOverflowData>, (), (TextOverflowData, )>,

    //     // pub default: DefaultStyle,

    //     // pub event: ChangeEvent,

	// 	// pub dirty_list: Alter<'w, &'static Size, (), (Size, )>,
    // }

    // // 默认值
    // #[derive(SystemParam)]
    // pub struct DefaultStyle<'w> {
    //     pub size: OrInitSingleResMut<'w, Size>,
    //     pub margin: OrInitSingleResMut<'w, Margin>,
    //     pub padding: OrInitSingleResMut<'w, Padding>,
    //     pub border: OrInitSingleResMut<'w, Border>,
    //     pub position: OrInitSingleResMut<'w, Position>,
    //     pub min_max: OrInitSingleResMut<'w, MinMax>,
    //     pub flex_container: OrInitSingleResMut<'w, FlexContainer>,
    //     pub flex_normal: OrInitSingleResMut<'w, FlexNormal>,
    //     pub z_index: OrInitSingleResMut<'w, ZIndex>,
    //     pub overflow: OrInitSingleResMut<'w, Overflow>,
    //     pub opacity: OrInitSingleResMut<'w, Opacity>,
    //     pub blend_mode: OrInitSingleResMut<'w, BlendMode>,
    //     pub show: OrInitSingleResMut<'w, Show>,
    //     pub transform: OrInitSingleResMut<'w, Transform>,
    //     pub background_color: OrInitSingleResMut<'w, BackgroundColor>,
    //     pub border_color: OrInitSingleResMut<'w, BorderColor>,
    //     pub background_image: OrInitSingleResMut<'w, BackgroundImage>,
    //     pub background_image_texture: OrInitSingleResMut<'w, BackgroundImageTexture>,
    //     pub background_image_clip: OrInitSingleResMut<'w, BackgroundImageClip>,
    //     pub mask_image: OrInitSingleResMut<'w, MaskImage>,
    //     pub mask_image_clip: OrInitSingleResMut<'w, MaskImageClip>,
    //     pub hsi: OrInitSingleResMut<'w, Hsi>,
    //     pub blur: OrInitSingleResMut<'w, Blur>,
    //     pub clip_path: OrInitSingleResMut<'w, ClipPath>,
    //     pub background_image_mod: OrInitSingleResMut<'w, BackgroundImageMod>,
    //     pub border_image: OrInitSingleResMut<'w, BorderImage>,
    //     pub border_image_texture: OrInitSingleResMut<'w, BorderImageTexture>,
    //     pub border_image_clip: OrInitSingleResMut<'w, BorderImageClip>,
    //     pub border_image_slice: OrInitSingleResMut<'w, BorderImageSlice>,
    //     pub border_image_repeat: OrInitSingleResMut<'w, BorderImageRepeat>,
    //     pub border_radius: OrInitSingleResMut<'w, BorderRadius>,
    //     pub box_shadow: OrInitSingleResMut<'w, BoxShadow>,
    //     pub text_style: OrInitSingleResMut<'w, TextStyle>,
    //     pub text_shadow: OrInitSingleResMut<'w, TextShadow>,
    //     pub text_outer_glow: OrInitSingleResMut<'w, TextOuterGlow>,
    //     pub transform_will_change: OrInitSingleResMut<'w, TransformWillChange>,
    //     pub text_content: OrInitSingleResMut<'w, TextContent>,
    //     pub node_state: OrInitSingleResMut<'w, NodeState>,
    //     pub animation: OrInitSingleResMut<'w, Animation>,
	// 	pub transition: OrInitSingleResMut<'w, Transition>,
    //     pub class_name: OrInitSingleResMut<'w, ClassName>,
    //     pub as_image: OrInitSingleResMut<'w, AsImage>,

    //     // pub default: DefaultStyle,

    //     // pub event: ChangeEvent,

	// 	// pub dirty_list: Alter<'w, &'static Size, (), (Size, )>,

	// 	pub text_overflow: OrInitSingleResMut<'w, TextOverflowData>,
    // }

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
            }
        }
    }

    // impl StyleQuery {
    //     const fn get_alert<T: 'static>(&mut self) -> &mut Alter<'static, &'static mut T, (), (T, )> {
    //         match std::any::TypeId::of::<T>() {
    //             std::any::TypeId::of::<Size> => &self.size,
    //             std::any::TypeId::of::<Margin> => &self.margin,
    //             std::any::TypeId::of::<Padding> => &self.padding,
    //             std::any::TypeId::of::<Border> => &self.border,
    //             std::any::TypeId::of::<Position> => &self.position,
    //             std::any::TypeId::of::<MinMax> => &self.min_max,
    //             std::any::TypeId::of::<FlexContainer> => &self.flex_container,
    //             std::any::TypeId::of::<FlexNormal> => &self.flex_normal,
    //             std::any::TypeId::of::<ZIndex> => &self.z_index,
    //             std::any::TypeId::of::<Overflow> => &self.overflow,
    //             std::any::TypeId::of::<Opacity> => &self.opacity,
    //             std::any::TypeId::of::<BlendMode> => &self.blend_mode,
    //             std::any::TypeId::of::<Show> => &self.show,
    //             std::any::TypeId::of::<Transform> => &self.transform,
    //             std::any::TypeId::of::<BackgroundColor> => &self.background_color,
    //             std::any::TypeId::of::<BorderColor> => &self.border_color,
    //             std::any::TypeId::of::<BackgroundImage> => &self.background_image,
    //             std::any::TypeId::of::<BackgroundImageTexture> => &self.background_image_texture,
    //             std::any::TypeId::of::<BackgroundImageClip> => &self.background_image_clip,
    //             std::any::TypeId::of::<MaskImage> => &self.mask_image,
    //             std::any::TypeId::of::<MaskImageClip> => &self.mask_image_clip,
    //             std::any::TypeId::of::<Hsi> => &self.hsi,
    //             std::any::TypeId::of::<Blur> => &self.blur,
    //             std::any::TypeId::of::<ClipPath> => &self.clip_path,
    //             std::any::TypeId::of::<BackgroundImageMod> => &self.background_image_mod,
    //             std::any::TypeId::of::<BorderImage> => &self.border_image,
    //             std::any::TypeId::of::<BorderImageTexture> => &self.border_image_texture,
    //             std::any::TypeId::of::<BorderImageClip> => &self.border_image_clip,
    //             std::any::TypeId::of::<BorderImageSlice> => &self.border_image_slice,
    //             std::any::TypeId::of::<BorderImageRepeat> => &self.border_image_repeat,
    //             std::any::TypeId::of::<BorderRadius> => &self.border_radius,
    //             std::any::TypeId::of::<BoxShadow> => &self.box_shadow,
    //             std::any::TypeId::of::<TextStyle> => &self.text_style,
    //             std::any::TypeId::of::<TextShadow> => &self.text_shadow,
    //             std::any::TypeId::of::<TextOuterGlow> => &self.text_outer_glow,
    //             std::any::TypeId::of::<TransformWillChange> => &self.transform_will_change,
    //             std::any::TypeId::of::<TextContent> => &self.text_content,
    //             std::any::TypeId::of::<NodeState> => &self.node_state,
    //             std::any::TypeId::of::<Animation> => &self.animation,
    //             std::any::TypeId::of::<Transition> => &self.transition,
    //             std::any::TypeId::of::<StyleMark> => &self.style_mark,
    //             std::any::TypeId::of::<ClassName> => &self.class_name,
    //             std::any::TypeId::of::<AsImage> => &self.as_image,

    //             _ => panic!("get_alert fail")
    //         }
    //     }
    // }

    // pub struct StyleQuery {
    //     pub size: ColumnIndex,
    //     pub margin: ColumnIndex,
    //     pub padding: ColumnIndex,
    //     pub border: ColumnIndex,
    //     pub position: ColumnIndex,
    //     pub min_max: ColumnIndex,
    //     pub flex_container: ColumnIndex,
    //     pub flex_normal: ColumnIndex,
    //     pub z_index: ColumnIndex,
    //     pub overflow: ColumnIndex,
    //     pub opacity: ColumnIndex,
    //     pub blend_mode: ColumnIndex,
    //     pub show: ColumnIndex,
    //     pub transform: ColumnIndex,
    //     pub background_color: ColumnIndex,
    //     pub border_color: ColumnIndex,
    //     pub background_image: ColumnIndex,
    //     pub background_image_texture: ColumnIndex,
    //     pub background_image_clip: ColumnIndex,
    //     pub mask_image: ColumnIndex,
    //     pub mask_image_clip: ColumnIndex,
    //     pub hsi: ColumnIndex,
    //     pub blur: ColumnIndex,
    //     pub clip_path: ColumnIndex,
    //     pub background_image_mod: ColumnIndex,
    //     pub border_image: ColumnIndex,
    //     pub border_image_texture: ColumnIndex,
    //     pub border_image_clip: ColumnIndex,
    //     pub border_image_slice: ColumnIndex,
    //     pub border_image_repeat: ColumnIndex,
    //     pub border_radius: ColumnIndex,
    //     pub box_shadow: ColumnIndex,
    //     pub text_style: ColumnIndex,
    //     pub text_shadow: ColumnIndex,
    //     pub text_outer_glow: ColumnIndex,
    //     pub transform_will_change: ColumnIndex,
    //     pub text_content: ColumnIndex,
    //     pub node_state: ColumnIndex,
    //     pub animation: ColumnIndex,
	// 	pub transition: ColumnIndex,
    //     pub style_mark: ColumnIndex,
    //     pub class_name: ColumnIndex,
    //     pub as_image: ColumnIndex,

    //     // pub default: DefaultStyle,

    //     // pub event: ChangeEvent,

	// 	// pub dirty_list: ColumnIndex,

	// 	pub text_overflow: ColumnIndex,
    // }

    // pub struct DefaultStyle {
    //     pub size: ColumnIndex,
    //     pub margin: ColumnIndex,
    //     pub padding: ColumnIndex,
    //     pub border: ColumnIndex,
    //     pub position: ColumnIndex,
    //     pub min_max: ColumnIndex,
    //     pub flex_container: ColumnIndex,
    //     pub flex_normal: ColumnIndex,
    //     pub z_index: ColumnIndex,
    //     pub overflow: ColumnIndex,
    //     pub opacity: ColumnIndex,
    //     pub blend_mode: ColumnIndex,
    //     pub show: ColumnIndex,
    //     pub transform: ColumnIndex,
    //     pub background_color: ColumnIndex,
    //     pub border_color: ColumnIndex,
    //     pub background_image: ColumnIndex,
    //     pub background_image_clip: ColumnIndex,
    //     pub mask_image: ColumnIndex,
    //     pub mask_image_clip: ColumnIndex,
    //     pub hsi: ColumnIndex,
    //     pub blur: ColumnIndex,
    //     pub clip_path: ColumnIndex,
    //     pub background_image_mod: ColumnIndex,
    //     pub border_image: ColumnIndex,
    //     pub border_image_clip: ColumnIndex,
    //     pub border_image_slice: ColumnIndex,
    //     pub border_image_repeat: ColumnIndex,
    //     pub border_radius: ColumnIndex,
    //     pub box_shadow: ColumnIndex,
    //     pub text_style: ColumnIndex,
    //     pub text_shadow: ColumnIndex,
    //     pub text_outer_glow: ColumnIndex,
    //     pub transform_will_change: ColumnIndex,
    //     pub text_content: ColumnIndex,
    //     pub animation: ColumnIndex,
	// 	pub transition: ColumnIndex,
    //     pub node_state: ColumnIndex,
    //     pub as_image: ColumnIndex,
	// 	pub text_overflow: ColumnIndex,
    // }

    // impl FromWorld for DefaultStyle {
    //     fn from_world(world: &mut World) -> Self {
    //         Self {
    //             size: {
    //                 world.init_single_res::<DefaultComponent<Size>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Size>>())
    //                     .unwrap()
    //             },
    //             margin: {
    //                 world.init_single_res::<DefaultComponent<Margin>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Margin>>())
    //                     .unwrap()
    //             },
    //             padding: {
    //                 world.init_single_res::<DefaultComponent<Padding>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Padding>>())
    //                     .unwrap()
    //             },
    //             border: {
    //                 world.init_single_res::<DefaultComponent<Border>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Border>>())
    //                     .unwrap()
    //             },
    //             position: {
    //                 world.init_single_res::<DefaultComponent<Position>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Position>>())
    //                     .unwrap()
    //             },
    //             min_max: {
    //                 world.init_single_res::<DefaultComponent<MinMax>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<MinMax>>())
    //                     .unwrap()
    //             },
    //             flex_container: {
    //                 world.init_single_res::<DefaultComponent<FlexContainer>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<FlexContainer>>())
    //                     .unwrap()
    //             },
    //             flex_normal: {
    //                 world.init_single_res::<DefaultComponent<FlexNormal>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<FlexNormal>>())
    //                     .unwrap()
    //             },
    //             z_index: {
    //                 world.init_single_res::<DefaultComponent<ZIndex>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<ZIndex>>())
    //                     .unwrap()
    //             },
    //             overflow: {
    //                 world.init_single_res::<DefaultComponent<Overflow>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Overflow>>())
    //                     .unwrap()
    //             },
    //             opacity: {
    //                 world.init_single_res::<DefaultComponent<Opacity>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Opacity>>())
    //                     .unwrap()
    //             },
    //             blend_mode: {
    //                 world.init_single_res::<DefaultComponent<BlendMode>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BlendMode>>())
    //                     .unwrap()
    //             },
    //             show: {
    //                 world.init_single_res::<DefaultComponent<Show>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Show>>())
    //                     .unwrap()
    //             },
    //             transform: {
    //                 world.init_single_res::<DefaultComponent<Transform>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Transform>>())
    //                     .unwrap()
    //             },
    //             background_color: {
    //                 world.init_single_res::<DefaultComponent<BackgroundColor>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundColor>>())
    //                     .unwrap()
    //             },
    //             border_color: {
    //                 world.init_single_res::<DefaultComponent<BorderColor>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderColor>>())
    //                     .unwrap()
    //             },
    //             background_image: {
    //                 world.init_single_res::<DefaultComponent<BackgroundImage>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImage>>())
    //                     .unwrap()
    //             },
    //             background_image_clip: {
    //                 world.init_single_res::<DefaultComponent<BackgroundImageClip>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImageClip>>())
    //                     .unwrap()
    //             },
    //             mask_image: {
    //                 world.init_single_res::<DefaultComponent<MaskImage>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<MaskImage>>())
    //                     .unwrap()
    //             },
    //             mask_image_clip: {
    //                 world.init_single_res::<DefaultComponent<MaskImageClip>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<MaskImageClip>>())
    //                     .unwrap()
    //             },
    //             hsi: {
    //                 world.init_single_res::<DefaultComponent<Hsi>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Hsi>>())
    //                     .unwrap()
    //             },
    //             blur: {
    //                 world.init_single_res::<DefaultComponent<Blur>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Blur>>())
    //                     .unwrap()
    //             },
    //             clip_path: {
    //                 world.init_single_res::<DefaultComponent<ClipPath>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<ClipPath>>())
    //                     .unwrap()
    //             },
    //             background_image_mod: {
    //                 world.init_single_res::<DefaultComponent<BackgroundImageMod>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImageMod>>())
    //                     .unwrap()
    //             },
    //             border_image: {
    //                 world.init_single_res::<DefaultComponent<BorderImage>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImage>>())
    //                     .unwrap()
    //             },
    //             border_image_clip: {
    //                 world.init_single_res::<DefaultComponent<BorderImageClip>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageClip>>())
    //                     .unwrap()
    //             },
    //             border_image_slice: {
    //                 world.init_single_res::<DefaultComponent<BorderImageSlice>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageSlice>>())
    //                     .unwrap()
    //             },
    //             border_image_repeat: {
    //                 world.init_single_res::<DefaultComponent<BorderImageRepeat>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageRepeat>>())
    //                     .unwrap()
    //             },
    //             border_radius: {
    //                 world.init_single_res::<DefaultComponent<BorderRadius>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderRadius>>())
    //                     .unwrap()
    //             },
    //             box_shadow: {
    //                 world.init_single_res::<DefaultComponent<BoxShadow>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<BoxShadow>>())
    //                     .unwrap()
    //             },
    //             text_style: {
    //                 world.init_single_res::<DefaultComponent<TextStyle>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextStyle>>())
    //                     .unwrap()
    //             },
    //             text_shadow: {
    //                 world.init_single_res::<DefaultComponent<TextShadow>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextShadow>>())
    //                     .unwrap()
    //             },
    //             text_outer_glow: {
    //                 world.init_single_res::<DefaultComponent<TextOuterGlow>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextOuterGlow>>())
    //                     .unwrap()
    //             },
    //             transform_will_change: {
    //                 world.init_single_res::<DefaultComponent<TransformWillChange>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TransformWillChange>>())
    //                     .unwrap()
    //             },
    //             text_content: {
    //                 world.init_single_res::<DefaultComponent<TextContent>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextContent>>())
    //                     .unwrap()
    //             },
    //             animation: {
    //                 world.init_single_res::<DefaultComponent<Animation>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Animation>>())
    //                     .unwrap()
    //             },
	// 			transition: {
    //                 world.init_single_res::<DefaultComponent<Transition>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<Transition>>())
    //                     .unwrap()
    //             },
    //             node_state: {
    //                 world.init_single_res::<DefaultComponent<NodeState>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<NodeState>>())
    //                     .unwrap()
    //             },
    //             as_image: {
    //                 world.init_single_res::<DefaultComponent<AsImage>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<AsImage>>())
    //                     .unwrap()
    //             },
	// 			text_overflow: {
    //                 world.init_single_res::<DefaultComponent<TextOverflowData>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextOverflowData>>())
    //                     .unwrap()
    //             },
    //         }
    //     }
    // }

    // pub struct ChangeEvent {
    //     pub text_content: ColumnIndex,
    //     pub text_shadow: ColumnIndex,
    //     pub box_shadow: ColumnIndex,
    //     pub background_color: ColumnIndex,
    //     pub border_color: ColumnIndex,
    //     pub canvas: ColumnIndex,
	// 	pub transform_will_change: ColumnIndex,
    // }

    // impl FromWorld for ChangeEvent {
    //     fn from_world(world: &mut World) -> Self {
    //         Self {
    //             text_content: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<TextContent>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<TextContent>>>>())
    //                     .unwrap()
    //             },
    //             text_shadow: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<TextShadow>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<TextShadow>>>>())
    //                     .unwrap()
    //             },
    //             box_shadow: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<BoxShadow>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BoxShadow>>>>())
    //                     .unwrap()
    //             },
    //             background_color: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<BackgroundColor>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BackgroundColor>>>>())
    //                     .unwrap()
    //             },
    //             border_color: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<BorderColor>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BorderColor>>>>())
    //                     .unwrap()
    //             },
    //             canvas: {
    //                 world.init_single_res::<Events<ComponentEvent<Changed<Canvas>>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<Canvas>>>>())
    //                     .unwrap()
    //             },
	// 			transform_will_change: {
    //                 world.init_single_res::<Events<ComponentRemove<TransformWillChange>>>();
    //                 world
    //                     .components()
    //                     .get_resource_id(std::any::TypeId::of::<Events<ComponentRemove<TransformWillChange>>>())
    //                     .unwrap()
    //             },
    //         }
    //     }
    // }

    // pub struct DefaultStyle {
    //     pub size: SingleResMut<'a, DefaultComponent<Size>>,
    //     pub margin: SingleResMut<'a, DefaultComponent<Margin>>,
    //     pub padding: SingleResMut<'a, DefaultComponent<Padding>>,
    //     pub border: SingleResMut<'a, DefaultComponent<Border>>,
    //     pub position: SingleResMut<'a, DefaultComponent<Position>>,
    //     pub min_max: SingleResMut<'a, DefaultComponent<MinMax>>,
    //     pub flex_container: SingleResMut<'a, DefaultComponent<FlexContainer>>,
    //     pub flex_normal: SingleResMut<'a, DefaultComponent<FlexNormal>>,
    //     pub z_index: SingleResMut<'a, DefaultComponent<ZIndex>>,
    //     pub overflow: SingleResMut<'a, DefaultComponent<Overflow>>,
    //     pub opacity: SingleResMut<'a, DefaultComponent<Opacity>>,
    //     pub blend_mode: SingleResMut<'a, DefaultComponent<BlendMode>>,
    //     pub show: SingleResMut<'a, DefaultComponent<Show>>,
    //     pub transform: SingleResMut<'a, DefaultComponent<Transform>>,
    //     pub background_color: SingleResMut<'a, DefaultComponent<BackgroundColor>>,
    //     pub border_color: SingleResMut<'a, DefaultComponent<BorderColor>>,
    //     pub background_image: SingleResMut<'a, DefaultComponent<BackgroundImage>>,
    //     pub background_image_clip: SingleResMut<'a, DefaultComponent<BackgroundImageClip>>,
    //     pub mask_image: SingleResMut<'a, DefaultComponent<MaskImage>>,
    //     pub mask_image_clip: SingleResMut<'a, DefaultComponent<MaskImageClip>>,
    //     pub hsi: SingleResMut<'a, DefaultComponent<Hsi>>,
    //     pub blur: SingleResMut<'a, DefaultComponent<Blur>>,
    //     pub background_image_mod: SingleResMut<'a, DefaultComponent<BackgroundImageMod>>,
    //     pub border_image: SingleResMut<'a, DefaultComponent<BorderImage>>,
    //     pub border_image_clip: SingleResMut<'a, DefaultComponent<BorderImageClip>>,
    //     pub border_image_slice: SingleResMut<'a, DefaultComponent<BorderImageSlice>>,
    //     pub border_image_repeat: SingleResMut<'a, DefaultComponent<BorderImageRepeat>>,
    //     pub border_radius: SingleResMut<'a, DefaultComponent<BorderRadius>>,
    //     pub box_shadow: SingleResMut<'a, DefaultComponent<BoxShadow>>,
    //     pub text_style: SingleResMut<'a, DefaultComponent<TextStyle>>,
    //     pub transform_will_change: SingleResMut<'a, DefaultComponent<TransformWillChange>>,
    //     pub text_content: SingleResMut<'a, DefaultComponent<TextContent>>,
    //     pub animation: SingleResMut<'a, DefaultComponent<Animation>>,
    // 	pub node_state: SingleResMut<'a, DefaultComponent<NodeState>>,
    // }

    pub struct StyleAttr;

    impl StyleAttr {
        #[inline]
        pub fn get_type(style_type: u8) -> StyleType { (STYLE_ATTR[style_type as usize].get_type)() }

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
				(RESET_STYLE_ATTR[style_index as usize - STYLE_COUNT as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone)
			} else {
				(STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone)
			}
            
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
        pub fn reset(cur_style_mark: &mut StyleMarkType, style_index: u8, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity) {
            (RESET_STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, false);
        }

        #[inline]
        pub fn set_default(style_index: u8, buffer: &Vec<u8>, offset: usize, world: &mut World, query: &DefaultStyle) {
            (STYLE_ATTR[style_index as usize].set_default)(buffer, offset, world, query);
        }
    }
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
            StyleAttribute::Set(r) => style_to_buffer(style_buffer, r, &mut class_meta),
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


// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SvgWidthCmd(pub Entity, pub f32);

// impl Command for SvgWidthCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgContent>();
//         println!("component_id3: {:?}", component_id);
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
//             v.width = self.1;
//         } else {
//             let mut svg = SvgContent::default();
//             svg.width = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SvgHeightCmd(pub Entity, pub f32);

// impl Command for SvgHeightCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgContent>() };
//             v.height = self.1;
//         } else {
//             let mut svg = SvgContent::default();
//             svg.height = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }

//         // event_writer.send(ComponentEvent::<Changed<SvgContent>>::new(*entity));
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SvgColorCmd(pub Entity, pub Color);

// impl Command for SvgColorCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         println!("component_id3: {:?}", component_id);
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.fill_color = self.1;
//         } else {
//             let mut svg = SvgInnerContent::default();
//             svg.style.fill_color = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SvgStrokeColorCmd(pub Entity, pub CgColor);

// impl Command for SvgStrokeColorCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.stroke.color = self.1;
//         } else {
//             let mut svg = SvgInnerContent::default();
//             svg.style.stroke.color = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }

//         // event_writer.send(ComponentEvent::<Changed<SvgContent>>::new(*entity));
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SvgStrokeWidthCmd(pub Entity, pub NotNan<f32>);

// impl Command for SvgStrokeWidthCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.stroke.width = self.1;
//         } else {
//             let mut svg = SvgInnerContent::default();
//             svg.style.stroke.width = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }

//         // event_writer.send(ComponentEvent::<Changed<SvgContent>>::new(*entity));
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct StrokeDasharrayCmd(pub Entity, pub StrokeDasharray);

// impl Command for StrokeDasharrayCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.stroke_dasharray = self.1;
//         } else {
//             let mut svg = SvgInnerContent::default();
//             svg.style.stroke_dasharray = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

pub trait SerdEnum {
    // 枚举类型
	fn get_type() -> u8;

    /// 样式属性的牛内存大小
    fn size() -> usize where Self: Sized;
        
    /// 序列化自身到buffer中
    unsafe fn write(&self, buffer: &mut Vec<u8>);
}

pub enum SvgShapeEnum {
	Rect,
    Circle,
    Ellipse,
    Segment,
    Polygon,
    Polyline,
    Path,
}

/// svg属性类型
#[enum_type]
#[index_start(193)]
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
    SvgShaperRadius, // 13,
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
    #[v((Vec<f32>, Vec<f32>))]
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
    #[v(f32)]
    SvgStopColor, // 35,
    #[v(Entity)]
    SvgGradient, // 36,
    #[v(Entity)]
    SvgFilter, // 37,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rect { x: f32, y: f32, width: f32, height: f32 },
    Circle { cx: f32, cy: f32, radius: f32 },
    Ellipse { cx: f32, cy: f32, rx: f32, ry: f32 },
    Segment { ax: f32, ay: f32, bx: f32, by: f32},
    Polygon { points: Vec<[f32; 2]> },
    Polyline { points: Vec<[f32; 2]> },
    Path { points: Vec<[f32; 2]>, verb: Vec<pi_hal::pi_sdf::shape::PathVerb>}
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
                let mut p = points.iter().flat_map(|[x, y]| vec![*x, *y]).collect::<Vec<f32>>();
                p.push(5.0);
                p
            }
            Shape::Polyline { points } => {
                let mut p = points.iter().flat_map(|[x, y]| vec![*x, *y]).collect::<Vec<f32>>();
                p.push(6.0);
                p
            }
            Shape::Path { points, verb } => {
                let mut p = points.iter().flat_map(|[x, y]| vec![*x, *y]).collect::<Vec<f32>>();
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
            5 => Self::Polygon { points: value.2.chunks(2).map(|v|{[v[0],v[1]]}).collect::<Vec<[f32; 2]>>()},
            6 => Self::Polyline { points: value.2.chunks(2).map(|v|{[v[0],v[1]]}).collect::<Vec<[f32; 2]>>()},
            7 => Self::Path { 
                points: value.2.chunks(2).map(|v|{[v[0],v[1]]}).collect::<Vec<[f32; 2]>>(), 
                verb: value.1.iter().map(|v|unsafe { transmute(*v as u8) }).collect::<Vec<pi_hal::pi_sdf::shape::PathVerb>>()
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


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeCmd(pub Entity, pub Shape);

// impl Command for SvgShapeCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.shape = self.1;
//         } else {
//             let mut svg = SvgInnerContent::default();
//             svg.shape = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeWidthCmd(pub Entity, pub f32);

// impl Command for SvgShapeWidthCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Rect {
//                     x: _,
//                     y: _,
//                     width,
//                     height: _,
//                 } => {
//                     *width = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeHeightCmd(pub Entity, pub f32);

// impl Command for SvgShapeHeightCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Rect {
//                     x: _,
//                     y: _,
//                     width: _,
//                     height,
//                 } => {
//                     *height = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeXCmd(pub Entity, pub f32);

// impl Command for SvgShapeXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Rect {
//                     x,
//                     y: _,
//                     width: _,
//                     height: _,
//                 } => {
//                     *x = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeYCmd(pub Entity, pub f32);

// impl Command for SvgShapeYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Rect {
//                     x: _,
//                     y,
//                     width: _,
//                     height: _,
//                 } => {
//                     *y = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeCXCmd(pub Entity, pub f32);

// impl Command for SvgShapeCXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Circle { cx, cy: _, radius: _ } => {
//                     *cx = self.1;
//                 }
//                 Shape::Ellipse { cx, cy: _, rx, ry } => {
//                     *cx = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeCYCmd(pub Entity, pub f32);

// impl Command for SvgShapeCYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Ellipse { cx: _, cy, rx, ry } => {
//                     *cy = self.1;
//                 }
//                 Shape::Circle { cx: _, cy, radius: _ } => {
//                     *cy = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShaperRadiusCmd(pub Entity, pub f32);

// impl Command for SvgShaperRadiusCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Circle { cx: _, cy: _, radius } => {
//                     *radius = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeRadiusXCmd(pub Entity, pub f32);

// impl Command for SvgShapeRadiusXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Ellipse { cx: _, cy: _, rx, ry: _ } => {
//                     *rx = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeRadiusYCmd(pub Entity, pub f32);

// impl Command for SvgShapeRadiusYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Ellipse { cx: _, cy: _, rx: _, ry } => {
//                     *ry = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeAXCmd(pub Entity, pub f32);

// impl Command for SvgShapeAXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Segment { ax, ay, bx, by } => {
//                     *ax = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeAYCmd(pub Entity, pub f32);

// impl Command for SvgShapeAYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Segment { ax, ay, bx, by } => {
//                     *ay = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeBXCmd(pub Entity, pub f32);

// impl Command for SvgShapeBXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Segment { ax, ay, bx, by } => {
//                     *bx = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapeBYCmd(pub Entity, pub f32);

// impl Command for SvgShapeBYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Segment { ax, ay, bx, by } => {
//                     *by = self.1;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapePointsCmd(pub Entity, pub Vec<f32>);

// impl Command for SvgShapePointsCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Polygon { points } | Shape::Polyline { points } => *points = self.1.chunks(2).map(|v| [v[0], v[1]]).collect::<Vec<[f32; 2]>>(),
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShapePathCmd(pub Entity, pub Vec<f32>, pub Vec<f32>);

// impl Command for SvgShapePathCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };

//             match &mut v.shape {
//                 Shape::Path { points, verb } => {
//                     *points = self.2.chunks(2).map(|v| [v[0], v[1]]).collect::<Vec<[f32; 2]>>();
//                     *verb = self
//                         .1
//                         .iter()
//                         .map(|v| unsafe { transmute(*v as u8) })
//                         .collect::<Vec<pi_hal::pi_sdf::shape::PathVerb>>();
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShadowColorCmd(pub Entity, pub CgColor);

// impl Command for SvgShadowColorCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.shadow.color = self.1;
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShadowOffsetXCmd(pub Entity, pub f32);

// impl Command for SvgShadowOffsetXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.shadow.offset_x = self.1;
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShadowOffsetYCmd(pub Entity, pub f32);

// impl Command for SvgShadowOffsetYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.shadow.offset_y = self.1;
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgShadowBlurLevelCmd(pub Entity, pub f32);

// impl Command for SvgShadowBlurLevelCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgInnerContent>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgInnerContent>() };
//             v.style.shadow.blur_level = self.1;
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgFilterOffsetXCmd(pub Entity, pub f32);

// impl Command for SvgFilterOffsetXCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgFilterOffset>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgFilterOffset>() };
//             v.offset_x = self.1;
//         } else {
//             let mut svg = SvgFilterOffset::default();
//             svg.offset_x = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgFilterOffsetYCmd(pub Entity, pub f32);

// impl Command for SvgFilterOffsetYCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgFilterOffset>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgFilterOffset>() };
//             v.offset_y = self.1;
//         } else {
//             let mut svg = SvgFilterOffset::default();
//             svg.offset_y = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgFilterColorCmd(pub Entity, pub f32);

// impl Command for SvgFilterColorCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgFilterOffset>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgFilterOffset>() };
//             v.color = self.1;
//         } else {
//             let mut svg = SvgFilterOffset::default();
//             svg.color = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgFilterBlurLevelCmd(pub Entity, pub f32);

// impl Command for SvgFilterBlurLevelCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgFilterBlurLevel>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgFilterBlurLevel>() };
//             v.level = self.1;
//         } else {
//             let mut svg = SvgFilterBlurLevel::default();
//             svg.level = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgGradientX1Cmd(pub Entity, pub f32);

// impl Command for SvgGradientX1Cmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgGradient>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgGradient>() };
//             v.x1 = self.1;
//         } else {
//             let mut svg = SvgGradient::default();
//             svg.x1 = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgGradientY1Cmd(pub Entity, pub f32);

// impl Command for SvgGradientY1Cmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgGradient>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgGradient>() };
//             v.y1 = self.1;
//         } else {
//             let mut svg = SvgGradient::default();
//             svg.y1 = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgGradientX2Cmd(pub Entity, pub f32);

// impl Command for SvgGradientX2Cmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgGradient>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgGradient>() };
//             v.x2 = self.1;
//         } else {
//             let mut svg = SvgGradient::default();
//             svg.x2 = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgGradientY2Cmd(pub Entity, pub f32);

// impl Command for SvgGradientY2Cmd {
//     fn apply(self, world: &mut World) {
//         log::debug!("SvgGradientY2Cmd: {:?}, {}", self.0, self.1);
//         let component_id = world.init_component::<SvgGradient>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgGradient>() };
//             v.y2 = self.1;
//         } else {
//             let mut svg = SvgGradient::default();
//             svg.y2 = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgStopOffsetCmd(pub Entity, pub f32);

// impl Command for SvgStopOffsetCmd {
//     fn apply(self, world: &mut World) {
       
//         let component_id = world.init_component::<SvgStop>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgStop>() };
//             v.offset = self.1;
//         } else {
//             let mut svg = SvgStop::default();
//             svg.offset = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgStopColorCmd(pub Entity, pub CgColor);

// impl Command for SvgStopColorCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgStop>();
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgStop>() };
//             v.color = self.1;
//         } else {
//             let mut svg = SvgStop::default();
//             svg.color = self.1;
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgGradientCmd(pub Entity, pub Entity);

// impl Command for SvgGradientCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgGradient>();
//         log::debug!("SvgGradientCmd: {:?}, {:?}", self.0,self.1);
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgGradient>() };
//             v.id.push(self.1);
//         } else {
//             let mut svg = SvgGradient::default();
//             svg.id.push(self.1);
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SvgFilterCmd(pub Entity, pub Entity);

// impl Command for SvgFilterCmd {
//     fn apply(self, world: &mut World) {
//         let component_id = world.init_component::<SvgFilter>();
//         log::debug!("SvgFilterCmd: {:?}, {:?}", self.0, self.1);
//         if let Some(mut component) = world.get_mut_by_id(self.0, component_id) {
//             component.set_changed();
//             let v = unsafe { component.into_inner().deref_mut::<SvgFilter>() };
//             v.0.push(self.1);
//         } else {
//             let mut svg = SvgFilter::default();
//             svg.0.push(self.1);
//             world.entity_mut(self.0).insert(svg);
//         }
//     }
// }
// // // svg属性
// // pub trait SvgAttr {
// //     /// 将样式属性设置到组件上
// //     /// ptr为样式属性的指针
// //     /// 安全： entity必须存在
// //     fn set<'w, 's>(ptr: *const u8, query: &mut Setting, entity: Entity)
// //     where
// //         Self: Sized;
    
// //     fn get_index() -> u8 where Self: Sized;
// // }
