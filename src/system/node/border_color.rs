use std::io::Result;

use ordered_float::NotNan;
use pi_assets::asset::{Handle, Asset};
use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{Or, Deleted, With, ChangeTrackers, ParamSet};
use pi_ecs::prelude::{Query, Changed, EntityCommands, Commands, Write, Res, Event, Id};
use pi_ecs_macros::{listen, setup};
use pi_flex_layout::prelude::Rect;
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_share::Share;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_polygon::split_by_radius_border;
use wgpu::IndexFormat;

use crate::components::calc::LayoutResult;
use crate::components::user::{CgColor, BorderRadius};
use crate::system::shader_utils::StaticIndex;
use crate::system::shader_utils::color::{ColorStaticIndex, COLOR_GROUP};
use crate::utils::tools::{calc_hash, get_content_radius};
use crate::{
	components::{
		user::{Node, BorderColor}, 
		calc::{NodeId, DrawList}, 
		draw_obj::{DrawObject, DrawState}
	}, 
	resource::draw_obj::Shaders,
};
// use crate::utils::tools::calc_hash;

pub struct CalcBorderColor;

#[setup]
impl CalcBorderColor {
	/// 创建RenderObject，用于渲染背景颜色
	#[system]
	pub async fn calc_border_color(
		mut query: ParamSet<(
			// 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
			Query<Node, (
				Id<Node>, 
				&'static BorderColor,
				Option<&'static BorderRadius>,
				&'static LayoutResult,
				Write<BackgroundDrawId>, 
				Write<DrawList>,
				ChangeTrackers<BorderColor>,
				ChangeTrackers<BorderRadius>,
				ChangeTrackers<LayoutResult>,
			), (With<BorderColor>, Or<(
				
				Changed<BorderColor>,
				Changed<BorderRadius>,
				Deleted<BorderRadius>,
				Changed<LayoutResult>,
			)>)>,

			// BackgroundColor删除，需要删除对应的DrawObject
			Query<Node, (
				Option<&'static BorderColor>,
				Write<BackgroundDrawId>,
				Write<DrawList>,
			), Deleted<BorderColor>>
		)>,

		query_draw: Query<DrawObject, Write<DrawState>>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'static, RenderDevice>,
		static_index: Res<'static, ColorStaticIndex>,
		shader_static: Res<'static, Shaders>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
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
			border_color, 
			radius, 
			layout, 
			mut draw_index, 
			mut render_list,
			background_color_change,
			radius_change,
			layout_change) in query.p0_mut().iter_mut() {

			match draw_index.get() {
				// background_color已经存在一个对应的DrawObj， 则修改color group
				Some(r) => {
					let mut draw_state_item = query_draw.get_unchecked(**r);
					let draw_state = draw_state_item.get_mut().unwrap();
					modify(
						border_color, 
						radius,
						layout,
						draw_state,
						&device, 
						&buffer_assets, 
						&bind_group_assets,
						&background_color_change,
						&radius_change,
						&layout_change,
						&static_index,
						&shader_static).await;
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
					// 设置DrawState（包含color group）
					let mut draw_state = DrawState::default();
					modify(
						&border_color, 
						radius,
						layout,
						&mut draw_state,
						&device, 
						&buffer_assets, 
						&bind_group_assets,
						&background_color_change,
						&radius_change,
						&layout_change,
						&static_index,
						&shader_static).await;
					
					draw_state_commands.insert(new_draw_obj, draw_state);
					// 建立DrawObj对Node的索引
					node_id_commands.insert(new_draw_obj, NodeId(node));
					shader_static_commands.insert(new_draw_obj, static_index.clone());

					// 建立Node对DrawObj的索引
					draw_index.write(BackgroundDrawId(new_draw_obj));
					
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
pub struct BackgroundDrawId(Id<DrawObject>);

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BorderColor, Delete), component=(Node, Node, Delete))]
pub fn background_color_delete(
	e: Event,
	query: Query<Node, &BackgroundDrawId>,
	mut draw_obj: EntityCommands<DrawObject>,
) {
	if let Some(index) = query.get_by_entity(e.id) {
		draw_obj.despawn(**index);
	}
}

// 返回当前需要的StaticIndex
async fn modify<'a> (
	color: &CgColor, 
	radius: Option<&BorderRadius>, 
	layout: &LayoutResult,
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
	bg_color_change: &ChangeTrackers<BorderColor>,
	border_change: &ChangeTrackers<BorderRadius>,
	layout_change: &ChangeTrackers<LayoutResult>,
	color_static: &'a StaticIndex,
	shader_static: &Shaders,
) {
	// 颜色改变，重新设置color_group
	if bg_color_change.is_changed() {
		let color_group_layout = shader_static.get(color_static.shader).unwrap().bind_group.get(COLOR_GROUP).unwrap();
		let color_bind_group = create_rgba_bind_group(color, device, color_group_layout, buffer_assets, bind_group_assets);
		// 插入color_bind_group到drawstate中
		draw_state.bind_groups.insert(COLOR_GROUP, color_bind_group);
	}

	// 否则，需要切分顶点，如果是渐变色，还要设置color vb
	// ib、position vb、color vb
	if border_change.is_changed() || layout_change.is_changed() {
		let radius = get_content_radius(radius, layout);
		let vert_key = calc_hash(&("border color vert", &radius)); // layout TODO
		let index_key = calc_hash(&("border color index", &radius)); // layout TODO
		let (vert, index) = match (buffer_assets.get(&vert_key), buffer_assets.get(&index_key)) {
			(Some(v), Some(i)) => (v, i),
			(v, i) => {
				let (vert, indices) = get_geo_flow(&radius, layout);
				(
					match v {
						Some(r) => r,
						None => {
							let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
								label: Some("border Vertex Buffer"),
								contents: bytemuck::cast_slice(vert.as_slice()),
								usage: wgpu::BufferUsages::VERTEX,
							});
							buffer_assets.cache(vert_key, RenderRes::new(buf, vert.len() * 4));
							buffer_assets.get(&vert_key).unwrap()
						}
					},
					match i {
						Some(r) => r,
						None => {
							let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
								label: Some("border Index Buffer"),
								contents: bytemuck::cast_slice(indices.as_slice()),
								usage: wgpu::BufferUsages::INDEX,
							});
							buffer_assets.cache(index_key, RenderRes::new(buf, indices.len() * 2));
							buffer_assets.get(&index_key).unwrap()
						}
					},
				)

			}
		};
		draw_state.vbs.insert(0, (vert, 0));
		let size = (index.size() / 2) as u64;
		draw_state.ib = Some((index, size, IndexFormat::Uint16));
	}
}

