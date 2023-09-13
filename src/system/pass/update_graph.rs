use bevy_ecs::{
    prelude::{Entity, RemovedComponents, With},
    query::{Added, Changed, Or, ReadOnlyWorldQuery},
    system::{ParamSet, Query, ResMut},
};
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};
use pi_bevy_render_plugin::{NodeId, PiRenderGraph};
use pi_slotmap::Key;

use crate::{
    components::{
        calc::{InPassId, RenderContextMark},
        pass_2d::{Camera, GraphId, ParentPassId, PostProcessInfo},
        user::{Canvas, AsImage},
    },
    system::{pass::pass_graph_node::Pass2DNode, draw_obj::calc_text::IsRun}, resource::PassGraphMap,
};

pub fn update_graph(
    mut pass_query: ParamSet<(
        Query<(&mut GraphId, Entity, &ParentPassId, &PostProcessInfo), (Or<(Added<Camera>, Changed<RenderContextMark>)>, With<Camera>)>,
        (
			Query<(&ParentPassId, &GraphId, Option<&AsImage>), (Or<(Changed<ParentPassId>, Changed<AsImage>)>, With<Camera>)>, 
			Query<(&ParentPassId, &GraphId), With<Camera>>,
			Query<&GraphId>
		),
    )>,
    mut del: RemovedComponents<Camera>,
    canvas_query: Query<(&Canvas, &InPassId, Option<&AsImage>), Changed<Canvas>>,
    inpass_query: Query<&ParentPassId>,
    mut rg: ResMut<PiRenderGraph>,
	mut pass_graph_map: OrInitResMut<PassGraphMap>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // 创建渲染图节点
    // 插入Draw2DList
    for (mut graph_id, entity, parent_passs_id, post_info) in pass_query.p0().iter_mut() {
		log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node1, entity={entity:?}, has_effect={:?}, parent_passs_id={:?}", post_info.has_effect(), parent_passs_id);
        if post_info.has_effect() || pi_null::Null::is_null(&parent_passs_id.0) {
            // 存在后处理效果，或者节点本身是根节点， 才能成为一个渲染节点
            if !graph_id.0.is_null() {
                continue;
            }

            let graph_node_id = match rg.add_node(format!("Pass2D_{:?}", entity), Pass2DNode::new(entity)) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("node: {:?}, {:?}", format!("Pass2D_{:?}", entity), e);
                    return;
                }
            };
			pass_graph_map.insert(graph_node_id, entity);
            log::debug!(entity=format!("entity_{:?}", entity).as_str();  "add graph node, entity: {entity:?}: {graph_node_id:?}");

            *graph_id = GraphId(graph_node_id);
        } else {
            if graph_id.0.is_null() {
                continue;
            }

            if let Ok(graph_id) = rg.remove_node(graph_id.0) {
				pass_graph_map.remove(&graph_id);
			}
            *graph_id = GraphId(NodeId::null());
        }
    }

    // 移除渲染图节点
    for id in del.iter() {
		log::debug!(entity=format!("entity_{:?}", id).as_str(); "remove graph node, entity={id:?}");
		if let Ok(graph_id) = rg.remove_node(format!("Pass2D_{:?}", id)) {
			pass_graph_map.remove(&graph_id);
		}
    }

    let p2 = pass_query.p1();
    // 父修改设置图节点依赖
    for (parent_id, graph_id, as_image) in p2.0.iter() {
        if graph_id.0.is_null() {
            continue;
        }

        if pi_null::Null::is_null(&parent_id.0) {
            if let Err(e) = rg.set_finish(**graph_id, true) {
                log::error!("{:?}", e);
            }
			// 根节点忽略post_process
        } else {
            let parent_graph_id = get_to(***parent_id, &p2.1);
			let id = type_to_post_process(**graph_id, as_image, &p2.2, &mut rg);

            // 建立父子依赖关系，使得子pass先渲染
            log::debug!("add_depend======{:?}, {:?}", id, parent_graph_id);
            if let Err(e) = rg.add_depend(id, parent_graph_id) {
                log::error!("{:?}", e);
            }
        }
    }

    // canvas的图节点id由外部系统设置
    for (canvas, in_pass_id, as_image) in canvas_query.iter() {
        if let Ok(from_graph_id) = p2.2.get(canvas.0) {
			let id = type_to_post_process(**from_graph_id, as_image, &p2.2, &mut rg);
            let mut in_pass_id = **in_pass_id;
            loop {
                if let Ok(to_graph_id) = p2.2.get(*in_pass_id) {
                    if !to_graph_id.is_null() {
                        if let Err(e) = rg.add_depend(id, **to_graph_id) {
                            log::error!("add_depend fail, {:?}", e);
                        }
                        break;
                    }
                }
                if let Ok(parent_pass_id) = inpass_query.get(*in_pass_id) {
                    in_pass_id = **parent_pass_id;
                } else {
                    break;
                }
            }
        }
    }
}

// 如果存在后处理，连接到后处理
fn type_to_post_process(id: NodeId, as_image: Option<&AsImage>, graph_id_query: &Query<&GraphId>, rg: &mut PiRenderGraph) -> NodeId {
	if let Some(r) = as_image {
		if let Ok(post_process_graph) = graph_id_query.get(*r.post_process) {
			if !post_process_graph.is_null() {
				log::debug!("add_depend======{:?}, {:?}", id, **post_process_graph);
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

pub fn get_to<'w, 's, F: ReadOnlyWorldQuery>(parent_id: Entity, query: &Query<(&'w ParentPassId, &'s GraphId), F>) -> NodeId {
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
