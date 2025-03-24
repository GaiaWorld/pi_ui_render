//! 文字劈分系統
//! 将文字劈分为字符，放入NodeState中，并设置好每个字符的布局宽高。等待布局系统布局
use std::intrinsics::transmute;

use pi_world::{filter::Or, prelude::{Changed, Entity, Mut, OrDefault, Query, SingleResMut}};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Up, Layer};

use ordered_float::NotNan;
use pi_flex_layout::{
    prelude::{CharNode, Rect, Size},
    style::{AlignContent, AlignItems, Dimension, FlexWrap, JustifyContent, PositionType},
};
use pi_null::Null;
use pi_render::font::{split, Font, FontId, FontSheet, GlyphId, SplitResult};
use pi_slotmap::DefaultKey;
use pi_style::style::{StyleType, TextAlign, VerticalAlign, TextOverflow};

use crate::{
    components::{
        calc::{EntityKey, NodeState, StyleMark},
        user::{get_size, FlexContainer, FlexNormal, LineHeight, Size as FlexSize, TextContent, TextOverflowChar, TextOverflowData, TextStyle},
    },
    resource::{IsRun, ShareFontSheet},
};

/// 文字劈分
/// 将可以简单布局的问文字节点转化为。。
/// 将需要图文混排的文字节点，劈分为单个文字节点
pub fn text_split(
    mut query: Query<
        (
            Entity,
            &'static TextContent,
            OrDefault<TextStyle>,
            &'static Up,
            OrDefault<FlexSize>,
            OrDefault<FlexNormal>,
            &'static mut NodeState,
            &'static StyleMark,
            Option<&'static mut FlexContainer>,
            &Layer,
			Option<&'static mut TextOverflowData>,
        ),
        Or<(Changed<TextContent>, Changed<TextStyle>, Changed<Layer>, Changed<TextOverflowData>)>,
    >,
    font_sheet: SingleResMut<ShareFontSheet>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let mut font_sheet = font_sheet.0.borrow_mut();
    for (entity, text_content, text_style, up, size, normal_style, node_state, style_mark, flex_container, layer, text_overflow_data) in query.iter_mut() {
        if layer.layer().is_null() {
            continue;
        }

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
            // text_style.text_stroke.width, // todo 或许应该设置比例
            // None,
            // None,
        ));

        let font_height = font_sheet.font_height(font_id, font_size as usize);
        // 字体描边宽度
        let sw = text_style.text_stroke.width;

        if let Some(r) = flex_container {
            fit_text_style(text_style, style_mark, r);
        }

        let mut calc = Calc {
            id: entity,
            text: (**text_content).0.as_ref(),
            text_style: text_style,

            font_sheet: &mut font_sheet,
            font_id,
            font_size,
            line_height: get_line_height(font_height as usize, &text_style.line_height),
            sw: sw,
            char_margin: text_style.letter_spacing,
            node_state: node_state,
        };

        // 将文字劈分为字符形式，放入nodestate中
        calc.calc_simple();

        // 如果父节点没有其它子节点，或者，自身定义了宽度或高度，则可使用简单布局
        if !EntityKey(up.parent()).is_null() && EntityKey(up.prev()).is_null() && EntityKey(up.next()).is_null() {
            calc.node_state.set_vnode(false);
        } else if size.width != Dimension::Undefined || size.height != Dimension::Undefined || normal_style.position_type == PositionType::Absolute {
            calc.node_state.set_vnode(false);
        } else {
            calc.node_state.set_vnode(true);
        }

		// 如果存在text_overflow, 并且相关属性更改， 需要重新劈分text_overflow的字符
		if let Some(mut text_overflow) = text_overflow_data {
			let TextOverflowData { text_overflow, text_overflow_char } = text_overflow.bypass_change_detection();
			text_overflow_char.clear();
			
			match &text_overflow {
				TextOverflow::Ellipsis => {
					let width = font_sheet.measure_width(font_id, '.');
					text_overflow_char.push(TextOverflowChar {
						width,
						ch: '.',
						ch_id: DefaultKey::default(),
					});
				},
				TextOverflow::Custom(s) => {
					for c in s.chars() {
						let width = font_sheet.measure_width(font_id, c);
						text_overflow_char.push(TextOverflowChar {
							width,
							ch: c,
							ch_id: DefaultKey::default(),
						});
					}
					
				},
				_ => (),
			}
		}

        // event_writer.send(ComponentEvent::new(entity));
    }
}

#[allow(dead_code)]
struct Calc<'a> {
    id: Entity,
    text: &'a str,
    text_style: &'a TextStyle,

    font_sheet: &'a mut FontSheet,
    font_id: FontId,
    font_size: usize,
    line_height: f32,
    sw: NotNan<f32>,
    char_margin: f32,
    node_state: Mut<'a, NodeState>,
}

