/// 字体表， 管理字体的几何信息和图像信息
/// 依赖环境中的文字测量：
/// ctx.measureText("h")
/// actualBoundingBoxAscent: 22
/// actualBoundingBoxDescent: 0
/// actualBoundingBoxLeft: -1
/// actualBoundingBoxRight: 15
/// fontBoundingBoxAscent: 27
/// fontBoundingBoxDescent: 6
/// width: 16.6845703125
use std::{collections::hash_map::Entry, default::Default, str::Chars};

// use pi_data_view::GetView;
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey, Key};
use pi_data_view::GetView;
use pi_ucd::Codepoint;
use pi_hash::XHashMap;

// use crate::render::res::TextureRes;

// use crate::components::user::*;
use crate::{components::user::{CgColor, Vector2, FontSize, LineHeight, Aabb2, Point2}};

use super::font_tex::*;


// 默认字体尺寸
pub const FONT_SIZE: f32 = 32.0;

/// 默认纹理宽度为2048，永远向下扩展
pub const TEX_WIDTH: f32 = 2048.0;
pub const INIT_TEX_HEIGHT: u32 = 256;

// // 小字体的大小， 小于该字体，默认勾1个px的边
// const SMALL_FONT: usize = 20;

// 粗体字的font-weight
const BLOD_WEIGHT: usize = 700;

// 粗体字的放大因子
const BLOD_FACTOR: f32 = 1.13;

// TODO 将字体样式和字符做为键，查hashmap，获得slab id，slab中放字符和字形信息， 其他地方仅使用id
// TODO pixel font 和 xysdf font, 都使用一个文字纹理。 也是预处理的纹理。
// 相同的font_farmly在不同的字符上也可能使用不同的font， text_layout需要根据pixel和sdf分成char_blocks 的2个arr
// 测量字符宽度时， 计算出Glyth，并创建font_char_id. 将未绘制的放入全局wait_draw_list, 统一绘制

/// 字体表 使用SDF(signed distance field 有向距离场)渲染字体， 支持预定义字体纹理及配置， 也支持动态计算字符的SDF
pub struct FontSheet<T: FontTexture> {
    size: f32,
    color: CgColor,
    pub src_map: XHashMap<usize, TexFont<T>>,
    face_map: XHashMap<usize, FontFace>,
    char_w_map: XHashMap<
        (usize/*font-family */, char, bool /*是否为加粗字体*/),
        (
            f32,
            /* char width */ usize,
            /* font */ f32,
			/* factor_t */ f32,
			/* factor_b */
			 bool,
        ),
    >,
    pub char_map: XHashMap<
        (
            usize, /*font-family */
            usize,
            /* font_size */ usize,
            /* stroke_width */ usize,
            /* weight */ char,
        ),
        DefaultKey, /* slab id */
    >, // key (font, stroke_width, char) // 永不回收
    pub char_slab: SlotMap<DefaultKey, (char, Glyph)>, // 永不回收 (char, Glyph, font_size, stroke_width) // 永不回收
    pub wait_draw_list: Vec<TextInfo>,
    pub wait_draw_map: XHashMap<
        (
            usize, /*font-family */
            usize, /*font_size*/
            usize, /*stroke_width*/
            usize, /*font_weight */
        ),
        (usize /* TextInfo_Index */, f32 /* v */),
    >,
    measure_char: Box<dyn Fn(usize/*font-family */ ,usize, char) -> f32 + Send + Sync + 'static>,
	pub font_tex: FontTex<T>,
	pub tex_version: usize,

	// pub msdf_font_texs: Vec<Share<TextureRes>>,
}

impl<T: FontTexture> FontSheet<T> {
    pub fn new(
        texture: Share<T>,
        measure: Box<dyn Fn(usize, usize, char) -> f32 + Send + Sync + 'static>,
    ) -> Self {
        FontSheet {
            size: FONT_SIZE,
            color: CgColor::default(),
            src_map: XHashMap::default(),
            face_map: XHashMap::default(),
            char_w_map: XHashMap::default(),
            char_map: XHashMap::default(),
            char_slab: SlotMap::default(),
            wait_draw_list: Vec::new(),
            wait_draw_map: XHashMap::default(),
            measure_char: measure,
			font_tex: FontTex::new(texture),
			// msdf_font_texs: Vec::default(),
			tex_version: 0,
        }
    }

	// 清空字形
	pub fn clear_gylph(&mut self) {
		self.wait_draw_list.clear();
		self.wait_draw_map.clear();
		self.char_map.clear();
		self.char_slab.clear();
		self.font_tex.clear();
	}
	
