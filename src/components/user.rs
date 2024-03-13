//！ 定义用户设置的组件

use std::marker::PhantomData;
use std::mem::{transmute, forget};
use std::ptr::read_unaligned;
use std::{collections::VecDeque, fmt::Debug};

use bevy_ecs::event::Event;
use bevy_ecs::prelude::{Changed, Component, DetectChangesMut, Entity};
use bitvec::prelude::BitArray;
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_flex_layout::prelude::INode;
pub use pi_flex_layout::prelude::{Dimension, Number, Rect, Size as FlexSize};
use pi_flex_layout::style::{AlignContent, AlignItems, AlignSelf, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, OverflowWrap};
use pi_null::Null;
use pi_slotmap::DefaultKey;
use pi_style::style::TextOverflow;
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

use crate::resource::animation_sheet::TransitionData;

use super::calc::{NeedMark, EntityKey};
pub use super::root::{ClearColor, RenderDirty, RenderTargetType, Viewport};
use smallvec::SmallVec;

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

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

#[derive(Deref, Clone, Debug, Event)]
pub struct ComponentRemove<T: Component> {
	#[deref]
	pub id: Entity,
	mark: PhantomData<T>,
}

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

#[derive(Deref, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug, Component)]
#[component(storage = "SparseSet")]
pub struct ZIndex(pub isize);

/// 当post_process不为null时， 节点需要通过post_process对应的图节点进行处理，输出结果再渲染到gui上(注意，当前节点问根节点时，设置post_process，将不能把结果再渲染回gui)
#[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug, Component)]
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

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
pub struct BoxShadow(pub BoxShadow1);

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
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

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
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

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
pub struct ClipPath(pub BaseShape);
impl NeedMark for ClipPath {
    fn need_mark(&self) -> bool { true }
}

#[derive(Clone, Default, Deref, Debug, Serialize, Deserialize, Component)]
pub struct BlendMode(pub BlendMode1);

#[derive(Default, Debug, Clone, Serialize, Deserialize, Component)]
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

#[derive(Default, Debug, Clone, Serialize, Deserialize, Component)]
pub struct Transition {
    pub property: SmallVec<[usize; 1]>, // 指定过度影响的属性
	pub duration: SmallVec<[Time; 1]>,                           // 指定需要多少毫秒完成过度
	pub delay: SmallVec<[Time; 1]>,                    // 启动过度前的延迟间隔。
    pub timing_function: SmallVec<[AnimationTimingFunction; 1]>, // 插值函数

	// 计算数据
	pub mark: BitArray<[u32;3]>,
	pub data: SmallVec<[TransitionData; 1]>,
	pub is_all: usize,
}

impl Transition {
    pub fn get_attr<T: Default + Clone>(i: usize, vec: &SmallVec<[T; 1]>) -> T {
        Animation::get_attr(i, vec)
    }
}


//ObjectFit
#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, Component)]
pub struct BackgroundImageMod {
    pub object_fit: FitType,
    pub repeat: ImageRepeat,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref, Component)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref, Component)]
pub struct BorderRadius(pub BorderRadius1);

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deref, Component, Hash)]
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

#[derive(Deref, Clone, Debug, Serialize, Deserialize, Component, Default)]
pub struct TextContent(pub TextContent1);


// 将display、visibility、enable合并为show组件
#[derive(Deref, Clone, Debug, PartialEq, Serialize, Deserialize, Component)]
pub struct Show(pub usize);

// 变换
#[derive(Debug, Clone, Default, Serialize, Deserialize, Component)]
#[component(storage = "SparseSet")]
pub struct Transform {
    pub all_transform: AllTransform,
    pub origin: TransformOrigin,
}

impl Transform {
    pub fn add_func(&mut self, f: TransformFunc) { self.all_transform.transform.push(f); }
    pub fn set_origin(&mut self, o: TransformOrigin) { self.origin = o; }
}
// 背景色和class
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, Component)]
pub struct BackgroundColor(pub Color);

// class名称， 支持多个class， 当只有一个或两个class时， 有优化
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, Component)]
pub struct ClassName(pub SmallVec<[usize; 1]>);

// 边框颜色
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, Component)]
pub struct BorderColor(pub CgColor);

// 图片路劲及纹理
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Default, Hash, Component, PartialEq, Eq)]
pub struct BackgroundImage(pub Atom);

impl From<Atom> for BackgroundImage {
    fn from(value: Atom) -> Self { BackgroundImage(value) }
}

impl BackgroundImage {
    pub fn set_url() {}
}

#[derive(Debug, Deref, Clone, Serialize, Deserialize, Component)]
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
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Component)]
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
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Default, Hash, Component, PartialEq, Eq)]
pub struct BorderImage(pub Atom);

impl From<Atom> for BorderImage {
    fn from(value: Atom) -> Self { Self(value) }
}

// borderImage图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, Clone, Serialize, Deserialize, Component, Hash)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, Deref, Component)]
pub struct BorderImageRepeat(pub ImageRepeat);

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
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


#[derive(Debug, Clone, Serialize, Deserialize, Component, Default)]
pub struct TextOverflowData {
	pub text_overflow: TextOverflow,
	pub text_overflow_char: SmallVec<[TextOverflowChar;1]>, // 通常是...
}


#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct TextOverflowChar {
	pub width: f32,
	pub ch: char,
	pub ch_id: DefaultKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component, Default, Deref)]
pub struct TextShadow(pub TextShadowList);

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
#[derive(Default, Debug, Clone, Serialize, Deserialize, Component)]
#[component(storage = "SparseSet")]
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
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug, Component)]
pub struct Margin(pub Rect<Dimension>);

/// 布局内边距
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug, Component)]
pub struct Padding(pub Rect<Dimension>);

/// 布局边框尺寸
#[derive(Default, Deref, Clone, Serialize, Deserialize, Debug, Component)]
pub struct Border(pub Rect<Dimension>);

#[derive(Deref, Clone, Serialize, Deserialize, Debug, Component)]
pub struct Position(pub Rect<Dimension>);

#[derive(Default, Clone, Serialize, Deserialize, Debug, Component)]
pub struct MinMax {
    pub min: FlexSize<Dimension>,
    pub max: FlexSize<Dimension>,
}

