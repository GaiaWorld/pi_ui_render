use std::mem::transmute;
use std::ops::Range;

use pi_atom::get_by_hash;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::Down;
use pi_bevy_ecs_extend::prelude::Root;
use pi_bevy_ecs_extend::prelude::Up;
use pi_bevy_ecs_extend::system_param::tree::Layer;
use pi_bevy_render_plugin::asimage_url::RenderTarget;
use pi_null::Null;
use pi_render::rhi::asset::TextureRes;
use pi_render::rhi::shader::GetBuffer;
use pi_render::rhi::shader::WriteBuffer;
use pi_style::style::Aabb2;
use pi_style::style::ImageRepeat;
use pi_world::filter::With;
use pi_world::query::Query;
use crate::components::calc::DrawInfo;
use crate::components::calc::InPassId;
use crate::components::calc::RenderContextMark;
use crate::components::calc::StyleMark;
use crate::components::calc::{TransformWillChangeMatrixInner, TransformWillChangeMatrix};
use crate::components::draw_obj::InstanceIndex;
use crate::components::draw_obj::RenderCount;
use crate::components::pass_2d::Camera;
use crate::components::pass_2d::GraphId;
use crate::components::pass_2d::ParentPassId;
use crate::components::user::serialize::StyleAttr;
use crate::components::user::{BorderRadius, BorderImageSlice, BorderImageRepeat, BorderImageClip, BorderImage, Border, Blur, BackgroundImageMod, BackgroundImageClip, BackgroundImage, AsImage, Animation};
use crate::components::user::{TextStyle, TextShadow, TextOverflowData, TextContent, Show, RadialWave, Position, Padding, Opacity, MaskImageClip, Margin, Hsi, FlexContainer, ClipPath, Canvas, BoxShadow};
use crate::components::user::{BorderColor, BackgroundColor, BlendMode, ClassName, FlexNormal, MaskImage, MinMax, NodeState, StyleAttribute, FitType, ZIndex, Vector2, TransformWillChange, Transform};
use crate::components::SettingComponentIds;
use crate::resource::draw_obj::InstanceContext;
use crate::resource::fragment::DebugInfo;
use crate::resource::BackgroundColorRenderObjType;
use crate::resource::BackgroundImageRenderObjType;
use crate::resource::BorderColorRenderObjType;
use crate::resource::BorderImageRenderObjType;
use crate::resource::CanvasRenderObjType;
use crate::resource::RenderContextMarkType;
use crate::resource::TextRenderObjType;
use crate::shader1::batch_meterial::MeterialBind;
use crate::shader1::batch_meterial::RenderFlagType;
use serde::{Deserialize, Serialize};

use pi_style::style::Point2;

