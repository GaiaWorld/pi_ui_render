//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use bevy_ecs::{
    prelude::{Entity, EventWriter, RemovedComponents},
    query::{Added, Changed},
    system::{Commands, ParamSet, Query},
};
use pi_bevy_ecs_extend::{
    prelude::Root,
    system_param::{layer_dirty::ComponentEvent, res::OrInitRes},
};

use crate::{
    components::{calc::RenderContextMark, user::ClearColor, RootBundle},
    resource::RenderContextMarkType,
    system::{pass::pass_life::render_mark_true, draw_obj::calc_text::IsRun},
};

/// 处理根节点
/// 如果Root组件被移除，则移除RootBundle
/// 如果Root组件被创建，则插入RootBundle
pub fn root_calc(
    mut query_set: ParamSet<(
        Query<(Entity, &mut RenderContextMark, Option<&ClearColor>), Added<Root>>,
        Query<&'static mut RenderContextMark>,
    )>,

    mut del: RemovedComponents<Root>,
    mark_type: OrInitRes<RenderContextMarkType<Root>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,

    mut command: Commands,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // Root组件删除，取消渲染上下文标记， 并删除RootBundle
    let mut render_context = query_set.p1();
    for del in del.iter() {
        if let Ok(mut render_mark_value) = render_context.get_mut(del) {
            if unsafe { render_mark_value.replace_unchecked(***mark_type, false) } {
                // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
                event_writer.send(ComponentEvent::new(del));
            }
            // 删除root对应的RootBundle
            command.entity(del).remove::<RootBundle>();
        }
    }


    // Root组件添加，为其添加RootBundle
    for (entity, mut render_mark_value, clear_color) in query_set.p0().iter_mut() {
        render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        match clear_color {
            Some(_) => command.entity(entity).insert(RootBundle::default()),
            None => command.entity(entity).insert((RootBundle::default(), ClearColor::default())),
        };
    }
}
