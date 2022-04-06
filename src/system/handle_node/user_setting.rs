use std::intrinsics::transmute;

use bitvec::array::BitArray;
use pi_ecs::{prelude::{Write, Query, ResMut, EntityCommands, Res}, entity::Entity};
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_null::Null;
use pi_slotmap_tree::{InsertType, Storage};

use crate::{components::{user::{Node, Size, Margin, Padding, Position, Border, MinMax, FlexContainer, FlexNormal, ZIndex, Overflow, Opacity, BlendMode, Transform, Show, BackgroundColor, BorderColor, BackgroundImage, MaskImage, MaskImageClip, Hsi, Blur, ObjectFit, BackgroundImageClip, BorderImage, BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderRadius, BoxShadow, TextContent, TextStyle, ClassName}, calc::{StyleMark}}, utils::style::{style_sheet::{ClassSheet, StyleReader, StyleQuery, StyleAttr}}, resource::{UserCommands, NodeCommand}};

pub fn cal_matrix(
	entitys: Query<Node, Entity>,

	size: Query<Node, Write<Size>>,
	margin: Query<Node, Write<Margin>>,
	padding: Query<Node, Write<Padding>>,
	border: Query<Node, Write<Border>>,
	position: Query<Node, Write<Position>>,
	min_max: Query<Node, Write<MinMax>>,
	flex_container: Query<Node, Write<FlexContainer>>,
	flex_normal: Query<Node, Write<FlexNormal>>,
	z_index: Query<Node, Write<ZIndex>>,
	overflow: Query<Node, Write<Overflow>>,
	opacity: Query<Node, Write<Opacity>>,
	blend_mode: Query<Node, Write<BlendMode>>,
	show: Query<Node, Write<Show>>,
	transform: Query<Node, Write<Transform>>,
	background_color: Query<Node, Write<BackgroundColor>>,
	border_color: Query<Node, Write<BorderColor>>,
	background_image: Query<Node, Write<BackgroundImage>>,
	background_image_clip: Query<Node, Write<BackgroundImageClip>>,
	mask_image: Query<Node, Write<MaskImage>>,
	mask_image_clip: Query<Node, Write<MaskImageClip>>,
	hsi: Query<Node, Write<Hsi>>,
	blur: Query<Node, Write<Blur>>,
	object_fit: Query<Node, Write<ObjectFit>>,
	border_image: Query<Node, Write<BorderImage>>,
	border_image_clip: Query<Node, Write<BorderImageClip>>,
	border_image_slice: Query<Node, Write<BorderImageSlice>>,
	border_image_repeat: Query<Node, Write<BorderImageRepeat>>,
	border_radius: Query<Node, Write<BorderRadius>>,
	box_shadow: Query<Node, Write<BoxShadow>>,
	text_style: Query<Node, Write<TextStyle>>,

	class_sheet: Res<ClassSheet>,

	mut class_query: Query<Node, Write<ClassName>>,

	mut style_mark: Query<Node, &mut StyleMark>, // TODO OrDefaultMut

	mut text_content: Query<Node, Write<TextContent>>,

	mut tree: EntityTreeMut<Node>,

	mut entity_commands: EntityCommands<Node>,
	mut user_commands: ResMut<UserCommands>,
) {
	let mut style_query = StyleQuery{
		size,
		margin,
		padding,
		border,
		position,
		min_max,
		flex_container,
		flex_normal,
		z_index,
		overflow,
		opacity,
		blend_mode,
		show,
		transform,
		background_color,
		border_color,
		background_image,
		background_image_clip,
		mask_image,
		mask_image_clip,
		hsi,
		blur,
		object_fit,
		border_image,
		border_image_clip,
		border_image_slice,
		border_image_repeat,
		border_radius,
		box_shadow,
		text_style,
	};

	// 操作节点(节点的创建、销毁、挂载、删除)
	for c in user_commands.node_commands.drain(..) {
		match c {
			NodeCommand::AppendNode(node, parent) => {
				if entitys.get(node).is_some() {
					tree.insert_child(node, parent, std::usize::MAX);
				}
			},
			NodeCommand::InsertBefore(node, anchor) => {
				if entitys.get(node).is_some() {
					tree.insert_brother(node, anchor, InsertType::Front);
				}
			},
			NodeCommand::RemoveNode(node) => {
				tree.remove(node);
			},
			NodeCommand::DestroyNode(node) => {
				tree.remove(node);
				entity_commands.despawn(node);

				// 删除所有子节点对应的实体
				if let Some(down) = tree.get_down(node) {
					let head = down.head();
					if !head.is_null() {
						for node in tree.recursive_iter(head) {
							entity_commands.despawn(node);
						}
					}
				}
			},
		};
	}

	// 设置style只要节点存在,样式一定能设置成功
	let style_commands = &mut user_commands.style_commands;
	let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
	for (node, start, end) in commands.drain(..) {
		// 不存在实体，不处理
		if entitys.get(node).is_none() {
			continue;
		}

		let mut style_reader = StyleReader::new(style_buffer, start, end);
		let mut s = style_reader.next_type();
		while let Some(ty) = s {
			style_reader.set(ty, &mut style_query, node);
			style_mark.get_unchecked_mut(node).local_style.set(ty as usize, true);
			s = style_reader.next_type();
		}
	}
	unsafe { style_buffer.set_len(0) };

	// 设置文字
	for (node, text) in user_commands.text_commands.drain(..) {
		match text {
			Some(r) => if let Some(mut t) = text_content.get_mut(node) {
				t.write(r);
			},
			None => if let Some(mut t) = text_content.get_mut(node) {
				t.remove();
			}
		}
	}

	// 设置class样式
	for (node, class) in user_commands.class_commands.drain(..) {
		set_class(
			node, 
			&mut style_query, 
			&mut class_query,
			class, 
			&mut style_mark,
			&class_sheet,
		)
	}
}

