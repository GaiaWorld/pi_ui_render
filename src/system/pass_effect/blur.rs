use pi_world::prelude::{Changed, ParamSet, Query, Removed, Has};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crate::{components::user::Blur, resource::RenderContextMarkType, system::draw_obj::calc_text::IsRun};

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use pi_postprocess::effect::BlurDual;

// 处理blur属性，将其设置在PostProcess上
// 如果Blur删除，设置PostProcess的blur_dual属性为None
// 如果Blur修改，设置PostProcess中的blur_dual属性为对应值
pub fn blur_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<Blur>>,
    mut query: ParamSet<(
        Query<(&Blur, &mut PostProcess, &mut PostProcessInfo), Changed<Blur>>,
        Query<(&mut PostProcess, &mut PostProcessInfo, Has<Blur>), Removed<Blur>>,
    )>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let p1 = query.p1();
    for (mut post_list, mut post_info, has_blur) in p1.iter_mut() {
        if has_blur {
            continue;
        }
        post_list.blur_dual = None;
        post_info.effect_mark.set(***mark_type, false);
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
