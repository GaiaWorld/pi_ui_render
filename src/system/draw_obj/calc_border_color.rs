//! 圆角从有到删除，没有正确处理顶点（TODO）

use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::{DetectChanges, Ref, Query, Changed, Or, With, DetectChangesMut};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_window::AddFrameEvent;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};

use crate::components::calc::{LayoutResult, WorldMatrix, DrawList};
use crate::components::draw_obj::{BorderColorMark, InstanceIndex};
use crate::components::user::BorderRadius;
use crate::components::user::BorderColor;
use crate::resource::BorderColorRenderObjType;
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{BorderColorUniform, RenderFlagType, TyUniform, BorderWidthUniform};
use crate::system::system_set::UiSystemSet;

use super::calc_text::IsRun;
use super::{life_drawobj, set_box};

pub struct BorderColorPlugin;

impl Plugin for BorderColorPlugin {
    fn build(&self, app: &mut bevy_app::App) {
		 // BorderColor功能
		app
		.add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
		.add_systems(
			Update, 
			life_drawobj::draw_object_life_new::<
				BorderColor,
				BorderColorRenderObjType,
				BorderColorMark,
				{ BORDER_COLOR_ORDER },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.before(calc_border_color),
		)
		.add_systems(
			Update, 
			calc_border_color
				.after(super::super::node::world_matrix::cal_matrix)
				.in_set(UiSystemSet::PrepareDrawObj)
		);
    }
}

pub const BORDER_COLOR_ORDER: u8 = 4;

/// 设置边框颜色的顶点、索引、和边框颜色uniform
pub fn calc_border_color(
	mut instances: OrInitResMut<InstanceContext>,
    query: Query<
        (
			Ref<BorderColor>,
            Ref<LayoutResult>,
            Ref<WorldMatrix>,
			&DrawList
        ),
        Or<(Changed<BorderColor>, Changed<BorderRadius>, Changed<LayoutResult>, Changed<WorldMatrix>)>,
    >,
	mut query_draw: Query<&InstanceIndex, With<BorderColorMark>>,
	render_type: OrInitRes<BorderColorRenderObjType>,
	r: OrInitRes<IsRun>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	for (border_color, layout, world_matrix, draw_list) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		if let Ok(instance_index) = query_draw.get_mut(draw_id) {
			// 节点可能设置为dispaly none， 此时instance_index可能为Null
			if pi_null::Null::is_null(&instance_index.0.start) {
				continue;
			}
			
			let mut instance_data = instances.bypass_change_detection().instance_data.instance_data_mut(instance_index.0.start);
			let mut render_flag = instance_data.get_render_ty();
			if border_color.is_changed() {
				// 颜色改变，重新设置color_group
				instance_data.set_data(&BorderColorUniform(&[border_color.x, border_color.y, border_color.z, border_color.w]));
			}

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			let is_add =  border_color.is_added();
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

			if is_add || world_matrix.is_changed() || layout.is_changed() {
				set_box(&world_matrix, &layout.border_aabb(), &mut instance_data);
			}
			if is_add || layout.is_changed() {
				instance_data.set_data(&BorderWidthUniform([
					layout.border.top,
					layout.border.right,
					layout.border.bottom,
					layout.border.left,].as_slice()));
			}
		}
	}
}