fn set_class(
	node: Entity, 
	style_query: &mut StyleQuery, 
	class_query: &mut Query<Node, Write<ClassName>>,
	class: ClassName, 
	style_mark: &mut Query<Node, &mut StyleMark>,
	class_sheet: &ClassSheet,
) {
	if let Some(mut component) = class_query.get(node) {
		let style_mark = style_mark.get_unchecked_mut(node);
		let old_class_style_mark = style_mark.class_style; // 旧的class样式
		let local_style_mark = style_mark.local_style;
		let mut new_class_style_mark: BitArray<[u32;3]> = BitArray::new([0, 0, 0]);

		// 设置class样式
		for i in class.iter() {
			if let Some(class) = class_sheet.class_map.get(i) {
				let mut style_reader = StyleReader::new(&class_sheet.style_buffer, class.start, class.end);
				let mut s = style_reader.next_type();
				while let Some(r) = s {
					// 本地样式不存在，才会设置class样式
					if !local_style_mark[r as usize] {
						style_reader.set(r, style_query, node);
					}
					s = style_reader.next_type();
				}
				new_class_style_mark |= class.class_style_mark;
			}
		}

		// 旧的class_style中存在，但新的class_style和local_style中都不存在的样式，需要重置为默认值
		let cur_style_mark = new_class_style_mark | local_style_mark;
		let invalid_style = old_class_style_mark^cur_style_mark&old_class_style_mark;
		for i in invalid_style.iter_ones() {
			StyleAttr::reset(unsafe{transmute(i as u8)}, cur_style_mark, style_query, node);
		}
		component.write(class);
	}
}

// fn set_style(query: &mut StyleQuery, attr: Attribute, entity: Entity) -> Option<StyleType> {
// 	// TODO，默认组件从world上fetch
// 	match attr {
// 		Attribute::PositionType(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_normal.get(entity) {
// 				let component = component.get_mut_or_default(&FlexNormal::default());
// 				component.position_type = v;
// 				return Some(StyleType::PositionType);
// 			}
// 		},
// 		Attribute::FlexShrink(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_normal.get(entity) {
// 				let component = component.get_mut_or_default(&FlexNormal::default());
// 				component.flex_shrink = v;
// 				return Some(StyleType::FlexShrink);
// 			}
// 		},
// 		Attribute::FlexGrow(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_normal.get(entity) {
// 				let component = component.get_mut_or_default(&FlexNormal::default());
// 				component.flex_grow = v;
// 				return Some(StyleType::FlexGrow);
// 			}
// 		},
// 		Attribute::FlexWrap(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_container.get(entity) {
// 				let component = component.get_mut_or_default(&FlexContainer::default());
// 				component.flex_wrap = v;
// 				return Some(StyleType::FlexWrap);
// 			}
// 		},
// 		Attribute::FlexDirection(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_container.get(entity) {
// 				let component = component.get_mut_or_default(&FlexContainer::default());
// 				component.flex_direction = v;
// 				return Some(StyleType::FlexDirection);
// 			}
// 		},
// 		Attribute::AlignContent(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_container.get(entity) {
// 				let component = component.get_mut_or_default(&FlexContainer::default());
// 				component.align_content = v;
// 				return Some(StyleType::AlignContent);
// 			}
// 		},
// 		Attribute::AlignItems(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_container.get(entity) {
// 				let component = component.get_mut_or_default(&FlexContainer::default());
// 				component.align_items = v;
// 				return Some(StyleType::AlignItems);
// 			}
// 		},
// 		Attribute::AlignSelf(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_normal.get(entity) {
// 				let component = component.get_mut_or_default(&FlexNormal::default());
// 				component.align_self = v;
// 				return Some(StyleType::AlignSelf);
// 			}
// 		},
// 		Attribute::FlexBasis(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_normal.get(entity) {
// 				let component = component.get_mut_or_default(&FlexNormal::default());
// 				component.flex_basis = v;
// 				return Some(StyleType::FlexBasis);
// 			}
// 		},
// 		Attribute::JustifyContent(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.flex_container.get(entity) {
// 				let component = component.get_mut_or_default(&FlexContainer::default());
// 				component.justify_content = v;
// 				return Some(StyleType::JustifyContent);
// 			}
// 		},

