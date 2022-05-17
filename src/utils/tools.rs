use std::hash::{Hash, Hasher};

use pi_hash::DefaultHasher;

use crate::components::user::{Aabb2, Point2};


pub fn calc_hash<T: Hash>(v: &T)-> u64 {
	let mut hasher = DefaultHasher::default();
	v.hash(&mut hasher);
	hasher.finish()
}

pub fn box_aabb(aabb1: &mut Aabb2, aabb2: &Aabb2) {
	aabb1.mins.x = aabb1.mins.x.min(aabb2.mins.x);
	aabb1.mins.y = aabb1.mins.y.min(aabb2.mins.y);
	aabb1.maxs.x = aabb1.maxs.x.max(aabb2.maxs.x);
	aabb1.maxs.y = aabb1.maxs.y.max(aabb2.maxs.y);
}

// 计算两个aabb的交集
#[inline]
pub fn intersect(a: &Aabb2, b: &Aabb2) -> Option<Aabb2> {
	let r = Aabb2::new(
		Point2::new(a.mins.x.max(b.mins.x), a.mins.y.max(b.mins.y)),
		Point2::new(a.maxs.x.min(b.maxs.x), a.maxs.y.min(b.maxs.y))
	);
	if r.maxs.x <= r.mins.x || r.maxs.y <= r.mins.y {
		return None

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

