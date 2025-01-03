

use pi_world::event::{ComponentChanged, ComponentRemoved};
use pi_world::filter::Or;
use pi_world::prelude::{Changed, Entity, FilterComponents, Has, ParamSet, Query, SingleRes, SingleResMut, With};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::{NodeId, PiRenderGraph, NodeLabel};
use pi_null::Null;
use crate::components::user::Size;

use crate::resource::draw_obj::GuiSubGraphNode;
use crate::resource::IsRun;
use crate::{
    components::{
        calc::{EntityKey, RenderContextMark},
        pass_2d::{Camera, ChildrenPass, GraphId, ParentPassId, PostProcessInfo},
        user::AsImage,
    }, resource::{draw_obj::LastGraphNode, PassGraphMap}, system::base::pass::pass_graph_node::Pass2DNode
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

    last_graph_id.0 = rg.add_node("Pass2DLast".to_string(), Pass2DNode::new(EntityKey::null().0), NodeId::default()).unwrap();
    if let Err(e) = rg.set_finish(last_graph_id.0, true) {
        log::error!("{:?}", e);
    }
}

/// 根据声明创建图节点，删除图节点， 建立图节点的依赖关系
pub fn update_graph(
    mut pass_query: ParamSet<(
        Query<(&mut GraphId, Entity, &ParentPassId, &PostProcessInfo)>,
        (
			Query<(&ParentPassId, &GraphId, Option<&AsImage>), (Or<(Changed<ParentPassId>, Changed<AsImage>, Changed<GraphId>)>, With<Camera>)>, 
			Query<(&ParentPassId, &GraphId), With<Camera>>,
			Query<&GraphId>,
			Query<&ChildrenPass>,
		),

    )>,
    mark_changed: ComponentChanged<RenderContextMark>,
    removed: ComponentRemoved<Camera>,
    last_graph_id: SingleRes<LastGraphNode>,
    del: Query<(Entity, Has<Camera>), With<Size>>,
    mut rg: SingleResMut<PiRenderGraph>,
	mut pass_graph_map: OrInitSingleResMut<PassGraphMap>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 创建渲染图节点
    // 插入Draw2DList
    
    for entity in mark_changed.iter() {
        let mut p0 = pass_query.p0();
        if let Ok((mut graph_id, entity, parent_passs_id, post_info)) = p0.get_mut(*entity) {
            let is_root = pi_null::Null::is_null(&parent_passs_id.0);
            log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node, entity={entity:?}, has_effect={:?}, is_root: {:?}, parent_passs_id={:?}", post_info.has_effect(),  is_root, parent_passs_id);
            if post_info.has_effect() || is_root {
                // 存在后处理效果，或者节点本身是根节点， 才能成为一个渲染节点
                if !graph_id.0.is_null() {
                    continue;
                }

                let add_r = rg.add_node_not_run(format!("Pass2D_{:?}", entity), Pass2DNode::new(entity), NodeId::default());
                let graph_node_id = match add_r {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("node: {:?}, {:?}", format!("Pass2D_{:?}", entity), e);
                        return;
                    }
                };

                if is_root {
                    log::debug!("add_depend======{:?}, {:?}", graph_node_id, last_graph_id.0);
                    rg.add_depend(graph_node_id, last_graph_id.0).unwrap();
                    
                }
                pass_graph_map.insert(graph_node_id, entity);
                log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node, entity: {entity:?} graph_node_id: {graph_node_id:?}");

                *graph_id = GraphId(graph_node_id);
            } else {
                if graph_id.0.is_null() {
                    continue;
                }
                
                remove_node(**graph_id, &mut rg, &mut pass_graph_map);
                *graph_id = GraphId(NodeId::null());
            }
        }
    }

    for i in removed.iter() {
        // 移除渲染图节点
        if let Ok((id, has_camera)) = del.get(*i) {
            if has_camera {
                continue;
            }
           
        }

        log::debug!(entity=format!("entity_{:?}", *i).as_str(); "remove graph node, entity={i:?}");
        remove_node(format!("Pass2D_{:?}", *i), &mut rg, &mut pass_graph_map);
    }

    let p1 = pass_query.p1();
    // 父修改设置图节点依赖 TODO 遍历优化
    for (parent_id, graph_id, as_image) in p1.0.iter() {
        log::debug!("parent_id====={:?}", (parent_id, graph_id, as_image));
        if graph_id.0.is_null() {
            continue;
        }

        // if pi_null::Null::is_null(&parent_id.0) {
        //     if let Err(e) = rg.set_finish(**graph_id, true) {
        //         log::error!("{:?}", e);
        //     }
		// 	// 根节点忽略post_process
        // } else {
            let parent_graph_id = get_to(***parent_id, &p1.1);
			let id = type_to_post_process(**graph_id, as_image, &p1.2, &mut rg);

            // 建立父子依赖关系，使得子pass先渲染
            log::debug!("add_depend======{:?}, {:?}, {:?}", id, graph_id, parent_graph_id);
            if let Err(e) = rg.add_depend(id, parent_graph_id) {
                log::error!("{:?}", e);
            }
        // }
    }

	// 更新图结构
	let _ = rg.update();
}

