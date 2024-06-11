use pi_world::fetch::Ticker;
use pi_world::filter::{Changed, Or};
use pi_world::prelude::{With, Query, SingleResMut, Entity, Plugin, IntoSystemConfigs, SingleRes};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::PiRenderGraph;
use pi_bevy_render_plugin::render_cross::GraphId;
use pi_null::Null;

use crate::components::calc::{InPassId, DrawList, WorldMatrix, LayoutResult};
use crate::components::draw_obj::{CanvasMark, InstanceIndex, FboInfo};
use crate::components::pass_2d::ParentPassId;
use crate::components::user::{Canvas, AsImage};
use crate::resource::CanvasRenderObjType;
use crate::resource::draw_obj::{InstanceContext, LastGraphNode};
use crate::shader1::meterial::{RenderFlagType, TyUniform};
use crate::system::draw_obj::set_box;
use crate::system::pass::update_graph::{type_to_post_process, update_graph};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use super::calc_text::IsRun;
use super::life_drawobj::{self, update_render_instance_data};


pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
		// .add_frame_event::<ComponentEvent<Changed<Canvas>>>()
		.add_system(UiStage, 
			life_drawobj::draw_object_life_new::<
				Canvas,
				CanvasRenderObjType,
				(CanvasMark, GraphId, FboInfo),
				{ CANVAS_ORDER },
			>
				.in_set(UiSystemSet::LifeDrawObject),
		)
		.add_system(
			UiStage, 
			calc_canvas
				.in_set(UiSystemSet::PrepareDrawObj)
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

/// 设置canvas的实例数据
pub fn calc_canvas(
	mut canvas_query: Query<(&DrawList, &WorldMatrix, &LayoutResult), (Or<(Changed<Canvas>, Changed<WorldMatrix>)>, With<Canvas>)>,
	mut instances: OrInitSingleResMut<InstanceContext>,
	mut instance_index_query: Query<&InstanceIndex, With<CanvasMark>>,
	render_type: OrInitSingleRes<CanvasRenderObjType>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    for (draw_list, world_matrix, layout) in canvas_query.iter_mut() {
		
		// let (canvas_changed, world_matrix_changed, layout_result_changed) = (canvas.is_changed(),  world_matrix.is_changed(), layout.is_changed());
		// log::trace!("set canvas data1==========={:?}, {:?} {:?} {:?}, {:?}, {:?}, {:?}",  world_matrix, canvas_changed, world_matrix_changed, layout_result_changed, draw_list.get_one(***render_type), render_type, draw_list);
		// if canvas_changed || world_matrix_changed || layout_result_changed {
			// 设置世界矩阵、布局uniform
			if let Some(draw_entity) = draw_list.get_one(***render_type) {
				if let Ok(instance_index) = instance_index_query.get_mut(draw_entity.id) {
					// 节点可能设置为dispaly none， 此时instance_index可能为Null
					if pi_null::Null::is_null(&instance_index.0.start) {
						continue;
					}
					let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
					let mut render_flag = instance_data.get_render_ty();
					render_flag |= 1 << RenderFlagType::Uv as usize;
					// instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
					// instance_data.set_data(&BoxUniform(layout.padding_box().as_slice()));
					set_box(&world_matrix, &layout.padding_aabb(), &mut instance_data);
					instance_data.set_data(&TyUniform(&[render_flag as f32]));

					log::trace!("set canvas data==========={:?}, {:?}", instance_index,  world_matrix);
				}
			}

		// }
    }
}

/// 为canvas节点添加图依赖结构
pub fn calc_canvas_graph(
	mut canvas_query: Query<(&mut Canvas, &InPassId, Entity)>,
	canvas_other_query: Query<Option<&AsImage>>,
	graph_id_query: Query<(Ticker<&GraphId>, Ticker<&InPassId>)>,
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
        if let Ok((from_graph_id, in_pass)) = graph_id_query.get(canvas.id) {
			if !from_graph_id.is_changed() && !in_pass.is_changed() {
				continue; // 未改变， 什么也不做
			}
			// let (from_graph_id_changed, in_pass_id_changed) = (from_graph_id.is_changed(), in_pass_id.is_changed());
			// log::trace!("calc_canvas_graph, graph_id={:?}, from_graph_id_changed={:?}, in_pass_id_changed={:?}", canvas.id, from_graph_id_changed, in_pass_id_changed);
			// if !from_graph_id_changed && !in_pass_id_changed {
			// 	continue; // 未改变， 什么也不做
			// }

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



