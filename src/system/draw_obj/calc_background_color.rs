

use pi_flex_layout::prelude::Rect;
use pi_style::style::{CgColor, Stroke, StyleType};
use pi_world::event::{Event, EventSender};
use pi_world::fetch::OrDefault;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};
use pi_world::single_res::SingleRes;
use pi_polygon::mult_to_triangle;
use pi_world::world::Entity;

use crate::components::calc::{style_bit, DrawInfo, DrawList, LayoutResult, SdfSlice, SdfUv, StyleBit, StyleMarkType, LAYOUT_DIRTY};
use crate::components::draw_obj::{BackgroundColorMark, BoxType, InstanceIndex, PolygonType, RenderCount, TempGeo, TempGeoBuffer, VColor};
use crate::resource::{BackgroundColorRenderObjType, GlobalDirtyMark, IsRun, OtherDirtyType};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::batch_meterial::{ColorUniform, RenderFlagType, SdfUvUniform, StrokeColorUniform, TyMeterial};
use crate::components::user::{BackgroundColor, Color};
use crate::system::base::node::layout::calc_layout;
use crate::system::draw_obj::geo_split::OtherInfo;
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::{self, update_render_instance_data};


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
				(BackgroundColorMark, RenderCount),
				{ BACKGROUND_COLOR_ORDER },
				{ BoxType::Padding },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.before(calc_background_color),
		)
		.add_system(
			UiStage, 
			calc_background_color
				.after(crate::system::base::node::world_matrix::cal_matrix)
				.in_set(UiSystemSet::PrepareDrawObj)
				.run_if(background_color_change)
		)
		.add_system(UiStage, 
			calc_background_color_instance_count
				.after(UiSystemSet::LifeDrawObjectFlush)
				.before(update_render_instance_data)
				.after(calc_layout)
				.run_if(background_color_change)
		)
		;
    }
}

pub const BACKGROUND_COLOR_ORDER: u8 = 2;

lazy_static! {
	pub static ref BACKGROUND_COLOR_DATA_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BackgroundColor as usize)| &*LAYOUT_DIRTY;
}

pub fn background_color_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	let r = mark.mark.has_any(&*BACKGROUND_COLOR_DATA_DIRTY);
	r
}


#[derive(Debug)]
pub enum ColorEvent {
	Color(CgColor, SdfUv),
	Geo(TempGeo),
}

#[derive(Debug, Default)]
pub struct BackgroundColorChange(Vec<(Entity, ColorEvent)>, TempGeoBuffer);