use crate::components::calc::ContentBox;
use pi_world::world::World;
use crate::components::calc::IsShow;
use crate::components::calc::LayoutResult;
use crate::components::calc::WorldMatrix;
use crate::components::calc::{DrawList, EntityKey, ZRange};
use crate::components::user::serialize::StyleTypeReader;
use crate::components::user::Vector4;
// use pi_ui_render::components::user::*;
use crate::components::user::{Overflow, Size};
use crate::resource::ClassSheet;
use pi_world::prelude::Entity;
use crate::components::calc::View;
use smallvec::SmallVec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Quad {
    pub left_top: Point2,
    pub left_bottom: Point2,
    pub right_bottom: Point2,
    pub right_top: Point2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layout1 {
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
pub struct Info {
    pub render_obj: Vec<RenderObject>,
    pub pass_info: Option<PassInfo>,
    pub overflow: bool,
    // pub by_overflow: usize,
    pub visibility: bool,
    pub display: bool,
    pub enable: bool,
    pub layout: Layout,
    pub transform: Option<Transform>,
    pub transform_will_change: Option<TransformWillChange>,
    pub world_matrix: WorldMatrix,
    pub border_box: Quad,
    pub padding_box: Quad,
    pub content_box: Quad,
    pub blend_mode: Option<BlendMode>,
    pub opacity: f32,
    pub blur: f32,
    pub zindex: isize,
    pub zdepth: f32,
    // pub culling: bool,
    // char_block: Option<CharBlock1>,
    pub class_name: Option<ClassName>,
    pub image: Option<String>,
    pub mask_image: Option<MaskImage>,
    pub border_image: Option<String>,
    // pub render_context: bool,
    pub background_color: Option<BackgroundColor>,
    pub border_color: Option<BorderColor>,
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
    pub parent_id: String,
	pub inpass: String,
    pub content_bound_box: Option<ContentBox>,
    pub quad: Option<crate::components::calc::Quad>,

    text: Option<TextStyle>,
	text_shadow: Option<TextShadow>,
    text_content: Option<TextContent>,
    // style_mark: StyleMark,
    children: Vec<f64>,
	pub animation: String,
	pub as_image: String,
	pub canvas: String,
	pub layer: String,
	
	pub text_overflow_data: String,
	
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PassInfo {
    
    pub copy_render_obj: RenderObject,
    pub context_mark: String,
    pub view_port: Aabb2,      // 非渲染视口区域（相对于全局的0,0点）
    // 当前target所对应的在该节点的非旋转坐标系下的包围盒（分配fbo的尺寸）
    pub bound_box: Aabb2,
    // 精确的bound_box，由于bound_box是整数数据， 并非渲染时的精确包围盒， 需要记录精确包围盒，以免出现渲染下次（范围： 0~1， 表示距离边界的偏移比例）
    pub accurate_bound_box: Aabb2,
    // 是否渲染自身内容（如果为false，该相机不会渲染任何物体）
    // draw_changed为true时， is_render_own一定为true
    // draw_changed为false时，还需要看，从当前上下文开始向上递归，是否有上下文渲染目标被缓存，如果有，则is_render_own为false，否则为true
    pub is_render_own: bool,
    // 表示相机内的渲染内容是否改变
    pub draw_changed: bool,
    // 是否渲染到父目标(表示该pass是否渲染到父目标上)
    pub is_render_to_parent: bool, 
    pub has_render_target: bool, 
    pub view: View,
    pub transform_will_change_matrix: Option<TransformWillChangeMatrixInner>,
    pub parentpass: Entity,
	pub graph_id: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RenderObject {
    pub id: Entity,
    pub instance_index: Range<usize>, // 实例索引
    pub instance_count: usize, // 实例数量
    pub instance_ty: InstanceType,
    pub instance_data: Vec<CommonMeterial>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InstanceType {
    BackgroundImage,
    BackgroundColor,
    BorderImage,
    BorderColor,
    Char,
    Canvas,
    CopyFbo,
    Unknown,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RasterStateDesc {
    // pub cull_mode: Option<CullMode>,
    pub is_front_face_ccw: bool,
    pub polygon_offset: (f32, f32),
}

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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GuiNode {
    uniqueID: f64,
    uniqueIDString: String,
    tag: String,
    attrs: Vec<ClassAttr>, 
    childs: Vec<GuiNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClassAttr {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GloabalInfo {
    // pass2d 渲染排序
    pass2d_sort: Vec<String>,
    next_node_with_depend: Vec<usize>, // 下一个有依赖的pass2d节点的索引
}

pub fn get_global_info(world: &World) -> GloabalInfo {
    let instance_context = &**world.get_single_res::<InstanceContext>().unwrap();
    GloabalInfo {
        pass2d_sort: instance_context.pass_toop_list.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>(),
        next_node_with_depend: instance_context.next_node_with_depend.clone(),
    }
}

pub fn get_gui_root(world: &mut World) -> Option<Entity> {
    let mut query = world.query::<Entity, (With<Root>, With<Size>)>();
    query.iter(world).next()
}

pub fn get_document_tree(world: &mut World, root: Entity) -> GuiNode {
    let mut query = world.query::<(&Down, &Up, Option<&ClassName>), With<Size>>();
    let query = query.get_param(world);
    let mut n = GuiNode::default();
    init_node( root, &mut n, &query);
    return n;
}

pub fn get_roots(world: &mut World) -> Vec<Entity> {
    let mut query = world.query::<Entity, (With<Root>, With<Size>)>();
    return query.iter(world).collect();
}


pub fn init_node( 
    id: Entity, 
    node: &mut GuiNode, 
    query: &Query<(&Down, &Up, Option<&ClassName>), With<Size>>,
) {
    let (down, _, class_name) = query.get(id).unwrap();
    node.tag = "div".to_string();
    node.uniqueID = unsafe {transmute(id)};
    node.uniqueIDString = format!("{:?}", id);
    if let Some(class_name) = class_name {
        let mut r: Vec<String> = Vec::new();
        for c in class_name.0.iter() {
            r.push(c.to_string());
        }
        node.attrs.push(ClassAttr {
            name: "wclass".to_string(),
            value: r.join(" "),
        });

    }

    let mut cur_child = down.head();
    while !cur_child.is_null() {
        let mut n = GuiNode::default();
        init_node( cur_child, &mut n, query);
        node.childs.push(n);
        let (_c_down, c_up, _class_name) = query.get(cur_child).unwrap();
        cur_child = c_up.next();
    }
}

pub fn get_layout(world: &World, node_id: Entity) -> String {
    let node_id = unsafe { transmute(node_id) };
    let (node_state, size, margin, padding, border, position, minmax, flex_container, flex_normal, show, layout_ret) =
        (
            world.get_component::<NodeState>(node_id).ok(),
            world.get_component::<Size>(node_id).ok(),
            world.get_component::<Margin>(node_id).ok(),
            world.get_component::<Padding>(node_id).ok(),
            world.get_component::<Border>(node_id).ok(),
            world.get_component::<Position>(node_id).ok(),
            world.get_component::<MinMax>(node_id).ok(),
            world.get_component::<FlexContainer>(node_id).ok(),
            world.get_component::<FlexNormal>(node_id).ok(),
            world.get_component::<Show>(node_id).ok(),
            world.get_component::<LayoutResult>(node_id).ok(),
        );
        // query.get(&world, node_id).unwrap();

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

pub fn get_class_names(world: &World, entity: Entity) -> String {

    let names = match world.get_component::<ClassName>(entity) {
        Ok(r) => r.0.iter().map(|r| *r).collect::<Vec<_>>(),
        _ => return "[]".to_string(),
    };

	serde_json::to_string(&names).unwrap()
}

pub fn get_class(world: &World, class_name: u32) -> String {
    let class = match world.get_single_res::<ClassSheet>() {
		Some(class_sheet) if let Some(class) = class_sheet.class_map.get(&(class_name as usize)) => {
			let mut ret = "".to_string();
            // println!("set class1==========={}", i);
            let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
            while let Some(r) = style_reader.to_attr() {
				if let StyleAttribute::Set(r) = r {
					let (s, v) = crate::tools::to_css_str(&r);
					if s != "" {
						ret += (s.to_string() + ":" + v.as_str() + ";").as_str();
					}
				}
            }
            Some(ret)
		},
		_ => None
	};

	serde_json::to_string(&class).unwrap()
}

pub fn get_style(world: &World, entity: Entity) -> String {
    let mut ret = "".to_string();
    let style_mark = match world.get_component::<StyleMark>(entity) {
        Ok(r) => r.local_style,
        Err(_) => return ret,
    };
    let setting_components = &**world.get_single_res::<SettingComponentIds>().unwrap();
    
    
    for style_index in style_mark.iter_ones() {
        let attr = StyleAttr::get(style_index as u16, world, setting_components, entity);
        if let Some(attr) = attr {
            let (s, v) = crate::tools::to_css_str(&attr);
            if s != "" {
                ret += (s.to_string() + ":" + v.as_str() + ";").as_str();
            }
        }
    }
    return ret;
}


pub fn node_info(world: &World, entity: Entity) -> Info {
    let (node_state, is_vnode) = match world.get_component::<NodeState>(entity) {
        Ok(r) => (Some(r.clone()), r.is_vnode()),
        Err(_) => (None, false),
    };
    let layout = world.get_component::<LayoutResult>(entity).unwrap();
    let layout1 = Layout {
        node_state: node_state,
        size: world.get_component::<Size>(entity).ok().map(|r| r.clone()),
        margin: world.get_component::<Margin>(entity).ok().map(|r| r.clone()),
        padding: world.get_component::<Padding>(entity).ok().map(|r| r.clone()),
        border: world.get_component::<Border>(entity).ok().map(|r| r.clone()),
        position: world.get_component::<Position>(entity).ok().map(|r| r.clone()),
        minmax: world.get_component::<MinMax>(entity).ok().map(|r| r.clone()),
        flex_container: world.get_component::<FlexContainer>(entity).ok().map(|r| r.clone()),
        flex_normal: world.get_component::<FlexNormal>(entity).ok().map(|r| r.clone()),
        show: world.get_component::<Show>(entity).ok().map(|r| r.clone()),
        layout_ret: Some(layout.clone()),
        is_vnode,

    };
    
           

    let world_matrix = &world.get_component::<WorldMatrix>(entity).unwrap().clone();


	let mark_type_as_image = world.get_single_res::<RenderContextMarkType<AsImage>>().unwrap();
	let mark_type_overflow = world.get_single_res::<RenderContextMarkType<Overflow>>().unwrap();
	let mark_type_blur = world.get_single_res::<RenderContextMarkType<Blur>>().unwrap();
	let mark_type_hsi = world.get_single_res::<RenderContextMarkType<Hsi>>().unwrap();
	let mark_type_opacity = world.get_single_res::<RenderContextMarkType<Opacity>>().unwrap();
	let mark_type_radial_wave = world.get_single_res::<RenderContextMarkType<RadialWave>>().unwrap();
	let mark_type_clippath = world.get_single_res::<RenderContextMarkType<ClipPath>>().unwrap();
	let mark_type_transform_willchange = world.get_single_res::<RenderContextMarkType<TransformWillChange>>().unwrap();
	

    // let draw_list =  world.query::<&DrawList>();

    // let mask_image =  world.query::<&MaskImage>();

    // let mask_image_clip =  world.query::<&MaskImageClip>();

    // let content_boxs = world.query::<&ContentBox>();

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

    let draw_list = match world.get_component::<DrawList>(entity) {
        Ok(r) => r.0.clone(),
        _ => SmallVec::default(),
    };

    let mut draw_objs = Vec::new();
    let instance_context = &**world.get_single_res::<InstanceContext>().unwrap();
    let border_image_type = &**world.get_single_res::<BorderImageRenderObjType>().unwrap();
    let border_color_type = &**world.get_single_res::<BorderColorRenderObjType>().unwrap();
    let bg_color_type = &**world.get_single_res::<BackgroundColorRenderObjType>().unwrap();
    let bg_image_type = &**world.get_single_res::<BackgroundImageRenderObjType>().unwrap();
    let canvas_type = &**world.get_single_res::<CanvasRenderObjType>().unwrap();
    let text_type = &**world.get_single_res::<TextRenderObjType>().unwrap();

    let create_render_obj = |instance_index: Range<usize>, id: Entity, instance_ty: InstanceType| {
        
        let mut render_obj = RenderObject {
            id: id,

            instance_index: instance_index.start / MeterialBind::SIZE .. instance_index.end / MeterialBind::SIZE,
            instance_count: 1,
            instance_ty,
            instance_data: Vec::new(),
        };

        for i in render_obj.instance_index.clone() {
            let index = i * MeterialBind::SIZE;
            let mut r = CommonMeterial::default();
            r.get_data(index as u32, &instance_context.instance_data.data());
            // if r.ty as usize & RenderFlagType::ClipRectRadius as usize > 0 {
            //     r.render_flag += "&ClipRectRadius";
            // }
            if r.ty as usize & (1 << RenderFlagType::IgnoreCamera as usize) > 0 {
                r.render_flag += "&IgnoreCamera";
            }
            if r.ty as usize & (1 << RenderFlagType::Premulti as usize) > 0 {
                r.render_flag += "&Premulti";
            }
            if r.ty as usize & (1 << RenderFlagType::R8 as usize) > 0 {
                r.render_flag += "&R8";
            }
            // if r.ty as usize & RenderFlagType::ClipSector as usize > 0 {
            //     r.render_flag += "&ClipSector";
            // }
            // if r.ty as usize & RenderFlagType::Uv as usize > 0 {
            //     r.render_flag += "&Uv";
            // }
            // if r.ty as usize & RenderFlagType::Color as usize > 0 {
            //     r.render_flag += "&Color";
            // }
            if r.ty as usize & (1 << RenderFlagType::Stroke as usize) > 0 {
                r.render_flag += "&Stroke";
            }
            // if r.ty as usize & RenderFlagType::TextStroke as usize > 0 {
            //     r.render_flag += "&TextStroke";
            // }
            if r.ty as usize & (1 << RenderFlagType::NotVisibility as usize) > 0 {
                r.render_flag += "&NotVisibility";
            }
            if r.ty as usize & (1 << RenderFlagType::Invalid as usize) > 0 {
                r.render_flag += "&Invalid";
            }
            if r.ty as usize & (1 << RenderFlagType::LinearGradient as usize) > 0 {
                r.render_flag += "&LinearGradient";
            }
            // if r.ty as usize & RenderFlagType::Border as usize > 0 {
            //     r.render_flag += "&Border";
            // }
            // if r.ty as usize & RenderFlagType::BoxShadow as usize > 0 {
            //     r.render_flag += "&BoxShadow";
            // }
            // if r.ty as usize & RenderFlagType::ImageRepeat as usize > 0 {
            //     r.render_flag += "&ImageRepeat";
            // }
            // if r.ty as usize & RenderFlagType::BorderImage as usize > 0 {
            //     r.render_flag += "&BorderImage";
            // }
            // if r.ty as usize & RenderFlagType::Sdf2 as usize > 0 {
            //     r.render_flag += "&Sdf2";
            // }
            // if r.ty as usize & RenderFlagType::Sdf2OutGlow as usize > 0 {
            //     r.render_flag += "&Sdf2OutGlow";
            // }
            // if r.ty as usize & RenderFlagType::SvgStrokeDasharray as usize > 0 {
            //     r.render_flag += "&SvgStrokeDasharray";
            // }
            // if r.ty as usize & RenderFlagType::Svg as usize > 0 {
            //     r.render_flag += "&Svg";
            // }
            // if r.ty as usize & RenderFlagType::Sdf2Shadow as usize > 0 {
            //     r.render_flag += "&Sdf2Shadow";
            // }
            render_obj.instance_data.push(r);
           
            // instance_context.instance_data.instance_data_mut()
        }
        return render_obj;

    };
    for i in draw_list.iter() {
        if let (Ok(_), instance_index, count) = (
            world.get_component::<DrawInfo>(i.id),
            world.get_component::<InstanceIndex>(i.id),
            world.get_component::<RenderCount>(i.id),
        ) {
            let instance_index = match instance_index {
                Ok(r) => r.0.clone(),
                Err(_) => Null::null(),
            };
            let ty = *i.ty;
            let ty = if ty == ***bg_color_type {
                InstanceType::BackgroundColor
            } else if ty == ***bg_image_type {
                InstanceType::BackgroundImage
            } else if ty == ***canvas_type {
                InstanceType::Canvas
            } else if ty == ***text_type {
                InstanceType::Char
            } else if ty == ***border_color_type{
                InstanceType::BorderColor
            } else if ty == ***border_image_type{
                InstanceType::BorderImage
            } else {
                InstanceType::Unknown
            };
    
            let mut render_obj = create_render_obj(instance_index, i.id.clone(), ty);
            render_obj.instance_count = match count {
                Ok(c) => c.0 as usize,
                _ => 1,
            };
			draw_objs.push(render_obj);
		}
    }

    

    let mut children = Vec::new();

    if let Ok(down) = world.get_component::<Down>(entity) {
        let mut n = down.head();
        while !EntityKey(n).is_null() {
            children.push(unsafe { transmute::<_, f64>(n) });
            n = match world.get_component::<Up>(n) {
                Ok(r) => r.next(),
                _ => break,
            };
        }
    }
    let parent = match world.get_component::<Up>(entity) {
        Ok(r) => r.parent(),
        __ => EntityKey::null().0,
    };

    let (
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
        inpass,
        animation,
        text_shadow,
        as_image,
        canvas,
        layer,
        text_overflow_data,
        context_mark,
        blend_mode,
    ) =
        (
            world.get_component::<Overflow>(entity).ok(),
            world.get_component::<IsShow>(entity).ok(),
            world.get_component::<MaskImage>(entity).ok(),
            world.get_component::<MaskImageClip>(entity).ok(),
            world.get_component::<Blur>(entity).ok(),
            world.get_component::<ZIndex>(entity).ok(),
            world.get_component::<ZRange>(entity).ok(),
            world.get_component::<ContentBox>(entity).ok(),
            world.get_component::<crate::components::calc::Quad>(entity).ok(),
            world.get_component::<TextStyle>(entity).ok(),
            world.get_component::<TextContent>(entity).ok(),
            world.get_component::<ClassName>(entity).ok(),
            world.get_component::<BackgroundImage>(entity).ok(),
            world.get_component::<BorderImage>(entity).ok(),
            world.get_component::<BackgroundColor>(entity).ok(),
            world.get_component::<BorderColor>(entity).ok(),
            world.get_component::<Opacity>(entity).ok(),
            world.get_component::<Transform>(entity).ok(),
            world.get_component::<BoxShadow>(entity).ok(),
            world.get_component::<BorderImageClip>(entity).ok(),
            world.get_component::<BorderImageSlice>(entity).ok(),
            world.get_component::<BorderImageRepeat>(entity).ok(),
            world.get_component::<BackgroundImageClip>(entity).ok(),
            world.get_component::<BorderRadius>(entity).ok(),
            world.get_component::<BackgroundImageMod>(entity).ok(),
            world.get_component::<Hsi>(entity).ok(),
            world.get_component::<TransformWillChange>(entity).ok(),
            world.get_component::<InPassId>(entity).ok(),
            world.get_component::<Animation>(entity).ok(),
            world.get_component::<TextShadow>(entity).ok(),
            world.get_component::<AsImage>(entity).ok(),
            world.get_component::<Canvas>(entity).ok(),
            world.get_component::<Layer>(entity).ok(),
            world.get_component::<TextOverflowData>(entity).ok(),
            world.get_component::<RenderContextMark>(entity).unwrap(),
            world.get_component::<BlendMode>(entity).ok(),
        );

	let mut mark_str = Vec::new();
	if context_mark.get(***mark_type_as_image).as_deref() == Some(&true) {
		mark_str.push("AsImage");
	}
	if context_mark.get(***mark_type_overflow).as_deref() == Some(&true) {
		mark_str.push("Overflow");
	}
	if context_mark.get(***mark_type_blur).as_deref() == Some(&true) {
		mark_str.push("Blur");
	}
	if context_mark.get(***mark_type_hsi).as_deref() == Some(&true) {
		mark_str.push("Hsi");
	}
	if context_mark.get(***mark_type_opacity).as_deref() == Some(&true) {
		mark_str.push("Opacity");
	}
	if context_mark.get(***mark_type_radial_wave).as_deref() == Some(&true) {
		mark_str.push("RadialWave");
	}
	if context_mark.get(***mark_type_clippath).as_deref() == Some(&true) {
		mark_str.push("ClipPath");
	}
	if context_mark.get(***mark_type_transform_willchange).as_deref() == Some(&true) {
		mark_str.push("TransformWillChange");
	}

    let pass_info = if let (Ok(instance_index), Ok(camera)) = (world.get_component::<InstanceIndex>(entity), world.get_component::<Camera>(entity)) {
        let render_obj = create_render_obj(instance_index.0.clone(), Null::null(), InstanceType::CopyFbo);
	    let view =  world.get_component::<View>(entity).unwrap();
        let has_render_target = match world.get_component::<RenderTarget>(entity) {
            Ok(render_target) => render_target.0.is_some(),
            _ => false,
        };
        let render_target1 = world.get_component::<crate::components::pass_2d::RenderTarget>(entity).unwrap();
        Some(PassInfo{
            copy_render_obj: render_obj,
            has_render_target,
            bound_box: render_target1.bound_box.clone(),
            accurate_bound_box: render_target1.accurate_bound_box.clone(),
            is_render_own: camera.is_render_own,
            draw_changed: camera.draw_changed,
            is_render_to_parent: camera.is_render_to_parent,
            transform_will_change_matrix: match world.get_component::<TransformWillChangeMatrix>(entity) {
                Ok(r) => match &r.0 {
                    Some(r) => Some((**r).clone()),
                    _ => None
                },
                _ => None,
            },
            parentpass:  world.get_component::<ParentPassId>(entity).unwrap().0.0,
            graph_id: format!("{:?}", world.get_component::<GraphId>(entity).ok()),
            view_port: camera.view_port,
            view: view.clone(),
            context_mark: mark_str.join("|"),
        })
    } else {
        None
    };
    
	
    let mut info = Info {
        pass_info,
        // char_block: char_block,
        overflow: overflow.map_or(false, |r| r.0),
		blend_mode: blend_mode.map(|r| r.clone()),
        // by_overflow: by_overflow,
        visibility: is_show.map_or(false, |r| r.get_visibility()),
        display: is_show.map_or(false, |r| r.get_display()),
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
        layout: layout1,
        world_matrix: world.get_component::<WorldMatrix>(entity).unwrap().clone(),
        border_box: absolute_b_box,
        padding_box: absolute_p_box,
        content_box: absolute_c_box,
        content_bound_box: content_box.map(|r| r.clone()),
        quad: quad.map(|r| r.clone()),
        // culling: gui.gui.culling.lend()[node_id].0,
        text: text_style.map(|r| r.clone()),
		text_shadow: text_shadow.map(|r| r.clone()),
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
        parent_id: format!("{:?}", parent),
		inpass: format!("{:?}", inpass),
        children: children,
		animation: format!("{:?}", animation),
		as_image: format!("{:?}", as_image),
		canvas: "".to_string(),
		layer: format!("{:?}", layer),
		text_overflow_data: format!("{:?}", text_overflow_data),
    };
	let canvas = canvas.map(|r| {r.clone()});
	let canvas_graph_id = if let Some(canvas) = canvas.clone() {
		world.get_component::<GraphId>(canvas.id).ok()
	} else {
		None
	};
	info.canvas = format!("{:?}, {:?}", canvas, canvas_graph_id);
    info
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommonMeterial {
    pub matrix: [f32; 16],
    pub layout_offset_or_point0: [f32; 2],
    pub layout_scale_or_point1: [f32; 2],
	pub linear_point2: [f32; 2],
    pub texture_index: f32,
    pub ty: f32,
	pub slope_x: f32,
    pub slope_y: f32,
    pub slope_origin: [f32;2],
    pub color0: [f32; 4],
	pub color_or_color1: [f32; 4],
	pub uv_or_color2: [f32; 4], // min, max  (min是左上角)
	pub stroke_color: [f32; 4],
	pub distance_px_range: f32,
    pub fill_bound: f32,
    pub stroke_bound: f32,
    pub depth: f32,
	pub sdf_uv_or_sdf0_sdf1: [f32; 4],
    pub sdf_uv2: [f32; 2], // 当为渐变颜色时存在， 表示sdfUv2
    pub render_flag: String,
}
   

impl pi_render::rhi::shader::WriteBuffer for CommonMeterial {
	fn write_into(&self, index: u32, buffer: &mut [u8]) {
		
		unsafe { std::ptr::copy_nonoverlapping(
			self as *const Self as usize as *const u8,
			buffer.as_mut_ptr().add(index as usize),
			216,
		) };
	}
	#[inline]
	fn byte_len(&self) -> u32 {
		216
	}

	#[inline]
	fn offset(&self) -> u32 {
		0
	}
}
impl pi_render::rhi::shader::Uniform for CommonMeterial {
	type Binding = MeterialBind;
}

impl GetBuffer for CommonMeterial {
	fn get_data(&mut self, index: u32, buffer: &[u8]) {
		let len = self.byte_len() as usize;
		unsafe {
			buffer.as_ptr().add(index as usize + self.offset() as usize).copy_to_nonoverlapping(self as *const Self as usize as *mut u8, len);
		};
	}
}



#[derive(Serialize, Deserialize, Debug)]
pub struct TexInfo {
    pub name: Option<Atom>,
    pub size: f64,
    pub is_used: bool,
    pub timeout: f64
}

pub fn texture_info(world: &mut World) -> String {
    use pi_render::rhi::asset::AssetWithId;
    let mut res = Vec::new();
    let info = world.get_single_res::<ShareAssetMgr<AssetWithId<TextureRes>>>().unwrap();
    
    for info in &info.account().used{
        res.push(TexInfo{ name: get_by_hash(info.name.clone().parse::<pi_atom::Usize>().unwrap()), size: info.size as f64, is_used: true,  timeout: info.remain_timeout as f64})
    }

    for info in &info.account().unused{
        res.push(TexInfo{ name: get_by_hash(info.name.clone().parse::<pi_atom::Usize>().unwrap()), size: info.size as f64, is_used: false,  timeout: info.remain_timeout as f64})
    }
    serde_json::to_string(&res).unwrap()
}

pub fn debug_info(world: &mut World) -> Vec<f64> {

    let mut res = Vec::new();
    let info = world.get_single_res::<DebugInfo>().unwrap();
    res.push(info.font_size as f64);
    res.push(info.draw_obj_count as f64);
    res.push(world.entities_iter().size_hint().0 as f64);
    res.push(world.mem_size() as f64);
    res
}

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


pub fn get_world_matrix(world: &World, node_id: Entity) -> String {

    let world_matrix = match world.get_component::<WorldMatrix>(node_id) {
        Ok(r) => r,
        _ => return "undefined".to_string(),
    };

	serde_json::to_string(world_matrix).unwrap()
}

pub fn get_transform(world: &World, node_id: Entity) -> String {
    let transform = match world.get_component::<Transform>(node_id) {
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

