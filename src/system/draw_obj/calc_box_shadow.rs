use pi_flex_layout::prelude::Rect;
use pi_null::Null;
use pi_style::style::{Aabb2, CgColor, Point2, StyleType};
use pi_world::event::{ComponentAdded, ComponentChanged, ComponentRemoved};
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};
use pi_world::single_res::{SingleRes, SingleResMut};
use std::hash::Hash;
use std::hash::Hasher;
use pi_world::world::Entity;
use std::ops::Range;

use crate::components::calc::{style_bit, DrawList, LayoutResult, StyleBit, StyleMarkType};
use crate::components::draw_obj::{BoxShadowMark, BoxType, InstanceIndex, RenderCount};
use crate::resource::{BoxShadowRenderObjType, GlobalDirtyMark, OtherDirtyType, ShareFontSheet};
use crate::resource::draw_obj::InstanceContext;
use crate::components::user::BoxShadow;
use crate::shader1::batch_meterial::{ColorUniform, SdfUniform, StrokeColorUniform};
use crate::system::base::node::layout::calc_layout;
use crate::system::draw_obj::geo_split::{grid_split_simple, set_grid_instance};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::{self, update_render_instance_data};
use crate::resource::IsRun;
use crate::utils::tools::calc_hash;

use super::geo_split::GridBufer;

pub struct BoxShadowPlugin;

impl Plugin for BoxShadowPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		 // BoxShadow功能
		app
		// .add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
		.add_system(
			UiStage, 
			life_drawobj::draw_object_life_new::<
				BoxShadow,
				BoxShadowRenderObjType,
				BoxShadowMark,
				{ BOX_SHADOW_ORDER },
				{ BoxType::None },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.run_if(box_shadow_life_change)
				.before(calc_box_shadow),
		)
		.add_system(
			UiStage, 
			calc_box_shadow
				.after(crate::system::base::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
				.run_if(box_shadow_change)
		)
		.add_system(UiStage, 
			calc_box_shadow_instace_count
				.after(UiSystemSet::LifeDrawObjectFlush)
				.before(update_render_instance_data)
				.after(calc_layout)
				.run_if(box_shadow_change)
				.in_set(UiSystemSet::IsRun)
		)
		;
    }
}

pub const BOX_SHADOW_ORDER: u8 = 1;

#[derive(Default)]
pub struct BoxShadowTemp (pub GridBufer, pub Vec<(Entity, CgColor, [(Range<usize>, Range<usize>); 9])>);

