use bevy::{
    ecs::{prelude::RemovedComponents, query::Changed, system::Query},
    prelude::{Added, Or, ParamSet, Plugin, IntoSystemConfigs, Update},
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;

use crate::{
    components::{
        pass_2d::PostProcessInfo,
        user::RadialWave,
    },
    resource::RenderContextMarkType,
    system::{
        draw_obj::calc_text::IsRun, system_set::UiSystemSet,
    },
};

use crate::{components::pass_2d::PostProcess, system::pass::pass_life};

/// 水波纹效果插件
pub struct RadialWavePlugin;

impl Plugin for RadialWavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            // 标记有RadialWave组件的节点为渲染上下文
            .add_systems(Update, 
                pass_life::pass_mark::<RadialWave>
                    .before(pass_life::cal_context).in_set(UiSystemSet::PassMark)
            )
            .add_systems(Update, 
                radial_wave_post_process
                    .in_set(UiSystemSet::PassSetting)
            );
    }
}

/// 处理RadialWave组件
/// 如果RadialWave删除， 设置PostProcessList的radial_wave属性为None
/// 如果RadialWave修改， 设置PostProcessList的radial_wave属性为对应值
pub fn radial_wave_post_process(
    mut del: RemovedComponents<RadialWave>,
    mark_type: OrInitRes<RenderContextMarkType<RadialWave>>,
    mut query: ParamSet<(
        Query<(&RadialWave, &mut PostProcess, &mut PostProcessInfo), Or<(Changed<RadialWave>, Added<PostProcess>)>>,
        Query<(&mut PostProcess, &mut PostProcessInfo)>,
    )>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // RadialWave 如果删除， 取消RadialWave的后处理
    let mut p1 = query.p1();
    for del in del.iter() {
        if let Ok((mut post_list, mut post_info)) = p1.get_mut(del) {
            post_info.effect_mark.set(***mark_type, false);
			post_list.radial_wave = None;
        }
    }

	// RadialWave 如果修改，添加上下文标记， 并设置后处理
    for (radial_wave, mut post_list, mut post_info) in query.p0().iter_mut() {
		post_info.effect_mark.set(***mark_type, true);
        post_list.radial_wave = Some(radial_wave.0.clone());

		log::debug!("set RadialWave: {:?}", &post_list.radial_wave);
    }
}

