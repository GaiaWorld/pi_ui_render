use pi_flex_layout::prelude::Rect;
use pi_null::Null;
use pi_style::style::{CgColor, StyleType};
use pi_world::fetch::OrDefault;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};
use pi_world::single_res::SingleRes;
use pi_polygon::mult_to_triangle;
use pi_world::world::Entity;

use crate::components::calc::{style_bit, DrawList, IsRotate, LayoutResult, RectSdfSlice, SdfSlice, SdfUv, StyleBit, StyleMarkType, LAYOUT_DIRTY};
use crate::components::draw_obj::{BackgroundColorMark, BoxType, InstanceIndex, PolygonType, RenderCount, TempGeo, TempGeoBuffer, VColor};
use crate::resource::{BackgroundColorRenderObjType, GlobalDirtyMark, IsRun, OtherDirtyType};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::batch_meterial::{ColorUniform, LayoutUniform, RenderFlagType, SdfUvUniform, TyMeterial};
use crate::components::user::{BackgroundColor, Color};
use crate::system::base::node::layout::calc_layout;
use crate::system::draw_obj::geo_split::{LinearData, OtherInfo};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::{self, update_render_instance_data};
use crate::utils::tools::is_large_size;


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
				BackgroundColorMark,
				{ BACKGROUND_COLOR_ORDER },
				{ BoxType::None },
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
				.in_set(UiSystemSet::IsRun)
		)
		;
    }
}

pub const BACKGROUND_COLOR_ORDER: u8 = 2;

lazy_static! {
	pub static ref BACKGROUND_COLOR_DATA_DIRTY: StyleMarkType = style_bit()
		.set_bit(OtherDirtyType::NodeTreeAdd as usize)
		.set_bit(StyleType::BackgroundColor as usize)| &*LAYOUT_DIRTY;
}

pub fn background_color_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	let r = mark.mark.has_any(&*BACKGROUND_COLOR_DATA_DIRTY);
	r
}


#[derive(Debug)]
pub enum ColorEvent {
	Color(CgColor, SdfUv, Rect<f32>),
	Geo(TempGeo),
}

#[derive(Debug, Default)]
pub struct BackgroundColorChange { 
	list: Vec<(Entity, ColorEvent, bool/*is_opacity*/)>, 
	buffer: TempGeoBuffer
}

