//! 根据世界矩阵，计算包围盒
use pi_ecs::monitor::Event;
use pi_ecs::prelude::{Query, Write};
use pi_ecs_macros::listen;

use crate::components::user::{Node, Vector4, Aabb2, Point2};
use crate::components::calc::{WorldMatrix, Oct, LayoutResult};


pub fn collect() {
	// oct.collect();
}

/// 监听世界矩阵变化，修改包围盒
#[listen(component = (Node, WorldMatrix, Modify))]
pub fn calc_oct(e: Event, query: Query<Node, (&WorldMatrix, &LayoutResult)>, mut oct: Query<Node, Write<Oct>>) {
	let id = e.id;
	let (world_matrix, layout) = query.get_unchecked(id);
	let width = layout.rect.right - layout.rect.left;
	let height = layout.rect.bottom - layout.rect.top;
	let aabb = cal_bound_box((width, height), world_matrix);

	oct.get_unchecked_mut(id).write(Oct::new(aabb));
}

fn cal_bound_box(size: (f32, f32), matrix: &WorldMatrix) -> Aabb2 {
    let left_top = matrix * Vector4::new(0.0, 0.0, 0.0, 1.0);
    let right_top = matrix * Vector4::new(size.0, 0.01, 0.0, 1.0);
    let left_bottom = matrix * Vector4::new(0.0, size.1, 0.0, 1.0);
    let right_bottom = matrix * Vector4::new(size.0,  size.1, 0.0, 1.0);

    let min = Point2::new(
        left_top
            .x
            .min(right_top.x)
            .min(left_bottom.x)
            .min(right_bottom.x),
        left_top
            .y
            .min(right_top.y)
            .min(left_bottom.y)
            .min(right_bottom.y),
    );

    let max = Point2::new(
        left_top
            .x
            .max(right_top.x)
            .max(left_bottom.x)
            .max(right_bottom.x),
        left_top
            .y
            .max(right_top.y)
            .max(left_bottom.y)
            .max(right_bottom.y),
    );

    Aabb2::new(min, max)
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use pi_async::rt::{multi_thread::{StealableTaskPool, MultiTaskRuntimeBuilder}, AsyncRuntime};
    use pi_ecs::{prelude::{World, SingleDispatcher, StageBuilder, IntoSystem}, monitor::{Listeners, ListenSetup}};

    use crate::system::handle_node::world_matrix::{cal_matrix, test::{get_dispatcher, modfiy_world_matrix}};

    use super::calc_oct;

	#[test]
	fn test() {
		// 创建world
		let mut world = World::new();

		// 穿件派发器
		let mut dispatcher = get_dispatcher(&mut world);

		let oct_listener = calc_oct.listeners();
		oct_listener.setup(&mut world);

		// 修改世界矩阵
		modfiy_world_matrix(&mut world, &mut dispatcher);


	}
}