    // pub fn mem_size(&self) -> usize {
    //     self.src_map.capacity() * (std::mem::size_of::<usize>() + std::mem::size_of::<TexFont>())
    //         + self.face_map.capacity()
    //             * (std::mem::size_of::<usize>() + std::mem::size_of::<(FontFace)>())
    //         + self.char_w_map.capacity()
    //             * (std::mem::size_of::<(usize, char, bool)>()
    //                 + std::mem::size_of::<(f32, usize, f32, bool)>())
    //         + self.char_map.capacity()
    //             * (std::mem::size_of::<(usize, usize, usize, char)>() + std::mem::size_of::<usize>())
    //         + self.char_slab.mem_size()
    //         + self.wait_draw_list.capacity() * std::mem::size_of::<TextInfo>()
    //         + self.wait_draw_map.capacity()
    //             * (std::mem::size_of::<(usize, usize, usize, usize)>()
    //                 + std::mem::size_of::<(usize, f32)>())
    // }
    // 设置默认字号
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
    // 设置默认字色
    pub fn set_color(&mut self, color: CgColor) {
        self.color = color;
    }
    // 设置Font
    pub fn set_src(&mut self, name: usize, is_pixel: bool, factor_t: f32, factor_b: f32, ascender: f32, descender: f32) {
        match self.src_map.entry(name){
			Entry::Occupied(mut e) => {
				let r = e.get_mut();
				r.factor_t = factor_t;
				r.factor_b = factor_b;
				r.is_pixel = is_pixel;
				r.ascender = ascender;
				r.descender = descender;
			},
			Entry::Vacant(r) => { 
				r.insert(TexFont {
					name,
					factor_t,
					factor_b,
					is_pixel,
					ascender,
					descender,
					textures: if is_pixel {
						None
					} else {
						Some(Vec::new())
					},
					font_size: FONT_SIZE,
				});
			}
		}
    }

    pub fn get_src(&self, name: &usize) -> Option<&TexFont<T>> {
        self.src_map.get(name)
    }

    // 取字体详情
    pub fn get_font_info(&self, font_face: &usize) -> Option<(&TexFont<T>, usize /* font_size */)> {
        match self.face_map.get(font_face) {
            Some(face) => {
                for name in &face.src {
                    match self.src_map.get(name) {
                        Some(font) => return Some((font, face.size)),
                        _ => (),
                    }
                }
            }
            _ => (),
        };
        None
    }

    // 设置FontFace
    pub fn set_face(
        &mut self,
        family: usize,
        _oblique: f32,
        size: usize,
        _weight: usize,
        src: Vec<usize>,
    ) {
        let face = FontFace {
            // oblique: oblique,
            size: size,
            // weight: weight,
            src: src,
        };
        self.face_map.entry(family.clone()).or_insert(face);
    }

    // 取字体信息
    pub fn get_font(&self, font_face: &usize) -> Option<&TexFont<T>> {
        match self.face_map.get(font_face) {
            Some(face) => {
                for name in &face.src {
                    if let Some(font) = self.src_map.get(name) {
                        return Some(font);
                    }
                }
                None
            }
            None => None,
        }
    }
    // TODO 改成返回一个查询器， 这样在多个字符查询时，少很多hash查找
    // 测量指定字符的宽高，返回字符宽高(不考虑scale因素)，字符的slab_id，是否为pixel字体。 不同字符可能使用不同字体。 测量时，计算出Glyth, 并创建font_char_id, 将未绘制的放入wait_draw_list。
    pub fn measure(
        &mut self,
        font: &TexFont<T>,
        font_size: usize,
        sw: usize,
        weight: usize,
        c: char,
    ) -> (f32 /*width,height*/, f32 /*base_width*/) {
		if font.is_pixel {
			let is_blod = c.is_ascii() && weight >= BLOD_WEIGHT;
			match self.char_w_map.entry((font.name, c, is_blod)) {
				Entry::Occupied(e) => {
					let r = e.get();
					return (
						r.0 * font_size as f32 / FONT_SIZE + sw as f32,
						r.0,
					);
				}
				Entry::Vacant(r) => {
					let mut w = self.measure_char.as_ref()(font.name, FONT_SIZE as usize, c);
					if w > 0.0 {
						if is_blod {
							w = w * BLOD_FACTOR;
						}
						r.insert((w, font.name, font.factor_t, font.factor_b, font.is_pixel));
						return (w * font_size as f32 / FONT_SIZE + sw as f32, w);
					}
				}
			}
			
		} else {
			match self.char_map.get(&(font.name, 0, 0, 0, c)) {
                Some(id) => {
					match self.char_slab.get(*id) {
						Some(r) => return (r.1.advance * (font_size as f32/font.font_size), r.1.advance),
						None => (),
					}
				},
                _ => (),
            }
		}
		(0.0, 0.0)
    }

