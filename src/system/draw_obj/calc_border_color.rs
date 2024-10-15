//! 圆角从有到删除，没有正确处理顶点（TODO）

use pi_flex_layout::prelude::Rect;
use pi_hal::svg::{Path, PathVerb};
// use pi_sdf::shape::PathVerb;
use pi_style::style::{CgColor, StyleType};
use pi_world::event::ComponentRemoved;
use pi_world::fetch::OrDefault;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};
use pi_world::single_res::{SingleRes, SingleResMut};
use std::ops::Range;
use std::hash::Hash;
use std::hash::Hasher;

use crate::components::calc::{style_bit, BorderSdfUv, DrawList, LayoutResult, SdfSlice, SdfUv, StyleBit, StyleMarkType};
use crate::components::draw_obj::{BorderColorMark, BoxType, InstanceIndex, RenderCount};
use crate::components::user::BorderRadius;
use crate::components::user::BorderColor;
use crate::resource::{BorderColorRenderObjType, GlobalDirtyMark, OtherDirtyType, ShareFontSheet};
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::batch_meterial::{ColorUniform, SdfUniform, StrokeColorUniform};
use crate::system::base::node::layout::calc_layout;
use crate::system::draw_obj::calc_border_radius::gen_sdf;
use crate::system::draw_obj::geo_split::set_grid_instance;
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::{self, update_render_instance_data};
use crate::resource::IsRun;
use crate::utils::tools::{cal_border_radius, calc_hash, eq_f32, BorderRadiusPixel};

