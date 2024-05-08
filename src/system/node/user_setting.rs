//! 每个实体必须写入StyleMark组件
use std::intrinsics::transmute;

use pi_world::prelude::{Changed, World, Alter, With, Query, SingleResMut, Entity, ParamSet, Local, Or};
use pi_bevy_ecs_extend::{prelude::{EntityTreeMut, OrInitSingleRes, OrInitSingleResMut}, system_param::tree::TreeKey};

use bitvec::array::BitArray;
use pi_map::Map;
use pi_null::Null;
use pi_slotmap_tree::InsertType;

use crate::{
    components::{
        calc::{EntityKey, StyleMarkType}, draw_obj::DrawState, user::{serialize::DefaultStyle, RenderDirty, Size, Viewport, ZIndex}, NodeBundle
    }, resource::{
        fragment::{FragmentMap, NodeTag}, ClassSheet, NodeChanged, QuadTree
    }, system::draw_obj::calc_text::IsRun,
};
use crate::{
    components::{
        calc::{DrawList, StyleMark, StyleType},
        user::{
            serialize::{Setting, StyleAttr, StyleQuery, StyleTypeReader},
            ClassName,
        },
    },
    resource::{NodeCommand, StyleCommands, UserCommands},
};

/// 处理用户设置的指令，将其作用到组件上（包含添加子节点、设置样式、设置class、设置视口等）
pub fn user_setting1(
    world: &mut World,
    mut id: Local<usize>,
) {
    if *id == 0 {
        *id = world.or_register_single_res::<UserCommands>();
    }
    let user_commands = world.index_single_res_mut::<UserCommands>(*id).unwrap().0;
    let mut other_commands = std::mem::replace(&mut user_commands.other_commands, pi_world::prelude::CommandQueue::default());
    other_commands.apply(world);

    let user_commands = world.index_single_res_mut::<UserCommands>(*id).unwrap().0;
    user_commands.other_commands = other_commands;

}