    // 添加一个字形信息,
    pub fn calc_gylph(
        &mut self,
        font: &TexFont<T>,
        font_size: usize,
        stroke_width: usize,
        weight: usize,
        scale: f32,
        base_width: f32,
        c: char,
    ) -> DefaultKey {
        if font.is_pixel {
            // 像素纹理
            //let fs = font_size as f32 * font.factor;
            let fs_scale_f = font_size as f32 * scale;
            let fs_scale = fs_scale_f.floor() as usize;
            // 为了泛用，渲染的字符总是会有边框， 要么是默认的，要么是参数指定的
            let sw = if stroke_width != 0 {
                let r = (stroke_width as f32 * scale).round() as usize; // 勾边也要用缩放后
                if r == 0 {
                    1
                } else {
                    r
                } // 保证最少1个像素
                  // }else if fs_scale < SMALL_FONT {
                  //     1
                  // }else{
                  //     2
            } else {
                0
            };
            // 根据缩放后的字体及勾边大小来查找Glyth, 返回的w需要除以scale
            let id = match self
                .char_map
                .entry((font.name, fs_scale, sw, weight, c))
            {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(r) => {
                    // 在指定字体及字号下，查找该字符的宽度
                    let w = (base_width as f32 * font_size as f32 / FONT_SIZE + stroke_width as f32) * scale;
                    // 将缩放后的实际字号乘字体的修正系数，得到实际能容纳下的行高
					let height = (font_size as f32 * (font.factor_t + font.factor_b + 1.0) + stroke_width as f32) * scale;
					
					let ww = w.ceil();
					let hh = height.ceil();
						
                    let mut line = self.font_tex.alloc_line(hh as usize);
                    let p = line.alloc(ww);

					// 超出最大纹理范围，需要清空所有文字，重新布局
					if *(line.last_v) > line.tex_width {
						return DefaultKey::null(); // 0表示异常情况，不能计算字形
					}

                    let id = self.char_slab.insert((
                        c,
                        Glyph {
                            x: p.x,
                            y: p.y,
                            ox: 0.0,
                            oy: 0.0,
                            width: w,
                            height: height as f32,
                            advance: w,
                        },
                    ));
                    // 将需要渲染的字符放入等待队列
                    match self
                        .wait_draw_map
                        .entry((font.name, fs_scale, sw, weight))
                    {
                        Entry::Occupied(mut e) => {
                            let mut r = *e.get_mut();
                            if r.1 == p.y {
                                let info = &mut self.wait_draw_list[r.0];
                                info.chars.push(WaitChar {
                                    ch: c,
                                    width: ww,
                                    x: p.x as u32,
                                    y: p.y as u32,
                                });
                                info.size.x += ww;
                            } else {
                                r.0 = self.wait_draw_list.len();
                                r.1 = p.y;
                                self.wait_draw_list.push(TextInfo {
                                    font: font.name,
                                    font_size: fs_scale,
                                    stroke_width: sw,
									weight: weight,
									top: (fs_scale as f32 * font.factor_t) as usize,
                                    size: Vector2::new(ww, hh as f32),
                                    chars: vec![WaitChar {
                                        ch: c,
                                        width: ww,
                                        x: p.x as u32,
                                        y: p.y as u32,
                                    }],
                                });
                            }
                        }
                        Entry::Vacant(r) => {
                            r.insert((self.wait_draw_list.len(), p.y));
                            self.wait_draw_list.push(TextInfo {
                                font: font.name,
								font_size: fs_scale,
								top: (fs_scale as f32 * font.factor_t) as usize,
                                stroke_width: sw,
                                weight: weight,
                                size: Vector2::new(ww, hh),
                                chars: vec![WaitChar {
                                    ch: c,
                                    width: ww,
                                    x: p.x as u32,
                                    y: p.y as u32,
                                }],
                            });
                        }
                    }
                    r.insert(id);
                    id
                }
            };
            return id;
        } else {
            // SDF 字体， 根据字形Glyph计算宽度
            match self.char_map.get(&(font.name, 0, 0, 0, c)) {
                Some(id) => return *id,
                _ => (),
            }
        }
        DefaultKey::null()
    }

