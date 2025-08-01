

use std::collections::hash_map::Entry;

use pi_hash::XHashMap;
use pi_world::event::ComponentChanged;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, Entity, FilterComponents, ParamSet, Query, SingleRes, SingleResMut};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};

use pi_bevy_render_plugin::{NodeId, PiRenderGraph, NodeLabel};
use pi_null::Null;

use crate::components::pass_2d::PostProcessInfo;
use crate::resource::IsRun;
use crate::{
    components::{
        calc::{EntityKey, RenderContextMark},
        pass_2d::{ChildrenPass, GraphId, ParentPassId},
        user::AsImage,
    }, resource::draw_obj::LastGraphNode, system::base::pass::pass_graph_node::{Pass2DNode, CustomCopyNode}
};

// 初始化渲染图的根节点
pub fn init_root_graph(
    mut last_graph_id: OrInitSingleResMut<LastGraphNode>,
    mut rg: SingleResMut<PiRenderGraph>,
	r: OrInitSingleRes<IsRun>
) {
    if r.0 {
		return;
	}

    last_graph_id.0 = rg.add_node("Pass2DLast".to_string(), Pass2DNode::new(EntityKey::null().0), NodeId::default(), Entity::null()).unwrap();
    let _ = rg.set_is_build(last_graph_id.0, false); // 不build
    if let Err(e) = rg.set_finish(last_graph_id.0, true) {
        log::error!("{:?}", e);
    }
}

/// 根据声明创建图节点，删除图节点， 建立图节点的依赖关系
pub fn update_graph(
    mut pass_query: ParamSet<(
        Query<(Option<&mut GraphId>, &ParentPassId, &PostProcessInfo)>,
        (
			Query<(&mut ParentPassId, &GraphId, Option<&mut AsImage>, Entity), Or<(Changed<ParentPassId>, Changed<AsImage>, Changed<GraphId>)>>, 
			Query<(&ParentPassId, &GraphId)>,
			Query<&GraphId>,
			Query<&ChildrenPass>,
		),

    )>,
    mark_query: Query<&RenderContextMark>,

    mark_changed: ComponentChanged<RenderContextMark>,
    // removed: ComponentRemoved<Camera>,
    last_graph_id: SingleRes<LastGraphNode>,
    // del: Query<(Entity, Has<Camera>), With<Size>>,
    mut rg: SingleResMut<PiRenderGraph>,
    mut ref_count: OrInitSingleResMut<AsImageRefCount>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 创建渲染图节点
    // 插入Draw2DList
    
    let ref_count = &mut *ref_count;
    for entity in mark_changed.iter() {
        let p0 = pass_query.p0();
        log::debug!(entity=format!("entity_{:?}", entity).as_str();  "update graph, entity={entity:?}");
        if let Ok(mark) = mark_query.get(*entity) {
            if mark.any() {
                if let Ok((graph_id, parent_passs_id, post_info)) = p0.get_mut(*entity) {
                    let is_root = pi_null::Null::is_null(&parent_passs_id.0);
                    log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node1, entity={entity:?}, mark={:?}, is_root: {:?}, parent_passs_id={:?}", mark.any(),  is_root, parent_passs_id);
                    // if post_info.has_effect() || is_root {
                    // 存在后处理效果，或者节点本身是根节点， 才能成为一个渲染节点
                    let mut graph_id = match graph_id {
                        Some(r) => r,
                        None => continue,
                    };

                    let is_tansfer = !post_info.has_effect() && !is_root;
                    
                    if !graph_id.0.is_null() {
                        rg.set_is_transfer(graph_id.0, is_tansfer);
                        continue;
                    }

                    let add_r = rg.add_node_not_run(format!("Pass2D_{:?}", entity), Pass2DNode::new(*entity), NodeId::default(), *entity);
                    let graph_node_id = match add_r {
                        Ok(r) => r,
                        Err(e) => {
                            log::error!("node: {:?}, {:?}", format!("Pass2D_{:?}", entity), e);
                            continue;
                        }
                    };
                    rg.set_is_transfer(graph_node_id, is_tansfer);

                    if is_root {
                        log::debug!("add_depend======{:?}, {:?}", graph_node_id, last_graph_id.0);
                        rg.add_depend(graph_node_id, last_graph_id.0).unwrap();
                        
                    }
                    log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node, entity: {entity:?} graph_node_id: {graph_node_id:?}");

                    *graph_id = GraphId(graph_node_id);
                }

                // 从无fbo， 变为有fbo，需要添加当前图节点到父图节点的依赖关系
                // 其父需要删除不再对应的依赖关系, 
                // TODO: 此处实现了， 后续遍历设置依赖关系不再需要
            } else { 
                log::debug!("remove graph======{:?}", &entity);
                remove_node(format!("Pass2D_{:?}", entity), &mut rg, ref_count);
            }
        }
    }

    // for i in removed.iter() {
    //     // 移除渲染图节点
    //     if let Ok((_id, has_camera)) = del.get(*i) {
    //         if has_camera {
    //             continue;
    //         }
           
    //     }

    //     log::debug!(entity=format!("entity_{:?}", *i).as_str(); "remove graph node, entity={i:?}");
    //     remove_node(format!("Pass2D_{:?}", *i), &mut rg, ref_count);
    // }


    let p1 = pass_query.p1();
    // 父修改设置图节点依赖 TODO 遍历优化
    for (mut parent_id, graph_id, mut as_image, entity) in p1.0.iter_mut() {
        let parent_id = parent_id.bypass_change_detection();
        log::trace!("parent_id====={:?}", (&parent_id, graph_id, &as_image));
        if graph_id.0.is_null() {
            continue;
        }
        let asimage = match as_image {
            Some(ref mut v) => Some(v.bypass_change_detection()),
            None => None,
        };
        let parent_graph_id = find_parent_graph_id(parent_id.0.clone(), &p1.1);
        let id = type_to_post_process(**graph_id, asimage, &p1.2, &mut rg, parent_graph_id, entity);
        // let id = graph_id.0.clone();
        // 建立父子依赖关系，使得子pass先渲染
        if id != parent_id.1.0 || parent_graph_id != parent_id.1.1 {
            log::debug!("add_depend======id: {:?}, graph_id: {:?}, parent_graph_id: {:?}, entity: {:?}, parent_pass_entity: {:?}", id, graph_id, parent_graph_id, entity, parent_id.0);
            if let Err(e) = rg.add_depend(id, parent_graph_id) {
                log::error!("{:?}", e);
            }
            if !parent_id.1.1.is_null() {
                let _ = rg.remove_depend(parent_id.1.0, parent_id.1.1);
            }
        }
        
        parent_id.1 = (id, parent_graph_id);
        
    }

	// 更新图结构
	let _ = rg.update();
}