// 		Attribute::ObjectFit(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.object_fit.get(entity) {
// 				let component = component.get_mut_or_default(&ObjectFit::default());
// 				*component = v;
// 				return Some(StyleType::ObjectFit);
// 			}
// 		},
// 		Attribute::TextAlign(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.text_align = v;
// 				return Some(StyleType::TextAlign);
// 			}
// 		},
// 		Attribute::Color(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.color = v;
// 				return Some(StyleType::Color);
// 			}
// 		},
// 		//SmallVec<[TextShadow;1]>
// 		Attribute::TextShadow(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.text_shadow = v;
// 				return Some(StyleType::TextShadow);
// 			}
// 		},
// 		Attribute::TextStroke(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.text_stroke = v;
// 				return Some(StyleType::TextStroke);
// 			}
// 		},

// 		Attribute::LineHeight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.line_height = v;
// 				return Some(StyleType::LineHeight);
// 			}
// 		},

// 		Attribute::VerticalAlign(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.vertical_align = v;
// 				return Some(StyleType::VerticalAlign);
// 			}
// 		},
// 		Attribute::WhiteSpace(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.white_space = v;
// 				return Some(StyleType::WhiteSpace);
// 			}
// 		},
		
// 		Attribute::LetterSpacing(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.letter_spacing = v;
// 				return Some(StyleType::LetterSpacing);
// 			}
// 		},
// 		Attribute::TextIndent(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.text_indent = v;
// 				return Some(StyleType::TextIndent);
// 			}
// 		},
// 		Attribute::WordSpacing(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.word_spacing = v;
// 				return Some(StyleType::WordSpacing);
// 			}
// 		},

// 		Attribute::FontStyle(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.font_style = v;
// 				return Some(StyleType::FontStyle);
// 			}
// 		},

// 		Attribute::FontSize(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.font_size = v;
// 				return Some(StyleType::FontSize);
// 			}
// 		},

// 		Attribute::FontWeight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.font_weight = v;
// 				return Some(StyleType::FontWeight);
// 			}
// 		},
// 		Attribute::FontFamily(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.text_style.get(entity) {
// 				let component = component.get_mut_or_default(&TextStyle::default());
// 				component.font_family = v;
// 				return Some(StyleType::FontFamily);
// 			}
// 		},
// 		Attribute::Enable(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.show.get(entity) {
// 				let component = component.get_mut_or_default(&Show::default());
// 				component.set_enable(v);
// 				return Some(StyleType::Enable);
// 			}
// 		},
// 		Attribute::Display(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.show.get(entity) {
// 				let component = component.get_mut_or_default(&Show::default());
// 				component.set_display(v);
// 				return Some(StyleType::Display);
// 			}
// 		},

// 		Attribute::Visibility(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.show.get(entity) {
// 				let component = component.get_mut_or_default(&Show::default());
// 				component.set_visibility(v);
// 				return Some(StyleType::Visibility);
// 			}
// 		},
// 		Attribute::Overflow(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.overflow.get(entity) {
// 				let component = component.get_mut_or_default(&Overflow::default());
// 				component.0 = v;
// 				return Some(StyleType::Overflow);
// 			}
// 		},
// 		Attribute::ZIndex(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.z_index.get(entity) {
// 				let component = component.get_mut_or_default(&ZIndex::default());
// 				component.0 = v;
// 				return Some(StyleType::ZIndex);
// 			}
// 		},
// 		Attribute::BackgroundImage(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.background_image.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BackGroundImage);
// 			}
// 		},
// 		Attribute::BorderImage(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_image.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderImage);
// 			}
// 		},

