use bevy::{ecs::{
    prelude::RemovedComponents,
    query::Changed,
    system::{ParamSet, Query},
}, prelude::{Added, Or}};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

use crate::{components::user::Opacity, resource::RenderContextMarkType};

use pi_postprocess::effect::Alpha;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};


/// 处理opacity属性
/// 如果Opacity删除， 设置PostProcessList的alpha属性为None
/// 如果Opacity修改， 设置PostProcessList的alpha属性为对应值
pub fn opacity_post_process(
    mut del: RemovedComponents<Opacity>,
    mark_type: OrInitRes<RenderContextMarkType<Opacity>>,
    mut query: ParamSet<(Query<(&Opacity, &mut PostProcess, &mut PostProcessInfo), Or<(Changed<Opacity>, Added<PostProcess>)>>, Query<(&mut PostProcess, &mut PostProcessInfo)>)>,
) {
    // opacity 如果删除， 取消opacity的后处理
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok((mut post_list, mut post_info)) = p1.get_mut(del) {
            post_list.alpha = None;
            post_info.effect_mark.set(***mark_type, false);
        }
    }

    for (opacity, mut post_list, mut post_info) in query.p0().iter_mut() {
        if **opacity >= 1.0 {
            post_list.alpha = None;
            post_info.effect_mark.set(***mark_type, false);
        } else {
            post_list.alpha = Some(Alpha { a: opacity.0 });
            post_info.effect_mark.set(***mark_type, true);
        }
    }
}