// 解析html， 并渲染
// html中只能包含一个根节点, 如果存在多个， 只会渲染第一个
#![feature(str_as_str)]

#[path = "../framework.rs"]
mod framework;

use framework::{Param, Example};
use pi_flex_layout::style::{Dimension, PositionType};
use pi_hash::XHashMap;
use pi_null::Null;
use pi_style::{style::{Aabb2, Point2}, style_parse::parse_class_map_from_string, style_type::{AsImageType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, WidthType}};
use pi_ui_render::{
    components::{calc::EntityKey, user::{CgColor, ClearColor, Viewport}},
    resource::{ExtendCssCmd, FragmentCommand, NodeCmd},
};
use pi_ui_render::resource::fragment::NodeTag;

use std::{collections::VecDeque, io::Read};

use cssparser::{Parser, ParserInput};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::parse_document;
use markup5ever_rcdom::{Handle, RcDom};
use pi_style::style_parse::parser_style_items;
use pi_ui_render::components::user::serialize::{SvgShapeType, SvgTypeAttr};
use pi_ui_render::components::user::svg_parser_style_items;
use pi_ui_render::resource::fragment::{Attributes, Fragments, NodeFragment};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
fn main() { framework::start(QuadExample::default()) }

#[derive(Default)]
pub struct QuadExample;

impl Example for QuadExample {
    fn init(&mut self, mut world: Param, size: (usize, usize)) {
        let args: Vec<String> = std::env::args().collect();
        log::warn!("args========{:?}",args);
        let mut example_name = None;
        let mut arg_iter =  args.iter();
        while let Some(n) = arg_iter.next()  {
            if !n.ends_with(".exe") {
                example_name = Some(n.clone());
                break;
            }
        }

        let example_name = example_name.unwrap_or("css".to_string());
        let path = "examples/a_html/cases/".to_string() + example_name.as_str() + ".html";

        let fragments = parse_html(path.as_str(), &mut world);
        let entity_len: usize = fragments.len();

        if entity_len == 0 {
            return;
        }

        // 创建fragment
        let mut fragments = Fragments {
            fragments,
            map: XHashMap::default(),
        };
        let fragment_key = 5;/*随便的key*/
        fragments.map.insert(fragment_key, 0..entity_len);
        world.user_cmd.extend_fragment_bin(pi_ui_render::resource::ExtendFragmentCmd(fragments));

        // 添加根节点
        let root = world.spawn(NodeTag::Div);
        world.user_cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        world.user_cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));
        world.user_cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        world.user_cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
        world.user_cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        world.user_cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        world.user_cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
		world.user_cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
        world.user_cmd.append(root, EntityKey::null().0);
        
        // 更具fragment， 创建节点树
        let mut entitys = Vec::with_capacity(entity_len);
        for _ in 0..entity_len {
            let entity = world.insert.insert(());
            entitys.push(entity);
        }
        let body = entitys[0].clone();
        world.user_cmd.fragment_commands.push(FragmentCommand { key: fragment_key, entitys});
        world.user_cmd.append(body, root);
    }
}


pub fn parse_html(path: &str,  world: &mut Param) -> Vec<NodeFragment> {
    let current_dir = std::env::current_dir().unwrap();
    let p = current_dir.join(path);
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    #[cfg(target_os = "android")]
    let data = include_bytes!("./cases/css.html").to_vec();
    #[cfg(target_os = "windows")]
    let data = std::fs::read(p).unwrap();
    // let data = data.as_mut_slice();
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut data.as_slice())
        .unwrap();
    
    let mut fragments = Vec::new();
    dom_to_fragment(&dom.document, Null::null(), &mut fragments, world);
    fragments
}

