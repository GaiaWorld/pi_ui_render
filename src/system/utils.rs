use bevy::{ecs::{
	prelude::RemovedComponents,
    prelude::Component,
    system::{Commands, Query},
}};
use geo::BooleanOps;
use pi_bevy_render_plugin::{PiVertexBufferAlloter, PiIndexBufferAlloter};
use pi_render::{renderer::vertices::{EVerticesBufferUsage, RenderVertices, RenderIndices}, rhi::buffer_alloc::BufferIndex};
use pi_share::Share;
use pi_style::style::Aabb2;
use wgpu::IndexFormat;

use crate::{components::{calc::DrawList, user::{Vector4, Matrix4, Point2, Vector2}, draw_obj::DrawState}, resource::RenderObjType};

pub fn clear_draw_obj<'w, 's, T: Component>(
    render_type: RenderObjType,
    mut del: RemovedComponents<'w, 's, T>,
    mut query: Query<'w, 's, (Option<&T>, &mut DrawList)>,
    commands: &mut Commands,
) {
    for del in del.iter() {
        if let Ok((bg_color, mut draw_list)) = query.get_mut(del) {
            if bg_color.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            if let Some(draw_obj) = draw_list.remove(*render_type as u32) {
                commands.entity(draw_obj).despawn();
            }
        }
    }
}

pub fn clear_draw_obj_mul<'w, 's, T: Component>(
    render_types: &[RenderObjType],
    mut del: RemovedComponents<'w, 's, T>,
    mut query: Query<'w, 's, (Option<&'static T>, &'static mut DrawList)>,
    commands: &mut Commands,
) {
    for del in del.iter() {
        if let Ok((bg_color, mut draw_list)) = query.get_mut(del) {
            if bg_color.is_some() {
                continue;
            }
            // 删除对应的DrawObject
			for i in render_types.iter() {
				if let Some(draw_obj) = draw_list.remove(**i as u32) {
					commands.entity(draw_obj).despawn();
				}
			}
        }
    }
}

// 将四边形放进数组中
pub fn push_quad(index_arr: &mut Vec<u16>, p1: u16, p2: u16, p3: u16, p4: u16) { index_arr.extend_from_slice(&[p1, p2, p3, p1, p3, p4]); }

pub fn rotatequad_quad_intersection(
	rotatequad: &(Vector2, Vector2, Vector2, Vector2),
	rotate_matrix: &Matrix4,
	quad: &Aabb2,
) -> Aabb2 {
	let left_top = rotate_matrix * Vector4::new(rotatequad.0.x, rotatequad.0.y, 0.0, 1.0);
	let left_bottom = rotate_matrix * Vector4::new(rotatequad.1.x, rotatequad.1.y, 0.0, 1.0);
	let right_bottom = rotate_matrix * Vector4::new(rotatequad.2.x, rotatequad.2.y, 0.0, 1.0);
	let right_top = rotate_matrix * Vector4::new(rotatequad.3.x, rotatequad.3.y, 0.0, 1.0);

	let rotate_quad:  geo::Polygon<f32> = geo::Polygon::new(geo::LineString::from(vec![
		(left_top.x, left_top.y), 
		(left_bottom.x, left_bottom.y), 
		(right_bottom.x, right_bottom.y), 
		(right_top.x, right_top.y), 
		(left_top.x, left_top.y)
	]), vec![]);

	let quad: geo::Polygon<f32> = geo::Polygon::new(geo::LineString::from(vec![
		(quad.mins.x, quad.mins.y), 
		(quad.mins.x, quad.maxs.y), 
		(quad.maxs.x, quad.maxs.y), 
		(quad.maxs.x, quad.mins.y), 
		(quad.mins.x, quad.mins.y)
	]), vec![]);
	
	let (mut min_x, mut min_y, mut max_x, mut max_y) = (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN);
	let intersect = rotate_quad.intersection(&quad);

	for i in intersect.into_iter() {
		for coord in i.exterior() {
			min_x = min_x.min(coord.x);
			min_y = min_y.min(coord.y);
			max_x = max_x.max(coord.x);
			max_y = max_y.max(coord.y);
		}
	}

	if min_x != std::f32::MAX {
		// 取当前裁剪区域与父裁剪区域相交部分
		Aabb2::new(Point2::new(min_x, min_y), Point2::new(max_x, max_y))
	} else {
		// 与父裁剪区域不想交， 则设置裁剪区域大小为0
		Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0))
	}
}

pub fn set_vert_buffer(
	slot: u32,
	size_per_value: u64,
    buffer: &[u8],
    // label: &'static str,
    // device: &RenderDevice,
    // buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	vertex_buffer_alloter: &PiVertexBufferAlloter,
	draw_state: &mut DrawState,
	
) {
	match draw_state.vertices.get_mut(slot) {
		Some(r) => if let EVerticesBufferUsage::Part(index) = &mut r.buffer {
			// 正常逻辑下， 只有这里会取到可变，这里直接通过非安全方式转换（逻辑需要保证）
			vertex_buffer_alloter.update(unsafe{&mut *(Share::as_ptr(index) as usize as *mut BufferIndex)}, buffer);
		},
		None => {
			let index = vertex_buffer_alloter.alloc(buffer);
			draw_state.insert_vertices(RenderVertices { slot: slot, buffer: EVerticesBufferUsage::Part(Share::new(index)), buffer_range: None, size_per_value })
		}
	}
	
    // let key = calc_hash_slice(buffer, calc_hash(&"vert", 0));
    // match buffer_assets.get(&key) {
    //     Some(r) => r,
    //     None => {
    //         let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
    //             label: Some(label),
    //             contents: buffer,
    //             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //         });
    //         buffer_assets.insert(key, RenderRes::new(uniform_buf, buffer.len())).unwrap()
    //     }
    // }
}

pub fn set_index_buffer(
    buffer: &[u8],
    // label: &'static str,
    // device: &RenderDevice,
    // buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	index_buffer_alloter: &PiIndexBufferAlloter,
	draw_state: &mut DrawState,
	
) {
	if let Some(i) = &mut draw_state.indices {
		if let EVerticesBufferUsage::Part(index) = &mut i.buffer {
			// 正常逻辑下， 只有这里会取到可变，这里直接通过非安全方式转换（逻辑需要保证）
			index_buffer_alloter.update(unsafe{&mut *(Share::as_ptr(index) as usize as *mut BufferIndex)}, buffer);
			return;
		}
	}
	let index = index_buffer_alloter.alloc(buffer);
	draw_state.indices = Some(RenderIndices {buffer: EVerticesBufferUsage::Part(Share::new(index)), buffer_range: None, format: IndexFormat::Uint16 });
}