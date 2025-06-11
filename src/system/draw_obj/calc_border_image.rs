use std::ops::Range;

use pi_null::Null;
use pi_world::fetch::OrDefault;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};

use pi_flex_layout::prelude::{Rect, Size};
use pi_style::style::{ImageRepeatOption, StyleType};
use pi_world::single_res::SingleRes;
use pi_world::world::Entity;

use crate::components::calc::{style_bit, BorderImageTexture, DrawList, LayoutResult, SdfSlice, SdfUv, StyleBit, StyleMarkType, Texture, LAYOUT_DIRTY};
use crate::components::draw_obj::{BorderImageMark, BoxType, InstanceIndex, RenderCount, TempGeo};
use crate::components::user::{BorderImageClip, BorderImageRepeat, BorderImageSlice};
use crate::resource::draw_obj::InstanceContext;
use crate::resource::{BorderImageRenderObjType, GlobalDirtyMark, OtherDirtyType};
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::update_render_instance_data;
use crate::system::base::node::layout::calc_layout;
use crate::system::base::node::transition::transition_2;
use crate::system::draw_obj::geo_split::RepeatInfo;
use crate::system::system_set::UiSystemSet;
use crate::components::user::BorderImage;
use crate::system::base::draw_obj::{image_texture_load, life_drawobj, set_box_type, set_box_type_count};
use crate::resource::IsRun;
use crate::utils::tools::eq_f32;

use super::geo_split::{set_grid_instance, DirectionDesc, GridBufer};

pub struct BorderImagePlugin;

impl Plugin for BorderImagePlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
			.add_system(UiStage, image_texture_load::image_load::<BorderImage, BorderImageTexture, {OtherDirtyType::BorderImageTexture}, BorderImageRenderObjType>
				.in_set(UiSystemSet::NextSetting)
				.after(transition_2))
			.add_system(UiStage, 
				life_drawobj::draw_object_life_new::<
					BorderImageTexture,
					BorderImageRenderObjType,
					(BorderImageMark, RenderCount),
					{ BORDER_IMAGE_ORDER },
					{ BoxType::None },
				>
					.in_set(UiSystemSet::LifeDrawObject)
					.run_if(border_image_life_change)
					.after(image_texture_load::image_load::<BorderImage, BorderImageTexture, {OtherDirtyType::BorderImageTexture}, BorderImageRenderObjType>),
			)
			.add_system(UiStage, 
				calc_border_image
					.after(crate::system::base::node::world_matrix::cal_matrix)
					.run_if(border_texture_change)
					.in_set(UiSystemSet::PrepareDrawObj)
			)
			.add_system(UiStage, 
				calc_border_image_instance_count
					.after(UiSystemSet::LifeDrawObjectFlush)
					.before(update_render_instance_data)
					.after(calc_layout)
					.run_if(border_texture_change)
					.in_set(UiSystemSet::IsRun)
			)
		;
    }
}

pub const BORDER_IMAGE_ORDER: u8 = 5;

lazy_static! {
	pub static ref BORDER_IMAGE_DIRTY: StyleMarkType = style_bit() | &*LAYOUT_DIRTY
		.set_bit(StyleType::BorderImageClip as usize)
		.set_bit(StyleType::BorderImageSlice as usize)
		.set_bit(StyleType::BorderImageRepeat as usize)
		.set_bit(OtherDirtyType::BorderImageTexture as usize);
}

pub fn border_texture_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*BORDER_IMAGE_DIRTY)
}

pub fn border_image_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::BorderImage as usize).map_or(false, |display| {*display == true})
}

pub fn border_image_life_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(OtherDirtyType::BorderImageTexture as usize).map_or(false, |display| {*display == true})
}

#[derive(Default)]
pub struct BorderImageTemp (pub GridBufer, pub Vec<(Entity, [(Range<usize>, Range<usize>); 9])>);

