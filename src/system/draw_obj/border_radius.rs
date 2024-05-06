
use pi_ecs_macros::setup;
use pi_ecs::prelude::{Changed, Query, Write};
use pi_ecs::prelude::{Deleted, Or, OrDefault, SingleResMut};
use pi_render::rhi::dyn_uniform_buffer::{Group, Bind};
use pi_style::style::BorderRadius;

use crate::components::calc::LayoutResult;
use crate::components::draw_obj::{BoxType, VSDefines, FSDefines};
use crate::resource::draw_obj::DynUniformBuffer;

use crate::shaders::image::{UiMaterialBind, UiMaterialGroup, ClipSdfUniform};
use crate::utils::tools::{cal_content_border_radius, cal_border_radius};
use crate::components::{
	calc::DrawList,
	draw_obj::{DrawObject, DrawState},
	user::Node,
};
lazy_static! {
	static ref BORDER_RADIUS: String = "BORDER_RADIUS".to_string();
}

pub struct CalcBorderRadius;

#[setup]
impl CalcBorderRadius {
    /// 创建RenderObject，用于渲染背景颜色
    #[system]
    pub fn calc_background_image(
		query_delete: Query<
			'static,
			'static,
			Node,
			(
				Option<&'static BorderRadius>,
				&'static DrawList,
			),
			Deleted<BorderRadius>,
		>,
        query: Query<
			'static,
			'static,
			Node,
			(
				&'static BorderRadius,
				&'static LayoutResult,
				&'static DrawList,
			),
			(
				Changed<BorderRadius>,
				Changed<LayoutResult>,
			),
		>,
		
		mut query_draw: Query<'static, 'static, DrawObject, (Write<DrawState>, OrDefault<BoxType>, Write<VSDefines>, Write<FSDefines>)>,

		mut dyn_uniform_buffer: SingleResMut<'static, DynUniformBuffer>,
    ) {
        for (border_radius, render_list) in query_delete.iter() {
            // border_radius不存在时，删除对应DrawObject的uniform
            if border_radius.is_some() {
                continue;
            };

			for i in render_list.iter() {
				if let Some((_draw_state, _box_type, mut vs_defines, mut fs_defines)) = query_draw.get_mut(i.clone()) {
					if let Some(vs_defines) = vs_defines.get_mut() {
						vs_defines.remove(&*BORDER_RADIUS);
					}
					if let Some(fs_defines) = fs_defines.get_mut() {
						fs_defines.remove(&*BORDER_RADIUS);
					}
				}
			}

			
        }

		for (border_radius, layout, render_list) in query.iter() {
			if render_list.len() == 0 {
				continue;
			}

			let border_radius = cal_border_radius(border_radius, layout);
			for i in render_list.iter() {
				if let Some((mut draw_state, box_type, mut vs_defines, mut fs_defines)) = query_draw.get_mut(i.clone()) {
					if let Some(draw_state_item) = draw_state.get_mut() {
						
						let (width, height)  = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
						let (x, y, z, w) = match box_type {
							BoxType::BorderRect | BoxType::ContentRect  => (
								width/2.0, 
								height/2.0, 
								width, 
								height, 
							),
							BoxType::BorderNone | BoxType::ContentNone => (
								width/2.0, 
								height/2.0, 
								1.0, 1.0
							),
							BoxType::Border => continue, // 渲染边框，不需要额外添加圆角的uniform
						};

						// 修改宏
						let (vs_defines_item, fs_defines_item) = (vs_defines.get_mut_or_default(), fs_defines.get_mut_or_default());
						if vs_defines_item.insert(BORDER_RADIUS.clone()) {
							vs_defines.notify_modify()
						}
						if fs_defines_item.insert(BORDER_RADIUS.clone()) {
							fs_defines.notify_modify()
						}

						// 修改uniform
						let dyn_offset = draw_state_item.bind_groups.get_group(UiMaterialGroup::id()).unwrap().get_offset(UiMaterialBind::index()).unwrap();
						let temp;
						let border_radius = match box_type {
							BoxType::ContentNone | BoxType::ContentRect  => {
								temp = cal_content_border_radius(&border_radius, (
									layout.border.top,
									layout.border.right,
									layout.border.bottom,
									layout.border.left,
								));
								&temp
							},
							BoxType::BorderNone | BoxType::BorderRect => &border_radius, 
							_ => continue,
						};
						dyn_uniform_buffer.set_uniform(dyn_offset, &ClipSdfUniform(&[
							x, y, z, w, 
							width/2.0, height/2.0, 0.0, 0.0,
							border_radius.y[0], border_radius.x[0], border_radius.x[1], border_radius.y[1],
							border_radius.y[2], border_radius.x[2], border_radius.x[3], border_radius.y[3],
						]));
						draw_state.notify_modify();
					}
				}
			}
		}
    }
}