fn dom_to_fragment(handle: &Handle, mut parent: u32, fragments: &mut Vec<NodeFragment>, world: &mut Param) {
   parse_element(handle, parent, fragments, world);

    if fragments.len() > 0 {
        parent = (fragments.len() - 1) as u32;
    }

    let children = handle.children.borrow();
    for child in children.iter() {
        dom_to_fragment(child, parent, fragments, world);
    }
}

fn parse_element(handle: &Handle, parent: u32, fragments: &mut Vec<NodeFragment>, world: &mut Param) {
    // println!("========== hadle: {:?}", handle);
     match &handle.data {
        markup5ever_rcdom::NodeData::Element { name, attrs, .. } => {
           let tag = match &*name.local {
                "div" => NodeTag::Div,
                "image" => NodeTag::Image,
                "img" => NodeTag::Image,
                "span" => NodeTag::Span,
                "canvas" => NodeTag::Canvas,
                "tempalte" => NodeTag::VNode,
                "svg" => NodeTag::Svg,
                "rect" => NodeTag::Rect,
                "circle" => NodeTag::Circle,
                "ellipse" => NodeTag::Ellipse,
                "line" => NodeTag::Line,
                "polygon" => NodeTag::Polygon,
                "polyline" => NodeTag::Polyline,
                "path" => NodeTag::Path,
                "filter" => NodeTag::Filter,
                "linear-gradient" => NodeTag::LinearGradient,
                "stop" => NodeTag::Stop,
                "defs" => NodeTag::Defs,
                "fe-drop-shadow" => NodeTag::FeDropShadow, 
                r => {
                    // println!("==========r: {}", r);
                    match r{
                        "html" | "head" | "body" => (),
                        _ => log::error!("不支持节点： {:?}", (r, attrs)),
                        
                    };
                    return;
                }
            };

            let mut node = NodeFragment {
                tag,
                parent,
                style: Attributes::GuiAttributes(Default::default()),
                class: Default::default(),
            };


            for attr in attrs.borrow().iter() {
                // println!("========== &*attr.name.local: {}", &*attr.name.local);
                match &*attr.name.local {
                    "style" => {
                        
                        let style = &*attr.value;
                        let mut input = ParserInput::new(style);
                        let mut parse = Parser::new(&mut input);
                        let style = if let NodeTag::Svg  | NodeTag::Rect | NodeTag::Circle | NodeTag::Ellipse| NodeTag::Line | NodeTag::Polygon | NodeTag::Polyline | NodeTag::Path | NodeTag::Defs
                        |  NodeTag::Filter | NodeTag::LinearGradient | NodeTag::Stop | NodeTag::FeDropShadow = tag {
                            let mut styles = VecDeque::new();
                            // println!("========== tag.to_svg_shape(): {:?}", (tag, tag.to_svg_shape()));
                            if let Some(shape) = tag.to_svg_shape(){
                                styles.push_back(SvgTypeAttr::SvgShape(SvgShapeType(shape)));
                            }
                            svg_parser_style_items(&mut parse, &mut styles, 0, tag);
                            Attributes::SvgAttributes(styles)
                        } else { 
                            let mut styles = VecDeque::new(); 
                            parser_style_items(&mut parse, &mut styles, 0);
                            Attributes::GuiAttributes(styles)
                        };
                        node.style = style;
                    },
                    "class" => {
                        for s in attr.value.as_str().split(" "){
                            // println!("=========== class: {:?}", s.split_at(1).1.parse::<u32>().unwrap());
                            node.class.push(s.split_at(1).1.parse::<u32>().unwrap())
                        }
                       
                    }
                    _ => {}
                };
            }
            fragments.push(node);
                        
            // println!("=============={:?}, {:?}", name, attrs); 
        },
        markup5ever_rcdom::NodeData::Text{contents} => {
             let class_map = parse_class_map_from_string(contents.take().as_str(), 0).unwrap();
            //  println!("========= class_map: {:?}", class_map);
             world.user_cmd.push_cmd(ExtendCssCmd(vec![class_map]));
        }
        _ => (),
    };
    
}

