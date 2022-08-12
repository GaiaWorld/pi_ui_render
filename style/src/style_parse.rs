//! 解析字符串格式的样式

use std::intrinsics::transmute;
use std::str::FromStr;

use bitvec::prelude::BitArray;
use cssparser::{Parser, BasicParseError, ParseError, Token, Delimiter, CowRcStr, ParserInput};
use nalgebra::{RealField, Point2};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_flex_layout::{style::{Display, Dimension, AlignItems, AlignSelf, AlignContent, FlexDirection, JustifyContent, PositionType, FlexWrap}, prelude::Rect};
use smallvec::SmallVec;

use crate::style::{BackgroundColor, Color, BorderColor, ObjectFit, BorderImageRepeat, MaskImage, BorderRadius, Opacity, Hsi, Blur, Enable, WhiteSpace, LinearGradientColor, CgColor, ColorAndPosition, BorderImageSlice, BlendMode, LineHeight, TextAlign, FontSize, TextShadow, BoxShadow, Stroke, TransformFunc, TransformOrigin, LengthUnit, FitType, BorderImageRepeatOption, BackgroundImage, BorderImage, MaskImageClip, BackgroundImageClip, BorderImageClip, Margin, Padding, Overflow, ZIndex, TextShadows, Border};

use super::style_type::*;

pub fn parse_class_map_from_string(value: &str, class_sheet: &mut ClassSheet) -> Result<(), String> {
	let mut input = ParserInput::new(value);
	let mut parse = Parser::new(&mut input);

	loop {
		if parse.is_exhausted() {
			return Ok(());
		}

		if let Err(e) = parse_class(class_sheet, &mut parse) {
			log::error!("parse class err: {:?}", e);
		}
	}
}

// 解析class
pub fn parse_class<'i, 't>(context: &mut ClassSheet, input: &mut Parser<'i, 't>) -> Result<(), BasicParseError<'i>> {
	// log::debug!("next==============={:?}", input.next());
	if let Err(r) = input.expect_delim('.') {
		// 不以"."开头，则不是class（目前只解析class，keyframe、id等不解析）， 不是class则跳过
		log::info!("Unexpected css: {:?}", r);
		loop {
			if input.is_exhausted() {
				return Ok(());
			}
			if let Ok(_) = input.expect_curly_bracket_block() {
				return Ok(());
			}
		}
	}

	let class_name = input.expect_ident()?.as_ref();
	log::info!("class: {}", class_name);

	let class_name = match usize::from_str(&class_name[1..class_name.len()]) {
		Ok(r) => r,
		Err(_) => usize::MAX,
	};

	input.expect_curly_bracket_block()?;

	let mut class_meta = ClassMeta {
		start: context.style_buffer.len(),
		end: 0,
		class_style_mark: BitArray::default(),
	};

	match input.parse_nested_block::<_, _, ValueParseErrorKind>(|i| {
		loop {
			if let Err(e) = parse_style_item(&mut context.style_buffer, &mut class_meta, i) {
				if i.is_exhausted() {
					break;
				} else {
					log::error!("parse_style error: {:?}", e);
				}
			}
		}
		Ok(())
	}) {
		Ok(r) => r,
		Err(r) => {
			log::error!("parse_class fail, {:?}", r);
		},
	};

	if class_name != usize::MAX {
		class_meta.end = context.style_buffer.len();
	
		context.class_map.insert(class_name, class_meta);
	}
	Ok(())
}

fn parse_border_image_slice<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BorderImageSlice, ParseError<'i, ValueParseErrorKind>> {
	let r = match input.try_parse(|input| {input.expect_percentage()}) {
		Ok(r1) => match input.try_parse(|input| {input.expect_percentage()}) {
			Ok(r2) => match input.try_parse(|input| {input.expect_percentage()}) {
				Ok(r3) => match input.try_parse(|input| {input.expect_percentage()}) {
					Ok(r4) => [r1, r2, r3, r4],
					Err(_) => [r1, r2, r3, r2],
				},
				Err(_) => [r1, r2, r1, r2],
			},
			Err(_) => [r1, r1, r1, r1],
		},
		Err(_) => [0.0, 0.0, 0.0, 0.0],
	};

	let fill = match input.try_parse(|input| {input.expect_ident_matching("fill")}) {
		Ok(_) => true,
		Err(_) => false,
	};

	Ok(BorderImageSlice {
		top: match NotNan::new(r[0]) {
			Ok(r) => r,
			Err(_) => unsafe { NotNan::new_unchecked(0.0) },
		},
		right:  match NotNan::new(r[1]) {
			Ok(r) => r,
			Err(_) => unsafe { NotNan::new_unchecked(0.0) },
		},
		bottom:  match NotNan::new(r[2]) {
			Ok(r) => r,
			Err(_) => unsafe { NotNan::new_unchecked(0.0) },
		},
		left:  match NotNan::new(r[3]) {
			Ok(r) => r,
			Err(_) => unsafe { NotNan::new_unchecked(0.0) },
		},
		fill
	})
}

fn parse_top_right_bottom_left<'i, 't, T: StyleParse + Copy + Default>(input: &mut Parser<'i, 't>) -> Result<Rect<T>, ParseError<'i, ValueParseErrorKind>> {
	let r = match input.try_parse(|input| {T::parse(input)}) {
		Ok(r1) => match input.try_parse(|input| {T::parse(input)}) {
			Ok(r2) => match input.try_parse(|input| {T::parse(input)}) {
				Ok(r3) => match input.try_parse(|input| {T::parse(input)}) {
					Ok(r4) => Rect {top: r1, right: r2, bottom: r3, left: r4},
					Err(_) => Rect {top: r1, right: r2, bottom: r3, left: r2},
				},
				Err(_) => Rect {top: r1, right: r2, bottom: r1, left: r2},
			},
			Err(_) => Rect {top: r1, right: r1, bottom: r1, left: r1},
		},
		Err(_) => Rect {top: T::default(), right: T::default(), bottom: T::default(), left: T::default()},
	};
	Ok(r)
}

fn parse_enable<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Enable, ParseError<'i, ValueParseErrorKind>> {
    match input.expect_ident()?.as_ref() {
        "auto" => Ok(Enable::Auto),
        "none" => Ok(Enable::None),
        "visible" => Ok(Enable::Visible),
        _ => Ok(Enable::Auto),
    }
}

fn parse_visibility<'i, 't>(input: &mut Parser<'i, 't>) -> Result<bool, ParseError<'i, ValueParseErrorKind>> {
	match input.expect_ident()?.as_ref() {
        "hidden" => Ok(false),
        _ => Ok(true),
    }
}

fn parse_display<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Display, ParseError<'i, ValueParseErrorKind>> {
	match input.expect_ident()?.as_ref() {
        "flex" => Ok(Display::Flex),
        "none" => Ok(Display::None),
        _ => Ok(Display::Flex), // 默认情况
    }
}

