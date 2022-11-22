use std::hash::{Hash, Hasher};

use num_traits::Float;
use ordered_float::NotNan;
use pi_flex_layout::prelude::{Rect, Size};
use pi_hash::DefaultHasher;

use crate::components::{
    calc::{LayoutResult, WorldMatrix},
    user::{Aabb2, BorderRadius, LengthUnit, Matrix4, Point2, Vector4},
};

const EPSILON: f32 = std::f32::EPSILON * 1024.0;
#[inline]
pub fn eq_f32(v1: f32, v2: f32) -> bool { v1 == v2 || ((v2 - v1).abs() <= EPSILON) }

pub fn calc_hash<T: Hash>(v: &T, cur: u64) -> u64 {
    let mut hasher = DefaultHasher::default();
    cur.hash(&mut hasher);
    v.hash(&mut hasher);
    hasher.finish()
}

pub fn calc_hash_slice<T: Hash>(v: &[T], cur: u64) -> u64 {
    let mut hasher = DefaultHasher::default();
    cur.hash(&mut hasher);
    for i in v.iter() {
        i.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn calc_float_hash<T: Float>(v: &[T], cur: u64) -> u64 {
    let mut hasher = DefaultHasher::default();
    cur.hash(&mut hasher);
    for i in v.iter() {
        unsafe { NotNan::new_unchecked(*i) }.hash(&mut hasher);
    }
    hasher.finish()
}

// 计算aabb
pub fn calc_aabb(aabb: &Aabb2, matrix: &WorldMatrix) -> Aabb2 {
    let min = matrix * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
    let max = matrix * Vector4::new(aabb.maxs.x, aabb.maxs.y, 0.0, 1.0);

    let min = Point2::new(min.x, min.y);
    let max = Point2::new(max.x, max.y);

    Aabb2::new(min, max)
}

// 计算包围盒
pub fn calc_bound_box(aabb: &Aabb2, matrix: &Matrix4) -> Aabb2 {
    let left_top = matrix * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
    let right_top = matrix * Vector4::new(aabb.maxs.x, aabb.mins.y, 0.0, 1.0);
    let left_bottom = matrix * Vector4::new(aabb.mins.x, aabb.maxs.y, 0.0, 1.0);
    let right_bottom = matrix * Vector4::new(aabb.maxs.x, aabb.maxs.y, 0.0, 1.0);

    let min = Point2::new(
        left_top.x.min(right_top.x).min(left_bottom.x).min(right_bottom.x),
        left_top.y.min(right_top.y).min(left_bottom.y).min(right_bottom.y),
    );

    let max = Point2::new(
        left_top.x.max(right_top.x).max(left_bottom.x).max(right_bottom.x),
        left_top.y.max(right_top.y).max(left_bottom.y).max(right_bottom.y),
    );

    Aabb2::new(min, max)
}

// 两个aabb，求整体包围盒
pub fn box_aabb(aabb1: &mut Aabb2, aabb2: &Aabb2) {
    aabb1.mins.x = aabb1.mins.x.min(aabb2.mins.x);
    aabb1.mins.y = aabb1.mins.y.min(aabb2.mins.y);
    aabb1.maxs.x = aabb1.maxs.x.max(aabb2.maxs.x);
    aabb1.maxs.y = aabb1.maxs.y.max(aabb2.maxs.y);
}

pub struct BorderRadiusPixel {
    pub x: [f32; 4],
	pub y: [f32; 4],
}

/// 计算圆角半径
pub fn cal_border_radius(border_radius: &BorderRadius, layout: &LayoutResult) -> BorderRadiusPixel {
	#[inline]
	fn trans(l: LengthUnit, size: f32) -> f32 {
		match l {
			LengthUnit::Pixel(r) => r,
			LengthUnit::Percent(r) => r * size,
		}
	}
	let (width, height) = (layout.rect.right - layout.rect.left, layout.rect.bottom - layout.rect.top);
	let mut r = BorderRadiusPixel {
		x: [
			trans(border_radius.x[0], width),
			trans(border_radius.x[1], width),
			trans(border_radius.x[2], width),
			trans(border_radius.x[3], width),
		],
		y: [
			trans(border_radius.y[0], height),
			trans(border_radius.y[1], height),
			trans(border_radius.y[2], height),
			trans(border_radius.y[3], height),
		],
	};
	let (top, bottom, left, right) = (
		r.x[0] + r.x[1], 
		r.x[2] + r.x[3], 
		r.y[0] + r.y[3], 
		r.y[1] + r.y[2], 
	);
	// 修正圆角的，同一条边长的圆角半径之和不能大于边长
	if top > width {
		r.x[0] = width/top * r.x[0];
		r.x[1] = width/top * r.x[1];
	}
	if bottom > width {
		r.x[2] = width/bottom * r.x[2];
		r.x[3] = width/bottom * r.x[3];
	}
	if left > height {
		r.y[0] = height/left * r.y[0];
		r.y[3] = height/left * r.y[3];
	}
	if right > height {
		r.y[1] = height/right * r.y[1];
		r.y[2] = height/right * r.y[2];
	}
	r
	
}

/// 计算内容区域的圆角半径
pub fn cal_content_border_radius(border_radius: &BorderRadiusPixel, border: (f32, f32, f32, f32)/*上右下左*/) -> BorderRadiusPixel {
	BorderRadiusPixel {
		x: [
			border_radius.x[0] - border.3,
			border_radius.x[1] - border.1,
			border_radius.x[2] - border.1,
			border_radius.x[3] - border.3,
		],
		y: [
			border_radius.y[0] - border.0,
			border_radius.y[1] - border.0,
			border_radius.y[2] - border.2,
			border_radius.y[3] - border.2,
		],
	}
}

// pub fn get_radius(radius: &BorderRadius, layout: &LayoutResult) -> Rect<NotNan<f32>> {
//     let width = layout.rect.right - layout.rect.left;
//     let height = layout.rect.bottom - layout.rect.top;
//     let half_width = width / 2.0;
//     let half_height = height / 2.0;

//     Rect {
//         top: match radius.y {
//             LengthUnit::Pixel(v) => unsafe { NotNan::new_unchecked(half_height.min(v)) },
//             LengthUnit::Percent(v) => unsafe { NotNan::new_unchecked(half_height.min(v * height)) },
//         },
//         right: match radius.x {
//             LengthUnit::Pixel(v) => unsafe { NotNan::new_unchecked(half_width.min(v)) },
//             LengthUnit::Percent(v) => unsafe { NotNan::new_unchecked(half_width.min(v * width)) },
//         },
//         bottom: match radius.y {
//             LengthUnit::Pixel(v) => unsafe { NotNan::new_unchecked(half_height.min(v)) },
//             LengthUnit::Percent(v) => unsafe { NotNan::new_unchecked(half_height.min(v * height)) },
//         },
//         left: match radius.x {
//             LengthUnit::Pixel(v) => unsafe { NotNan::new_unchecked(half_width.min(v)) },
//             LengthUnit::Percent(v) => unsafe { NotNan::new_unchecked(half_width.min(v * width)) },
//         },
//     }
// }

pub fn get_content_size(layout: &LayoutResult) -> Size<NotNan<f32>> {
    Size {
        width: unsafe { NotNan::new_unchecked(layout.rect.right - layout.rect.left - layout.border.left - layout.border.right) },
        height: unsafe { NotNan::new_unchecked(layout.rect.bottom - layout.rect.top - layout.border.bottom - layout.border.top) },
    }
}

#[inline]
pub fn get_content_rect(layout: &LayoutResult) -> Rect<NotNan<f32>> {
    Rect {
        top: unsafe { NotNan::new_unchecked(layout.border.top + layout.border.top) },
        right: unsafe { NotNan::new_unchecked(layout.rect.right - layout.border.right) },
        bottom: unsafe { NotNan::new_unchecked(layout.rect.bottom - layout.border.bottom) },
        left: unsafe { NotNan::new_unchecked(layout.rect.left + layout.border.left) },
    }
}

#[inline]
pub fn get_box_rect(layout: &LayoutResult) -> Rect<NotNan<f32>> {
    Rect {
        top: unsafe { NotNan::new_unchecked(0.0) },
        right: unsafe { NotNan::new_unchecked(layout.rect.right - layout.rect.left) },
        bottom: unsafe { NotNan::new_unchecked(layout.rect.bottom - layout.rect.top) },
        left: unsafe { NotNan::new_unchecked(0.0) },
    }
}

// 计算两个aabb的交集
#[inline]
pub fn intersect(a: &Aabb2, b: &Aabb2) -> Option<Aabb2> {
    let r = Aabb2::new(
        Point2::new(a.mins.x.max(b.mins.x), a.mins.y.max(b.mins.y)),
        Point2::new(a.maxs.x.min(b.maxs.x), a.maxs.y.min(b.maxs.y)),
    );
    if r.maxs.x <= r.mins.x || r.maxs.y <= r.mins.y {
        return None;
    }
    Some(r)
}


// pub fn query_quad_by_aabb(tree: QuadTree<LocalVersion, f32, ()>, aabb: &Aabb2) {
// 	let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
// 	let mut args = AbQueryArgs::new(
// 			enables,
// 			by_overflows,
// 			z_depths,
// 			overflow_clip,
// 			idtree,
// 			aabb.clone(),
// 			0,
// 		);
// 	tree.query(&aabb, intersects, &mut (), ab_query_func);
// }

// async fn load<L: Load>(
// 	mgr: &Share<AssetMgr<R1, G>>,
// 	k: usize,
// 	p: MultiTaskRuntime<()>,
// ) -> io::Result<Handle<R1>> {
// 	match AssetMgr::load(mgr, &k) {
// 		LoadResult::Ok(r) => Ok(r),
// 		LoadResult::Wait(f) => f.await,
// 		LoadResult::Receiver(recv) => {
// 			p.wait_timeout(1).await;
// 			println!("---------------load:{:?}", k);
// 			recv.receive(k, Ok(R1(TrustCell::new((k, k, 0))))).await
// 		}
// 	}
// }