use super::calc_border_radius::{min_level, BorderSdfInfo};
use super::geo_split::GridBufer;

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
				(BorderColorMark, RenderCount, BorderSdfUv),
				{ BORDER_COLOR_ORDER },
				{ BoxType::None },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.run_if(border_color_life_change)
				.before(calc_border_color),
		)
		.add_system(
			UiStage, 
			calc_border_color
				.after(crate::system::base::node::world_matrix::cal_matrix)
				.in_set(UiSystemSet::PrepareDrawObj)
				.run_if(border_color_change)
		)
		.add_system(UiStage, 
			calc_border_color_instace_count
				.after(UiSystemSet::LifeDrawObjectFlush)
				.before(update_render_instance_data)
				.after(calc_layout)
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

#[derive(Default)]
pub struct BorderColorTemp (pub GridBufer, pub Vec<(pi_world::world::Entity, CgColor, f32/*sdf_px_distance*/, f32/*fill_bound*/, [(Range<usize>, Range<usize>); 8])>);
/// 计算边框渲染的实例个数
pub fn calc_border_color_instace_count(
	font_sheet: SingleResMut<ShareFontSheet>,
	mut grid_buffer: OrInitSingleResMut<BorderColorTemp>,
    query: Query<
        (
			&BorderColor,
			OrDefault<BorderRadius>,
			OrDefault<SdfUv>,
            &LayoutResult,
			&DrawList,
        ),
        Or<(Changed<BorderColor>, Changed<BorderRadius>, Changed<LayoutResult>)>, // 圆角和Border颜色， 都需要设置border宽度
    >,
	mut query_draw: Query<&mut RenderCount, With<BorderColorMark>>,
	render_type: OrInitSingleRes<BorderColorRenderObjType>,
	r: OrInitSingleRes<IsRun>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
) {
	if r.0 {
		return;
	}

	let render_type = ***render_type;
	let grid_buffer = &mut **grid_buffer;
	for (border_color, border_radius, sdf_uv, layout, draw_list) in query.iter() {
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};

		let mut render_count = query_draw.get_mut(draw_id).unwrap();

		// 边框宽度为0， 不渲染
		if eq_f32(layout.border.top, 0.0) 
			&& eq_f32(layout.border.right, 0.0) 
			&& eq_f32(layout.border.bottom, 0.0) 
			&& eq_f32(layout.border.left, 0.0) {
			if render_count.0 > 0 {
				render_count.0 = 0;
			}
			continue;
		}
		let sdf_uv0 = &sdf_uv.0;

		// 边框大小相等， 可以重用圆角sdf或矩形sdf
		let mut font_sheet = font_sheet.borrow_mut();
		let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;
		let width = layout.rect.right - layout.rect.left;
		let height = layout.rect.bottom - layout.rect.top; 
		let rd = cal_border_radius(border_radius, &layout.rect);
		let sdf_info = boder_sdf_info(&rd, &layout.border);
		log::debug!("calc_border_color1========= {:?}", (&sdf_info));
		let hash = calc_hash(&sdf_info, 0);
		

		let sdf_glyph = match sdf2_table.shapes_tex_info.get(&hash) {
			Some(r) => r,
			None => {
				let (mut verb, mut points) = (vec![], vec![]);
				gen_sdf(&sdf_info.0, &mut points, &mut verb);
				gen_inner_sdf(&sdf_info.0, &sdf_info.1.0, &mut points, &mut verb);

				log::debug!("calc_border_color: {:?}", (&verb, &points, &sdf_info));
				let svg_info = Path::new1(verb, points).get_svg_info();

				sdf2_table.add_shape(hash, svg_info, sdf_info.0.width as usize, 1,  2);
				sdf2_table.shapes_tex_info.get(&hash).unwrap()
			},
		};

		let sdf_uv = SdfUv (Rect {
			left:   sdf_glyph.x as f32,
			right:  sdf_glyph.x as f32 + sdf_glyph.width as f32,
			top:    sdf_glyph.y as f32,
			bottom: sdf_glyph.y as f32 + sdf_glyph.height as f32,
		}, 2.0);
		let sdf_uv0 = &sdf_uv.0;

		log::debug!("radius sdf_uv====={:?}, {:?}", &sdf_uv, sdf_glyph);

		let border_box = layout.border_rect();

		let slice = SdfSlice {
			sdf_slice: Rect {
				left: sdf_info.0.sdf_radius.x[0].max(sdf_info.0.sdf_radius.x[3]).max(sdf_info.1.0.left),
				right: (sdf_info.0.width - sdf_info.0.sdf_radius.x[1].max(sdf_info.0.sdf_radius.x[2]).max(sdf_info.1.0.right)),
				top: sdf_info.0.sdf_radius.y[0].max(sdf_info.0.sdf_radius.y[1]).max(sdf_info.1.0.top),
				bottom: (sdf_info.0.height - sdf_info.0.sdf_radius.y[2].max(sdf_info.0.sdf_radius.y[3]).max(sdf_info.1.0.bottom)),
			},
			layout_slice: Rect {
				left: rd.x[0].max(rd.x[3]).max(layout.border.left),
				right: width - rd.x[1].max(rd.x[2]).max(layout.border.right),
				top: rd.y[0].max(rd.y[1]).max(layout.border.top),
				bottom: height - rd.y[2].max(rd.y[3]).max(layout.border.bottom),
			},
		};

		

		let px_range = slice.layout_slice.left/slice.sdf_slice.left  * sdf_uv.1;
		// let fill_bound = 1.0_f32.min(0.5 + (slice.layout_slice.left / px_range)) ;

		log::debug!("fill_bound=========={:?}", (px_range, slice.layout_slice.left, slice.sdf_slice.left, sdf_uv0.left, sdf_uv.1));

		let fill_x_size = layout.rect.right - layout.rect.left - layout.border.left - layout.border.right;
		let fill_y_size = layout.rect.bottom - layout.rect.top - layout.border.top - layout.border.bottom;
		let mut count = 0;
		let start = grid_buffer.0.positions.len();
		grid_buffer.0.positions.extend_from_slice(&[
			border_box.left, slice.layout_slice.left, slice.layout_slice.right, border_box.right, // 纬线
			border_box.top, slice.layout_slice.top, slice.layout_slice.bottom, border_box.bottom, // 经线
		]);
		grid_buffer.0.sdf_uvs.extend_from_slice(&[
			sdf_uv0.left, sdf_uv0.left + slice.sdf_slice.left, sdf_uv0.left + slice.sdf_slice.right, sdf_uv0.right, // 纬线
			sdf_uv0.top, sdf_uv0.top + slice.sdf_slice.top, sdf_uv0.top + slice.sdf_slice.bottom, sdf_uv0.bottom, // 经线
		]);
		log::debug!("fill_bound11=========={:?}", [
			sdf_uv0.left, sdf_uv0.left + slice.sdf_slice.left, sdf_uv0.left + slice.sdf_slice.right, sdf_uv0.right, // 纬线
			sdf_uv0.top, sdf_uv0.top + slice.sdf_slice.top, sdf_uv0.top + slice.sdf_slice.bottom, sdf_uv0.bottom, // 经线
		]);
		log::debug!("slice=========={:?}",slice);

		let left = if layout.border.left > 0.0 {
			start + 0..start +2
		} else {
			0..0
		};

		let fill_latitude = if fill_x_size > 0.0 && !eq_f32(fill_x_size, 0.0) {
			start + 1..start + 3
		} else {
			0..0
		};

		let right = if layout.border.right > 0.0 {
			start + 2..start + 4
		} else {
			0..0
		};
		
		let top = if layout.border.top > 0.0 {
			start + 4..start + 6
		} else {
			0..0
		};

		let fill_meridian = if fill_y_size > 0.0 && !eq_f32(fill_y_size, 0.0) {
			start + 5..start + 7
		} else {
			0..0
		};

		let bottom = if layout.border.bottom > 0.0 {
			start + 6..start + 8
		} else {
			0..0
		};


		count += left.len() * (top.len() + bottom.len() + fill_meridian.len());
		count += right.len() * (top.len() + bottom.len() + fill_meridian.len());
		count += fill_latitude.len() * (top.len() + bottom.len());
		count = count / 4;
		log::debug!("border-color==============={:?}", count);

		grid_buffer.1.push((draw_id, border_color.0.clone(), px_range, 0.5, [
			(left.clone(), top.clone()),
			(right.clone(), top.clone()),
			(right.clone(), bottom.clone()),
			(left.clone(), bottom.clone()),
			(left, fill_meridian.clone()),
			(fill_latitude.clone(), top),
			(right, fill_meridian),
			(fill_latitude, bottom),
		]));

		if render_count.0 != count as u32 {
			render_count.0 = count as u32;
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
		}
	
	}
}


