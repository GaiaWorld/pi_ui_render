use std::mem::transmute;

use js_proxy_gen_macro::pi_js_export;
use pi_bevy_ecs_extend::prelude::Down;
use pi_bevy_ecs_extend::prelude::Up;
use pi_flex_layout::prelude::*;
use pi_null::Null;
use pi_style::style_parse::Attribute;
use serde::{Deserialize, Serialize};

use pi_style::style::Point2;

use crate::components::calc::ContentBox;
use crate::components::calc::IsShow;
use crate::components::calc::LayoutResult;
use crate::components::calc::WorldMatrix;
use crate::components::calc::{DrawList, EntityKey, ZRange};
use crate::components::draw_obj::PipelineMeta;
use crate::components::user::serialize::StyleTypeReader;
use crate::components::user::Vector4;
use crate::components::user::*;
use crate::components::user::{Overflow, Size};
pub use crate::export::{Engine};
use crate::resource::ClassSheet;
use bevy::ecs::prelude::Entity;
use pi_map::vecmap::VecMap;
use pi_style::style::ImageRepeatOption;

#[derive(Serialize, Deserialize, Debug)]
struct Quad {
    pub left_top: Point2,
    pub left_bottom: Point2,
    pub right_bottom: Point2,
    pub right_top: Point2,
}

#[derive(Serialize, Deserialize, Debug)]
struct Layout1 {
    rect: Rect<f32>,
    border: Rect<f32>,
    padding: Rect<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    pub overflow: bool,
    // pub by_overflow: usize,
    pub visibility: bool,
    pub enable: bool,
    pub opacity: f32,
    pub blur: f32,
    pub zindex: isize,
    pub zdepth: f32,
    pub layout: Layout1,
    pub border_box: Quad,
    pub padding_box: Quad,
    pub content_box: Quad,
    // pub culling: bool,
    // char_block: Option<CharBlock1>,
    pub class_name: Option<ClassName>,
    pub image: Option<String>,
    pub mask_image: Option<MaskImage>,
    pub border_image: Option<String>,
    // pub render_context: bool,
    pub background_color: Option<BackgroundColor>,
    pub border_color: Option<BorderColor>,
    pub transform: Option<Transform>,
    pub box_shadow: Option<BoxShadow>,
    pub border_image_clip: Option<BorderImageClip>,
    pub border_image_slice: Option<BorderImageSlice>,
    pub border_image_repeat: Option<BorderImageRepeat>,
    pub image_clip: Option<BackgroundImageClip>,
    pub mask_image_clip: Option<MaskImageClip>,
    pub border_radius: Option<BorderRadius>,
    pub object_fit: Option<FitType>,
    pub background_repeat: Option<ImageRepeat>,
    pub filter: Option<Hsi>,
    pub transform_will_change: Option<TransformWillChange>,
    pub parent_id: Option<f64>,
    pub content_bound_box: Option<ContentBox>,
    pub quad: Option<crate::components::calc::Quad>,

    text: Option<TextStyle>,
    text_content: Option<TextContent>,
    // style_mark: StyleMark,
    children: Vec<f64>,
    pub render_obj: Vec<RenderObject>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RenderObject {
    // pub depth: f32,
    // pub depth_diff: f32,
    // pub visibility: bool,
    // pub is_opacity: bool,
    // pub vs_name: String,
    // pub fs_name: String,
    // pub vs_defines: Vec<String>,
    // pub fs_defines: Vec<String>,
    // // pub paramter: XHashMap<String, Paramter>,
    // pub program_dirty: bool,

    // pub program: bool,
    // pub geometry: bool,
    // // pub state: State,

    // pub context: usize,
    // pub post: bool,
    // pub post_copy: usize,
    pub id: String,
    pub name: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// enum Paramter {
//     Uniform(UniformValue),
//     Ubo(XHashMap<String, UniformValue>),
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct State {
//     pub rs: RasterStateDesc,
//     pub bs: BlendStateDesc,
//     pub ss: StencilStateDesc,
//     pub ds: DepthStateDesc,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct RasterStateDesc {
    // pub cull_mode: Option<CullMode>,
    pub is_front_face_ccw: bool,
    pub polygon_offset: (f32, f32),
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BlendStateDesc {
//     pub rgb_equation: BlendFunc,
//     pub alpha_equation: BlendFunc,

//     pub src_rgb_factor: BlendFactor,
//     pub dst_rgb_factor: BlendFactor,

//     pub src_alpha_factor: BlendFactor,
//     pub dst_alpha_factor: BlendFactor,

//     pub const_rgba: (f32, f32, f32, f32),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct OverflowClip {
//     pub id_map: XHashMap<usize, usize>,
//     pub clip: Vec<(usize, Clip)>,
//     pub clip_map: XHashMap<usize, Aabb2>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CharBlock1 {
    pub font_size: f32,    // 字体高度
    pub font_height: f32,  // 字体高度
    pub stroke_width: f32, //描边宽度
    pub line_height: f32,
    pub chars: Vec<CharNode>,            // 字符集合
    pub lines: Vec<(usize, usize, f32)>, // 不折行下的每行的起始字符位置、单词数量和总宽度。 自动折行不会影响该值
    pub last_line: (usize, usize, f32),  // 最后一行的起始字符位置、单词数量和总宽度
    pub size: Vector2,
    pub wrap_size: Vector2,
    pub pos: Point2,
    pub line_count: usize,  // 行数，
    pub fix_width: bool,    // 如果有字宽不等于font_size
    pub style_class: usize, // 使用的那个样式类
    pub is_pixel: bool,
}

// 字符节点， 对应一个字符的
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharNode {
    pub ch: char,              // 字符
    pub width: f32,            // 字符宽度
    pub pos: Point2,           // 位置
    pub ch_id_or_count: usize, // 字符id或单词的字符数量
    pub base_width: f32,       // font_size 为32 的字符宽度
}

#[allow(unused_attributes)]
#[pi_js_export]
pub fn get_layout(engine: &mut Engine, node_id: f64) -> String {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });

    let mut query = engine.world.query::<(
        Option<&NodeState>,
        Option<&Size>,
        Option<&Margin>,
        Option<&Padding>,
        Option<&Border>,
        Option<&Position>,
        Option<&MinMax>,
        Option<&FlexContainer>,
        Option<&FlexNormal>,
        Option<&Show>,
        Option<&LayoutResult>,
    )>();
    let (node_state, size, margin, padding, border, position, minmax, flex_container, flex_normal, show, layout_ret) =
        query.get(&engine.world, node_id).unwrap();

	serde_json::to_string(&Layout {
        size: size.map(|r| r.clone()),
        margin: margin.map(|r| r.clone()),
        padding: padding.map(|r| r.clone()),
        border: border.map(|r| r.clone()),
        position: position.map(|r| r.clone()),
        minmax: minmax.map(|r| r.clone()),
        flex_container: flex_container.map(|r| r.clone()),
        flex_normal: flex_normal.map(|r| r.clone()),
        show: show.map(|r| r.clone()),
        node_state: node_state.map(|r| r.clone()),
        is_vnode: node_state.map_or(false, |r| r.0.is_vnode()),
        layout_ret: layout_ret.map(|r| r.clone()),
    }).unwrap()
}


// #[pi_js_export]
// pub fn get_layout(gui: &Gui, node_id: f64) {
//     let node_id = Entity::from_bits(unsafe {transmute(node_id)});
//
//
// 	let rect_layout_style = gui.gui.rect_layout_style.lend();
// 	let other_layout_style = gui.gui.other_layout_style.lend();
// 	let layouts = gui.gui.layout.lend();

// 	unsafe{
// 		console::log_2(&"rect_style:".into(), &format!("{:?}", rect_layout_style.get(node_id)).into());
// 		console::log_2(&"other_style:".into(),&format!("{:?}", other_layout_style.get(node_id)).into());
// 		console::log_2(&"layout:".into(), &format!("{:?}", layouts.get(node_id)).into());
// 		console::log_2(&"node_state:".into(), &format!("{:?}", gui.gui.node_state.lend().get(node_id)).into());
// 	}
// }


#[pi_js_export]
pub fn get_class_name(engine: &mut Engine, node_id: f64) -> String {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });
	serde_json::to_string(&&match engine.world.query::<&ClassName>().get(&engine.world, node_id) {
        Ok(r) => Some(r),
        _ => None,
    }).unwrap()
}

#[allow(unused_attributes)]
#[pi_js_export]
pub fn get_class(engine: &mut Engine, class_name: u32) -> String {
    let class = match engine.world.get_resource::<ClassSheet>() {
		Some(class_sheet) if let Some(class) = class_sheet.class_map.get(&(class_name as usize)) => {
			let mut ret = "".to_string();
            // println!("set class1==========={}", i);
            let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
            while let Some(r) = style_reader.to_attr() {
                let s = to_css_str(r);
                if s.as_str() != "" {
                    ret += (s + ";").as_str();
                }
            }
            Some(ret)
		},
		_ => None
	};

	serde_json::to_string(&class).unwrap()
}

