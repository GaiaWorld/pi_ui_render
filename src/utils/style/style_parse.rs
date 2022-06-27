//! 解析字符串格式的样式

use std::intrinsics::transmute;
use std::str::FromStr;

use ordered_float::NotNan;
use pi_atom::Atom;
use pi_flex_layout::{style::{Display, Dimension, AlignItems, AlignSelf, AlignContent, FlexDirection, JustifyContent, PositionType, FlexWrap}, prelude::Rect};
use pi_hash::XHashMap;
use smallvec::SmallVec;

use crate::components::{user::{BackgroundColor, Color, BorderColor, ObjectFit, BorderImageRepeat, MaskImage, BorderRadius, Opacity, Hsi, Blur, Enable, WhiteSpace, LinearGradientColor, CgColor, ColorAndPosition, BorderImageSlice, BlendMode, LineHeight, TextAlign, FontSize, TextShadow, BoxShadow, Stroke, TransformFunc, TransformOrigin, LengthUnit, FitType, BorderImageRepeatOption, BackgroundImage, BorderImage, MaskImageClip, BackgroundImageClip, BorderImageClip, Margin, Padding, Overflow, ZIndex, TextShadows}, calc::StyleType};

use super::style_sheet::*;

pub fn parse_class_map_from_string(value: &str) -> Result<ClassSheet, String> {
    let mut parser = ClassMapParser(value);
    let map = XHashMap::default();
	let buffer = Vec::new();
	let mut class_sheet = ClassSheet {style_buffer: buffer, class_map: map};
    loop {
        match parser.next_class() {
            Ok(r) => match r {
                Some(r) => {
					let class = parse_class_from_string(r.1, &mut class_sheet)?;
					class_sheet.class_map.insert(r.0, class);
				},
                None => break,
            },
            Err(_) => continue,
        };
    }
    Ok(class_sheet)
}

pub fn parse_class_from_string(value: &str, class_sheet: &mut ClassSheet) -> Result<ClassMeta, String> {
    let mut class = ClassMeta::default();
	let start_index = class_sheet.style_buffer.len();
    for p in value.split(";") {
        match p.find(":") {
            Some(index) => {
                let p = p.split_at(index);
                let key = p.0.trim();
                let value = p.1[1..p.1.len()].trim();
                match match_key(key, value, &mut class, class_sheet) {
                    Err(r) => log::warn!("err: {}, key: {}, value: {}", r, key, value),
                    _ => (),
                };
            }
            None => {
                if p.trim() != "" {
                    return Err(format!("class parse err: {:?}", p));
                }
            }
        }
    }
	class.start = start_index;
	class.end = class_sheet.style_buffer.len();
    Ok(class)
}

struct ClassMapParser<'a>(&'a str);

impl<'a> ClassMapParser<'a> {
    fn next_class(&mut self) -> Result<Option<(usize, &'a str)>, String> {
        let i = match self.0.find("{") {
            Some(i) => i,
            None => return Ok(None),
        };
        let j = match find_end(&self.0) {
            Some(j) => j,
            None => return Ok(None),
        };

        let r = (
            match usize::from_str(&self.0[..i].trim()[1..]) {
                Ok(r) => r,
                Err(_) => {
                    self.0 = &self.0[j + 1..];
                    return Err("".to_string());
                }
            },
            self.0[i + 1..j].trim(),
        );
        self.0 = &self.0[j + 1..];
        Ok(Some(r))
    }
}

fn find_end(value: &str) -> Option<usize> {
    let mut j1 = 0;
    let mut i1 = 0;
    j1 += match value[j1 + 1..].find("}") {
        Some(r) => r,
        None => return None,
    };
    loop {
        i1 += match value[i1 + 1..].find("{") {
            Some(r) => r + 1,
            None => j1,
        };

        if i1 < j1 {
            j1 += match value[j1 + 1..].find("}") {
                Some(r) => r + 1,
                None => return None,
            };
        } else {
            break;
        }
    }

    Some(j1)
}