/// 计算背景颜色的实例个数（线性渐变需要渲染多个实例）
pub fn calc_background_color_instance_count(
	mut events: OrInitSingleResMut<BackgroundColorChange>,
    query: Query<(&BackgroundColor, &LayoutResult, &DrawList, Entity, OrDefault<SdfUv>, Option<&SdfSlice>), (Changed<BackgroundColor>, Changed<LayoutResult>)>,
	mut query_draw: Query<(&mut RenderCount, &mut DrawInfo), With<BackgroundColorMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
) {

	if r.0 {
		return;
	}

	let render_type = ***render_type;
	for (background_color, layout, draw_list, entity, sdf_uv, sdf_slice) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		let (mut render_count, mut draw_info) = query_draw.get_mut(draw_id).unwrap();

		let padding_rect = layout.padding_rect();
		let (count, event, is_opacity) = match &background_color.0 {
			Color::RGBA(color) => {
				match sdf_slice {
					Some(slice) => {
						let len = events.1.positions.len();
						TempGeo::grid_aabbs(&TempGeo::absolute_slice(&slice.layout_slice, &padding_rect), &padding_rect, &mut events.1.positions);
						TempGeo::grid_aabbs(&TempGeo::absolute_slice(&slice.sdf_slice, &sdf_uv.0), &sdf_uv.0, &mut events.1.sdf_uvs);

						(9, ColorEvent::Geo(TempGeo {
							
							colors: VColor::CgColor(color.clone()),
							polygons: PolygonType::Rect(len..events.1.positions.len()),
							sdf_px_range: sdf_uv.1,
						}), false)
					},
					None => (1, ColorEvent::Color(color.clone(), sdf_uv.clone()), color.is_opacity()),
				}
			}
			Color::LinearGradient(color) => {
				let mut temp_geo = TempGeo::default();
				temp_geo.sdf_px_range = sdf_uv.1;
				let is_opacity = match sdf_slice {
					Some(slice) => {
						temp_geo.polygons = TempGeo::grid_index(events.1.positions.len() as u16);
						TempGeo::grid_point(&TempGeo::absolute_slice(&slice.layout_slice, &padding_rect), &padding_rect, &mut events.1.positions);
						TempGeo::grid_point(&TempGeo::absolute_slice(&slice.sdf_slice, &sdf_uv.0), &sdf_uv.0, &mut events.1.sdf_uvs);
						
						false
					}
					None => {
						let start = events.1.positions.len();
						TempGeo::rect_to_quad(&padding_rect, &mut events.1.positions);
						TempGeo::rect_to_quad(&sdf_uv.0, &mut events.1.sdf_uvs);
						temp_geo.polygons = PolygonType::Rule(4, start..events.1.positions.len()); // 规则的四边形
						color.is_opacity()
					}
				};
				temp_geo.linear_gradient_split(color, &padding_rect, &mut events.1);
				let out_indices = match temp_geo.polygons {
					PolygonType::NoRule(indices) => {
						let mut out_indices = Vec::with_capacity(indices.counts.len() * 4); // 预计多边形为四边形
						mult_to_triangle(&indices, &mut out_indices);
						out_indices
					},
					_ => todo!(), // 不会是三角形和规则多边形
				};
				
				(out_indices.len() / 3, ColorEvent::Geo(
					TempGeo {
						colors: temp_geo.colors,
						polygons: PolygonType::Triangle(out_indices),
						sdf_px_range: sdf_uv.1,
					}
				), is_opacity)
			}
		};

		events.0.push((entity, event));

		if count as u32 != render_count.0 {
			render_count.0 = count as u32;
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
		}
		
		if draw_info.is_opacity() != is_opacity {
			draw_info.set_is_opacity(is_opacity);
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true); // opacity修改， 实例需要重新排序， 这里比较粗暴的直接设置实例数量改变
		}
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_background_color(
	mut events: OrInitSingleResMut<BackgroundColorChange>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    query: Query<&DrawList>,
    mut query_draw: Query<(&InstanceIndex, &mut BoxType), With<BackgroundColorMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	let events = &mut **events;
	for (entity, change) in events.0.drain(..) {
		if let Ok(draw_list) = query.get(entity) {
			let draw_id = match draw_list.get_one(render_type) {
				Some(r) => r.id,
				None => continue,
			};
			if let Ok((instance_index, mut box_type)) = query_draw.get_mut(draw_id) {
				log::debug!("calc_background_color, draw_id={:?}, instance_index={:?}, background_color={:?}", draw_id, instance_index, change);
	
				// 节点可能设置为dispaly none， 此时instance_index可能为Null
				if pi_null::Null::is_null(&instance_index.0.start) {
					continue;
				}

				let box_type_new = match change {
					ColorEvent::Color(color, sdf_uv) => {
						let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
						let mut render_flag = instance_data.get_render_ty();
						instance_data.set_data(&ColorUniform(&[color.x, color.y, color.z, color.w]));
						instance_data.set_data(&SdfUvUniform(&[sdf_uv.0.left, sdf_uv.0.top, sdf_uv.0.right, sdf_uv.0.bottom]));
						render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
						instance_data.set_data(&TyMeterial(&[render_flag as f32]));
						BoxType::Padding
					}
					ColorEvent::Geo(geo) => {
						let ty = instances.instance_data.instance_data_mut(instance_index.start).get_render_ty();
						geo.set_instance_data(instance_index.0.start, &mut instances, Some(&OtherInfo {
							sdf_info: [geo.sdf_px_range, 0.5, 0.5],
							stroke_color: [0.0, 0.0, 0.0, 0.0],
							ty: ty as f32,
						}), &events.1);
						BoxType::None
					},
				};

				if box_type_new != *box_type {
					*box_type = box_type_new;
				}
			}
		}
	}
	events.1.clear();
}