/// 设置设置boxShadow颜色、偏移、模糊半径
pub fn calc_box_shadow_instace_count(
	font_sheet: SingleResMut<ShareFontSheet>,
	mut grid_buffer: OrInitSingleResMut<BoxShadowTemp>,
    query: Query<(&BoxShadow, &DrawList, &LayoutResult, &Layer), Or<(Changed<BoxShadow>, Changed<LayoutResult>, Changed<Layer>)>>,
	changed: ComponentChanged<BoxShadow>,
	added: ComponentAdded<BoxShadow>,
    mut query_draw: Query<&mut RenderCount, With<BoxShadowMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BoxShadowRenderObjType>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
) {
	if r.0 {
		return;
	}
	log::debug!("calc_box_shadow_instace_count========================");
	let render_type = ***render_type;
	let mut font_sheet = font_sheet.borrow_mut();
	let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;
	let grid_buffer = &mut **grid_buffer;

	for entity in changed.iter().chain(added.iter()) {
		log::debug!("calc_box_shadow_instace_count0========================");
		if let Ok((box_shadow, draw_list, layout, layer)) = query.get(*entity) {
			if layer.layer().is_null() {
				continue;
			}
			log::debug!("calc_box_shadow_instace_count1========================");
			let draw_id = match draw_list.get_one(render_type) {
				Some(r) => r.id,
				None => continue,
			};
			log::debug!("calc_box_shadow_instace_count2========================");

			let mut render_count = query_draw.get_mut(draw_id).unwrap();

			let layout_width = layout.rect.width();
			let layout_height = layout.rect.height();

			let info = BoxShadowInfo::new(box_shadow.blur, box_shadow.spread, layout_width, layout_height);

			let hash = calc_hash(&info, 0);

			let sdf_glyph = match sdf2_table.shapes_shadow_tex_info.get(&(hash, info.blur as u32)) {
                Some(r) => r,
                None => {
                    sdf2_table.add_box_shadow(
						hash,  
						Aabb2::new(Point2::new(0.0, 0.0), Point2::new(info.sdf_width as f32, info.sdf_height as f32)),
						info.sdf_height.max(info.sdf_width),
						info.blur as u32);
                    sdf2_table.shapes_shadow_tex_info.get(&(hash, info.blur as u32)).unwrap()
                },
            };

			let extend = info.blur as f32 + box_shadow.spread;
			let blur2 = info.blur as f32 * 2.0;

			let mut layout_rect = layout.border_rect();
			layout_rect.left = layout_rect.left - extend + box_shadow.h;
			layout_rect.top = layout_rect.top - extend + box_shadow.v;
			layout_rect.right = layout_rect.right + extend + box_shadow.h;
			layout_rect.bottom = layout_rect.bottom + extend + box_shadow.v;

			let layout_slice = Rect {
				left: layout_rect.left + blur2,
				right: layout_rect.right - blur2,
				top: layout_rect.top + blur2,
				bottom: layout_rect.bottom - blur2,
			};

			let sdf_slice = Rect {
				top: sdf_glyph.y as f32 + info.blur as f32,
				left: sdf_glyph.x as f32 + info.blur as f32,
				right: sdf_glyph.x as f32 + sdf_glyph.width as f32 - info.blur as f32,
				bottom: sdf_glyph.y as f32 + sdf_glyph.height as f32 - info.blur as f32,
			};

			let sdf_uv = Rect {
				top: sdf_slice.top - blur2,
				left: sdf_slice.left - blur2,
				right: sdf_slice.right  + blur2,
				bottom: sdf_slice.bottom + blur2,
			};

			let (count, range) = grid_split_simple(
				&mut grid_buffer.0, 
				&layout_rect,
				&sdf_uv,
				&sdf_slice,
				&layout_slice
			);
			log::debug!("box_shadow======{:?}", (count, &range, sdf_glyph, layout_rect, layout_slice, box_shadow, layout.border_rect()));
			grid_buffer.1.push((draw_id, box_shadow.color.clone(), range));


			if render_count.transparent != count as u32 {
				render_count.transparent = count as u32;
				global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
			}
		}
	}
}

/// 设置边框颜色的顶点、索引、和边框颜色uniform
pub fn calc_box_shadow(
	mut grid_buffer: OrInitSingleResMut<BoxShadowTemp>,
	mut instances: OrInitSingleResMut<InstanceContext>,

	query_draw: Query<&InstanceIndex, With<BoxShadowMark>>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
	log::debug!("calc_box_shadow0======");
	let grid_buffer = &mut **grid_buffer;
	// log::trace!("bg image========================{:?}", (mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY1), mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY2)));
	for (draw_id, color, range) in grid_buffer.1.drain(..) {
		log::debug!("calc_box_shadow======{:?}", (&color, &range));
		if let Ok(instanceindex) = query_draw.get(draw_id) {
			let instanceindex = &instanceindex.transparent;
			let mut start = instanceindex.start;
			for (x_range, y_range) in range {
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
				1.0, -0.5, 0.0,
			]));

			log::debug!("calc_box_shadow==={:?}", (draw_id, color));
			
		}
	}

	grid_buffer.0.positions.clear();
	grid_buffer.0.sdf_uvs.clear();
}

lazy_static! {
	// 子节点脏， 仅设自身child_dirty
	pub static ref BOX_SHADOW_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BoxShadow as usize)
		.set_bit(OtherDirtyType::NodeTreeAdd as usize);
}

pub fn box_shadow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	// log::warn!("box_shadow_change==========={:?}", mark.mark.has_any(&*BOX_SHADOW_DIRTY));
	mark.mark.has_any(&*BOX_SHADOW_DIRTY)
}

pub fn box_shadow_life_change(mark: SingleRes<GlobalDirtyMark>, removed: ComponentRemoved<BoxShadow>) -> bool {
	let r = removed.len() > 0 || mark.mark.get(StyleType::BoxShadow as usize).map_or(false, |display| {*display == true});
	removed.mark_read();
	r
}

#[derive(Debug)]
struct BoxShadowInfo {
    pub blur: usize,
	pub ty: usize, // 阴影为什么类型（ 0： 宽度高度都不大于模糊半径，1：宽度大于， 但高度不大于， 2. 高度大于， 但宽度不大于， 3： 宽度， 高度都大于）
	pub sdf_width: usize,
	pub sdf_height: usize,
}

