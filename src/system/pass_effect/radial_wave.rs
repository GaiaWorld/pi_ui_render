use pi_style::style::StyleType;
use pi_world::{event::{ComponentAdded, ComponentChanged}, prelude::{App, Changed, ComponentRemoved, Has, IntoSystemConfigs, ParamSet, Plugin, Query}, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crate::{
    components::{
        pass_2d::PostProcessInfo,
        user::RadialWave,
    },
    resource::{GlobalDirtyMark, IsRun, RenderContextMarkType},
    system::system_set::UiSystemSet,
};

use crate::{components::pass_2d::PostProcess, system::base::pass::pass_life};
use crate::prelude::UiStage;

/// 水波纹效果插件
pub struct RadialWavePlugin;

impl Plugin for RadialWavePlugin {
    fn build(&self, app: &mut App) {
        app
            // 标记有RadialWave组件的节点为渲染上下文
            .add_system(UiStage, 
                pass_life::pass_mark::<RadialWave>
                    .run_if(radial_wave_changed)
                    .before(pass_life::cal_context)
                    .in_set(UiSystemSet::PassMark)
            )
            .add_system(UiStage, 
                radial_wave_post_process
                    .run_if(radial_wave_changed)
                    .in_set(UiSystemSet::PassSetting)
            );
    }
}

/// 处理RadialWave组件
/// 如果RadialWave删除， 设置PostProcessList的radial_wave属性为None
/// 如果RadialWave修改， 设置PostProcessList的radial_wave属性为对应值
/// RadialWave不可删除， 需删除时， 请设置默认值
pub fn radial_wave_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<RadialWave>>,
    changed: ComponentChanged<RadialWave>,
    added: ComponentAdded<RadialWave>,
    mut query: Query<(&RadialWave, &mut PostProcess, &mut PostProcessInfo)>,
    // remove: ComponentRemoved<RadialWave>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // RadialWave 如果删除， 取消RadialWave的后处理
    // let p1 = query.p1();
    // for i in remove.iter() {
    //     if let Ok((mut post_list, mut post_info, has_radial_wave)) = p1.get_mut(*i) {
    //         if has_radial_wave {
    //             continue;
    //         }
    //         render_mark_false(***mark_type, &mut render_mark_value);
    //         post_list.radial_wave = None;
    //     }
    // }

	// RadialWave 如果修改，添加上下文标记， 并设置后处理
    for entity in changed.iter().chain(added.iter()) {
        if let Ok( (radial_wave, mut post_list, mut post_info)) = query.get_mut(*entity) {
            if radial_wave.0.weight == 0.0 {
                post_info.effect_mark.set(***mark_type, false);
                post_list.radial_wave = None;
            } else {
                post_info.effect_mark.set(***mark_type, true);
                post_list.radial_wave = Some(radial_wave.0.clone());
            }
    
            log::debug!("set RadialWave: {:?}", &post_list.radial_wave);
        }
    }
}

pub fn radial_wave_changed(mut changed: ComponentAdded<RadialWave>) -> bool {
    let r = changed.len() > 0;
    changed.mark_read();
    r
}

