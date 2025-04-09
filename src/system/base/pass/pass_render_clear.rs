
use pi_ecs::prelude::Query;
use pi_ecs_macros::setup;

use crate::components::pass_2d::{Camera, Pass2D};

pub struct CalcRenderClear;

/// 设置所有相机为非激活状（只有当存在脏区域时，会重新激活相机）
#[setup]
impl CalcRenderClear {
    #[system]
    pub fn calc_render<'a>(
        mut query_pass: Query<Pass2D, &mut Camera>,
        // render_dirty: Query<'a, 'a, Node, (&mut DirtyRect, &RenderDirty), With<DirtyRect>>,
		// render_dirty_mark: SingleRes<'a, RenderDirty>,
    ) {
        // 不脏，不需要组织渲染图， 也不需要渲染
		// for (global_dirty_rect, render_dirty_mark) in render_dirty.iter_mut() {
		// 	if global_dirty_rect.state == DirtyRectState::UnInit && !render_dirty_mark.0{
		// 		return ;
		// 	}
		// }

        for mut camera in query_pass.iter_mut() {
            camera.bypass_change_detection().is_active = false;
        }
    }
}