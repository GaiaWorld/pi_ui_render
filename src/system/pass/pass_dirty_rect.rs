/// 1. 计算每Pass的脏区域
/// 2. 根据每个脏区域的脏，合并为全局脏区域

use pi_ecs::{monitor::Event, prelude::{Write, Join, Query, ResMut, Res, ParamSet}};
use pi_ecs_macros::{listen, setup};

use crate::{components::{pass_2d::{Pass2D, DirtyRect, DirtyRectState}, draw_obj::{DrawObject, DrawState}, calc::{NodeId, Quad, InPassId, TransformWillChangeMatrix, ContentBox}, user::{Node, Aabb2}}, utils::tools::{box_aabb, intersect, calc_aabb}, resource::{Viewport, draw_obj::LayerPass2D}};

pub struct CalcDirtyRect;

#[setup]
impl CalcDirtyRect {
	/// 本函数执行先决条件： Quad、ContentBox已经准备好
	/// 当DrawObject的DrawState组件修改时，需要更新脏区域
	/// 当DrawObject删除时，需要更新脏区域
	/// 当DrawObject的NodeId创建时，需要更新脏区域
	#[listen(component=(DrawObject, DrawState, Modify), entity=(DrawObject, Delete), component=(DrawObject, NodeId, Create))]
	pub fn cal_dirty_rect_by(
		e: Event,
		query: Query<DrawObject, Join<NodeId, Node, (&InPassId, &Quad)>>,
		query_pass: Query<Pass2D, Write<DirtyRect>>,
	) {
		// log::info!("cal_dirty_rect_by=================");
		let (pass_id, quad) = match query.get_by_entity(e.id) {
			Some(r) => r,
			None => return
		};

		let mut dirty_rect_item = query_pass.get_unchecked(**pass_id);
		let dirty_rect = dirty_rect_item.get_mut_or_default();
		let new_dirty_rect = match dirty_rect.state {
			// 脏区域处于未初始化状态，则设置脏区域为当前DrawObject对应节点的包围盒
			DirtyRectState::UnInit | DirtyRectState::Active => DirtyRect {
				value: *quad.clone(),
				state: DirtyRectState::Inited,
			},
			// 如果脏区域已经初始化，这设置脏区域为当前DrawObject对应节点的包围盒与当前脏区域的合并包围盒
			DirtyRectState::Inited => {
				box_aabb(&mut dirty_rect.value, &quad);
				DirtyRect {
					value: dirty_rect.value.clone(),
					state: DirtyRectState::Inited,
				}
			},
		};
		// println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
		dirty_rect_item.write(new_dirty_rect);
	}

	/// 根据每个Pass的脏区域，计算全局脏区域
	#[system]
	pub fn calc_global_dirty_rect(
		// 
		mut query_pass: ParamSet<(
			Query<Pass2D, (&'static mut DirtyRect, &'static NodeId)>,
			Query<Pass2D, (&'static mut DirtyRect, Join<NodeId, Node, (&'static ContentBox, Option<&'static TransformWillChangeMatrix>)>)>)>,
		query_node: Query<Node, &TransformWillChangeMatrix>,
		mut global_dirty_rect: ResMut<DirtyRect>,
		viewport: Res<Viewport>, // 视口
	) {
		
		let mut dirty_rect = &mut *global_dirty_rect;
		// 先恢复到初始状态
		dirty_rect.state = DirtyRectState::UnInit;
		dirty_rect.value = viewport.0.clone();

		// 遍历所有pass的脏区域，求并，得全局脏区域
		for (mut pass_dirty_rect, node_id) in query_pass.p0_mut().iter_mut() {
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

		// 视口改变，全局脏区域就为视口
		if viewport.is_changed() {
			dirty_rect.state = DirtyRectState::Inited;
			dirty_rect.value = viewport.0.clone();
		}
	}
}

