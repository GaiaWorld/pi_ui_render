/// 1. 计算每Pass的脏区域
/// 2. 根据每个脏区域的脏，合并为全局脏区域

use pi_ecs::{monitor::Event, prelude::{Write, Join, Query, ResMut, Res}};
use pi_ecs_macros::{listen, setup};

use crate::{components::{pass_2d::{Pass2D, DirtyRect, DirtyRectState}, draw_obj::{DrawObject, DrawState}, calc::{NodeId, Quad, InPassId}, user::Node}, utils::tools::{box_aabb, intersect}, resource::Viewport};

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
			DirtyRectState::UnInit => DirtyRect {
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
			}
		};
		// println!("quad======{:?}, id:{:?}, new_dirty_rect:{:?}", quad, e.id, new_dirty_rect);
		dirty_rect_item.write(new_dirty_rect);
	}

	/// 根据每个Pass的脏区域，计算全局脏区域
	#[system]
	pub fn calc_global_dirty_rect(
		mut query_pass: Query<Pass2D, &mut DirtyRect>,
		mut global_dirty_rect: ResMut<DirtyRect>,
		viewport: Res<Viewport>, // 视口
	) {
		let mut dirty_rect = &mut *global_dirty_rect;
		// 先恢复到初始状态
		dirty_rect.state = DirtyRectState::UnInit;
		// 先用第一个pass的脏区域，初始化全局脏区域
		for mut pass_dirty_rect in query_pass.iter_mut() {
			*dirty_rect = pass_dirty_rect.clone();
			pass_dirty_rect.state = DirtyRectState::UnInit;
			break;
		}

		// 遍历所有pass的脏区域，求并，得全局脏区域
		for mut pass_dirty_rect in query_pass.iter_mut() {
			if pass_dirty_rect.state == DirtyRectState::Inited {
				box_aabb(&mut dirty_rect.value, &pass_dirty_rect.value);
				dirty_rect.state = DirtyRectState::Inited;
				pass_dirty_rect.state = DirtyRectState::UnInit;
			}
		}

		// 最后要与视口求交
		match intersect(&dirty_rect.value, &viewport) {
			Some(r) => dirty_rect.value = r,
			None => dirty_rect.state = DirtyRectState::UnInit
		}
	}
}