/// 处理用户设置的指令，将其作用到组件上（包含添加子节点、设置样式、设置class、设置视口等）
pub fn user_setting2(
    // world: &mut World,

    // mut query: ParamSet<(
    //     &mut World,
    //     (
    //         StyleQuery,
    //         SingleResMut<UserCommands>,
    //         SingleResMut<ClassSheet>,
    //         OrInitSingleResMut<FragmentMap>,
    //         Query<Entity, With<Size>>,
    //         EntityTreeMut,
    //         SingleResMut<QuadTree>,
    //         Query<Entity, With<Viewport>>,
    //         OrInitSingleResMut<NodeChanged>,
    //         OrInitSingleRes<IsRun>,
    //     )
    // )>,

    mut query: ParamSet<(
        (
            StyleQuery,
            EntityTreeMut,
            Query<&mut RenderDirty, With<Viewport>>,
            Query<Entity, With<Size>>,

            Query<&DrawList>,

            Alter<(), Or<(With<Size>, With<DrawState>)>>,
            Query<&mut StyleMark>,
        ),
        // Query<(), With<Size>>, // TODO, 应该提供一个Entitys参数， 用于判断实体是否存在， 而不需要捕获原型
        Alter<(), (), NodeBundle>,
        Alter<(), (), (NodeBundle, ZIndex,)>,
    )>,
    mut default_style: DefaultStyle,

    // entitys: Query<(), With<Size>>,
    // insert_bundle: Insert<NodeBundle>,
    // insert_bundle1: Insert<(NodeBundle, ZIndex,)>,

    // tree: EntityTreeMut,
    // roots: Query<Entity, With<Viewport>>,
    // entitys: Query<Entity, With<Size>>,
    // all: Query<(), With<Size>>,

    mut user_commands: SingleResMut<UserCommands>,
    class_sheet: SingleResMut<ClassSheet>,
    fragments: OrInitSingleResMut<FragmentMap>, 
    mut quad_tree: SingleResMut<QuadTree>,
    mut node_changed: OrInitSingleResMut<NodeChanged>,
    mut destroy_entity_list: Local<Vec<Entity>>, // 需要销毁的实体列表作为本地变量，避免每次重新分配内存
    r: OrInitSingleRes<IsRun>,

) {
    // let (mut user_commands, _class_sheet, _fragments, r, events) = commands.get_mut(world);
	if r.0 {
		return;
	}

	// 此处强制转换是安全的， 本system逻辑保证， events访问不会读写冲突， 且生命周期足够
	// let events: EventWriter<'static, StyleChange> = unsafe { transmute(events) };
	// let mut dirty_list = StyleDirtyList {
	// 	list: events,
	// 	mark: &mut *dirty_mark,
	// };

    // let (class_commands_len, style_commands_len, node_len) = (user_commands.class_commands.len(), user_commands.style_commands.commands.len(), user_commands.node_commands.len());
    // let mut user_commands = std::mem::replace(&mut *user_commands, UserCommands::default());
    // let class_sheet = std::mem::replace(&mut *class_sheet, ClassSheet::default());

    // 先作用other_commands（通常是修改单例， 如动画表，css表）
    // user_commands.other_commands.apply(world);

    // let (_user_commands, mut class_sheet, mut fragments, _,  _) = commands.get_mut(world);
    // let fragments = std::mem::replace(&mut **fragments, FragmentMap::default());
    // let class_sheet = std::mem::replace(&mut *class_sheet, ClassSheet::default());

	// if user_commands.node_init_commands.len() > 0 || user_commands.fragment_commands.len() > 0 || user_commands.node_commands.len() > 0{
	// 	log::warn!("insert entity====================node_list: {:?}, \n{:?}, \n{:?}, \n: {:?}", user_commands.version, &user_commands.node_init_commands, &user_commands.fragment_commands, &mut user_commands.node_commands);
	// }

	user_commands.version += 1;

	let mut is_node_change = user_commands.node_init_commands.len() > 0 || user_commands.fragment_commands.len() > 0 || user_commands.node_commands.len() > 0;

	// 初始化节点, 插入bundle
	for (node, tag) in user_commands.node_init_commands.drain(..) {
		if query.p0().0.entitys.contains(node) {
			let mut bundle = NodeBundle::default();
			if tag == NodeTag::VNode {
				bundle.node_state.set_vnode(true);
				log::debug!("insert NodeBundle, {:?}", node);
				let _ = query.p2().alter(node, (bundle, ZIndex(-1))); // vnode节点，Zindex应该为auto
				continue;
			}
			log::debug!("insert NodeBundle, {:?}", node);
			let _ = query.p1().alter(node, bundle);
		} else {
			log::error!("insert NodeBundle fail, entity is not exist, {:?}, {:?}", node, tag);
		}
	}

    // 插入bundle
    for c in user_commands.fragment_commands.iter() {
        // 组织模板的节点关系
        let t = match fragments.map.get(&c.key) {
            Some(r) => r,
            _ => {
                log::warn!("fragment is not exist, {}", c.key);
                continue;
            }
        };
        log::debug!("fragment_commands === {}", c.key);
        debug_assert_eq!(t.end - t.start, c.entitys.len());
		if t.end - t.start != c.entitys.len() {
			panic!("fragment_commands === {}, {}, {}", c.key, t.end - t.start, c.entitys.len());
		}

        for i in t.clone() {
            let n = &fragments.fragments[i];
            let node = &c.entitys[i - t.start];
            if query.p1().contains(*node) {
                let mut bundle = NodeBundle::default();
                if n.tag == NodeTag::VNode {
                    bundle.node_state.set_vnode(true);
					log::debug!("insert NodeBundle for fragment , {:?}", node);
					let _ = query.p2().alter(*node, (bundle, ZIndex(-1))); // vnode节点，Zindex应该为auto
					continue;
                }
                log::debug!("insert NodeBundle for fragment , {:?}", node);
                let _ = query.p1().alter(*node, bundle);
            } else {
				log::error!("insert NodeBundle fail, fragment entity is not exist, {:?}, {:?}", node, n.tag);
			}
        }
    }

    // let (draw_list, entitys, mut tree, mut r) = state.get_mut(world);

    for c in user_commands.fragment_commands.iter() {
        // 组织模板的节点关系
        let t = match fragments.map.get(&c.key) {
            Some(r) => r,
            _ => {
                log::info!("fragment is not exist, {}", c.key);
                continue;
            }
        };
        debug_assert_eq!(t.end - t.start, c.entitys.len());

        for i in t.clone() {
            let n = &fragments.fragments[i];
            let node = &c.entitys[i - t.start];
            log::debug!(
                "fragment_commands insertChild!!====================node：{:?}, parent {:?}",
                node,
                n.parent
            );
            if let (false, true) = (n.parent.is_null(), query.p1().get(*node).is_ok()) {
                log::debug!(
                    "fragment_commands insertChild====================node：{:?}, parent {:?}",
                    node,
                    c.entitys[n.parent]
                );
                // log::warn!("fragment_commands insertChild====================node：{:?}, parent {:?}", node, c.entitys[n.parent]);
                query.p0().1.insert_child(*node, c.entitys[n.parent], std::usize::MAX);
            }
        }
    }

    // 操作节点(节点的销毁、挂载、删除)
    for c in user_commands.node_commands.drain(..) {
        match c {
            NodeCommand::AppendNode(node, parent) => {
                if query.p1().contains(node) {
					// if !EntityKey( parent ).is_null() && draw_list.get(node).is_err() {
					// 	log::warn!("AppendNode parent error============={:?}, {:?}", parent, unsafe{transmute::<_, f64>(parent.to_bits())});
					// 	r.0 = true;
					// 	return;
					// }
					// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
					// 	log::warn!("AppendNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
					// 	r.0 = true;
					// 	return;
					// }
					
                    log::debug!("AppendNode node====================node： {:?}, parent： {:?}", node, parent);
                    // log::warn!("AppendNode node====================node： {:?}, parent： {:?}", node, parent);
                    query.p0().1.insert_child(node, parent, std::usize::MAX);
                }
            }
            NodeCommand::InsertBefore(node, anchor) => {
				// if !EntityKey( anchor ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("InsertBefore anchor error============={:?}, {:?}", anchor, unsafe{transmute::<_, f64>(anchor.to_bits())});
				// 	r.0 = true;
				// 	return;
				// }
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("InsertBefore node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 	return;
				// }

                if query.p1().contains(node) {
                    log::debug!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    // log::warn!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    query.p0().1.insert_brother(node, anchor, InsertType::Front);
                }
            }
            NodeCommand::RemoveNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("RemoveNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }


				log::debug!("RemoveNode node====================node={node:?}");
                query.p0().1.remove(node);
            }
            NodeCommand::DestroyNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("DestroyNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }
				log::debug!("DestroyNode node====================node={node:?}");
                // 删除所有子节点对应的实体
                if let Some(down) = query.p0().1.get_down(node) {
                    let head = down.head();
                    if !TreeKey(head).is_null() {
                        let p0 = query.p0();
                        for node in p0.1.recursive_iter(head) {
                            delete_draw_list(node, &p0.4, &mut destroy_entity_list);
                        }
                    }
                }
                query.p0().1.remove(node);
                delete_draw_list(node, &query.p0().4, &mut destroy_entity_list);
            }
        };
    }

	
	is_node_change = is_node_change || destroy_entity_list.len() > 0;
	if is_node_change {
        node_changed.0 = true;
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

		// log::warn!("DestroyNode entity====================node_list: {:?}", destroy_entity_list);
        // 删除实体
        for entity in destroy_entity_list.iter() {
            let _ = query.p1().destroy(*entity);
        }

        // 删除包围盒
        for entity in destroy_entity_list.iter() {
            quad_tree.remove(&EntityKey(*entity));
        }

        destroy_entity_list.clear();
        // 设置所有的root渲染脏（节点删除后， 组件被删除，很多状态丢失， 除非立即处理脏区域）
        for mut render_dirty in query.p0().2.iter_mut() {
            render_dirty.0 = true;
        }
    }

    let p0 = query.p0();
    let mut setting = Setting { style: &mut p0.0, default_value: &mut default_style };

    // 设置模板节点的style
    for c in user_commands.fragment_commands.drain(..) {
        let t = match fragments.map.get(&c.key) {
            Some(r) => r,
            _ => {
                continue;
            }
        };
        debug_assert_eq!(t.end - t.start, c.entitys.len());

        for i in t.clone() {
            let n = &fragments.fragments[i];
            let node = c.entitys[i - t.start];
            if n.style_meta.end > n.style_meta.start {
                set_style(node, n.style_meta.start, n.style_meta.end, &fragments.style_buffer, &mut setting, &mut p0.6,  true);
            }
            if n.class.len() > 0 {
                set_class(node, &mut setting, &mut p0.6, n.class.clone(), &class_sheet);
            }
        }
    }


    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, &mut p0.6);
    // 设置class样式
    for (node, class) in user_commands.class_commands.drain(..) {
        set_class(node, &mut setting, &mut p0.6, class, &class_sheet);
    }
	// // 清理标记（该标记用于将本次修改样式的操作合并成一个事件）
	// dirty_list.clear_mark();

    // 指令需要手动apply
    // state.apply(world);

    // log::warn!("new time=============={:?}, {}, {}, {}", std::time::Instant::now() - tt, class_commands_len, style_commands_len, node_len);
}

