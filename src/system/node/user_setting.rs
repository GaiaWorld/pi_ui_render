//! 每个实体必须写入StyleMark组件

use pi_world::{filter::Or, prelude::{Alter, Changed, Entity, Local, Query, SingleResMut, With, World}, single_res::SingleRes, world::FromWorld};
use pi_bevy_ecs_extend::prelude::{EntityTreeMut, OrInitSingleResMut};

use bitvec::array::BitArray;
use pi_map::Map;
use pi_null::Null;
use pi_slotmap_tree::InsertType;

use crate::{
    components::{
        calc::{DrawInfo, EntityKey, StyleMarkType}, user::{serialize::DefaultStyle, Size, STYLE_COUNT}, SettingComponentIds
    }, resource::{
        fragment::{FragmentMap, NodeTag}, ClassSheet, NodeChanged, QuadTree
    }
};
use crate::{
    components::{
        calc::{DrawList, StyleMark, StyleType},
        user::{
            serialize::{Setting, StyleAttr, StyleTypeReader},
            ClassName,
        },
    },
    resource::{NodeCommand, StyleCommands, UserCommands},
};

pub struct SingleId {
    pub user_commands: usize,
    pub class_sheet: usize,
    pub fragments: usize,
    // pub quad_tree: usize,
    // pub node_changed: usize,
}

impl FromWorld for SingleId {
    fn from_world(world: &mut World) -> Self {
        Self {
            user_commands: world.init_single_res::<UserCommands>(),
            class_sheet: world.init_single_res::<ClassSheet>(),
            fragments: world.init_single_res::<FragmentMap>(),
            // quad_tree: world.or_register_single_res::<QuadTree>(),
            // node_changed: world.or_register_single_res::<NodeChanged>(),
        }
    }
}

/// 处理用户设置的指令
pub fn user_setting1(
    world: &mut World,
    id: Local<SingleId>,
    setting_components: Local<SettingComponentIds>,
    default_style: Local<DefaultStyle>,
) {
    // let mut fragments_default = FragmentMap::default();
    // let mut class_sheet_default = ClassSheet::default();

    let mut w1 = world.unsafe_world();
    let mut w2 = world.unsafe_world();
    let mut w3 = world.unsafe_world();

    let user_commands = w1.index_single_res_mut::<UserCommands>(id.user_commands).unwrap().0;
    
    let fragments = w2.index_single_res_mut::<FragmentMap>(id.fragments).unwrap().0;

    let class_sheet = w3.index_single_res_mut::<ClassSheet>(id.class_sheet).unwrap().0;

    // 应用other_commands指令
    user_commands.other_commands.apply(world);

    user_commands.version += 1;

	let is_node_change = user_commands.is_node_change || user_commands.fragment_commands.len() > 0 || user_commands.node_commands.len() > 0;
    user_commands.is_node_change = is_node_change;

    // 添加基础组件id
    let mut base_component_ids = UserCommands::init_component_ids(NodeTag::Div, &setting_components);
    // 添加基础组件id
    let mut v_node_base_component_ids = UserCommands::init_component_ids(NodeTag::VNode, &setting_components);
    let mut component_ids1 = Vec::new();
    let mut component_ids;
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
            if world.contains_entity(*node) {
                if n.tag == NodeTag::VNode {
                    component_ids = &mut v_node_base_component_ids;
                } else {
                    component_ids = &mut base_component_ids;
                }
                let old_len = component_ids.len();
                // 添加style所组件id
                if n.style_meta.end > n.style_meta.start {
                    add_component_ops(n.style_meta.start, n.style_meta.end, &fragments.style_buffer, &setting_components, &mut component_ids);
                }
                // 添加class所组件id
                if n.class.len() > 0 {           
                    for i in n.class.iter() {
                        if let Some(class) = class_sheet.class_map.get(i) {
                            add_component_ops(class.start, class.end, &class_sheet.style_buffer, &setting_components, component_ids)
                        }
                    }
                }

                // 初始化组件
                let _ = world.alter_components(*node, &component_ids);
                unsafe { component_ids.set_len(old_len); }

                log::debug!("insert NodeBundle for fragment , {:?}", node);
            } else {
				log::error!("insert NodeBundle fail, fragment entity is not exist, {:?}, {:?}", node, n.tag);
			}
        }
    }

	
    let mut setting = Setting {world,  style: &setting_components, default_value: &default_style };

    // 设置模板节点的style
    for c in user_commands.fragment_commands.iter() {
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
                set_style(node, n.style_meta.start, n.style_meta.end, &fragments.style_buffer, &mut setting,  true);
            }
            if n.class.len() > 0 {
                set_class(node, &mut setting, n.class.clone(), &class_sheet, &mut component_ids1);
            }
        }
    }

    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, base_component_ids, v_node_base_component_ids);
    // 设置class样式
    for (node, class) in user_commands.class_commands.drain(..) {
        // 添加组件
        for i in class.iter() {
            if let Some(class) = class_sheet.class_map.get(i) {
                add_component_ops(class.start, class.end, &class_sheet.style_buffer, &setting_components, &mut component_ids1)
            }
        }
        let _ = setting.world.alter_components(node, &mut component_ids1);
        component_ids1.clear();

        set_class(node, &mut setting,  class, &class_sheet, &mut component_ids1);

    }
}

