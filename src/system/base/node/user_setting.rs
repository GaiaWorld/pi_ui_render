//! 每个实体必须写入StyleMark组件

use pi_bevy_render_plugin::{render_cross::GraphId, PiRenderGraph};
use pi_world::{event::{Event, EventSender}, filter::Or, prelude::{Alter, Entity, Local, Mut, Query, SingleResMut, With, World}, single_res::SingleRes, system::{SystemMeta, TypeInfo}, system_params::SystemParam, world::FromWorld};
use pi_world::world::ComponentIndex;
use pi_key_alloter::Key;
use pi_bevy_ecs_extend::prelude::{EntityTreeMut, OrInitSingleResMut};

use bitvec::array::BitArray;
use pi_map::Map;
use pi_null::Null;
use pi_slotmap_tree::InsertType;
use pi_style::style_type::STYLE_COUNT;

use crate::{
    components::{
        calc::{DrawInfo, EntityKey, NodeState, StyleMarkType}, user::{serialize::DefaultStyle, Size, ZIndex}, SettingComponentIds
    }, resource::{
        animation_sheet::KeyFramesSheet, fragment::{FragmentMap, NodeTag}, ClassSheet, GlobalDirtyMark, OtherDirtyType, PassGraphMap, QuadTree
    }, system::base::pass::update_graph::remove_node
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
    pub style_dirty_mark: usize,
    pub class_sheet: usize,
    pub fragments: usize,
    pub global_mark: usize,
    // pub quad_tree: usize,
    // pub node_changed: usize,
}

