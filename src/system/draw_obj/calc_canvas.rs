
use pi_world::fetch::Ticker;
use pi_world::prelude::{Query, SingleResMut, Entity, Plugin, IntoSystemConfigs, SingleRes};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use pi_bevy_render_plugin::PiRenderGraph;
use pi_bevy_render_plugin::render_cross::GraphId;
use pi_null::Null;

use crate::components::calc::InPassId;
use crate::components::draw_obj::{BoxType, CanvasMark, FboInfo};
use crate::components::pass_2d::ParentPassId;
use crate::components::user::{Canvas, AsImage};
use crate::resource::CanvasRenderObjType;
use crate::resource::draw_obj::LastGraphNode;
use crate::system::base::pass::update_graph::{type_to_post_process, update_graph};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::{draw_object_life_new, update_render_instance_data};
use crate::resource::IsRun;


pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
		.add_system(UiStage, 
			draw_object_life_new::<
				Canvas,
				CanvasRenderObjType,
				(CanvasMark, GraphId, FboInfo),
				{ CANVAS_ORDER },
				{ BoxType::Padding },
			>
				.in_set(UiSystemSet::LifeDrawObject)
		)
		.add_system(
			UiStage, 
			calc_canvas_graph
				.after(update_graph)
				.before(update_render_instance_data)
				
		)
		;
    }
}

pub const CANVAS_ORDER: u8 = 6;


/// 为canvas节点添加图依赖结构
pub fn calc_canvas_graph(
	mut canvas_query: Query<(&mut Canvas, Ticker<&InPassId>, Entity)>,
	canvas_other_query: Query<Option<&AsImage>>,
	graph_id_query: Query<Ticker<&GraphId>>,
	graph_id_query1: Query<&GraphId>,
	inpass_query: Query<&ParentPassId>,

	mut rg: SingleResMut<PiRenderGraph>,
	last_graph_id: SingleRes<LastGraphNode>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

	// canvas的图节点id由外部系统设置
    for (mut canvas, in_pass_id, entity) in canvas_query.iter_mut() {
        if let Ok(from_graph_id) = graph_id_query.get(canvas.id) {
			if !from_graph_id.is_changed() && !in_pass_id.is_changed() {
				continue; // 未改变， 什么也不做
			}

			// 如果canvas关联的内容发生改变， 则设置Canvas改变
			// if from_graph_id_changed {
				canvas.set_changed();
			// }

			let as_image = match canvas_other_query.get(entity) {
				Ok(r) => r,
				Err(_) => continue,
			};

			if from_graph_id.is_null() {
				continue;
			}
			
			let id = type_to_post_process(**from_graph_id, as_image, &graph_id_query1, &mut rg);
            let mut in_pass_id = **in_pass_id;
            loop {
                if let Ok(to_graph_id) = graph_id_query1.get(*in_pass_id) {
                    if !to_graph_id.is_null() {
						log::trace!("canvas add graph depend, before={:?}, after={:?}", id, to_graph_id);
                        if let Err(e) = rg.add_depend(id, **to_graph_id) {
                            log::error!("add_depend fail, {:?}", e);
                        }
						// 把canvas节点与根节点相连，在根节点处处理canvas bingroup
						if let Err(e) = rg.add_depend(id, last_graph_id.0) {
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



