//! 处理hsi属性，对hsi中存在不为0的属性时，标记为渲染上下文（设置RenderContextMark中的位标记）

use bevy::ecs::{
    prelude::{Entity, EventWriter},
    query::Changed,
    system::{Query, RemovedComponents},
};
use pi_bevy_ecs_extend::system_param::{layer_dirty::ComponentEvent, res::OrInitRes};

use crate::{
    components::{calc::RenderContextMark, user::Hsi},
    resource::RenderContextMarkType,
};

use super::context::{context_attr_del, render_mark_false, render_mark_true};

pub fn hsi_calc(
    mut query_set: bevy::ecs::system::ParamSet<(
        Query<(Entity, &Hsi, &mut RenderContextMark), Changed<Hsi>>,
        Query<&'static mut RenderContextMark>,
    )>,
    del: RemovedComponents<Hsi>,
    mark_type: OrInitRes<RenderContextMarkType<Hsi>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
) {
    // Opacity组件删除，取消渲染上下文标记
    let mut render_context = query_set.p1();
    context_attr_del(del, ***mark_type, &mut event_writer, &mut render_context);

    // Hsi修改，如果saturate、hue_rotate、bright_ness都为0， 则取消渲染上下文标记, 否则设置渲染上下文标记
    for (entity, hsi, mut render_mark_value) in query_set.p0().iter_mut() {
        if hsi.saturate != 0.0 || hsi.hue_rotate != 0.0 || hsi.bright_ness != 0.0 {
            render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        } else {
            render_mark_false(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        }
    }
}
