
/// canvas功能
/// 0. draw_object_life_new系统， 为canvas创建对应的DrawObj
/// 1. calc_canvas_graph检测canvas对应GraphId和InPassId组件的变化， 将canvas的GraphId链接到当前canvas所在的图节点上
/// 2. calc_canvas_graph将canvas对应GraphId连接到gui最终图节点上
/// 3. 在pass_graph_node图节点的build中， 遍历所有cnavas，将其GraphId对应的fbo设置在canvas draw obj 的OutTarget上以便后续渲染（需要步骤2支持，才能找到对应的fbo）
/// 4. canvsas组件删除后， 需要删除对应的依赖关系（这里由外部系统保证， 当外部系统删除了对应的GraphId对应的图节点，依赖关系也随之删除了）
use pi_world::prelude::{Query, SingleResMut, Entity, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;
use pi_bevy_render_plugin::asimage_url::RenderTarget as RenderTarget1;

use pi_bevy_render_plugin::{NodeId, PiRenderGraph};
use pi_bevy_render_plugin::render_cross::GraphId;
use pi_null::Null;

use crate::components::calc::{CanvasGraph, DrawInfo, DrawList, InPassId};
use crate::components::draw_obj::{BoxType, CanvasMark, FboInfo};
use crate::components::user::Canvas;
use crate::resource::{CanvasRenderObjType, GlobalDirtyMark, OtherDirtyType};
use crate::system::base::pass::pass_graph_node::CustomCopyNode;
use crate::system::base::pass::update_graph::update_graph;
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
				(CanvasMark, GraphId, FboInfo, RenderTarget1),
				{ CANVAS_ORDER },
				{ BoxType::Padding },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.before(calc_canvas_graph),
		)
		.add_system(
			UiStage, 
			calc_canvas_graph
				.after(update_graph)
				.before(update_render_instance_data)
				.in_set(UiSystemSet::IsRun)
				
		)
		;
    }
}

pub const CANVAS_ORDER: u8 = 6;

/// 为canvas节点添加图依赖结构
/// 同时， 设置ByCross标记
pub fn calc_canvas_graph(
	mut canvas_query: Query<(&Canvas, &mut CanvasGraph, &InPassId, Entity, &DrawList)>,
	// mut canvas_other_query: Query<Option<&mut AsImage>>,
	graph_id_query: Query<&GraphId>,
	mut draw_info: Query<&mut DrawInfo>,

	canvas_render_type: SingleResMut<CanvasRenderObjType>,
	mut global_dirty_mark: SingleResMut<GlobalDirtyMark>,
	mut rg: SingleResMut<PiRenderGraph>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

	// canvas的图节点id由外部系统设置
    for (canvas, mut canvas_graph, in_pass_id, entity, draw_list) in canvas_query.iter_mut() {
		// 重新设置渲染数量
		let render_obj = draw_list.get_one(**canvas_render_type).unwrap();
		let mut draw_info = draw_info.get_mut(render_obj.id).unwrap();
		let is_by_cross = draw_info.is_by_cross();
		let new_is_by_cross = !canvas.id.is_null() && canvas.by_draw_list;
		if new_is_by_cross != is_by_cross {
			draw_info.set_by_cross(new_is_by_cross);
			global_dirty_mark.mark.set(OtherDirtyType::CanvasBylist as usize, new_is_by_cross);
			
		}

		let canvas_graph_id = match graph_id_query.get(canvas.id) {
			Ok(r) => r.0,
			Err(_) => Null::null(),
		};
		let in_pass_id = **in_pass_id;
		let parent_pass_id = if let Ok(to_graph_id) = graph_id_query.get(*in_pass_id) {
			to_graph_id.0
		} else {
			// TODO
			// find_parent_graph_id(in_pass_id, query)
			unreachable!()
		};

		if canvas_graph.copy_graph_id.is_null() {
			// 该节点用于将后处理结果拷贝回RenderTaget，并添加copy节点与父的链接关系
			// 无论图节点是否存在， 始终保持copy节点存在并build， 这样当外部系统不设置graph时，canvas能正确的不渲染（在copy节点， 判断了输入没有target， 设置为无效渲染）
			canvas_graph.copy_graph_id = rg.add_node_not_run(format!("Canvas_CopyTarget_{:?}", entity), CustomCopyNode::new(render_obj.id), NodeId::default(), Null::null()).unwrap();
			let _ = rg.add_depend(canvas_graph.copy_graph_id, parent_pass_id);
		}

		if canvas_graph.old_canvas_graph_id == canvas_graph_id {
			continue;
		}

        if !canvas_graph_id.is_null() {
			let old_canvas_graph_id = canvas_graph.old_canvas_graph_id;
			canvas_graph.old_canvas_graph_id = canvas_graph_id;
			// let id = type_to_post_process(**from_graph_id, as_image.as_mut().map(|r| {r.bypass_change_detection()}), &graph_id_query1, &mut rg, Default::default());


			if !old_canvas_graph_id.is_null() {
				// 移除原有依赖
				let _ = rg.remove_depend(old_canvas_graph_id, parent_pass_id);
				let _ = rg.remove_depend(old_canvas_graph_id, canvas_graph.copy_graph_id);
			}
			log::debug!("canvas========: {:?}", (entity, &in_pass_id, old_canvas_graph_id, canvas_graph_id, canvas_graph.copy_graph_id, old_canvas_graph_id));
			let _ = rg.add_depend(canvas_graph_id, parent_pass_id);
			let _ = rg.add_depend(canvas_graph_id, canvas_graph.copy_graph_id);
			
			
        } else if !canvas_graph.old_canvas_graph_id.is_null(){
			let _ = rg.remove_depend(canvas_graph.old_canvas_graph_id, parent_pass_id);
			let _ = rg.remove_depend(canvas_graph.old_canvas_graph_id, canvas_graph.copy_graph_id);
			// let _ = rg.remove_depend(canvas_graph.copy_graph_id, parent_pass_id); // 这里不能移除， 使得copy_graph_id在判断输入为空时， 设置canvas渲染不可见

			// log::warn!("remove_depend======={:?}", (pre_graph_id, canvas_graph.to_graph_id));
			canvas_graph.old_canvas_graph_id = NodeId::null();
		}
    }
}



