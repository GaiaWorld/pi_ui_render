/// 1. 计算每Pass的脏区域
/// 2. 根据每个脏区域的脏，合并为全局脏区域

use pi_ecs::{monitor::Event, prelude::{Write, Join, Query, ParamSet, Id}, query::{With, ChangeTrackers}};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::Layer;
use pi_null::Null;
use pi_style::style::Aabb2;

use crate::{components::{user::{Viewport, ShowChange}, pass_2d::{Pass2D, DirtyRect, DirtyRectState, ParentPassId}, draw_obj::{DrawObject, DrawState}, calc::{NodeId, Quad, InPassId, TransformWillChangeMatrix, ContentBox}, user::Node}, utils::tools::{box_aabb, calc_aabb}};

pub struct CalcDirtyRect;

#[setup]
impl CalcDirtyRect {
	/// 根据每个Pass的脏区域，计算全局脏区域
	#[system]
	pub fn calc_global_dirty_rect(
		query_show_change: Query<Node, (ChangeTrackers<ShowChange>, &Quad, &InPassId), With<ShowChange>>,
		// 
		mut query_pass: ParamSet<(
			Query<Pass2D, (&'static mut DirtyRect, &'static NodeId, Join<NodeId, Node, &'static Layer<Node>>)>,
			Query<Pass2D, (&'static mut DirtyRect, Join<NodeId, Node, (&'static ContentBox, Option<&'static TransformWillChangeMatrix>)>)>,
			Query<Pass2D, Write<DirtyRect>>,)>,
		query_node: Query<Node, &TransformWillChangeMatrix>,
		mut query_root: Query<Node, (Write<DirtyRect>, &'static Viewport, ChangeTrackers<Viewport>), With<Viewport>>,
		// query_root: Query<Node, &mut DirtyRect>,

		// mut global_dirty_rect: ResMut<DirtyRect>,
	) {
		
		// let mut dirty_rect = &mut *global_dirty_rect;
		// // 先恢复到初始状态
		// dirty_rect.state = DirtyRectState::UnInit;
		// dirty_rect.value = viewport.0.clone();

		// 如果有节点修改了ShowChange，需要设置脏区域
		let p2 = query_pass.p2_mut();
		for (change_ticker, quad, in_pass_id ) in query_show_change.iter() {
			if change_ticker.is_changed() {
				mark_pass_dirty_rect(in_pass_id.0, &*quad, p2);
			}
		}

		for (mut dirty_rect, viewport, _) in query_root.iter_mut() {
			let dirty_rect = dirty_rect.get_mut_or_default();
			// 先恢复到初始状态
			dirty_rect.state = DirtyRectState::UnInit;
			dirty_rect.value = viewport.0.clone();
		}

		// 遍历所有pass的脏区域，求并，得全局脏区域
		for (mut pass_dirty_rect, node_id, layer) in query_pass.p0_mut().iter_mut() {
			let (mut dirty_rect, viewport, viewport_tracker) = match query_root.get_mut(layer.root()) {
				Some(r) => r,
				None => continue
			};
			let dirty_rect = dirty_rect.get_mut_or_default();

			// 视口改变，全局脏区域就为视口
			if viewport_tracker.is_changed() {
				dirty_rect.state = DirtyRectState::Inited;
				dirty_rect.value = viewport.0.clone();
				pass_dirty_rect.state = DirtyRectState::UnInit;
				continue;
			}

			if pass_dirty_rect.state == DirtyRectState::Inited {
				let aabb = match query_node.get(**node_id) {
					Some(matrix) => calc_aabb(&pass_dirty_rect.value, &matrix.will_change),
					None => pass_dirty_rect.value.clone()
				};

				box_aabb(&mut dirty_rect.value, &aabb);
				dirty_rect.state = DirtyRectState::Inited;
				pass_dirty_rect.state = DirtyRectState::UnInit;
			}
		}

	}

	/// 本函数执行先决条件： Quad、ContentBox已经准备好
	/// 当DrawObject的DrawState组件修改时，需要更新脏区域
	/// 当DrawObject删除时，需要更新脏区域
	/// 当DrawObject的NodeId创建时，需要更新脏区域
	#[listen(component=(DrawObject, DrawState, Modify), entity=(DrawObject, Delete), component=(DrawObject, NodeId, Create))]
	pub fn cal_dirty_rect_by(
		e: Event,
		query: Query<DrawObject, Join<NodeId, Node, (&InPassId, &Quad)>>,
		mut query_pass: Query<Pass2D, Write<DirtyRect>>,
	) {
		let (pass_id, quad) = match query.get_by_entity(e.id) {
			Some(r) => r,
			None => return
		};

		mark_pass_dirty_rect(pass_id.0, &*quad, &mut query_pass);
	}

	/// 当pass2d删除时，更新父的Pass2d的脏区域
	#[listen(entity=(Pass2D, Delete))]
	pub fn cal_dirty_rect_by_pass_2d(
		e: Event,
		query: Query<Pass2D,(&ParentPassId, Join<NodeId, Node, &ContentBox>)>,
		mut query_pass: Query<Pass2D, Write<DirtyRect>>,
	) {
		let (pass_id, content_box) = match query.get_by_entity(e.id) {
			Some(r) => r,
			None => return
		};

		// 如果存在父的pass2d， 则将子pass2d的内容包围盒更新到父的pass2d的脏区域中
		if !pass_id.0.is_null() {
			mark_pass_dirty_rect(pass_id.0, &content_box.0, &mut query_pass);
		}
	}

	/// 监听quad的删除事件，更新脏区域（Quad的创建、修改，最终表现为DrawState的变化，已被其他监听器监听）
	/// 注意，Quad实际上没有真正意义的删除事件，只有当节点销毁时才会删除，但节点销毁不会额外发出组件删除的事件
	/// 这里的Quad删除事件，实际上是Quad在修改前，额外发出的事件，一遍某些system能了解Quad修改前的数据（此事件的处，见Quad System）
	#[listen(component=(Node, Quad, Delete))]
	pub fn cal_dirty_rect_by_quad(
		e: Event,
		// query: Query<Node, (&DrawList, &Quad, &InPassId)>,
		query: Query<Node, (&Quad, &InPassId)>,
		mut query_pass: Query<Pass2D, Write<DirtyRect>>,
	) {
		let (quad, pass_id) = match query.get_by_entity(e.id) {
			Some(r) => r,
			None => return
		};

		// if draw_list.len() > 0 { // 本应该判断DrawList是否存在，但如果对DrawList进行查询，会脏成某些system依赖循环，因此暂时不判断，不影响渲染结果
			mark_pass_dirty_rect(pass_id.0, &*quad, &mut query_pass);
		// }
	}
}

fn mark_pass_dirty_rect(
	pass_id: Id<Pass2D>,
	rect: &Aabb2,
	query_pass: &mut Query<Pass2D, Write<DirtyRect>>,
) {
	let mut dirty_rect_item = query_pass.get_unchecked(pass_id);
	let dirty_rect = dirty_rect_item.get_mut_or_default();
	let new_dirty_rect = match dirty_rect.state {
		// 脏区域处于未初始化状态，则设置脏区域为当前DrawObject对应节点的包围盒
		DirtyRectState::UnInit => DirtyRect {
			value: rect.clone(),
			state: DirtyRectState::Inited,
		},
		// 如果脏区域已经初始化，这设置脏区域为当前DrawObject对应节点的包围盒与当前脏区域的合并包围盒
		DirtyRectState::Inited => {
			box_aabb(&mut dirty_rect.value, &rect);
			DirtyRect {
				value: dirty_rect.value.clone(),
				state: DirtyRectState::Inited,
			}
		},
	};
	// println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
	dirty_rect_item.write(new_dirty_rect);
}