fn parse_overflow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<bool, ParseError<'i, ValueParseErrorKind>> {
	match input.expect_ident()?.as_ref() {
        "hidden" => Ok(true),
        _ => Ok(false), // 默认情况
    }
}

fn pasre_white_space<'i, 't>(input: &mut Parser<'i, 't>) -> Result<WhiteSpace, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let r = match input.expect_ident()?.as_ref() {
        "normal" => WhiteSpace::Normal,
        "pre" => WhiteSpace::Pre,
        "nowrap" => WhiteSpace::Nowrap,
        "pre-wrap" => WhiteSpace::PreWrap,
        "pre-line" => WhiteSpace::PreLine,
        _ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    };
    Ok(r)
}

fn parse_blend_mode<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BlendMode, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let r = match input.expect_ident()?.as_ref() {
		"normal" => BlendMode::Normal,
		"alpha-add" => BlendMode::AlphaAdd,
		"subtract" => BlendMode::Subtract,
		"multiply" => BlendMode::Multiply,
		"one-one" => BlendMode::OneOne,
        _ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
	};
	Ok(r)
}

fn parse_font_weight<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let toke = input.next()?;
	let r = match toke {
		Token::Ident(r) => {
			match r.as_ref() {
				"bold" => 700.0,
				_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
			}
		},
		Token::Number{value,..} => *value,
        _ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    };
	Ok(r)
}

fn parse_text_align<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TextAlign, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let r = match input.expect_ident()?.as_ref() {
        "left" => Ok(TextAlign::Left),
        "right" => Ok(TextAlign::Right),
        "center" => Ok(TextAlign::Center),
        "justify" => Ok(TextAlign::Justify),
        _ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    };
	r
}

fn parse_yg_align_items<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignItems, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignItems::FlexStart),
        "center" => Ok(AlignItems::Center),
        "flex-end" => Ok(AlignItems::FlexEnd),
        "stretch" => Ok(AlignItems::Stretch),
        "baseline" => Ok(AlignItems::Baseline),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_align_self<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignSelf, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignSelf::FlexStart),
        "center" => Ok(AlignSelf::Center),
        "flex-end" => Ok(AlignSelf::FlexEnd),
        "stretch" => Ok(AlignSelf::Stretch),
        "baseline" => Ok(AlignSelf::Baseline),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_align_content<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignContent, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignContent::FlexStart),
        "center" => Ok(AlignContent::Center),
        "flex-end" => Ok(AlignContent::FlexEnd),
        "stretch" => Ok(AlignContent::Stretch),
        "space-between" => Ok(AlignContent::SpaceBetween),
        "space-around" => Ok(AlignContent::SpaceAround),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_direction<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FlexDirection, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
        "column" => Ok(FlexDirection::Column),
        "column-reverse" => Ok(FlexDirection::ColumnReverse),
        "row" => Ok(FlexDirection::Row),
        "row-reverse" => Ok(FlexDirection::RowReverse),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_justify_content<'i, 't>(input: &mut Parser<'i, 't>) -> Result<JustifyContent, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
        "flex-start" => Ok(JustifyContent::FlexStart),
        "center" => Ok(JustifyContent::Center),
        "flex-end" => Ok(JustifyContent::FlexEnd),
        "space-between" => Ok(JustifyContent::SpaceBetween),
        "space-around" => Ok(JustifyContent::SpaceAround),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_position_type<'i, 't>(input: &mut Parser<'i, 't>) -> Result<PositionType, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
        "relative" => Ok(PositionType::Relative),
        "absolute" => Ok(PositionType::Absolute),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_yg_wrap<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FlexWrap, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	match input.expect_ident()?.as_ref() {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_line_height<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LineHeight, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let toke = input.next()?;
	match toke {
		Token::Ident(r) => {
			match r.as_ref() {
				"normal" => Ok(LineHeight::Normal),
				_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
			}
		},
		Token::Percentage{unit_value,..} => Ok(LineHeight::Percent(*unit_value / 100.0)),
		Token::Dimension{value,..} => Ok(LineHeight::Length(*value)),
		Token::Number{value,..} => Ok(LineHeight::Length(*value)),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_font_size<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FontSize, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let toke = input.next()?;
	match toke {
		Token::Percentage{unit_value,..} => Ok(FontSize::Percent(*unit_value / 100.0)),
		Token::Dimension{value,..} => Ok(FontSize::Length(*value as usize)),
		Token::Number{value,..} => Ok(FontSize::Length(*value as usize)),
		_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
	}
}

fn parse_text_stroke<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Stroke, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	Ok(Stroke {
		width: match NotNan::new(parse_len(input)?) {
			Ok(r) => r,
			Err(_) => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
		},
		color: parse_color(input)?
	})
}

fn parse_transform_origin<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TransformOrigin, ParseError<'i, ValueParseErrorKind>> {
	let x =  parse_transform_origin1(input)?;
    Ok(TransformOrigin::XY(
        x,
		match input.try_parse(parse_transform_origin1) {
			Ok(r) => r,
			Err(_) => x
		}
    ))
}

fn parse_transform_origin1<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LengthUnit, ParseError<'i, ValueParseErrorKind>> {
    let location = input.current_source_location();
	let toke = input.next()?;
	match toke {
		Token::Ident(r) => {
			match r.as_ref() {
				"center" => Ok(LengthUnit::Percent(0.5)),
				_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
			}
		},
		Token::Percentage{unit_value,..} => Ok(LengthUnit::Percent(*unit_value / 100.0)),
		Token::Dimension{value,..} => Ok(LengthUnit::Pixel(*value)),
		Token::Number{value,..} => Ok(LengthUnit::Pixel(*value)),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

pub fn parse_len_or_percent<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LengthUnit, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let toke = input.next()?;
	match toke {
		Token::Percentage{unit_value,..} => Ok(LengthUnit::Percent(*unit_value / 100.0)),
		Token::Dimension{value,..} => Ok(LengthUnit::Pixel(*value)),
		Token::Number{value,..} => Ok(LengthUnit::Pixel(*value)),
        _ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
    }
}