/// 计算背景颜色的实例个数（线性渐变需要渲染多个实例）
pub fn calc_background_color_instance_count(
	rect_sdf_slice: OrInitSingleRes<RectSdfSlice>,
	mut events: OrInitSingleResMut<BackgroundColorChange>,
    query: Query<(&BackgroundColor, &LayoutResult, &IsRotate, &DrawList, Entity, OrDefault<SdfUv>, Option<&SdfSlice>, &Layer), Or<(Changed<BackgroundColor>, Changed<LayoutResult>, Changed<IsRotate>, Changed<Layer>)>>,
	mut query_draw: Query<&mut RenderCount, With<BackgroundColorMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
) {

	if r.0 {
		return;
	}

	let render_type = ***render_type;
	let mut layout_slice;
	for (background_color, layout, is_rotate, draw_list, entity, sdf_uv, sdf_slice, layer) in query.iter() {
		if layer.layer().is_null() {
			continue;
		}
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		log::debug!("calc_background_color_instance_count, {:?}", (entity, draw_id));
		let mut render_count = query_draw.get_mut(draw_id).unwrap();

		let padding_rect = layout.padding_rect();
		let width = padding_rect.width();
		let height = padding_rect.height();
		let large_size = is_large_size( width, height);

		

		let is_opacity = background_color.0.is_opaque(); 
		let sdf_slice = match sdf_slice {
			Some(slice) => Some((&slice.layout_slice, &slice.sdf_slice)),
			_ => {
				if is_opacity && large_size && is_rotate.0 {
					layout_slice = Rect {
						left: 2.0 / width,
						right: (width - 2.0) / width,
						top: 2.0 / height,
						bottom: (height - 2.0) / height,
					};
					Some((&rect_sdf_slice.0, &layout_slice))
				} else {
					// 半透明不需要切九宫格， 整体渲染
					None
				}
			}
		};

		fn linear_gradient<'a>(sdf_px_range: f32, mut temp_geo: TempGeo, data: &mut LinearData<'a>, colors: &mut Vec<f32> )  -> (usize, ColorEvent){
			temp_geo.linear_gradient_split_exec(data, colors);
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
						sdf_px_range,
					}
				))

		}
		let events = &mut **events;
		let buffer = &mut events.buffer;
		let (opacity_count, transparent_count) = match &background_color.0 {
			Color::RGBA(color) => {
				let is_opacity = color.is_opacity(); 
				match sdf_slice {
					Some((layout_slice, sdf_slice)) => {
						let start = buffer.positions.len();
						TempGeo::grid_aabbs(&TempGeo::absolute_slice(layout_slice, &padding_rect), &padding_rect, &mut buffer.positions);
						TempGeo::grid_aabbs(&TempGeo::absolute_slice(sdf_slice, &sdf_uv.0), &sdf_uv.0, &mut buffer.sdf_uvs);

						if is_opacity {
							// 如果是不透明物体， 切分的中间部分是不透明的， 边缘部分是半透明的
							events.list.push((entity, ColorEvent::Geo(TempGeo {
								colors: VColor::CgColor(color.clone()),
								polygons: PolygonType::Rect((buffer.positions.len() - 4)..buffer.positions.len()),// 最后一个矩形是中间不透明部分
								sdf_px_range: sdf_uv.1,
							}), true));
							events.list.push((entity, ColorEvent::Geo(TempGeo {
								colors: VColor::CgColor(color.clone()),
								polygons: PolygonType::Rect(start..(buffer.positions.len() - 4)), // 前八个矩形是中间不透明部分 
								sdf_px_range: sdf_uv.1,
							}), false));
							(1, 8)
						} else {
							// 如果本身就是半透明物体， 切分后， 所有实例都是半透明的
							events.list.push((entity, ColorEvent::Geo(TempGeo {
								colors: VColor::CgColor(color.clone()),
								polygons: PolygonType::Rect(start..buffer.positions.len()),
								sdf_px_range: sdf_uv.1,
							}), false));
							(0, 9)
						}
					},
					None => {
						if is_opacity {
							events.list.push((entity, ColorEvent::Color(color.clone(), sdf_uv.clone(), padding_rect), true));
							(1, 0)
						} else {
							events.list.push((entity, ColorEvent::Color(color.clone(), sdf_uv.clone(), padding_rect), false));
							(0, 1)
						}
					},
				}
			}
			Color::LinearGradient(color) => {
				let mut temp_geo = TempGeo::default();
				temp_geo.sdf_px_range = sdf_uv.1;
				let is_opacity = color.is_opacity(); // TODO， 旋转的对象应该当做透明对象处理
				
				
				match sdf_slice {
					Some((layout_slice, sdf_slice )) => {
						let position_start = buffer.positions.len() / 2;
						TempGeo::grid_point(&TempGeo::absolute_slice(layout_slice, &padding_rect), &padding_rect, &mut buffer.positions);
						TempGeo::grid_point(&TempGeo::absolute_slice(sdf_slice, &sdf_uv.0), &sdf_uv.0, &mut buffer.sdf_uvs);
						let mut linear_data = TempGeo::linear_gradient_split_ready(color, &padding_rect, &mut buffer.positions, &mut buffer.sdf_uvs, &mut buffer.uvs);
						if is_opacity {
							
							let (tranparent_count, tranparent_geo) = linear_gradient(sdf_uv.1, TempGeo::new(TempGeo::grid_border_index(position_start as u16)), &mut linear_data,  &mut buffer.colors);
							events.list.push((entity, tranparent_geo, false));
							let (opacity_count, opacity_geo) = linear_gradient(sdf_uv.1, TempGeo::new(TempGeo::grid_fill_index(position_start as u16)), &mut linear_data,  &mut buffer.colors);
							events.list.push((entity, opacity_geo, true));
							(opacity_count, tranparent_count)
						} else {
							let (tranparent_count, tranparent_geo) = linear_gradient(sdf_uv.1, TempGeo::new(TempGeo::grid_index(position_start as u16)), &mut linear_data, &mut buffer.colors);
							events.list.push((entity, tranparent_geo, false));
							(0, tranparent_count)
						}
					}
					None => {
						let start = buffer.positions.len();
						TempGeo::rect_to_quad(&padding_rect, &mut buffer.positions);
						TempGeo::rect_to_quad(&sdf_uv.0, &mut buffer.sdf_uvs);
						temp_geo.polygons = PolygonType::Rule(4, start..buffer.positions.len()); // 规则的四边形
						let mut linear_data = TempGeo::linear_gradient_split_ready(color, &padding_rect, &mut buffer.positions, &mut buffer.sdf_uvs, &mut buffer.uvs);
						let (count, geo) = linear_gradient(sdf_uv.1, temp_geo, &mut linear_data,  &mut buffer.colors);
						if is_opacity {
							events.list.push((entity, geo, true));
							(count, 0)
						} else {
							events.list.push((entity, geo, false));
							(0, count)
						}
					}
				}
			}
		};

		if opacity_count as u32 != render_count.opacity || 
			transparent_count as u32 != render_count.transparent 
		{
			render_count.opacity = opacity_count as u32;
			render_count.transparent = transparent_count as u32;
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
		}
		
		// if draw_info.is_opacity() != is_opacity {
		// 	draw_info.set_is_opacity(is_opacity);
		// 	global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true); // opacity修改， 实例需要重新排序， 这里比较粗暴的直接设置实例数量改变
		// }
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_background_color(
	mut events: OrInitSingleResMut<BackgroundColorChange>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    query: Query<&DrawList>,
    mut query_draw: Query<&InstanceIndex, With<BackgroundColorMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	let events = &mut **events;
	for (entity, change, is_opacity) in events.list.drain(..) {
		if let Ok(draw_list) = query.get(entity) {
			let draw_id = match draw_list.get_one(render_type) {
				Some(r) => r.id,
				None => continue,
			};
			if let Ok(instance_index) = query_draw.get_mut(draw_id) {
				log::debug!("calc_background_color, draw_id={:?}, instance_index={:?}, background_color={:?}", draw_id, instance_index, change);
				
				let start = instance_index.index(is_opacity).start;
				// 节点可能设置为dispaly none， 此时instance_index可能为Null
				if pi_null::Null::is_null(&start) {
					continue;
				}

				match change {
					ColorEvent::Color(color, sdf_uv, rect) => {
						let mut instance_data = instances.instance_data.instance_data_mut(start);
						let mut render_flag = instance_data.get_render_ty();
						instance_data.set_data(&LayoutUniform(&[rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top]));
						instance_data.set_data(&ColorUniform(&[color.x, color.y, color.z, color.w]));
						instance_data.set_data(&SdfUvUniform(&[sdf_uv.0.left, sdf_uv.0.top, sdf_uv.0.right, sdf_uv.0.bottom]));
						render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
						instance_data.set_data(&TyMeterial(&[render_flag as f32]));
					}
					ColorEvent::Geo(geo) => {
						let ty = instances.instance_data.instance_data_mut(start).get_render_ty();
						geo.set_instance_data(start, &mut instances, Some(&OtherInfo {
							sdf_info: [geo.sdf_px_range, 0.5, 0.5],
							stroke_color: [0.0, 0.0, 0.0, 0.0],
							ty: ty as f32,
						}), &events.buffer);
					},
				};
			}
		}
	}
	events.buffer.clear();
}




