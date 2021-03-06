use std::intrinsics::transmute;
use std::io::Result;

use ordered_float::NotNan;
use pi_assets::asset::{Handle, Asset};
use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{Or, Deleted, With, ChangeTrackers, ParamSet, OrDefault};
use pi_ecs::prelude::{Query, Changed, EntityCommands, Commands, Write, Res, Event, Id};
use pi_ecs_macros::{listen, setup};
use pi_flex_layout::prelude::{Rect, Size};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_share::Share;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_polygon::{split_by_radius, find_lg_endp, split_by_lg, interp_mult_by_lg, LgCfg, mult_to_triangle, to_triangle};
use wgpu::IndexFormat;

use crate::components::calc::{LayoutResult, DrawInfo};
use crate::components::user::{CgColor, BorderRadius};
use crate::system::shader_utils::StaticIndex;
use crate::system::shader_utils::color::{ColorStaticIndex, COLOR_GROUP};
use crate::system::shader_utils::with_vert_color::WithVertColorStaticIndex;
use crate::utils::tools::{calc_hash, get_content_rect, get_content_radius};
use crate::{components::{user::{Node, BackgroundColor, Color}, calc::{NodeId, DrawList}, draw_obj::{BoxType, DrawObject, DrawState}}, resource::draw_obj::{Shaders, UnitQuadBuffer}};
// use crate::utils::tools::calc_hash;

pub struct CalcBackGroundColor;

#[setup]
impl CalcBackGroundColor {
	/// 创建RenderObject，用于渲染背景颜色
	#[system]
	pub async fn calc_background(
		mut query: ParamSet<(
			// 布局修改、颜色修改、圆角修改或删除，需要修改或创建背景色的DrawObject
			Query<'static, 'static, Node, (
				Id<Node>, 
				&'static BackgroundColor,
				Option<&'static BorderRadius>,
				&'static LayoutResult,
				Write<BackgroundDrawId>, 
				Write<DrawList>,
				ChangeTrackers<BackgroundColor>,
				ChangeTrackers<BorderRadius>,
				ChangeTrackers<LayoutResult>,
			), (With<BackgroundColor>, Or<(
				
				Changed<BackgroundColor>,
				Changed<BorderRadius>,
				Deleted<BorderRadius>,
				Changed<LayoutResult>,
			)>)>,

			// BackgroundColor删除，需要删除对应的DrawObject
			Query<'static, 'static, Node, (
				Option<&'static BackgroundColor>,
				Write<BackgroundDrawId>,
				Write<DrawList>,
			), Deleted<BackgroundColor>>
		)>,

		query_draw: Query<'static, 'static, DrawObject, (Write<DrawState>, OrDefault<BoxType>, &'static StaticIndex)>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut is_unit_quad_commands: Commands<DrawObject, BoxType>,
		mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		mut order_commands: Commands<DrawObject, DrawInfo>,
		
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'static, RenderDevice>,
		bg_static_index: Res<'static, ColorStaticIndex>,
		with_vert_color_static_index: Res<'static, WithVertColorStaticIndex>,
		shader_static: Res<'static, Shaders>,
		unit_quad_buffer: Res<'static, UnitQuadBuffer>,

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
			background_color, 
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
					let (mut draw_state_item, old_unit_quad, old_static_index) = query_draw.get_unchecked(**r);
					let draw_state = draw_state_item.get_mut().unwrap();
					let (new_static_index, new_unit_quad) = modify(
						&background_color, 
						radius,
						layout,
						draw_state,
						&device, 
						&buffer_assets, 
						&bind_group_assets,
						&background_color_change,
						&radius_change,
						&layout_change,
						&bg_static_index,
						&with_vert_color_static_index,
						&shader_static,
						&unit_quad_buffer).await;
					draw_state_item.notify_modify();
					if unsafe {transmute::<_, u64>(node)} == 4294967627 {
						println!("xxxxxxxxxxx")
					}
					if *old_unit_quad != new_unit_quad {
						is_unit_quad_commands.insert(**r, new_unit_quad);
					}

					if old_static_index != new_static_index {
						shader_static_commands.insert(**r, new_static_index.clone());
					}
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
					let (new_static_index, new_unit_quad) = modify(
						&background_color, 
						radius,
						layout,
						&mut draw_state,
						&device, 
						&buffer_assets, 
						&bind_group_assets,
						&background_color_change,
						&radius_change,
						&layout_change,
						&bg_static_index,
						&with_vert_color_static_index,
						&shader_static,
						&unit_quad_buffer).await;
					
					draw_state_commands.insert(new_draw_obj, draw_state);
					// 建立DrawObj对Node的索引
					node_id_commands.insert(new_draw_obj, NodeId(node));
					is_unit_quad_commands.insert(new_draw_obj, new_unit_quad);

					shader_static_commands.insert(new_draw_obj, new_static_index.clone());
					if unsafe {transmute::<_, u64>(node)} == 4294967627 {
						println!("xxxxxxxxxxx")
					}
					order_commands.insert(new_draw_obj, DrawInfo::new(9, background_color.is_opaque()));
					

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
#[listen(component=(Node, BackgroundColor, Delete), component=(Node, Node, Delete))]
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
	color: &Color, 
	radius: Option<&BorderRadius>, 
	layout: &LayoutResult,
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>,
	bg_color_change: &ChangeTrackers<BackgroundColor>,
	border_change: &ChangeTrackers<BorderRadius>,
	layout_change: &ChangeTrackers<LayoutResult>,
	color_static: &'a StaticIndex,
	linear_static: &'a StaticIndex,
	shader_static: &Shaders,
	unit_quad_buffer: &UnitQuadBuffer,
) -> (&'a StaticIndex, BoxType) {
	// modify_radius_linear_geo
	let static_index = match color {
		Color::RGBA(color) => {
			// 颜色改变，重新设置color_group
			if bg_color_change.is_changed() {
				let color_group_layout = shader_static.get(color_static.shader).unwrap().bind_group.get(COLOR_GROUP).unwrap();
				let color_bind_group = create_rgba_bind_group(color, device, color_group_layout, buffer_assets, bind_group_assets);
				// 插入color_bind_group到drawstate中
				draw_state.bind_groups.insert(COLOR_GROUP, color_bind_group);
			}
			
			color_static
		},
		_ => {
			linear_static
		},
	};

	let radius = get_content_radius(radius, layout);
	// 如果既没有圆角，也不是渐变色，则不需要切分顶点,直接设置单位四边形的ib、vb
	if radius.is_none() {
		if let Color::LinearGradient(_) = color{} 
		else{
			if border_change.is_changed() || bg_color_change.is_changed() {
				draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
				draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
			}
			return (static_index, BoxType::Content);
		}
	}

	// 否则，需要切分顶点，如果是渐变色，还要设置color vb
	// ib、position vb、color vb
	if border_change.is_changed() || bg_color_change.is_changed() || layout_change.is_changed() {
		try_modify_as_radius_linear_geo(
			&radius, 
			layout,
			device,
			draw_state,
			buffer_assets,
			color
		);
	}

	(static_index, BoxType::None)
}

pub fn create_rgba_bind_group(
	color: &CgColor,
	device: &RenderDevice, 
	color_group_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(&color, calc_hash(&"uniform", 0));
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
					buffer_assets.insert(key, RenderRes::new(uniform_buf, 5)).unwrap()
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
			bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap()
		}
	}
}


