use pi_style::style::StyleType;
use pi_world::app::App;
use pi_world::prelude::{Changed, ParamSet, Query, Has, ComponentRemoved, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;
use pi_world::single_res::SingleRes;

use crate::resource::{GlobalDirtyMark, IsRun};
use crate::system::base::pass::pass_life;
use crate::system::system_set::UiSystemSet;
use crate::{components::user::Blur, resource::RenderContextMarkType};

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use pi_postprocess::effect::BlurDual;
use pi_world::prelude::Plugin;
use crate::prelude::UiStage;

pub struct BlurPlugin;

impl Plugin for BlurPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(UiStage, 
                pass_life::pass_mark::<Blur>
                .in_set(UiSystemSet::PassMark)
                .run_if(blur_change)
            )
            .add_system(UiStage, 
                blur_post_process
                .run_if(blur_change)
                .in_set(UiSystemSet::PassSetting)
                .after(UiSystemSet::PassFlush)
            )
        ;
    }
}

// 处理blur属性，将其设置在PostProcess上
// 如果Blur删除，设置PostProcess的blur_dual属性为None
// 如果Blur修改，设置PostProcess中的blur_dual属性为对应值
pub fn blur_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<Blur>>,
    mut query: ParamSet<(
        Query<(&Blur, &mut PostProcess, &mut PostProcessInfo), Changed<Blur>>,
        Query<(&mut PostProcess, &mut PostProcessInfo, Has<Blur>)>,
    )>,
    remove: ComponentRemoved<Blur>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let p1 = query.p1();
    for i in remove.iter() {
        if let Ok((mut post_list, mut post_info, has_blur)) = p1.get_mut(*i) {
            if has_blur {
                continue;
            }
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

pub fn blur_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::Blur as usize).map_or(false, |display| {*display == true})
}
