use std::intrinsics::transmute;

use bitvec::array::BitArray;
use pi_ecs::prelude::{DefaultComponent, EntityDelete, Event, Id, OrDefault, Query, Res, ResMut, Write};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::EntityTreeMut;
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
            Opacity, Overflow, Padding, Position, Show, Size, TextContent, TextStyle, Transform, TransformWillChange, ZIndex, ClearColor, Viewport, serialize::{StyleQuery, StyleTypeReader, StyleAttr}, RenderTargetType, Canvas,
        }, draw_obj::DrawObject
    },
    resource::{animation_sheet::KeyFramesSheet, DefaultStyle, NodeCommand, StyleCommands, UserCommands, TimeInfo},
    utils::cmd::DataQuery,
};
use pi_style::{
    style::{Animation, BackgroundImageMod, BorderImageRepeat},
    style_type::{ClassSheet},
};

pub struct CalcUserSetting;

#[setup]
impl CalcUserSetting {
    #[system]
    pub fn user_setting(
        size: Query<'static, 'static, Node, Write<Size>>,
        margin: Query<'static, 'static, Node, Write<Margin>>,
        padding: Query<'static, 'static, Node, Write<Padding>>,
        border: Query<'static, 'static, Node, Write<Border>>,
        position: Query<'static, 'static, Node, Write<Position>>,
        min_max: Query<'static, 'static, Node, Write<MinMax>>,
        flex_container: Query<'static, 'static, Node, Write<FlexContainer>>,
        flex_normal: Query<'static, 'static, Node, Write<FlexNormal>>,
        z_index: Query<'static, 'static, Node, Write<ZIndex>>,
        overflow: Query<'static, 'static, Node, Write<Overflow>>,
        opacity: Query<'static, 'static, Node, Write<Opacity>>,
        blend_mode: Query<'static, 'static, Node, Write<BlendMode>>,
        show: Query<'static, 'static, Node, Write<Show>>,
        transform: Query<'static, 'static, Node, Write<Transform>>,
        background_color: Query<'static, 'static, Node, Write<BackgroundColor>>,
        border_color: Query<'static, 'static, Node, Write<BorderColor>>,
        background_image: Query<'static, 'static, Node, Write<BackgroundImage>>,
        background_image_clip: Query<'static, 'static, Node, Write<BackgroundImageClip>>,
        background_image_mod: Query<'static, 'static, Node, Write<BackgroundImageMod>>,
        mask_image: Query<'static, 'static, Node, Write<MaskImage>>,
        mask_image_clip: Query<'static, 'static, Node, Write<MaskImageClip>>,
        hsi: Query<'static, 'static, Node, Write<Hsi>>,
        blur: Query<'static, 'static, Node, Write<Blur>>,
        border_image: Query<'static, 'static, Node, Write<BorderImage>>,
        border_image_clip: Query<'static, 'static, Node, Write<BorderImageClip>>,
        border_image_slice: Query<'static, 'static, Node, Write<BorderImageSlice>>,
        border_image_repeat: Query<'static, 'static, Node, Write<BorderImageRepeat>>,
        border_radius: Query<'static, 'static, Node, Write<BorderRadius>>,
        box_shadow: Query<'static, 'static, Node, Write<BoxShadow>>,
        text_style: Query<'static, 'static, Node, Write<TextStyle>>,
        transform_will_change: Query<'static, 'static, Node, Write<TransformWillChange>>,
        node_state: Query<'static, 'static, Node, Write<NodeState>>,
        text_content: Query<'static, 'static, Node, Write<TextContent>>,
        mut animation: Query<'static, 'static, Node, Write<Animation>>,
		view_port: Query<'static, 'static, Node, Write<Viewport>>,
		clear_color: Query<'static, 'static, Node, Write<ClearColor>>,
		render_target_type: Query<'static, 'static, Node, Write<RenderTargetType>>,
		canvas: Query<'static, 'static, Node, Write<Canvas>>,

		mut draw_list: Query<'static, 'static, Node, &'static mut DrawList>,

        class_sheet: ResMut<'static, ClassSheet>,
        keyframes_sheet: ResMut<'static, KeyFramesSheet>,

        mut class_query: Query<'static, 'static, Node, Write<ClassName>>,

        entitys: Query<'static, 'static, Node, Id<Node>>,
        mut style_mark: Query<'static, 'static, Node, &mut StyleMark>, // TODO OrDefaultMut

        mut tree: EntityTreeMut<Node>,

        mut entity_delete: EntityDelete<Node>,
		mut draw_obj_delete: EntityDelete<DrawObject>,

        mut user_commands: ResMut<UserCommands>,
		mut time_info: ResMut<TimeInfo>,
    ) {
		let time = Instant::now();
		*time_info = TimeInfo {
			cur_time: time,
			delta: (time - time_info.cur_time).as_millis() as u64,
		};
        let mut style_query = StyleQuery {
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
            background_image_mod,
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
            animation: &mut animation,
        };
        let mut data_query = DataQuery {
            clear_color,
            view_port,
			render_target_type,
			canvas,
            class_sheet,
            keyframes_sheet,
        };

        for c in user_commands.css_commands.drain(..) {
            data_query.class_sheet.extend_from_class_sheet(c);
        }

        // 先作用other_commands（通常是修改单例， 如动画表，css表）
        user_commands.other_commands.apply(&mut data_query);

        // 操作节点(节点的创建、销毁、挂载、删除)
        for c in user_commands.node_commands.drain(..) {
            match c {
                NodeCommand::AppendNode(node, parent) => {
                    if entitys.get(node).is_some() {
                        tree.insert_child(node, parent, std::usize::MAX);
                    }
                }
                NodeCommand::InsertBefore(node, anchor) => {
                    if entitys.get(node).is_some() {
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
                        if !head.is_null() {
                            for node in tree.recursive_iter(head) {
								delete_draw_list(node, &mut draw_list, &mut draw_obj_delete);
								// log::warn!("DestroyNode================={:?}", node);
                                entity_delete.despawn(node);
                            }
                        }
                    }

					delete_draw_list(node, &mut draw_list, &mut draw_obj_delete);
					// log::warn!("DestroyNode================={:?}", node);
                    entity_delete.despawn(node);
                }
            };
        }

        // 设置style只要节点存在,样式一定能设置成功
        set_style(&mut user_commands.style_commands, &mut style_query, &entitys, &mut style_mark);

        // 设置class样式
        for (node, class) in user_commands.class_commands.drain(..) {
            set_class(node, &mut style_query, &mut class_query, class, &mut style_mark, &data_query.class_sheet)
        }
    }