// 如果存在后处理，连接到后处理
pub fn type_to_post_process(id: NodeId, as_image: Option<&mut AsImage>, graph_id_query: &Query<&GraphId>, rg: &mut PiRenderGraph, parent_id: NodeId, entity: Entity) -> NodeId {
    if let Some(as_image) = as_image {
        let post_process_graph =  match graph_id_query.get(*as_image.post_process) {
            Ok(r) => r.0,
            _ => Null::null()
        };

        if as_image.old_post_graph_id != post_process_graph  {
            // log::warn!("post_process_graph changed, old: {:?}, new: {:?}");
            // 后处理节点id改变， 移除pass2did和后处理节点的链接关系（注意， 如果是pass2d节点发生改变， 则不需要移除， 因为pass2d节点改变， 旧的psss2d节点必然被移除，也就自然跟后处理节点解绑了）
            if !as_image.old_post_graph_id.is_null()  {
                let _ = rg.remove_depend(as_image.old_pass2d_graph_id, as_image.old_post_graph_id);
            }

            // 后处理不为null，尝试添加后处理需要的链接关系
            if !post_process_graph.is_null() {
                if as_image.copy_graph_id.is_null() { 
                    // 移除id与父的链接关系
                    let _ = rg.remove_depend(id, parent_id);
                    // 该节点用于将后处理结果拷贝回RenderTaget，并添加copy节点与父的链接关系
                    as_image.copy_graph_id = rg.add_node_not_run(format!("Pass2D_CopyTarget_{:?}", entity), CustomCopyNode::new(entity), NodeId::default(), Null::null()).unwrap();
                } else if !as_image.old_post_graph_id.is_null() {
                    let _ = rg.remove_depend(as_image.old_post_graph_id, as_image.copy_graph_id); // 移除旧的后处理链接关系
                    let _ = rg.remove_depend(as_image.old_post_graph_id, parent_id); // 移除旧的后处理链接关系
                }
                // 添加当前节点与后处理节点的链接关系
                let _ = rg.add_depend(id, post_process_graph);
                // 添加后处理节点和copy节点的关系
                let _ = rg.add_depend(post_process_graph, as_image.copy_graph_id);
                // 后处理节点和parent节点相连， 是为了避免parent节点build时， 后处理节点的输出fbo已经调用reset方法释放了
                let _ = rg.add_depend(post_process_graph, parent_id);
                as_image.old_post_graph_id = post_process_graph;
                as_image.old_pass2d_graph_id = id;
                log::debug!("post graph ==========post_process_graph: {:?}, copy_graph_id: {:?}, parent_id:{:?}", post_process_graph, as_image.copy_graph_id, parent_id);
                return as_image.copy_graph_id;
            }
            as_image.old_post_graph_id = post_process_graph;
            // 如果last_graph_id不是pass2did，也不是null， 则一定是copy节点， 此时需要销毁copy节点（没有字段记录copy节点的id了）
            if !as_image.copy_graph_id.is_null() {
                let _ = rg.remove_node(as_image.copy_graph_id);
                as_image.copy_graph_id = Null::null();
            } 
        }
        if post_process_graph.is_null() {
            return id;
        } else {
            return as_image.copy_graph_id;
        }
	}
	return id;
}

