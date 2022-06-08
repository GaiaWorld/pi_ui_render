//! 处理transform_will_change属性，计算出TransformWillChangeMatrix
//! TransformWillChange属性常用与，子节点数量较多，又频繁改变Transform的节点
//! 将变化设置到Transform设置到TransformWillChange上，所有的子节点不需要重新计算WorldMatrix
//! 假定某个节点A上设置的TransformWillChange为T1， A的世界矩阵为Wa， 
//! A存在一个子节点B，由B的Transform变换所得的局部矩阵为Tb，因此B的世界矩阵为Wa * Tb, 记作Wb 
//! 又由于A上存在TransformWillChange T1，其也能影响B， 
//! B的最终变换应该为Wa * T1 * Tb = Wa * T1 * Wa逆 * Wa * Tb = Wa * T1 * Wa逆 * Wb;
//! 将Wa * T1 * Wa称为TransformWillChangeMatrix TW。
//! 渲染A下所有子节点时，将TW作为视图矩阵。
//! 
//! 

use pi_dirty::LayerDirty;
use pi_ecs::{monitor::Event, prelude::{Query, Write, FromWorld, Res, Local, With, ChangeTrackers, ParamSet}, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::{Layer, NodeUp};
use pi_null::Null;


use crate::{components::{user::{Node, TransformWillChange}, calc::{RenderContextMark, TransformWillChangeMatrix, LayoutResult, WorldMatrix, Pass2DId, NodeId}, pass_2d::{ParentPassId, Pass2D}}, resource::RenderContextMarkType};

pub struct CalcTransformWillChange;

#[derive(Deref)]
pub struct TransformWillChangeRenderContextMarkType(RenderContextMarkType);

impl FromWorld for TransformWillChangeRenderContextMarkType{
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
        Self(RenderContextMarkType::from_world(world))
    }
}

#[setup]
impl CalcTransformWillChange {
	#[system]
	pub fn calc_transform_willchange(
		query: Query<Node, (
			Id<Node>,
			&NodeUp<Node>,
			&Pass2DId,
			&Layer,
			ChangeTrackers<WorldMatrix>,
			ChangeTrackers<TransformWillChange>, 
			ChangeTrackers<Layer>), With<TransformWillChange>>,
		query_node_trans: Query<Node, (
			&TransformWillChange,
			&LayoutResult)>,
		query_node_matrix: Query<Node, &WorldMatrix>,
		query_pass2d_parent: Query<Pass2D, &ParentPassId>,
		query_pass2d_nodeid: Query<Pass2D, &NodeId>,
		mut write: ParamSet< (
			Query<Node, Write<TransformWillChangeMatrix>>, 
			Query<Node, (ChangeTrackers<TransformWillChangeMatrix>, &'static TransformWillChangeMatrix)>)>,
		mut local: Local<LayerDirty<(Id<Node>, Id<Node>, Id<Pass2D>, bool)>>,
	) {
		for (id, up, pass_id, layer, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
			local.mark( 
				(id, 
					up.parent(),
					**pass_id, 
					tracker_willchange.is_changed() || 
						tracker_layer.is_changed() || 
						tracker_matrix.is_changed()
				), **layer);
		}

		for ((id, node_p_id, pass_id, is_changed), _layer) in local.iter() {
			let mut changed = false;
			let mut parent_will_change_matrix = None;
			let mut parent_pass_id = query_pass2d_parent.get(pass_id.clone());
			let p1 = write.p1();
			let r1;
			loop {
				let pass_id = match parent_pass_id {
					Some(r) if !r.is_null() => r,
					_ => break,
				};
				let parent_id = query_pass2d_nodeid.get_unchecked(**pass_id);

				if let Some(r) = p1.get(**parent_id) {
					if r.0.is_changed() {
						changed = true;
						r1 = r;
						parent_will_change_matrix = Some(&r1.1);
						break;
					}
				}

				parent_pass_id = query_pass2d_parent.get(**pass_id);
			}
			
			if changed || *is_changed {
				
				let (will_change, layout) = query_node_trans.get_unchecked(*id);
				let width = layout.rect.right - layout.rect.left;
				let height = layout.rect.bottom - layout.rect.top;
				let mut matrix = WorldMatrix::form_transform(&will_change.0, width, height);
				let p_matrix = query_node_matrix.get_unchecked(*node_p_id).clone();
				let invert = p_matrix.invert().unwrap();
				
				let mut m = p_matrix * &matrix * invert;

				if let Some(parent_will_change_matrix) = parent_will_change_matrix {
					m = &parent_will_change_matrix.0 * &m;
					matrix = &parent_will_change_matrix.1 * &matrix;
				}

				write.p0_mut().get_unchecked_mut(*id).write(
					TransformWillChangeMatrix(m, matrix));
			}
		}

		local.clear();
	}

	#[listen(component=(Node, TransformWillChange, (Create, Modify, Delete)))]
	pub fn transform_willchange_change(
		e: Event,
		query: Query<Node, &TransformWillChange>,
		mut write: Query<Node, (Write<RenderContextMark>, Write<TransformWillChangeMatrix>)>,
		mark_type: Res<TransformWillChangeRenderContextMarkType>,
	) {
		let query_item = query.get_by_entity(e.id);

		let mut write_item = write.get_unchecked_mut_by_entity(e.id);
		let mut render_mark_value = write_item.0.get_or_default().clone();

		match query_item {
			Some(_) => {
				render_mark_value.set(***mark_type, true);
			},
			_ => {
				write_item.1.remove();
				render_mark_value.set(***mark_type, false);
				if render_mark_value.not_any() {
					write_item.0.remove();
					return;
				}
			},
		};

		write_item.0.write(render_mark_value);
	}
}