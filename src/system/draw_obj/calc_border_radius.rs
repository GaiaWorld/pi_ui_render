/// 为圆角设置渲染数据

use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::Query;
use bevy_ecs::prelude::DetectChangesMut;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::InstanceIndex;

use crate::components::{calc::DrawList, user::BorderRadius};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{RenderFlagType, ClipRadiusUniform, ClipRectRoundUniform, TyUniform};
use crate::utils::tools::cal_border_radius;

use super::calc_text::IsRun;

/// 设置圆角Unifrom
pub fn calc_border_radius(
    mut remove: RemovedComponents<BorderRadius>, 
	mut instances: OrInitResMut<InstanceContext>,
    query_delete: Query<(Option<&'static BorderRadius>, &'static DrawList)>,
    query: Query<
        (&'static BorderRadius, &'static LayoutResult, &'static DrawList),
        Or<(Changed<BorderRadius>, Changed<LayoutResult>, Changed<DrawList>)>,
    >,

    mut query_draw: Query<&InstanceIndex>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
	let instances = instances.bypass_change_detection();
    for del in remove.iter() {
        if let Ok((border_radius, render_list)) = query_delete.get(del) {
            // border_radius不存在时，删除对应DrawObject的uniform
            if border_radius.is_some() {
                continue;
            };

            for i in render_list.iter() {
                if let Ok(instance_index) = query_draw.get_mut(i.id) {
					let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
					let mut render_flag = instance_data.get_render_ty();
					render_flag &= !(1 << RenderFlagType::ClipRectRadius as usize);
                    instance_data.set_data(&TyUniform(&[render_flag as f32]));
                }
            }
        }
    }

	
    for (border_radius, layout, render_list) in query.iter() {
        if render_list.len() == 0 {
            continue;
        }
        let border_radius = cal_border_radius(border_radius, &layout.rect);
        for i in render_list.iter() {
            if let Ok(instance_index) = query_draw.get_mut(i.id) {
				// 节点可能设置为dispaly none， 此时instance_index可能为Null
				if pi_null::Null::is_null(&instance_index.0.start) {
					continue;
				}

				let (width, height) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);

                // 修改uniform
				let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
				let mut render_flag = instance_data.get_render_ty();
				instance_data.set_data(&ClipRectRoundUniform([
                    width / 2.0,
                    height / 2.0,
                    width / 2.0,
                    height / 2.0].as_slice()));
				instance_data.set_data(&ClipRadiusUniform([
					border_radius.y[0],
                    border_radius.x[0],
                    border_radius.x[1],
                    border_radius.y[1],
                    border_radius.y[2],
                    border_radius.x[2],
                    border_radius.x[3],
                    border_radius.y[3]].as_slice()));
				render_flag |= 1 << RenderFlagType::ClipRectRadius as usize;
				instance_data.set_data(&TyUniform(&[render_flag as f32]));
            }
        }
    }
}