    #[listen(entity=(Node, Create))]
    pub fn prepare_data(e: Event, mut query: Query<Node, Write<StyleMark>>) { 
		query.get_unchecked_mut_by_entity(e.id).write(StyleMark::default()); 
	}

    // 设置图片节点的默认大小
    #[listen(component=(Node, BackgroundImageTexture, (Create, Modify)), component=(Node, BackgroundImageClip, (Create, Modify, Delete)))]
    pub fn set_image_default_size(e: Event, query: Query<Node, (Write<Size>, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark)>) {
        if let Some((mut size_item, texture, clip, style_mark)) = query.get_mut_by_entity(e.id) {
            let size = size_item.get_mut_or_default();
            let mut is_change = false;
            // 本地样式和class样式都未设置宽度，设置默认图片宽度
            if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
                size.width = Dimension::Points(texture.width as f32 * (*clip.right - *clip.left));
                is_change = true;
            }

            // 本地样式和class样式都未设置高度，设置默认图片高度
            if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
                size.height = Dimension::Points(texture.height as f32 * (*clip.bottom - *clip.top));
                is_change = true;
            }

            if is_change {
                size_item.notify_modify();
            }
        }
    }

    // 取消图片的默认大小（BackgroundImageTexture删除后，不在使用默认size）
    #[listen(component=(Node, BackgroundImageTexture, Delete))]
    pub fn cancel_image_default_size(e: Event, mut query: Query<Node, (Write<Size>, &StyleMark)>) {
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
        background_image_mod: ResMut<'a, DefaultComponent<BackgroundImageMod>>,
        mask_image: ResMut<'a, DefaultComponent<MaskImage>>,
        mask_image_clip: ResMut<'a, DefaultComponent<MaskImageClip>>,
        hsi: ResMut<'a, DefaultComponent<Hsi>>,
        blur: ResMut<'a, DefaultComponent<Blur>>,
        border_image: ResMut<'a, DefaultComponent<BorderImage>>,
        border_image_clip: ResMut<'a, DefaultComponent<BorderImageClip>>,
        border_image_slice: ResMut<'a, DefaultComponent<BorderImageSlice>>,
        border_image_repeat: ResMut<'a, DefaultComponent<BorderImageRepeat>>,
        border_radius: ResMut<'a, DefaultComponent<BorderRadius>>,
        box_shadow: ResMut<'a, DefaultComponent<BoxShadow>>,
        text_style: ResMut<'a, DefaultComponent<TextStyle>>,
        transform_will_change: ResMut<'a, DefaultComponent<TransformWillChange>>,
        text_content: ResMut<'a, DefaultComponent<TextContent>>,
        animation: ResMut<'a, DefaultComponent<Animation>>,

        class_sheet: Res<'a, ClassSheet>,
        // default_style_mark: ResMut<DefaultStyleMark>,
    ) {
        let mut style_query = crate::components::user::serialize::DefaultStyle {
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
            background_image_mod,
            border_image,
            border_image_clip,
            border_image_slice,
            border_image_repeat,
            border_radius,
            box_shadow,
            text_style,
            transform_will_change,
            text_content,
            animation,
        };
        // let old_class_style_mark = default_style_mark.0; // 旧的class样式
        // let mut new_class_style_mark: BitArray<[u32;3]> = BitArray::new([0, 0, 0]);

        // 设置默认样式
        if let Some(class) = class_sheet.class_map.get(&0) {
            // key为0的样式为默认样式
            let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
            while style_reader.write_to_default(&mut style_query).is_some() {}
            // new_class_style_mark |= class.class_style_mark;
        }

        // 是否需要将
    }
}

