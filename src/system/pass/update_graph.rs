use bevy::ecs::{
    prelude::{Entity, RemovedComponents},
    query::{Added, Changed, Or},
    system::{ParamSet, Query, ResMut},
};
use pi_bevy_render_plugin::{NodeId, PiRenderGraph};
use pi_slotmap::Key;

use crate::{
    components::{
        calc::{InPassId, RenderContextMark},
        pass_2d::{Camera, GraphId, ParentPassId, PostProcessList},
        user::Canvas,
    },
    system::pass::pass_graph_node::Pass2DNode,
};

pub fn update_graph(
    mut pass_query: ParamSet<(
        Query<(&mut GraphId, Entity, &ParentPassId, &PostProcessList), Or<(Added<Camera>, Changed<RenderContextMark>)>>,
        Query<&GraphId>,
        (
			Query<(&ParentPassId, &GraphId), Changed<ParentPassId>>,
			Query<(&ParentPassId, &GraphId)>,
		),
    )>,
    mut del: RemovedComponents<Camera>,
    canvas_query: Query<(&Canvas, &InPassId), Changed<Canvas>>,
    mut rg: ResMut<PiRenderGraph>,
) {
    // 创建渲染图节点
    // 插入Draw2DList
    for (mut graph_id, entity, parent_passs_id, post_list) in pass_query.p0().iter_mut() {
        if post_list.has_effect() || pi_null::Null::is_null(&parent_passs_id.0){
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

            *graph_id = GraphId(graph_node_id);
        } else {
            if graph_id.0.is_null() {
                continue;
            }

            let _ = rg.remove_node(graph_id.0);
            *graph_id = GraphId(NodeId::null());
        }
    }

    // 移除渲染图节点
    for id in del.iter() {
        let _ = rg.remove_node(format!("Pass2D_{:?}", id));
    }

    let p2 = pass_query.p2();
    // 父修改设置图节点依赖
    for (parent_id, graph_id) in p2.0.iter() {
        if graph_id.0.is_null() {
            continue;
        }
        if pi_null::Null::is_null(&parent_id.0) {
            if let Err(e) = rg.set_finish(**graph_id, true) {
                log::error!("{:?}", e);
            }
        } else {
            if let Ok((mut parent_id, mut parent_graph_id)) = p2.1.get(***parent_id) {
                // 父的pass2d不存在图节点， 继续找父
                while parent_graph_id.0.is_null() {
                    if let Ok((parent_id1, parent_graph_id1)) = p2.1.get(***parent_id) {
                        parent_id = parent_id1;
                        parent_graph_id = parent_graph_id1;
                    } else {
                        break;
                    }
                }

                if parent_graph_id.0.is_null() {
                    continue;
                }

                // 建立父子依赖关系，使得子pass先渲染
                if let Err(e) = rg.add_depend(**graph_id, **parent_graph_id) {
                    log::error!("{:?}", e);
                }
            }
        }
    }

    // canvas的图节点id由外部系统设置
    let graph_id_query = pass_query.p1();
    for (canvas, in_pass_id) in canvas_query.iter() {
		if let Ok(from_graph_id) = graph_id_query.get(canvas.0) {
            if let Ok(to_graph_id) = graph_id_query.get(***in_pass_id) {
				if let Err(e) = rg.add_depend(**from_graph_id, **to_graph_id) {
					log::error!("add_depend fail, {:?}", e);
				}
			}
        }
    }
}