    pub fn get_font_tex(&self) -> &Share<T> {
        &self.font_tex.texture
    }

    pub fn get_glyph(&self, id: DefaultKey) -> Option<&(char, Glyph)> {
        self.char_slab.get(id)
    }

    // msdf 需要修正字形信息
    pub fn fix_gylph(gylph: &Glyph, font_size: f32) -> (f32, f32) {
        let radio = font_size / FONT_SIZE;
        (gylph.width * radio, gylph.width * radio)
    }
}

// 字体表现
#[derive(Default, Debug)]
pub struct FontFace {
    // oblique: f32,
    size: usize,
    // weight: usize,
    src: Vec<usize>,
}

pub fn get_size(size: usize, s: &FontSize) -> usize {
    match s {
        &FontSize::None => size,
        &FontSize::Length(r) => r.round() as usize,
        &FontSize::Percent(r) => (r * size as f32).round() as usize,
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
// // 倾斜度造成的间距
// pub fn oblique_spacing(oblique: f32, font_size: f32, char_width: f32) -> f32 {
//     oblique * font_size * char_width // TODO FIX!!!
// }
pub struct TexFont<T: FontTexture> {
    pub name: usize,
	pub is_pixel: bool,
	pub factor_t: f32,    // 像素纹理字体大小有时超出，需要一个字体的修正系数
	pub factor_b: f32,    // 像素纹理字体大小有时超出，需要一个字体的修正系数
	pub textures: Option<Vec<Share<T>>>, // sdf字体才存在

	// ascender - descender = lineheight
	pub ascender: f32,  // 为正数
	pub descender: f32, // 通常为一个负数
	pub font_size: f32, // 字体大小（sdf才会有，表示sdf中的纹理是该尺寸的文字）
}

impl<T: FontTexture> Clone for TexFont<T> {
	fn clone(&self) -> Self {
		Self {
			name: self.name,
			is_pixel: self.is_pixel,
			factor_t: self.factor_t,
			factor_b: self.factor_b,
			textures: match &self.textures {
				Some(r) => {
					let mut v = Vec::with_capacity(r.len());
					for l in r.iter() {
						v.push(l.clone())
					}
					Some(v)
				},
				None => None,
			},
			ascender: self.ascender,
			descender: self.descender,
			font_size: self.font_size,
		}
	}
}

impl<T: FontTexture> TexFont<T> {
    #[inline]
    //  获得字体大小, 0表示没找到该font_face
    pub fn get_font_height(&self, size: usize, stroke_width: f32) -> f32 {
		if self.is_pixel {
			size as f32 + (size as f32 * self.factor_t + size as f32 * self.factor_b).round() + stroke_width
		} else {
			size as f32 * (self.ascender - self.descender)
		}
    }
}

#[derive(Debug, Default, Clone)]
pub struct Glyph {
    pub x: f32,
    pub y: f32,
    pub ox: f32, //文字可见区域左上角相对于文字外边框的左上角在水平轴上的距离
    pub oy: f32, //文字可见区域左上角相对于文字外边框的左上角在垂直轴上的距离
    pub width: f32,
    pub height: f32,
    pub advance: f32,
}

impl Glyph {
    pub fn get_uv(&self, tex_size: &Vector2) -> Aabb2 {
        Aabb2::new(
            Point2::new(self.x / tex_size.x, self.y / tex_size.y),
            Point2::new(
                (self.x + self.width) / tex_size.x,
                (self.y + self.height) / tex_size.y,
            ),
        )
    }

    pub fn parse(value: &[u8], offset: &mut usize) -> Self {
        let x = value.get_lu16(*offset);
        *offset += 2;
        let y = value.get_lu16(*offset);
        *offset += 2;
        let ox = value.get_li8(*offset);
        *offset += 1;
        let oy = value.get_u8(*offset);
        *offset += 1;
        let width = value.get_u8(*offset);
        *offset += 1;
        let height = value.get_u8(*offset);
        *offset += 1;
        let advance = value.get_u8(*offset);
        *offset += 1;

		*offset += 1; // 加1， 对齐

        Glyph {
            x: x as f32,
            y: y as f32,
            ox: ox as f32,
            oy: oy as f32,
            width: width as f32,
            height: height as f32,
            advance: advance as f32,
        }
    }
}

#[derive(Debug)]
pub struct TextInfo {
    pub font: usize, /*font-famliy */
    pub font_size: usize,
    pub stroke_width: usize,
    pub weight: usize,
    pub size: Vector2,
	pub chars: Vec<WaitChar>,
	pub top: usize,
}

#[derive(Debug)]
pub struct WaitChar {
    pub ch: char,
    pub width: f32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
// 劈分结果
pub enum SplitResult {
    Newline(isize),
    Whitespace(isize),
    Word(isize,char),      // 单字词
    WordStart(isize,char), // 单词开始, 连续的字母或数字(必须字符的type_id相同)组成的单词
    WordNext(isize,char),  // 单词字符继续
    WordEnd(isize),         // 单词字符结束
}

// 劈分字符迭代器
pub struct SplitChar<'a> {
	cur_index: usize,
    iter: Chars<'a>,
    word_split: bool,
    merge_whitespace: bool,
    last: Option<char>,
    type_id: usize, // 0表示单字词, 1表示ascii字母 2及以上代表字符的type_id, MAX表示数字
}

impl<'a> Iterator for SplitChar<'a> {
    type Item = SplitResult;
    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            Some(c) if self.type_id == 0 => {
                if c == '\n' {
                    self.last = self.iter.next();
					self.cur_index += 1;
                    Some(SplitResult::Newline((self.cur_index - 1) as isize))
                } else if c.is_whitespace() {
                    if self.merge_whitespace {
                        loop {
							self.cur_index += 1;
                            match self.iter.next() {
                                Some(cc) if cc.is_whitespace() => continue,
                                r => {
                                    self.last = r;
                                    break;
                                }
                            }
                        }
                    } else {
                        self.last = self.iter.next();
						self.cur_index += 1;
                    }
                    Some(SplitResult::Whitespace((self.cur_index - 1) as isize))
                } else if !self.word_split {
                    self.last = self.iter.next();
					self.cur_index += 1;
                    Some(SplitResult::Word((self.cur_index - 1) as isize,c))
                } else {
                    self.type_id = get_type_id(c, char::from(0));
                    if self.type_id == 0 {
                        self.last = self.iter.next();
						self.cur_index += 1;
                        Some(SplitResult::Word((self.cur_index - 1) as isize,c))
                    } else {
                        // 如果是单词开始，不读取下个字符，因为需要保留当前字符做是否为单词的判断
                        Some(SplitResult::WordStart(self.cur_index as isize,c))
                    }
                }
            }
            Some(old_c) => {
                self.last = self.iter.next();
				self.cur_index += 1;
                match self.last {
                    Some(c) => {
                        let id = get_type_id(c, old_c);
                        if id == self.type_id {
                            Some(SplitResult::WordNext(self.cur_index as isize,c))
                        } else {
                            self.type_id = 0;
                            Some(SplitResult::WordEnd(-1))
                        }
                    }
                    _ => Some(SplitResult::WordEnd(-1)),
                }
            }
            _ => None,
        }
    }
}