/// 设置边框颜色的顶点、索引、和边框颜色uniform
pub fn calc_border_color(
	mut grid_buffer: OrInitSingleResMut<BorderColorTemp>,
	mut instances: OrInitSingleResMut<InstanceContext>,

	query_draw: Query<&InstanceIndex, With<BorderColorMark>>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}

	let grid_buffer = &mut **grid_buffer;
	// log::trace!("bg image========================{:?}", (mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY1), mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY2)));
	for (draw_id, color, px_range, fill_bound, range) in grid_buffer.1.drain(..) {
		
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
			
			let alignment = instances.instance_data.alignment;
			instances.instance_data.set_data_mult(instanceindex.start, (instanceindex.end - instanceindex.start) / alignment, &StrokeColorUniform(&[
				color.x, color.y, color.z, color.w
			]));
			instances.instance_data.set_data_mult(instanceindex.start, (instanceindex.end - instanceindex.start) / alignment, &ColorUniform(&[
				color.x, color.y, color.z, color.w
			]));
			instances.instance_data.set_data_mult(instanceindex.start, (instanceindex.end - instanceindex.start) / alignment, &SdfUniform(&[
				px_range, fill_bound, 0.5,
			]));

			log::debug!("calc_border_image==={:?}", (draw_id, &px_range, &fill_bound));
			
		}
	}

	grid_buffer.0.positions.clear();
	grid_buffer.0.sdf_uvs.clear();
}

#[derive(Debug)]
pub struct BorderWidth (pub Rect<f32>);

impl Hash for BorderWidth {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0.top * 1000.0) as usize).hash(state);
		((self.0.right * 1000.0) as usize).hash(state);
		((self.0.bottom * 1000.0) as usize).hash(state);
		((self.0.left * 1000.0) as usize).hash(state);
	}
}