pub fn set_style(
    style_commands: &mut StyleCommands,
    style_query: &mut StyleQuery,
    entitys: &Query<'static, 'static, Node, Id<Node>>,
    style_mark: &mut Query<'static, 'static, Node, &mut StyleMark>,
) {
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end) in commands.drain(..) {
        // 不存在实体，不处理
        if entitys.get(node).is_none() {
            log::error!("node is not exist: {:?}", node);
            continue;
        }

        let mut style_mark_item = style_mark.get_unchecked_mut(node);

        let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
        let style_mark = &mut style_mark_item.local_style;
        while style_reader.write_to_component(style_mark, node, style_query) {}
        // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
    }
    unsafe { style_buffer.set_len(0) };
}


fn set_class(
    node: Id<Node>,
    style_query: &mut StyleQuery,
    class_query: &mut Query<Node, Write<ClassName>>,
    class: ClassName,
    style_mark: &mut Query<Node, &mut StyleMark>,
    class_sheet: &ClassSheet,
) {
    if let (Some(mut component), Some(mut style_mark)) = (class_query.get(node), style_mark.get_mut(node)) {
        let old_class_style_mark = style_mark.class_style; // 旧的class样式
        let local_style_mark = style_mark.local_style;
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

        style_mark.class_style = new_class_style_mark;
        component.write(class);
    }
}


fn delete_draw_list(id: Id<Node>, draw_list: &mut Query<'static, 'static, Node, &'static mut DrawList>,
draw_objects: &mut EntityDelete<DrawObject>) {
	if let Some(mut list) = draw_list.get(id) {
		for i in list.iter() {
			draw_objects.despawn(i.clone())
		}
		list.clear()
	}
}
