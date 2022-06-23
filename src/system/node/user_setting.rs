use std::intrinsics::transmute;

use bitvec::array::BitArray;
use pi_ecs::prelude::{Write, Query, ResMut, Res, EntityDelete, Id, Event};
use pi_ecs_macros::{setup, listen};
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_null::Null;
use pi_slotmap_tree::{InsertType, Storage};

use crate::{components::{user::{Node, Size, Margin, Padding, Position, Border, MinMax, FlexContainer, FlexNormal, ZIndex, Overflow, Opacity, BlendMode, Transform, Show, BackgroundColor, BorderColor, BackgroundImage, MaskImage, MaskImageClip, Hsi, Blur, ObjectFit, BackgroundImageClip, BorderImage, BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderRadius, BoxShadow, TextContent, TextStyle, ClassName, TransformWillChange}, calc::{StyleMark, StyleType}}, utils::style::{style_sheet::{ClassSheet, StyleTypeReader, StyleQuery, StyleAttr}}, resource::{UserCommands, NodeCommand}};

pub struct CalcUserSetting;

#[setup]
impl CalcUserSetting {

	#[system]
	pub fn user_setting(
		entitys: Query<Node, Id<Node>>,
	
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
		transform_will_change: Query<Node, Write<TransformWillChange>>,
	
		class_sheet: Res<ClassSheet>,
	
		mut class_query: Query<Node, Write<ClassName>>,
	
		mut style_mark: Query<Node, &mut StyleMark>, // TODO OrDefaultMut
	
		text_content: Query<Node, Write<TextContent>>,
	
		mut tree: EntityTreeMut<Node>,
	
		mut entity_delete: EntityDelete<Node>,
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
			transform_will_change,
			text_content,
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
					entity_delete.despawn(node);
	
					// 删除所有子节点对应的实体
					if let Some(down) = tree.get_down(node) {
						let head = down.head();
						if !head.is_null() {
							for node in tree.recursive_iter(head) {
								entity_delete.despawn(node);
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
	
			let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
			while let Some(ty) = style_reader.write_to_component(node, &mut style_query) {
				style_mark.get_unchecked_mut(node).local_style.set(ty as usize, true);
			}
		}
		unsafe { style_buffer.set_len(0) };
	
		// // 设置文字
		// for (node, text) in user_commands.text_commands.drain(..) {
		// 	match text {
		// 		Some(r) => if let Some(mut t) = text_content.get_mut(node) {
		// 			t.write(r);
		// 		},
		// 		None => if let Some(mut t) = text_content.get_mut(node) {
		// 			t.remove();
		// 		}
		// 	}
		// }
	
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

	#[listen(entity=(Node, Create))]
	pub fn prepare_data(
		e: Event,
		mut query: Query<Node, Write<StyleMark>>
	) {
		query.get_unchecked_mut_by_entity(e.id).write(StyleMark::default());
	}
}


fn set_class(
	node: Id<Node>, 
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
				let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
				let is_write = |ty: StyleType| {
					// 本地样式不存在，才会设置class样式
					!local_style_mark[ty as usize]
				};
				while style_reader.or_write_to_component(node, style_query, is_write).is_some() {
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