impl<'a> Calc<'a> {
    // 简单布局， 将文字劈分，单词节点的内部字符使用绝对布局，其余节点使用相对布局
    // 与图文混排的布局方式不同，该布局不需要为每个字符节点创建实体
    #[allow(unused_variables)]
    fn calc_simple(&mut self) {
        let (id, text_style, text, node_state) = (self.id, self.text_style, self.text, &mut self.node_state);

        let chars: &'static mut Vec<CharNode> = unsafe { transmute(&mut node_state.text) };
        let (mut word_index, mut p_x, mut char_index) = (0, 0.0, 0);

        if text_style.text_indent > 0.0 {
            self.create_or_get_indice(chars, text_style.text_indent, char_index, -1);
            char_index += 1;
        }

        // 根据每个字符, 创建charNode
        for cr in self.font_sheet.font_mgr_mut().split(self.font_id, text, true, text_style.white_space.preserve_spaces()) {
            println!("cacl_simple, cr: {:?}, char_index:{}, word_index: {}, p_x:{}", cr, char_index, word_index, p_x);
            // 如果是单词的结束字符，释放掉当前节点后面的所有兄弟节点， 并将当前节点索引重置为当前节点的父节点的下一个兄弟节点
            match cr {
                SplitResult::Word(char_i, c, id) => {
                    let cn = self.create_or_get(c, id, chars, char_index, p_x, char_i);
                    if let Some(id) = id { cn.ch_id = *id; }
                    char_index += 1;
                }
                SplitResult::WordNext(char_i, c, id) => {
                    let cn = self.create_or_get(c, id, chars, char_index, p_x, char_i);
                    cn.context_id = word_index as isize;
                    if let Some(id) = id { cn.ch_id = *id; }
                    p_x += dimension_points(cn.size.width) + self.char_margin; // 下一个字符的位置
                    char_index += 1;
                    chars[word_index].count += 1;
                }
                // 存在WordStart， 表示开始一个多字符单词
                SplitResult::WordStart(char_i, c, id) => {
                    self.create_or_get_container(chars, char_index, -1);
                    word_index = char_index;
                    p_x = 0.0;
                    char_index += 1;

                    let cn = self.create_or_get(c, id, chars, char_index, p_x, char_i);
                    println!("===== WordStart: {:?}", (&cn, id));
                    cn.context_id = word_index as isize;
                    if let Some(id) = id { cn.ch_id = *id; }
                    p_x += dimension_points(cn.size.width) + self.char_margin; // 下一个字符的位置
                    chars[word_index].count += 1;
                    char_index += 1;
                }
                SplitResult::WordEnd(_) => {
                    chars[word_index].size = Size {
                        width: Dimension::Points(p_x - self.char_margin),
                        height: Dimension::Points(self.line_height),
                    };
                }
                SplitResult::Whitespace(char_i) => {
                    let cn = self.create_or_get(' ', None, chars, char_index, p_x, char_i);
                    char_index += 1;
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

    fn create_char_node(&mut self, ch: char, glyph_id: Option<GlyphId>, p_x: f32, char_i: isize) -> CharNode {
        let width = if let Some(id) = glyph_id {self.font_sheet.font_mgr_mut().measure_width_of_glyph_id(self.font_id, id)} else {0.0};
        CharNode {
            ch,
            size: Size {
                width: Dimension::Points(width),
                height: Dimension::Points(self.line_height),
            },
            pos: Rect {
                left: p_x,
                top: 0.0,
                right: p_x + width,
                bottom: self.line_height,
            },
            count: 0,
            ch_id: DefaultKey::null(),
            char_i,
            context_id: -1,
        }
    }

    fn create_or_get<'b>(&mut self, ch: char, glyph_id: Option<GlyphId>, chars: &'b mut Vec<CharNode>, index: usize, p_x: f32, char_i: isize) -> &'b mut CharNode {
        if index >= chars.len() {
            chars.push(self.create_char_node(ch, glyph_id, p_x, char_i));
        } else {
            let cn = &chars[index];
            if cn.ch != ch {
                chars[index] = self.create_char_node(ch, glyph_id, p_x, char_i);
            }
        }
        let cn = &mut chars[index];
        cn.pos.left = p_x;
        cn.char_i = char_i;
        cn
    }

    fn create_or_get_container<'b>(&mut self, chars: &'b mut Vec<CharNode>, index: usize, char_i: isize) -> &'b mut CharNode {
        let r = CharNode {
            ch: char::from(0),
            size: Size {
                width: Dimension::Points(0.0),
                height: Dimension::Points(self.line_height),
            },
            pos: Rect {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
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
        LineHeight::Number(r) => *r + size as f32,  //设置数字，此数字会与当前的字体尺寸相加来设置行间距。
        LineHeight::Percent(r) => *r * size as f32, //	基于当前字体尺寸的百分比行间距.
        LineHeight::Normal => size as f32,
    }
}

fn fit_text_style(style: &TextStyle, style_mark: &StyleMark, mut flex_container: Mut<FlexContainer>) {
    let StyleMark {
        local_style, class_style, ..
    } = style_mark;
    // 兼容目前使用父节点的对齐属性来对齐文本， 如果项目将其修改正确， 应该去掉该段TODO
    if !local_style[StyleType::JustifyContent as usize] && !class_style[StyleType::JustifyContent as usize] {
        flex_container.justify_content = match style.text_align {
            TextAlign::Center => JustifyContent::Center,
            TextAlign::Right => JustifyContent::FlexEnd,
            TextAlign::Left => JustifyContent::FlexStart,
            TextAlign::Justify => JustifyContent::SpaceBetween,
        };
    }

    if !local_style[StyleType::AlignItems as usize] && !class_style[StyleType::AlignItems as usize] {
        let r = match style.vertical_align {
            VerticalAlign::Middle => AlignItems::Center,
            VerticalAlign::Bottom => AlignItems::FlexEnd,
            VerticalAlign::Top => AlignItems::FlexStart,
        };
        flex_container.align_items = r;
    }
    if !local_style[StyleType::AlignContent as usize] && !class_style[StyleType::AlignContent as usize] {
        let r = match style.vertical_align {
            VerticalAlign::Middle => AlignContent::Center,
            VerticalAlign::Bottom => AlignContent::FlexEnd,
            VerticalAlign::Top => AlignContent::FlexStart,
        };
        flex_container.align_content = r;
    }

    if !local_style[StyleType::FlexWrap as usize] && !class_style[StyleType::FlexWrap as usize] {
        flex_container.flex_wrap = if style.white_space.allow_wrap() {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        }
    }
}
