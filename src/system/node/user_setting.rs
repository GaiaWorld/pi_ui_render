//! 每个实体必须写入StyleMark组件
use std::intrinsics::transmute;

use bevy::{prelude::{Query, ResMut, Entity, World, Commands, Local, Res, EventReader, Changed, EventWriter, RemovedComponents}, ecs::system::{SystemState, Command}};
use bitvec::array::BitArray;
use pi_ecs_utils::{prelude::{EntityTreeMut, OrDefault}, system_param::{layer_dirty::ComponentEvent, tree::TreeKey}};
use pi_flex_layout::style::Dimension;
use pi_null::Null;
use pi_slotmap_tree::{InsertType, Storage};
use pi_time::Instant;

use crate::{
    components::{
        calc::{BackgroundImageTexture, NodeState, StyleMark, StyleType, DrawList},
        user::{
            BackgroundColor, BackgroundImage, BackgroundImageClip, BlendMode, Blur, Border, BorderColor, BorderImage, BorderImageClip,
            BorderImageSlice, BorderRadius, BoxShadow, ClassName, FlexContainer, FlexNormal, Hsi, Margin, MaskImage, MaskImageClip, MinMax, Node,
            Opacity, Overflow, Padding, Position, Show, Size, TextContent, TextStyle, Transform, TransformWillChange, ZIndex, ClearColor, Viewport, serialize::{StyleQuery, StyleTypeReader, StyleAttr, Setting, set_style_attr_or_default, get_component_mut}, RenderTargetType, Canvas, BorderImageRepeat,
        }, draw_obj::DrawObject
    },
    resource::{animation_sheet::KeyFramesSheet, DefaultStyle, NodeCommand, StyleCommands, UserCommands, TimeInfo, MuexUserCommands},
    utils::cmd::DataQuery,
};
use pi_style::{
    style::{Animation, BackgroundImageMod},
};
use crate::resource::ClassSheet;