fn boder_sdf_info(
    rd: &BorderRadiusPixel, border: &Rect<f32>) -> (BorderSdfInfo, BorderWidth) {
	// 最小的圆角值
    let min_radius = rd.x[0].min(rd.x[1]).min(rd.y[0]).min(rd.y[1]).min(rd.x[2]).min(rd.y[2]).min(rd.x[3]).min(rd.y[3]);
    
    // 所有圆角和边框的最大值
    let max_size = (rd.x[0].max(border.left) + rd.x[1].max(border.right))
						.max(rd.x[2].max(border.right) + rd.x[3].max(border.left))
						.max(rd.y[0].max(border.top) + rd.y[3].max(border.top))
						.max(rd.y[1].max(border.bottom) + rd.y[2].max(border.bottom));

    let level = min_level(min_radius, max_size);
    let size = 32.0 * level;

    let border_size = 30.0 * level;

    let border_scale = border_size / max_size; // 缩放比例
	
	log::debug!("boder_sdf_info==========={:?}", (level, min_radius, max_size, size,  border_size, border_scale, max_size));

    (BorderSdfInfo {
        width: size,
        height: size,
        sdf_radius: BorderRadiusPixel {
            x: [rd.x[0] * border_scale, rd.x[1] * border_scale, rd.x[2] * border_scale, rd.x[3] * border_scale],
            y: [rd.y[0] * border_scale, rd.y[1] * border_scale, rd.y[2] * border_scale, rd.y[3] * border_scale],
        },
    }, BorderWidth(Rect {
		top: border.top * border_scale, 
		right: border.right * border_scale, 
		bottom: border.bottom * border_scale, 
		left: border.left * border_scale,
	}))
}

// 逆时针顺序
fn gen_inner_sdf(sdf_info: &BorderSdfInfo, rect: &Rect<f32>,  points: &mut Vec<f32>, verb: &mut Vec<PathVerb>) {
	let rd1 = &sdf_info.sdf_radius;

    let rd = BorderRadiusPixel {
        x: [(rd1.x[0] - rect.left).max(0.0), (rd1.x[1] - rect.right).max(0.0), (rd1.x[2] - rect.right).max(0.0), (rd1.x[3] - rect.left).max(0.0)],
        y: [(rd1.y[0] - rect.top).max(0.0), (rd1.y[1] - rect.top).max(0.0), (rd1.y[2] - rect.bottom).max(0.0), (rd1.y[3] - rect.bottom).max(0.0)],
    };
	let (box_min_x, box_min_y, box_max_x, box_max_y) = ( rect.left, rect.top, sdf_info.width - rect.right, sdf_info.height - rect.bottom);

    let mut x  ;
    let mut y = box_min_y + rd.y[3];
	let mut pre_x;
	let mut pre_y ;
    verb.push(PathVerb::MoveTo);
    points.extend_from_slice(&[box_min_x, y]);

	// 左下圆角
	x = box_min_x + rd.x[3];
	if rd.x[3] != 0.0 && rd.y[3] != 0.0 {
		// 椭圆弧
		verb.push(PathVerb::EllipticalArcTo);
		points.extend_from_slice(&[rd.x[3], rd.y[3], 0.0, 1.0, x, box_min_y]);
	}

	// 下边直线
	pre_x = x;
	x = box_max_x - rd.x[2];
    if !eq_f32(pre_x, x) {
        verb.push(PathVerb::LineTo);
        points.extend_from_slice(&[x, box_min_y]);
    }

	log::debug!("x:{}", box_min_y);

	// 右下圆角
    y = box_min_y + rd.y[2];
    if rd.x[2] != 0.0 && rd.y[2] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[2], rd.y[2], 0.0, 1.0, box_max_x, y]);
    } 

	// 右边直线
	pre_y = y;
	y = box_max_y - rd.y[1];
	if !eq_f32(y, pre_y) {
		verb.push(PathVerb::LineTo);
		points.extend_from_slice(&[box_max_x, y]);
	}

	// 右上圆角
	x = box_max_x - rd.x[1];
	if rd.x[1] != 0.0 && rd.y[1] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[1], rd.y[1], 0.0, 1.0, x, box_max_y]);
    }

	// 上边直线
	pre_x = x;
	x = box_min_x + rd.x[0];
	if !eq_f32(pre_x, x) {
		verb.push(PathVerb::LineTo);
		points.extend_from_slice(&[x, box_max_y]);
	}

	// 左上圆角
	y = box_max_y - rd.y[0];
	if rd.x[0] != 0.0 && rd.y[0] != 0.0 {
		// 椭圆弧
		verb.push(PathVerb::EllipticalArcTo);
		points.extend_from_slice(&[rd.x[0], rd.y[0], 0.0, 1.0, box_min_x, y]);
	}

    // 左边直线
	verb.push(PathVerb::Close);         
}