/// 清理StyleMark上的脏标记
pub fn clear_dirty_mark(mut style_mark: Query<&mut StyleMark, Changed<StyleMark>>) {
	for mut r in style_mark.iter_mut() {
		r.bypass_change_detection().dirty_style = Default::default();
	}
}

// pub struct StyleDirtyList<'s> {
// 	// pub list: EventWriter<'w, StyleChange>,
// 	pub mark: &'s mut bitvec::vec::BitVec<usize>,
// }

// impl<'s, 'w> StyleDirtyList<'s, 'w> {
// 	/// 标记脏
// 	pub fn mark_dirty(&mut self, entity: Entity) {
// 		let index = entity.index() as usize;
// 		if self.mark.len() <= index {
// 			let count = (index - self.mark.len()) / std::mem::size_of::<usize>()  + 1;
// 			for _ in 0..count {
// 				self.mark.extend(Some(0));
// 			}
// 		}

// 		if !self.mark[index] {
// 			self.list.send(StyleChange(entity));
// 		}
// 	}

// 	/// 清理标记
// 	pub fn clear_mark(&mut self) {
// 		self.mark.clear();
// 	}
// }

// #[derive(Debug, Copy, Clone, Deref)]
// pub struct StyleChange(pub Entity);

pub fn set_styles<'w, 's>(style_commands: &mut StyleCommands, style_query: &mut Setting, style_mark: &mut Query<'w, &'static mut StyleMark>,) {
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end) in commands.drain(..) {
        set_style(node, start, end, style_buffer, style_query, style_mark, false);
    }
    unsafe { style_buffer.set_len(0) };
}