pub fn user_setting(
	world: &mut World,

	state: &mut SystemState<(
		ResMut<UserCommands>,
		ResMut<ClassSheet>,
		ResMut<KeyFramesSheet>,
		ResMut<TimeInfo>,
		Query<&mut ClassName>,
		Query<&DrawList>,
		// Query<&mut ClearColor>,
		// Query<&mut Viewport>,
		// Query<&mut RenderTargetType>,
		// Query<&mut Canvas>,
		Query<Entity>,
		EntityTreeMut,

	)>,
	mut style_query: Local<StyleQuery>,
	
	// view_port: Query<Node, &'static mut Viewport>,
	// clear_color: Query<Node, &'static mut ClearColor>,
	// render_target_type: Query<Node, &'static mut RenderTargetType>,
	// canvas: Query<Node, &'static mut Canvas>,

	// mut draw_list: Query<Node, &'static mut DrawList>,

	// class_sheet: ResMut<'static, ClassSheet>,
	// keyframes_sheet: ResMut<'static, KeyFramesSheet>,

	// mut class_query: Query<Node, &'static mut ClassName>,

	// entitys: Query<Node, Entity>,
	// mut style_mark: Query<Node, &'static mut StyleMark>, // TODO OrDefaultMut

	// mut tree: EntityTreeMut,

	// mut entity_delete: EntityDelete<Node>,
	// mut draw_obj_delete: EntityDelete<DrawObject>,

	// mut user_commands: ResMut<UserCommands>,
	// mut time_info: ResMut<TimeInfo>,
) {
	let time = Instant::now();
	let (
		mut user_commands,
		mut class_sheet,
		mut keyframes_sheet,
		mut time_info,
		mut class_query,
		draw_list,
		// clear_color,
		// view_port,
		// render_target_type,
		// canvas,
		entitys,
		
		mut tree,
	) = state.get_mut(world);

	*time_info = TimeInfo {
		cur_time: time,
		delta: (time - time_info.cur_time).as_millis() as u64,
	};

	let mut user_commands = std::mem::replace(&mut *user_commands, UserCommands::default());
	let class_sheet = std::mem::replace(&mut *class_sheet, ClassSheet::default());
	
	// let mut data_query = DataQuery {
	// 	clear_color,
	// 	view_port,
	// 	render_target_type,
	// 	canvas,
	// 	class_sheet,
	// 	keyframes_sheet,
	// };

	// for c in user_commands.css_commands.drain(..) {
	// 	class_sheet.extend_from_class_sheet(c);
	// }

	// 先作用other_commands（通常是修改单例， 如动画表，css表）
	// user_commands.other_commands.apply(&mut data_query);
	
	let mut destroy_node_list = Vec::new();
	// 操作节点(节点的创建、销毁、挂载、删除)
	for c in user_commands.node_commands.drain(..) {
		match c {
			NodeCommand::AppendNode(node, parent) => {
				if entitys.get(node).is_ok() {
					tree.insert_child(node, parent, std::usize::MAX);
				}
			}
			NodeCommand::InsertBefore(node, anchor) => {
				if entitys.get(node).is_ok() {
					tree.insert_brother(node, anchor, InsertType::Front);
				}
			}
			NodeCommand::RemoveNode(node) => {
				// log::warn!("RemoveNode================={:?}", node);
				tree.remove(node);
			}
			NodeCommand::DestroyNode(node) => {
				tree.remove(node);
				
				// 删除所有子节点对应的实体
				if let Some(down) = tree.get_down(node) {
					let head = down.head();
					if !TreeKey(head).is_null() {
						for node in tree.recursive_iter(head) {
							delete_draw_list(node, &draw_list, &mut destroy_node_list);
						}
					}
				}

				delete_draw_list(node, &draw_list, &mut destroy_node_list);
			}
		};
	}

	let mut setting = Setting {
		style: &style_query,
		world,
	};

	// 设置style只要节点存在,样式一定能设置成功
	set_style(&mut user_commands.style_commands, &mut setting);

	// 设置class样式
	for (node, class) in user_commands.class_commands.drain(..) {
		set_class(node, &mut setting, class, &class_sheet);
	}

	let (
		mut user_commands1,
		mut class_sheet1,
		mut _key_frames_sheet,
		mut _time_info,
		mut  _class_query,
		draw_list,
		_entitys,
		_tree,
	) = state.get_mut(world);
	*user_commands1 = user_commands;
	*class_sheet1 = class_sheet;
}


pub fn set_image_default_size(
	mut event_reader: EventReader<ComponentEvent<Changed<BackgroundImageTexture>>>,
	mut event_writer: EventWriter<ComponentEvent<Changed<Size>>>,
	mut query: Query<(&mut Size, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark)>,

	removed_components: RemovedComponents<BackgroundImageTexture>,
) {
	for removed in removed_components.iter() {
		if let Ok((mut size, texture, clip, style_mark)) = query.get_mut(removed) {
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
				event_writer.send(ComponentEvent::new(removed));
			}
		}
	}

	for event in event_reader.iter() {
		if let Ok((mut size, texture, clip, style_mark)) = query.get_mut(event.id) {
			let mut is_change = false;
			// 本地样式和class样式都未设置宽度，设置默认图片宽度
			if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
				size.width = Dimension::Points(texture.width as f32 * (clip.right - clip.left));
				is_change = true;
			}

			// 本地样式和class样式都未设置高度，设置默认图片高度
			if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
				size.height = Dimension::Points(texture.height as f32 * (clip.bottom - clip.top));
				is_change = true;
			}

			if is_change {
				event_writer.send(ComponentEvent::new(event.id));
			}
		}
	}
	
	
}

// #[setup]
// impl CalcUserSetting {

//     // 设置图片节点的默认大小
//     #[listen(component=(Node, BackgroundImageTexture, (Create, Modify)), component=(Node, BackgroundImageClip, (Create, Modify, Delete)))]
//     pub fn set_image_default_size(e: Event, query: Query<Node, (Write<Size>, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark)>) {
//         if let Some((mut size_item, texture, clip, style_mark)) = query.get_mut_by_entity(e.id) {
            
//         }
//     }

