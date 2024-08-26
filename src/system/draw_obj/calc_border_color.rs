//! 圆角从有到删除，没有正确处理顶点（TODO）

use pi_style::style::StyleType;
use pi_world::event::ComponentRemoved;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};
use pi_world::single_res::SingleRes;

use crate::components::calc::{style_bit, DrawList, LayoutResult, StyleBit, StyleMarkType};
use crate::components::draw_obj::{BorderColorMark, BoxType, InstanceIndex};
use crate::components::user::BorderRadius;
use crate::components::user::BorderColor;
use crate::resource::{BorderColorRenderObjType, GlobalDirtyMark};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{BorderColorUniform, RenderFlagType, TyUniform, BorderWidthUniform};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use super::calc_text::IsRun;
use super::life_drawobj;

pub struct BorderColorPlugin;

impl Plugin for BorderColorPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		 // BorderColor功能
		app
		// .add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
		.add_system(
			UiStage, 
			life_drawobj::draw_object_life_new::<
				BorderColor,
				BorderColorRenderObjType,
				(BorderColorMark, ),
				{ BORDER_COLOR_ORDER },
				{ BoxType::Border },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.run_if(border_color_life_change)
				.before(calc_border_color),
		)
		.add_system(
			UiStage, 
			calc_border_color
				.after(super::super::node::world_matrix::cal_matrix)
				.in_set(UiSystemSet::PrepareDrawObj)
				.run_if(border_color_change)
		);
    }
}

pub const BORDER_COLOR_ORDER: u8 = 4;

lazy_static! {
	pub static ref BORDER_IMAGE_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BorderColor as usize)
		.set_bit(StyleType::BorderRadius as usize);
}

pub fn border_color_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*BORDER_IMAGE_DIRTY)
}

pub fn border_color_life_change(mark: SingleRes<GlobalDirtyMark>, removed: ComponentRemoved<BorderColor>) -> bool {
	let r = removed.len() > 0 || mark.mark.get(StyleType::BorderColor as usize).map_or(false, |display| {*display == true});
	removed.mark_read();
	r
}

/// 设置边框颜色的顶点、索引、和边框颜色uniform
pub fn calc_border_color(
	mut instances: OrInitSingleResMut<InstanceContext>,
    query: Query<
        (
			&BorderColor,
            &LayoutResult,
			&DrawList
        ),
        Or<(Changed<BorderColor>, Changed<BorderRadius>, Changed<LayoutResult>)>, // 圆角和Border颜色， 都需要设置border宽度
    >,
	mut query_draw: Query<&InstanceIndex, With<BorderColorMark>>,
	render_type: OrInitSingleRes<BorderColorRenderObjType>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	for (border_color, layout, draw_list) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		if let Ok(instance_index) = query_draw.get_mut(draw_id) {
			// 节点可能设置为dispaly none， 此时instance_index可能为Null
			if pi_null::Null::is_null(&instance_index.0.start) {
				continue;
			}
			
			let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
			let mut render_flag = instance_data.get_render_ty();
			// if border_color.is_changed() {
				// 颜色改变，重新设置color_group
				instance_data.set_data(&BorderColorUniform(&[border_color.x, border_color.y, border_color.z, border_color.w]));
			// }

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			render_flag |= 1 << RenderFlagType::Color as usize;
			render_flag |= 1 << RenderFlagType::Border as usize;
			instance_data.set_data(&TyUniform(&[render_flag as f32]));

			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
			// if is_add || layout.is_changed() {
			// 	instance_data.set_data(&BoxUniform(layout.border_box().as_slice()));
			// 	instance_data.set_data(&BorderWidthUniform([
			// 		layout.border.top,
			// 		layout.border.right,
			// 		layout.border.bottom,
			// 		layout.border.left,].as_slice()));
			// }

			// if is_add || world_matrix.is_changed() || layout.is_changed() {
				// set_box(&world_matrix, &layout.border_aabb(), &mut instance_data);
			// }
			// if is_add || layout.is_changed() {
				instance_data.set_data(&BorderWidthUniform([
					layout.border.top,
					layout.border.right,
					layout.border.bottom,
					layout.border.left,].as_slice()));
			// }
		}
	}
}
