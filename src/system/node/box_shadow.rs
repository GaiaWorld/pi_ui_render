use std::io::Result;
use std::slice;

use pi_assets::asset::Asset;
use pi_assets::mgr::AssetMgr;
use pi_cg2d::Polygon;
use pi_ecs::prelude::{Or, Deleted, With, ParamSet, ResMut};
use pi_ecs::prelude::{Query, Changed, EntityCommands, Commands, Write, Res, Event, Id};
use pi_ecs_macros::{listen, setup};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::dyn_uniform_buffer::{Group, Bind};
use pi_share::Share;
use pi_polygon::split_by_radius;
use polygon2::difference;
use wgpu::IndexFormat;
use smallvec::smallvec;

use crate::components::calc::{LayoutResult, DrawInfo};
use crate::components::draw_obj::{DrawGroup, DynDrawGroup, FSDefines, VSDefines};
use crate::components::user::{ BorderRadius, BoxShadow, Point2};
use crate::resource::draw_obj::{StaticIndex, DynUniformBuffer, DynBindGroupIndex, ColorStaticIndex};
use crate::shaders::color::{ColorMaterialGroup, PositionVertexBuffer, ColorMaterialBind, ColorUniform, UrectUniform, BlurUniform};
use crate::utils::tools::{calc_hash, get_content_radius, get_box_rect, calc_float_hash};
use crate::{components::{user::Node, calc::{NodeId, DrawList}, draw_obj::{DrawObject, DrawState}}, resource::draw_obj::Shaders};
// use crate::utils::tools::calc_hash;

pub struct CalcBoxShadow;

#[setup]
impl CalcBoxShadow {
	/// 创建RenderObject，用于渲染背景颜色
	#[system]
	pub async fn calc_box_shadow(
		mut query: ParamSet<(
			// 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
			Query<'static, 'static, Node, (
				Id<Node>, 
				&'static BoxShadow,
				Option<&'static BorderRadius>,
				&'static LayoutResult,
				Write<BoxShadowDrawId>, 
				Write<DrawList>
			), (With<BoxShadow>, Or<(
				
				Changed<BoxShadow>,
				Changed<BorderRadius>,
				Deleted<BorderRadius>,
				Changed<LayoutResult>,
			)>)>,

			// BackgroundColor删除，需要删除对应的DrawObject
			Query<'static, 'static, Node, (
				Option<&'static BoxShadow>,
				Write<BoxShadowDrawId>,
				Write<DrawList>,
			), Deleted<BoxShadow>>
		)>,

		query_draw: Query<'static, 'static, DrawObject, Write<DrawState>>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		mut order_commands: Commands<DrawObject, DrawInfo>,
		mut fs_defines_commands: Commands<DrawObject, FSDefines>,
		mut vs_defines_commands: Commands<DrawObject, VSDefines>,
		
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'static, RenderDevice>,
		shader_static: Res<'static, Shaders>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,

		color_static_index: Res<'static, ColorStaticIndex>,

		mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
		color_material_bind_group: Res<'static, DynBindGroupIndex<ColorMaterialGroup>>,
	) -> Result<()> {
		// log::info!("calc_background=================");
		for (
			background_color,
			mut draw_index,
			mut render_list) in query.p1_mut().iter_mut() {
			// BackgroundColor不存在时，删除对应DrawObject
			if background_color.is_some() {
				continue;
			};
			// 删除对应的DrawObject
			if let Some(draw_index_item) = draw_index.get() {
				draw_obj_commands.despawn(draw_index_item.0.clone());
				if let Some(r) = render_list.get_mut() {
					for i in 0..r.len() {
						let item = &r[i];
						if item == &draw_index_item.0 {
							r.swap_remove(i);
						}
					}
				}
				draw_index.remove();
			}
		}

		for (
			node, 
			box_shadow, 
			radius, 
			layout, 
			mut draw_index, 
			mut render_list) in query.p0_mut().iter_mut() {
			
			match draw_index.get() {
				// background_color已经存在一个对应的DrawObj， 则修改color group
				Some(r) => {
					let mut draw_state_item = query_draw.get_unchecked(**r);
					let draw_state = draw_state_item.get_mut().unwrap();
					modify(
						&device, 
						draw_state,
						layout,
						&box_shadow,
						radius,
						&buffer_assets,
						&bind_group_assets,
						&mut dyn_uniform_buffer);
					draw_state_item.notify_modify();
				},
				// 否则，创建一个新的DrawObj，并设置color group; 
				// 修改以下组件：
				// * <Node, BackgroundDrawId>
				// * <Node, DrawList>
				// * <DrawObject, DrawState>
				// * <DrawObject, NodeId>
				// * <DrawObject, IsUnitQuad>
				None => {
					// log::info!("create_background=================");
					// 创建新的DrawObj
					let new_draw_obj = draw_obj_commands.spawn();

					let mut vs_defines = VSDefines::default();
					vs_defines.insert("SHADOW".to_string());
					vs_defines_commands.insert(new_draw_obj, vs_defines);

					let mut fs_defines = FSDefines::default();
					fs_defines.insert("SHADOW".to_string());
					fs_defines_commands.insert(new_draw_obj, fs_defines);
					
					// 设置DrawState（包含color group）
					let mut draw_state = DrawState::default();

					// 创建color材质
					let color_material_dyn_offset = dyn_uniform_buffer.alloc_binding::<ColorMaterialBind>();
					let group = DrawGroup::Dyn(
						DynDrawGroup::new(
							(*color_material_bind_group).clone(),
							smallvec![color_material_dyn_offset]
						));
					draw_state.bind_groups.insert_group(ColorMaterialGroup::id(), group);

					modify(
						&device, 
						&mut draw_state,
						layout,
						&box_shadow,
						radius, 
						&buffer_assets,
						&bind_group_assets,
						&mut dyn_uniform_buffer);
					
					draw_state_commands.insert(new_draw_obj, draw_state);
					// 建立DrawObj对Node的索引
					node_id_commands.insert(new_draw_obj, NodeId(node));

					shader_static_commands.insert(new_draw_obj, (*color_static_index).clone());
					order_commands.insert(new_draw_obj, DrawInfo::new(8, false));

					// 建立Node对DrawObj的索引
					draw_index.write(BoxShadowDrawId(new_draw_obj));
					
					match render_list.get_mut() {
						Some(r) => {
							r.push(new_draw_obj);
							render_list.notify_modify();
						},
						None => {
							let mut r = DrawList::default();
							r.push(new_draw_obj);
							render_list.write(r);
						},
					};
				}
			}
		}
		return Ok(())
	}
}