/// 计算背景图片的实例数量
pub fn calc_border_image_instance_count(
	mut grid_buffer: OrInitSingleResMut<BorderImageTemp>,
	query1: Query<
		(
			&LayoutResult,
			&DrawList,
			&BorderImageTexture,
			OrDefault<BorderImageClip>,
			OrDefault<BorderImageRepeat>,
			OrDefault<BorderImageSlice>,
			&BorderImage,
			OrDefault<SdfSlice>,
			OrDefault<SdfUv>,
			&Layer,
		),
		Or<(Changed<BorderImageTexture>, Changed<BorderImageClip>, Changed<BorderImageRepeat>, Changed<BorderImageSlice>, Changed<LayoutResult>, Changed<Layer>)>,
	>,
	mut query_draw: Query<(&mut BoxType, &mut RenderCount)>,
	render_type: OrInitSingleRes<BorderImageRenderObjType>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
	let render_type = ***render_type;
	let grid_buffer = &mut **grid_buffer;
	for (layout, draw_list, border_image_texture, border_clip, border_repeat, border_slice, border_image, sdf_slice, sdf_uv, layer) in query1.iter() {
		// if border_image.0.as_str().contains("tongyongdianhei_yuanjiao10_bg") {
		// 	log::warn!("border image====={:?}", (border_image.0.as_str(), layer.layer().is_null(), draw_list.get_one(render_type), border_image_texture.is_some()));
		// }
		
		if layer.layer().is_null() {
			continue;
		}
		let sdf_uv = &sdf_uv.0;
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};

		let border_image_texture = match &border_image_texture.0 {
			Some(r) => {
				// 图片不一致， 返回
				if let Texture::All(r) = r {
					if *r.key() != border_image.0.str_hash() as u64 {
						log::debug!("calc_background_image1, {:?}", (r.key(), border_image.0.str_hash()));
						set_box_type(draw_id, BoxType::None2, &mut query_draw);
						continue;
					}
				}
				r
			},
			None => {
				set_box_type(draw_id, BoxType::None2, &mut query_draw);
				log::debug!("calc_background_image2");
				continue;
			}, 
		};
		
		
		let border_aabb = layout.border_aabb();

		let pmins = &border_aabb.mins;
		let pmaxs = &border_aabb.maxs;
		let layout_width = (border_aabb.maxs.x - border_aabb.mins.x).max(0.003);
		let layout_height = (border_aabb.maxs.y - border_aabb.mins.y).max(0.003);
		
		let (uv0, uv1) = border_image_texture.to_uv(border_clip);
		let mut clip = Rect {
			left: uv0.x,
			right: uv1.x,
			top: uv0.y,
			bottom: uv1.y,
		};
		verify_sero_size(&mut clip, 0.001);
		let clip_size = Size{ width: clip.right - clip.left, height: clip.bottom - clip.top };

		// 相对整个纹理， slice四条切割线的位置（0~1）
		let mut slice_uv = Rect {
			left:   (clip.left   + *border_slice.left   * clip_size.width),
			right:  (clip.right  - *border_slice.right  * clip_size.width),
			top:    (clip.top    + *border_slice.top    * clip_size.height),
			bottom: (clip.bottom - *border_slice.bottom * clip_size.height),
		};
		verify_sero_size(&mut slice_uv, 0.001);
		let slice_size_percent = Size {
			width: (slice_uv.right - slice_uv.left),
			height: (slice_uv.bottom - slice_uv.top),
		};
		let s = border_image_texture.size();
		let slice_size = Size {
			width: slice_size_percent.width * s.width as f32,
			height: slice_size_percent.height * s.height as f32,
		};

		// let slice_middle = Point2::new(
		// 	(slice_uv.right + slice_uv.left) / 2.0,
		// 	(slice_uv.bottom + slice_uv.top) / 2.0,
		// );
		// let ss = grid_buffer.0.positions.len();

		// border布局的四条切割线的位子
		let mut border = Rect {
			left:   layout.border.left.max(0.001),
			right:  (layout_width - layout.border.right).max(0.002),
			top:    layout.border.top.max(0.001),
			bottom: (layout_height - layout.border.bottom).max(0.002),
		};
		verify_sero_size(&mut border, 0.001);

		let w = pmaxs.x - pmins.x - layout.border.left - layout.border.right;
		let h = pmaxs.y - pmins.y - layout.border.top - layout.border.bottom;

		// 上右下左，边框布局宽度与图片边框部分的比率
		let factor = (
			layout.border.top / (*border_slice.top).max(0.001) / s.height as f32, 
			layout.border.right / (*border_slice.right).max(0.001) / s.width as f32, 
			layout.border.bottom / (*border_slice.bottom).max(0.001) / s.height as f32, 
			layout.border.left / (*border_slice.left).max(0.001) / s.width as f32
		);

		let layout_slice_absolute = Rect {
			left: sdf_slice.layout_slice.left * layout_width,
			right: sdf_slice.layout_slice.right * layout_width,
			top: sdf_slice.layout_slice.top * layout_height,
			bottom: sdf_slice.layout_slice.bottom * layout_height,
		};

		let direction_desc_y = DirectionDesc {
			sdf_uv: sdf_uv.top..sdf_uv.bottom, 
			sdf_slice: sdf_slice.sdf_slice.top..sdf_slice.sdf_slice.bottom, 
			layout_range: pmins.y..pmaxs.y,  
			split: layout_slice_absolute.top..layout_slice_absolute.bottom
		};

		let direction_desc_x = DirectionDesc {
			sdf_uv: sdf_uv.left..sdf_uv.right, 
			sdf_slice: sdf_slice.sdf_slice.left..sdf_slice.sdf_slice.right, 
			layout_range: pmins.x..pmaxs.x,  
			split: layout_slice_absolute.left..layout_slice_absolute.right
		};

		// 上边框
		let top_range = if layout.border.top > 0.0 {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.y,
				end: pmins.y + layout.border.top,
				bound_step: 0.0,
				space: 0.0,
				item_size: layout.border.top,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_y,
			clip.top..slice_uv.top,
			)
		} else {
			0..0
		};

		// 下边框
		let bottom_range = if layout.border.bottom > 0.0 {
			TempGeo::grid_split(&RepeatInfo {
				start: pmaxs.y - layout.border.bottom,
				end: pmaxs.y,
				bound_step: 0.0,
				space: 0.0,
				item_size: layout.border.bottom,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_y,
			slice_uv.bottom..clip.bottom)
		} else {
			0..0
		};

		// 左边框
		let left_range = if layout.border.left > 0.0 {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.x,
				end: pmins.x + layout.border.left,
				bound_step: 0.0,
				space: 0.0,
				item_size: layout.border.left,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_x,
			clip.left..slice_uv.left,)
		} else {
			0..0
		};

		// 右边框
		let right_range = if layout.border.right > 0.0 {
			TempGeo::grid_split(&RepeatInfo {
				start: pmaxs.x - layout.border.right,
				end: pmaxs.x,
				bound_step: 0.0,
				space: 0.0,
				item_size: layout.border.right,
			}, 
			&mut grid_buffer.0,	 
			&direction_desc_x,
			slice_uv.right..clip.right,)
		} else {
			0..0
		};
		
		let fill_y_size = pmaxs.y - layout.border.bottom - layout.border.top;
		let fill_x_size = pmaxs.x - layout.border.left - layout.border.right;

		// 中间纬线部分
		let fill_weft_range = if border_slice.fill && fill_y_size > 0.0 && fill_x_size > 0.0  {
			TempGeo::grid_split(&RepeatInfo {
				start: layout.border.left,
				end: pmaxs.x - layout.border.right,
				bound_step: 0.0,
				space: 0.0,
				item_size: fill_x_size,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_x,
			slice_uv.left..slice_uv.right,)
		} else {
			0..0
		};

		// 中间经线部分
		let fill_meridian_range = if border_slice.fill && fill_y_size > 0.0 && fill_y_size > 0.0  {
			TempGeo::grid_split(&RepeatInfo {
				start: layout.border.top,
				end: pmaxs.y - layout.border.bottom,
				bound_step: 0.0,
				space: 0.0,
				item_size: fill_y_size,
			}, 
			&mut grid_buffer.0,
			&direction_desc_y,
			slice_uv.top..slice_uv.bottom,)
		} else {
			0..0
		};

		
		// top, 中间部分
		let uv_size = slice_size.width * factor.0;
		let (layout_offset, bound, space, layout_space, _count) = calc_step(w,  uv_size, border_repeat.x);
		let top_repeat_range = if border.top > 0.0 && fill_x_size > 0.0  {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.x + border.left + layout_offset,
				end: pmins.x + border.left + fill_x_size,
				bound_step: bound * layout_space,
				space,
				item_size: layout_space,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_x,
			slice_uv.left..slice_uv.right,)
		} else {
			0..0
		};

		// bottom, 中间部分
		let uv_size = slice_size.width * factor.2;
		let (layout_offset, bound, space, layout_space, _count) = calc_step(w, uv_size, border_repeat.x);
		let bottom_repeat_range = if border.bottom > 0.0 && fill_x_size > 0.0  {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.x + border.left + layout_offset,
				end: pmins.x + border.left + fill_x_size,
				bound_step: bound * layout_space,
				space,
				item_size: layout_space,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_x,
			slice_uv.left..slice_uv.right,)
		} else {
			0..0
		};

		// left, 中间部分
		let uv_size = slice_size.height * factor.3;
		let (layout_offset, bound, space, layout_space, _count) = calc_step(h, uv_size, border_repeat.y);
		let left_repeat_range = if border.left > 0.0 && fill_y_size > 0.0  {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.y + border.top + layout_offset,
				end: pmins.y + border.top + fill_y_size,
				bound_step: bound * layout_space,
				space,
				item_size: layout_space,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_y,
			slice_uv.top..slice_uv.bottom,)
		} else {
			0..0
		};

		// right, 中间部分
		let uv_size = slice_size.height * factor.1;
		let (layout_offset, bound, space, layout_space, _count) = calc_step(h, uv_size, border_repeat.y);
		let right_repeat_range = if border.right > 0.0 && fill_y_size > 0.0 {
			TempGeo::grid_split(&RepeatInfo {
				start: pmins.y + border.top + layout_offset,
				end: pmins.y + border.top + fill_y_size,
				bound_step: bound * layout_space,
				space,
				item_size: layout_space,
			}, 
			&mut grid_buffer.0, 
			&direction_desc_y,
			slice_uv.top..slice_uv.bottom,)
		} else {
			0..0
		};

		let mut count = top_range.len() * (top_repeat_range.len() + left_range.len() + right_range.len()); // 上
		count += bottom_range.len() * (bottom_repeat_range.len() + left_range.len() + right_range.len()); // 下
		count += left_range.len() * left_repeat_range.len(); // 左、中
		count += right_range.len() * right_repeat_range.len(); // 右、中
		count += fill_meridian_range.len() * fill_weft_range.len(); // 中
		count = count / 4;

		
		set_box_type_count(draw_id, BoxType::None, count, &mut query_draw, &mut global_mark);
		log::debug!("border render_count=============={:?}, {:?}, {:?}", &border_image.0, count, (
			&top_range,
			&right_range,
			&bottom_range,
			&left_range,
			&top_repeat_range,
			&right_repeat_range,
			&bottom_repeat_range,
			&left_repeat_range,
			&fill_weft_range,
			&fill_meridian_range,
			border_slice,
		));

		let range = [
			(
				left_range.clone(),
				top_range.clone(),
			),
			(
				right_range.clone(),
				top_range.clone(),
			),
			(
				right_range.clone(),
				bottom_range.clone(),
			),
			(
				left_range.clone(),
				bottom_range.clone(),
			),
			(
				top_repeat_range.clone(),
				top_range.clone(),
			),
			(
				right_range.clone(),
				right_repeat_range.clone(),
			),
			(
				bottom_repeat_range.clone(),
				bottom_range.clone(),
			),
			(
				left_range.clone(),
				left_repeat_range.clone(),
			),
			(
				fill_weft_range.clone(),
				fill_meridian_range.clone(),
			),
		];
		// if border_image.0.as_str().contains("eff_xinshouquanquan/2.png") {
		// 	log::warn!("calc_border_image=======, {:?}", (&entity, &border_image, count, &range));
		// 	log::warn!("calc_border_image1=======, {:?}", (&grid_buffer.0.positions[ss..] ));
		// }


		grid_buffer.1.push((
			draw_id,
			range
		));
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_border_image(
	mut grid_buffer: OrInitSingleResMut<BorderImageTemp>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    query_draw: Query<&InstanceIndex, With<BorderImageMark>>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}

	let grid_buffer = &mut **grid_buffer;
	// log::trace!("bg image========================{:?}", (mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY1), mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY2)));
	for (draw_id, range) in grid_buffer.1.drain(..) {
		
		if let Ok(instanceindex) = query_draw.get(draw_id) {
			let mut start = instanceindex.start;
			for (x_range, y_range) in range {
				log::debug!("calc_border_image==={:?}", (draw_id, &x_range, &y_range));
				start = set_grid_instance(
					&grid_buffer.0,
					x_range,
					y_range,
					start,
					&mut instances,
				);
			}
			
		}
	}

	grid_buffer.0.positions.clear();
	grid_buffer.0.uvs.clear();
	grid_buffer.0.sdf_uvs.clear();

}


