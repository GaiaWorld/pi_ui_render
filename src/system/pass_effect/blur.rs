use bevy::ecs::{
    prelude::RemovedComponents,
    query::Changed,
    system::{ParamSet, Query},
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

use crate::{components::user::Blur, resource::RenderContextMarkType};

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use pi_postprocess::effect::BlurDual;

// 处理blur属性，将其设置在PostProcess上
// 如果Blur删除，设置PostProcess的blur_dual属性为None
// 如果Blur修改，设置PostProcess中的blur_dual属性为对应值
pub fn blur_post_process(
    mut del: RemovedComponents<Blur>,
    mark_type: OrInitRes<RenderContextMarkType<Blur>>,
    mut query: ParamSet<(Query<(&Blur, &mut PostProcess, &mut PostProcessInfo), Changed<Blur>>, Query<(&mut PostProcess, &mut PostProcessInfo)>)>,
) {
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok((mut post_list, mut post_info)) = p1.get_mut(del) {
            post_list.blur_dual = None;
            post_info.effect_mark.set(***mark_type, false);
        }
    }

    for (blur, mut post_list, mut post_info) in query.p0().iter_mut() {
        if **blur > 0.0 {
            post_list.blur_dual = Some(BlurDual {
                radius: blur.0 as u8,
                iteration: 2,
                intensity: 1.0,
                simplified_up: false,
            });
            post_info.effect_mark.set(***mark_type, true);
        } else {
            post_list.blur_dual = None;
            post_info.effect_mark.set(***mark_type, false);
        }
    }
}
