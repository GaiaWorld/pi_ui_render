//! 文字劈分系統
//! 将文字劈分为字符，放入NodeState中，并设置好每个字符的布局宽高。等待布局系统布局
use std::intrinsics::transmute;

use ordered_float::{NotNaN, NotNan};
use pi_ecs::prelude::{Query, Changed, Or, ResMut, OrDefault, Write, Id, Component};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::{EntityTree, Layer};
use pi_flex_layout::{prelude::{CharNode, Size, Rect}, style::{Dimension, PositionType}};
use pi_share::{Share, ShareCell};
use pi_slotmap::{DefaultKey, Key};
use pi_slotmap_tree::Storage;

use crate::{
	components::{
		user::{TextContent, Node, TextStyle, FontStyle, FlexNormal, Size as FlexSize, LineHeight}, 
		calc::NodeState
	}, 
	utils::stdcell::StdCell, font::{font::{FontMgr, get_size, FontSheet, Font, FontId}, text_split::{split, SplitResult}},

};

pub struct CalcTextSplit;

#[setup]
impl CalcTextSplit {
	/// 文字劈分
	/// 将可以简单布局的问文字节点转化为。。
	/// 将需要图文混排的文字节点，劈分为单个文字节点
	#[system]
	pub fn text_split(
		query: Query<
			Node, 
			(
				Id<Node>,
				&TextContent,
				OrDefault<TextStyle>
			), 
			Or<(Changed<TextContent>, Changed<FontStyle>, Changed<Layer>)>
		>,
		style: Query<
			Node, 
			(
				OrDefault<FlexSize>,
				OrDefault<FlexNormal>,
			)
		>,
		tree: EntityTree<Node>,
		mut node_states: Query<Node, Write<NodeState>>,
		font_sheet: ResMut<Share<ShareCell<FontMgr>>>
		
	) {
		let mut font_sheet = font_sheet.borrow_mut();
		for (
			entity,
			text_content, 
			text_style) in query.iter() {
			
			match tree.get_layer(entity) {
				Some(r) => if *r == 0 {return},
				None => return,
			};
			let up = tree.get_up(entity).unwrap();
			// // 取到字体详情
			// let tex_font: (TexFont<T>, usize) = match font_sheet.get_font_info(&text_style.font_family) {
			// 	Some(r) => (r.0.clone(), r.1),
			// 	None => {
			// 		log::error!("font is not exist, face_name: {:?}, id: {:?}",
			// 			text_style.font_family,
			// 			entity,
			// 		);
			// 		panic!("");
			// 	},
			// };
			
			// 字体大小，根据font-size样式，计算字体的绝对大小
			let font_size = get_size(&text_style.font_size);
			// 获得字体高度
			// let font_height = tex_font
			// 	.0
			// 	.get_font_height(font_size as usize, text_style.text_stroke.width);
			let font_id = font_sheet.font_id(Font::new(
				text_style.font_family.clone(),
				font_size,
				text_style.font_weight,
				text_style.text_stroke.width, // todo 或许应该设置比例
			));

			let font_height = font_sheet.font_height(font_id, font_size as usize);
			// 字体描边宽度
			let sw = text_style.text_stroke.width;

			let mut calc = Calc {
				id: entity,
				text: text_content.0.as_ref(),
				text_style: text_style,

				font_sheet: &mut font_sheet,
				font_id,
				font_size,
				line_height: get_line_height(font_height as usize, &text_style.line_height),
				sw: sw,
				char_margin: text_style.letter_spacing - *sw,
				node_states: &mut node_states,
			};

			// 将文字劈分为字符形式，放入nodestate中
			calc.cacl_simple();

			// 如果父节点没有其它子节点，或者，自身定义了宽度或高度，则可使用简单布局
			let mut node_state_item = node_states.get_unchecked_mut(entity);
			let node_state = node_state_item.get_mut().unwrap();
			let (size, normal_style) = style.get(entity).unwrap();
			if !up.parent().is_null() && up.prev().is_null() && up.next().is_null() {
				node_state.set_vnode(false);
			} else if size.width != Dimension::Undefined || size.height != Dimension::Undefined || normal_style.position_type == PositionType::Absolute {
				node_state.set_vnode(false);
			}else {
				node_state.set_vnode(true);
			}

			node_state_item.notify_modify();
		}
	}
}	