pub fn find_parent_graph_id<'w, 's, F: FilterComponents>(parent_id: Entity, query: &Query<(&'w ParentPassId, &'s GraphId), F>) -> NodeId {
    if let Ok((mut parent_id, mut parent_graph_id)) = query.get(parent_id) {
        // 父的pass2d不存在图节点， 继续找父
        while parent_graph_id.0.is_null() {
            if let Ok((parent_id1, parent_graph_id1)) = query.get(parent_id.0) {
                parent_id = parent_id1;
                parent_graph_id = parent_graph_id1;
            } else {
                break;
            }
        }

        return parent_graph_id.0;
    }
    NodeId::null()
}


// 移除图节点
// 调用该方法的节点，必定是一个Pass， 并且不为根， 因此to有且仅有一个
// 当前节点的from， 如果存在其为as_image, 需要重置as_image的引用计数
pub fn remove_node<T: Into<NodeLabel> + Clone>(graph_id: T, rg: &mut PiRenderGraph, ref_count: &mut AsImageRefCount) {
	if let (Ok(from), Ok(to)) = (rg.before_nodes(graph_id.clone()), rg.after_nodes(graph_id.clone())) {
		let from: Vec<NodeId> = Vec::from(from);
		let to: Vec<NodeId> = Vec::from(to);
        
		if let Ok(graph_id) = rg.remove_node(graph_id) {
            if from.len() > 0 {
                for from in from.iter() {
                    if ref_count.release_one((from.clone(), graph_id.clone())).is_some() {
                         // 如果存在引用计数，说明from为asImage节点， 且from为gui中的节点（非gui系统之外的节点）， 且graph_id对应实体一定不为gui的根节点（gui的根节点永远不会从有fbo变为无fbo， 不会调用此方法）
                        // 因此to有且仅有一个（gui中，pass为树状图）， 且to也为gui节点
                        // 需要对（from，to）的引用计数加一
                        for to in to.iter() {
                            ref_count.add_one((from.clone(), to.clone()));
                        }
                    }
                }
            }
			// 重新绑定依赖关系
			if from.len() > 0 && to.len() > 0 {
				for before in from.into_iter() {
					for after in to.iter() {
						let _ = rg.add_depend(before, *after);
					}
				}
			}
		}
	}
}

// 引用计数
// 如节点1 使用`asimage:://节点3`作为图片路径
// 如节点2也使用`asimage:://节点3`作为图片路径
// 节点1和节点2在同一个`图节点1`中渲染
// 节点3在            `图节点2`中渲染
// 在此处将记录 (`图节点2`, `图节点1`) -> 2
#[derive(Debug, Default)]
pub struct AsImageRefCount(pub XHashMap<(NodeId, NodeId), usize>);

impl AsImageRefCount {
	pub fn add_one(&mut self, key: (NodeId, NodeId  )) {
		match self.0.entry(key) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        };
	}

    pub fn release_one(&mut self, key: (NodeId, NodeId  )) -> Option<usize> {
        match self.0.entry(key) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() -= 1;
                let r = if *occupied_entry.get() == 0 { // 引用计数减为0时， 移除引用计数
                    occupied_entry.remove();
                    0
                } else {
                    *occupied_entry.get()
                };
                Some(r)
            },
            Entry::Vacant(_vacant_entry) => None
        }
	}
}