fn to_css_str(attr: Attribute) -> String {
    match attr {
        Attribute::ClipPath(_) => todo!(),
        Attribute::PositionType(r) => match r.0 {
            PositionType::Relative => "position:relative".to_string(),
            PositionType::Absolute => "position:absolute".to_string(),
        },
        Attribute::FlexWrap(r) => match r.0 {
            FlexWrap::NoWrap => "flex-wrap:nowrap".to_string(),
            FlexWrap::Wrap => "flex-wrap:wrap".to_string(),
            FlexWrap::WrapReverse => "flex-wrap:wrapreverse".to_string(),
        },
        Attribute::FlexDirection(r) => match r.0 {
            FlexDirection::Column => "flex-direction:column".to_string(),
            FlexDirection::ColumnReverse => "flex-direction:columnreverse".to_string(),
            FlexDirection::Row => "flex-direction:row".to_string(),
            FlexDirection::RowReverse => "flex-direction:rowreverse".to_string(),
        },
        Attribute::AlignContent(r) => match r.0 {
            // AlignContent::Auto => "align-content:auto".to_string(),
            AlignContent::FlexStart => "align-content:flex-start".to_string(),
            AlignContent::Center => "align-content:center".to_string(),
            AlignContent::FlexEnd => "align-content:flex-end".to_string(),
            AlignContent::Stretch => "align-content:stretch".to_string(),
            // AlignContent::Baseline => "align-content:baseline".to_string(),
            AlignContent::SpaceBetween => "align-content:space-between".to_string(),
            AlignContent::SpaceAround => "align-content:space-around".to_string(),
        },
        Attribute::AlignItems(r) => match r.0 {
            // AlignItems::Auto => "align-items:auto".to_string(),
            AlignItems::FlexStart => "align-items:flex-start".to_string(),
            AlignItems::Center => "align-items:center".to_string(),
            AlignItems::FlexEnd => "align-items:flex-end".to_string(),
            AlignItems::Stretch => "align-items:stretch".to_string(),
            AlignItems::Baseline => "align-items:baseline".to_string(),
            // AlignItems::SpaceBetween => "align-items:space-between".to_string(),
            // AlignItems::SpaceAround => "align-items:space-around".to_string(),
        },
        Attribute::AlignSelf(r) => match r.0 {
            AlignSelf::Auto => "align-self:auto".to_string(),
            AlignSelf::FlexStart => "align-self:flex-start".to_string(),
            AlignSelf::Center => "align-self:center".to_string(),
            AlignSelf::FlexEnd => "align-self:flex-end".to_string(),
            AlignSelf::Stretch => "align-self:stretch".to_string(),
            AlignSelf::Baseline => "align-self:baseline".to_string(),
            // AlignSelf::SpaceBetween => "align-self:space-between".to_string(),
            // AlignSelf::SpaceAround => "align-self:space-around".to_string(),
        },
        Attribute::JustifyContent(r) => match r.0 {
            JustifyContent::FlexStart => "justify-content:flex-start".to_string(),
            JustifyContent::Center => "justify-content:center".to_string(),
            JustifyContent::FlexEnd => "justify-content:flex-end".to_string(),
            JustifyContent::SpaceBetween => "justify-content:space-between".to_string(),
            JustifyContent::SpaceAround => "justify-content:space-around".to_string(),
            JustifyContent::SpaceEvenly => "justify-content:space-evenly".to_string(),
        },

        Attribute::ObjectFit(r) => match r.0 {
            FitType::None => "object-fit:none".to_string(),
            FitType::Fill => "object-fit:fill".to_string(),
            FitType::Contain => "object-fit:contain".to_string(),
            FitType::Cover => "object-fit:cover".to_string(),
            FitType::ScaleDown => "object-fit:scale-down".to_string(),
            // FitType::Repeat => "object-fit:repeat".to_string(),
            // FitType::RepeatX => "object-fit:repeat-x".to_string(),
            // FitType::RepeatY => "object-fit:repeat-y".to_string(),
        },

        Attribute::BackgroundRepeat(r) => {
            "background-repeat".to_string()
                + match r.x {
                    ImageRepeatOption::Stretch => "stretch ",
                    ImageRepeatOption::Repeat => "repeat ",
                    ImageRepeatOption::Round => "round ",
                    ImageRepeatOption::Space => "space ",
                }
                + match r.y {
                    ImageRepeatOption::Stretch => "stretch",
                    ImageRepeatOption::Repeat => "repeat",
                    ImageRepeatOption::Round => "round",
                    ImageRepeatOption::Space => "space",
                }
        }
        Attribute::TextAlign(r) => match r.0 {
            TextAlign::Left => "text-align:left".to_string(),
            TextAlign::Right => "text-align:right".to_string(),
            TextAlign::Center => "text-align:center".to_string(),
            TextAlign::Justify => "text-align:justify".to_string(),
        },
        Attribute::VerticalAlign(r) => match r.0 {
            VerticalAlign::Top => "vertical-align:top".to_string(),
            VerticalAlign::Middle => "vertical-align:middle".to_string(),
            VerticalAlign::Bottom => "vertical-align:bottom".to_string(),
        },
        Attribute::WhiteSpace(r) => match r.0 {
            WhiteSpace::Normal => "white-space:normal".to_string(),
            WhiteSpace::Nowrap => "white-space:nowrap".to_string(),
            WhiteSpace::PreWrap => "white-space:pre-wrap".to_string(),
            WhiteSpace::Pre => "white-space:pre".to_string(),
            WhiteSpace::PreLine => "white-space:pre-line".to_string(),
        },
        Attribute::FontStyle(r) => match r.0 {
            FontStyle::Normal => "font-style:normal".to_string(),
            FontStyle::Ttalic => "font-style:ttalic".to_string(),
            FontStyle::Oblique => "font-style:oblique".to_string(),
        },
        Attribute::Enable(r) => match r.0 {
            pi_style::style::Enable::Auto => "enable:auto".to_string(),
            pi_style::style::Enable::None => "enable:none".to_string(),
            pi_style::style::Enable::Visible => "enable:visible".to_string(),
        },
        Attribute::Display(r) => match r.0 {
            Display::Flex => "display:flex".to_string(),
            Display::None => "display:none".to_string(),
        },
        Attribute::Visibility(r) => match r.0 {
            true => "visibility:visible".to_string(),
            false => "visibility:hidden".to_string(),
        },
        Attribute::Overflow(r) => match r.0 {
            true => "overflow:hidden".to_string(),
            false => "overflow:visible".to_string(),
        },
        Attribute::LetterSpacing(r) => "letter-spacing:".to_string() + r.to_string().as_str(),
        Attribute::LineHeight(r) => match r.0 {
            LineHeight::Normal => "line-height:normal".to_string(),
            LineHeight::Length(r) => "line-height:".to_string() + r.to_string().as_str() + "px",
            LineHeight::Number(r) => "line-height:".to_string() + r.to_string().as_str(),
            LineHeight::Percent(r) => "line-height:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::TextIndent(r) => "text-indent:".to_string() + r.to_string().as_str() + "px",
        Attribute::WordSpacing(r) => "word-space:".to_string() + r.to_string().as_str() + "px",
        Attribute::FontWeight(r) => "font-weight:".to_string() + r.to_string().as_str(),
        Attribute::FontSize(_r) => "".to_string(), // TODO
        Attribute::FontFamily(r) => "font-family:".to_string() + r.to_string().as_str(),
        Attribute::ZIndex(r) => "z-index:".to_string() + r.to_string().as_str(),
        Attribute::Opacity(r) => "opacity:".to_string() + r.0.to_string().as_str(),
        // Attribute::BorderImageRepeat(BorderImageRepeat)(x, y) => "border-image-repeat:" + r.to_string().as_str() + " " +,
        Attribute::BackgroundImage(r) => "src:".to_string() + r.to_string().as_str(),
        Attribute::BorderImage(r) => "border-image-src:".to_string() + r.to_string().as_str(),

        Attribute::FlexShrink(r) => "flex-shrink:".to_string() + r.to_string().as_str(),
        Attribute::FlexGrow(r) => "flex-grow:".to_string() + r.to_string().as_str(),
        Attribute::Width(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "width:auto".to_string(),
            Dimension::Points(r) => "width:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "width:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::Height(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "height:auto".to_string(),
            Dimension::Points(r) => "height:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "height:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MarginLeft(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "margin-left:auto".to_string(),
            Dimension::Points(r) => "margin-left:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "margin-left:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MarginTop(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "margin-top:auto".to_string(),
            Dimension::Points(r) => "margin-top:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "margin-top:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MarginBottom(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "margin-bottom:auto".to_string(),
            Dimension::Points(r) => "margin-bottom:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "margin-bottom:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MarginRight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "margin-right:auto".to_string(),
            Dimension::Points(r) => "margin-right:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "margin-right:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PaddingLeft(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "padding-left:auto".to_string(),
            Dimension::Points(r) => "padding-left:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "padding-left:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PaddingTop(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "padding-top:auto".to_string(),
            Dimension::Points(r) => "padding-top:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "padding-top:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PaddingBottom(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "padding-bottom:auto".to_string(),
            Dimension::Points(r) => "padding-bottom:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "padding-bottom:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PaddingRight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "padding-right:auto".to_string(),
            Dimension::Points(r) => "padding-right:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "padding-right:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::BorderLeft(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "border-left:auto".to_string(),
            Dimension::Points(r) => "borderleft:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "borderleft:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::BorderTop(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "border-top:auto".to_string(),
            Dimension::Points(r) => "border-top:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "border-top:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::BorderBottom(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "border-bottom:auto".to_string(),
            Dimension::Points(r) => "border-bottom:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "border-bottom:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::BorderRight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "border-right:auto".to_string(),
            Dimension::Points(r) => "border-right:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "border-right:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        // Attribute::Border(r) => match r.0 {
        //     Dimension::Undefined => "".to_string(),
        //     Dimension::Auto => "width:auto".to_string(),
        //     Dimension::Points(r) => "width:".to_string() + r.to_string().as_str() + "px",
        //     Dimension::Percent(r) => "width:".to_string() + (r * 100.0).to_string().as_str() + "%",
        // },
        Attribute::MinWidth(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "min-width:auto".to_string(),
            Dimension::Points(r) => "min-width:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "min-width:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MinHeight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "min-height:auto".to_string(),
            Dimension::Points(r) => "min-height:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "min-height:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MaxHeight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "max-height:auto".to_string(),
            Dimension::Points(r) => "max-height:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "max-height:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::MaxWidth(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "max-width:auto".to_string(),
            Dimension::Points(r) => "max-width:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "max-width:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::FlexBasis(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "flex-basis:auto".to_string(),
            Dimension::Points(r) => "flex-basis:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "flex-basis:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PositionLeft(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "left:auto".to_string(),
            Dimension::Points(r) => "left:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "left:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PositionTop(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "top:auto".to_string(),
            Dimension::Points(r) => "top:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "top:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PositionRight(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "right:auto".to_string(),
            Dimension::Points(r) => "right:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "right:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::PositionBottom(r) => match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "bottom:auto".to_string(),
            Dimension::Points(r) => "bottom:".to_string() + r.to_string().as_str() + "px",
            Dimension::Percent(r) => "bottom:".to_string() + (r * 100.0).to_string().as_str() + "%",
        },
        Attribute::BackgroundColor(color) => match &color.0 {
            Color::RGBA(r) => {
                "background-color:rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "background-color:linear-gradient".to_string(),
        },

        Attribute::BorderColor(r) => {
            let r = r.0;
            "border-color:rgba(".to_string()
                + r.x.to_string().as_str()
                + ","
                + r.y.to_string().as_str()
                + ","
                + r.z.to_string().as_str()
                + ","
                + r.w.to_string().as_str()
                + ")"
        }
        Attribute::BoxShadow(r) => {
            "box-shadow:".to_string()
                + r.h.to_string().as_str()
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
        }

        Attribute::BackgroundImageClip(r) => {
            "image-clip:".to_string()
                + (r.top * 100.0).to_string().as_str()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }
        Attribute::MaskImageClip(r) => {
            "mask-image-clip:".to_string()
                + (r.top * 100.0).to_string().as_str()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }

        Attribute::BorderImageClip(r) => {
            "border-image-clip:".to_string()
                + (r.top * 100.0).to_string().as_str()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }
        Attribute::BorderImageSlice(r) => {
            let mut f = "";
            if r.fill {
                f = " fill";
            }
            "border-image-slice:".to_string()
                + (r.top * 100.0).to_string().as_str()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
                + f
        }

        Attribute::Color(r) => match r.0 {
            Color::RGBA(r) => {
                "color:rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "color:linear-gradient".to_string(),
        },
        Attribute::TextShadow(r) => {
            let mut rr = "text-shadow:".to_string();
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
        }
        Attribute::TextStroke(r) => {
            "text-stroke:".to_string()
                + " rgba("
                + r.0.color.x.to_string().as_str()
                + ","
                + r.0.color.y.to_string().as_str()
                + ","
                + r.0.color.z.to_string().as_str()
                + ","
                + r.0.color.w.to_string().as_str()
                + ")"
        }

        Attribute::BorderRadius(_r) => "".to_string(),    // TODO
        Attribute::TransformFunc(_r) => "".to_string(),   // TODO
        Attribute::TransformOrigin(_r) => "".to_string(), // TODO
        Attribute::Hsi(_r) => "".to_string(),
        Attribute::BorderImageRepeat(r) => "border-image-repeat:".to_string() + format!("{:?}", r.x).as_str() + " " + format!("{:?}", r.y).as_str(),
        Attribute::Blur(r) => "blur:".to_string() + r.0.to_string().as_str() + "px",
        Attribute::MaskImage(r) => "mask-image:".to_string() + format!("{:?}", r.0).as_str(),
        Attribute::Transform(r) => "".to_string(),               // TODO
        Attribute::TransformWillChange(r) => "".to_string(),     // TODO
        Attribute::BlendMode(r) => "".to_string(),               // TODO
        Attribute::Direction(r) => "".to_string(),               // TODO
        Attribute::AspectRatio(r) => "".to_string(),             // TODO
        Attribute::Order(r) => "".to_string(),                   // TODO
        Attribute::TextContent(r) => "".to_string(),             // TODO
        Attribute::VNode(r) => "".to_string(),                   // TODO
        Attribute::AnimationName(r) => "".to_string(),           // TODO
        Attribute::AnimationDuration(r) => "".to_string(),       // TODO
        Attribute::AnimationTimingFunction(r) => "".to_string(), // TODO
        Attribute::AnimationDelay(r) => "".to_string(),          // TODO
        Attribute::AnimationIterationCount(r) => "".to_string(), // TODO
        Attribute::AnimationDirection(r) => "".to_string(),      // TODO
        Attribute::AnimationFillMode(r) => "".to_string(),       // TODO
        Attribute::AnimationPlayState(r) => "".to_string(),      // TODO
    }
}

// 打印节点信息
#[allow(unused_attributes)]
#[pi_js_export]
pub fn node_info(engine: &mut Engine, node_id: f64) -> String {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });


    // let down = engine.world.query::<&Down<Node>>().get(&engine.world, node_id);
    // let up = engine.world.query::<&Up<Node>>().get(&engine.world, node_id);

    // let z_depth = unsafe { engine.world.z_depth.lend()[node_id]}.0;

    // let enable =  engine.world.query::<&crate::components::calc::IsEnable>().get(&engine.world, node_id);
    // let visibility =   engine.world.query::<&Visibility>().get(&engine.world, node_id);

    // let opacity =  engine.world.query::<&Opacity>().get(&engine.world, node_id);

    let layout = engine.world.query::<&LayoutResult>().get(&engine.world, node_id).unwrap().clone();

    let world_matrix = &engine.world.query::<&WorldMatrix>().get(&engine.world, node_id).unwrap().clone();

    // let transform =  engine.world.query::<&Transform>();

    // let draw_list =  engine.world.query::<&DrawList>();

    // let mask_image =  engine.world.query::<&MaskImage>();

    // let mask_image_clip =  engine.world.query::<&MaskImageClip>();

    // let content_boxs = engine.world.query::<&ContentBox>();

    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;
    // border box
    let b_left_top = world_matrix * Vector4::new(0.0, 0.0, 1.0, 1.0);
    let b_left_bottom = world_matrix * Vector4::new(0.0, height, 1.0, 1.0);
    let b_right_bottom = world_matrix * Vector4::new(width, height, 1.0, 1.0);
    let b_right_top = world_matrix * Vector4::new(width, 0.0, 1.0, 1.0);

    // border box
    let absolute_b_box = Quad {
        left_top: Point2::new(b_left_top.x, b_left_top.y),
        left_bottom: Point2::new(b_left_bottom.x, b_left_bottom.y),
        right_bottom: Point2::new(b_right_bottom.x, b_right_bottom.y),
        right_top: Point2::new(b_right_top.x, b_right_top.y),
    };

    // padding box
    let p_left_top = world_matrix * Vector4::new(layout.border.left, layout.border.top, 1.0, 1.0);
    let p_left_bottom = world_matrix * Vector4::new(layout.border.left, height - layout.border.bottom, 1.0, 1.0);
    let p_right_bottom = world_matrix * Vector4::new(width - layout.border.right, height - layout.border.bottom, 1.0, 1.0);
    let p_right_top = world_matrix * Vector4::new(width - layout.border.right, layout.border.top, 1.0, 1.0);

    let absolute_p_box = Quad {
        left_top: Point2::new(p_left_top.x, p_left_top.y),
        left_bottom: Point2::new(p_left_bottom.x, p_left_bottom.y),
        right_bottom: Point2::new(p_right_bottom.x, p_right_bottom.y),
        right_top: Point2::new(p_right_top.x, p_right_top.y),
    };

    // content box
    let c_left_top = world_matrix * Vector4::new(layout.border.left + layout.padding.left, layout.border.top + layout.padding.top, 1.0, 1.0);
    let c_left_bottom = world_matrix
        * Vector4::new(
            layout.border.left + layout.padding.left,
            height - layout.border.bottom - layout.padding.bottom,
            1.0,
            1.0,
        );
    let c_right_bottom = world_matrix
        * Vector4::new(
            width - layout.border.right - layout.padding.right,
            height - layout.border.bottom - layout.padding.bottom,
            1.0,
            1.0,
        );
    let c_right_top = world_matrix
        * Vector4::new(
            width - layout.border.right - layout.padding.right,
            layout.border.top + layout.padding.top,
            1.0,
            1.0,
        );

    let absolute_c_box = Quad {
        left_top: Point2::new(c_left_top.x, c_left_top.y),
        left_bottom: Point2::new(c_left_bottom.x, c_left_bottom.y),
        right_bottom: Point2::new(c_right_bottom.x, c_right_bottom.y),
        right_top: Point2::new(c_right_top.x, c_right_top.y),
    };

    // let yogas = gui.gui.yoga.lend();
    // let yoga = yogas[node_id];

    // let octs = gui.gui.oct.lend();
    // let oct = octs[node_id];

    let draw_list = match engine.world.query::<&DrawList>().get(&engine.world, node_id) {
        Ok(r) => r.0.clone(),
        _ => VecMap::default(),
    };

    let mut draw_objs = Vec::new();
    for i in draw_list.iter() {
        if let Some(i) = i {
            if let Ok(pipeline_meta) = engine.world.query::<&PipelineMeta>().get(&engine.world, i.clone()) {
                draw_objs.push(RenderObject {
                    id: format!("{:?}", i),
                    name: pipeline_meta.program.shader_meta.name.clone(),
                });
            }
        }
    }
    let mut children = Vec::new();

    let mut down_list = engine.world.query::<&Down>();
    if let Ok(down) = down_list.get(&engine.world, node_id) {
        let mut n = down.head();
        let mut up = engine.world.query::<&Up>();
        while !EntityKey(n).is_null() {
            children.push(unsafe { transmute::<_, f64>(n) });
            n = match up.get(&engine.world, n) {
                Ok(r) => r.next(),
                _ => break,
            };
        }
    }
    let mut up = engine.world.query::<&Up>();
    let parent = match up.get(&engine.world, node_id) {
        Ok(r) => r.parent(),
        __ => EntityKey::null().0,
    };

    let mut query = engine.world.query::<(
        (
            Option<&Overflow>,
            Option<&IsShow>,
            Option<&MaskImage>,
            Option<&MaskImageClip>,
            Option<&Blur>,
            Option<&ZIndex>,
            Option<&ZRange>,
            Option<&ContentBox>,
            Option<&crate::components::calc::Quad>,
            Option<&TextStyle>,
            Option<&TextContent>,
            Option<&ClassName>,
            Option<&BackgroundImage>,
            Option<&BorderImage>,
            Option<&BackgroundColor>,
        ),
        (
            Option<&BorderColor>,
            Option<&Opacity>,
            Option<&Transform>,
            Option<&BoxShadow>,
            Option<&BorderImageClip>,
            Option<&BorderImageSlice>,
            Option<&BorderImageRepeat>,
            Option<&BackgroundImageClip>,
            Option<&BorderRadius>,
            Option<&BackgroundImageMod>,
            Option<&Hsi>,
            Option<&TransformWillChange>,
        ),
    )>();
    let (
        (
            overflow,
            is_show,
            mask_image,
            mask_image_clip,
            blur,
            zindex,
            z_range,
            content_box,
            quad,
            text_style,
            text_content,
            class_name,
            background_image,
            border_image,
            background_color,
        ),
        (
            border_color,
            opacity,
            transform,
            box_shadow,
            border_image_clip,
            border_image_slice,
            border_image_repeat,
            background_image_clip,
            border_radius,
            background_image_mod,
            hsi,
            transform_will_change,
        ),
    ) = query.get(&engine.world, node_id).unwrap();

    let info = Info {
        // char_block: char_block,
        overflow: overflow.map_or(false, |r| r.0),
        // by_overflow: by_overflow,
        visibility: is_show.map_or(false, |r| r.get_visibility()),
        enable: is_show.map_or(false, |r| r.get_enable()),
        mask_image: mask_image.map(|r| r.clone()),
        mask_image_clip: mask_image_clip.map(|r| r.clone()),
        // context_mark: match context_marks.get(node_id) {
        //     Some(r) => Some(r.clone()),
        //     None => None,
        // },
        // render_context: match render_contexts {
        //     Some(r) => true,
        //     None => false,
        // },
        opacity: opacity.map_or(1.0, |r| r.0),
        blur: blur.map_or(0.0, |r| r.0),
        zindex: zindex.map_or(0, |r| r.0),
        zdepth: z_range.map_or(0.0, |r| r.start as f32),
        layout: unsafe { transmute(layout.clone()) },
        border_box: absolute_b_box,
        padding_box: absolute_p_box,
        content_box: absolute_c_box,
        content_bound_box: content_box.map(|r| r.clone()),
        quad: quad.map(|r| r.clone()),
        // culling: gui.gui.culling.lend()[node_id].0,
        text: text_style.map(|r| r.clone()),
        text_content: text_content.map(|r| r.clone()),
        render_obj: draw_objs,
        class_name: class_name.map(|r| r.clone()),
        image: background_image.map(|r| r.0.as_str().to_string()),
        border_image: border_image.map(|r| r.0.as_str().to_string()),
        background_color: background_color.map(|r| r.clone()),
        border_color: border_color.map(|r| r.clone()),
        transform: transform.map(|r| r.clone()),
        box_shadow: box_shadow.map(|r| r.clone()),
        border_image_clip: border_image_clip.map(|r| r.clone()),
        border_image_slice: border_image_slice.map(|r| r.clone()),
        border_image_repeat: border_image_repeat.map(|r| r.clone()),
        image_clip: background_image_clip.map(|r| r.clone()),
        border_radius: border_radius.map(|r| r.clone()),
        object_fit: background_image_mod.map(|r| r.object_fit.clone()),
        background_repeat: background_image_mod.map(|r| r.repeat.clone()),
        filter: hsi.map(|r| r.clone()),
        // style_mark: gui.gui.style_mark.lend()[node_id],
        transform_will_change: transform_will_change.map(|r| r.clone()),
        parent_id: Some(unsafe { transmute::<_, f64>(parent) }),
        children: children,
    };

	serde_json::to_string(&info).unwrap()
}

// #[allow(unused_attributes)]
// #[pi_js_export]
// pub fn overflow_clip(gui: &Gui) -> JsValue {


//     let overflow_clip = gui.gui.overflow_clip.lend();

//     let mut clips: Vec<(usize, Clip)> = Vec::new();
//     for (index, v) in overflow_clip.clip.iter() {
//         clips.push((index, v.clone()));
//     }

//     let mut clip_map = XHashMap::default();
//     for (k, v) in overflow_clip.clip_map.iter() {
//         clip_map.insert(*k, v.0.clone());
//     }
//     let c = OverflowClip {
//         id_map: overflow_clip.id_map.clone(),
//         clip: clips,
//         clip_map: clip_map,
//     };
//     return JsValue::from_serde(&c).unwrap();
// }

// pub fn create_gui(engine: u32, width: f32, height: f32) -> u32 {

// // 打开性能检视面板
// #[allow(unused_attributes)]
// #[pi_js_export]
// pub fn open_performance_inspector(gui: &Gui, width: f32, height: f32) -> u32 {
// 	let world = unsafe {&mut *(world as usize as *mut GuiWorld)};
// 	if gui.gui.performance_inspector == 0 {
//
// 		let performance_gui = create_gui(Box::into_raw(Box::new((*gui.gui.engine).clone()) as u32, width, height);
// 		let performance_gui = unsafe {&mut *(performance_gui as usize as *mut GuiWorld)};
// 		gui_tool::open_performance_inspection(world, PerformanceStatisticians::new(&mut performance_gui.gui));
// 		gui.gui.performance_inspector = performance_gui;
// 		performance_gui as u32
// 	}
// }

// // 关闭性能检视面板
// #[allow(unused_attributes)]
// #[pi_js_export]
// pub fn close_performance_inspector(gui: &Gui) {
// 	let world = unsafe {&mut *(world as usize as *mut GuiWorld)};
// 	if gui.gui.performance_inspector > 0 {
// 		let world = Box::from_raw(unsafe {world as usize as *mut GuiWorld});
//
// 		gui_tool::close_performance_inspection(world);
// 	}
// }

// #[pi_js_export]
// pub fn res_size(gui: &Gui) -> JsValue {


//     let engine = gui.gui.engine.lend();
//     let mut size = ResMgrSize::default();

//     let texture = engine.texture_res_map.all_res();
//     for i in texture.0.iter() {
//         size.texture += i.1;
//         size.count_texture += 1;
//     }
//     for i in texture.1.iter() {
//         size.catch_texture += i.1.elem.cost;
//         size.count_catch_texture += 1;
//     }

//     let geometry = engine.geometry_res_map.all_res();
//     for i in geometry.0.iter() {
//         size.geometry += i.1;
//         size.count_geometry += 1;
//     }
//     for i in geometry.1.iter() {
//         size.catch_geometry += i.1.elem.cost;
//         size.count_catch_geometry += 1;
//     }

//     let buffer = engine.buffer_res_map.all_res();
//     for i in buffer.0.iter() {
//         size.buffer += i.1;
//         size.count_buffer += 1;
//     }
//     for i in buffer.1.iter() {
//         size.catch_buffer += i.1.elem.cost;
//         size.count_catch_buffer += 1;
//     }

//     let rs = engine.rs_res_map.all_res();
//     for i in rs.0.iter() {
//         // i.0
//         size.rs += i.1;
//         size.count_rs += 1;
//     }
//     for i in rs.1.iter() {
//         size.catch_rs += i.1.elem.cost;
//         size.count_catch_rs += 1;
//     }

//     let bs = engine.bs_res_map.all_res();
//     for i in bs.0.iter() {
//         size.bs += i.1;
//         size.count_bs += 1;
//     }
//     for i in bs.1.iter() {
//         size.catch_bs += i.1.elem.cost;
//         size.count_catch_bs += 1;
//     }

//     let ss = engine.ss_res_map.all_res();
//     for i in ss.0.iter() {
//         size.ss += i.1;
//         size.count_ss += 1;
//     }
//     for i in ss.1.iter() {
//         size.catch_ss += i.1.elem.cost;
//         size.count_catch_ss += 1;
//     }

//     let ds = engine.ds_res_map.all_res();
//     for i in ds.0.iter() {
//         size.ds += i.1;
//         size.count_ds += 1;
//     }
//     for i in ds.1.iter() {
//         size.catch_ds += i.1.elem.cost;
//         size.count_catch_ds += 1;
//     }

//     let sampler = engine.sampler_res_map.all_res();
//     for i in sampler.0.iter() {
//         size.sampler += i.1;
//         size.count_sampler += 1;
//     }
//     for i in sampler.1.iter() {
//         size.catch_sampler += i.1.elem.cost;
//         size.count_catch_sampler += 1;
//     }

//     let res_mgr_ref = engine.res_mgr.borrow();
//     let ucolor = res_mgr_ref.fetch_map::<UColorUbo>(0).unwrap();
//     let ucolor = ucolor.all_res();
//     for i in ucolor.0.iter() {
//         size.ucolor += i.1;
//         size.count_ucolor += 1;
//     }
//     for i in ucolor.1.iter() {
//         size.catch_ucolor += i.1.elem.cost;
//         size.count_catch_ucolor += 1;
//     }

//     let hsv = res_mgr_ref.fetch_map::<HsvUbo>(0).unwrap();
//     let hsv = hsv.all_res();
//     for i in hsv.0.iter() {
//         size.hsv += i.1;
//         size.count_hsv += 1;
//     }
//     for i in hsv.1.iter() {
//         size.catch_hsv += i.1.elem.cost;
//         size.count_catch_hsv += 1;
//     }

//     let msdf_stroke = res_mgr_ref.fetch_map::<MsdfStrokeUbo>(0).unwrap();
//     let msdf_stroke = msdf_stroke.all_res();
//     for i in msdf_stroke.0.iter() {
//         size.msdf_stroke += i.1;
//         size.count_msdf_stroke += 1;
//     }
//     for i in msdf_stroke.1.iter() {
//         size.catch_msdf_stroke += i.1.elem.cost;
//         size.count_catch_msdf_stroke += 1;
//     }

//     let canvas_stroke = res_mgr_ref.fetch_map::<CanvasTextStrokeColorUbo>(0).unwrap();
//     let canvas_stroke = canvas_stroke.all_res();
//     for i in canvas_stroke.0.iter() {
//         size.canvas_stroke += i.1;
//         size.count_canvas_stroke += 1;
//     }
//     for i in canvas_stroke.1.iter() {
//         size.catch_canvas_stroke += i.1.elem.cost;
//         size.count_catch_canvas_stroke += 1;
//     }

//     size.total_capacity = res_mgr_ref.total_capacity;

//     size.texture_max_capacity = engine.texture_res_map.cache.max_capacity();

//     return JsValue::from_serde(&size).unwrap();
// }

// #[derive(Default, Serialize, Deserialize)]
// pub struct TexureInfo {
//     list: Vec<(usize, usize, bool, usize)>, /*key, cost, isUsed, freeTime*/
//     min_capacity: usize,
//     max_capacity: usize,
//     cur_cost: usize,
// }
// /// 列出现有的纹理资源
// #[allow(non_snake_case)]
// #[pi_js_export]
// pub fn list_texture(gui: &Gui) -> JsValue {


//     let engine = gui.gui.engine.lend();
//     let sys_time = gui.gui.system_time.lend_mut();

//     let mut info = TexureInfo::default();
//     let list = &mut info.list;

//     let texture = engine.texture_res_map.all_res();
//     for i in texture.0.iter() {
//         list.push((*i.0.get_key(), i.1, true, sys_time.cur_time as usize));
//     }

//     for (key, v) in texture.2.iter() {
//         if *v.get_id() > 0 {
//             // 在lru中的资源
//             list.push((*key, texture.1[*v.get_id()].elem.cost, false, texture.1[*v.get_id()].elem.timeout));
//         }
//     }
//     info.min_capacity = engine.texture_res_map.cache.min_capacity();
//     info.max_capacity = engine.texture_res_map.cache.max_capacity();
//     info.cur_cost = engine.texture_res_map.cache.size();
//     return JsValue::from_serde(&info).unwrap();
// }

// #[allow(non_snake_case)]
// #[pi_js_export]
// pub fn common_statistics(gui: &Gui) -> JsValue {

//     let world = &mut gui.gui.gui;

//     let mut all_run_time = std::time::Duration::from_micros(0);
//     let mut sys_time = Vec::new();
//     for t in gui.gui.runtime.iter() {
//         sys_time.push((t.sys_name.as_ref().to_string(), (t.cost_time.as_secs_f64() * 1000.0) as f32));
//         all_run_time += t.cost_time;
//     }

//     let statistics = gui.gui.fetch_single::<Statistics>().unwrap();
//     let statistics = statistics.lend_mut();
//     sys_time.push(("runTotalTimes".to_string(), (all_run_time.as_secs_f64() * 1000.0) as f32));
//     sys_time.push(("drawCallTimes".to_string(), statistics.drawcall_times as f32));

//     return JsValue::from_serde(&sys_time).unwrap();
// }

// #[pi_js_export]
// pub fn is_dirty(gui: &Gui) -> bool {

//     if gui.gui.gui.dirty_list.lend().0.len() > 0 {
//         true
//     } else {
//         gui.gui.gui.renderSys.owner.deref().borrow().dirty
//     }
// }

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonStatistics {
    pub renderTime: f32,
    pub layoutTime: f32,
    pub runTotalTimes: f32,
    pub drawCallTimes: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MemStatistics {
    pub textureTotalCount: u32,
    pub textureTotalMemory: u32,
}

// #[test]
// fn test11() {
// 	let r = vec![46,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,243,1,0,0,169,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,243,1,0,0,169,3,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,9,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,11,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,13,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,14,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,15,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,17,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,18,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,19,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,20,0,0,0,0,0,0,0,0,0,0,0,0,63,0,0,0,63,0,0,0];
// 	exec_dyn_texture(r);
// }

// #[pi_js_export]
// pub fn exec_dyn_texture(bin: Vec<u8>) {
// 	match bincode::deserialize(bin.as_slice()) {
// 		Ok(r) => exedebug(&r),
// 		Err(e) => {
// 			println!("deserialize_class_map error: {:?}", e);
// 			return;
// 		}
// 	}
// }

// #[pi_js_export]
// pub fn get_debug_dyn_texture(gui: &Gui) -> Option<Vec<u8>> {
//
// 	let mut dyn_texture = gui.gui.gui.gui.gui.fetch_single::<Share<RefCell<DynAtlasSet>>>().unwrap();
// 	let dyn_texture = dyn_texture.lend_mut();
// 	let dyn_texture = &***dyn_texture;
// 	let dyn_texture = unsafe { &mut *(dyn_texture.as_ptr() ) };

// 	match bincode::serialize(&dyn_texture.debugList) {
// 		Ok(bin) => {
// 			return Some(bin);
// 		},
// 		Err(r) => {
// 			return None;
// 		},
// 	};
// }

// #[pi_js_export]
// pub fn mem_statistics(_gui: &Gui) {}

// #[pi_js_export]
// pub fn res_debug(res_mgr: &ResMgr) -> JsValue {
//     let res_mgr = res_mgr.get_inner().clone();
//     let res_mgr = res_mgr.borrow_mut();

//     let mut use_all = 0;
//     let mut lru_all = 0;
//     let mut res_list = ResDebugList {
//         un_use_total_cost: 0,
//         using_total_cost: 0,
//         details: Vec::new(),
//     };
//     for (k, i) in res_mgr.tables.iter() {
//         let list = i.res_map.debug();

//         for (_g, l) in list.into_iter() {
//             res_list.un_use_total_cost += l.un_use_total_cost;
//             res_list.using_total_cost += l.using_total_cost;
//             res_list.details.push(l);
//         }
//     }

//     return JsValue::from_serde(&res_list).unwrap();
// }

// #[derive(Serialize)]
// struct ResDebugList {
//     pub using_total_cost: usize,
//     pub un_use_total_cost: usize,
//     pub details: Vec<ResDebug>,
// }

// #[pi_js_export]
// pub fn get_font_sheet_debug(gui: &Gui) {

//     let font_sheet = gui.gui.gui.font_sheet.lend();
//     log::info!("char_slab: {:?}", font_sheet.borrow().char_slab);
// }


// #[pi_js_export]
// pub fn get_opcaity(gui: &Gui) {


//     let itree = gui.gui.gui.idtree.lend();
//     let opacity = gui.gui.gui.opacity.lend();

//     for (id, _node) in itree.recursive_iter(1) {
//         if let Some(r) = opacity.get(id) {
//             if r.0 < 1.0 {
//                 log::info!("opcaity==============={},{}", id, r.0);
//             }
//         }
//     }
// }
// /// 打印内存情况
// #[allow(unused_attributes)]
// #[pi_js_export]
// pub fn print_memory(gui: &Gui) {


//     log::info!("print_memory begin");

//     let mut total = 0;

//     let r = gui.gui.node_id.lend().mem_size();
//     total += r;
//     log::info!("    world::node_id = {:?}", r);
//     let r = gui.gui.transform.lend().mem_size();
//     total += r;
//     log::info!("    world::transform = {:?}", r);
//     let r = gui.gui.z_index.lend().mem_size();
//     total += r;
//     log::info!("    world::z_index = {:?}", r);
//     let r = gui.gui.overflow.lend().mem_size();
//     total += r;
//     log::info!("    world::overflow = {:?}", r);
//     let r = gui.gui.show.lend().mem_size();
//     total += r;
//     log::info!("    world::show = {:?}", r);
//     let r = gui.gui.opacity.lend().mem_size();
//     total += r;
//     log::info!("    world::opacity = {:?}", r);
//     let r = gui.gui.background_color.lend().mem_size();
//     total += r;
//     log::info!("    world::background_color = {:?}", r);
//     let r = gui.gui.box_shadow.lend().mem_size();
//     total += r;
//     log::info!("    world::box_shadow = {:?}", r);
//     let r = gui.gui.border_color.lend().mem_size();
//     total += r;
//     log::info!("    world::border_color = {:?}", r);
//     let r = gui.gui.border_image.lend().mem_size();
//     total += r;
//     log::info!("    world::border_image = {:?}", r);
//     let r = gui.gui.border_image_clip.lend().mem_size();
//     total += r;
//     log::info!("    world::border_image_clip = {:?}", r);
//     let r = gui.gui.border_image_slice.lend().mem_size();
//     total += r;
//     log::info!("    world::border_image_slice = {:?}", r);
//     let r = gui.gui.border_image_repeat.lend().mem_size();
//     total += r;
//     log::info!("    world::border_image_repeat = {:?}", r);
//     let r = gui.gui.text_style.lend().mem_size();
//     total += r;
//     log::info!("    world::text_style = {:?}", r);
//     let r = gui.gui.text_content.lend().mem_size();
//     total += r;
//     log::info!("    world::text_content = {:?}", r);
//     // let r = gui.gui.font.lend().mem_size();
//     // total += r;
//     // log::info!("    world::font = {:?}", r);
//     let r = gui.gui.border_radius.lend().mem_size();
//     total += r;
//     log::info!("    world::border_radius = {:?}", r);
//     let r = gui.gui.image.lend().mem_size();
//     total += r;
//     log::info!("    world::image = {:?}", r);
//     let r = gui.gui.image_clip.lend().mem_size();
//     total += r;
//     log::info!("    world::image_clip = {:?}", r);
//     let r = gui.gui.object_fit.lend().mem_size();
//     total += r;
//     log::info!("    world::object_fit = {:?}", r);
//     let r = gui.gui.filter.lend().mem_size();
//     total += r;
//     log::info!("    world::filter = {:?}", r);
//     let r = gui.gui.rect_layout_style.lend().mem_size();
//     total += r;
//     log::info!("    world::rect_layout_style = {:?}", r);
//     let r = gui.gui.other_layout_style.lend().mem_size();
//     total += r;
//     log::info!("    world::other_layout_style = {:?}", r);
//     let r = gui.gui.class_name.lend().mem_size();
//     total += r;
//     log::info!("    world::class_name = {:?}", r);
//     let r = gui.gui.style_mark.lend().mem_size();
//     total += r;
//     log::info!("    world::style_mark = {:?}", r);
//     let r = gui.gui.z_depth.lend().mem_size();
//     total += r;
//     log::info!("world::z_depth = {:?}", r);
//     let r = gui.gui.enable.lend().mem_size();
//     total += r;
//     log::info!("    world::enable = {:?}", r);
//     let r = gui.gui.visibility.lend().mem_size();
//     total += r;
//     log::info!("    world::visibility = {:?}", r);
//     let r = gui.gui_matrix.lend().mem_size();
//     total += r;
//     log::info!("    world::world_matrix = {:?}", r);
//     let r = gui.gui.by_overflow.lend().mem_size();
//     total += r;
//     log::info!("    world::by_overflow = {:?}", r);
//     let r = gui.gui.copacity.lend().mem_size();
//     total += r;
//     log::info!("    world::copacity = {:?}", r);
//     let r = gui.gui.layout.lend().mem_size();
//     total += r;
//     log::info!("    world::layout = {:?}", r);
//     let r = gui.gui.hsv.lend().mem_size();
//     total += r;
//     log::info!("    world::hsv = {:?}", r);
//     let r = gui.gui.culling.lend().mem_size();
//     total += r;
//     log::info!("    world::culling = {:?}", r);
//     // let r = gui.gui.idtree.lend().mem_size();
//     // total += r;
//     // log::info!("    world::idtree = {:?}", r);
//     let r = gui.gui.oct.lend().mem_size();
//     total += r;
//     log::info!("    world::oct = {:?}", r);
//     let r = gui.gui.overflow_clip.lend().mem_size();
//     total += r;
//     log::info!("    world::overflow_clip = {:?}", r);
//     let r = gui.gui.engine.lend().res_mgr.borrow().mem_size();
//     total += r;
//     log::info!("    world::engine.resMap = {:?}", r);
//     let r = gui.gui.render_objs.lend().mem_size();
//     total += r;
//     {
//         let render_objs = gui.gui.render_objs.lend();
//         let mut text: usize = 0;
//         let mut img: usize = 0;
//         let mut color: usize = 0;
//         let mut canvas: usize = 0;
//         let mut fbo: usize = 0;
//         let mut clip: usize = 0;
//         for (i, r) in render_objs.iter() {
//             if &*r.vs_name == &"color_vs" {
//                 color += 1;
//             } else if &*r.vs_name == &"image_vs" {
//                 img += 1;
//             } else if &*r.vs_name == &"canvas_text_vs" {
//                 text += 1;
//             } else if &*r.vs_name == &"canvas_vs" {
//                 canvas += 1;
//             } else if &*r.vs_name == &"fbo_vs" {
//                 fbo += 1;
//             } else if &*r.vs_name == &"clip_vs" {
//                 clip += 1;
//             }
//         }
//         log::info!(
//             "    world::render_objs = {:?}, {}, color:{}, img:{}, canvas_text:{}, canvas:{}, fbo:{}, clip:{}",
//             r,
//             gui.gui.render_objs.lend().len(),
//             color,
//             img,
//             text,
//             canvas,
//             fbo,
//             clip
//         );
//     }

//     let r = gui.gui.font_sheet.lend().borrow().mem_size();
//     total += r;
//     log::info!("    world::font_sheet = {:?}", r);
//     // let r = gui.gui.class_sheet.lend().borrow().mem_size();
//     // total += r;
//     // log::info!("    world::class_sheet = {:?}", r);
//     // let r = gui.gui.image_wait_sheet.lend().mem_size();
//     // total += r;
//     log::info!("    world::image_wait_sheet = {:?}", r);

//     let engine = gui.gui.engine.lend_mut();
//     let stat = engine.gl.render_get_stat();

//     total += stat.slab_mem_size;
//     log::info!("    world::engine::slab_mem_size = {:?}", stat.slab_mem_size);

//     let total: f32 = total as f32;
//     log::info!(" slab total bytes = {:?} MB", total / 1024.0 / 1024.0);
//     log::info!("");

//     log::info!("    world::engine::rt_count = {:?}", stat.rt_count);
//     log::info!("    world::engine::texture_count = {:?}", stat.texture_count);
//     log::info!("    world::engine::buffer_count = {:?}", stat.buffer_count);
//     log::info!("    world::engine::geometry_count = {:?}", stat.geometry_count);
//     log::info!("    world::engine::program_count = {:?}", stat.program_count);

//     log::info!("print_memory end");
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// struct ResMgrSize {
//     pub count_texture: usize,
//     pub count_geometry: usize,
//     pub count_buffer: usize,
//     pub count_sampler: usize,
//     pub count_rs: usize,
//     pub count_bs: usize,
//     pub count_ss: usize,
//     pub count_ds: usize,
//     pub count_ucolor: usize,
//     pub count_hsv: usize,
//     pub count_msdf_stroke: usize,
//     pub count_canvas_stroke: usize,

//     pub count_catch_texture: usize,
//     pub count_catch_geometry: usize,
//     pub count_catch_buffer: usize,
//     pub count_catch_sampler: usize,
//     pub count_catch_rs: usize,
//     pub count_catch_bs: usize,
//     pub count_catch_ss: usize,
//     pub count_catch_ds: usize,
//     pub count_catch_ucolor: usize,
//     pub count_catch_hsv: usize,
//     pub count_catch_msdf_stroke: usize,
//     pub count_catch_canvas_stroke: usize,

//     pub texture: usize,
//     pub geometry: usize,
//     pub buffer: usize,
//     pub sampler: usize,
//     pub rs: usize,
//     pub bs: usize,
//     pub ss: usize,
//     pub ds: usize,
//     pub ucolor: usize,
//     pub hsv: usize,
//     pub msdf_stroke: usize,
//     pub canvas_stroke: usize,

//     pub catch_texture: usize,
//     pub catch_geometry: usize,
//     pub catch_buffer: usize,
//     pub catch_sampler: usize,
//     pub catch_rs: usize,
//     pub catch_bs: usize,
//     pub catch_ss: usize,
//     pub catch_ds: usize,
//     pub catch_ucolor: usize,
//     pub catch_hsv: usize,
//     pub catch_msdf_stroke: usize,
//     pub catch_canvas_stroke: usize,

//     pub total_capacity: usize,
//     pub texture_max_capacity: usize,
// }

// #[allow(unused_attributes)]
// #[pi_js_export]
// pub fn bound_box(gui: &Gui, node_id: f64) {
//     let node_id = node_id as usize
//     let world = unsafe {&mut *(world as usize as *mut GuiWorld)};
//
//     let overflow_clip = gui.gui.fetch_single::<OverflowClip>().unwrap();
//     js!{
//         console.log("overflow_clip:", @{format!("{:?}", &overflow_clip.value)});
//     }
// }

#[pi_js_export]
pub fn get_world_matrix(engine: &mut Engine, node_id: f64) -> String {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });

    let world_matrix = match engine.world.query::<&WorldMatrix>().get(&engine.world, node_id) {
        Ok(r) => r,
        _ => return "undefined".to_string(),
    };

	serde_json::to_string(world_matrix).unwrap()
}

#[allow(unused_attributes)]
#[pi_js_export]
pub fn get_transform(engine: &mut Engine, node_id: f64) -> String {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });


    let transform = match engine.world.query::<&Transform>().get(&engine.world, node_id) {
        Ok(r) => r,
        _ => return "undefined".to_string(),
    };
	serde_json::to_string(transform).unwrap()
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Layout {
    pub size: Option<Size>,
    pub margin: Option<Margin>,
    pub padding: Option<Padding>,
    pub border: Option<Border>,
    pub position: Option<Position>,
    pub minmax: Option<MinMax>,
    pub flex_container: Option<FlexContainer>,
    pub flex_normal: Option<FlexNormal>,
    pub show: Option<Show>,
    pub node_state: Option<NodeState>,
    pub is_vnode: bool,
    pub layout_ret: Option<LayoutResult>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Point2{
//     x: f32,
//     y: f32,
// }
// js_serializable!( Point2 );

// impl Point2 {
//     fn new(x: f32, y: f32) -> Self {
//         Self {x, y}
//     }
// }

// #[allow(unused_attributes)]
// #[no_mangle]
// #[js_export]
// // pub fn test_create_render_obj(gui: &Gui, count: u32) {
// 	let world = unsafe {&mut *(world as usize as *mut GuiWorld)};
//

// 	let default_state = gui.gui.gui.gui.fetch_single::<gui::single::DefaultState>().unwrap();
// 	let default_state = default_state.lend();
// 	let render_objs = gui.gui.gui.gui.fetch_single::<RenderObjs>().unwrap();
// 	let render_objs = render_objs.lend_mut();
// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj(default_state);
// 	}
// 	log::info!("create_render_obj: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj1(default_state);
// 	}
// 	log::info!("create_render_obj1: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj3(default_state);
// 	}
// 	log::info!("create_render_obj3: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj4(default_state);
// 	}
// 	log::info!("create_render_obj4: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj5(default_state);
// 	}
// 	log::info!("create_render_obj5: {:?}", std::time::Instant::now() - time);

// 	let mut m = map::vecmap::VecMap::default();
// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj6(&mut m, 2, render_objs, default_state);
// 	}
// 	log::info!("create_render_obj6: {:?}", std::time::Instant::now() - time);

// 	let mut m = map::vecmap::VecMap::default();
// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj7(&mut m, 2, render_objs, default_state);
// 	}
// 	log::info!("create_render_obj7: {:?}", std::time::Instant::now() - time);

// 	let p: share::Share<dyn hal_core::ProgramParamter> = share::Share::new(ImageParamter::default());
// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		create_render_obj13(&mut m, 2, render_objs, default_state, &p);
// 	}
// 	log::info!("create_render_obj13: {:?}", std::time::Instant::now() - time);

// 	let read = (gui.gui.copacity.lend(), gui.gui.visibility.lend(), gui.gui.hsv.lend(), gui.gui.z_depth.lend(), gui.gui.culling.lend());
// 	let render_objs = gui.gui.gui.gui.fetch_single::<gui::single::RenderObjs>().unwrap();
// 	let node_render_map = gui.gui.gui.gui.fetch_single::<gui::single::NodeRenderMap>().unwrap();
// 	let write = (render_objs.lend_mut(), node_render_map.lend_mut());
// 	let v:Option<share::Share<dyn UniformBuffer>> = Some(share::Share::new(gui::component::calc::ViewMatrixUbo::new(hal_core::UniformValue::MatrixV4(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]))));
// 	let p:Option<share::Share<dyn UniformBuffer>> = Some(share::Share::new(gui::component::calc::ProjectMatrixUbo::new(hal_core::UniformValue::MatrixV4(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]))));

// 	// let mut m = map::vecmap::VecMap::default();
// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		render_objs_create8((read.0, read.1, read.2, read.3, read.4), (write.0, write.1), &v, &p);
// 	}
// 	log::info!("create_render_obj8: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		render_objs_create9((read.0, read.1, read.2, read.3, read.4), (write.0, write.1), &v, &p);
// 	}
// 	log::info!("render_objs_create9: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		render_objs_create10((read.0, read.1, read.2, read.3, read.4), (write.0, write.1), &v, &p);
// 	}
// 	log::info!("render_objs_create10: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		render_objs_create11((read.0, read.1, read.2, read.3, read.4), (write.0, write.1), &v, &p);
// 	}
// 	log::info!("render_objs_create11: {:?}", std::time::Instant::now() - time);

// 	let time = std::time::Instant::now();
// 	for i in 0..count {
// 		render_objs_create12((read.0, read.1, read.2, read.3, read.4), (write.0, write.1), &v, &p);
// 	}
// 	log::info!("render_objs_create12: {:?}", std::time::Instant::now() - time);

// }

// //  RenderObj {
// //         depth: 0.0,
// //         program_dirty: true,
// //         visibility: false,
// //         vs_defines: Box::new(VsDefines::default()),
// //         fs_defines: Box::new(FsDefines::default()),
// //         program: None,
// //         geometry: None,
// //         depth_diff,
// //         is_opacity,
// //         vs_name,
// //         fs_name,
// //         paramter,
// //         state,
// //         context,
// //     }

// #[inline]
// pub fn create_render_obj(
//     default_state: &gui::single::DefaultState,
// ){
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
//     let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	gui::system::util::new_render_obj(1, 2.0, true, gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone(), gui::system::render::shaders::image::IMAGE_FS_SHADER_NAME.clone(), share::Share::new(gui::component::calc::ImageParamter::default()), state);
// }

// #[inline]
// pub fn create_render_obj1(
//     default_state: &gui::single::DefaultState,
// ){
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
//     let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();

// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// 	let notify = default_state.df_ds.clone();
// }

// #[inline]
// pub fn create_render_obj3(
//     default_state: &gui::single::DefaultState,
// ){
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
//     let vs = gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone();
// 	let fs = gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone();
// 	let p = share::Share::new(gui::component::calc::ImageParamter::default());

// }

// #[inline]
// pub fn create_render_obj4(
//     default_state: &gui::single::DefaultState,
// ){
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
// 	let p = share::Share::new(gui::component::calc::ImageParamter::default());

// }

// #[inline]
// pub fn create_render_obj5(
//     default_state: &gui::single::DefaultState,
// ){
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
// 	share::Share::new(1);
// 	share::Share::new(1);
// 	share::Share::new(1);
// 	share::Share::new(1);
// 	share::Share::new(1);
// 	share::Share::new(1);
// 	share::Share::new(1);
// }

// #[inline]
// fn create_render_obj6(
// 	render_map: &mut map::vecmap::VecMap<usize>,
// 	id: usize,
// 	render_objs: &mut ecs::SingleCaseImpl<RenderObjs>,
// 	default_state: &DefaultState,
// ) -> usize{
// 	gui::system::util::create_render_obj(
// 		id,
// 		-0.1,
// 		true,
// 		gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone(),
// 		gui::system::render::shaders::image::IMAGE_FS_SHADER_NAME.clone(),
// 		share::Share::new(ImageParamter::default()),
// 		default_state, render_objs,
// 		render_map
// 	)
// }

// #[inline]
// fn create_render_obj7(
// 	render_map: &mut map::vecmap::VecMap<usize>,
// 	id: usize,
// 	render_objs: &mut ecs::SingleCaseImpl<RenderObjs>,
// 	default_state: &DefaultState,
// ) -> usize{
// 	create_render_obj_(
// 		id,
// 		-0.1,
// 		true,
// 		gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone(),
// 		gui::system::render::shaders::image::IMAGE_FS_SHADER_NAME.clone(),
// 		share::Share::new(ImageParamter::default()),
// 		default_state, render_objs,
// 		render_map
// 	)
// }

// #[inline]
// pub fn create_render_obj_(
//     context: usize,
//     depth_diff: f32,
//     is_opacity: bool,
//     vs_name: atom::Atom,
//     fs_name: atom::Atom,
//     paramter: share::Share<dyn ProgramParamter>,
//     default_state: &DefaultState,
//     render_objs: &mut ecs::SingleCaseImpl<RenderObjs>,
//     render_map: &mut map::vecmap::VecMap<usize>,
// ) -> usize{
//     let state = gui::single::State {
//         bs: default_state.df_bs.clone(),
//         rs: default_state.df_rs.clone(),
//         ss: default_state.df_ss.clone(),
//         ds: default_state.df_ds.clone(),
//     };
//     let notify = render_objs.get_notify();
//     let render_index = render_objs.insert(
//         gui::system::util::new_render_obj(context, depth_diff, is_opacity, vs_name, fs_name, paramter, state),
//         None
//     );
//     render_map.insert(context, render_index);
//     render_index
// }

// fn render_objs_create8<'a>(read: (
//         &'a ecs::MultiCaseImpl<Node, gui::component::calc::Opacity>,
//         &'a ecs::MultiCaseImpl<Node, Visibility>,
//         &'a ecs::MultiCaseImpl<Node, HSV>,
//         &'a ecs::MultiCaseImpl<Node, ZDepth>,
//         &'a ecs::MultiCaseImpl<Node, Culling>,
//     ),
// 	write: (&'a mut ecs::SingleCaseImpl<RenderObjs>, &'a mut ecs::SingleCaseImpl<NodeRenderMap>),
// 	view_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// 	project_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// ) {
// 	let (opacitys, visibilitys, hsvs, z_depths, cullings) = read;
// 	let (render_objs, node_render_map) = write;
// 	let render_obj = unsafe { render_objs.get_unchecked_mut(3) };
// 	let notify = node_render_map.get_notify();
// 	unsafe{ node_render_map.add_unchecked(render_obj.context, 3, &notify) };

// 	let paramter = &mut render_obj.paramter;

// 	paramter.set_value("viewMatrix", view_matrix_ubo.clone().unwrap()); // VIEW_MATRIX
// 	paramter.set_value("projectMatrix", project_matrix_ubo.clone().unwrap()); // PROJECT_MATRIX

// 	let z_depth = unsafe { z_depths.get_unchecked(render_obj.context) }.0;
// 	let opacity = unsafe { opacitys.get_unchecked(render_obj.context) }.0;
// 	paramter.set_single_uniform("alpha", UniformValue::Float1(opacity)); // alpha
// 	log::info!("id: {}, alpha: {:?}", render_obj.context, opacity);

// 	let visibility = unsafe { visibilitys.get_unchecked(render_obj.context) }.0;
// 	let culling = unsafe { cullings.get_unchecked(render_obj.context) }.0;
// 	render_obj.visibility = visibility & !culling;

// 	render_obj.depth = z_depth + render_obj.depth_diff;

// 	let hsv = unsafe { hsvs.get_unchecked(render_obj.context) };
// 	if !(hsv.h == 0.0 && hsv.s == 0.0 && hsv.v == 0.0) {
// 		render_obj.fs_defines.add("HSV");
// 		// paramter.set_value("hsvValue", self.create_hsv_ubo(hsv)); // hsv
// 	}
// }

// fn render_objs_create9<'a>(read: (
//         &'a ecs::MultiCaseImpl<Node, gui::component::calc::Opacity>,
//         &'a ecs::MultiCaseImpl<Node, Visibility>,
//         &'a ecs::MultiCaseImpl<Node, HSV>,
//         &'a ecs::MultiCaseImpl<Node, ZDepth>,
//         &'a ecs::MultiCaseImpl<Node, Culling>,
//     ),
// 	write: (&'a mut ecs::SingleCaseImpl<RenderObjs>, &'a mut ecs::SingleCaseImpl<NodeRenderMap>),
// 	view_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// 	project_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// ) {
// 	let (opacitys, visibilitys, hsvs, z_depths, cullings) = read;
// 	let (render_objs, node_render_map) = write;
// 	let render_obj = unsafe { render_objs.get_unchecked_mut(3) };
// 	let notify = node_render_map.get_notify();
// 	unsafe{ node_render_map.add_unchecked(render_obj.context, 3, &notify) };

// 	let paramter = &mut render_obj.paramter;

// 	paramter.set_value("viewMatrix", view_matrix_ubo.clone().unwrap()); // VIEW_MATRIX
// 	paramter.set_value("projectMatrix", project_matrix_ubo.clone().unwrap()); // PROJECT_MATRIX

// }

// fn render_objs_create10<'a>(read: (
//         &'a ecs::MultiCaseImpl<Node, gui::component::calc::Opacity>,
//         &'a ecs::MultiCaseImpl<Node, Visibility>,
//         &'a ecs::MultiCaseImpl<Node, HSV>,
//         &'a ecs::MultiCaseImpl<Node, ZDepth>,
//         &'a ecs::MultiCaseImpl<Node, Culling>,
//     ),
// 	write: (&'a mut ecs::SingleCaseImpl<RenderObjs>, &'a mut ecs::SingleCaseImpl<NodeRenderMap>),
// 	view_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// 	project_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// ) {
// 	let (opacitys, visibilitys, hsvs, z_depths, cullings) = read;
// 	let (render_objs, node_render_map) = write;
// 	let render_obj = unsafe { render_objs.get_unchecked_mut(3) };
// 	let notify = node_render_map.get_notify();
// 	unsafe{ node_render_map.add_unchecked(render_obj.context, 3, &notify) };
// }

// fn render_objs_create11<'a>(read: (
//         &'a ecs::MultiCaseImpl<Node, gui::component::calc::Opacity>,
//         &'a ecs::MultiCaseImpl<Node, Visibility>,
//         &'a ecs::MultiCaseImpl<Node, HSV>,
//         &'a ecs::MultiCaseImpl<Node, ZDepth>,
//         &'a ecs::MultiCaseImpl<Node, Culling>,
//     ),
// 	write: (&'a mut ecs::SingleCaseImpl<RenderObjs>, &'a mut ecs::SingleCaseImpl<NodeRenderMap>),
// 	view_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// 	project_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// ) {
// 	let (opacitys, visibilitys, hsvs, z_depths, cullings) = read;
// 	let (render_objs, node_render_map) = write;
// 	let render_obj = unsafe { render_objs.get_unchecked_mut(3) };
// 	let notify = node_render_map.get_notify();
// 	// unsafe{ node_render_map.add_unchecked(render_obj.context, 3, &notify) };
// }

// fn render_objs_create12<'a>(read: (
//         &'a ecs::MultiCaseImpl<Node, gui::component::calc::Opacity>,
//         &'a ecs::MultiCaseImpl<Node, Visibility>,
//         &'a ecs::MultiCaseImpl<Node, HSV>,
//         &'a ecs::MultiCaseImpl<Node, ZDepth>,
//         &'a ecs::MultiCaseImpl<Node, Culling>,
//     ),
// 	write: (&'a mut ecs::SingleCaseImpl<RenderObjs>, &'a mut ecs::SingleCaseImpl<NodeRenderMap>),
// 	view_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// 	project_matrix_ubo: &Option<share::Share<dyn UniformBuffer>>,
// ) {
// 	let (opacitys, visibilitys, hsvs, z_depths, cullings) = read;
// 	let (render_objs, node_render_map) = write;
// 	let render_obj = unsafe { render_objs.get_unchecked_mut(3) };
// 	let notify = node_render_map.get_notify();
// 	// unsafe{ node_render_map.add_unchecked(render_obj.context, 3, notify) };
// }

// #[inline]
// fn create_render_obj13(
// 	render_map: &mut map::vecmap::VecMap<usize>,
// 	id: usize,
// 	render_objs: &mut ecs::SingleCaseImpl<RenderObjs>,
// 	default_state: &DefaultState,
// 	p: &share::Share<dyn hal_core::ProgramParamter>
// ) -> usize{
// 	create_render_obj_(
// 		id,
// 		-0.1,
// 		true,
// 		gui::system::render::shaders::image::IMAGE_VS_SHADER_NAME.clone(),
// 		gui::system::render::shaders::image::IMAGE_FS_SHADER_NAME.clone(),
// 		p.clone(),
// 		default_state, render_objs,
// 		render_map
// 	)
// }
