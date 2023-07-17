use bevy::{ecs::{prelude::RemovedComponents, query::Changed, system::Query}, prelude::{Or, Added, Entity}};
use pi_bevy_ecs_extend::{system_param::res::OrInitRes};

use crate::{components::user::Hsi, resource::RenderContextMarkType};

use bevy::ecs::system::ParamSet;
use pi_postprocess::effect::HSB;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};


/// 处理hsi属性
/// 如果hsi删除，设置PostProcess中的hsb位None
/// 如果hsi修改，将其设置在PostProcess中
pub fn hsi_post_process(
    mut del: RemovedComponents<Hsi>,
    mark_type: OrInitRes<RenderContextMarkType<Hsi>>,
    mut query: ParamSet<(Query<(&Hsi, &mut PostProcess, &mut PostProcessInfo, Entity), Or<(Changed<Hsi>, Added<PostProcess>)>>, Query<(&mut PostProcess, &mut PostProcessInfo)>)>,
) {
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok((mut post_list, mut post_info)) = p1.get_mut(del) {
            post_list.hsb = None;
            post_info.effect_mark.set(***mark_type, false);
        }
    }

    for (hsi, mut post_list, mut post_info, entity) in query.p0().iter_mut() {
        if hsi.saturate != 0.0 || hsi.hue_rotate != 0.0 || hsi.bright_ness != 0.0 {
            post_list.hsb = Some(HSB {
                hue: (hsi.hue_rotate * 360.0) as i16,
                saturate: (hsi.saturate * 100.0) as i8,
                brightness: (hsi.bright_ness * 100.0) as i8,
            });
            post_info.effect_mark.set(***mark_type, true);
        } else {
            post_list.hsb = None;
			log::warn!("hsb none======={:?}", entity);
            post_info.effect_mark.set(***mark_type, false);
        }
    }
}