impl BoxShadowInfo {
	fn new(blur: f32, spread: f32, mut width: f32, mut height: f32) -> BoxShadowInfo {
		let spread_2 = spread * 2.0;
		width = width + spread_2;
		height = height + spread_2;
		let blur = blur.round() as usize;
		let mut info = BoxShadowInfo {
			blur,
			ty: 0,
			sdf_width: 0,
			sdf_height: 0,
		};

		let blur_size = blur * 3;
		let size = (blur_size as f32 / 32.0 -0.001).ceil() as usize * 32;
		if width > blur_size as f32 {
			info.ty += 1;
			// 如果是宽度更大，则渲染一个固定尺寸的sdf
			info.sdf_width = size;
		} else {
			// 否则渲染width对应尺寸的sdf
			info.sdf_width = width as usize;
		}
		if height > blur_size as f32 {
			// 如果是高度更大，则渲染一个固定尺寸的sdf
			info.ty += 2;
			info.sdf_height = size;
		} else {
			// 否则渲染height对应尺寸的sdf
			info.sdf_height = height as usize;
		}

		info
	}
}

impl Hash for BoxShadowInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
		"box_shadow".hash(state);
		self.ty.hash(state);
		self.sdf_width.hash(state);
		self.sdf_height.hash(state);
        self.blur.hash(state);
    }
}




// use std::slice;

// use pi_world::query::{Changed, Or, With};
// use pi_world::system::{Query, SingleRes};
// use pi_assets::mgr::AssetMgr;
// use pi_bevy_ecs_extend::system_param::res::OrInitSingleRes;
// use pi_cg2d::Polygon;

// use pi_bevy_asset::ShareAssetMgr;
// use pi_bevy_render_plugin::PiRenderDevice;
// use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderIndices, RenderVertices};
// use pi_render::rhi::asset::RenderRes;
// use pi_render::rhi::buffer::Buffer;
// use pi_render::rhi::device::RenderDevice;
// use pi_render::rhi::shader::Input;
// use pi_share::Share;
// use polygon2::difference;
// use wgpu::IndexFormat;

// use crate::components::calc::LayoutResult;
// use crate::components::draw_obj::{BoxShadowMark, PipelineMeta};
// use crate::components::user::{BoxShadow, Point2};
// use crate::components::{calc::NodeId, draw_obj::DrawState};
// use crate::shader::color::{PositionVert, SHADOW_DEFINE};
// use crate::shader::ui_meterial::{BlurUniform, ColorUniform, StrokeColorOrURectUniform};
// use crate::utils::tools::{calc_float_hash, calc_hash, get_box_rect};

// use super::calc_text::IsRun;
// // use crate::utils::tools::calc_hash;

// pub const BOX_SHADOW_ORDER: u8 = 1;

// /// 设置阴影的顶点、索引，和阴影颜色、阴影模糊半径的uniform
// pub fn calc_box_shadow(
//     // 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
//     query: Query<(&BoxShadow, &LayoutResult), Or<(Changed<BoxShadow>, Changed<LayoutResult>)>>,
//     mut query_draw: Query<(&mut DrawState, &mut PipelineMeta, &NodeId), With<BoxShadowMark>>,

//     device: SingleRes<PiRenderDevice>,

//     buffer_assets: SingleRes<ShareAssetMgr<RenderRes<Buffer>>>,
// 	r: OrInitSingleRes<IsRun>
// ) {
// 	if r.0 {
// 		return;
// 	}
//     for (mut draw_state, mut pipeline_meta, node_id) in query_draw.iter_mut() {
//         if let Ok((box_shadow, layout)) = query.get(***node_id) {
//             modify(&device, &mut draw_state, layout, &box_shadow, &buffer_assets);
//             pipeline_meta.defines.insert(SHADOW_DEFINE.clone());
//         }
//     }
// }

// fn modify(
//     device: &RenderDevice,
//     draw_state: &mut DrawState,
//     layout: &LayoutResult,
//     box_shadow: &BoxShadow,
//     buffer_assets_mgr: &Share<AssetMgr<RenderRes<Buffer>>>,
// ) {
//     let border_box = get_box_rect(layout);
//     if *(border_box.right) - *(border_box.left) == 0.0 || *(border_box.bottom) - *(border_box.top) == 0.0 {
//         return;
//     }

//     let left = *(border_box.left) + box_shadow.h - box_shadow.spread - (box_shadow.blur / 2.0);
//     let top = *(border_box.top) + box_shadow.v - box_shadow.spread - (box_shadow.blur / 2.0);
//     let right = *border_box.right + box_shadow.spread + box_shadow.blur;
//     let bottom = *border_box.bottom + box_shadow.spread + box_shadow.blur;

