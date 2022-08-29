//! 根据世界矩阵，计算包围盒
use pi_ecs::monitor::Event;
use pi_ecs::prelude::{Query, Write};
use pi_ecs_macros::{listen, setup};

use crate::components::user::{Node, Aabb2, Point2};
use crate::components::calc::{WorldMatrix, Quad, LayoutResult};
use crate::utils::tools::calc_bound_box;

pub struct CalcQuad;

#[setup]
impl CalcQuad {
	// pub fn collect() {
	// 	// oct.collect();
	// }
	
	/// 监听世界矩阵变化，修改包围盒
	#[listen(component = (Node, WorldMatrix, (Create, Modify)))]
	pub fn calc_quad(e: Event, query: Query<Node, (&WorldMatrix, &LayoutResult)>, mut oct: Query<Node, Write<Quad>>) {
		let id = e.id;
		let (world_matrix, layout ) = query.get_unchecked_by_entity(id);
		let width = layout.rect.right - layout.rect.left;
		let height = layout.rect.bottom - layout.rect.top;
		let aabb = calc_bound_box(&Aabb2::new(Point2::new(0.0, 0.0), Point2::new(width, height)), world_matrix);
	
		oct.get_unchecked_mut_by_entity(id).write(Quad::new(aabb));
	}
}

// fn cal_bound_box(size: (f32, f32), matrix: &WorldMatrix) -> Aabb2 {
// 	let left_top = matrix * Vector4::new(0.0, 0.0, 0.0, 1.0);
// 	let right_bottom = matrix * Vector4::new(size.0,  size.1, 0.0, 1.0);

// 	let min = Point2::new(left_top.x, left_top.y);
// 	let max = Point2::new(right_bottom.x, right_bottom.y);

// 	Aabb2::new(min, max)
// }



#[cfg(test)]
mod test {

    use pi_ecs::prelude::{QueryState,World, StageBuilder, Setup, Id};

    use crate::system::node::world_matrix::{test::{get_dispatcher, modfiy_world_matrix, AbsolutePosition}};
	use crate::components::{calc::Quad, user::Node};

    use super::CalcQuad;

	#[test]
	fn test() {
		// 创建world
		let mut world = World::new();

		// 穿件派发器
		let mut dispatcher = get_dispatcher(&mut world);

		let mut stage = StageBuilder::new();
		CalcQuad::setup(&mut world, &mut stage);

		// 修改世界矩阵
		modfiy_world_matrix(&mut world, &mut dispatcher);

		// 检查quad计算结果
		let mut query = world.query::<Node, (Id<Node>, &Quad, &AbsolutePosition)>();
		asset_quad(&mut world, &mut query);
	}

	fn asset_quad(world: &mut World, query: &mut QueryState<Node, (Id<Node>, &Quad, &AbsolutePosition)>) {
		for (_e, quad, a_p) in query.iter(world) {
			// println!("e: {}, quad: {:?}, a_p:{:?}", e.local().offset(), quad, a_p);
			assert_eq!(quad.mins.x, a_p.left);
			assert_eq!(quad.mins.y, a_p.top);
			assert_eq!(quad.maxs.x, a_p.right);
			assert_eq!(quad.maxs.y, a_p.bottom);
		}
	}
}