//     // 取消图片的默认大小（BackgroundImageTexture删除后，不在使用默认size）
//     #[listen(component=(Node, BackgroundImageTexture, Delete))]
//     pub fn cancel_image_default_size(e: Event, mut query: Query<Node, (Write<Size>, &StyleMark)>) {
//         let (mut size_item, style_mark) = query.get_unchecked_mut_by_entity(e.id);

//         let size = size_item.get_mut_or_default();
//         let mut is_change = false;

//         // 本地样式和class样式都未设置宽度，设置默认图片宽度
//         if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
//             size.width = Dimension::Undefined;
//             is_change = true;
//         }

//         // 本地样式和class样式都未设置高度，设置默认图片高度
//         if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
//             size.height = Dimension::Undefined;
//             is_change = true;
//         }

//         if is_change {
//             size_item.notify_modify();
//         }
//     }

//     #[listen(resource=(DefaultStyle, (Create, Modify, Delete)))]
//     pub fn default_style_change<'a>(
//         _e: Event,
//         size: ResMut<'a, DefaultComponent<Size>>,
//         margin: ResMut<'a, DefaultComponent<Margin>>,
//         padding: ResMut<'a, DefaultComponent<Padding>>,
//         border: ResMut<'a, DefaultComponent<Border>>,
//         position: ResMut<'a, DefaultComponent<Position>>,
//         min_max: ResMut<'a, DefaultComponent<MinMax>>,
//         flex_container: ResMut<'a, DefaultComponent<FlexContainer>>,
//         flex_normal: ResMut<'a, DefaultComponent<FlexNormal>>,
//         z_index: ResMut<'a, DefaultComponent<ZIndex>>,
//         overflow: ResMut<'a, DefaultComponent<Overflow>>,
//         opacity: ResMut<'a, DefaultComponent<Opacity>>,
//         blend_mode: ResMut<'a, DefaultComponent<BlendMode>>,
//         show: ResMut<'a, DefaultComponent<Show>>,
//         transform: ResMut<'a, DefaultComponent<Transform>>,
//         background_color: ResMut<'a, DefaultComponent<BackgroundColor>>,
//         border_color: ResMut<'a, DefaultComponent<BorderColor>>,
//         background_image: ResMut<'a, DefaultComponent<BackgroundImage>>,
//         background_image_clip: ResMut<'a, DefaultComponent<BackgroundImageClip>>,
//         background_image_mod: ResMut<'a, DefaultComponent<BackgroundImageMod>>,
//         mask_image: ResMut<'a, DefaultComponent<MaskImage>>,
//         mask_image_clip: ResMut<'a, DefaultComponent<MaskImageClip>>,
//         hsi: ResMut<'a, DefaultComponent<Hsi>>,
//         blur: ResMut<'a, DefaultComponent<Blur>>,
//         border_image: ResMut<'a, DefaultComponent<BorderImage>>,
//         border_image_clip: ResMut<'a, DefaultComponent<BorderImageClip>>,
//         border_image_slice: ResMut<'a, DefaultComponent<BorderImageSlice>>,
//         border_image_repeat: ResMut<'a, DefaultComponent<BorderImageRepeat>>,
//         border_radius: ResMut<'a, DefaultComponent<BorderRadius>>,
//         box_shadow: ResMut<'a, DefaultComponent<BoxShadow>>,
//         text_style: ResMut<'a, DefaultComponent<TextStyle>>,
//         transform_will_change: ResMut<'a, DefaultComponent<TransformWillChange>>,
//         text_content: ResMut<'a, DefaultComponent<TextContent>>,
//         animation: ResMut<'a, DefaultComponent<Animation>>,