//     let vb_hash = calc_hash(&"box_shadow vert", calc_float_hash(&[left, top, right, bottom, box_shadow.blur], 0));
//     let ib_hash = calc_hash(&"box_shadow index", calc_float_hash(&[left, top, right, bottom, box_shadow.blur], 0));

//     let (vb, ib) = match (buffer_assets_mgr.get(&vb_hash), buffer_assets_mgr.get(&ib_hash)) {
//         (Some(vb), Some(ib)) => (vb, ib),
//         (vb, ib) => {
//             let bg = vec![*border_box.left, *border_box.top, *border_box.left, *border_box.bottom, *border_box.right, *border_box.bottom, *border_box.right, *border_box.top];
//             let box_shadow = vec![left, top, left, bottom, right, bottom, right, top];

//             let polygon_shadow = convert_to_f32_tow(box_shadow.as_slice());
//             let polygon_bg = convert_to_f32_tow(bg.as_slice());
//             let difference_polygons = difference(polygon_shadow, polygon_bg);

//             let mut curr_index = 0;
//             let mut positions: Vec<f32> = vec![];
//             let mut indices: Vec<u16> = vec![];
//             for p_slice in difference_polygons.into_iter() {
//                 let p = Polygon::new(convert_to_point(convert_to_f32(p_slice.as_slice())));
//                 positions.extend_from_slice(convert_to_f32(p_slice.as_slice()));

//                 let tri_indices = p.triangulation();
//                 indices.extend_from_slice(tri_indices.iter().map(|&v| (v + curr_index) as u16).collect::<Vec<u16>>().as_slice());

//                 curr_index += p.vertices.len();
//             }

//             if positions.len() == 0 {
//                 return;
//             }

//             let vb = match vb {
//                 Some(r) => r,
//                 None => {
//                     let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                         label: Some("radius or linear Vertex Buffer"),
//                         contents: bytemuck::cast_slice(positions.as_slice()),
//                         usage: wgpu::BufferUsages::VERTEX,
//                     });
//                     buffer_assets_mgr.insert(vb_hash, RenderRes::new(buf, positions.len() * 4)).unwrap()
//                 }
//             };
//             let ib = match ib {
//                 Some(r) => r,
//                 None => {
//                     let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                         label: Some("radius or linear Index Buffer"),
//                         contents: bytemuck::cast_slice(indices.as_slice()),
//                         usage: wgpu::BufferUsages::INDEX,
//                     });
//                     buffer_assets_mgr.insert(ib_hash, RenderRes::new(buf, indices.len() * 2)).unwrap()
//                 }
//             };
//             (vb, ib)
//         }
//     };
//     draw_state.vertex = 0..(vb.size() / 8) as u32;
//     draw_state.insert_vertices(RenderVertices {
//         slot: PositionVert::location(),
//         buffer: EVerticesBufferUsage::GUI(vb),
//         buffer_range: None,
//         size_per_value: 8,
//     });
//     draw_state.indices = Some(RenderIndices {
//         buffer: EVerticesBufferUsage::GUI(ib),
//         buffer_range: None,
//         format: IndexFormat::Uint16,
//     });

//     let mut blur = box_shadow.blur;

//     let min_size = (right - left).min(bottom - top);
//     if blur * 2.0 > min_size {
//         blur = min_size / 2.0
//     }

//     // uniform
//     let color = &box_shadow.color;
//     draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));
//     draw_state
//         .bindgroups
//         .set_uniform(&StrokeColorOrURectUniform(&[left + blur, top + blur, right - blur, bottom - blur]));
//     draw_state.bindgroups.set_uniform(&BlurUniform(&[box_shadow.blur]));
// }

// #[inline]
// fn convert_to_point(pts: &[f32]) -> &[Point2] {
//     let ptr = pts.as_ptr();
//     let ptr = ptr as *const Point2;
//     unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
// }

// // #[inline]
// // fn convert_to_f32(pts: &[Point2]) -> &[f32] {
// //     let ptr = pts.as_ptr();
// //     let ptr = ptr as *const f32;
// //     unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
// // }

// #[inline]
// fn convert_to_f32_tow(pts: &[f32]) -> &[[f32; 2]] {
//     let ptr = pts.as_ptr();
//     let ptr = ptr as *const [f32; 2];
//     unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
// }

// #[inline]
// fn convert_to_f32(pts: &[[f32; 2]]) -> &[f32] {
//     let ptr = pts.as_ptr();
//     let ptr = ptr as *const f32;
//     unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
// }