// 为节点添加依赖父子依赖关系 和 销毁节点
pub fn user_setting2(
    mut entitys: Alter<(), Or<(With<Size>, With<DrawInfo>)>, (), ()>,
    dirty_list: Query<&DrawList>,

    mut user_commands: SingleResMut<UserCommands>,
    mut quad_tree: OrInitSingleResMut<QuadTree>,
    mut tree: EntityTreeMut,
    fragments: SingleRes<FragmentMap>,
    mut node_changed: OrInitSingleResMut<NodeChanged>,
) {
    let mut is_node_change = user_commands.is_node_change;
    // 添加父子关系
    for c in user_commands.fragment_commands.drain(..) {
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
            if let (false, true) = (n.parent.is_null(), entitys.contains(*node)) {
                log::debug!(
                    "fragment_commands insertChild====================node：{:?}, parent {:?}",
                    node,
                    c.entitys[n.parent]
                );
                // log::warn!("fragment_commands insertChild====================node：{:?}, parent {:?}", node, c.entitys[n.parent]);
                tree.insert_child(*node, c.entitys[n.parent], std::usize::MAX);
            }
        }
    }

    // 操作节点(节点的销毁、挂载、删除)
    for c in user_commands.node_commands.drain(..) {
        match c {
            NodeCommand::AppendNode(node, parent) => {
                if entitys.contains(node) {
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
                    tree.insert_child(node, parent, std::usize::MAX);
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

                if entitys.contains(node) {
                    log::debug!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    // log::warn!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    tree.insert_brother(node, anchor, InsertType::Front);
                }
            }
            NodeCommand::RemoveNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("RemoveNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }


				log::debug!("RemoveNode node====================node={node:?}");
                tree.remove(node);
            }
            NodeCommand::DestroyNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("DestroyNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }
				log::debug!("DestroyNode node====================node={node:?}");
                is_node_change = true;
                // 删除所有子节点对应的实体
                if let Some(down) = tree.get_down(node) {
                    let head = down.head();
                    if !head.is_null() {
                        for node in tree.recursive_iter(head) {
                            quad_tree.remove(&EntityKey(node));
                            delete_draw_list(node, &dirty_list, &mut entitys);
                        }
                    }
                }
                quad_tree.remove(&EntityKey(node));
                delete_draw_list(node, &dirty_list, &mut entitys);
            }
        };
    }

	if is_node_change {
        node_changed.0 = true;
        user_commands.is_node_change = false;
	}
    
    // // 设置所有的root渲染脏（节点删除后， 组件被删除，很多状态丢失， 除非立即处理脏区域）
    // for mut render_dirty in query.p0().2.iter_mut() {
    //     render_dirty.0 = true;
    // }
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

pub fn set_styles<'w, 's>(
    style_commands: &mut StyleCommands, 
    style_query: &mut Setting,
    mut base_component_ids: Vec<(u32, bool)>,
    mut v_node_base_component_ids: Vec<(u32, bool)>,
) -> (Vec<(u32, bool)>, Vec<(u32, bool)>) {
    let mut component_ids1 = Vec::new();
    let mut component_ids;
    let mut old_len;
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end, need_init) in commands.drain(..) {
        
        println!("style========{:?}",( node, start, end, need_init));
        if end - start == 0 {
            continue;
        }
        if let Some(tag) = need_init {
            if tag == NodeTag::VNode {
                component_ids = &mut v_node_base_component_ids;
            } else {
                component_ids = &mut base_component_ids;
            }
        } else {
            component_ids = &mut component_ids1;
        }

        old_len = component_ids.len();
        add_component_ops(start, end, style_buffer, &style_query.style, component_ids);
        unsafe { component_ids.set_len(old_len); }

        set_style(node, start, end, style_buffer, style_query, false);
    }
    unsafe { style_buffer.set_len(0) };

    (base_component_ids, v_node_base_component_ids)
}

