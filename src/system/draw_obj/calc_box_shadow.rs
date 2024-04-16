use bevy_app::Plugin;
use bevy_ecs::change_detection::DetectChangesMut;
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::{
    prelude::Ref,
    system::Query,
};
use bevy_ecs::prelude::DetectChanges;
use bevy_window::AddFrameEvent;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};

use crate::components::calc::{LayoutResult, WorldMatrix, DrawList};
use crate::components::draw_obj::{InstanceIndex, BoxShadowMark};
use crate::resource::BoxShadowRenderObjType;
use crate::resource::draw_obj::InstanceContext;
use crate::shader1::meterial::{RenderFlagType, ColorUniform, TyUniform, BoxShadowUniform};
use crate::components::user::BoxShadow;
use crate::system::draw_obj::set_box;
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiSchedule;

use super::calc_text::IsRun;
use super::life_drawobj;

pub struct BoxShadowPlugin;

impl Plugin for BoxShadowPlugin {
    fn build(&self, app: &mut bevy_app::App) {
		 // BoxShadow功能
		app
		.add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
		.add_systems(
			UiSchedule, 
			life_drawobj::draw_object_life_new::<
				BoxShadow,
				BoxShadowRenderObjType,
				BoxShadowMark,
				{ BOX_SHADOW_ORDER },
			>
				.in_set(UiSystemSet::LifeDrawObject)
				.before(calc_box_shadow),
		)
		.add_systems(
			UiSchedule, 
			calc_box_shadow
				.after(super::super::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
		);
    }
}

pub const BOX_SHADOW_ORDER: u8 = 1;

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_box_shadow(
	mut instances: OrInitResMut<InstanceContext>,
    query: Query<(Ref<WorldMatrix>, Ref<BoxShadow>, Ref<LayoutResult>, &DrawList), Or<(Changed<BoxShadow>, Changed<WorldMatrix>)>>,
    mut query_draw: Query<&InstanceIndex, With<BoxShadowMark>>,
	r: OrInitRes<IsRun>,
	render_type: OrInitRes<BoxShadowRenderObjType>,
) {
	if r.0 {
		return;
	}
	log::trace!("bg========================");
	let render_type = ***render_type;
	for (world_matrix, box_shadow, layout, draw_list) in query.iter() {
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

			if box_shadow.is_changed(){
				render_flag |= 1 << RenderFlagType::BoxShadow as usize;

				instance_data.set_data(&ColorUniform(&[box_shadow.color.x, box_shadow.color.y, box_shadow.color.z, box_shadow.color.w].as_slice()));
				instance_data.set_data(&BoxShadowUniform([box_shadow.h, box_shadow.v, box_shadow.spread, box_shadow.blur].as_slice()));
				instance_data.set_data(&TyUniform(&[render_flag as f32]));
			}

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			let is_add = box_shadow.is_added();
			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
			// if is_add || layout.is_changed() {
			// 	instance_data.set_data(&BoxUniform(layout.border_box().as_slice()));
			// }

			if is_add || layout.is_changed() || world_matrix.is_changed() {
				set_box(&world_matrix, &layout.border_aabb(), &mut instance_data);
			}
		}
	}
}




// use std::slice;

// use bevy_ecs::query::{Changed, Or, With};
// use bevy_ecs::system::{Query, Res};
// use pi_assets::mgr::AssetMgr;
// use pi_bevy_ecs_extend::system_param::res::OrInitRes;
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

//     device: Res<PiRenderDevice>,

//     buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,
// 	r: OrInitRes<IsRun>
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