pub fn create_rgba_bind_group(
	color: &CgColor,
	device: &RenderDevice, 
	color_group_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&("color", color));
	match bind_group_assets.get(&key) {
		Some(r) => r,
		None => {
			let uniform_buf = match buffer_assets.get(&key) {
				Some(r) => r,
				None => {
					let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("color buffer init"),
						contents: bytemuck::cast_slice(&[color.x, color.y, color.z, color.w]),
						usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
					});
					buffer_assets.cache(key, RenderRes::new(uniform_buf, 5));
					buffer_assets.get(&key).unwrap()
				}
			};
			let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: color_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: uniform_buf.as_entire_binding(),
					},
				],
				label: Some("color group create"),
			});
			bind_group_assets.cache(key, RenderRes::new(group, 5));
			bind_group_assets.get(&key).unwrap()
		}
	}
}


#[inline]
/// 取几何体的顶点流和属性流
fn get_geo_flow(radius: &Option<Rect<NotNan<f32>>>, layout: &LayoutResult) -> (Vec<f32>, Vec<u16>) {

	let width = layout.rect.right - layout.rect.left;
	let height = layout.rect.bottom - layout.rect.top;
	match radius {
		None => {
			let border_start_x = layout.border.left;
			let border_start_y = layout.border.top;
			let border_end_x = width - layout.border.right;
			let border_end_y = height - layout.border.bottom;

			(
				vec![
					0.0,
					0.0,
					0.0,
					height,
					width,
					height,
					width,
					0.0,
					border_start_x,
					border_start_y,
					border_start_x,
					border_end_y,
					border_end_x,
					border_end_y,
					border_end_x,
					border_start_y,
				],
				vec![
					0, 1, 4, 0, 4, 3, 3, 4, 7, 3, 7, 2, 2, 7, 6, 2, 6, 1, 1, 6, 5, 1, 5, 4,
				],
			)
		},
		Some(radius) => {
			split_by_radius_border(
				0.0,
				0.0,
				width,
				height,
				*radius.left,
				layout.border.left,
				None,
			)
		}
	}
}

