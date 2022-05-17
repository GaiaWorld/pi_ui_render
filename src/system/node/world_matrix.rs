//! 根据LayoutR, Tranform组件， 计算节点的世界矩阵
//! 
//! ## 计算过程
//! 节点必须存在如下组件：
//! * NodeState
//! * LayoutResult
//! 节点可能存在如下组件：
//! * Transform
//! 
//! Transform组件描述节点的变化，通常可能是以自身中心点（或其它点）为原点进行变换
//! 而LayoutR描述的节点矩形位置，是以父节点布局矩形的左上角为原点的坐标描述
//! 因此需要同一它们描述的原点坐标
//! 
//! 本模块将Transform描述的变换转换到以父节点布局的矩形区域左上角为原点
//! 
//! 设节点布局后的矩形区域：left_top_x, left_top_y, right_bottom_x, right_bottom_y
//! Transform的转化过程为：M(移动left_top_x、left_top_y) * M(Transform) * M(-left_top_x、-left_top_y)
//! 这样Transform的变幻矩阵就变为了以父节点布局的矩形左上角为原点的变化。
//! 
//! 尽管Transform的转换过程稍显复杂，但大部分情况下，是不存在Transform组件的，因此不会计算这种转化，只需要计算自身布局属性包含的变化信息，世界矩阵计算的常数时间不会太长。
//! 
//! 世界矩阵计算公式：
//! 	  ParentMatrix * TransformMatrix
//! 	或ParentMatrix
//! 
//! ## 优化
//! ### 层次脏优化
//! 
//! WorldMatrix的计算存在以下特性：
//! * Transform的变化和Layout的变化，会影响自身世界矩阵，但不会影响父节点、兄弟节点的世界矩阵（除非它们的对应属性发生改变）
//! * WorldMatrix除了受到自身Transform和Layout的影响，也收到父矩阵的影响
//! 
//! 因此，在计算时，需要先计算父节点的世界矩阵（如果先计算子节点，在计算父节点，父节点的世界矩阵会再次作用到子节点，子节点不要重新算）
//! 
//! 层次脏根据节点树，记录了修改节点的层，在遍历时优先遍历层较低的节点（父），同时递归遍历该节点下的所有子节点。
//! 
//! ## 并行
//! 暂时无并行。
//! 
//! 可以考虑： 当父矩阵计算完成后，父节点所有子节点所形成的子树，可以并行计算（他们依赖的父矩阵已经计算完毕）

use pi_ecs::prelude::{Or, Query, Write};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::{LayerDirty, EntityTree};
use pi_ecs::prelude::{Changed, Id};
use pi_null::Null;
use pi_slotmap_tree::Storage;

use crate::components::user::{ Node, Transform, Point2 };
use crate::components::calc:: {
	LayoutResult,
	WorldMatrix
};

pub struct CalcMatrix;

#[setup]
impl CalcMatrix {
	/// 计算世界矩阵
	/// 世界矩阵以自身左上角为原点
	#[system]
	pub fn cal_matrix(
		query: Query<Node, (Option<&Transform>, &LayoutResult)>,
		idtree: EntityTree<Node>,
		dirtys: LayerDirty<Node, Or<(Changed<Transform>, Changed<LayoutResult>)>>,
		mut matrixs: Query<Node, Write<WorldMatrix>>,
	) {
		for id in dirtys.iter() {
			let (transform, layout) =  query.get_unchecked(id);
			let parent_id = idtree.get_up(id).map_or(Id::<Node>::null(), |up|{up.parent()});

			let width = layout.rect.right - layout.rect.left;
			let height = layout.rect.bottom - layout.rect.top;

			let matrix = if parent_id.is_null() {
				// 父为空，则其为根节点，其世界矩阵为单位阵
				WorldMatrix::default()
			} else {
				// 否则
				let p_m= matrixs.get_mut(parent_id).unwrap();
				let parent_world_matrix = match p_m.get() {
					Some(r) => r,
					None => {
						log::error!("calc matrix fail, parent matrix is not exist!, id:{:?}, parent_id: {:?}", id, parent_id);
						return;
					}
				};
				
				match transform {
					// transform存在时，根据transform和布局计算得到变换矩阵，再乘以父矩阵
					Some(transform) => parent_world_matrix*WorldMatrix::form_transform(
						transform,
						width,
						height,
						&Point2::new(layout.rect.left, layout.rect.top),
					),
					// transform不存在时，节点的变换矩阵可以直接由布局结果得出，世界矩阵计算更快，大部分情况也是走这条快速路径
					None => {
						let mut w = parent_world_matrix.clone();
						w.translate(layout.rect.left, layout.rect.top, 0.0);
						w
					}
				}
			};
			
			// 将计算结果写入组件
			matrixs.get_mut(id).unwrap().write(matrix);
		}
	}
}



#[cfg(test)] 
pub mod test {
	use std::sync::Arc;

	use pi_async::rt::{multi_thread::{MultiTaskRuntimeBuilder, StealableTaskPool}, AsyncRuntime};
	use pi_ecs::prelude::{System, World, SingleDispatcher, IntoSystem, StageBuilder, Dispatcher, Write, QueryState, Query, In, Setup, Id};
	use pi_ecs_utils::prelude::EntityTreeMut;
	use pi_flex_layout::prelude::Rect;
	use pi_null::Null;