struct Calc<'a> {
	id: Id<Node>,
	text: &'a str,
	text_style: &'a TextStyle,

	font_sheet: &'a mut FontMgr,
	font_id: FontId,
	font_size: usize,
	line_height: f32,
	sw: NotNan<f32>,
	char_margin: f32,
	node_states: &'a mut Query<Node, Write<NodeState>>
}

impl<'a> Calc<'a> {
	// 简单布局， 将文字劈分，单词节点的内部字符使用绝对布局，其余节点使用相对布局
	// 与图文混排的布局方式不同，该布局不需要为每个字符节点创建实体
	fn cacl_simple(&mut self) {
		let (id, text_style, text) = (self.id, self.text_style, self.text);
		let mut node_state = self.node_states.get_unchecked_mut(id);
		let node_state = node_state.get_mut().unwrap();
		
		let chars: &'static mut Vec<CharNode> = unsafe { transmute( &mut node_state.text ) };
		let (mut word_index, mut p_x, mut word_margin_start, mut char_index) = (0, 0.0, 0.0, 0);

		if text_style.text_indent > 0.0 {
			self.create_or_get_indice(chars, text_style.text_indent, char_index, -1);
			char_index += 1;
		}

		// 根据每个字符, 创建charNode
		for cr in split(text, true, text_style.white_space.preserve_spaces()) {
			// println!("cacl_simple, cr: {:?}, char_index:{}, word_index: {}, word_margin_start: {}, p_x:{}", cr, char_index, word_index, word_margin_start, p_x);
			// 如果是单词的结束字符，释放掉当前节点后面的所有兄弟节点， 并将当前节点索引重置为当前节点的父节点的下一个兄弟节点
			match cr {
				SplitResult::Word(char_i,c) => {
					let cn = self.create_or_get(c, chars, char_index, p_x, char_i);
					cn.margin.left = Dimension::Points(word_margin_start);
					char_index += 1;
					word_margin_start = self.char_margin;
				}
				SplitResult::WordNext(char_i,c) => {
					let cn = self.create_or_get(c, chars, char_index, p_x, char_i);
					cn.context_id = word_index as isize;
					p_x += dimension_points(cn.size.width) + self.char_margin; // 下一个字符的位置
					char_index += 1;
					chars[word_index].count += 1;
				}
				// 存在WordStart， 表示开始一个多字符单词
				SplitResult::WordStart(char_i,c) => {
					self.create_or_get_container(chars, char_index, word_margin_start, -1);
					word_index = char_index;
					p_x = 0.0;
					word_margin_start = self.char_margin;
					char_index += 1;

					let cn = self.create_or_get(c, chars, char_index, p_x, char_i);
					cn.context_id = word_index as isize;
					p_x += dimension_points(cn.size.width) + self.char_margin; // 下一个字符的位置
					chars[word_index].count += 1;
					char_index += 1;
				}
				SplitResult::WordEnd(_) => {
					chars[word_index].size = Size {width: Dimension::Points(p_x - self.char_margin), height: Dimension::Points(self.line_height)};
				},
				SplitResult::Whitespace(char_i) => {
					let cn = self.create_or_get(' ', chars, char_index, p_x, char_i);
					cn.margin.left = Dimension::Points(word_margin_start); //word_margin_start;
					char_index += 1;
					word_margin_start = self.char_margin;
					// word_margin_start += self.font_size/3.0 + self.word_margin;
				}
				SplitResult::Newline(char_i) => {
					self.create_or_get_breakline(chars, char_index, char_i);
					char_index += 1;
				}
			}
		}

		while char_index < chars.len() {
			chars.pop();
		}
	}

