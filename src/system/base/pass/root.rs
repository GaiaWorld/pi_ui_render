//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_world::prelude::{Alter, Changed, Has, ParamSet, ComponentRemoved};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Root};

use crate::{
    components::{calc::RenderContextMark, RootBundle},
    resource::{IsRun, RenderContextMarkType},
    system::base::pass::pass_life::render_mark_true,
};

/// 处理根节点
/// 如果Root组件被移除，则移除RootBundle
/// 如果Root组件被创建，则插入RootBundle
pub fn root_calc(
    mut query_set: ParamSet<(
        Alter<&mut RenderContextMark, Changed<Root>, RootBundle>, // 这里的过滤本应该是Added<Root>, pi_world中不支持Added，这里用Changed代替效果一样， 因为Root组件通常只会Added， 不会Changed
        Alter<(&'static mut RenderContextMark, Has<Root>), (), (), RootBundle>,
    )>,
    remove: ComponentRemoved<Root>,

    mark_type: OrInitSingleRes<RenderContextMarkType<Root>>,
    // mut l: Local<usize>,

	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // *l += 1;
    // if *l {
    //     return;
    // }
    // *l = true;
    // Root组件删除，取消渲染上下文标记， 并删除RootBundle
    let render_context = query_set.p1();
    for i in remove.iter() {
        if let Ok((mut render_mark_value, has_root)) = render_context.get_mut(*i) {
            if has_root {
                continue;
            }
            unsafe { render_mark_value.replace_unchecked(***mark_type, false) };
            // 删除root对应的RootBundle
            let _ = render_context.alter(*i, ());
        }
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