	use crate::components::user::{Node, Transform, TransformFunc, Vector4};
	use crate::components::calc::LayoutResult;
	use crate::components::calc::WorldMatrix;

	use super::{CalcMatrix};

	#[test]
	fn test() {
		// 创建world
		let mut world = World::new();

		// 创建派发器
		let mut dispatcher = get_dispatcher(&mut world);

		modfiy_world_matrix(&mut world, &mut dispatcher);
	}

	// 绝对位置,节点以左上为原点，经过布局、变化，得到的最终位置
	#[derive(Deref, DerefMut, Debug)]
	pub struct AbsolutePosition(Rect<f32>);

	// 初始化，将所有节点以根节点作为父节点组织为树
	fn init_tree(
		root: In<Id<Node>>,
		mut tree: EntityTreeMut<Node>,
		entitys: Query<Node, Id<Node>>,
	){
		let r = root.0;
		for e in entitys.iter() {
			if e != r {
				tree.insert_child(e, r, std::usize::MAX);
			} else {
				tree.insert_child(e, Id::null(), std::usize::MAX);
			}
		}
	}

	pub fn modfiy_world_matrix(world: &mut World, dispatcher: &mut impl Dispatcher) {
		// 创建原型
		world.new_archetype::<Node>()
			.register::<AbsolutePosition>()
			.create();
		
		let mut entitys = Vec::new();
		let root = world.spawn::<Node>()
		.insert(LayoutResult {
			rect: Rect{left:0.0, right:1000.0, top:0.0, bottom:1000.0},
			border: Rect{left:0.0, right:0.0, top:0.0, bottom:0.0},
			padding: Rect{left:0.0, right:0.0, top:0.0, bottom:0.0},
		})
		.insert(AbsolutePosition(Rect{left:0.0, right:1000.0, top:0.0, bottom:1000.0}))
		.id();
		
		//插入根节点
		entitys.push(root);

		let size = 50.0;
		let mut left_top = 0.0;
		let mut right_bottom;
		// 插入三个节点作为子节点
		for _i in 0..3 {
			right_bottom = left_top + size;

			let entity = world.spawn::<Node>()
				.insert(LayoutResult {
					rect: Rect{left:left_top, right: right_bottom, top: left_top, bottom: right_bottom},
					border: Rect{left:0.0, right:0.0, top:0.0, bottom:0.0},
					padding: Rect{left:0.0, right:0.0, top:0.0, bottom:0.0},
				})
				.insert(AbsolutePosition(Rect{left:left_top, right: right_bottom, top: left_top, bottom: right_bottom})).id();
			// 插入实体，以根节点作为父节点
			entitys.push(entity);

			left_top += size;
		}

		// 组织为树结构
		let mut init_tree_sys = init_tree.system(world);
		init_tree_sys.run(In(root));

		let mut query = world.query::<Node, (Id<Node>, &WorldMatrix, &LayoutResult, &mut AbsolutePosition)>();
	
		// 测试矩阵计算
		dispatcher.run();
		asset_matrix(world, &mut query);

		// 最后一个实体，添加一个缩放为0.5的Transform
		let mut transform_mut = world.query::<Node, (Id<Node>, Write<Transform>)>();
		let last_entity = entitys[entitys.len() - 1];
		let mut t = Transform::default();
		t.funcs.push(TransformFunc::Scale(0.5, 0.5));
		transform_mut.get_mut(world, last_entity).unwrap().1.write(t);
		
		// 测试矩阵计算, 最后一个实体组件缩放为原来的0.5
		dispatcher.run();
		*query.get_mut(world, last_entity).unwrap().3 = AbsolutePosition(Rect{left:112.5, right: 137.5, top: 112.5, bottom: 137.5});
		asset_matrix(world, &mut query);
	}

	pub fn get_dispatcher(world: &mut World) -> SingleDispatcher<StealableTaskPool<()>> {
		let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
	
		let mut stage = StageBuilder::new();
		CalcMatrix::setup(world, &mut stage);
		
		let mut stages = Vec::new();
		stages.push(Arc::new(stage.build(world)));
		let mut dispatcher = SingleDispatcher::new(rt);
		dispatcher.init(stages, world);
	
		dispatcher
	}

	fn asset_matrix(world: &mut World, query: &mut QueryState<Node, (Id<Node>, &WorldMatrix, &LayoutResult, &mut AbsolutePosition)>) {
		for (_e, w, l, a_p) in query.iter_mut(world) {
			let left_top = w * Vector4::new(0.0, 0.0, 1.0, 1.0);
			let right_bottom = w * Vector4::new(l.rect.right - l.rect.left, l.rect.bottom - l.rect.top, 1.0, 1.0);
			// println!("e: {:?}, a_p: {:?}, left_top: {:?}, right_bottom: {:?}", e, a_p, left_top, right_bottom);
			// println!("matrix: {:?}, layout:{:?}", w, l);
			assert_eq!(left_top.x, a_p.left);
			assert_eq!(left_top.y, a_p.top);
			assert_eq!(right_bottom.x, a_p.right);
			assert_eq!(right_bottom.y, a_p.bottom);
		}
	}
}








