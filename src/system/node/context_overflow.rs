//! 处理overflow属性，
//! 1. 对overflow设置为true的节点，标记为渲染上下文（设置RenderContextMark中的位标记）
//! 2.

use bevy::ecs::{
    prelude::{Entity, EventWriter, RemovedComponents},
    query::Changed,
    system::{ParamSet, Query},
};
use pi_bevy_ecs_extend::system_param::{layer_dirty::ComponentEvent, res::OrInitRes};

use crate::{components::calc::RenderContextMark, resource::RenderContextMarkType};

use super::context::{context_attr_del, render_mark_false, render_mark_true};

use crate::components::user::Overflow;

pub fn overflow_calc(
    mut query_set: ParamSet<(
        Query<(Entity, &Overflow, &mut RenderContextMark), Changed<Overflow>>,
        Query<&'static mut RenderContextMark>,
    )>,
    del: RemovedComponents<Overflow>,
    mark_type: OrInitRes<RenderContextMarkType<Overflow>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
) {
    // Oveflow组件删除，取消渲染上下文标记
    let mut render_context = query_set.p1();
    context_attr_del(del, ***mark_type, &mut event_writer, &mut render_context);

    // Oveflow为true时，标记render_context_mark对应的位
    // Oveflow为false时, 取消render_context_mark对应的位，如果发现位标记全为空，则删除RenderContextMark组件
    for (entity, overflow, mut render_mark_value) in query_set.p0().iter_mut() {
        if **overflow == true {
            render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        } else {
            render_mark_false(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        }
    }
}
