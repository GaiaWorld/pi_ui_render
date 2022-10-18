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
use pi_ecs::{
    entity::Id,
    monitor::Event,
    prelude::{ChangeTrackers, FromWorld, Join, Local, ParamSet, Query, Res, With, Write},
};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::{Layer, Up};
use pi_null::Null;


use crate::{
    components::{
        calc::{LayoutResult, NodeId, Pass2DId, RenderContextMark, TransformWillChangeMatrix, WorldMatrix},
        pass_2d::{ParentPassId, Pass2D},
        user::{Node, TransformWillChange},
    },
    resource::{draw_obj::LayerPass2D, RenderContextMarkType},
};

pub struct CalcTransformWillChange;

#[derive(Deref)]
pub struct TransformWillChangeRenderContextMarkType(RenderContextMarkType);

impl FromWorld for TransformWillChangeRenderContextMarkType {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self { Self(RenderContextMarkType::from_world(world)) }
}

#[setup]
impl CalcTransformWillChange {
    #[system]
    pub fn calc_transform_willchange(
        layer_pass_2d: Res<LayerPass2D>,
        query: Query<
            Node,
            (
                Id<Node>,
                &Up<Node>,
                &Pass2DId,
                &Layer<Node>,
                // transform_willchange_matrix在父节点的WorldMatrix、节点自身的TransformWillChange， Layer修改时，需要改变
                // 父节点的WorldMatrix, 子节点的WorldMatrix一定改变，因此这里拿到本节点的节拍
                ChangeTrackers<WorldMatrix>,
                ChangeTrackers<TransformWillChange>,
                ChangeTrackers<Layer<Node>>,
            ),
            With<TransformWillChange>,
        >,
        query_node_trans: Query<Node, (&TransformWillChange, &LayoutResult)>,
        query_node_matrix: Query<Node, &WorldMatrix>,
        query_pass2d_parent: Query<Pass2D, &ParentPassId>,
        query_pass2d_nodeid: Query<Pass2D, &NodeId>,
        mut write: ParamSet<(
            Query<Node, Write<TransformWillChangeMatrix>>,
            Query<Node, (ChangeTrackers<TransformWillChangeMatrix>, &'static TransformWillChangeMatrix)>,
            Query<
                Pass2D,
                (
                    &'static ParentPassId,
                    &'static NodeId,
                    Join<NodeId, Node, Option<&'static TransformWillChange>>,
                ),
            >,
        )>,
        mut local: Local<LayerDirty<(Id<Node>, Id<Node>, Id<Pass2D>, bool)>>,
    ) {
        let mut has_change = false;
        for (id, up, pass_id, layer, tracker_matrix, tracker_willchange, tracker_layer) in query.iter() {
            local.mark(
                (
                    id,
                    up.parent(),
                    **pass_id,
                    tracker_willchange.is_changed() || tracker_layer.is_changed() || tracker_matrix.is_changed(),
                ),
                layer.layer(),
            );
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
                has_change = true;
                let (will_change, layout) = query_node_trans.get_unchecked(*id);
                let width = layout.rect.right - layout.rect.left;
                let height = layout.rect.bottom - layout.rect.top;
                let mut matrix = WorldMatrix::form_transform_funcs(&will_change.0, width, height);
                let p_matrix = query_node_matrix.get_unchecked(*node_p_id).clone();
                let invert = p_matrix.invert().unwrap();

                let mut m = p_matrix * &matrix * invert;

                if let Some(parent_will_change_matrix) = parent_will_change_matrix {
                    m = &parent_will_change_matrix.will_change * &m;
                    matrix = &parent_will_change_matrix.primitive * &matrix;
                }

                write
                    .p0_mut()
                    .get_unchecked_mut(*id)
                    .write(TransformWillChangeMatrix::new(m.invert().unwrap(), m, matrix));
            }
        }

        local.clear();

        // 如果willChange发生了变化，pass2DLayer发生了变化，从新设置非TransformWillchange的节点的matrix
        if has_change || layer_pass_2d.is_changed() {
            for (pass_id, _layer) in layer_pass_2d.iter() {
                let (parent_id, node_id, will_change) = write.p2().get_unchecked(*pass_id);
                // 节点存在TransformWillChange， 不处理(前面已经处理)
                if will_change.is_some() || parent_id.is_null() {
                    continue;
                }
                let node_id = node_id.clone();

                let parent_node_id = query_pass2d_nodeid.get_unchecked(**parent_id);

                // 非TransformWillChange节点添加TransformWillChangeMatrix组件
                let parent_will_change = write.p0().get_unchecked(**parent_node_id);
                if let Some(parent_will_change) = parent_will_change.get() {
                    let p = parent_will_change.clone();
                    write.p0().get_unchecked(**node_id).write(p);
                }
            }
        }
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
            }
            _ => {
                write_item.1.remove();
                render_mark_value.set(***mark_type, false);
                if render_mark_value.not_any() {
                    write_item.0.remove();
                    return;
                }
            }
        };

        write_item.0.write(render_mark_value);
    }
}
