/// 标记脏

use pi_ecs::{prelude::{Join, Write, Query}, monitor::Event};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Up;

use crate::components::{calc::{InPassId, NodeId}, draw_obj::{DrawObject}, pass_2d::{DirtyMark, DirtyType, Pass2D}, user::Node};

pub struct CalcMark;

#[setup]
impl CalcMark {
	/// 监听DrawObject的创建，设置上下文的脏类型
	/// Pass2D必须准备就绪
	#[listen(entity=(DrawObject, (Create, Delete)))]
	pub fn drawobj_create(
		e: Event,
		query: Query<DrawObject, Join<NodeId, Node, &InPassId>>,
		pass: Query<Pass2D, &mut DirtyMark>,
	) {
		let pass_id = query.get_unchecked_by_entity(e.id);
		pass.get_unchecked(**pass_id).set(DirtyType::List as usize, true);
	}

	/// Pass2D创建时，插入DirtyMark组件
	#[listen(entity=(Pass2D, Create))]
	pub fn pass2d_create(
		e: Event,
		query: Query<Pass2D, Write<DirtyMark>>,
	) {
		if let Some(mut r) = query.get_mut_by_entity(e.id) {
			r.write(DirtyMark::default());
		}
	}

	/// Pass2D创建和删除时，设置父的Pass2D脏
	#[listen(component=(Pass2D, NodeId, Create), entity=(Pass2D, Delete))]
	pub fn pass2d_create_or_delete(
		e: Event,
		query_pass2d: Query<Pass2D, &NodeId>,
		up: Query<Node, &Up<Node>>,
		in_pass_id: Query<Node, &InPassId>,
		mut dirty_mark: Query<Pass2D, &mut DirtyMark>,
	) {
		if let Some(node_id) = query_pass2d.get_by_entity(e.id) {
			if let Some(up) = up.get(**node_id) {
				if let Some(in_pass_id) = in_pass_id.get(up.parent()) {
					if let Some(mut dirty_mark) = dirty_mark.get_mut(**in_pass_id) {
						dirty_mark.set(DirtyType::List as usize, true);
					}
				}
			}
		}
	}

}