//         class_sheet: Res<'a, ClassSheet>,
//         // default_style_mark: ResMut<DefaultStyleMark>,
//     ) {
//         let mut style_query = crate::components::user::serialize::DefaultStyle {
//             size,
//             margin,
//             padding,
//             border,
//             position,
//             min_max,
//             flex_container,
//             flex_normal,
//             z_index,
//             overflow,
//             opacity,
//             blend_mode,
//             show,
//             transform,
//             background_color,
//             border_color,
//             background_image,
//             background_image_clip,
//             mask_image,
//             mask_image_clip,
//             hsi,
//             blur,
//             background_image_mod,
//             border_image,
//             border_image_clip,
//             border_image_slice,
//             border_image_repeat,
//             border_radius,
//             box_shadow,
//             text_style,
//             transform_will_change,
//             text_content,
//             animation,
//         };
//         // let old_class_style_mark = default_style_mark.0; // 旧的class样式
//         // let mut new_class_style_mark: BitArray<[u32;3]> = BitArray::new([0, 0, 0]);

//         // 设置默认样式
//         if let Some(class) = class_sheet.class_map.get(&0) {
//             // key为0的样式为默认样式
//             let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
//             while style_reader.write_to_default(&mut style_query).is_some() {}
//             // new_class_style_mark |= class.class_style_mark;
//         }

//         // 是否需要将
//     }
// }

pub fn set_style(
    style_commands: &mut StyleCommands,
    style_query: &mut Setting,
) {
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end) in commands.drain(..) {
        // 不存在实体，不处理
        if style_query.world.get_entity(node).is_none() {
            log::error!("node is not exist: {:?}", node);
            continue;
        }

        let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
		let mut local_mark = BitArray::new([0, 0, 0]);
        while style_reader.write_to_component(&mut local_mark, node, style_query) {}

		set_style_attr_or_default(&mut style_query.world, node, style_query.style.style_mark, local_mark, |style_mark: &mut StyleMark, v| {
			style_mark.local_style |= v;
		});
		
        // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
    }
    unsafe { style_buffer.set_len(0) };
}


fn set_class(
    node: Entity,
    style_query: &mut Setting,
    class: ClassName,
    class_sheet: &ClassSheet,
) {
	if style_query.world.get_entity(node).is_none() {
		log::error!("node is not exist: {:?}", node);
		return;
	}
	let style_mark = unsafe { get_component_mut::<StyleMark>(&mut style_query.world, node, style_query.style.style_mark) };
	let (old_class_style_mark, local_style_mark) = (style_mark.class_style.clone(), style_mark.local_style.clone());
	let mut new_class_style_mark: BitArray<[u32; 3]> = BitArray::new([0, 0, 0]);

	// 设置class样式
	for i in class.iter() {
		if *i == 1100330128 {
			log::warn!("zzz:{:?}", node);
		}
		if let Some(class) = class_sheet.class_map.get(i) {
			// println!("set class1==========={}", i);
			let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
			let is_write = |ty: StyleType| {
				// if local_style_mark[ty as usize] {
				// 	log::warn!("!==========={:?}", ty);
				// }
				// 本地样式不存在，才会设置class样式
				!local_style_mark[ty as usize]
			};
			while style_reader
				.or_write_to_component(&mut new_class_style_mark, node, style_query, is_write)
				.is_some()
			{}
			// new_class_style_mark |= class.class_style_mark;
		}
	}

	// 旧的class_style中存在，但新的class_style和local_style中都不存在的样式，需要重置为默认值
	let mut cur_style_mark = new_class_style_mark | local_style_mark;
	let invalid_style = old_class_style_mark ^ cur_style_mark & old_class_style_mark;
	let buffer = Vec::new();
	for i in invalid_style.iter_ones() {
		StyleAttr::reset(&mut cur_style_mark, unsafe { transmute(i as u8) }, &buffer, 0, style_query, node);
	}

	set_style_attr_or_default(&mut style_query.world, node, style_query.style.style_mark, new_class_style_mark, |item: &mut StyleMark, v| {
		item.class_style |= v;
	});

	set_style_attr_or_default(&mut style_query.world, node, style_query.style.class_name, class, |item: &mut ClassName, v| {
		*item = v;
	});
}


fn delete_draw_list(id: Entity, draw_list: &Query<&DrawList>, draw_objects: &mut Vec<Entity>) {
	draw_objects.push(id);
	if let Ok(list) = draw_list.get(id) {
		for i in list.iter() {
			draw_objects.push(i.clone());
		}
	}
}