impl FromWorld for SingleId {
    fn from_world(world: &mut World) -> Self {
        Self {
            style_dirty_mark: world.init_single_res::<StyleDirtyMark>(),
            user_commands: world.init_single_res::<UserCommands>(),
            class_sheet: world.init_single_res::<ClassSheet>(),
            fragments: world.init_single_res::<FragmentMap>(),
            global_mark: world.init_single_res::<GlobalDirtyMark>(),
            // quad_tree: world.or_register_single_res::<QuadTree>(),
            // node_changed: world.or_register_single_res::<NodeChanged>(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StyleDirtyMark(pub bitvec::vec::BitVec<usize>);

/// 处理用户设置的指令
pub fn user_setting1(
    world: &mut World,
    id: Local<SingleId>,
    setting_components: Local<SettingComponentIds>,
    default_style: Local<DefaultStyle>,
    // mut dirty_mark: Local<bitvec::vec::BitVec<usize>>,
) {
    // let mut fragments_default = FragmentMap::default();
    // let mut class_sheet_default = ClassSheet::default();

    let mut w1 = world.unsafe_world();

    let user_commands = w1.index_single_res_mut::<UserCommands>(id.user_commands).unwrap();
    // 应用other_commands指令
    user_commands.other_commands.apply(world);
    user_commands.version += 1;

    if user_commands.node_commands.is_empty() && 
        user_commands.fragment_commands.is_empty() && 
        user_commands.style_commands.commands.is_empty() &&
        user_commands.class_commands.is_empty() {
        return;
    }

    let mut w2 = world.unsafe_world();
    let mut w3 = world.unsafe_world();
    let mut w4 = world.unsafe_world();
    let w5 = world.unsafe_world();
    let mut w6 = world.unsafe_world();
    let mut w7 = world.unsafe_world();
    
    let mut global_mark = w7.index_single_res_mut::<GlobalDirtyMark>(id.global_mark).unwrap();
    let fragments = w2.index_single_res_mut::<FragmentMap>(id.fragments).unwrap();

    let class_sheet = w3.index_single_res_mut::<ClassSheet>(id.class_sheet).unwrap();
    let dirty_mark = w4.index_single_res_mut::<StyleDirtyMark>(id.style_dirty_mark).unwrap();
    let mut s_meta = SystemMeta::new(TypeInfo::of::<()>());

    let mut events = EventSender::<'_, StyleChange>::init_state(&mut w6, &mut s_meta);
    let mut dirty_list = StyleDirtyList {
		list: EventSender::<'_, StyleChange>::get_param(&w5, &mut s_meta, &mut events, world.tick()),
		mark: &mut dirty_mark.0,
	};

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
                let _ = world.make_entity_editor().alter_components_by_index(*node, &component_ids);
                unsafe { component_ids.set_len(old_len); }

                log::debug!("insert NodeBundle for fragment , {:?}", node);
                dirty_list.mark_dirty(*node);
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

            // 设置为vnode
            if n.tag == NodeTag::VNode {
                crate::components::user::serialize::set_style_attr(&mut setting.world, setting.style.node_state, node, true, |mut n: Mut<NodeState>, v| {
                    n.set_vnode(v);
                });
                crate::components::user::serialize::set_style_attr(&mut setting.world, setting.style.z_index, node, -1, |mut n: Mut<ZIndex>, v| {
                    n.0 = v;
                });
            }

            if n.style_meta.end > n.style_meta.start {
                set_style(node, n.style_meta.start, n.style_meta.end, &fragments.style_buffer, &mut setting,  true, &mut dirty_list, &mut global_mark);
            }
            if n.class.len() > 0 {
                set_class(node, &mut setting, n.class.clone(), &class_sheet, &mut component_ids1, &mut dirty_list, &mut global_mark);
            }
        }
    }

    // 设置style只要节点存在,样式一定能设置成功
    set_styles(&mut user_commands.style_commands, &mut setting, base_component_ids, v_node_base_component_ids, &mut dirty_list, &mut global_mark);
    // 设置class样式
    for (node, class) in user_commands.class_commands.drain(..) {
        // 添加组件
        for i in class.iter() {
            if let Some(class) = class_sheet.class_map.get(i) {
                add_component_ops(class.start, class.end, &class_sheet.style_buffer, &setting_components, &mut component_ids1)
            }
        }
        let _ = setting.world.make_entity_editor().alter_components_by_index(node, &mut component_ids1);
        component_ids1.clear();

        set_class(node, &mut setting,  class, &class_sheet, &mut component_ids1, &mut dirty_list, &mut global_mark);

    }
}

pub struct AddEvent(pub Entity);
pub struct RemoveEvent(pub Entity);

// 为节点添加依赖父子依赖关系 和 销毁节点
pub fn user_setting2(
    mut entitys: Alter<(Option<&Size>, Option<&DrawInfo>), Or<(With<Size>, With<DrawInfo>)>, (), ()>,
    dirty_list: Query<(Option<&DrawList>, Option<&GraphId>)>,
    mut user_commands: SingleResMut<UserCommands>,
    mut quad_tree: OrInitSingleResMut<QuadTree>,
    mut tree: EntityTreeMut,
    fragments: SingleRes<FragmentMap>,

    event_sender: EventSender<'_, StyleChange>,
    mut style_dirty_mark: SingleResMut<StyleDirtyMark>,

    mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
    mut global_mark: SingleResMut<GlobalDirtyMark>,
    add_events: EventSender<AddEvent>,
    remove_events: EventSender<RemoveEvent>,
    mut rg: SingleResMut<PiRenderGraph>,
	mut pass_graph_map: OrInitSingleResMut<PassGraphMap>,
) {

    let mut dirty_list_mark = StyleDirtyList {
		list: event_sender,
		mark: &mut style_dirty_mark.0,
	};
    let mut is_add = false;
    let mut is_del = false;
    let mut is_remove = false;

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

    let keyframes_sheet = &mut *keyframes_sheet;

    // 操作节点(节点的销毁、挂载、删除)
    for c in user_commands.node_commands.drain(..) {
        match c {
            NodeCommand::AppendNode(node, parent) => {
                is_add = true;
                log::debug!("AppendNode====================node： {:?}, parent： {:?}, node_is_exist：{:?}, parent_is_exist: {:?}", node, parent, entitys.contains(node), entitys.contains(parent));
                // if entitys.contains(node) && (parent.is_null() || entitys.contains(parent)) {
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
					
                    // log::warn!("AppendNode node====================node： {:?}, parent： {:?}", node, parent);
                    dirty_list_mark.mark_dirty(node);
                    tree.insert_child(node, parent, std::usize::MAX);
                    if !tree.layer(node).layer().is_null() {
                        add_events.send(AddEvent(node));
                    }
                // }
            }
            NodeCommand::InsertBefore(node, anchor) => {
                is_add = true;
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
                
                log::debug!("InsertBefore node1====================node：{:?}, anchor： {:?}, node_is_exist：{:?}", node, anchor, entitys.contains(node));
                // if entitys.contains(node) && (anchor.is_null() || entitys.contains(anchor)) {
                   
                    // log::warn!("InsertBefore node====================node：{:?}, anchor： {:?}", node, anchor);
                    dirty_list_mark.mark_dirty(node);
                    tree.insert_brother(node, anchor, InsertType::Front);
                    if !tree.layer(node).layer().is_null() {
                        add_events.send(AddEvent(node));
                    }
                // }
            }
            NodeCommand::RemoveNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("RemoveNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }
                if !tree.layer(node).layer().is_null() {
                    remove_events.send(RemoveEvent(node));
                    let up = tree.up(node);
                    if !up.parent().is_null() {
                        dirty_list_mark.mark_dirty(up.parent());
                    }
                    
                    is_remove = true;
                    log::debug!("RemoveNode node====================node={node:?}");
                    tree.remove(node);
                }
            }
            NodeCommand::DestroyNode(node) => {
				// if !EntityKey( node ).is_null() && draw_list.get(node).is_err() {
				// 	log::warn!("DestroyNode node error============={:?}, {:?}", node, unsafe{transmute::<_, f64>(node.to_bits())});
				// 	r.0 = true;
				// 		return;
				// }
                is_del = true;
                
                // 删除所有子节点对应的实体
                if let Some(down) = tree.get_down(node) {
                    let up = tree.up(node);
                    if !up.parent().is_null() {
                        dirty_list_mark.mark_dirty(up.parent());
                    }

                    let head = down.head();
                    if !head.is_null() {
                        for node in tree.recursive_iter(head) {
                            quad_tree.remove(&EntityKey(node));
                            delete_entity(node, &dirty_list, &mut entitys, keyframes_sheet, &mut rg, &mut pass_graph_map);

                        }
                    }
                }
                quad_tree.remove(&EntityKey(node));
                tree.remove(node);
                delete_entity(node, &dirty_list, &mut entitys, keyframes_sheet, &mut rg, &mut pass_graph_map);
            }
        };
    }
    if is_add {
        global_mark.mark.set(OtherDirtyType::NodeTreeAdd as usize, true);
    }
    if is_del {
        global_mark.mark.set(OtherDirtyType::NodeTreeDel as usize, true);
    }

    if is_remove {
        global_mark.mark.set(OtherDirtyType::NodeTreeRemove as usize, true);
    }

	// if is_node_change {
    //     node_changed.node_changed = true;
    //     user_commands.is_node_change = false;
    //     log::debug!("node_changed4============{:p}", &*node_changed);
	// }
    
    // // 设置所有的root渲染脏（节点删除后， 组件被删除，很多状态丢失， 除非立即处理脏区域）
    // for mut render_dirty in query.p0().2.iter_mut() {
    //     render_dirty.0 = true;
    // }
}


/// 清理StyleMark上的脏标记
pub fn clear_dirty_mark(
    mut style_mark: Query<&mut StyleMark>,
    event: Event<StyleChange>,
    mut dirty_mark: OrInitSingleResMut<StyleDirtyMark>,
) {
	for r in event.iter() {
        if let Ok(mut r) = style_mark.get_mut(r.0) {
            r.bypass_change_detection().dirty_style = Default::default();
        }
	}
    // 清理标记（该标记用于将本次修改样式的操作合并成一个事件）
	dirty_mark.0.clear();
}

pub struct StyleDirtyList<'s, 'w> {
	pub list: EventSender<'w, StyleChange>,
	pub mark: &'s mut bitvec::vec::BitVec<usize>,
}

impl<'s, 'w> StyleDirtyList<'s, 'w> {
	/// 标记脏
	pub fn mark_dirty(&mut self, entity: Entity) {
		let index = entity.index() as usize;
		if self.mark.len() <= index {
			let count: usize = (index - self.mark.len()) / std::mem::size_of::<usize>()  + 1;
			for _ in 0..count {
				self.mark.extend(Some(0));
			}
		}
		if !self.mark[index] {
			self.list.send(StyleChange(entity));
		}
        self.mark.set(index, true);
	}