fn match_key(key: &str, value: &str, class: &mut ClassMeta, class_sheet: &mut ClassSheet) -> Result<(), String> {
     match key  {
		"filter" => {
			parse_filter(value, class, &mut class_sheet.style_buffer)?;
        }
        "background-color" => unsafe {
			StyleAttr::write(BackgroundColorType(BackgroundColor(Color::RGBA(
                    parse_color_string(value)?,
                ))),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BackgroundColor as usize, true);
        }
        "background" => unsafe {
            if value.starts_with("linear-gradient") {
				StyleAttr::write(
					BackgroundColorType(BackgroundColor(
						parse_linear_gradient_color_string(value)?,
					)),
					&mut class_sheet.style_buffer,
				);
                class.class_style_mark.set(StyleType::BackgroundColor as usize, true);
            } else {
                log::warn!("background err: {}", value);
                return Ok(());
            }
        }

        "border-color" => unsafe {
			StyleAttr::write(
				BorderColorType(BorderColor(parse_color_string(
                    value,
                )?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "box-shadow" => unsafe {
			StyleAttr::write(
				BoxShadowType(parse_box_shadow(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BoxShadow as usize, true);
        }

        "background-image" => unsafe {
			if value.starts_with("linear-gradient") {
				StyleAttr::write(
					BackgroundColorType(BackgroundColor(
						parse_linear_gradient_color_string(value)?,
					)),
					&mut class_sheet.style_buffer,
				);
                class.class_style_mark.set(StyleType::BackgroundColor as usize, true);
            } else {
				StyleAttr::write(
					BackgroundImageType(BackgroundImage(parse_url(value)?.get_hash())),
					&mut class_sheet.style_buffer,
				);
				class.class_style_mark.set(StyleType::BackgroundImage as usize, true);
			}
		}
		"image-clip" => unsafe {
			StyleAttr::write(
				BackgroundImageClipType(BackgroundImageClip(
					transmute(f32_4_to_aabb(parse_percent_to_f32_4(value, " ")?))
				)),
				&mut class_sheet.style_buffer,
			);
            // class.class_style_mark.set  |= StyleType::BackgroundColor as usize;
			class.class_style_mark.set(StyleType::BackgroundColor as usize, true)
        }
        "background-image-clip" => unsafe {
			StyleAttr::write(
				BackgroundImageClipType(BackgroundImageClip(
					transmute(f32_4_to_aabb(parse_percent_to_f32_4(value, " ")?))
				)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BackgroundImageClip as usize, true);
        }
        "object-fit" => unsafe {
			StyleAttr::write(
				ObjectFitType(ObjectFit(parse_object_fit(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::ObjectFit as usize, true);
        }

        "border-image" => unsafe {
			StyleAttr::write(
				BorderImageType(BorderImage(parse_url(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderImage as usize, true);
        }
        "border-image-clip" => unsafe {
			StyleAttr::write(
				BorderImageClipType(BorderImageClip(transmute(f32_4_to_aabb(parse_percent_to_f32_4(value, " ")?)))),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderImageClip as usize, true);
        }
        "border-image-slice" => unsafe {
			StyleAttr::write(
				BorderImageSliceType(parse_border_image_slice(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderImageSlice as usize, true);
        }
        "border-image-repeat" => unsafe {
			let ty = parse_border_image_repeat(value)?;
			StyleAttr::write(
				BorderImageRepeatType(BorderImageRepeat(ty, ty)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderImageRepeat as usize, true);
		}
		"mask-image" => unsafe {
			if value.starts_with("linear-gradient") {
				StyleAttr::write(
					MaskImageType(MaskImage::LinearGradient(
						parse_linear_gradient_color(value)?
					)),
					&mut class_sheet.style_buffer,
				);
			} else {
				StyleAttr::write(
					MaskImageType(MaskImage::Path(parse_url(value)?.get_hash())),
					&mut class_sheet.style_buffer,
				);
			}
			class.class_style_mark.set(StyleType::MaskImage as usize, true);
		}
		"mask-image-clip" => unsafe {
			StyleAttr::write(
				MaskImageClipType(MaskImageClip(
					transmute(f32_4_to_aabb(parse_percent_to_f32_4(value, " ")?))
				)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MaskImageClip as usize, true);
        }
		"blend-mode" => unsafe {
			StyleAttr::write(
				BlendModeType(parse_blend_mode(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BlendMode as usize, true);
        }
		"text-gradient" => unsafe {
			StyleAttr::write(
				ColorType(parse_linear_gradient_color_string(value)?),
				&mut class_sheet.style_buffer,
			);
			class.class_style_mark.set(StyleType::Color as usize, true);
		}
        "color" => unsafe {
			StyleAttr::write(
				ColorType(Color::RGBA(parse_color_string(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Color as usize, true);
        }
        "letter-spacing" => unsafe {
			StyleAttr::write(
				LetterSpacingType(parse_px(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::LetterSpacing as usize, true);
        }
        "line-height" => unsafe {
			StyleAttr::write(
				LineHeightType(parse_line_height(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::LineHeight as usize, true);
        }
        "text-align" => unsafe {
			StyleAttr::write(
				TextAlignType(parse_text_align(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::TextAlign as usize, true);
        }
        "text-indent" => unsafe {
			StyleAttr::write(
				TextIndentType(parse_px(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::TextIndent as usize, true);
        }
        "text-shadow" => unsafe {
			StyleAttr::write(
				TextShadowType(TextShadows(parse_text_shadow(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::TextShadow as usize, true);
        }
        // "vertical-align" => show_attr.push(Attribute::Color( Color::RGBA(parse_color_string(value)?) )),
        "white-space" => unsafe {
			StyleAttr::write(
				WhiteSpaceType(pasre_white_space(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::WhiteSpace as usize, true);
        }
        "word-spacing" => unsafe {
			StyleAttr::write(
				WordSpacingType(parse_px(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::WordSpacing as usize, true);
        }

        "text-stroke" => unsafe {
			StyleAttr::write(
				TextStrokeType(parse_text_stroke(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::TextStroke as usize, true);
        }

        // "font-style" => show_attr.push(Attribute::FontStyle( Color::RGBA(parse_color_string(value)?) )),
        "font-weight" => unsafe {
			StyleAttr::write(
				FontWeightType(parse_font_weight(value)? as usize),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FontWeight as usize, true);
        }
        "font-size" => unsafe {
			StyleAttr::write(
				FontSizeType(parse_font_size(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FontSize as usize, true);
        }
        "font-family" => unsafe {
			StyleAttr::write(
				FontFamilyType(Atom::from(value)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FontFamily as usize, true);
        }

        "border-radius" => unsafe {
			let v = parse_len_or_percent(value)?;
			StyleAttr::write(
				BorderRadiusType(BorderRadius {
					x: v.clone(),
					y: v,
				}),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderRadius as usize, true);
        }
        "opacity" => unsafe {
			StyleAttr::write(
				OpacityType(Opacity(parse_f32(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Opacity as usize, true);
        }
        "transform" => unsafe {
			StyleAttr::write(
				TransformType(parse_transform(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Transform as usize, true);
        }
        "transform-origin" => unsafe {
			StyleAttr::write(
				TransformOriginType(parse_transform_origin(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::TransformOrigin as usize, true);
        }
        "z-index" => unsafe {
			StyleAttr::write(
				ZIndexType(ZIndex(parse_f32(value)? as isize)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::ZIndex as usize, true);
        }
        "visibility" => unsafe {
			StyleAttr::write(
				VisibilityType(parse_visibility(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Visibility as usize, true);
        }
        "pointer-events" => unsafe {
			StyleAttr::write(
				EnableType(parse_enable(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Enable as usize, true);
        }
        "display" => unsafe {
			StyleAttr::write(
				DisplayType(parse_display(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Display as usize, true);
        }
        "overflow" => unsafe {
			StyleAttr::write(
				OverflowType(Overflow(parse_overflow(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Overflow as usize, true);
        }
        "overflow-y" => unsafe {
			StyleAttr::write(
				OverflowType(Overflow(parse_overflow(value)?)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Overflow as usize, true);
        }
        "width" => unsafe {
			StyleAttr::write(
				WidthType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Width as usize, true);
        }
        "height" => unsafe {
			StyleAttr::write(
				HeightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Height as usize, true);
        }
        "left" => unsafe {
			StyleAttr::write(
				PositionLeftType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PositionLeft as usize, true);
        }
        "bottom" => unsafe {
			StyleAttr::write(
				PositionBottomType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PositionBottom as usize, true);
        }
        "right" => unsafe {
            StyleAttr::write(
				PositionRightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PositionRight as usize, true);
        }
        "top" => unsafe {
            StyleAttr::write(
				PositionTopType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PositionTop as usize, true);
        }
        "margin-left" => unsafe {
            StyleAttr::write(
				MarginLeftType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MarginLeft as usize, true);
        }
        "margin-bottom" => unsafe {
            StyleAttr::write(
				MarginBottomType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MarginBottom as usize, true);
        }
        "margin-right" => unsafe {
            StyleAttr::write(
				MarginRightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MarginRight as usize, true);
        }
        "margin-top" => unsafe {
            StyleAttr::write(
				MarginTopType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MarginTop as usize, true);
        }
        "margin" => unsafe {
            let [top, right, bottom, left] = parse_four_f32(value)?;
			StyleAttr::write(
				MarginType(Margin(Rect{top, right, bottom, left})),
				&mut class_sheet.style_buffer,
			);
			class.class_style_mark.set(StyleType::Margin as usize, true);
        }
        "padding-left" => unsafe {
			StyleAttr::write(
				PaddingLeftType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PaddingLeft as usize, true);
        }
        "padding-bottom" => unsafe {
            StyleAttr::write(
				PaddingBottomType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PaddingBottom as usize, true);
        }
        "padding-right" => unsafe {
            StyleAttr::write(
				PaddingRightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PaddingRight as usize, true);
        }
        "padding-top" => unsafe {
            StyleAttr::write(
				PaddingTopType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PaddingTop as usize, true);
        }
        "padding" => unsafe {
			let [top, right, bottom, left] = parse_four_f32(value)?;
			StyleAttr::write(
				PaddingType(Padding(Rect{top, right, bottom, left})),
				&mut class_sheet.style_buffer,
			);
			class.class_style_mark.set(StyleType::Padding as usize, true);
        }
        "border-left" => unsafe {
            let r = parse_border(value)?;
			StyleAttr::write(
				BorderLeftType(r.0),
				&mut class_sheet.style_buffer,
			);
			StyleAttr::write(
				BorderColorType(BorderColor(r.1)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderLeft as usize, true);
            class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "border-bottom" => unsafe {
            let r = parse_border(value)?;
			StyleAttr::write(
				BorderBottomType(r.0),
				&mut class_sheet.style_buffer,
			);
			StyleAttr::write(
				BorderColorType(BorderColor(r.1)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderBottom as usize, true);
            class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "border-right" => unsafe {
            let r = parse_border(value)?;
			StyleAttr::write(
				BorderRightType(r.0),
				&mut class_sheet.style_buffer,
			);
			StyleAttr::write(
				BorderColorType(BorderColor(r.1)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderRight as usize, true);
            class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "border-top" => unsafe {
            let r = parse_border(value)?;
			StyleAttr::write(
				BorderTopType(r.0),
				&mut class_sheet.style_buffer,
			);
			StyleAttr::write(
				BorderColorType(BorderColor(r.1)),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::BorderTop as usize, true);
            class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "border" => unsafe {
            let r = parse_border(value)?;
			StyleAttr::write(
				PaddingType(Padding(Rect{top: r.0, right: r.0, bottom: r.0, left: r.0})),
				&mut class_sheet.style_buffer,
			);
            StyleAttr::write(
				BorderColorType(BorderColor(r.1)),
				&mut class_sheet.style_buffer,
			);
			class.class_style_mark.set(StyleType::Border as usize, true);
			class.class_style_mark.set(StyleType::BorderColor as usize, true);
        }
        "border-width" => unsafe {
            let [top, right, bottom, left] = parse_four_f32(value)?;
            StyleAttr::write(
				PaddingType(Padding(Rect{top, right, bottom, left})),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::Border as usize, true);
        }
        "min-width" => unsafe {
			StyleAttr::write(
				MinWidthType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MinWidth as usize, true);
        }
        "min-height" => unsafe {
            StyleAttr::write(
				MinHeightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MinHeight as usize, true);
        }
        "max-width" => unsafe {
            StyleAttr::write(
				MaxWidthType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MaxWidth as usize, true);
        }
        "max-height" => unsafe {
            StyleAttr::write(
				MaxHeightType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::MaxHeight as usize, true);
        }
        "flex-basis" => unsafe {
            StyleAttr::write(
				FlexBasisType(parse_unity(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FlexBasis as usize, true);
        }
        "flex-shrink" => unsafe {
			StyleAttr::write(
				FlexShrinkType(parse_f32(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FlexShrink as usize, true);
        }
        "flex-grow" => unsafe {
			StyleAttr::write(
				FlexGrowType(parse_f32(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FlexGrow as usize, true);
        }
        "position" => unsafe {
			StyleAttr::write(
				PositionTypeType(parse_yg_position_type(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::PositionType as usize, true);
        }
        "flex-wrap" => unsafe {
			StyleAttr::write(
				FlexWrapType(parse_yg_wrap(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FlexWrap as usize, true);
        }
        "flex-direction" => unsafe {
			StyleAttr::write(
				FlexDirectionType(parse_yg_direction(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::FlexDirection as usize, true);
        }
        "align-content" => unsafe {
			StyleAttr::write(
				AlignContentType(parse_yg_align_content(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::AlignContent as usize, true);
        }
        "align-items" => unsafe {
			StyleAttr::write(
				AlignItemsType(parse_yg_align_items(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::AlignItems as usize, true);
        }
        "align-self" => unsafe {
			StyleAttr::write(
				AlignSelfType(parse_yg_align_self(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::AlignSelf as usize, true);
        }
        "justify-content" => unsafe {
			StyleAttr::write(
				JustifyContentType(parse_yg_justify_content(value)?),
				&mut class_sheet.style_buffer,
			);
            class.class_style_mark.set(StyleType::JustifyContent as usize, true);
		},
        _ => (),
    };
    Ok(())
}

fn parse_filter(value: &str, class: &mut ClassMeta, buffer: &mut Vec<u8>) -> Result<(), String>{
	let mut i = 0;
	let mut hsi = Hsi::default(); 
	let mut hah_hsi = false;
	let err = Err(format!("parse_filter fail, str: {:?}", value));
	loop {
		match iter_fun(value, &mut i) {
			Ok((n, v)) => {
				match n {
					"hue-rotate" => {
						let r = parse_deg(v)?;
						hsi.hue_rotate = if r > 180.0 {r - 360.0}else{r};
						hah_hsi = true;
					},
					"saturate" => {
						hsi.saturate = parse_percent_to_f32(v)?*100.0 - 100.0;
						hah_hsi = true;
					},
					"brightness" => {
						hsi.bright_ness = parse_percent_to_f32(v)?*100.0 - 100.0;
						hah_hsi = true;
					},
					"grayscale" => {
						hsi.saturate = -parse_percent_to_f32(v)?*100.0;
						hah_hsi = true;
					},
					"hsi" => {
						let mut iter = v.split(",");
						let h = match iter.next() {
							Some(r) => parse_f32(r.trim())?,
							None => return err,
						} ;
						let s = match iter.next() {
							Some(r) => parse_f32(r.trim())?,
							None => return err,
						};
						let i = match iter.next() {
							Some(r) => parse_f32(r.trim())?,
							None => return err,
						};
						hsi = trans_filter(h, s, i);
						hah_hsi = true;
					},
					"blur" => unsafe {
						StyleAttr::write(
							BlurType(Blur(parse_f32(&v[0..v.len() - 2])?)),
							buffer,
						);
						class.class_style_mark.set(StyleType::Blur as usize, true);
					},
					_ => (),
				};
			},
			Err(_) => break,
		}
	}
	if hah_hsi {
		unsafe { StyleAttr::write(
			HsiType(hsi),
			buffer,
		)};
		class.class_style_mark.set(StyleType::Hsi as usize, true);
	}
	Ok(())
}

// fn parse_filter(value: &str) -> Result<Filter, String>{
// 	let err = Err(format!("parse_filter fail, str: {:?}", value));
// 	if value.starts_with("hsi(") && value.ends_with(")") {
// 		let mut iter = value[4..value.len() - 1].trim().split(",");
// 		let h = match iter.next() {
// 			Some(r) => parse_f32(r)?,
// 			None => return err,
// 		} ;
// 		let s = match iter.next() {
// 			Some(r) => parse_f32(r)?,
// 			None => return err,
// 		};
// 		let i = match iter.next() {
// 			Some(r) => parse_f32(r)?,
// 			None => return err,
// 		};
// 		return Ok(trans_filter(h, s, i));
// 	} else {
// 		let mut hsi = Filter::default();
// 		let mut i = 0;
// 		loop {
// 			match iter_fun(value, &mut i) {
// 				Ok((n, v)) => {
// 					match n {
// 						"hue-rotate" => {
// 							let r = parse_deg(v)?;
// 							hsi.hue_rotate = if r > 180.0 {r - 360.0}else{r};
// 						},
// 						"saturate" => hsi.saturate = parse_percent_to_f32(v)?*100.0 - 100.0,
// 						"brightness" => hsi.bright_ness = parse_percent_to_f32(v)?*100.0 - 100.0,
// 						"grayscale" => hsi.saturate = -parse_percent_to_f32(v)?*100.0,
// 						_ => (),
// 					};
// 				},
// 				Err(_) => break,
// 			}
// 		}
// 		Ok(hsi)
// 	}
// }

fn trans_filter(mut h: f32, mut s: f32, mut i: f32) -> Hsi {
	if h > 180.0 {
        h = 180.0;
    } else if h < -180.0 {
        h = -180.0
    }
    if s > 100.0 {
        s = 100.0;
    } else if s < -100.0 {
        s = -100.0
    }
    if i > 100.0 {
        i = 100.0;
    } else if i < -100.0 {
        i = -100.0
    }
    return Hsi {
        hue_rotate: h / 360.0,
        saturate: s / 100.0,
        bright_ness: i / 100.0,
	};
}

fn parse_enable(value: &str) -> Result<Enable, String> {
    match value {
        "auto" => Ok(Enable::Auto),
        "none" => Ok(Enable::None),
        "visible" => Ok(Enable::Visible),
        _ => return Err(format!("parse_enable:{}", value)),
    }
}

fn parse_visibility(value: &str) -> Result<bool, String> {
    match value {
        "hidden" => Ok(false),
        "visible" => Ok(true),
        _ => return Err(format!("parse_visibility:{}", value)),
    }
}

fn parse_display(value: &str) -> Result<Display, String> {
    match value {
        "flex" => Ok(Display::Flex),
        "none" => Ok(Display::None),
        _ => Ok(Display::Flex), // 默认情况
    }
}

fn parse_overflow(value: &str) -> Result<bool, String> {
    match value {
        "hidden" => Ok(true),
        _ => Ok(false), // 默认情况
    }
}

fn pasre_white_space(value: &str) -> Result<WhiteSpace, String> {
    let r = match value {
        "normal" => WhiteSpace::Normal,
        "pre" => WhiteSpace::Pre,
        "nowrap" => WhiteSpace::Nowrap,
        "pre-wrap" => WhiteSpace::PreWrap,
        "pre-line" => WhiteSpace::PreLine,
        _ => return Err(format!("pasre_white_space_err:{}", value)),
    };
    Ok(r)
}

fn parse_linear_gradient_color_string(value: &str) -> Result<Color, String> {
    let color = parse_linear_gradient_color(value)?;
    Ok(Color::LinearGradient(color))
}

fn parse_linear_gradient_color(value: &str) -> Result<LinearGradientColor, String> {
    let value = &value[15..].trim();
    let value = value[1..value.len() - 1].trim();
    let mut iter = value.split(",");
    let first = iter.nth(0);
    let mut color = LinearGradientColor::default();
    let mut list = Vec::new();
    let mut pre_percent = 0.0;
    match first {
        Some(first) => {
            let first = first.trim();
            if first.ends_with("deg") {
                color.direction = parse_f32(&first[0..first.len() - 3])? - 90.0;
            } else {
                parser_color_stop(first, &mut list, &mut color.list, &mut pre_percent)?;
            }
        }
        None =>  {
			return Ok(color)
		},
    };

    for value in iter {
        let value = value.trim();
        parser_color_stop(value, &mut list, &mut color.list, &mut pre_percent)?;
    }

    parser_color_stop_last(1.0, &mut list, &mut color.list, &mut pre_percent, None)?;

    Ok(color)
}

fn parser_color_stop(
    value: &str,
    list: &mut Vec<CgColor>,
    color_stop: &mut Vec<ColorAndPosition>,
    pre_percent: &mut f32,
) -> Result<(), String> {
    if value.ends_with("%") {
        if let Some(index) = value.find(" ") {
            let r = value.split_at(index);
            let pos = r.1.trim();
            let v = match f32::from_str(&pos[0..pos.len() - 1]) {
                Ok(r) => r,
                Err(e) => return Err(e.to_string()),
            };
            let v = v / 100.0;
            return parser_color_stop_last(
                v,
                list,
                color_stop,
                pre_percent,
                Some(parse_color_string(r.0.trim())?),
            );
        }
    }
    list.push(parse_color_string(value.trim())?);
    Ok(())
}

// fn parse_f32_2(value: &str, split: &str) -> Result<[f32; 2], String> {
//     let mut r = [1.0, 1.0];
//     let mut i = 0;
//     for v in value.split(split) {
//         if i > 1 {
//             return Err(format!("parse_f32_2 error, value: {:?}", value));
//         }
//         let v = v.trim();
//         if v != "" {
//             r[i] = parse_f32(v)?;
//             i += 1;
//         }
//     }
//     Ok(r)
// }

fn parse_f32_3(value: &str, split: &str) -> Result<[f32; 3], String> {
    let mut r = [0.0, 0.0, 0.0];
    let mut i = 0;
    for v in value.split(split) {
        if i > 2 {
            return Err(format!("parse_f32_2 error, value: {:?}", value));
        }
        let v = v.trim();
        if v != "" {
            r[i] = parse_f32(v)?;
            i += 1;
        }
    }
    Ok(r)
}

fn f32_4_to_aabb(value: [f32; 4]) -> [f32; 4]{
	return [value[3], value[0], value[1], value[2]];
}

fn parse_percent_to_f32_4(value: &str, split: &str) -> Result<[f32; 4], String> {
    let mut r = Vec::new();
    let mut i = 0;
    for v in value.split(split) {
        if i > 3 {
            return Err(format!("parse_percent_to_f32_4 error, value: {:?}", value));
        }
        let v = v.trim();
        if v != "" {
            r.push(parse_percent_to_f32(v)?);
            i += 1;
        }
    }
    Ok(to_four_f32(&r)?)
}

fn parse_f32_4(value: &str, split: &str) -> Result<[f32; 4], String> {
    let mut r = [0.0, 0.0, 0.0, 0.0];
    let mut i = 0;
    for v in value.split(split) {
        if i > 3 {
            return Err(format!("parse_f32_4 error, value: {:?}", value));
        }
        let v = v.trim();
        if v != "" {
            r[i] = parse_f32(v)?;
            i += 1;
        }
    }
    Ok(r)
}

fn parse_border_image_slice(value: &str) -> Result<BorderImageSlice, String> {
    let mut slice = BorderImageSlice::default();
    let mut arr = Vec::default();
    let mut i = 0;
    for v in value.split(" ") {
        if i > 4 {
            return Err(format!(
                "parse_border_image_slice error, value: {:?}",
                value
            ));
        }
        let v = v.trim();
        match v {
            "fill" => slice.fill = true,
            " " => (),
            _ => {
                arr.push(v);
                i += 1;
            }
        };
    }
    let r = to_four(arr)?;
    match r[0] {
        Dimension::Percent(r) => slice.top = unsafe { NotNan::unchecked_new(r) },
        _ => (),
    };
    match r[1] {
        Dimension::Percent(r) => slice.right = unsafe { NotNan::unchecked_new(r) },
        _ => (),
    };
    match r[2] {
        Dimension::Percent(r) => slice.bottom = unsafe { NotNan::unchecked_new(r) },
        _ => (),
    };
    match r[3] {
        Dimension::Percent(r) => slice.left = unsafe { NotNan::unchecked_new(r) },
        _ => (),
    };
    Ok(slice)
}

fn parse_blend_mode(value: &str) -> Result<BlendMode, String> {
	match value {
		"normal" => Ok(BlendMode::Normal),
		"alpha-add" => Ok(BlendMode::AlphaAdd),
		"subtract" => Ok(BlendMode::Subtract),
		"multiply" => Ok(BlendMode::Multiply),
		"one-one" => Ok(BlendMode::OneOne),
        _ => Err(format!("parse_blend_mode error, value: {}, please modify to: normal | screen", value)),
	}
}

fn parse_font_weight(value: &str) -> Result<f32, String> {
    match value {
        "bold" => Ok(700.0),
        _ => parse_f32(value),
    }
}

fn parse_line_height(value: &str) -> Result<LineHeight, String> {
    let r = if value == "normal" {
        LineHeight::Normal
    } else if value.ends_with("%") {
        let v = match f32::from_str(value) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        LineHeight::Percent(v / 100.0)
    } else if value.ends_with("px") {
        let v = match f32::from_str(&value[0..value.len() - 2]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        LineHeight::Length(v)
    } else {
        return Err(format!("parse_line_height error, value: {}", value));
    };
    Ok(r)
}

fn parse_text_align(value: &str) -> Result<TextAlign, String> {
    match value {
        "left" => Ok(TextAlign::Left),
        "right" => Ok(TextAlign::Right),
        "center" => Ok(TextAlign::Center),
        "justify" => Ok(TextAlign::Justify),
        _ => Err(format!("parse_text_align error, value: {}", value)),
    }
}

fn parse_yg_align_items(value: &str) -> Result<AlignItems, String> {
    match value {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignItems::FlexStart),
        "center" => Ok(AlignItems::Center),
        "flex-end" => Ok(AlignItems::FlexEnd),
        "stretch" => Ok(AlignItems::Stretch),
        "baseline" => Ok(AlignItems::Baseline),
        _ => Err(format!("parse_yg_align_items error, value: {}", value)),
    }
}

fn parse_yg_align_self(value: &str) -> Result<AlignSelf, String> {
    match value {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignSelf::FlexStart),
        "center" => Ok(AlignSelf::Center),
        "flex-end" => Ok(AlignSelf::FlexEnd),
        "stretch" => Ok(AlignSelf::Stretch),
        "baseline" => Ok(AlignSelf::Baseline),
        _ => Err(format!("parse_yg_align_self error, value: {}", value)),
    }
}

fn parse_yg_align_content(value: &str) -> Result<AlignContent, String> {
    match value {
		// "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignContent::FlexStart),
        "center" => Ok(AlignContent::Center),
        "flex-end" => Ok(AlignContent::FlexEnd),
        "stretch" => Ok(AlignContent::Stretch),
        "space-between" => Ok(AlignContent::SpaceBetween),
        "space-around" => Ok(AlignContent::SpaceAround),
        _ => Err(format!("parse_yg_align_content error, value: {}", value)),
    }
}

fn parse_yg_direction(value: &str) -> Result<FlexDirection, String> {
    match value {
        "column" => Ok(FlexDirection::Column),
        "column-reverse" => Ok(FlexDirection::ColumnReverse),
        "row" => Ok(FlexDirection::Row),
        "row-reverse" => Ok(FlexDirection::RowReverse),
        _ => Err(format!("parse_yg_direction error, value: {}", value)),
    }
}

fn parse_yg_justify_content(value: &str) -> Result<JustifyContent, String> {
    match value {
        "flex-start" => Ok(JustifyContent::FlexStart),
        "center" => Ok(JustifyContent::Center),
        "flex-end" => Ok(JustifyContent::FlexEnd),
        "space-between" => Ok(JustifyContent::SpaceBetween),
        "space-around" => Ok(JustifyContent::SpaceAround),
        _ => Err(format!("parse_yg_justify_content error, value: {}", value)),
    }
}

fn parse_yg_position_type(value: &str) -> Result<PositionType, String> {
    match value {
        "relative" => Ok(PositionType::Relative),
        "absolute" => Ok(PositionType::Absolute),
        _ => Err(format!("parse_yg_position_type error, value: {}", value)),
    }
}

fn parse_yg_wrap(value: &str) -> Result<FlexWrap, String> {
    match value {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => Err(format!("parse_yg_wrap error, value: {}", value)),
    }
}

fn parse_font_size(value: &str) -> Result<FontSize, String> {
    if value.ends_with("%") {
        let v = match f32::from_str(value) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(FontSize::Percent(v / 100.0))
    } else if value.ends_with("px") {
        let v = match f32::from_str(&value[0..value.len() - 2]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(FontSize::Length(v as usize))
    } else {
        Err("parse_font_size error".to_string())
    }
}

pub fn parse_text_shadow(value: &str) -> Result<SmallVec<[TextShadow;1]>, String> {
    let mut i = 0;
	let mut arr = SmallVec::default();
	
	loop {
		parse_text_shadow1(value, &mut arr, &mut i)?;
		if i >= value.len() {
			break;
		}
	}

	return Ok(arr)
}

fn parse_text_shadow1(value: &str, arr: &mut SmallVec<[TextShadow;1]>, i: &mut usize) -> Result<(), String> {
    let mut shadow = TextShadow::default();
	let mut px = Vec::with_capacity(3);

	loop {
		match parse_color_string1(value, i) {
			Ok(r) => shadow.color = r,
			Err(_) => {
				*i = next_no_empty(value, *i);
				let r = iter_by(value, i, &[",", " "]);
				if let Ok(r) = r {
					if let Ok(r) = parse_px(r) {
						px.push(r);
						continue;
					}
					
					*i = *i - r.len();
				}
				break;
			},
		}
	}

	if px.len() > 0 {
		shadow.h = px[0];
	}
	if px.len() > 1 {
		shadow.v = px[1];
	}
	if px.len() > 2 {
		shadow.blur = px[2];
	}

	arr.push(shadow);

	while *i < value.len() - 1 && (&value[*i..*i+1] == "," || &value[*i..*i+1] == " " || &value[*i..*i+1] == "	") {
		*i = *i + 1;
	}

	Ok(())
}

fn parse_box_shadow(value: &str) -> Result<BoxShadow, String> {
    let mut i = 0;
    let mut shadow = BoxShadow::default();

    let r = iter_by_space(value, &mut i)?;
    match parse_color_string(r) {
        Ok(r) => {
            shadow.color = r;
            parse_box_shadow_number(value, &mut i, &mut shadow)?;
        }
        Err(_) => {
            i = 0;
            parse_box_shadow_number(value, &mut i, &mut shadow)?;
            let r = iter_by_space(value, &mut i)?;
            shadow.color = parse_color_string(r)?;
        }
    };
    Ok(shadow)
}

fn parse_box_shadow_number(
    value: &str,
    i: &mut usize,
    shadow: &mut BoxShadow,
) -> Result<(), String> {
    shadow.h = parse_px(iter_by_space(value, i)?)?;
    shadow.v = parse_px(iter_by_space(value, i)?)?;
    let j = *i;
    match iter_by_space(value, i) {
        Ok(r) => match parse_px(r) {
            Ok(r) => {
                shadow.blur = r;
                let j = *i;
                match iter_by_space(value, i) {
                    Ok(r) => match parse_px(r) {
                        Ok(r) => shadow.spread = r,
                        Err(_) => *i = j,
                    },
                    Err(_) => *i = j,
                }
            }
            Err(_) => *i = j,
        },
        _ => {
            return Err("".to_string());
        }
    };
    Ok(())
}

fn parse_text_stroke(value: &str) -> Result<Stroke, String> {
    let mut i = 0;
    let mut stroke = Stroke::default();
	let r = parse_px(iter_by_space(value, &mut i)?)?;
	if r.is_nan() {
		return Err("stroke is nan".to_string());
	}
    stroke.width = NotNan::new( r).unwrap();
    stroke.color = parse_color_string(iter_by_space(value, &mut i)?)?;
    Ok(stroke)
}

fn parse_transform(value: &str) -> Result<Vec<TransformFunc>, String> {
    let mut i = 0;
    let mut transforms = Vec::default();
    loop {
        match iter_fun(value, &mut i) {
            Ok((n, v)) => transforms.push(parse_transform_fun(n, v)?),
            Err(_) => break,
        }
    }
    Ok(transforms)
}

fn parse_transform_origin(value: &str) -> Result<TransformOrigin, String> {
    let mut i = 0;
    Ok(TransformOrigin::XY(
        parse_transform_origin1(iter_by_space(value, &mut i)?)?,
        parse_transform_origin1(iter_by_space(value, &mut i)?)?,
    ))
}

fn parse_transform_origin1(value: &str) -> Result<LengthUnit, String> {
    match value {
        "center" => Ok(LengthUnit::Percent(50.0)),
        n @ _ => parse_len_or_percent(n),
    }
}

fn parse_deg(value: &str) -> Result<f32, String> {
    if value.ends_with("deg") {
        return Ok(parse_f32(&value[0..value.len() - 3])?);
    } else {
        return Err(format!("parse_deg err: {}", value));
    }
}

fn parse_border(value: &str) -> Result<(Dimension, CgColor), String> {
    let mut i = 0;
    let width = parse_unity(iter_by_space(value, &mut i)?)?;
    let color = match iter_by_space(value, &mut i) {
        Ok(r) => parse_color_string(r)?,
        Err(_) => parse_color_string(iter_by_space(value, &mut i)?)?,
    };
    Ok((width, color))
}

fn parse_transform_fun(key: &str, value: &str) -> Result<TransformFunc, String> {
    let r = match key {
        "scale" => {
            let mut r = [1.0, 1.0];
            let mut i = 0;
            for v in value.split(",") {
                if i > 1 {
                    return Err(format!("parse_f32_2 error, value: {:?}", value));
                }
                let v = v.trim();
                if v != "" {
                    r[i] = parse_f32(v)?;
                    i += 1;
                }
            }
            if i == 1 {
                let r0 = r[0];
                r[1] = r0;
            }
            TransformFunc::Scale(r[0], r[1])
        }
        "scaleX" => TransformFunc::ScaleX(parse_f32(value)?),
        "scaleY" => TransformFunc::ScaleY(parse_f32(value)?),
        "translate" => {
            let r = parse_len_or_percent_2(value, ",")?;
            match (r[0], r[1]) {
                (LengthUnit::Percent(r), LengthUnit::Percent(r1)) => {
                    TransformFunc::TranslatePercent(r, r1)
                }
                (LengthUnit::Pixel(r), LengthUnit::Pixel(r1)) => TransformFunc::Translate(r, r1),
                _ => {
                    return Err(format!(
                        "parse_transform_fun error, key: {}, value: {}",
                        key, value
                    ))
                }
            }
        }
        "translateX" => {
            let r = parse_len_or_percent(value)?;
            match r {
                LengthUnit::Percent(r) => TransformFunc::TranslateXPercent(r),
                LengthUnit::Pixel(r) => TransformFunc::TranslateX(r),
            }
        }
        "translateY" => {
            let r = parse_len_or_percent(value)?;
            match r {
                LengthUnit::Percent(r) => TransformFunc::TranslateYPercent(r),
                LengthUnit::Pixel(r) => TransformFunc::TranslateY(r),
            }
        }
        "rotate" | "rotateZ" => TransformFunc::RotateZ(parse_deg(value)?),
        _ => {
            return Err(format!(
                "parse_transform_fun error, key: {}, value: {}",
                key, value
            ))
        }
    };
    Ok(r)
}

fn iter_by_space<'a, 'b>(value: &'a str, i: &'b mut usize) -> Result<&'a str, String> {
    let value = &value[*i..];
    let first = match value.find(" ") {
        Some(r) => r,
        None => {
            if value.len() == 0 {
                return Err("".to_string());
            } else {
                *i += value.len();
                return Ok(value);
            }
        }
    };
    *i += first;
    let pre = &value[0..first];
    let next = &value[first..];
    let r = next.trim();
    *i += next.len() - r.len();
    Ok(pre)
}

fn iter_by<'a, 'b>(value: &'a str, i: &'b mut usize,s: &[&str]) -> Result<&'a str, String> {
    let value = &value[*i..];

	let mut first = std::usize::MAX;
	for ss in s.iter() {
		first = first.min( match value.find(*ss) {
			Some(r) => r,
			None => continue,
		});
	}

	if first == std::usize::MAX {
		if value.len() == 0 {
			return Err("".to_string());
		} else {
			*i += value.len();
			return Ok(value);
		}
	}

    *i += first;
    let pre = &value[0..first];
    let next = &value[first..];
    let r = next.trim();
    *i += next.len() - r.len();
    Ok(pre)
}

fn iter_fun<'a, 'b>(value: &'a str, i: &'b mut usize) -> Result<(&'a str, &'a str), String> {
    // let value = &value[*i..];
    let len = value.len();
    let mut n = value;
    let mut v = value;
    let mut is_success = false;
    for j in *i..len {
        if &value[j..j + 1] != " " {
            *i = j;
            break;
        }
    }
    for j in *i..len {
        if &value[j..j + 1] == "(" {
            n = &value[*i..j];
            *i = j + 1;
            break;
        }
    }

    for j in *i..len {
        if &value[j..j + 1] == ")" {
            v = &value[*i..j];
            *i = j + 1;
            is_success = true;
            break;
        }
    }
    if is_success == false {
        return Err("iter_fun error".to_string());
    }
    Ok((n, v.trim()))
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

fn parse_color_string(value: &str) -> Result<CgColor, String> {
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
    let color = match value {
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

        "transparent" => rgba(0, 0, 0, 0.0),
        _ => {
            if value.starts_with("#") {
                parse_color_hex(&value[1..])?
            } else if value.starts_with("rgb") {
                match iter_fun(value, &mut 0) {
                    Ok((n, v)) => parse_color_fun(n, v)?,
                    Err(_) => return Err(format!("parse color err: '{}'", value)),
                }
            } else {
                return Err(format!("parse color err: '{}'", value));
            }
        }
	};
    Ok(color)
}

fn next_no_empty(value: &str, mut i: usize) -> usize {
	while i < value.len() {
		let r = &value[i..i+1];
		if r == " " || r == "	" {
			 i+= 1;
		} else {
			return i;
		}
	}
	return i;
}

fn next_empty(value: &str, mut i: usize) -> usize {
	while i < value.len() {
		let r = &value[i..i+1];
		if r == " " || r == "	" {
			return i;
		} else {
			i += 1;
		}
	}
	return i;
}

fn parse_color_string1(value: &str, i: &mut usize) -> Result<CgColor, String> {
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
	let start = next_no_empty(value, *i);
	if start >= value.len() {
		return Err(String::from_str("").unwrap());
	}

	let mut end = next_empty(value, start);

	let value1 = &value[start..end];
    let color = match value1 {
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

        "transparent" => rgba(0, 0, 0, 0.0),
        _ => {
            if value1.starts_with("#") {
               	parse_color_hex(&value[(start+ 1)..])?
            } else if value1.starts_with("rgb") {
				end = start;
                match iter_fun(value, &mut end) {
                    Ok((n, v)) => {
						parse_color_fun(n, v)?
					},
                    Err(_) => return Err(format!("parse color err: '{}'", value)),
                }
            } else {
                return Err(format!("parse color err: '{}'", value));
            }
        }
	};

	*i = end;
    Ok(color)
}

// 上右下左
fn parse_four_f32(value: &str) -> Result<[Dimension; 4], String> {
    let mut i = 0;
    let mut arr = Vec::default();
    loop {
        match iter_by_space(value, &mut i) {
            Ok(r) => {
                arr.push(r);
            }
            Err(_) => break,
        }
    }
    if arr.len() == 0 {
        return Err(format!("parse_four_f32 error: {}", value));
    }

    to_four(arr)
}

fn to_four_f32(arr: &Vec<f32>) -> Result<[f32; 4], String> {
	let r = if arr.len() == 1 {
        let v = arr[0];
        Ok([v, v, v, v])
    } else if arr.len() == 2 {
        let v = arr[0];
        let v1 = arr[1];
        Ok([v, v1, v, v1])
    } else if arr.len() == 3 {
        let v = arr[0];
        let v1 = arr[1];
        let v2 = arr[2];
        Ok([v, v1, v2, v1])
    } else if arr.len() == 4 {
        Ok([
            arr[0],
            arr[1],
            arr[2],
            arr[3],
        ])
    } else {
        Err(format!("to_four_f32 error"))
	};
	r
}

fn to_four(arr: Vec<&str>) -> Result<[Dimension; 4], String> {
    let r = if arr.len() == 1 {
        let v = parse_unity(arr[0])?;
        [v.clone(), v.clone(), v.clone(), v]
    } else if arr.len() == 2 {
        let v = parse_unity(arr[0])?;
        let v1 = parse_unity(arr[1])?;
        [v.clone(), v1.clone(), v, v1]
    } else if arr.len() == 3 {
        let v = parse_unity(arr[0])?;
        let v1 = parse_unity(arr[1])?;
        let v2 = parse_unity(arr[2])?;
        [v, v1.clone(), v2, v1]
    } else if arr.len() == 4 {
        [
            parse_unity(arr[0])?,
            parse_unity(arr[1])?,
            parse_unity(arr[2])?,
            parse_unity(arr[3])?,
        ]
    } else {
        return Err(format!("to_four error"));
    };
    Ok(r)
}

fn parse_unity(value: &str) -> Result<Dimension, String> {
    if value.ends_with("%") {
        let v = match f32::from_str(&value[..value.len() - 1]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Dimension::Percent(v/100.0))
    } else if value == "auto" {
        Ok(Dimension::Auto)
    } else if value.ends_with("px") {
        let v = match f32::from_str(&value[0..value.len() - 2]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Dimension::Points(v))
    } else {
        // 如果value不符合css规范，直接解析成0px（css默认值）
        Ok(Dimension::Points(0.0))
    }
}

fn parse_len_or_percent_2(value: &str, split: &str) -> Result<[LengthUnit; 2], String> {
    let mut r = [LengthUnit::Pixel(0.0), LengthUnit::Pixel(0.0)];
    let mut i = 0;
    for v in value.split(split) {
        if i > 1 {
            return Err(format!("parse_f32_2 error, value: {:?}", value));
        }
        let v = v.trim();
        if v != "" {
            r[i] = parse_len_or_percent(v)?;
            i += 1;
        }
    }
    Ok(r)
}

fn parse_percent_to_f32(value: &str) -> Result<f32, String> {
	if value.ends_with("%") {
        let v = match f32::from_str(&value[..value.len() - 1]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(v / 100.0)
    } else {
		Err("parse_len_or_percent error".to_string())
	}
}

fn parse_len_or_percent(value: &str) -> Result<LengthUnit, String> {
    if value.ends_with("%") {
        let v = match f32::from_str(&value[..value.len() - 1]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(LengthUnit::Percent(v / 100.0))
    } else if value.ends_with("px") {
        let v = match f32::from_str(&value[0..value.len() - 2]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(LengthUnit::Pixel(v))
    } else {
        Err("parse_unity error".to_string())
    }
}

fn parse_px(value: &str) -> Result<f32, String> {
    if value.ends_with("px") {
        let v = match f32::from_str(&value[0..value.len() - 2]) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        Ok(v)
    } else {
        Err("parse_unity error".to_string())
    }
}

// fn parse_usize(value: &str) -> Result<usize, String> {
//     match usize::from_str(value) {
//         Ok(r) => Ok(r),
//         Err(e) => Err(e.to_string()),
//     }
// }

fn parse_f32(value: &str) -> Result<f32, String> {
    match f32::from_str(value) {
        Ok(r) => Ok(r),
        Err(e) => Err(format!("{:?}: {}", e.to_string(), value)),
    }
}

fn parse_url(value: &str) -> Result<Atom, String> {
    if value.len() < 7 {
        //"url()" 就有5个字符
        return Err(format!("parse_url error, {}", value));
    }
    let value = value[4..value.len() - 1].trim();
    if value.len() > 2
        && ((value.starts_with("'") && value.ends_with("'"))
            || (value.starts_with("\"") && value.ends_with("\"")))
    {
        Ok(Atom::from(&value[1..value.len() - 1]))
    } else {
        Ok(Atom::from(&value[1..value.len() - 1]))
    }
}

fn parse_object_fit(value: &str) -> Result<FitType, String> {
    let r = match value {
        "contain" => FitType::Contain,
        "cover" => FitType::Cover,
        "fill" => FitType::Fill,
        "none" => FitType::None,
        "scale-down" => FitType::ScaleDown,
        _ => return Err(format!("parse_object_fit error, value: {:?}", value)),
    };
    Ok(r)
}

fn parse_border_image_repeat(value: &str) -> Result<BorderImageRepeatOption, String> {
    let r = match value {
        "stretch" => BorderImageRepeatOption::Stretch,
        "repeat" => BorderImageRepeatOption::Repeat,
        "round" => BorderImageRepeatOption::Round,
        "space" => BorderImageRepeatOption::Space,
        _ => {
            return Err(format!(
                "parse_border_image_repeat error, value: {:?}",
                value
            ))
        }
    };
    Ok(r)
}

fn parse_color_fun(key: &str, value: &str) -> Result<CgColor, String> {
    let r = match key {
        "rgba" => {
            let r = parse_f32_4(value, ",")?;
            rgba(r[0] as u8, r[1] as u8, r[2] as u8, r[3])
        }
        "rgb" => {
            let r = parse_f32_3(value, ",")?;
            rgb(r[0] as u8, r[1] as u8, r[2] as u8)
        }
        _ => {
            return Err(format!(
                "parse_color_fun error, key: {}, value: {}",
                key, value
            ))
        }
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

fn rgb(red: u8, green: u8, blue: u8) -> CgColor {
    CgColor::new(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}

fn from_hex(c: u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err("".to_string()),
    }
}

#[test]
pub fn test() {
	let s = ".1{
		background: linear-gradient(180deg, #fff, #000);
	}";
	parse_class_map_from_string(s).unwrap();

	// .t_s_12181a_3{
	// 	text-shadow: 0px 3px 3px #12181a;
	//   }
	//   .t_s_bb7be5_3{
	// 	text-shadow: 0px 3px 3px #bb7be5;
	//   }
	//   .t_s_7031e4_3{
	// 	text-shadow: 0px 3px 3px #7031e4;
	//   }
	let s = " 0px 3px 3px #7031e4";
	let r = parse_text_shadow(s);
	println!("r============{:?}", r);


	// let s = "hsi(100, -1,-2) blur(3px)";
	// let mut class = Class::default();
	// let _r = parse_filter(s, &mut class);
	// println!("test filter============{:?}", class);
}