	fn create_char_node(&mut self, ch: char, p_x: f32, char_i: isize) -> CharNode {
		let width = self.font_sheet.measure_width(
			self.font_id,
			ch,
		);

		CharNode {
			ch,
			size: Size{width:Dimension::Points(width), height:Dimension::Points(self.line_height)},
			margin: Rect {
				left: Dimension::Points(self.char_margin),
				top:  Dimension::Points(0.0),
				right:  Dimension::Points(0.0),
				bottom:  Dimension::Points(0.0),
			},
			pos: Rect {
				left: p_x,
				top: 0.0,
				right: 0.0,
				bottom: 0.0
			},
			count: 0,
			ch_id: DefaultKey::null(),
			char_i,
			context_id: -1,
		}
	}

	fn create_or_get<'b>(&mut self, ch: char, chars: &'b mut Vec<CharNode>, index: usize, p_x: f32, char_i: isize) -> &'b mut CharNode {
		if index >= chars.len() {
			chars.push(self.create_char_node(ch, p_x, char_i));
		} else {
			let cn = &chars[index];
			if cn.ch != ch {
				chars[index] = self.create_char_node(ch, p_x, char_i);
			}
		}
		let cn = &mut chars[index];
		cn.pos.left = p_x;
		cn.char_i = char_i;
		cn
	}

	fn create_or_get_container<'b>(&mut self, chars: &'b mut Vec<CharNode>, index: usize, word_margin_start: f32, char_i: isize) -> &'b mut CharNode {
		let r = CharNode {
			ch: char::from(0),
			size: Size{width: Dimension::Points(0.0), height: Dimension::Points(self.line_height)},
			margin: Rect { 
				left: Dimension::Points(word_margin_start), 
				right: Dimension::Points(0.0), 
				top: Dimension::Points(0.0), 
				bottom: Dimension::Points(0.0) 
			},
			pos: Rect {
				top: 0.0, right: 0.0, bottom: 0.0, left: 0.0
			},
			count: 1,
			ch_id: DefaultKey::null(),
			char_i,
			context_id: -1,
		};
		if index >= chars.len() {
			chars.push(r);
		} else {
			chars[index] = r;
		}
		&mut chars[index]
	}

	fn create_or_get_breakline<'b>(&mut self, chars: &'b mut Vec<CharNode>, index: usize, char_i: isize) -> &'b mut CharNode {
		if index == chars.len() {
			let c = CharNode::default();
			chars.push(c);
		}
		let c = &mut chars[index];
		c.size.height = Dimension::Points(self.line_height);
		c.char_i = char_i;
		c.ch = '\n';
		c
	}

	fn create_or_get_indice<'b>(&mut self, chars: &'b mut Vec<CharNode>, indice: f32, index: usize, char_i: isize) -> &'b mut CharNode {
		if index == chars.len() {
			let c = CharNode::default();
			chars.push(c);
		} 

		let c = &mut chars[index];
		c.size.height = Dimension::Points(self.line_height);
		c.size.width = Dimension::Points(indice);
		c.char_i = char_i;
		c.ch = ' ';
		
		c
	}
}

fn dimension_points(v: Dimension) -> f32 {
	match v {
		Dimension::Points(r) => r,
		_ => panic!(""),
	}
}

// 行高
pub fn get_line_height(size: usize, line_height: &LineHeight) -> f32 {
    match line_height {
        LineHeight::Length(r) => *r,                //固定像素
        LineHeight::Number(r) => *r + size as f32, //设置数字，此数字会与当前的字体尺寸相加来设置行间距。
        LineHeight::Percent(r) => *r * size as f32, //	基于当前字体尺寸的百分比行间距.
        LineHeight::Normal => size as f32,
    }
}