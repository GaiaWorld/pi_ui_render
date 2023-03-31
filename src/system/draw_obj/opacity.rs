use bevy::ecs::{
    query::Changed,
    system::{ParamSet, Query},
	prelude::RemovedComponents,
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_postprocess::effect::alpha::Alpha;

use crate::{components::{pass_2d::PostProcessList, user::Opacity}, resource::RenderContextMarkType};


/// 计算半透明后处理#[system]
pub fn opacity_post_process(
    mut del: RemovedComponents<Opacity>,
	mark_type: OrInitRes<RenderContextMarkType<Opacity>>,
    mut query: ParamSet<(Query<(&Opacity, &mut PostProcessList), Changed<Opacity>>, Query<&mut PostProcessList>)>,
) {
    // opacity 如果删除， 取消opacity的后处理
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok(mut post_list) = p1.get_mut(del) {
            post_list.alpha = None;
			post_list.effect_mark.set(***mark_type, false);
        }
    }

    for (opacity, mut post_list) in query.p0().iter_mut() {
        if **opacity >= 1.0 {
            post_list.alpha = None;
			post_list.effect_mark.set(***mark_type, false);
        } else {
            post_list.alpha = Some(Alpha { a: opacity.0 });
			post_list.effect_mark.set(***mark_type, true);
        }
    }
}
