use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::Query;
use bevy_ecs::prelude::DetectChangesMut;
use pi_bevy_ecs_extend::prelude::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::{BoxType, PipelineMeta};

use crate::components::{calc::DrawList, draw_obj::DrawState, user::BorderRadius};
use crate::shader::{sdf::BORDER_RADIUS_DEFINE, ui_meterial::ClipSdfUniform};
use crate::utils::tools::{cal_border_radius, cal_content_border_radius};

use super::calc_text::IsRun;

/// 设置圆角Unifrom
pub fn calc_border_radius(
    mut remove: RemovedComponents<BorderRadius>,
    query_delete: Query<(Option<&'static BorderRadius>, &'static DrawList)>,
    query: Query<
        (&'static BorderRadius, &'static LayoutResult, &'static DrawList),
        Or<(Changed<BorderRadius>, Changed<LayoutResult>, Changed<DrawList>)>,
    >,

    mut query_draw: Query<(&mut DrawState, OrDefault<BoxType>, &mut PipelineMeta)>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    for del in remove.iter() {
        if let Ok((border_radius, render_list)) = query_delete.get(del) {
            // border_radius不存在时，删除对应DrawObject的uniform
            if border_radius.is_some() {
                continue;
            };

            for i in render_list.iter() {
                if let Ok((_draw_state, _box_type, mut pipeline_meta)) = query_draw.get_mut(i.id) {
                    pipeline_meta.defines.remove(&*BORDER_RADIUS_DEFINE);
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
            if let Ok((mut draw_state, box_type, mut pipeline_meta)) = query_draw.get_mut(i.id) {
				let (width, height) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
				let (content_width, content_height) = (width - layout.border.left - layout.border.right, height - layout.border.top - layout.border.bottom);
			
                let (x, y, z, w, width, height) = match box_type {
					BoxType::BorderUnitRect => (width / 2.0, height / 2.0, width, height, width, height),
					BoxType::BorderNone => (width / 2.0, height / 2.0,  1.0, 1.0, width, height),
                    BoxType::ContentUnitRect | BoxType::PaddingUnitRect => (content_width / 2.0, content_height / 2.0, content_width, content_height, content_width, content_height),
					BoxType::ContentNone | BoxType::PaddingNone | BoxType::ContentRect => (content_width / 2.0, content_height / 2.0, 1.0, 1.0, content_width, content_height),
                    BoxType::Border => continue,    // 渲染边框，不需要额外添加圆角的uniform
                    BoxType::NotChange => continue, // 不改变
                };

                // 修改宏
                if pipeline_meta.bypass_change_detection().defines.insert(BORDER_RADIUS_DEFINE.clone()) {
                    pipeline_meta.set_changed()
                }

                // 修改uniform
                let temp;
                let border_radius = match box_type {
                    BoxType::ContentNone | BoxType::ContentUnitRect | BoxType::PaddingUnitRect => {
                        temp = cal_content_border_radius(
                            &border_radius,
                            (layout.border.top, layout.border.right, layout.border.bottom, layout.border.left),
                        );
                        &temp
                    }
                    BoxType::BorderNone | BoxType::BorderUnitRect => &border_radius,
                    _ => continue,
                };
                draw_state.bindgroups.set_uniform(&ClipSdfUniform(&[
                    x,
                    y,
                    z,
                    w,
                    width / 2.0,
                    height / 2.0,
                    0.0,
                    0.0,
                    border_radius.y[0],
                    border_radius.x[0],
                    border_radius.x[1],
                    border_radius.y[1],
                    border_radius.y[2],
                    border_radius.x[2],
                    border_radius.x[3],
                    border_radius.y[3],
                ]));
            }
        }
    }
}