// 		Attribute::Opacity(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.opacity.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::Opacity);
// 			}
// 		},
// 		Attribute::Blur(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.blur.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::Blur);
// 			}
// 		},
		
// 		Attribute::Width(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.size.get(entity) {
// 				let component = component.get_mut_or_default(&Size::default());
// 				component.width = v;
// 				return Some(StyleType::Width);
// 			}
// 		},
// 		Attribute::Height(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.size.get(entity) {
// 				let component = component.get_mut_or_default(&Size::default());
// 				component.height = v;
// 				return Some(StyleType::Height);
// 			}
// 		},
// 		Attribute::MarginLeft(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.margin.get(entity) {
// 				let component = component.get_mut_or_default(&Margin::default());
// 				component.left = v;
// 				return Some(StyleType::MarginLeft);
// 			}
// 		},
// 		Attribute::MarginTop(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.margin.get(entity) {
// 				let component = component.get_mut_or_default(&Margin::default());
// 				component.top = v;
// 				return Some(StyleType::MarginTop);
// 			}
// 		},
// 		Attribute::MarginBottom(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.margin.get(entity) {
// 				let component = component.get_mut_or_default(&Margin::default());
// 				component.bottom = v;
// 				return Some(StyleType::MarginBottom);
// 			}
// 		},
// 		Attribute::MarginRight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.margin.get(entity) {
// 				let component = component.get_mut_or_default(&Margin::default());
// 				component.right = v;
// 				return Some(StyleType::MarginRight);
// 			}
// 		},
// 		Attribute::Margin(v) => {
// 			// 取不到说明实体已经销毁
// 			// if let Some(mut component) = query.margin.get(entity) {
// 			// 	let component = component.get_mut_or_default(&Margin::default());
// 			// 	component.top = v;
// 			// 	component.right = v;
// 			// 	component.bottom = v;
// 			// 	component.left = v;
// 			// 	return Some(StyleType::Margin);
// 			// }
// 		},
// 		Attribute::PaddingLeft(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.padding.get(entity) {
// 				let component = component.get_mut_or_default(&Padding::default());
// 				component.left = v;
// 				return Some(StyleType::PaddingLeft);
// 			}
// 		},
// 		Attribute::PaddingTop(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.padding.get(entity) {
// 				let component = component.get_mut_or_default(&Padding::default());
// 				component.top = v;
// 				return Some(StyleType::PaddingTop);
// 			}
// 		},
// 		Attribute::PaddingBottom(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.padding.get(entity) {
// 				let component = component.get_mut_or_default(&Padding::default());
// 				component.bottom = v;
// 				return Some(StyleType::PaddingBottom);
// 			}
// 		},
// 		Attribute::PaddingRight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.padding.get(entity) {
// 				let component = component.get_mut_or_default(&Padding::default());
// 				component.right = v;
// 				return Some(StyleType::PaddingRight);
// 			}
// 		},
// 		Attribute::Padding(v) => {
// 			// 取不到说明实体已经销毁
// 			// if let Some(mut component) = query.padding.get(entity) {
// 			// 	let component = component.get_mut_or_default(&Padding::default());
// 			// 	component.top = v;
// 			// 	component.right = v;
// 			// 	component.bottom = v;
// 			// 	component.left = v;
// 			// 	return Some(StyleType::Padding);
// 			// }
// 		},
// 		Attribute::BorderLeft(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border.get(entity) {
// 				let component = component.get_mut_or_default(&Border::default());
// 				component.left = v;
// 				return Some(StyleType::BorderLeft);
// 			}
// 		},
// 		Attribute::BorderTop(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border.get(entity) {
// 				let component = component.get_mut_or_default(&Border::default());
// 				component.top = v;
// 				return Some(StyleType::BorderTop);
// 			}
// 		},
// 		Attribute::BorderBottom(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border.get(entity) {
// 				let component = component.get_mut_or_default(&Border::default());
// 				component.bottom = v;
// 				return Some(StyleType::BorderBottom);
// 			}
// 		},
// 		Attribute::BorderRight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border.get(entity) {
// 				let component = component.get_mut_or_default(&Border::default());
// 				component.right = v;
// 				return Some(StyleType::BorderRight);
// 			}
// 		},
// 		Attribute::Border(v) => {
// 			// 取不到说明实体已经销毁
// 			// if let Some(mut component) = query.border.get(entity) {
// 			// 	let component = component.get_mut_or_default(&Border::default());
// 			// 	component.top = v;
// 			// 	component.right = v;
// 			// 	component.bottom = v;
// 			// 	component.left = v;
// 			// 	return Some(StyleType::Border);
// 			// }
// 		},
// 		Attribute::MinWidth(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.min_max.get(entity) {
// 				let component = component.get_mut_or_default(&MinMax::default());
// 				component.min.width = v;
// 				return Some(StyleType::MinWidth);
// 			}
// 		},
// 		Attribute::MinHeight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.min_max.get(entity) {
// 				let component = component.get_mut_or_default(&MinMax::default());
// 				component.min.height = v;
// 				return Some(StyleType::MinHeight);
// 			}
// 		},
// 		Attribute::MaxHeight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.min_max.get(entity) {
// 				let component = component.get_mut_or_default(&MinMax::default());
// 				component.max.height = v;
// 				return Some(StyleType::MaxHeight);
// 			}
// 		},
// 		Attribute::MaxWidth(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.min_max.get(entity) {
// 				let component = component.get_mut_or_default(&MinMax::default());
// 				component.max.width = v;
// 				return Some(StyleType::MaxWidth);
// 			}
// 		},
// 		Attribute::PositionLeft(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.position.get(entity) {
// 				let component = component.get_mut_or_default(&Position::default());
// 				component.left = v;
// 				return Some(StyleType::PositionLeft);
// 			}
// 		},
// 		Attribute::PositionTop(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.position.get(entity) {
// 				let component = component.get_mut_or_default(&Position::default());
// 				component.top = v;
// 				return Some(StyleType::PositionTop);
// 			}
// 		},
// 		Attribute::PositionRight(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.position.get(entity) {
// 				let component = component.get_mut_or_default(&Position::default());
// 				component.right = v;
// 				return Some(StyleType::PositionRight);
// 			}
// 		},
// 		Attribute::PositionBottom(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.position.get(entity) {
// 				let component = component.get_mut_or_default(&Position::default());
// 				component.bottom = v;
// 				return Some(StyleType::PositionBottom);
// 			}
// 		},