fn parse_transform<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Vec<TransformFunc>, ParseError<'i, ValueParseErrorKind>> {
    let mut transforms = Vec::default();
	let location = input.current_source_location();
    loop {
		if let Ok(r) = input.try_parse(|input| {
			let location = input.current_source_location();
			let f = input.expect_function()?;
			match f.as_ref() {
				"scale" => input.parse_nested_block(|input| {
					let x = input.expect_number()?;
					let y = match input.expect_comma() {
						Ok(_) => input.expect_number().unwrap_or(x),
						Err(_) => x
					};
					Ok(TransformFunc::Scale(x, y))
				}),
				"scaleX" => input.parse_nested_block(|input| {
					Ok(TransformFunc::ScaleX(input.expect_number()?))
				}),
				"scaleY" => input.parse_nested_block(|input| {
					Ok(TransformFunc::ScaleY(input.expect_number()?))
				}),
				"translate" => input.parse_nested_block(|input| {
					let location = input.current_source_location();
					let x = parse_len_or_percent(input)?;
					input.expect_comma()?;
					let y = parse_len_or_percent(input)?;
					match (x, y) {
						(LengthUnit::Percent(x), LengthUnit::Percent(y)) => Ok(TransformFunc::TranslatePercent(x, y)),
						(LengthUnit::Pixel(x), LengthUnit::Pixel(y)) => Ok(TransformFunc::Translate(x, y)),
						_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter))
					}
				}),
				"translateX" => input.parse_nested_block(|input| {
					match parse_len_or_percent(input)? {
						LengthUnit::Percent(v) => Ok(TransformFunc::TranslateXPercent(v)),
						LengthUnit::Pixel(v) => Ok(TransformFunc::TranslateX(v)),
					}
				}),
				"translateY" => input.parse_nested_block(|input| {
					match parse_len_or_percent(input)? {
						LengthUnit::Percent(v) => Ok(TransformFunc::TranslateYPercent(v)),
						LengthUnit::Pixel(v) => Ok(TransformFunc::TranslateY(v)),
					}
				}),
				"rotate" | "rotateZ" => input.parse_nested_block(|input| {
					Ok(TransformFunc::RotateZ(parse_angle(input)?))
				}),
				"skewX" => input.parse_nested_block(|input| {
					Ok(TransformFunc::SkewX(parse_angle(input)?))
				}),
				"skewY" => input.parse_nested_block(|input| {
					Ok(TransformFunc::SkewY(parse_angle(input)?))
				}),
				_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
			}
		}) {
			transforms.push(r);
		} else {
			break;
		}
    }
	if transforms.len() > 0 {
		Ok(transforms)
	} else {
		Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter))
	}
}

fn parse_object_fit<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FitType, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let r = match input.expect_ident()?.as_ref() {
		"contain" => FitType::Contain,
        "cover" => FitType::Cover,
        "fill" => FitType::Fill,
        "none" => FitType::None,
        "scale-down" => FitType::ScaleDown,
		_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidObjectFit)),
	};
    Ok(r)
}

fn parse_border_image_repeat<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BorderImageRepeatOption, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
    let r = match input.expect_ident()?.as_ref() {
        "stretch" => BorderImageRepeatOption::Stretch,
        "repeat" => BorderImageRepeatOption::Repeat,
        "round" => BorderImageRepeatOption::Round,
        "space" => BorderImageRepeatOption::Space,
        _ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidRepeat)),
    };
    Ok(r)
}

fn parse_color_hex(value: &str) -> Result<CgColor, String> {
    let value = value.as_bytes();
    match value.len() {
        8 => Ok(rgba(
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            (from_hex(value[6])? * 16 + from_hex(value[7])?) as f32 / 255.0,
        )),
        6 => Ok(rgba(
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            1.0,
        )),
        4 => Ok(rgba(
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
            (from_hex(value[3])? * 17) as f32 / 255.0,
        )),
        3 => Ok(rgba(
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
            1.0,
        )),
        _ => Err("".to_string()),
    }
}

fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> CgColor {
    CgColor::new(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        alpha,
    )
}

fn parser_color_stop_last(
    v: f32,
    list: &mut Vec<CgColor>,
    color_stop: &mut Vec<ColorAndPosition>,
    pre_percent: &mut f32,
    last_color: Option<CgColor>,
) -> Result<(), String> {
    if list.len() > 0 {
        if color_stop.len() != 0 {
			let pos = (v - *pre_percent) / list.len() as f32;
            for i in 0..list.len() {
                color_stop.push(ColorAndPosition {
                    position: *pre_percent + pos * (i + 1) as f32,
                    rgba: list[i].clone(),
                });
            }
        } else {
			let pos = if list.len() == 1 {
				0.0
			} else {
				(v - *pre_percent) / (list.len() as f32 - 1.0)
			};
            for i in 0..list.len() {
                color_stop.push(ColorAndPosition {
                    position: *pre_percent + pos * i as f32,
                    rgba: list[i].clone(),
                });
            }
        }

        list.clear();
    }
    *pre_percent = v;
    if let Some(last_color) = last_color {
        color_stop.push(ColorAndPosition {
            position: v,
            rgba: last_color,
        });
    }
    Ok(())
}

fn rect_to_aabb<T: RealField>(value: Rect<T>) -> ncollide2d::bounding_volume::AABB<T> {
	ncollide2d::bounding_volume::AABB::new(Point2::new(value.left, value.top), Point2::new(value.right, value.bottom))
}

fn from_hex(c: u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err("".to_string()),
    }
}

fn trans_hsi_h(mut h: f32) -> f32 {
	if h > 180.0 {
        h = 180.0;
    } else if h < -180.0 {
        h = -180.0
    }
    h / 360.0
}

fn trans_hsi_s(mut s: f32) -> f32 {
    if s > 100.0 {
        s = 100.0;
    } else if s < -100.0 {
        s = -100.0
    }
    s / 100.0
}

fn trans_hsi_i( mut i: f32) -> f32 {
    if i > 100.0 {
        i = 100.0;
    } else if i < -100.0 {
        i = -100.0
    }
    i / 100.0
}


