use std::hash::{Hash, Hasher};

use num_traits::float::FloatCore;
use ordered_float::NotNan;
use pi_hash::DefaultHasher;

use crate::components::{user::{Aabb2, Point2, Vector4}, calc::WorldMatrix};


pub fn calc_hash<T: Hash>(v: &T)-> u64 {
	let mut hasher = DefaultHasher::default();
	v.hash(&mut hasher);
	hasher.finish()
}

pub fn calc_float_hash<T: FloatCore>(v: &[T])-> u64 {
	let mut hasher = DefaultHasher::default();
	for i in v.iter() {
		unsafe{NotNan::unchecked_new(*i)}.hash(&mut hasher);
	}
	hasher.finish()
}

pub fn calc_aabb(aabb: &Aabb2, matrix: &WorldMatrix) -> Aabb2 {
	let min = matrix * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
	let max = matrix * Vector4::new(aabb.maxs.x, aabb.maxs.y, 0.0, 1.0);

	let min = Point2::new(min.x, min.y);
	let max = Point2::new(max.x, max.y);

	Aabb2::new(min, max)
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