// 		Attribute::MaskImage(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.mask_image.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::MaskImage);
// 			}
// 		},
// 		Attribute::BlendMode(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.blend_mode.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BlendMode);
// 			}
// 		},

// 		Attribute::BackgroundColor(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.background_color.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BackgroundColor);
// 			}
// 		},
// 		Attribute::BorderColor(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_color.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderColor);
// 			}
// 		},
// 		Attribute::BoxShadow(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.box_shadow.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BoxShadow);
// 			}
// 		},

// 		Attribute::ImageClip(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.image_clip.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BackGroundImageClip);
// 			}
// 		},

// 		Attribute::BorderImageRepeat(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_image_repeat.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderImageRepeat);
// 			}
// 		},

// 		Attribute::BorderImageClip(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_image_clip.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderImageClip);
// 			}
// 		},
// 		Attribute::BorderImageSlice(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_image_slice.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderImageSlice);
// 			}
// 		},

// 		Attribute::BorderRadius(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.border_radius.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::BorderRadius);
// 			}
// 		},
// 		// Vec<TransformFunc>
// 		Attribute::Transform(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.transform.get(entity) {
// 				let component = component.get_mut_or_default(&Transform::default());
// 				component.funcs = v;
// 				return Some(StyleType::Transform);
// 			}
// 		},
// 		Attribute::TransformOrigin(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.transform.get(entity) {
// 				let component = component.get_mut_or_default(&Transform::default());
// 				component.origin = v;
// 				return Some(StyleType::TransformOrigin);
// 			}
// 		},
// 		Attribute::Hsi(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.hsi.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::Hsi);
// 			}
// 		},

// 		Attribute::MaskImageClip(v) => {
// 			// 取不到说明实体已经销毁
// 			if let Some(mut component) = query.mask_image_clip.get(entity) {
// 				component.write(v);
// 				return Some(StyleType::MaskImageClip);
// 			}
// 		},
// 	}
// 	None
// }

