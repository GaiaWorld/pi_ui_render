use std::intrinsics::transmute;

use bitvec::array::BitArray;
use pi_ecs::{prelude::{Write, Query, ResMut, Res, EntityDelete, Id, Event, DefaultComponent, OrDefault}, storage::Offset};
use pi_ecs_macros::{setup, listen};
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_flex_layout::style::Dimension;
use pi_null::Null;
use pi_slotmap_tree::{InsertType, Storage};

use crate::{
	components::{
		user::{Node, Size, Margin, Padding, Position, Border, MinMax, FlexContainer, FlexNormal, ZIndex, Overflow, Opacity, BlendMode, Transform, Show, BackgroundColor, BorderColor, BackgroundImage, MaskImage, MaskImageClip, Hsi, Blur, ObjectFit, BackgroundImageClip, BorderImage, BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderRadius, BoxShadow, TextContent, TextStyle, ClassName, TransformWillChange}, 
		calc::{StyleMark, StyleType, NodeState, BackgroundImageTexture}
	}, 
	utils::style::{
		style_sheet::{ClassSheet, StyleTypeReader, StyleQuery, StyleAttr}
	}, 
	resource::{UserCommands, NodeCommand, DefaultStyle},
};

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
		node_state: Query<Node, Write<NodeState>>,
	
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
			node_state,
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
				log::error!("node is not exist: {:?}", node);
				continue;
			}
			let mut style_mark_item = style_mark.get_unchecked_mut(node);
			
			let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
			let mut style_mark = &mut style_mark_item.local_style;
			while style_reader.write_to_component(style_mark, node, &mut style_query) {
			}
			// 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
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
			if unsafe {transmute::<_, u64>(node)} == 4294967330 {
				println!("XXXXXXXXXXX");
			}
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

	// 设置图片节点的默认大小
	#[listen(component=(Node, BackgroundImageTexture, (Create, Modify)), component=(Node, BackgroundImageClip, (Create, Modify, Delete)))]
	pub fn set_image_default_size(
		e: Event,
		query: Query<Node, (Write<Size>, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark)>
	){
		if let Some((mut size_item, texture, clip, style_mark)) = query.get_mut_by_entity(e.id) {
			let size = size_item.get_mut_or_default();
			let mut is_change = false;
			// 本地样式和class样式都未设置宽度，设置默认图片宽度
			if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
				size.width = Dimension::Points(texture.width as f32 * (clip.maxs.x - clip.mins.x));
				is_change = true;
			}

			// 本地样式和class样式都未设置高度，设置默认图片高度
			if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
				size.height = Dimension::Points(texture.height as f32 * (clip.maxs.y - clip.mins.y));
				is_change = true;
			}

			if is_change {
				size_item.notify_modify();
			}
		}
		
		
	}

	// 取消图片的默认大小（BackgroundImageTexture删除后，不在使用默认size）
	#[listen(component=(Node, BackgroundImageTexture, Delete))]
	pub fn cancel_image_default_size(
		e: Event,
		mut query: Query<Node, (Write<Size>, &StyleMark)>
	){
		let (mut size_item, style_mark) = query.get_unchecked_mut_by_entity(e.id);
		
		let size = size_item.get_mut_or_default();
		let mut is_change = false;

		// 本地样式和class样式都未设置宽度，设置默认图片宽度
		if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
			size.width = Dimension::Undefined;
			is_change = true;
		}

		// 本地样式和class样式都未设置高度，设置默认图片高度
		if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
			size.height = Dimension::Undefined;
			is_change = true;
		}

		if is_change {
			size_item.notify_modify();
		}
	}

	#[listen(resource=(DefaultStyle, (Create, Modify, Delete)))]
	pub fn default_style_change<'a>(
		_e: Event,
		size: ResMut<'a, DefaultComponent<Size>>,
		margin: ResMut<'a, DefaultComponent<Margin>>,
		padding: ResMut<'a, DefaultComponent<Padding>>,
		border: ResMut<'a, DefaultComponent<Border>>,
		position: ResMut<'a, DefaultComponent<Position>>,
		min_max: ResMut<'a, DefaultComponent<MinMax>>,
		flex_container: ResMut<'a, DefaultComponent<FlexContainer>>,
		flex_normal: ResMut<'a, DefaultComponent<FlexNormal>>,
		z_index: ResMut<'a, DefaultComponent<ZIndex>>,
		overflow: ResMut<'a, DefaultComponent<Overflow>>,
		opacity: ResMut<'a, DefaultComponent<Opacity>>,
		blend_mode: ResMut<'a, DefaultComponent<BlendMode>>,
		show: ResMut<'a, DefaultComponent<Show>>,
		transform: ResMut<'a, DefaultComponent<Transform>>,
		background_color: ResMut<'a, DefaultComponent<BackgroundColor>>,
		border_color: ResMut<'a, DefaultComponent<BorderColor>>,
		background_image: ResMut<'a, DefaultComponent<BackgroundImage>>,
		background_image_clip: ResMut<'a, DefaultComponent<BackgroundImageClip>>,
		mask_image: ResMut<'a, DefaultComponent<MaskImage>>,
		mask_image_clip: ResMut<'a, DefaultComponent<MaskImageClip>>,
		hsi: ResMut<'a, DefaultComponent<Hsi>>,
		blur: ResMut<'a, DefaultComponent<Blur>>,
		object_fit: ResMut<'a, DefaultComponent<ObjectFit>>,
		border_image: ResMut<'a, DefaultComponent<BorderImage>>,
		border_image_clip: ResMut<'a, DefaultComponent<BorderImageClip>>,
		border_image_slice: ResMut<'a, DefaultComponent<BorderImageSlice>>,
		border_image_repeat: ResMut<'a, DefaultComponent<BorderImageRepeat>>,
		border_radius: ResMut<'a, DefaultComponent<BorderRadius>>,
		box_shadow: ResMut<'a, DefaultComponent<BoxShadow>>,
		text_style: ResMut<'a, DefaultComponent<TextStyle>>,
		transform_will_change: ResMut<'a, DefaultComponent<TransformWillChange>>,
		text_content: ResMut<'a, DefaultComponent<TextContent>>,
	
		class_sheet: Res<'a, ClassSheet>,
		// default_style_mark: ResMut<DefaultStyleMark>,
	) {
		let mut style_query = crate::utils::style::style_sheet::DefaultStyle{
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
		// let old_class_style_mark = default_style_mark.0; // 旧的class样式
		// let mut new_class_style_mark: BitArray<[u32;3]> = BitArray::new([0, 0, 0]);

		// 设置默认样式
		if let Some(class) = class_sheet.class_map.get(&0) { // key为0的样式为默认样式
			let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
			while style_reader.write_to_default(&mut style_query).is_some() {
			}
			// new_class_style_mark |= class.class_style_mark;
		}

		// 是否需要将
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
		let mut style_mark = style_mark.get_unchecked_mut(node);
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
				while style_reader.or_write_to_component(&mut new_class_style_mark, node, style_query, is_write).is_some() {
				}
				// new_class_style_mark |= class.class_style_mark;
			}
		}

		// 旧的class_style中存在，但新的class_style和local_style中都不存在的样式，需要重置为默认值
		let mut cur_style_mark = new_class_style_mark | local_style_mark;
		let invalid_style = old_class_style_mark^cur_style_mark&old_class_style_mark;
		let buffer = Vec::new();
		for i in invalid_style.iter_ones() {
			StyleAttr::reset(&mut cur_style_mark, unsafe{transmute(i as u8)}, &buffer, 0, style_query, node);
		}

		style_mark.class_style = new_class_style_mark;
		component.write(class);
	}
}