	/// 清理标记
	pub fn clear_mark(&mut self) {
		self.mark.clear();
	}
}

#[derive(Debug, Copy, Clone, Deref)]
pub struct StyleChange(pub Entity);

pub fn set_styles<'w, 's>(
    style_commands: &mut StyleCommands, 
    style_query: &mut Setting,
    mut base_component_ids: Vec<(ComponentIndex, bool)>,
    mut v_node_base_component_ids: Vec<(ComponentIndex, bool)>,
    dirty_list: &mut StyleDirtyList<'w, 's>,
    global_mark: &mut GlobalDirtyMark,
) -> (Vec<(ComponentIndex, bool)>, Vec<(ComponentIndex, bool)>) {
    let mut component_ids1 = Vec::new();
    let mut component_ids;
    let mut old_len;
    let (style_buffer, commands) = (&mut style_commands.style_buffer, &mut style_commands.commands);
    for (node, start, end, need_init) in commands.drain(..) {
        
        if end - start == 0 && need_init.is_none() {
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
        log::debug!("add_component_ops===={:?}", (node, &component_ids, need_init));
        let _ = style_query.world.make_entity_editor().alter_components_by_index(node, component_ids);
        unsafe { component_ids.set_len(old_len); }

        set_style(node, start, end, style_buffer, style_query, false, dirty_list, global_mark);
    }
    unsafe { style_buffer.set_len(0) };

    (base_component_ids, v_node_base_component_ids)
}

pub fn set_style<'w, 's>(node: Entity, start: usize, end: usize, style_buffer: &Vec<u8>, style_query: &mut Setting, is_clone: bool, dirty_list: &mut StyleDirtyList<'w, 's>, global_mark: &mut GlobalDirtyMark,) {
    // 不存在实体，不处理
    if !style_query.world.contains(node) {
        log::debug!("node is not exist: {:?}", node);
        return;
    }
	log::trace!("set_style==========={:?}", node);

    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    let mut local_mark = BitArray::new([0, 0, 0, 0]);
    while style_reader.write_to_component(&mut local_mark, node, style_query, is_clone) {}

    if let Ok(mut style_mark) = style_query.world.get_component_mut_by_index::<StyleMark>(node, style_query.style.style_mark) {
        style_mark.local_style |= local_mark;
		style_mark.dirty_style |= local_mark;
        global_mark.mark |= local_mark;
    };
    // 取消样式， TODO，注意，宽高取消时，还要考虑图片宽高的重置问题
    dirty_list.mark_dirty(node);
}

pub fn add_component_ops<'w, 's>(start: usize, end: usize, style_buffer: &Vec<u8>, component_ids: &SettingComponentIds, ops: &mut Vec<(ComponentIndex, bool)>) {
    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    while style_reader.push_component_ops(component_ids, ops) {}
}


fn set_class<'w, 's>(node: Entity, style_query: &mut Setting, class: ClassName, class_sheet: &ClassSheet, component_ids1: &mut Vec<(ComponentIndex, bool)>, dirty_list: &mut StyleDirtyList<'w, 's>, global_mark: &mut GlobalDirtyMark) {
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
        let _ = style_query.world.make_entity_editor().alter_components_by_index(node, &component_ids1);
        component_ids1.clear();
    }


    if let Ok(mut style_mark) = style_query.world.get_component_mut_by_index::<StyleMark>(node, style_query.style.style_mark) {
        style_mark.class_style |= new_class_style_mark;
		style_mark.dirty_style |= new_class_style_mark;
        global_mark.mark |= new_class_style_mark;
    };

    if let Ok(mut class_name) = style_query.world.get_component_mut_by_index::<ClassName>(node, style_query.style.class_name) {
        *class_name = class;
    };

    dirty_list.mark_dirty(node);
}

fn delete_entity(
    del: Entity, 
    draw_list: &Query<(Option<&DrawList>, Option<&GraphId>)>, 
    entitys: &mut Alter<(Option<&Size>, Option<&DrawInfo>), Or<(With<Size>, With<DrawInfo>)>, (), ()>,
    keyframes_sheet: &mut KeyFramesSheet,
    rg: &mut PiRenderGraph,
    pass_graph_map: &mut PassGraphMap,
) {
    if let Ok((list, graph)) = draw_list.get(del) {
        if let Some(list) = list  {
            for i in list.iter() {
                let r = entitys.destroy(i.id);
                log::debug!("delete draw obj====================id: {:?}", (i, r));
            }
        }   
        
        if let Some(graph) = graph {
            remove_node(**graph, rg, pass_graph_map);
        }
    }

    let r = entitys.destroy(del);
    log::debug!("removed===={:?}", del);
    keyframes_sheet.unbind_animation_all(del);
    keyframes_sheet.remove_runtime_keyframs(del);

    log::debug!("deleteNode node====================id: {:?}", (del, r));
}