#[inline]
fn try_modify_as_radius_linear_geo(
    radius: &Option<Rect<NotNan<f32>>>,
    layout: &LayoutResult,
    device: &RenderDevice,
	darw_state: &mut DrawState,
	buffer_asset_mgr: &Share<AssetMgr<RenderRes<Buffer>>>,
	color: &Color,
) {
	let rect = get_content_rect(layout);
	let size = Size {width: rect.right - rect.left, height: rect.bottom - rect.top};
	let vb_hash = calc_hash(&(radius, rect), calc_hash(&"radius vert", 0));
	let ib_hash = calc_hash(&(radius, rect), calc_hash(&"radius index", 0));

	let (vb, ib, ) = match (buffer_asset_mgr.get(&vb_hash), buffer_asset_mgr.get(&ib_hash)) {
		(Some(vb), Some(ib)) => (vb, ib),
		(vb, ib) => {
			let (mut positions, mut indices) = match radius {
				Some(radius) => split_by_radius(
					layout.border.left,
					layout.border.top,
					*size.width,
					*size.height,
					*radius.left,
					None,
				),
				None => (
					vec![
                        *rect.left, *rect.top, // left_top
                        *rect.left, *rect.bottom, // left_bootom
                        *rect.right, *rect.bottom, // right_bootom
                        *rect.right, *rect.top, // right_top
                    ],
                    vec![0, 1, 2, 3],
				)
			};
			if let Color::LinearGradient(color) = color {
				let mut lg_pos = Vec::with_capacity(color.list.len());
				let mut colors = Vec::with_capacity(color.list.len() * 4);
				for v in color.list.iter() {
					lg_pos.push(v.position);
					colors.extend_from_slice(&[v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w]);
				}

				//渐变端点
				let endp = find_lg_endp(
					&[
						0.0,
						0.0,
						0.0,
						*size.height,
						*size.width,
						*size.height,
						*size.width,
						0.0,
					],
					color.direction,
				);

				let (positions1, indices1) = split_by_lg(
					positions,
					indices,
					lg_pos.as_slice(),
					endp.0.clone(),
					endp.1.clone(),
				);

				let mut colors = interp_mult_by_lg(
					positions1.as_slice(),
					&indices1,
					vec![Vec::new()],
					vec![LgCfg {
						unit: 4,
						data: colors,
					}],
					lg_pos.as_slice(),
					endp.0,
					endp.1,
				);

				indices = mult_to_triangle(&indices1, Vec::new());
				positions = positions1;

				let colors = colors.pop().unwrap();
				let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
					label: Some("radius or linear Color Buffer"),
					contents: bytemuck::cast_slice(colors.as_slice()),
					usage: wgpu::BufferUsages::VERTEX,
				});
				let color_hash = calc_hash(&(radius, rect), calc_hash(&"radius vert color", 0));

				let color = buffer_asset_mgr.get(&color_hash).unwrap_or_else(|| {buffer_asset_mgr.insert(color_hash, RenderRes::new(buf, colors.len() * 4)).unwrap()});
				darw_state.vbs.insert(1, (color, 0));
			} else {
				indices = to_triangle(&indices, Vec::with_capacity(indices.len()));
			}
			let vb = match vb {
				Some(r) => r,
				None => {
					let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("radius or linear Vertex Buffer"),
						contents: bytemuck::cast_slice(positions.as_slice()),
						usage: wgpu::BufferUsages::VERTEX,
					});
					buffer_asset_mgr.insert(vb_hash, RenderRes::new(buf, positions.len() * 4)).unwrap()
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
					buffer_asset_mgr.insert(ib_hash, RenderRes::new(buf, indices.len() * 2)).unwrap()
				}
			};
			(vb, ib)
		}
	};

	darw_state.vbs.insert(0, (vb, 0));
	let ib_size = (ib.size()/2) as u64;
	darw_state.ib =  Some((ib, ib_size, IndexFormat::Uint16));
}