// 描述子节点行为的flex布局属性
#[derive(Clone, Serialize, Deserialize, Debug, Component)]
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
#[derive(Clone, Serialize, Deserialize, Debug, Component)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Canvas {
	pub id: Entity,
	pub by_draw_list: bool,
}

/// 显示改变（一般是指canvas，gui不能感知除了style属性以外的属性改变，如果canvas内容发生改变，应该通过style设置，以便gui能感知，从而设置脏区域）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component)]
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
        calc::{BackgroundImageTexture, BorderImageTexture, StyleMark},
        user::*,
    };
    use pi_atom::Atom;
    // use pi_ecs::{
    //     prelude::{Query, ResMut},
    //     query::{DefaultComponent, Write},
    // };
    use bevy_ecs::{
        component::ComponentId,
        prelude::{Entity, FromWorld, World, Events}
    };
    use pi_bevy_ecs_extend::prelude::DefaultComponent;
    use pi_flex_layout::{
        prelude::Number,
        style::{
            AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent,
            PositionType as PositionType1,
        },
    };
    use pi_print_any::println_any;
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
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
        where
            Self: Sized;
    }

	pub trait ConvertToComponent: AttrSet {
		/// 获取属性
		fn get(
			query: &mut Setting,
			entity: Entity,
		) -> Option<Attribute>;

        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World)
        where
            Self: Sized;

        fn to_attr(ptr: *const u8) -> Attribute
        where
            Self: Sized;
    }

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

    fn set_style_attr<V: Debug, C: Component + Clone, F: FnMut(&mut C, V)>(
        world: &mut World,
        entity: Entity,
        component_id: ComponentId,
        default_component_id: ComponentId,
		// dirty_list: ComponentId,
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
        match world.get_mut_by_id(entity, component_id) {
            Some(mut component) => {
                component.set_changed();
                f(unsafe { component.into_inner().deref_mut::<C>() }, v);
            }
            None => {
                let default_value = world.get_resource_by_id(default_component_id).unwrap();
                let mut r = unsafe { default_value.deref::<DefaultComponent<C>>() }.0.clone();
                f(&mut r, v);
                world.entity_mut(entity).insert(r);
            }
        };
		// unsafe { world.get_resource_mut_by_id(dirty_list).unwrap().into_inner().deref_mut::<DirtyList>() }.push(entity);
		// unsafe { dirty_list.into_inner().deref_mut::<DirtyList>() }
		// dirty_list
    }

	// 在设置Class、styleMark时调用， 不需要进脏列表
    pub fn set_style_attr_or_default<V, C: Component + Clone + Default, F: FnMut(&mut C, V)>(
        world: &mut World,
        entity: Entity,
        component_id: ComponentId,
        v: V,
        mut f: F,
    ) {
        match world.get_mut_by_id(entity, component_id) {
            Some(mut component) => {
                component.set_changed();
                // SAFETY: `test_component` has unique access of the `EntityMut` and is not used afterwards
                f(unsafe { component.into_inner().deref_mut::<C>() }, v);
            }
            None => {
                let mut default_value = C::default();
                f(&mut default_value, v);
                world.entity_mut(entity).insert(default_value);
            }
        };
    }

    pub unsafe fn get_component_mut<C: Component + Clone + Default>(world: &mut World, entity: Entity, component_id: ComponentId) -> &mut C {
        match world.get_mut_by_id(entity, component_id) {
            Some(component) => unsafe { component.into_inner().deref_mut::<C>() },
            None => panic!("get_component fail, get_component is not exist: {:?}, entity: {:?}", component_id, entity),
        }
    }


    fn set_default_style_attr<V, C: Component + Clone, F: FnMut(&mut C, V)>(world: &mut World, default_component_id: ComponentId, v: V, mut f: F) {
		pi_print_any::out_any!(log::trace, "set_default_style_attr==={:?}", &v);
        match world.get_resource_mut_by_id(default_component_id) {
            Some(mut component) => {
                component.set_changed();
                // SAFETY: `test_component` has unique access of the `EntityMut` and is not used afterwards
                f(&mut *unsafe { component.into_inner().deref_mut::<DefaultComponent<C>>() }, v);
            }
            None => {
                log::error!(
                    "set_default_style_attr fail, default value is not exist, {:?}",
                    std::any::type_name::<C>()
                );
            }
        };
    }

    fn reset_style_attr<C: Component + Clone, F: FnMut(&mut C, &C)>(
        world: &mut World,
        entity: Entity,
        component_id: ComponentId,
        default_component_id: ComponentId,
		// dirty_list: ComponentId,
        mut f: F,
    ) {
        match world.get_resource_by_id(default_component_id) {
            Some(component) => {
                // SAFETY: 这里取组件的默认值单例，和修改组件不冲突，transmute只是为了绕开借用检查，实际上是安全的
                let default_value = unsafe { transmute(&**component.deref::<DefaultComponent<C>>()) };
                if let Some(mut component) = world.get_mut_by_id(entity, component_id) {
                    component.set_changed();
                    f(unsafe { component.into_inner().deref_mut::<C>() }, default_value);
                };
            }
            None => {
                log::error!(
                    "set_default_style_attr fail, default value is not exist, {:?}",
                    std::any::type_name::<C>()
                );
            }
        };
		// unsafe { world.get_resource_mut_by_id(dirty_list).unwrap().into_inner().deref_mut::<DirtyList>() }.push(entity);
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
        pub fn write_to_component(&mut self, cur_style_mark: &mut BitArray<[u32; 3]>, entity: Entity, query: &mut Setting, is_clone: bool) -> bool {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
				// pi_print_any::out_any!(log::trace, "write_to_component==={:?}, cursor:{:?}, next_type: {:?}", style_type, self.cursor, next_type);
                StyleAttr::set(cur_style_mark, style_type, &self.buffer, self.cursor, query, entity, is_clone);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return true;
                // return Some(StyleAttr::get_type(style_type));
            }
            false
        }

        // 将当前style写入默认组件
        pub fn write_to_default(&mut self, query: &DefaultStyle, world: &mut World) -> Option<StyleType> {
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
                StyleAttr::set_default(style_type, &self.buffer, self.cursor, query, world);
                let size = StyleAttr::size(style_type);
                self.cursor += size;
                return Some(StyleAttr::get_type(style_type));
            }

            None
        }

        // 将当前style写入组件
        pub fn to_attr(&mut self) -> Option<StyleAttribute> {
			let c = self.cursor;
            let next_type = self.next_type();
            if let Some(style_type) = next_type {
                let r = if style_type < 96 {
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
            cur_style_mark: &mut BitArray<[u32; 3]>,
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
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					// query.style.dirty_list,
                    v,
                    |item: &mut $value_ty, v: $value_ty| *item = v,
                );
            }
        };
        // 表达式
        (@fun $name: ident, $value_ty: ty, $f: expr) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(&mut query.world, entity, query.style.$name, query.style.default.$name, v, $f);
            }
        };

        (@fun_send $name: ident, $value_ty: ty, $c_ty: ty, $f: expr) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
					clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(&mut query.world, entity, query.style.$name, query.style.default.$name, v, $f);
                if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.$name) {
                    unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<$c_ty>>>>() }
                        .send(ComponentEvent::<Changed<$c_ty>>::new(entity));
                };
            }
        };
        // 属性修改
        (@pack $name: ident, $pack_ty: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                // 取不到说明实体已经销毁
                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
                    v,
                    |item: &mut $pack_ty, v: $value_ty| *item = $pack_ty(v),
                );
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    v,
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$feild = v;
                    },
                );
            }
        };
        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);
                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    v,
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$set_func(v);
                    },
                );
            }
        };

        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };
                cur_style_mark.set(Self::get_type() as usize, true);

                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    v,
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$feild1.$feild2 = v;
                    },
                );
            }
        };

        // 盒模属性（上右下左）
        (@box_model $name: ident, $value_ty: ty) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool) {
                let v = ptr.cast::<$value_ty>();
                let v = if is_clone {
                    clone_unaligned(v)
                } else {
                    unsafe { v.read_unaligned() }
                };

                set_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    v,
                    |item: &mut $value_ty, v: $value_ty| {
                        *item = v;
                    },
                );

                cur_style_mark.set(Self::get_type() as usize, true);
            }
        };
    }

	macro_rules! get {
		(@empty) => {
			fn get(_query: &mut Setting, _entity: Entity) -> Option<Attribute> {
				None
			}
		};
        // 整体插入
        ($name: ident, $ty: ident, $struct_name: ident, $value_ty: ty) => {
            fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
				match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(mut component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$value_ty>().clone() })))
					None => None
				}
            }
        };

		// 属性修改
        (@pack $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty) => {
			fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
                match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$component_ty>().0.clone() }))),
					None => None
				}
            }
        };

		// 属性修改
        (@feild $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field: ident) => {
            fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
                match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$component_ty>().$field.clone() }))),
					None => None
				}
            }
        };

        // 属性修改
        (@feild2 $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field1: ident, $field2: ident) => {
			fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
                match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$component_ty>().$field1.$field2.clone() }))),
					None => None
				}
            }
        };

		// 属性修改
		(@feild3 $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $field1: ident, $field2: ident, $field3: ident) => {
			fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
                match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$component_ty>().$field1.$field2.$field3.clone() }))),
					None => None
				}
            }
        };

        // 表达式
        (@fun $name: ident, $ty: ident, $struct_name: ident, $component_ty: ty, $f: ident) => {
            fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
                match query.world.get_mut_by_id(entity, query.style.$name) {
					Some(component) => Some(Attribute::$ty($struct_name(unsafe { component.into_inner().deref_mut::<$component_ty>().$f() }))),
					None => None
				}
            }
        };
    }

    // 设置默认值
    macro_rules! set_default {
        (@empty) => {
            fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World) {}
        };
        // 整体插入
        ($name: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
                set_default_style_attr(
                    world,
                    query.$name,
                    unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() },
                    |item: &mut $value_ty, v: $value_ty| {
                        *item = v;
                    },
                );
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
                set_default_style_attr(
                    world,
                    query.$name,
                    unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() },
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$feild = v;
                    },
                );
            }
        };
        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
                set_default_style_attr(
                    world,
                    query.$name,
                    unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() },
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$set_func(v);
                    },
                );
            }
        };

        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
                set_default_style_attr(
                    world,
                    query.$name,
                    unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() },
                    |item: &mut $c_ty, v: $value_ty| {
                        item.$feild1.$feild2 = v;
                    },
                );
            }
        };

        // 盒模属性（上右下左）
        (@box_model $name: ident, $c_ty: ty, $value_ty: ty) => {
            fn set_default<'a>(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
                set_default_style_attr(
                    world,
                    query.$name,
                    unsafe { buffer.as_ptr().add(offset).cast::<$value_ty>().read_unaligned() },
                    |item: &mut $c_ty, v: $value_ty| {
                        c.top = v.top;
                        c.right = v.right;
                        c.bottom = v.bottom;
                        c.left = v.left;
                    },
                );
            }
        };
    }

    macro_rules! reset {
        // 空实现
        (@empty) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, _query: &mut Setting, _entity: Entity, _is_clone: bool) {}
        };
        ($name: ident, $value_ty: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $value_ty, v: &$value_ty| {
                        *item = v.clone();
                    },
                );
            }
        };
		// 属性重置， 并发送事件
        (@func_send $name: ident, $value_ty: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $value_ty, v: &$value_ty| {
                        *item = v.clone();
                    },
                );
				if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.$name) {
					unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<$value_ty>>>>() }
						.send(ComponentEvent::<Changed<$value_ty>>::new(entity));
				};
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild = v.$feild.clone();
                    },
                );
            }
        };

        // 属性修改
        (@func $name: ident, $c_ty: ty, $set_func: ident, $get_func: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$set_func(v.$get_func());
                    },
                );
            }
        };
        // 属性修改
        ($name: ident, $c_ty: ty, $feild1: ident, $feild2: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild1.$feild2 = v.$feild1.$feild2.clone();
                    },
                );
            }
        };
        // 属性修改
        (@box_model_single $name: ident, $c_ty: ty, $feild: ident) => {
            fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
                    |item: &mut $c_ty, v: &$c_ty| {
                        item.$feild = v.$feild;
                    },
                );
            }
        };

        (@box_model $name: ident, $ty: ident) => {
            fn set(cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool) {
                reset_style_attr(
                    &mut query.world,
                    entity,
                    query.style.$name,
                    query.style.default.$name,
					
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
			fn set_default<'a>(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, world: &mut World) {

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
			set!(@fun $name, $value_ty, |item: &mut $pack_ty, v: $value_ty| *item = $pack_ty(v));
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
			set!(@fun_send $name, $value_ty, $pack_ty, |item: &mut $pack_ty, v: $value_ty| *item = $pack_ty(v));
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
    // 	fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity)
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
    // 	fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity)
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
        fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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
                entity,
                query.style.text_content,
                query.style.default.text_content,
				// query.style.dirty_list,
                v,
                |item: &mut TextContent, v| {
                    item.0 = v;
                },
            );
            // 发送事件
            if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.text_content) {
                unsafe { component.into_inner().deref_mut::<Events<ComponentEvent<Changed<TextContent>>>>() }
                    .send(ComponentEvent::<Changed<TextContent>>::new(entity));
            };


            // 插入默认的FlexContainer组件
            if let None = query.world.get_mut_by_id(entity, query.style.flex_container) {
                let default_value = query.world.get_resource_by_id(query.style.default.flex_container).unwrap();
                let r = unsafe { default_value.deref::<DefaultComponent<FlexContainer>>() }.0.clone();
                query.world.entity_mut(entity).insert(r);
            };
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
    // 	fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity)
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
    // 	fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity)
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
    // 	fn set(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity)
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
    // 	fn set(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity)
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

	impl AttrSet for BackgroundImageType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<BackgroundImage>(),
                v,
                entity
            );
            match world.get_mut_by_id(entity, query.style.background_image) {
                Some(mut component) => {
                    component.set_changed();
                    unsafe { component.into_inner().deref_mut::<BackgroundImage>() }.0 = v;
                    // f(unsafe { component.into_inner().deref_mut::<Atom>() }, v);
                }
                None => {
                    // 顺便插入默认的BackgroundImageTexture， 以免后续修改原型
                    match world.get_mut_by_id(entity, query.style.background_image_texture) {
                        Some(_) => world.entity_mut(entity).insert(BackgroundImage(v)),
                        None => world.entity_mut(entity).insert((BackgroundImage(v), BackgroundImageTexture::default())),
                    };
                }
            };
        }
	}
    impl ConvertToComponent for BackgroundImageType {
        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World)
        where
            Self: Sized,
        {
            set_default_style_attr(
                world,
                query.background_image,
                unsafe { buffer.as_ptr().add(offset).cast::<Atom>().read_unaligned() },
                |item: &mut BackgroundImage, v: Atom| {
                    **item = v;
                },
            );
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
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            reset_style_attr(
                &mut query.world,
                entity,
                query.style.background_image,
                query.style.default.background_image,
				// query.style.dirty_list,
                |item: &mut BackgroundImage, v: &BackgroundImage| {
                    *item = v.clone();
                },
            );
            // 设置纹理， TODO
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
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<BorderImage>(),
                v,
                entity
            );
            match world.get_mut_by_id(entity, query.style.border_image) {
                Some(mut component) => {
                    component.set_changed();
                    unsafe { component.into_inner().deref_mut::<BorderImage>() }.0 = v;
                    // f(unsafe { component.into_inner().deref_mut::<Atom>() }, v);
                }
                None => {
                    // 顺便插入默认的BorderImageTexture， 以免后续修改原型
                    match world.get_mut_by_id(entity, query.style.border_image_texture) {
                        Some(_) => world.entity_mut(entity).insert(BorderImage(v)),
                        None => world.entity_mut(entity).insert((BorderImage(v), BorderImageTexture::default())),
                    };
                }
            };
        }

	}
    impl ConvertToComponent for BorderImageType {
        
        /// 为样式设置默认值
        fn set_default(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World)
        where
            Self: Sized,
        {
            set_default_style_attr(
                world,
                query.border_image,
                unsafe { buffer.as_ptr().add(offset).cast::<Atom>().read_unaligned() },
                |item: &mut BorderImage, v: Atom| {
                    **item = v;
                },
            );
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
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            reset_style_attr(
                &mut query.world,
                entity,
                query.style.border_image,
                query.style.default.border_image,
				// query.style.dirty_list,
                |item: &mut BorderImage, v: &BorderImage| {
                    *item = v.clone();
                },
            );
            // 设置纹理， TODO
        }
	}
    // impl_style!(@func1 TransformFuncType, transform, Transform, add_func, TransformFunc, TransformFunc, TransformFunc);

	impl AttrSet for TransformType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );

			if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.transform = v;
					return
				}
            };
			
            // 不存在transform_willChange， 则设置在Transfrom上
			match world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.transform = v;
				}
				None => {
					world.entity_mut(entity).insert(Transform {
						all_transform: AllTransform {
							transform: v,
							..Default::default()
						},
						..Default::default()
					});
				}
			}
        }

	}
    impl ConvertToComponent for TransformType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World)
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
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Some(mut component) = query.world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.transform = Default::default();
					return
				}
            };

            match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.transform = Default::default();
				}
				None => (),
			}
        }

	}


	impl AttrSet for TranslateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );
			if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.translate = Some(v);
					return
				}
            };

           // 不存在transform_willChange， 则设置在Transfrom上
		   match world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.translate = Some(v);
				}
				None => {
					world.entity_mut(entity).insert(Transform {
						all_transform: AllTransform {
							translate: Some(v),
							..Default::default()
						},
						..Default::default()
					});
				}
			}
        }
	}
    impl ConvertToComponent for TranslateType {
        

        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World)
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

		fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
			match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(component) => match unsafe { component.into_inner().deref_mut::<Transform>().all_transform.translate } {
					Some(r) => Some(Attribute::Translate(TranslateType(r))),
					None => None,
				},
				None => None
			}
		}
    }

	impl AttrSet for ResetTranslateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Some(mut component) = query.world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.translate = None;
					return
				}
            };

            match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.translate = None;
				}
				None => (),
			}
        }

	}

	impl AttrSet for ScaleType {
		 /// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );
			if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.scale = Some(v);
					return;
				}
            };
            // 不存在transform_willChange， 则设置在Transfrom上
			match world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.scale = Some(v);
				}
				None => {
					world.entity_mut(entity).insert(Transform {
						all_transform: AllTransform {
							scale: Some(v),
							..Default::default()
						},
						..Default::default()
					});
				}
			}
        }
	}
    impl ConvertToComponent for ScaleType {
       

        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World)
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

		fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
			match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(component) => match unsafe { component.into_inner().deref_mut::<Transform>().all_transform.scale } {
					Some(r) => Some(Attribute::Scale(ScaleType(r))),
					None => None,
				},
				None => None
			}
		}
    }

	impl AttrSet for ResetScaleType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Some(mut component) = query.world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.scale = None;
					return
				}
            };
			match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.scale = None;
				}
				None => (),
			}
        }
	}

	impl AttrSet for RotateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<Transform>(),
                v,
                entity
            );
			if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.rotate = Some(v);
					return;
				}
            };
            // 不存在transform_willChange， 则设置在Transfrom上
			match world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.rotate = Some(v);
				}
				None => {
					world.entity_mut(entity).insert(Transform {
						all_transform: AllTransform {
							rotate: Some(v),
							..Default::default()
						},
						..Default::default()
					});
				}
			}
        }

	}
    impl ConvertToComponent for RotateType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World)
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

		fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
			match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(component) => match unsafe { component.into_inner().deref_mut::<Transform>().all_transform.rotate } {
					Some(r) => Some(Attribute::Rotate(RotateType(r))),
					None => None,
				},
				None => None
			}
		}
    }

	impl AttrSet for ResetRotateType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
			if let Some(mut component) = query.world.get_mut_by_id(entity, query.style.transform_will_change) {
				let r = unsafe { component.as_ref().deref::<TransformWillChange>() };
				if r.0.is_some() {
					// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
					component.set_changed();
				}
				let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
				if let Some(r) = &mut r.0 {
					r.rotate = None;
					return;
				}
            };

            match query.world.get_mut_by_id(entity, query.style.transform) {
				Some(mut component) => {
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform.rotate = None;
				}
				None => (),
			}
        }

	}

	impl AttrSet for TransformWillChangeType {
		/// 将样式属性设置到组件上
        /// ptr为样式属性的指针
        /// 安全： entity必须存在
        fn set<'w, 's>(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool)
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

            let world = &mut query.world;
            log::debug!(
                "set_style_attr, type: {:?}, value: {:?}, entity: {:?}",
                std::any::type_name::<TransformWillChange>(),
                v,
                entity
            );

			if !v {
				if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
					let r = unsafe { component.as_ref().deref::<TransformWillChange>() }.clone();
					if r.0.is_some() {
						// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
						component.set_changed();
						let r = unsafe { component.into_inner().deref_mut::<TransformWillChange>() };
						r.0 = None;
					}
					if let Some(c) = &r.0 {
						// 删除TransformWillChange, 设置Transform
					
							// 设置transform
						match world.get_mut_by_id(entity, query.style.transform) {
							Some(mut component) => {
								// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
								component.set_changed();
								unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform = c.clone();
							}
							None => {
								world.entity_mut(entity).insert(Transform {
									all_transform: c.clone(),
									..Default::default()
								});
							}
						}
				
						return;
						// world.entity_mut(entity).remove::<TransformWillChange>();
						// if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.transform_will_change) {
						// 	unsafe { component.into_inner().deref_mut::<Events<ComponentRemove<TransformWillChange>>>() }
						// 		.send(ComponentRemove::<TransformWillChange>{id: entity, mark: PhantomData});
						// };
					}
					
				}
			} else {
				// 不存在transform_willChange， 则设置在Transfrom上
				match world.get_mut_by_id(entity, query.style.transform) {
					Some(component) => {
						let c = unsafe { component.into_inner().deref_mut::<Transform>() }.clone();
						world.entity_mut(entity).insert(TransformWillChange(Some(c.all_transform)));
					}
					None => {
						world.entity_mut(entity).insert(TransformWillChange::default());
					}
				}
			}
        }

	}
    impl ConvertToComponent for TransformWillChangeType {
        
        /// 为样式设置默认值
        fn set_default(_buffer: &Vec<u8>, _offset: usize, _query: &DefaultStyle, _world: &mut World)
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

		fn get(query: &mut Setting, entity: Entity) -> Option<Attribute> {
			match query.world.get_mut_by_id(entity, query.style.transform_will_change) {
				Some(component) => if unsafe { component.into_inner().deref_mut::<TransformWillChange>().0.is_some()} {
					Some(Attribute::TransformWillChange(TransformWillChangeType(true)))
				} else {
					None
				},
				None => None
			}
		}
    }

	impl AttrSet for ResetTransformWillChangeType {
		fn set<'w, 's>(_cur_style_mark: &mut BitArray<[u32; 3]>, _ptr: *const u8, query: &mut Setting, entity: Entity, _is_clone: bool)
        where
            Self: Sized,
        {
            let world = &mut query.world;
            log::debug!("reset_style_attr, type: TransformWillChange, entity: {:?}", entity);
            if let Some(mut component) = world.get_mut_by_id(entity, query.style.transform_will_change) {
                // 删除TransformWillChange, 设置Transform
                let r = unsafe { component.as_ref().deref::<TransformWillChange>() }.0.clone();
				if let Some(c) = r {
					component.set_changed();
					unsafe { component.into_inner().deref_mut::<TransformWillChange>() }.0 = None;

					// 设置transform
					match world.get_mut_by_id(entity, query.style.transform) {
						Some(mut component) => {
							// 如果存在transform_willChange,则将Transform设置在TransformWillChange上
							component.set_changed();
							unsafe { component.into_inner().deref_mut::<Transform>() }.all_transform = c;
						}
						None => {
							world.entity_mut(entity).insert(Transform {
								all_transform: c,
								..Default::default()
							});
						}
					}
				}
               
				
                // world.entity_mut(entity).remove::<TransformWillChange>();
				// if let Some(component) = query.world.get_resource_mut_by_id(query.style.event.transform_will_change) {
				// 	unsafe { component.into_inner().deref_mut::<Events<ComponentRemove<TransformWillChange>>>() }
				// 		.send(ComponentRemove::<TransformWillChange>{id: entity, mark: PhantomData});
				// };
				
            }
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

    impl_style!(@pack OpacityType, opacity, Opacity, f32);
    impl_style!(@pack BorderRadiusType, border_radius, BorderRadius, BorderRadius1);
    impl_style!(@pack HsiType, hsi, Hsi, Hsi1);
    impl_style!(@pack BlurType, blur, Blur, f32);
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

    impl_style!(@pack ZIndexType, z_index, ZIndex, isize);
    impl_style!(@pack OverflowType, overflow, Overflow, bool);

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
		get: fn(query: &mut Setting, entity: Entity) -> Option<Attribute>,
        // get_style_index: fn() -> u8,
        size: fn() -> usize,
        // /// 安全： entity必须存在
        // fn set(&self, cur_style_mark: &mut BitArray<[u32;3]>, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity);
        /// 安全： entity必须存在
        set: fn(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),

        /// 设置默认值
        set_default: fn(buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World),
        to_attr: fn(ptr: *const u8) -> Attribute,
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
                // add: T::add,
                // scale: T::scale,
            }
        }
    }

	pub struct ResetStyleFunc {
		set: fn(cur_style_mark: &mut BitArray<[u32; 3]>, ptr: *const u8, query: &mut Setting, entity: Entity, is_clone: bool),
	}

	impl ResetStyleFunc {
        fn new<T: AttrSet>() -> ResetStyleFunc {
            ResetStyleFunc {
                set: T::set,
            }
        }
    }

    lazy_static::lazy_static! {

        static ref STYLE_ATTR: [StyleFunc; 96] = [
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
		];

		
		static ref RESET_STYLE_ATTR: [ResetStyleFunc; 96] = [
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

        ];
    }

    pub struct Setting<'w> {
        pub style: &'w StyleQuery,
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

        pub fn new(style: &'w StyleQuery, world: &'w mut World) -> Self { Self { style, world } }
    }

    impl FromWorld for StyleQuery {
        fn from_world(world: &mut World) -> Self {
            Self {
                size: world.init_component::<Size>(),
                margin: world.init_component::<Margin>(),
                padding: world.init_component::<Padding>(),
                border: world.init_component::<Border>(),
                position: world.init_component::<Position>(),
                min_max: world.init_component::<MinMax>(),
                flex_container: world.init_component::<FlexContainer>(),
                flex_normal: world.init_component::<FlexNormal>(),
                z_index: world.init_component::<ZIndex>(),
                overflow: world.init_component::<Overflow>(),
                opacity: world.init_component::<Opacity>(),
                blend_mode: world.init_component::<BlendMode>(),
                show: world.init_component::<Show>(),
                transform: world.init_component::<Transform>(),
                background_color: world.init_component::<BackgroundColor>(),
                border_color: world.init_component::<BorderColor>(),
                background_image: world.init_component::<BackgroundImage>(),
                background_image_texture: world.init_component::<BackgroundImageTexture>(),
                background_image_clip: world.init_component::<BackgroundImageClip>(),
                mask_image: world.init_component::<MaskImage>(),
                mask_image_clip: world.init_component::<MaskImageClip>(),
                hsi: world.init_component::<Hsi>(),
                blur: world.init_component::<Blur>(),
                clip_path: world.init_component::<ClipPath>(),
                background_image_mod: world.init_component::<BackgroundImageMod>(),
                border_image: world.init_component::<BorderImage>(),
                border_image_texture: world.init_component::<BorderImageTexture>(),
                border_image_clip: world.init_component::<BorderImageClip>(),
                border_image_slice: world.init_component::<BorderImageSlice>(),
                border_image_repeat: world.init_component::<BorderImageRepeat>(),
                border_radius: world.init_component::<BorderRadius>(),
                box_shadow: world.init_component::<BoxShadow>(),
                text_style: world.init_component::<TextStyle>(),
                text_shadow: world.init_component::<TextShadow>(),
                transform_will_change: world.init_component::<TransformWillChange>(),
                text_content: world.init_component::<TextContent>(),
                node_state: world.init_component::<NodeState>(),
                animation: world.init_component::<Animation>(),
				transition: world.init_component::<Transition>(),
                style_mark: world.init_component::<StyleMark>(),
                class_name: world.init_component::<ClassName>(),
                as_image: world.init_component::<AsImage>(),
                default: DefaultStyle::from_world(world),
                event: ChangeEvent::from_world(world),

				// dirty_list: world.components().get_resource_id(std::any::TypeId::of::<DirtyList>()).unwrap(),

				text_overflow: world.init_component::<TextOverflowData>(),
            }
        }
    }

    pub struct StyleQuery {
        pub size: ComponentId,
        pub margin: ComponentId,
        pub padding: ComponentId,
        pub border: ComponentId,
        pub position: ComponentId,
        pub min_max: ComponentId,
        pub flex_container: ComponentId,
        pub flex_normal: ComponentId,
        pub z_index: ComponentId,
        pub overflow: ComponentId,
        pub opacity: ComponentId,
        pub blend_mode: ComponentId,
        pub show: ComponentId,
        pub transform: ComponentId,
        pub background_color: ComponentId,
        pub border_color: ComponentId,
        pub background_image: ComponentId,
        pub background_image_texture: ComponentId,
        pub background_image_clip: ComponentId,
        pub mask_image: ComponentId,
        pub mask_image_clip: ComponentId,
        pub hsi: ComponentId,
        pub blur: ComponentId,
        pub clip_path: ComponentId,
        pub background_image_mod: ComponentId,
        pub border_image: ComponentId,
        pub border_image_texture: ComponentId,
        pub border_image_clip: ComponentId,
        pub border_image_slice: ComponentId,
        pub border_image_repeat: ComponentId,
        pub border_radius: ComponentId,
        pub box_shadow: ComponentId,
        pub text_style: ComponentId,
        pub text_shadow: ComponentId,
        pub transform_will_change: ComponentId,
        pub text_content: ComponentId,
        pub node_state: ComponentId,
        pub animation: ComponentId,
		pub transition: ComponentId,
        pub style_mark: ComponentId,
        pub class_name: ComponentId,
        pub as_image: ComponentId,

        pub default: DefaultStyle,

        pub event: ChangeEvent,

		// pub dirty_list: ComponentId,

		pub text_overflow: ComponentId,
    }

    pub struct DefaultStyle {
        pub size: ComponentId,
        pub margin: ComponentId,
        pub padding: ComponentId,
        pub border: ComponentId,
        pub position: ComponentId,
        pub min_max: ComponentId,
        pub flex_container: ComponentId,
        pub flex_normal: ComponentId,
        pub z_index: ComponentId,
        pub overflow: ComponentId,
        pub opacity: ComponentId,
        pub blend_mode: ComponentId,
        pub show: ComponentId,
        pub transform: ComponentId,
        pub background_color: ComponentId,
        pub border_color: ComponentId,
        pub background_image: ComponentId,
        pub background_image_clip: ComponentId,
        pub mask_image: ComponentId,
        pub mask_image_clip: ComponentId,
        pub hsi: ComponentId,
        pub blur: ComponentId,
        pub clip_path: ComponentId,
        pub background_image_mod: ComponentId,
        pub border_image: ComponentId,
        pub border_image_clip: ComponentId,
        pub border_image_slice: ComponentId,
        pub border_image_repeat: ComponentId,
        pub border_radius: ComponentId,
        pub box_shadow: ComponentId,
        pub text_style: ComponentId,
        pub text_shadow: ComponentId,
        pub transform_will_change: ComponentId,
        pub text_content: ComponentId,
        pub animation: ComponentId,
		pub transition: ComponentId,
        pub node_state: ComponentId,
        pub as_image: ComponentId,
		pub text_overflow: ComponentId,
    }

    impl FromWorld for DefaultStyle {
        fn from_world(world: &mut World) -> Self {
            Self {
                size: {
                    world.init_resource::<DefaultComponent<Size>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Size>>())
                        .unwrap()
                },
                margin: {
                    world.init_resource::<DefaultComponent<Margin>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Margin>>())
                        .unwrap()
                },
                padding: {
                    world.init_resource::<DefaultComponent<Padding>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Padding>>())
                        .unwrap()
                },
                border: {
                    world.init_resource::<DefaultComponent<Border>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Border>>())
                        .unwrap()
                },
                position: {
                    world.init_resource::<DefaultComponent<Position>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Position>>())
                        .unwrap()
                },
                min_max: {
                    world.init_resource::<DefaultComponent<MinMax>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<MinMax>>())
                        .unwrap()
                },
                flex_container: {
                    world.init_resource::<DefaultComponent<FlexContainer>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<FlexContainer>>())
                        .unwrap()
                },
                flex_normal: {
                    world.init_resource::<DefaultComponent<FlexNormal>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<FlexNormal>>())
                        .unwrap()
                },
                z_index: {
                    world.init_resource::<DefaultComponent<ZIndex>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<ZIndex>>())
                        .unwrap()
                },
                overflow: {
                    world.init_resource::<DefaultComponent<Overflow>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Overflow>>())
                        .unwrap()
                },
                opacity: {
                    world.init_resource::<DefaultComponent<Opacity>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Opacity>>())
                        .unwrap()
                },
                blend_mode: {
                    world.init_resource::<DefaultComponent<BlendMode>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BlendMode>>())
                        .unwrap()
                },
                show: {
                    world.init_resource::<DefaultComponent<Show>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Show>>())
                        .unwrap()
                },
                transform: {
                    world.init_resource::<DefaultComponent<Transform>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Transform>>())
                        .unwrap()
                },
                background_color: {
                    world.init_resource::<DefaultComponent<BackgroundColor>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundColor>>())
                        .unwrap()
                },
                border_color: {
                    world.init_resource::<DefaultComponent<BorderColor>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderColor>>())
                        .unwrap()
                },
                background_image: {
                    world.init_resource::<DefaultComponent<BackgroundImage>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImage>>())
                        .unwrap()
                },
                background_image_clip: {
                    world.init_resource::<DefaultComponent<BackgroundImageClip>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImageClip>>())
                        .unwrap()
                },
                mask_image: {
                    world.init_resource::<DefaultComponent<MaskImage>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<MaskImage>>())
                        .unwrap()
                },
                mask_image_clip: {
                    world.init_resource::<DefaultComponent<MaskImageClip>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<MaskImageClip>>())
                        .unwrap()
                },
                hsi: {
                    world.init_resource::<DefaultComponent<Hsi>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Hsi>>())
                        .unwrap()
                },
                blur: {
                    world.init_resource::<DefaultComponent<Blur>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Blur>>())
                        .unwrap()
                },
                clip_path: {
                    world.init_resource::<DefaultComponent<ClipPath>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<ClipPath>>())
                        .unwrap()
                },
                background_image_mod: {
                    world.init_resource::<DefaultComponent<BackgroundImageMod>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BackgroundImageMod>>())
                        .unwrap()
                },
                border_image: {
                    world.init_resource::<DefaultComponent<BorderImage>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImage>>())
                        .unwrap()
                },
                border_image_clip: {
                    world.init_resource::<DefaultComponent<BorderImageClip>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageClip>>())
                        .unwrap()
                },
                border_image_slice: {
                    world.init_resource::<DefaultComponent<BorderImageSlice>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageSlice>>())
                        .unwrap()
                },
                border_image_repeat: {
                    world.init_resource::<DefaultComponent<BorderImageRepeat>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderImageRepeat>>())
                        .unwrap()
                },
                border_radius: {
                    world.init_resource::<DefaultComponent<BorderRadius>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BorderRadius>>())
                        .unwrap()
                },
                box_shadow: {
                    world.init_resource::<DefaultComponent<BoxShadow>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<BoxShadow>>())
                        .unwrap()
                },
                text_style: {
                    world.init_resource::<DefaultComponent<TextStyle>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextStyle>>())
                        .unwrap()
                },
                text_shadow: {
                    world.init_resource::<DefaultComponent<TextShadow>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextShadow>>())
                        .unwrap()
                },
                transform_will_change: {
                    world.init_resource::<DefaultComponent<TransformWillChange>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<TransformWillChange>>())
                        .unwrap()
                },
                text_content: {
                    world.init_resource::<DefaultComponent<TextContent>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextContent>>())
                        .unwrap()
                },
                animation: {
                    world.init_resource::<DefaultComponent<Animation>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Animation>>())
                        .unwrap()
                },
				transition: {
                    world.init_resource::<DefaultComponent<Transition>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<Transition>>())
                        .unwrap()
                },
                node_state: {
                    world.init_resource::<DefaultComponent<NodeState>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<NodeState>>())
                        .unwrap()
                },
                as_image: {
                    world.init_resource::<DefaultComponent<AsImage>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<AsImage>>())
                        .unwrap()
                },
				text_overflow: {
                    world.init_resource::<DefaultComponent<TextOverflowData>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<DefaultComponent<TextOverflowData>>())
                        .unwrap()
                },
            }
        }
    }

    pub struct ChangeEvent {
        pub text_content: ComponentId,
        pub text_shadow: ComponentId,
        pub box_shadow: ComponentId,
        pub background_color: ComponentId,
        pub border_color: ComponentId,
        pub canvas: ComponentId,
		pub transform_will_change: ComponentId,
    }

    impl FromWorld for ChangeEvent {
        fn from_world(world: &mut World) -> Self {
            Self {
                text_content: {
                    world.init_resource::<Events<ComponentEvent<Changed<TextContent>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<TextContent>>>>())
                        .unwrap()
                },
                text_shadow: {
                    world.init_resource::<Events<ComponentEvent<Changed<TextShadow>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<TextShadow>>>>())
                        .unwrap()
                },
                box_shadow: {
                    world.init_resource::<Events<ComponentEvent<Changed<BoxShadow>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BoxShadow>>>>())
                        .unwrap()
                },
                background_color: {
                    world.init_resource::<Events<ComponentEvent<Changed<BackgroundColor>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BackgroundColor>>>>())
                        .unwrap()
                },
                border_color: {
                    world.init_resource::<Events<ComponentEvent<Changed<BorderColor>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<BorderColor>>>>())
                        .unwrap()
                },
                canvas: {
                    world.init_resource::<Events<ComponentEvent<Changed<Canvas>>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentEvent<Changed<Canvas>>>>())
                        .unwrap()
                },
				transform_will_change: {
                    world.init_resource::<Events<ComponentRemove<TransformWillChange>>>();
                    world
                        .components()
                        .get_resource_id(std::any::TypeId::of::<Events<ComponentRemove<TransformWillChange>>>())
                        .unwrap()
                },
            }
        }
    }

    // pub struct DefaultStyle {
    //     pub size: ResMut<'a, DefaultComponent<Size>>,
    //     pub margin: ResMut<'a, DefaultComponent<Margin>>,
    //     pub padding: ResMut<'a, DefaultComponent<Padding>>,
    //     pub border: ResMut<'a, DefaultComponent<Border>>,
    //     pub position: ResMut<'a, DefaultComponent<Position>>,
    //     pub min_max: ResMut<'a, DefaultComponent<MinMax>>,
    //     pub flex_container: ResMut<'a, DefaultComponent<FlexContainer>>,
    //     pub flex_normal: ResMut<'a, DefaultComponent<FlexNormal>>,
    //     pub z_index: ResMut<'a, DefaultComponent<ZIndex>>,
    //     pub overflow: ResMut<'a, DefaultComponent<Overflow>>,
    //     pub opacity: ResMut<'a, DefaultComponent<Opacity>>,
    //     pub blend_mode: ResMut<'a, DefaultComponent<BlendMode>>,
    //     pub show: ResMut<'a, DefaultComponent<Show>>,
    //     pub transform: ResMut<'a, DefaultComponent<Transform>>,
    //     pub background_color: ResMut<'a, DefaultComponent<BackgroundColor>>,
    //     pub border_color: ResMut<'a, DefaultComponent<BorderColor>>,
    //     pub background_image: ResMut<'a, DefaultComponent<BackgroundImage>>,
    //     pub background_image_clip: ResMut<'a, DefaultComponent<BackgroundImageClip>>,
    //     pub mask_image: ResMut<'a, DefaultComponent<MaskImage>>,
    //     pub mask_image_clip: ResMut<'a, DefaultComponent<MaskImageClip>>,
    //     pub hsi: ResMut<'a, DefaultComponent<Hsi>>,
    //     pub blur: ResMut<'a, DefaultComponent<Blur>>,
    //     pub background_image_mod: ResMut<'a, DefaultComponent<BackgroundImageMod>>,
    //     pub border_image: ResMut<'a, DefaultComponent<BorderImage>>,
    //     pub border_image_clip: ResMut<'a, DefaultComponent<BorderImageClip>>,
    //     pub border_image_slice: ResMut<'a, DefaultComponent<BorderImageSlice>>,
    //     pub border_image_repeat: ResMut<'a, DefaultComponent<BorderImageRepeat>>,
    //     pub border_radius: ResMut<'a, DefaultComponent<BorderRadius>>,
    //     pub box_shadow: ResMut<'a, DefaultComponent<BoxShadow>>,
    //     pub text_style: ResMut<'a, DefaultComponent<TextStyle>>,
    //     pub transform_will_change: ResMut<'a, DefaultComponent<TransformWillChange>>,
    //     pub text_content: ResMut<'a, DefaultComponent<TextContent>>,
    //     pub animation: ResMut<'a, DefaultComponent<Animation>>,
    // 	pub node_state: ResMut<'a, DefaultComponent<NodeState>>,
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
            cur_style_mark: &mut BitArray<[u32; 3]>,
            style_index: u8,
            buffer: &Vec<u8>,
            offset: usize,
            query: &mut Setting,
            entity: Entity,
            is_clone: bool,
        ) {
			if style_index > 96 {
				(RESET_STYLE_ATTR[style_index as usize - 96].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone)
			} else {
				(STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, is_clone)
			}
            
        }

		pub fn get(
            style_index: u8,
            query: &mut Setting,
            entity: Entity,
        ) -> Option<Attribute> {
            (STYLE_ATTR[style_index as usize].get)(query, entity)
        }

        #[inline]
        pub fn to_attr(style_index: u8, buffer: &Vec<u8>, offset: usize) -> Attribute {
            (STYLE_ATTR[style_index as usize].to_attr)(unsafe { buffer.as_ptr().add(offset) })
        }

        #[inline]
        pub fn size(style_index: u8) -> usize { 
			if style_index > 96 {
				0
			} else {
				(STYLE_ATTR[style_index as usize].size)()
			}
		 }

        #[inline]
        pub fn reset(cur_style_mark: &mut BitArray<[u32; 3]>, style_index: u8, buffer: &Vec<u8>, offset: usize, query: &mut Setting, entity: Entity) {
            (RESET_STYLE_ATTR[style_index as usize].set)(cur_style_mark, unsafe { buffer.as_ptr().add(offset) }, query, entity, false);
        }

        #[inline]
        pub fn set_default(style_index: u8, buffer: &Vec<u8>, offset: usize, query: &DefaultStyle, world: &mut World) {
            (STYLE_ATTR[style_index as usize].set_default)(buffer, offset, query, world);
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