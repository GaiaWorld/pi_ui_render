
use pi_world::prelude::{Changed, Query, Has, Removed, ParamSet};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crate::{components::user::Opacity, resource::RenderContextMarkType, system::draw_obj::calc_text::IsRun};

use pi_postprocess::effect::Alpha;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};


/// 处理opacity属性
/// 如果Opacity删除， 设置PostProcessList的alpha属性为None
/// 如果Opacity修改， 设置PostProcessList的alpha属性为对应值
pub fn opacity_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<Opacity>>,
    mut query: ParamSet<(
        Query<(&Opacity, &mut PostProcess, &mut PostProcessInfo), Changed<Opacity>>,
        Query<(&mut PostProcess, &mut PostProcessInfo, Has<Opacity>), Removed<Opacity>>,
    )>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // opacity 如果删除， 取消opacity的后处理
    let p1 = query.p1();
    for (mut post_list, mut post_info, has_opacity) in p1.iter_mut() {
        if has_opacity {
            continue;
        }
        post_list.alpha = None;
        post_info.effect_mark.set(***mark_type, false);
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
