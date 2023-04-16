use bevy::ecs::{
	prelude::RemovedComponents,
    prelude::Component,
    system::{Commands, Query},
};
use geo::BooleanOps;
use pi_style::style::Aabb2;

use crate::{components::{calc::DrawList, user::{Vector4, Matrix4, Point2, Vector2}}, resource::RenderObjType};

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