pub fn parse_style_item<'i, 't>(buffer: &mut Vec<u8>, class_meta: &mut ClassMeta, input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ValueParseErrorKind>> {
	let key = input.expect_ident()?;
	match key.as_ref() {
		"filter" => {
			input.expect_colon()?;
			parse_filter1(buffer, class_meta, input)?;
        },
        "background-color" => unsafe {
			input.expect_colon()?;
			let ty = BackgroundColorType (BackgroundColor(Color::RGBA(parse_color(input)?)));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "background" => unsafe {
			input.expect_colon()?;
			let ty = BackgroundColorType (BackgroundColor(parse_background(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        "border-color" => unsafe {
			input.expect_colon()?;
			let ty = BorderColorType (BorderColor(parse_color(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "box-shadow" => unsafe {
			input.expect_colon()?;
			let ty = BoxShadowType(parse_box_shadow(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        "background-image" => unsafe {
			input.expect_colon()?;
			match parse_gradient_image(input)? {
				GradientImage::Linear(gradient) => {
					let ty = BackgroundColorType(BackgroundColor(Color::LinearGradient(gradient)));
					log::debug!("{:?}", ty);
					ty.write(buffer);
					class_meta.class_style_mark.set(ty.get_type() as usize, true);
				},
				GradientImage::Url(image) => {
					let ty = BackgroundImageType (BackgroundImage(Atom::from(image.as_ref().to_string())));
					log::debug!("{:?}", ty);
					ty.write(buffer);
					class_meta.class_style_mark.set(ty.get_type() as usize, true);
				},
			}
		}
		"image-clip" | "background-image-clip" => unsafe {
			input.expect_colon()?;
			let ty = BackgroundImageClipType(BackgroundImageClip(
				rect_to_aabb(transmute::<_, Rect<f32>>(parse_top_right_bottom_left::<Percentage>(input)?))
			));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "object-fit" => unsafe {
			input.expect_colon()?;
			let ty = ObjectFitType(ObjectFit(parse_object_fit(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        "border-image" => unsafe {
			input.expect_colon()?;
			let ty = BorderImageType(BorderImage(Atom::from( input.expect_url()?.as_ref().to_string())));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);

        }
        "border-image-clip" => unsafe {
			input.expect_colon()?;
			let ty = BorderImageClipType(BorderImageClip(
				transmute::<_, Rect<NotNan<f32>>>(parse_top_right_bottom_left::<Percentage>(input)?)
			));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-image-slice" => unsafe {
			input.expect_colon()?;
			let ty = BorderImageSliceType(parse_border_image_slice(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-image-repeat" => unsafe {
			input.expect_colon()?;
			let repeat = parse_border_image_repeat(input)?;
			let ty = BorderImageRepeatType(BorderImageRepeat(repeat, repeat));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
		}
		"mask-image" => unsafe {
			input.expect_colon()?;
			match parse_gradient_image(input)? {
				GradientImage::Linear(gradient) => {
					let ty = MaskImageType(MaskImage::LinearGradient(gradient));
					log::debug!("{:?}", ty);
					ty.write(buffer);
					class_meta.class_style_mark.set(ty.get_type() as usize, true);
				},
				GradientImage::Url(image) => {
					let ty = MaskImageType (MaskImage::Path(Atom::from(image.as_ref().to_string())));
					log::debug!("{:?}", ty);
					ty.write(buffer);
					class_meta.class_style_mark.set(ty.get_type() as usize, true);
				},
			}
		}
		"mask-image-clip" => unsafe {
			input.expect_colon()?;
			let ty = MaskImageClipType(MaskImageClip(
				rect_to_aabb(transmute::<_, Rect<f32>>(parse_top_right_bottom_left::<Percentage>(input)?))
			));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
		"blend-mode" => unsafe {
			input.expect_colon()?;
			let ty = BlendModeType(parse_blend_mode(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        },
		"text-gradient" => unsafe {
			input.expect_colon()?;
			let ty = ColorType(parse_background(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
		}
        "color" => unsafe {
			input.expect_colon()?;
			let ty = ColorType(Color::RGBA(parse_color(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "letter-spacing" => unsafe {
			input.expect_colon()?;
			let ty = LetterSpacingType(parse_len(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "line-height" => unsafe {
			input.expect_colon()?;
			let ty = LineHeightType(parse_line_height(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "text-align" => unsafe {
			input.expect_colon()?;
			let ty = TextAlignType(parse_text_align(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "text-indent" => unsafe {
			input.expect_colon()?;
			let ty = TextIndentType(parse_len(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "text-shadow" => unsafe {
			input.expect_colon()?;
			let ty = TextShadowType(parse_text_shadow(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        // "vertical-align" => show_attr.push(Attribute::Color( Color::RGBA(parse_color_string(value)?) )),
        "white-space" => unsafe {
			input.expect_colon()?;
			let ty = WhiteSpaceType(pasre_white_space(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "word-spacing" => unsafe {
			input.expect_colon()?;
			let ty = WordSpacingType(parse_len(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        "text-stroke" => unsafe {
			input.expect_colon()?;
			let ty = TextStrokeType(parse_text_stroke(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        // "font-style" => show_attr.push(Attribute::FontStyle( Color::RGBA(parse_color_string(value)?) )),
        "font-weight" => unsafe {
			input.expect_colon()?;
			let ty = FontWeightType(parse_font_weight(input)? as usize);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "font-size" => unsafe {
			input.expect_colon()?;
			let ty = FontSizeType(parse_font_size(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "font-family" => unsafe {
			input.expect_colon()?;
			let ty = FontFamilyType(Atom::from(input.expect_ident()?.as_ref().to_string()));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }

        "border-radius" => unsafe {
			input.expect_colon()?;
			let value = LengthUnit::Pixel(parse_len(input)?);
			let ty = BorderRadiusType(BorderRadius {
				x: value,
				y: value,
			});
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "opacity" => unsafe {
			input.expect_colon()?;
			let ty = OpacityType(Opacity(input.expect_number()?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "transform" => unsafe {
			input.expect_colon()?;
			let ty = TransformType(parse_transform(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "transform-origin" => unsafe {
			input.expect_colon()?;
			let ty = TransformOriginType(parse_transform_origin(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "z-index" => unsafe {
			input.expect_colon()?;
			let ty = ZIndexType(ZIndex(input.expect_number()? as isize));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "visibility" => unsafe {
			input.expect_colon()?;
			let ty = VisibilityType(parse_visibility(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "pointer-events" => unsafe {
			input.expect_colon()?;
			let ty = EnableType(parse_enable(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "display" => unsafe {
			input.expect_colon()?;
			let ty = DisplayType(parse_display(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "overflow" => unsafe {
			input.expect_colon()?;
			let ty = OverflowType(Overflow(parse_overflow(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "overflow-y" => unsafe {
			input.expect_colon()?;
			let ty = OverflowType(Overflow(parse_overflow(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "width" => unsafe {
			input.expect_colon()?;
			let ty = WidthType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
		},
		"height" => unsafe {
			input.expect_colon()?;
			let ty = HeightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
		},
        "left" => unsafe {
			input.expect_colon()?;
			let ty = PositionLeftType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "bottom" => unsafe {
			input.expect_colon()?;
			let ty = PositionBottomType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "right" => unsafe {
			input.expect_colon()?;
			let ty = PositionRightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "top" => unsafe {
			input.expect_colon()?;
			let ty = PositionTopType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "margin-left" => unsafe {
			input.expect_colon()?;
			let ty = MarginLeftType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "margin-bottom" => unsafe {
			input.expect_colon()?;
			let ty = MarginBottomType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "margin-right" => unsafe {
			input.expect_colon()?;
			let ty = MarginRightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "margin-top" => unsafe {
			input.expect_colon()?;
			let ty = MarginTopType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "margin" => unsafe {
			input.expect_colon()?;
			let ty = MarginType(Margin(parse_top_right_bottom_left(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "padding-left" => unsafe {
			input.expect_colon()?;
			let ty = PaddingLeftType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "padding-bottom" => unsafe {
			input.expect_colon()?;
			let ty = PaddingBottomType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "padding-right" => unsafe {
			input.expect_colon()?;
			let ty = PaddingRightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "padding-top" => unsafe {
			input.expect_colon()?;
			let ty = PaddingTopType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "padding" => unsafe {
			input.expect_colon()?;
			let ty = PaddingType(Padding(parse_top_right_bottom_left(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-left" => unsafe {
			input.expect_colon()?;
			let ty = BorderLeftType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-bottom" => unsafe {
			input.expect_colon()?;
			let ty = BorderBottomType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-right" => unsafe {
			input.expect_colon()?;
			let ty = BorderRightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-top" => unsafe {
			input.expect_colon()?;
			let ty = BorderTopType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border" => unsafe {
			input.expect_colon()?;
			let ty = BorderType(Border(parse_top_right_bottom_left(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "border-width" => unsafe {
			input.expect_colon()?;
			let ty = BorderType(Border(parse_top_right_bottom_left(input)?));
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "min-width" => unsafe {
			input.expect_colon()?;
			let ty = MinWidthType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "min-height" => unsafe {
			input.expect_colon()?;
			let ty = MinHeightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "max-width" => unsafe {
			input.expect_colon()?;
			let ty = MaxWidthType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "max-height" => unsafe {
			input.expect_colon()?;
			let ty = MaxHeightType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "flex-basis" => unsafe {
			input.expect_colon()?;
			let ty = FlexBasisType(Dimension::parse(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "flex-shrink" => unsafe {
			input.expect_colon()?;
			let ty = FlexShrinkType(input.expect_number()?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "flex-grow" => unsafe {
			input.expect_colon()?;
			let ty = FlexGrowType(input.expect_number()?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "position" => unsafe {
			input.expect_colon()?;
			let ty = PositionTypeType(parse_yg_position_type(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "flex-wrap" => unsafe {
			input.expect_colon()?;
			let ty = FlexWrapType(parse_yg_wrap(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "flex-direction" => unsafe {
			input.expect_colon()?;
			let ty = FlexDirectionType(parse_yg_direction(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "align-content" => unsafe {
			input.expect_colon()?;
			let ty = AlignContentType(parse_yg_align_content(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "align-items" => unsafe {
			input.expect_colon()?;
			let ty = AlignItemsType(parse_yg_align_items(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "align-self" => unsafe {
			input.expect_colon()?;
			let ty = AlignSelfType(parse_yg_align_self(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
        }
        "justify-content" => unsafe {
			input.expect_colon()?;
			let ty = JustifyContentType(parse_yg_justify_content(input)?);
			log::debug!("{:?}", ty);
			ty.write(buffer);
			class_meta.class_style_mark.set(ty.get_type() as usize, true);
		},
		key_name => {
			log::info!("Unexpected attribute: {:?}", key_name);
			loop {
				if input.is_exhausted() {
					return Ok(());
				}
				if let Ok(_) = input.expect_semicolon() {
					return Ok(());
				}
			}
		},
	};
	let _r = input.try_parse(|input| input.expect_semicolon());
	Ok(())
}

pub trait StyleParse: Sized {
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ValueParseErrorKind>>;
}

impl StyleParse for Dimension {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ValueParseErrorKind>> {
        if input.try_parse(|i| i.expect_ident_matching("auto")).is_ok() {
            return Ok(Dimension::Auto);
        }
		
		let location = input.current_source_location();
		let token = input.next()?;
		let dimension = match *token {
            Token::Dimension { value, ref unit, ..} => {
				match unit.as_ref() {
					"px" => Dimension::Points(value),
					_ => return Err(location.new_unexpected_token_error(token.clone())),
				}
			},
            Token::Percentage { unit_value, .. } => {
				Dimension::Percent(unit_value / 100.0)
            },
            Token::Number { value, .. } => {
				Dimension::Points(value)
			},
            _ => return Err(location.new_unexpected_token_error(token.clone())),
        };
		Ok(dimension)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Percentage(pub f32);

impl StyleParse for Percentage {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ValueParseErrorKind>> {
		Ok(Percentage(input.expect_percentage()?))
    }
}

fn parse_len<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let token = input.next()?;
	let dimension = match *token {
		Token::Dimension { value, ref unit, ..} => {
			match unit.as_ref() {
				"px" => value,
				_ => return Err(location.new_unexpected_token_error(token.clone())),
			}
		},
		Token::Number { value, .. } => {
			value
		},
		_ => return Err(location.new_unexpected_token_error(token.clone())),
	};
	Ok(dimension)
}

fn parse_filter1<'i, 't>(buffer: &mut Vec<u8>, class_meta: &mut ClassMeta, input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ValueParseErrorKind>> {
	let mut hah_hsi = false;
	let mut hsi = Hsi { hue_rotate: 0.0, saturate: 0.0, bright_ness: 0.0 };
	loop {
		let location = input.current_source_location();
		let function = match input.expect_function() {
			Ok(f) => f.clone(),
			Err(_e) => break,
		};
		

		input.parse_nested_block(|i| {
			match function.as_ref() {
				"blur" => {
					let ty = BlurType(Blur( match i.try_parse(|i| Dimension::parse(i))? {
						Dimension::Points(r) => r,
						_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidBlur)),
					}));
					log::debug!("{:?}", ty);
					unsafe { ty.write(buffer) };
					class_meta.class_style_mark.set(ty.get_type() as usize, true);
				},
				"hue-rotate" => {
					let r = i.try_parse(|i| parse_angle(i))?;
					hsi.hue_rotate = if r > 180.0 {r - 360.0}else{r};
					hah_hsi = true;
				},
				"saturate" => {
					hsi.saturate = i.try_parse(|i| i.expect_percentage())?*100.0 - 100.0;
					hah_hsi = true;
				},
				"brightness" => {
					hsi.bright_ness = i.try_parse(|i| i.expect_percentage())?*100.0 - 100.0;
					hah_hsi = true;
				},
				"grayscale" => {
					hsi.saturate = -i.try_parse(|i| i.expect_percentage())?*100.0;
					hah_hsi = true;
				},
				"hsi" => {
					i.try_parse(|i| {
						let location = i.current_source_location(); 
						i.skip_whitespace();
						i.parse_until_before::<_, _, ValueParseErrorKind>(Delimiter::Comma, |i| {
							hsi.hue_rotate = trans_hsi_h(i.expect_number()?);
							Ok(())
						})?;
						match i.next() {
							Ok(&Token::Comma) => (),
							_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
						}
						i.skip_whitespace();
						i.parse_until_before::<_, _, ValueParseErrorKind>(Delimiter::Comma, |i| {
							hsi.saturate = trans_hsi_s(i.expect_number()?);
							Ok(())
						})?;
						match i.next() {
							Ok(&Token::Comma) => (),
							_ => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
						}
						i.skip_whitespace();
						i.parse_until_before::<_, _, ValueParseErrorKind>(Delimiter::Comma, |i| {
							hsi.bright_ness = trans_hsi_i(i.expect_number()?);
							Ok(())
						})?;
						match i.next() {
							Ok(&Token::Comma) | Err(_) => (),
							Ok(_) => return Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
						}
						hah_hsi = true;
						Ok(())
					})?;
					
				},
				_ => return Err(location.new_custom_error(
					ValueParseErrorKind::InvalidFilter
				)),
			};
			Ok(())
		})?;
	}

	if hah_hsi {
		let ty = HsiType(hsi);
		log::debug!("{:?}", ty);
		unsafe { ty.write(buffer) };
		class_meta.class_style_mark.set(ty.get_type() as usize, true);
	}

	Ok(())
}

fn parse_color<'i, 't>(input: &mut Parser<'i, 't>) -> Result<CgColor, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let token = input.next()?;
	match *token {
		Token::Hash(ref value) | Token::IDHash(ref value) => {
			match parse_color_hex(value.as_ref()) {
				Ok(r) => Ok(r),
				Err(_) => Err(location.new_custom_error(ValueParseErrorKind::InvalidColor))
			}
		}
		Token::Ident(ref value) => match parse_color_keyword(value.as_ref()) {
			Ok(r) => Ok(r),
			Err(_) => Err(location.new_custom_error(ValueParseErrorKind::InvalidColor))
		},
		Token::Function(ref name) => {
			let name = name.clone();
			input.parse_nested_block(|input| {
				parse_color_function(&*name, input)
			})
		},
		_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidColor)),
	}
}

fn parse_background<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Color, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let function = input.expect_function()?;
	match function.as_ref(){
		"linear-gradient" => Ok(Color::LinearGradient(input.parse_nested_block(parse_linear)?)),
		_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidBackground)),
	}
}

fn parse_linear<'i, 't>(
	input: &mut Parser<'i, 't>,
) -> Result<LinearGradientColor, ParseError<'i, ValueParseErrorKind>> {
	let direction = if let Ok(d) =
		input.try_parse(|i| parse_angle(i))
	{
		input.expect_comma()?;
		d - 90.0
	} else {
		0.0
	};

	Ok(LinearGradientColor {
		direction,
		list: parse_stops(input)?
	})
}

fn parse_stops<'i, 't>(
	input: &mut Parser<'i, 't>,
) -> Result<Vec<ColorAndPosition>, ParseError<'i, ValueParseErrorKind>> {
	let mut list = Vec::new();
	let mut color_stop = Vec::new();
	let mut pre_percent = 0.0;

	let location = input.current_source_location();

	loop {
		if let Err(e) = parse_stop_item(&mut list, &mut color_stop, &mut pre_percent, input) {
			log::error!("parse_stops fail: {:?}", e);
		}
		match input.next() {
			Ok(&Token::Comma) => continue,
			_ => break,
		}
	}
	if let Err(_) = parser_color_stop_last(1.0, &mut list, &mut color_stop, &mut pre_percent, None) {
		return Err(location.new_custom_error(ValueParseErrorKind::InvalidLinear));
	}

	Ok(color_stop)
}

fn parse_stop_item<'i, 't>(
	list: &mut Vec<CgColor>,
	color_stop: &mut Vec<ColorAndPosition>,
	pre_percent: &mut f32,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let pos = input.try_parse(|i| i.expect_percentage());
	let color = parse_color(input)?;

	if let Ok(v) = pos {
		if let Err(_) = parser_color_stop_last(
			v,
			list,
			color_stop,
			pre_percent,
			Some(color),
		) {
			return Err(location.new_custom_error(ValueParseErrorKind::InvalidLinear));
		}
	} else {
		list.push(color);
	}

	Ok(())
}

pub fn parse_text_shadow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TextShadows, ParseError<'i, ValueParseErrorKind>> {
	let mut arr = SmallVec::default();
	let location = input.current_source_location();
	loop {
		if let Err(e) = parse_text_shadow_item(input, &mut arr) {
			log::error!("parse_text_shadow fail, {:?}", e);
			break;
		}
		
		match input.next() {
			Ok(&Token::Comma) => continue,
			_ => break,
		}
	}
	if arr.len() > 0 {
		Ok(TextShadows(arr))
	} else {
		Err(location.new_custom_error(ValueParseErrorKind::InvalidTextShadow))
	}
	
}

pub fn parse_text_shadow_item<'i, 't>(input: &mut Parser<'i, 't>, arr: &mut SmallVec<[TextShadow;1]>) -> Result<(), ParseError<'i, ValueParseErrorKind>> {
	let mut color = input.try_parse(parse_color);
	let h = input.try_parse(parse_len)?;
	let v = input.try_parse(parse_len)?;
	let blur = input.try_parse(|i| parse_len(i));
	if let Err(_) = color {
		color = input.try_parse(parse_color);
	}
	arr.push(TextShadow {
		h,
		v,
		blur: blur.unwrap_or(0.0),
		color: color.unwrap_or(CgColor::new(0.0, 0.0, 0.0, 1.0)),
	});
	Ok(())
}

fn parse_box_shadow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BoxShadow, ParseError<'i, ValueParseErrorKind>> {
	input.parse_until_before(Delimiter::Comma, |i| {
		let h = parse_len(i)?;
		let v = parse_len(i)?;
		let blur = i.try_parse(|i| parse_len(i));
		let spread = i.try_parse(|i| parse_len(i));
		let color = i.try_parse(parse_color);
		Ok(BoxShadow {
			h,
			v,
			spread: spread.unwrap_or(0.0),
			blur: blur.unwrap_or(0.0),
			color: color.unwrap_or(CgColor::new(0.0, 0.0, 0.0, 1.0)),
		})
	})
}

pub enum GradientImage<'a> {
	Linear(LinearGradientColor),
	Url(CowRcStr<'a>),
}

fn parse_gradient_image<'i, 't>(input: &mut Parser<'i, 't>) -> Result<GradientImage<'i>, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let toke = input.next()?;

	match toke {
		Token::UnquotedUrl(ref value) => Ok(GradientImage::Url(value.clone())),
		Token::Function(ref name) => {
			if name.eq_ignore_ascii_case("url")  {
				input.parse_nested_block(|input| {
					input.expect_string().map_err(Into::into).map(|s| GradientImage::Url(s.clone()))
				})
			} else if name.eq_ignore_ascii_case("url") {
				Ok(GradientImage::Linear(input.parse_nested_block(parse_linear)?))
			} else {
				Err(location.new_custom_error(ValueParseErrorKind::InvalidImage))
			}
		},
		_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidImage))
	}
	
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueParseErrorKind {
    /// An invalid token was encountered while parsing a color value.
    InvalidColor,
    /// An invalid filter value was encountered.
    InvalidFilter,
	InvalidBlur,
	InvalidHsi,
	InvalidAttr,
	InvalidBackground,
	InvalidLinear,
	InvalidTextShadow,
	InvalidImage,
	InvalidObjectFit,
	InvalidRepeat,
}

pub fn parse_angle<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, ParseError<'i, ValueParseErrorKind>> {
	let location = input.current_source_location();
	let t = input.next()?;
	match *t {
		Token::Dimension {
			value, ref unit, ..
		} => {
			match unit.as_ref() {
				"deg" => Ok(value),
				_ => Err(location.new_custom_error(ValueParseErrorKind::InvalidFilter)),
			}
		},
		ref t => {
			let t = t.clone();
			Err(input.new_unexpected_token_error(t))
		},
	}

}

#[inline]
pub fn parse_color_keyword(ident: &str) -> Result<CgColor, ()> {
    macro_rules! rgb {
        ($red: expr, $green: expr, $blue: expr) => {
            CgColor::new(
                $red as f32 / 255.0,
                $green as f32 / 255.0,
                $blue as f32 / 255.0,
                1.0,
            )
        };
    }
    let color = match ident {
		"black" => rgb!(0, 0, 0),
		"silver" => rgb!(192, 192, 192),
		"gray" => rgb!(128, 128, 128),
		"white" => rgb!(255, 255, 255),
		"maroon" => rgb!(128, 0, 0),
		"red" => rgb!(255, 0, 0),
		"purple" => rgb!(128, 0, 128),
		"fuchsia" => rgb!(255, 0, 255),
		"green" => rgb!(0, 128, 0),
		"lime" => rgb!(0, 255, 0),
		"olive" => rgb!(128, 128, 0),
		"yellow" => rgb!(255, 255, 0),
		"navy" => rgb!(0, 0, 128),
		"blue" => rgb!(0, 0, 255),
		"teal" => rgb!(0, 128, 128),
		"aqua" => rgb!(0, 255, 255),

		"aliceblue" => rgb!(240, 248, 255),
		"antiquewhite" => rgb!(250, 235, 215),
		"aquamarine" => rgb!(127, 255, 212),
		"azure" => rgb!(240, 255, 255),
		"beige" => rgb!(245, 245, 220),
		"bisque" => rgb!(255, 228, 196),
		"blanchedalmond" => rgb!(255, 235, 205),
		"blueviolet" => rgb!(138, 43, 226),
		"brown" => rgb!(165, 42, 42),
		"burlywood" => rgb!(222, 184, 135),
		"cadetblue" => rgb!(95, 158, 160),
		"chartreuse" => rgb!(127, 255, 0),
		"chocolate" => rgb!(210, 105, 30),
		"coral" => rgb!(255, 127, 80),
		"cornflowerblue" => rgb!(100, 149, 237),
		"cornsilk" => rgb!(255, 248, 220),
		"crimson" => rgb!(220, 20, 60),
		"cyan" => rgb!(0, 255, 255),
		"darkblue" => rgb!(0, 0, 139),
		"darkcyan" => rgb!(0, 139, 139),
		"darkgoldenrod" => rgb!(184, 134, 11),
		"darkgray" => rgb!(169, 169, 169),
		"darkgreen" => rgb!(0, 100, 0),
		"darkgrey" => rgb!(169, 169, 169),
		"darkkhaki" => rgb!(189, 183, 107),
		"darkmagenta" => rgb!(139, 0, 139),
		"darkolivegreen" => rgb!(85, 107, 47),
		"darkorange" => rgb!(255, 140, 0),
		"darkorchid" => rgb!(153, 50, 204),
		"darkred" => rgb!(139, 0, 0),
		"darksalmon" => rgb!(233, 150, 122),
		"darkseagreen" => rgb!(143, 188, 143),
		"darkslateblue" => rgb!(72, 61, 139),
		"darkslategray" => rgb!(47, 79, 79),
		"darkslategrey" => rgb!(47, 79, 79),
		"darkturquoise" => rgb!(0, 206, 209),
		"darkviolet" => rgb!(148, 0, 211),
		"deeppink" => rgb!(255, 20, 147),
		"deepskyblue" => rgb!(0, 191, 255),
		"dimgray" => rgb!(105, 105, 105),
		"dimgrey" => rgb!(105, 105, 105),
		"dodgerblue" => rgb!(30, 144, 255),
		"firebrick" => rgb!(178, 34, 34),
		"floralwhite" => rgb!(255, 250, 240),
		"forestgreen" => rgb!(34, 139, 34),
		"gainsboro" => rgb!(220, 220, 220),
		"ghostwhite" => rgb!(248, 248, 255),
		"gold" => rgb!(255, 215, 0),
		"goldenrod" => rgb!(218, 165, 32),
		"greenyellow" => rgb!(173, 255, 47),
		"grey" => rgb!(128, 128, 128),
		"honeydew" => rgb!(240, 255, 240),
		"hotpink" => rgb!(255, 105, 180),
		"indianred" => rgb!(205, 92, 92),
		"indigo" => rgb!(75, 0, 130),
		"ivory" => rgb!(255, 255, 240),
		"khaki" => rgb!(240, 230, 140),
		"lavender" => rgb!(230, 230, 250),
		"lavenderblush" => rgb!(255, 240, 245),
		"lawngreen" => rgb!(124, 252, 0),
		"lemonchiffon" => rgb!(255, 250, 205),
		"lightblue" => rgb!(173, 216, 230),
		"lightcoral" => rgb!(240, 128, 128),
		"lightcyan" => rgb!(224, 255, 255),
		"lightgoldenrodyellow" => rgb!(250, 250, 210),
		"lightgray" => rgb!(211, 211, 211),
		"lightgreen" => rgb!(144, 238, 144),
		"lightgrey" => rgb!(211, 211, 211),
		"lightpink" => rgb!(255, 182, 193),
		"lightsalmon" => rgb!(255, 160, 122),
		"lightseagreen" => rgb!(32, 178, 170),
		"lightskyblue" => rgb!(135, 206, 250),
		"lightslategray" => rgb!(119, 136, 153),
		"lightslategrey" => rgb!(119, 136, 153),
		"lightsteelblue" => rgb!(176, 196, 222),
		"lightyellow" => rgb!(255, 255, 224),
		"limegreen" => rgb!(50, 205, 50),
		"linen" => rgb!(250, 240, 230),
		"magenta" => rgb!(255, 0, 255),
		"mediumaquamarine" => rgb!(102, 205, 170),
		"mediumblue" => rgb!(0, 0, 205),
		"mediumorchid" => rgb!(186, 85, 211),
		"mediumpurple" => rgb!(147, 112, 219),
		"mediumseagreen" => rgb!(60, 179, 113),
		"mediumslateblue" => rgb!(123, 104, 238),
		"mediumspringgreen" => rgb!(0, 250, 154),
		"mediumturquoise" => rgb!(72, 209, 204),
		"mediumvioletred" => rgb!(199, 21, 133),
		"midnightblue" => rgb!(25, 25, 112),
		"mintcream" => rgb!(245, 255, 250),
		"mistyrose" => rgb!(255, 228, 225),
		"moccasin" => rgb!(255, 228, 181),
		"navajowhite" => rgb!(255, 222, 173),
		"oldlace" => rgb!(253, 245, 230),
		"olivedrab" => rgb!(107, 142, 35),
		"orange" => rgb!(255, 165, 0),
		"orangered" => rgb!(255, 69, 0),
		"orchid" => rgb!(218, 112, 214),
		"palegoldenrod" => rgb!(238, 232, 170),
		"palegreen" => rgb!(152, 251, 152),
		"paleturquoise" => rgb!(175, 238, 238),
		"palevioletred" => rgb!(219, 112, 147),
		"papayawhip" => rgb!(255, 239, 213),
		"peachpuff" => rgb!(255, 218, 185),
		"peru" => rgb!(205, 133, 63),
		"pink" => rgb!(255, 192, 203),
		"plum" => rgb!(221, 160, 221),
		"powderblue" => rgb!(176, 224, 230),
		"rebeccapurple" => rgb!(102, 51, 153),
		"rosybrown" => rgb!(188, 143, 143),
		"royalblue" => rgb!(65, 105, 225),
		"saddlebrown" => rgb!(139, 69, 19),
		"salmon" => rgb!(250, 128, 114),
		"sandybrown" => rgb!(244, 164, 96),
		"seagreen" => rgb!(46, 139, 87),
		"seashell" => rgb!(255, 245, 238),
		"sienna" => rgb!(160, 82, 45),
		"skyblue" => rgb!(135, 206, 235),
		"slateblue" => rgb!(106, 90, 205),
		"slategray" => rgb!(112, 128, 144),
		"slategrey" => rgb!(112, 128, 144),
		"snow" => rgb!(255, 250, 250),
		"springgreen" => rgb!(0, 255, 127),
		"steelblue" => rgb!(70, 130, 180),
		"tan" => rgb!(210, 180, 140),
		"thistle" => rgb!(216, 191, 216),
		"tomato" => rgb!(255, 99, 71),
		"turquoise" => rgb!(64, 224, 208),
		"violet" => rgb!(238, 130, 238),
		"wheat" => rgb!(245, 222, 179),
		"whitesmoke" => rgb!(245, 245, 245),
		"yellowgreen" => rgb!(154, 205, 50),

		"transparent" => CgColor::new(0.0, 0.0, 0.0, 0.0),
		_ => return Err(()),
    };
	Ok(color)
}

fn parse_color_function<'i, 't>(
    name: &str,
    input: &mut Parser<'i, 't>,
) -> Result<CgColor, ParseError<'i, ValueParseErrorKind>>
{
    let (red, green, blue, uses_commas) = match name {
        "rgb" | "rgba" => parse_rgb_components_rgb(input)?,
        // "hsl" | "hsla" => parse_rgb_components_hsl(input)?,
        _ => return Err(input.new_unexpected_token_error(Token::Ident(name.to_owned().into()))),
    };

    let alpha = if !input.is_exhausted() {
        if uses_commas {
            input.expect_comma()?;
        } else {
            input.expect_delim('/')?;
        };
		input.expect_number()?
    } else {
        1.0
    };

    input.expect_exhausted()?;
    Ok(CgColor::new(red, green, blue, alpha))
}

fn parse_rgb_components_rgb<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<(f32, f32, f32, bool), ParseError<'i, ValueParseErrorKind>>
{
    // Either integers or percentages, but all the same type.
    // https://drafts.csswg.org/css-color/#rgb-functions
    let red = input.expect_number()? ;
    let uses_commas = input.try_parse(|i| i.expect_comma()).is_ok();

	let green = input.expect_number()?/256.0;

	if uses_commas {
		input.expect_comma()?;
	}
	let blue = input.expect_number()?/256.0;

    Ok((red, green, blue, uses_commas))
}



#[test]
fn test1() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let s = ".c123{
		width: 10px;
		height:20px;
		filter:grayscale(50%) hue-rotate(90deg) saturate(20%) brightness(10%);

		background-color: rgba(255, 155, 0, 0.5);
		background-color: rgb(255, 155, 0);
		background-color: #ff00ffff;
		background-color: #ffff;
		background-color: #555;
		background-color: #ffffff;
		background-color: blue;
		background: linear-gradient(20deg, 10% #555, 100% #fff);

		background-image: url('a.png');
		background-image: linear-gradient(20deg, 10% #555, 100% #fff);

		border-image: url('a.png');
		border-image-slice: 10% 10% 20%;
		border-image-clip: 10% 10% 20%;
		border-image-repeat: repeat;

		box-shadow: 10px 10px 5px #888888;
		text-shadow: 2px 2px #ff0000;
	}.c456{width: 10px;height:20px;filter:blur(2px) hsi(10,10,10)}";

	let mut class_sheet = ClassSheet::default();

	if let Err(_r) = parse_class_map_from_string(s, &mut class_sheet) {
	}
	
	// log::debug!("parse: {:?}", parse);

}

#[test]
fn test2() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let s = "@keyframes role1Move1 {
		0.00% {
			width: 98px;
			height: 185px;
			left: 6.67%;
			top: 0.00%;
			background-image: url(psd/3652127002.80.png);
			image-clip: 16.9283% 30.8594% 33.5202% 0.0000%;
		}}.c123{width: 98px;
			height: 185px;}";
	let mut class_sheet = ClassSheet::default();

	if let Err(_r) = parse_class_map_from_string(s, &mut class_sheet) {
	}
}

#[test]
fn test3() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let s = ".c112024820{
		color: #00ffff;
	}";
	let mut class_sheet = ClassSheet::default();

	if let Err(_r) = parse_class_map_from_string(s, &mut class_sheet) {
	}
}

#[test]
fn test4() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let s = ".c1203870451{
		transform: scale(0.8,0.8);
	}";
	let mut class_sheet = ClassSheet::default();

	if let Err(_r) = parse_class_map_from_string(s, &mut class_sheet) {
	}
}

#[test]
fn test5() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let s = ".c2677724671{
		text-shadow: rgb(255,0,0) 0px 0px 5px,rgb(255,0,0) 0px 0px 3px,rgb(255,255,255) 0px 0px 1px;
	}";
	let mut class_sheet = ClassSheet::default();

	if let Err(_r) = parse_class_map_from_string(s, &mut class_sheet) {
	}
}