// 如果存在后处理，连接到后处理
pub fn type_to_post_process(id: NodeId, as_image: Option<&AsImage>, graph_id_query: &Query<&GraphId>, rg: &mut PiRenderGraph) -> NodeId {
	if let Some(r) = as_image {
		if let Ok(post_process_graph) = graph_id_query.get(*r.post_process) {
			if !post_process_graph.is_null() {
				log::debug!("add_depend1======{:?}, {:?}", id, **post_process_graph);
				if rg.add_depend(id, **post_process_graph).is_ok() {
					return **post_process_graph
				} else {
					// 添加失败，post_process图节点可能已经销毁， 则应该忽略post_process
				}
			}
		}
	}
	return id;
}

pub fn get_to<'w, 's, F: FilterComponents>(parent_id: Entity, query: &Query<(&'w ParentPassId, &'s GraphId), F>) -> NodeId {
    if let Ok((mut parent_id, mut parent_graph_id)) = query.get(parent_id) {
        // 父的pass2d不存在图节点， 继续找父
        while parent_graph_id.0.is_null() {
            if let Ok((parent_id1, parent_graph_id1)) = query.get(***parent_id) {
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

// pub fn children_depend<'w>(
// 	next_graph_node: GraphId, 
// 	node: Entity, 
// 	query_children: &Query<(&'w ChildrenPass, &'w GraphId)>,
// 	query: &Query<(&'w ParentPassId, &'w GraphId, Option<&'w AsImage>, Entity), (Or<(Changed<ParentPassId>, Changed<AsImage>)>, With<Camera>)>,
// ) -> NodeId {
// 	if let Ok((children, graph_id)) = query_children.get(node) {
// 		if graph_id.is_null() {
// 			for child in (**children).iter() {
// 				if query.contains(**child) {
// 					// 如果子节点的父pass或as_image改变， 则后续遍历会处理， 不需要在此处重复处理
// 					continue;
// 				}
// 				// 继续递归处理子节点
// 				children_depend(next_graph_node.clone(), **child, query_children, query);
// 			}
// 		} else {
// 			// 
// 		}
		
// 	}

//     NodeId::null()
// }


pub fn remove_node<T: Into<NodeLabel> + Clone>(graph_id: T, rg: &mut PiRenderGraph, pass_graph_map: &mut PassGraphMap) {
	if let (Ok(from), Ok(to)) = (rg.before_nodes(graph_id.clone()), rg.after_nodes(graph_id.clone())) {
		let from: Vec<NodeId> = Vec::from(from);
		let to: Vec<NodeId> = Vec::from(to);
		if let Ok(graph_id) = rg.remove_node(graph_id) {
			pass_graph_map.remove(graph_id);

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
