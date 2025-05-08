
use pi_style::style::StyleType;
use pi_world::{app::App, event::{ComponentChanged, ComponentAdded}, prelude::{IntoSystemConfigs, Plugin}, query::Query, single_res::SingleRes};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crate::{components::user::Opacity, resource::{GlobalDirtyMark, IsRun, RenderContextMarkType}, system::{base::pass::pass_life::{self, pass_mark}, system_set::UiSystemSet}};

use pi_postprocess::effect::Alpha;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use crate::prelude::UiStage;

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UiStage, pass_mark::<Opacity>
                .in_set(UiSystemSet::PassMark)
                .run_if(opacity_changed)
                .before(pass_life::cal_context))
            .add_system(UiStage, opacity_post_process
                .in_set(UiSystemSet::PassSetting)
                .run_if(opacity_changed)
                .after(UiSystemSet::PassFlush))
        ;
    }
}

/// 处理opacity属性
/// 如果Opacity修改， 设置PostProcessList的alpha属性为对应值
/// 如果Opacity修改为1， 设置PostProcessList的alpha属性为None
/// 不可删除Opacity组件， 请设置默认值为1
pub fn opacity_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<Opacity>>,
    opacity_change: ComponentChanged<Opacity>,
    opacity_added: ComponentAdded<Opacity>,
    mut query: Query<(&Opacity, &mut PostProcess, &mut PostProcessInfo)>,
    // remove: ComponentRemoved<Opacity>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // opacity 如果删除， 取消opacity的后处理
    // let p1 = query.p1();
    // for i in remove.iter() {
    //     if let Ok((mut post_list, mut post_info, has_opacity)) = p1.get_mut(*i) {
    //         if has_opacity {
    //             continue;
    //         }
    //         post_list.alpha = None;
    //         render_mark_false(***mark_type, &mut render_mark_value);
    //     }
    // }
   
    for entity in opacity_change.iter().chain(opacity_added.iter()) {
        if let Ok((opacity, mut post_list, mut post_info)) = query.get_mut(*entity) {
            log::debug!("opacity: {:?}", (entity, **opacity));
            if **opacity >= 1.0 {
                post_list.alpha = None;
                post_info.effect_mark.set(***mark_type, false);
            } else {
                post_list.alpha = Some(Alpha { a: opacity.0 });
                post_info.effect_mark.set(***mark_type, true);
            }
        }
    }
}

pub fn opacity_changed(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::Opacity as usize).map_or(false, |display| {*display == true})
}