pub fn verify_sero_size(value: &mut Rect<f32>, min_size: f32) {
	value.right = value.left + (value.right - value.left).max(min_size);
	value.bottom = value.top + (value.bottom - value.top).max(min_size);
}


pub fn calc_step(show_size: f32, img_size: f32, rtype: ImageRepeatOption) -> (f32/*第一个item的布局偏移*/, f32/*边缘部分需要渲染多少个item(0~1，单边)*/ , f32/*每个item间隔）*/, f32/*每个item占用的布局宽度*/, usize/*重复次数*/) {
	log::debug!("calc_step {:?} {:?} {:?}", show_size, img_size, rtype);
	if eq_f32(img_size, 0.0) || eq_f32(show_size, 0.0) {
		return (0.0, 0.0, 0.0, show_size.max(0.001), 0);  // 避免为0， 因为其将作为除数
	}

    let repeat_count = show_size / img_size; // 区域内可重复的次数
	
    match rtype {
        ImageRepeatOption::Repeat => { // repeat 只会渲染奇数个item（对称）
			let floor_count = (repeat_count.ceil() as usize >> 1 << 1) + 1; // 向上取奇数
			let offset = if floor_count > 1 {
				(repeat_count - (floor_count - 2) as f32) / 2.0
			} else {
				repeat_count + (1.0 -repeat_count) / 2.0
			};
			(0.0, offset, 0.0, img_size, floor_count as usize/*返回向上的奇数*/)
		},
        ImageRepeatOption::Round => {
			let count = repeat_count.round().max(1.0);
			let step = show_size/count;
			return (0.0, 0.0, 0.0, step, count as usize);
		}
        ImageRepeatOption::Space => {
            let space = show_size % img_size; // 空白尺寸
			let f = repeat_count.floor();
			let s = space / (f + 1.0);
			return (s, 0.0, s, img_size, f as usize)
        }
        ImageRepeatOption::Stretch => (0.0, 0.0, 0.0, show_size, 1),
    }
}

// 计算水平方向或垂直方向， 图片的重复次数
pub fn repeat_count(show_size: f32, img_size: f32, rtype: ImageRepeatOption) -> usize {
	log::debug!("repeat_count: {:?}", (show_size, img_size, rtype));
	if eq_f32(img_size, 0.0) || eq_f32(show_size, 0.0) {
		return 0;
	}

    let repeat_count = show_size / img_size; // 区域内可重复的次数
    match rtype {
        ImageRepeatOption::Repeat => (repeat_count.ceil() as usize >> 1 << 1) + 1, 
        ImageRepeatOption::Round => repeat_count.round().max(1.0) as usize, // 四舍五入个平铺
        ImageRepeatOption::Space => repeat_count.floor() as usize,
        ImageRepeatOption::Stretch => 1,
    }
}