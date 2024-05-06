//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_world::prelude::{Alter, Has, Removed, ParamSet, Changed};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Root};

use crate::{
    components::{calc::RenderContextMark, RootBundle},
    resource::RenderContextMarkType,
    system::{pass::pass_life::render_mark_true, draw_obj::calc_text::IsRun},
};

/// 处理根节点
/// 如果Root组件被移除，则移除RootBundle
/// 如果Root组件被创建，则插入RootBundle
pub fn root_calc(
    mut query_set: ParamSet<(
        Alter<&mut RenderContextMark, Changed<Root>, RootBundle>, // 这里的过滤本应该是Added<Root>, pi_world中不支持Added，这里用Changed代替效果一样， 因为Root组件通常只会Added， 不会Changed
        Alter<(&'static mut RenderContextMark, Has<Root>), Removed<Root>, (), RootBundle>,
    )>,

    mark_type: OrInitSingleRes<RenderContextMarkType<Root>>,

	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // Root组件删除，取消渲染上下文标记， 并删除RootBundle
    let render_context = query_set.p1();
    let mut iter = render_context.iter_mut();
    while let Some((mut render_mark_value, has_root)) = iter.next() {
        if has_root {
            continue;
        }
        unsafe { render_mark_value.replace_unchecked(***mark_type, false) };
        // 删除root对应的RootBundle
        let _ = iter.alter(());
    }


    // Root组件添加，为其添加RootBundle
    let mut iter = query_set.p0().iter_mut();
    while let Some(mut render_mark_value) = iter.next() {
        render_mark_true(***mark_type, &mut render_mark_value);
        let _ = iter.alter(RootBundle::default());
    }
    // for (entity, mut render_mark_value, clear_color) in iter {
    //     render_mark_true(***mark_type, &mut render_mark_value);
    //     iter.alter(RootBundle::default());
    //     // match clear_color {
    //     //     Some(_) => query_set.2.entity().insert(entity, RootBundle::default()),
    //     //     None => query_set.2.entity(entity).insert(entity, (RootBundle::default(), ClearColor::default())),
    //     // };
    // }
}