pub fn set_style<'w, 's>(node: Entity, start: usize, end: usize, style_buffer: &Vec<u8>, style_query: &mut Setting, is_clone: bool) {
    // 不存在实体，不处理
    if !style_query.world.contains(node) {
        log::debug!("node is not exist: {:?}", node);
        return;
    }
	log::trace!("set_style==========={:?}", node);

    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    let mut local_mark = BitArray::new([0, 0, 0, 0]);
    while style_reader.write_to_component(&mut local_mark, node, style_query, is_clone) {}

    if let Ok(style_mark) = style_query.world.get_component_by_index_mut::<StyleMark>(node, style_query.style.style_mark) {
        style_mark.local_style |= local_mark;
		style_mark.dirty_style |= local_mark;
    };
    // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
}

pub fn add_component_ops<'w, 's>(start: usize, end: usize, style_buffer: &Vec<u8>, component_ids: &SettingComponentIds, ops: &mut Vec<(u32, bool)>) {
    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    while style_reader.push_component_ops(component_ids, ops) {}
}


fn set_class<'w, 's>(node: Entity, style_query: &mut Setting, class: ClassName, class_sheet: &ClassSheet, component_ids1: &mut Vec<(u32, bool)>) {
    let style_mark = match style_query.world.get_component_by_index::<StyleMark>(node, style_query.style.style_mark) {
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
        StyleAttr::reset(&mut cur_style_mark, i as u8, &buffer, 0, style_query, node);
        StyleAttr::push_component_ops(i as u8 + STYLE_COUNT, &style_query.style,  component_ids1)
    }

    if component_ids1.len() > 0 {
        let _ = style_query.world.alter_components(node, &component_ids1);
        component_ids1.clear();
    }


    if let Ok(style_mark) = style_query.world.get_component_by_index_mut::<StyleMark>(node, style_query.style.style_mark) {
        style_mark.class_style |= new_class_style_mark;
		style_mark.dirty_style |= new_class_style_mark;
    };

    if let Ok(class_name) = style_query.world.get_component_by_index_mut::<ClassName>(node, style_query.style.class_name) {
        *class_name = class;
    };
}

fn delete_draw_list(id: Entity, draw_list: &Query<&DrawList>, entitys: &mut Alter<(), Or<(With<Size>, With<DrawInfo>)>, (), ()>) {
    let _ = entitys.destroy(id);
    log::debug!("deleteNode node====================node：{:?}", id);
    if let Ok(list) = draw_list.get(id) {
        for i in list.iter() {
            let _ = entitys.destroy(i.id);
        }
    }
}