/// 数字或字母, 返回对应的类型
fn get_type_id(c: char, prev: char) -> usize {
    if c.is_ascii() {
        if c.is_ascii_alphabetic() {
            return 1;
        } else if c.is_ascii_digit() {
            return usize::max_value();
        } else if c == '/' || c == '.' || c == '%' {
            if prev.is_ascii_digit() {
                return usize::max_value();
            }
        } else if c == '\'' {
            if prev.is_ascii_alphabetic() {
                return 1;
            }
        }
    } else if c.is_alphabetic() && !c.is_cased() {
        return c.get_type_id();
    }
    0
}
/// 劈分字符串, 返回字符迭代器
pub fn split<'a>(s: &'a str, word_split: bool, merge_whitespace: bool) -> SplitChar<'a> {
    let mut i = s.chars();
    let last = i.next();
    SplitChar {
		cur_index: 0,
        iter: i,
        word_split: word_split,
        merge_whitespace: merge_whitespace,
        last: last,
        type_id: 0,
    }
}

#[test]
fn test() {
	let mut ret = Vec::with_capacity(300);

	let s = "关于在线性复杂度内,判断线段是否在多边形内的做,已经描述得挺清楚了，不过因为这个题允许线段,和多边形的边重合，所以实际上还有不少细节需要讨论，这里就不细说了".to_string();

	let time = std::time::Instant::now();
	for _i in 0..20 {
		for cr in split(s.as_str(), true, true) {
			match cr {
				// 存在WordStart， 表示开始一个多字符单词
				SplitResult::Word(_index,_c) => {
					ret.push(_c);
				},
				_ => (),
			}
		}
	}
	println!("time==========={:?}", std::time::Instant::now() - time);
	println!("ret==========={:?}, {}", ret, ret.len());
}