pub fn set_style<'w, 's>(node: Entity, start: usize, end: usize, style_buffer: &Vec<u8>, style_query: &mut Setting, style_mark: &mut Query<'w, &'static mut StyleMark>, is_clone: bool) {
    // 不存在实体，不处理
    if !style_query.style.entitys.contains(node) {
        log::debug!("node is not exist: {:?}", node);
        return;
    }
	log::trace!("set_style==========={:?}", node);

    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    let mut local_mark = BitArray::new([0, 0, 0, 0]);
    while style_reader.write_to_component(&mut local_mark, node, style_query, is_clone) {}

    if let Ok(mut style_mark) = style_mark.get_mut(node) {
        style_mark.local_style |= local_mark;
		style_mark.dirty_style |= local_mark;
    };
    // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
}


fn set_class<'w, 's>(node: Entity, style_query: &mut Setting, style_marks: &mut Query<'w, &'static mut StyleMark>, class: ClassName, class_sheet: &ClassSheet) {
    let style_mark = match style_marks.get(node) {
        Ok(r) => r,
        Err(_) => {
            log::debug!("node is not exist: {:?}", node);
            return;
        },
    };

    let (old_class_style_mark, local_style_mark) = (style_mark.class_style.clone(), style_mark.local_style.clone());
    let mut new_class_style_mark: StyleMarkType = BitArray::new([0, 0, 0, 0]);

    // 设置class样式
    for i in class.iter() {
        if let Some(class) = class_sheet.class_map.get(i) {
            log::trace!("set class==========={:?}, {:?}", node, i);
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


    if let Ok(mut style_mark) = style_marks.get_mut(node) {
        style_mark.class_style |= new_class_style_mark;
		style_mark.dirty_style |= new_class_style_mark;
    };

    if let Ok(mut class_name) = style_query.style.class_name.get_mut(node) {
        *class_name = class;
    };
}

fn delete_draw_list(id: Entity, draw_list: &Query<&DrawList>, draw_objects: &mut Vec<Entity>) {
    draw_objects.push(id);
    log::debug!("deleteNode node====================node：{:?}", id);
    if let Ok(list) = draw_list.get(id) {
        for i in list.iter() {
            draw_objects.push(i.id);
        }
    }
}
