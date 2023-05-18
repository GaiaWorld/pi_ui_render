//! 处理opacity属性，对opacity设置小于1.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use bevy::ecs::{
    prelude::{Entity, EventWriter, RemovedComponents},
    query::Changed,
    system::{ParamSet, Query},
};
use pi_bevy_ecs_extend::system_param::{layer_dirty::ComponentEvent, res::OrInitRes};

use crate::{
    components::{calc::RenderContextMark, user::Opacity},
    resource::RenderContextMarkType,
};

use super::context::{context_attr_del, render_mark_false, render_mark_true};

pub fn opacity_calc(
    mut query_set: ParamSet<(
        Query<(Entity, &Opacity, &mut RenderContextMark), Changed<Opacity>>,
        Query<&'static mut RenderContextMark>,
    )>,
    del: RemovedComponents<Opacity>,
    // render_mark: Query<Write<>>,
    mark_type: OrInitRes<RenderContextMarkType<Opacity>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
) {
    let mut render_context = query_set.p1();
    // Opacity组件删除，取消渲染上下文标记
    context_attr_del(del, ***mark_type, &mut event_writer, &mut render_context);

    // Opacity修改，如果<1.0, 设置渲染上下文标记， 否则取消渲染上下文标记
    for (entity, opacity, mut render_mark_value) in query_set.p0().iter_mut() {
        if **opacity < 1.0 {
            render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        } else {
            render_mark_false(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        }
    }
}
