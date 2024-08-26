
use pi_style::style::{Aabb2, LinearGradientColor, Point2, StyleType};
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};
use pi_world::single_res::SingleRes;

use crate::components::calc::{style_bit, DrawList, LayoutResult, StyleBit, StyleMarkType};
use crate::components::draw_obj::{BackgroundColorMark, BoxType, InstanceIndex};
use crate::resource::{BackgroundColorRenderObjType, GlobalDirtyMark};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{GradientColorUniform, GradientPositionUniform, RenderFlagType, ColorUniform, TyUniform, GradientEndUniform};
use crate::components::user::{BackgroundColor, Color, Vector2};
use crate::shader1::InstanceData;
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use super::calc_text::IsRun;
use super::life_drawobj;

pub struct BackgroundColorPlugin;

impl Plugin for BackgroundColorPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		 // BackgroundColor功能
		app
		// .add_frame_event::<ComponentEvent<Changed<BackgroundColor>>>()
		.add_system(
			UiStage, 
			life_drawobj::draw_object_life_new::<
				BackgroundColor,
				BackgroundColorRenderObjType,
				(BackgroundColorMark, ),
				{ BACKGROUND_COLOR_ORDER },
				{ BoxType::Padding },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.before(calc_background_color),
		)
		.add_system(
			UiStage, 
			calc_background_color
				.after(super::super::node::world_matrix::cal_matrix)
				.in_set(UiSystemSet::PrepareDrawObj)
				.run_if(background_color_change)
		);
    }
}

pub const BACKGROUND_COLOR_ORDER: u8 = 2;

lazy_static! {
	pub static ref BACKGROUND_COLOR_DATA_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BackgroundColor as usize);
}

pub fn background_color_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	let r = mark.mark.has_any(&*BACKGROUND_COLOR_DATA_DIRTY);
	r
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_background_color(
	mut instances: OrInitSingleResMut<InstanceContext>,
    query: Query<(&BackgroundColor, &LayoutResult, &DrawList), Changed<BackgroundColor>>,
    mut query_draw: Query<&InstanceIndex, With<BackgroundColorMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	for (background_color, layout, draw_list) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		if let Ok(instance_index) = query_draw.get_mut(draw_id) {
			log::debug!("calc_background_color, draw_id={:?}, instance_index={:?}, background_color={:?}", draw_id, instance_index, &background_color);

			// 节点可能设置为dispaly none， 此时instance_index可能为Null
			if pi_null::Null::is_null(&instance_index.0.start) {
				continue;
			}

			// let mut instance_data = instances.bypass_change_detection().instance_data.instance_data_mut(instance_index.0.start);
			let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
			let mut render_flag = instance_data.get_render_ty();
			
			match &background_color.0 {
				Color::RGBA(color) => {
					// let r1 = query2.get(Entity::from_raw(13));
					// 颜色改变，重新设置color_group
					instance_data.set_data(&ColorUniform(&[color.x, color.y, color.z, color.w]));
	
					render_flag |= 1 << RenderFlagType::Color as usize;
					render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
				}
				Color::LinearGradient(color) => set_linear_gradient_instance_data(
					color, 
					&Aabb2::new(
						Point2::new( layout.border.left, layout.border.top), 
						Point2::new( layout.rect.right - layout.border.right - layout.rect.left, layout.rect.bottom - layout.border.bottom - layout.rect.top)
					), 
					&mut instance_data, 
					&mut render_flag
				),
			};
			instance_data.set_data(&TyUniform(&[render_flag as f32]));

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
			// if is_add || layout.is_changed() {
			// 	instance_data.set_data(&BoxUniform(layout.padding_box().as_slice()));
			// }

			// if is_add || layout.is_changed() || world_matrix.is_changed(){
			// 	set_box(&world_matrix, &layout.padding_aabb(), &mut instance_data);
			// }

			// set_box(&world_matrix, &layout.padding_aabb(), &mut instance_data);
		}
	}
}

// 渐变颜色实例数据
pub fn set_linear_gradient_instance_data(color: &LinearGradientColor, aabb: &Aabb2, instance_data: &mut InstanceData, render_flag: &mut usize) {
	let mut colors: [f32; 16] = [0.0; 16];
	let mut positions: [f32; 4] = [1.0; 4];
	if color.list.len() > 0 {
		for i in 0..4 {
			match color.list.get(i) {
				Some(r) => {
					positions[i] = r.position;
					let j = i * 4;
					colors[j] = r.rgba.x;
					colors[j + 1] = r.rgba.y;
					colors[j + 2] = r.rgba.z;
					colors[j + 3] = r.rgba.w;
				},
				None => {
					positions[i] = 1.0;
					let j = i * 4;
					colors[j] = colors[j - 4];
					colors[j + 1] = colors[j - 3];
					colors[j + 2] = colors[j - 2];
					colors[j + 3] = colors[j - 1];
				},
			}
		}
	}
	let normalize_direction = Vector2::new(color.direction.cos(), color.direction.sin());
	let r = [
		Vector2::new(aabb.mins.x, aabb.mins.y).dot(&normalize_direction), 
		Vector2::new(aabb.maxs.x, aabb.mins.y).dot(&normalize_direction),
		Vector2::new(aabb.maxs.x, aabb.maxs.y).dot(&normalize_direction),
		Vector2::new(aabb.mins.x, aabb.maxs.y).dot(&normalize_direction),
	];
	let (min, max) = (r[0].min(r[1]).min(r[2]).min(r[3]), r[0].max(r[1]).max(r[2]).max(r[3]));
	let end = (normalize_direction * min, normalize_direction * max);

	instance_data.set_data(&GradientColorUniform(&colors));
	instance_data.set_data(&GradientPositionUniform(&positions));
	instance_data.set_data(&GradientEndUniform([end.0.x, end.0.y, end.1.x, end.1.y].as_slice()));
	
	*render_flag |= 1 << RenderFlagType::LinearGradient as usize;
	*render_flag &= !(1 << RenderFlagType::Color as usize);
}
