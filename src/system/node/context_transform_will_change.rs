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
use bevy::ecs::{
    prelude::{Entity, EventWriter},
    query::Changed,
    system::{ParamSet, Query, RemovedComponents},
};
use pi_bevy_ecs_extend::system_param::{layer_dirty::ComponentEvent, res::OrInitRes};

use crate::{components::calc::RenderContextMark, resource::RenderContextMarkType};

use super::context::{context_attr_del, render_mark_true};

use crate::components::user::TransformWillChange;


pub fn transform_willchange_calc(
    mut query_set: ParamSet<(
        Query<(Entity, &mut RenderContextMark), Changed<TransformWillChange>>,
        Query<&'static mut RenderContextMark>,
    )>,
    del: RemovedComponents<TransformWillChange>,
    mark_type: OrInitRes<RenderContextMarkType<TransformWillChange>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
) {
    // Opacity组件删除，取消渲染上下文标记
    let mut render_context = query_set.p1();
    context_attr_del(del, ***mark_type, &mut event_writer, &mut render_context);

    // TransformWillchange创建，设置渲染上下文标记
    for (entity, mut render_mark_value) in query_set.p0().iter_mut() {
        render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
    }
}
