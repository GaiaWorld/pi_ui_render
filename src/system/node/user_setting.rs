//! 每个实体必须写入StyleMark组件
use std::intrinsics::transmute;

use bevy::{ecs::{
    prelude::{Changed, Entity, EventReader, Local, Query, RemovedComponents, ResMut, World},
    system::SystemState,
}, prelude::With};
use bitvec::array::BitArray;
use pi_bevy_ecs_extend::{
    prelude::{EntityTreeMut, OrDefault},
    system_param::{layer_dirty::ComponentEvent, tree::TreeKey},
};
use pi_flex_layout::style::Dimension;
use pi_null::Null;
use pi_slotmap_tree::InsertType;
use pi_time::Instant;

use crate::{resource::{ClassSheet, QuadTree}, components::{calc::{EntityKey}, user::{Viewport, RenderDirty}}};
use crate::{
    components::{
        calc::{BackgroundImageTexture, DrawList, StyleMark, StyleType},
        user::{
            serialize::{get_component_mut, set_style_attr_or_default, Setting, StyleAttr, StyleQuery, StyleTypeReader},
            BackgroundImageClip, ClassName, Size,
        },
    },
    resource::{NodeCommand, StyleCommands, TimeInfo, UserCommands},
};

/// 处理用户设置属性，将其设置到组件上
pub fn user_setting(
    world: &mut World,

    commands: &mut SystemState<(ResMut<UserCommands>, ResMut<ClassSheet>)>,

    state: &mut SystemState<(Query<&DrawList>, Query<Entity>, EntityTreeMut)>,
	quad_delete: &mut SystemState<(ResMut<QuadTree>, Query<Entity, With<Viewport>>)>,
    style_query: Local<StyleQuery>,
	mut destroy_entity_list: Local<Vec<Entity>>, // 需要销毁的实体列表作为本地变量，避免每次重新分配内存
) {
	
    

    let (mut user_commands, mut _class_sheet) = commands.get_mut(world);
    // let (class_commands_len, style_commands_len, node_len) = (user_commands.class_commands.len(), user_commands.style_commands.commands.len(), user_commands.node_commands.len());
    let mut user_commands = std::mem::replace(&mut *user_commands, UserCommands::default());
    // let class_sheet = std::mem::replace(&mut *class_sheet, ClassSheet::default());

    // 先作用other_commands（通常是修改单例， 如动画表，css表）
    user_commands.other_commands.apply(world);

    let (draw_list, entitys, mut tree) = state.get_mut(world);

   


    // 操作节点(节点的创建、销毁、挂载、删除)
    for c in user_commands.node_commands.drain(..) {
        match c {
            NodeCommand::AppendNode(node, parent) => {
                if entitys.get(node).is_ok() {
					log::debug!("AppendNode node====================node： {:?}, parent： {:?}", node, parent);
                    tree.insert_child(node, parent, std::usize::MAX);
                }
            }
            NodeCommand::InsertBefore(node, anchor) => {
                if entitys.get(node).is_ok() {
                    log::debug!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    tree.insert_brother(node, anchor, InsertType::Front);
                }
            }
            NodeCommand::RemoveNode(node) => {
                tree.remove(node);
            }
            NodeCommand::DestroyNode(node) => {
                // 删除所有子节点对应的实体
                if let Some(down) = tree.get_down(node) {
                    let head = down.head();
                    if !TreeKey(head).is_null() {
                        for node in tree.recursive_iter(head) {
                            delete_draw_list(node, &draw_list, &mut destroy_entity_list);
                        }
                    }
                }
				tree.remove(node);
                delete_draw_list(node, &draw_list, &mut destroy_entity_list);
            }
        };
    }

	// 删除需要销毁的实体
	if destroy_entity_list.len() > 0 {
		// let mut quad_tree = quad_tree.0.get_mut(world);
		// for entity in destroy_entity_list.iter() {
		// 	if let Some(r) = quad_tree.remove(EntityKey(*entity)) {
		// 		// 删除时需要发送该事件， 以便后续计算脏区域
		// 		// event_writer.send(OldQuad { entity: *entity, quad: Quad(r.0) });
		// 		// 设置全局脏
		// 	}
		// }
		// Query<(&RootDirtyRect, OrDefault<RenderDirty>, &Viewport)>,

		// 删除实体
		for entity in destroy_entity_list.iter() {
			world.despawn(*entity);
		}

		// 删除包围盒
		let (mut quad_tree, roots) = quad_delete.get_mut(world);
		for entity in destroy_entity_list.iter() {
			quad_tree.remove(EntityKey(*entity));
		}

		destroy_entity_list.clear();
		// 设置所有的root渲染脏（节点删除后， 组件被删除，很多状态丢失， 除非立即处理脏区域）
		for r in roots.iter().collect::<Vec<Entity>>() {
			world.entity_mut(r).insert(RenderDirty(true));
		}
	}

    let mut setting = Setting { style: &style_query, world };

    // 设置style只要节点存在,样式一定能设置成功
    set_style(&mut user_commands.style_commands, &mut setting);

    let (_, mut class_sheet) = commands.get_mut(world);
    let class_sheet = std::mem::replace(&mut *class_sheet, ClassSheet::default());
    let mut setting = Setting { style: &style_query, world };
    // 设置class样式
    for (node, class) in user_commands.class_commands.drain(..) {
        set_class(node, &mut setting, class, &class_sheet);
    }

    let (mut user_commands1, mut class_sheet1) = commands.get_mut(world);
    *user_commands1 = user_commands;
    *class_sheet1 = class_sheet;

    // 指令需要手动apply
    state.apply(world);

    // log::warn!("new time=============={:?}, {}, {}, {}", std::time::Instant::now() - tt, class_commands_len, style_commands_len, node_len);
}