#[derive(Deref, Default)]
pub struct BoxShadowDrawId(Id<DrawObject>);

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BoxShadow, Delete), component=(Node, Node, Delete))]
pub fn background_color_delete(
	e: Event,
	query: Query<Node, &BoxShadowDrawId>,
	mut draw_obj: EntityCommands<DrawObject>,
) {
	if let Some(index) = query.get_by_entity(e.id) {
		draw_obj.despawn(**index);
	}
}


fn modify(
    device: &RenderDevice,
    draw_state: &mut DrawState,
    layout: &LayoutResult,
    shadow: &BoxShadow,
    radius: Option<&BorderRadius>,
	buffer_assets_mgr: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets_mgr: &Share<AssetMgr<RenderRes<BindGroup>>>,
	
	dyn_uniform_buffer: &mut DynUniformBuffer,
) {
    let g_b = get_box_rect(layout);
    if *(g_b.right) - *(g_b.left) == 0.0 || *(g_b.bottom) - *(g_b.top) == 0.0 {
		return;
    }

	let radius = get_content_radius(radius, layout);

    let x = *(g_b.left) + shadow.h - shadow.spread - (shadow.blur/2.0);
    let y = *(g_b.top) + shadow.v - shadow.spread - (shadow.blur/2.0);
    let w = *(g_b.right) - *(g_b.left) + 2.0 * shadow.spread + shadow.blur;
    let h = *(g_b.bottom) - *(g_b.top) + 2.0 * shadow.spread + shadow.blur;

	let vb_hash = calc_hash(&(radius, calc_float_hash(&[x, y, h, w, shadow.blur], 0)), calc_hash(&"vert", 0));
	let ib_hash = calc_hash(&(radius, calc_float_hash(&[x, y, h, w, shadow.blur], 0)), calc_hash(&"index", 0));
	
	let (vb, ib) = match (buffer_assets_mgr.get(&vb_hash), buffer_assets_mgr.get(&ib_hash)) {
		(Some(vb), Some(ib)) => (vb, ib),
		(vb, ib) => {
			let radius = match radius {
				Some(r) => *(r.left),
				None => 0.0
			};
			// geo
			let x1 = *g_b.left;
			let y1 = *g_b.top;
			let w1 = *g_b.right - *g_b.left;
			let h1 = *g_b.bottom - *g_b.top;
			let bg = split_by_radius(x1, y1, w1, h1, radius, Some(16));
			if bg.0.len() == 0 {
				return;
			}

			let shadow_pts = split_by_radius(x, y, w, h, radius, Some(16));
			if bg.0.len() == 0 {
				return;
			}

			let polygon_shadow = convert_to_f32_tow(shadow_pts.0.as_slice());
			let polygon_bg = convert_to_f32_tow(bg.0.as_slice());
			let difference_polygons = difference (polygon_shadow, polygon_bg);

			// let polygon_shadow = Polygon::new(convert_to_point(shadow_pts.0.as_slice()));
			// let polygon_bg = Polygon::new(convert_to_point(bg.0.as_slice()));

			let mut curr_index = 0;
			let mut positions: Vec<f32> = vec![];
			let mut indices: Vec<u16> = vec![];
			for p_slice in difference_polygons.into_iter() {
				let p = Polygon::new(convert_to_point(convert_to_f32(p_slice.as_slice())));
				positions.extend_from_slice(convert_to_f32(p_slice.as_slice()));

				let tri_indices = p.triangulation();
				indices.extend_from_slice(
					tri_indices
						.iter()
						.map(|&v| (v + curr_index) as u16)
						.collect::<Vec<u16>>()
						.as_slice(),
				);

				curr_index += p.vertices.len();
			}

			if positions.len() == 0 {
				return;
			}

			let vb = match vb {
				Some(r) => r,
				None => {
					let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("radius or linear Vertex Buffer"),
						contents: bytemuck::cast_slice(positions.as_slice()),
						usage: wgpu::BufferUsages::VERTEX,
					});
					buffer_assets_mgr.insert(vb_hash, RenderRes::new(buf, positions.len() * 4)).unwrap()
				}
			};
			let ib = match ib {
				Some(r) => r,
				None => {
					let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("radius or linear Index Buffer"),
						contents: bytemuck::cast_slice(indices.as_slice()),
						usage: wgpu::BufferUsages::INDEX,
					});
					buffer_assets_mgr.insert(ib_hash, RenderRes::new(buf, indices.len() * 2)).unwrap()
				},
			};
			(vb, ib)
		}
	};
	draw_state.vbs.insert(PositionVertexBuffer::id() as usize, (vb, 0));
	let size = ib.size()/2;
	draw_state.ib = Some((ib, size as u64, IndexFormat::Uint16));

	let mut blur = shadow.blur;

	let min_size = w.min(h);
	if blur * 2.0 > min_size {
		blur = min_size / 2.0
	}
	
	// uniform
	let color_dyn_offset = draw_state.bind_groups.get_group(ColorMaterialGroup::id()).unwrap().get_offset(ColorMaterialBind::index()).unwrap();
	let color = &shadow.color;
	dyn_uniform_buffer.set_uniform(color_dyn_offset, &ColorUniform(&[color.x, color.y, color.z, color.w]));
	dyn_uniform_buffer.set_uniform(color_dyn_offset, &UrectUniform(&[x + blur, y + blur, x + w - blur, y + h - blur]));
	dyn_uniform_buffer.set_uniform(color_dyn_offset, &BlurUniform(&[shadow.blur]));

}

#[inline]
fn convert_to_point(pts: &[f32]) -> &[Point2] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const Point2;
    unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
}

// #[inline]
// fn convert_to_f32(pts: &[Point2]) -> &[f32] {
//     let ptr = pts.as_ptr();
//     let ptr = ptr as *const f32;
//     unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
// }

#[inline]
fn convert_to_f32_tow(pts: &[f32]) -> &[[f32; 2]] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const [f32; 2];
    unsafe { slice::from_raw_parts(ptr, pts.len() / 2) }
}

#[inline]
fn convert_to_f32(pts: &[[f32; 2]]) -> &[f32] {
    let ptr = pts.as_ptr();
    let ptr = ptr as *const f32;
    unsafe { slice::from_raw_parts(ptr, 2 * pts.len()) }
}