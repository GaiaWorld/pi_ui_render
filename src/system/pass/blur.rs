use bevy::ecs::{
    query::Changed,
    system::{ParamSet, Query},
	prelude::RemovedComponents,
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_postprocess::effect::blur_dual::BlurDual;

use crate::{components::{pass_2d::PostProcessList, user::Blur}, resource::RenderContextMarkType};

pub fn blur_post_process(
    mut del: RemovedComponents<Blur>,
	mark_type: OrInitRes<RenderContextMarkType<Blur>>,
    mut query: ParamSet<(Query<(&Blur, &mut PostProcessList), Changed<Blur>>, Query<&mut PostProcessList>)>,
) {
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok(mut post_list) = p1.get_mut(del) {
            post_list.blur_dual = None;
			post_list.effect_mark.set(***mark_type, false);
        }
    }

    for (blur, mut post_list) in query.p0().iter_mut() {
        if **blur > 0.0 {
            post_list.blur_dual = Some(BlurDual {
                radius: blur.0 as u8,
                iteration: 2,
                intensity: 1.0,
                simplified_up: false,
            });
			post_list.effect_mark.set(***mark_type, true);
        } else {
            post_list.blur_dual = None;
			post_list.effect_mark.set(***mark_type, false);
        }
    }
}
