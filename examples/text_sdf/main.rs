
// // 一个简单的四边形渲染

// #[path = "../framework.rs"]
// mod framework;

// use std::mem::swap;


// use pi_world::prelude::World;
// use framework::{Param, Example};
// use pi_atom::Atom;
// use pi_flex_layout::style::{Dimension, PositionType};
// use pi_hal::font::sdf_brush::FontCfg;
// use pi_null::Null;
// use pi_ui_render::{
//     components::{
//         calc::EntityKey,
//         user::{CgColor, ClearColor, Color, FontSize, RenderDirty, Viewport},
// 
//     },
//     resource::{NodeCmd, UserCommands, ShareFontSheet},
// };
// use smallvec::smallvec;

// fn main() { framework::start(QuadExample::default()) }
// use pi_style::{
//     style::{Aabb2, Point2, TextContent},
//     style_type::{
//         BackgroundColorType, ColorType, FontFamilyType, FontSizeType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType,
//         PositionTypeType, TextContentType, WidthType, AsImageType,
//     },
// };

// #[derive(Default)]
// pub struct QuadExample {
//     cmd: UserCommands,
//     root: EntityKey,
// }

// impl Example for QuadExample {
//     fn init(&mut self, mut world: Param, size: (usize, usize)) {
// 		{
// 			let font_sheet = world.get_single_res_mut::<ShareFontSheet>().unwrap();
// 			let mut font_sheet = font_sheet.borrow_mut();
// 			let sdf_cfg = match postcard::from_bytes::<FontCfg>(include_bytes!("../z_source/hwxw.sdf").as_slice()) {
// 				Ok(r) => r,
// 				Err(e) => {
// 					panic!("parse fail================{:?}", e);
// 				}
// 			};
			
// 			font_sheet.font_mgr_mut().add_sdf_cfg(sdf_cfg);
// 			font_sheet.font_mgr_mut().add_sdf_default_char(Atom::from("hwxw"), '□')
// 		}
		

//         // 添加根节点
//         let root = world.spawn();
//         self.root = EntityKey(root);
//         self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
//         self.cmd.push_cmd(NodeCmd(
//             Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
//             root,
//         ));
//         self.cmd.push_cmd(NodeCmd(RenderDirty(true), root));

//         self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
//         self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

//         self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
//         self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
//         self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
// 		self.cmd.set_style(root, AsImageType(pi_style::style::AsImage::Force));
//         self.cmd
//             .set_style(root, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 1.0, 1.0, 1.0))));

//         self.cmd.append(root, EntityKey::null().0);

//         // 添加一个红色div
//         let div1 = world.spawn();
//         self.cmd.set_style(div1, WidthType(Dimension::Points(50.0)));
//         self.cmd.set_style(div1, HeightType(Dimension::Points(100.0)));
//         self.cmd.set_style(div1, PositionTopType(Dimension::Points(20.0)));
//         self.cmd.set_style(div1, PositionLeftType(Dimension::Points(20.0)));
//         self.cmd
//             .set_style(div1, TextContentType(TextContent("庆".to_string(), Atom::from("庆"))));
    
//         // rgb(255,0,0) 0px 0px 5px, rgb(255,0,0) 0px 0px 3px, rgb(255,255,255) 0px 0px 1px;
//         self.cmd.set_style(div1, FontFamilyType(Atom::from("hwxw")));
// 		  self.cmd.set_style(div1, pi_style::style_type::TextStrokeType(pi_style::style::Stroke {
//         	width: unsafe {ordered_float::NotNan::new_unchecked(2.0)},
//         	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
//         self.cmd.set_style(div1, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
//         self.cmd.set_style(div1, FontSizeType(FontSize::Length(17)));
//         // self.cmd.set_style(div1, TextStrokeType(Stroke {
//         // 	width: unsafe {NotNan::new_unchecked(2.0)},
//         // 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
//         self.cmd.append(div1, root);

//         let div2 = world.spawn();
//         self.cmd.set_style(div2, WidthType(Dimension::Points(50.0)));
//         self.cmd.set_style(div2, HeightType(Dimension::Points(100.0)));
//         self.cmd
//             .set_style(div2, TextContentType(TextContent("庆".to_string(), Atom::from("庆"))));
//         self.cmd.set_style(div2, FontFamilyType(Atom::from(""))); // 测试使用默认文字的情况
//         self.cmd.set_style(div2, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
//         self.cmd.set_style(div2, FontSizeType(FontSize::Length(17)));
//         // self.cmd.set_style(div1, TextStrokeType(Stroke {
//         // 	width: unsafe {NotNan::new_unchecked(2.0)},
//         // 	color: CgColor::new(1.0, 0.0, 0.0, 1.0)}));
//         self.cmd.append(div2, root);


// 		let div3 = world.spawn();
//         self.cmd.set_style(div3, WidthType(Dimension::Points(50.0)));
//         self.cmd.set_style(div3, HeightType(Dimension::Points(100.0)));
//         self.cmd.set_style(div3, PositionTopType(Dimension::Points(70.0)));
//         self.cmd.set_style(div3, PositionLeftType(Dimension::Points(70.0)));
//         self.cmd
//             .set_style(div3, TextContentType(TextContent("庆".to_string(), Atom::from("庆"))));
    
//         // rgb(255,0,0) 0px 0px 5px, rgb(255,0,0) 0px 0px 3px, rgb(255,255,255) 0px 0px 1px;
//         self.cmd.set_style(div3, FontFamilyType(Atom::from("hwxw")));
//         self.cmd.set_style(div3, ColorType(Color::RGBA(CgColor::new(0.0, 1.0, 0.0, 1.0))));
//         self.cmd.set_style(div3, FontSizeType(FontSize::Length(17)));
// 		self.cmd.set_style(
//             div3,
//             pi_style::style_type::TextShadowType(smallvec![
//                 pi_style::style::TextShadow {
//                     h: 0.0,
//                     v: 0.0,
//                     blur: 5.0,
//                     color: CgColor::new(1.0, 0.0, 0.0, 1.0)
//                 },
//                 pi_style::style::TextShadow {
//                     h: 0.0,
//                     v: 0.0,
//                     blur: 3.0,
//                     color: CgColor::new(1.0, 0.0, 0.0, 1.0)
//                 },
//             ]),
//         );
//         self.cmd.append(div3, root);
//     }

//     fn render(&mut self, cmd: &mut UserCommands) {
//         self.cmd.push_cmd(NodeCmd(RenderDirty(true), self.root.0));
//         swap(&mut self.cmd, cmd);
//     }

// 	fn use_sdf(&self) -> bool {
// 		true
// 	}
// }

fn main() {}