/// 处理图片纹理加载成功，为没设置Size的节点设置默认的Size组件（与图片宽高相同）
/// 处理图片纹理删除， 如果实体依然存在，并且用户未设置Size组件， 则设置实体的Size为Undefined
pub fn set_image_default_size(
    mut event_reader: EventReader<ComponentEvent<Changed<BackgroundImageTexture>>>,
    mut query: Query<(&mut Size, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark)>,

    mut removed_components: RemovedComponents<BackgroundImageTexture>,
) {
    // 处理删除的图片纹理
    for removed in removed_components.iter() {
        if let Ok((mut size, _texture, _clip, style_mark)) = query.get_mut(removed) {
            // 本地样式和class样式都未设置宽度，设置默认图片宽度
            if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
                size.width = Dimension::Undefined;
            }

            // 本地样式和class样式都未设置高度，设置默认图片高度
            if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
                size.height = Dimension::Undefined;
            }
        }
    }

    // 处理增加的图片问题
    for event in event_reader.iter() {
        if let Ok((mut size, texture, clip, style_mark)) = query.get_mut(event.id) {
			
            // 本地样式和class样式都未设置宽度，设置默认图片宽度
            if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
                size.width = Dimension::Points(texture.width as f32 * *(clip.right - clip.left));
            }

            // 本地样式和class样式都未设置高度，设置默认图片高度
            if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
                size.height = Dimension::Points(texture.height as f32 * *(clip.bottom - clip.top));
            }
        }
    }
}

pub fn set_style(style_commands: &mut StyleCommands, style_query: &mut Setting) {
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end) in commands.drain(..) {
        // 不存在实体，不处理
        if style_query.world.get_entity(node).is_none() {
            log::debug!("node is not exist: {:?}", node);
            continue;
        }

        let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
        let mut local_mark = BitArray::new([0, 0, 0]);
        while style_reader.write_to_component(&mut local_mark, node, style_query) {}

        set_style_attr_or_default(
            &mut style_query.world,
            node,
            style_query.style.style_mark,
            local_mark,
            |style_mark: &mut StyleMark, v| {
                style_mark.local_style |= v;
            },
        );

        // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
    }
    unsafe { style_buffer.set_len(0) };
}


fn set_class(node: Entity, style_query: &mut Setting, class: ClassName, class_sheet: &ClassSheet) {
    if style_query.world.get_entity(node).is_none() {
        log::error!("node is not exist: {:?}", node);
        return;
    }
    let style_mark = unsafe { get_component_mut::<StyleMark>(&mut style_query.world, node, style_query.style.style_mark) };
    let (old_class_style_mark, local_style_mark) = (style_mark.class_style.clone(), style_mark.local_style.clone());
    let mut new_class_style_mark: BitArray<[u32; 3]> = BitArray::new([0, 0, 0]);

    // 设置class样式
    for i in class.iter() {
        if let Some(class) = class_sheet.class_map.get(i) {
            // println!("set class1==========={}", i);
            let mut style_reader = StyleTypeReader::new(&class_sheet.style_buffer, class.start, class.end);
            let is_write = |ty: StyleType| {
                // if !local_style_mark[ty as usize] {
                // 	count.fetch_add(1, Ordering::Relaxed);
                // }
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
        // count.fetch_add(1, Ordering::Relaxed);
        StyleAttr::reset(&mut cur_style_mark, unsafe { transmute(i as u8) }, &buffer, 0, style_query, node);
    }

    set_style_attr_or_default(
        &mut style_query.world,
        node,
        style_query.style.style_mark,
        new_class_style_mark,
        |item: &mut StyleMark, v| {
            item.class_style |= v;
        },
    );

    set_style_attr_or_default(
        &mut style_query.world,
        node,
        style_query.style.class_name,
        class,
        |item: &mut ClassName, v| {
            *item = v;
        },
    );
}

fn delete_draw_list(id: Entity, draw_list: &Query<&DrawList>, draw_objects: &mut Vec<Entity>) {
    draw_objects.push(id);
    if let Ok(list) = draw_list.get(id) {
        for (i, _) in list.iter() {
            draw_objects.push(*i);
        }
    }
}
