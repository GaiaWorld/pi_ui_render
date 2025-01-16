use pi_style::style::StyleType;
use pi_world::{event::{ComponentAdded, ComponentChanged}, prelude::{Entity, Query}, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crate::{components::user::Hsi, resource::{GlobalDirtyMark, IsRun, RenderContextMarkType}, system::{base::pass::pass_life::pass_mark, system_set::UiSystemSet}};

use pi_postprocess::effect::HSB;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use pi_world::prelude::{App, Plugin, IntoSystemConfigs};
use crate::prelude::UiStage;

pub struct HsiPlugin;

impl Plugin for HsiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(UiStage, hsi_post_process
            .run_if(hsi_change)
            .in_set(UiSystemSet::PassSetting)
            .after(UiSystemSet::PassFlush))
        .add_system(UiStage, pass_mark::<Hsi>
            .run_if(hsi_change)
            .in_set(UiSystemSet::PassMark))
        ;
    }
}

/// 处理hsi属性
/// 如果hsi删除，设置PostProcess中的hsb位None
/// 如果hsi修改，将其设置在PostProcess中
/// hsi组件不可删除， 需要删除时， 应该设置为默认值
pub fn hsi_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<Hsi>>,
    mut query: Query<(&Hsi, &mut PostProcess, &mut PostProcessInfo, Entity)>,
    changed: ComponentChanged<Hsi>,
    added: ComponentAdded<Hsi>,
    // remove: ComponentRemoved<Hsi>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // let p1 = query.p1();
    // for i in remove.iter() {
    //     if let Ok((mut post_list, mut post_info, hsi)) = p1.get_mut(*i) {
    //         if hsi {
    //             continue;
    //         }
    //         post_list.hsb = None;
    //         render_mark_false(***mark_type, &mut render_mark_value);
    //     }
    // }
    for entity in changed.iter().chain(added.iter()) {
        if let Ok((hsi, mut post_list, mut post_info, _entity)) = query.get_mut(*entity) {
            if hsi.saturate != 0.0 || hsi.hue_rotate != 0.0 || hsi.bright_ness != 0.0 {
                post_list.hsb = Some(HSB {
                    hue: (hsi.hue_rotate * 360.0) as i16,
                    saturate: (hsi.saturate * 100.0) as i8,
                    brightness: (hsi.bright_ness * 100.0) as i8,
                });
                post_info.effect_mark.set(***mark_type, true);
            } else {
                post_list.hsb = None;
                post_info.effect_mark.set(***mark_type, false);
            }
        }
    }
    
}


pub fn hsi_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::Hsi as usize).map_or(false, |display| {*display == true})
}
