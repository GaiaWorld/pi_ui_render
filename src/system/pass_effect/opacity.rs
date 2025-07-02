
use pi_style::style::StyleType;
use pi_world::{app::App, event::{ComponentAdded, ComponentChanged}, filter::{With, Without}, prelude::{IntoSystemConfigs, Plugin}, query::Query, single_res::SingleRes, world::Entity};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};

use crate::{components::{calc::{DrawList, RenderContextMark}, draw_obj::InstanceIndex, user::{IsLeaf, Opacity}}, resource::{draw_obj::InstanceContext, GlobalDirtyMark, IsRun, RenderContextMarkType}, shader1::batch_meterial::OpacityUniform, system::{base::pass::pass_life::{self, pass_mark, render_mark_false, render_mark_true}, system_set::UiSystemSet}};

use pi_postprocess::effect::Alpha;

use crate::components::pass_2d::{PostProcess, PostProcessInfo};
use crate::prelude::UiStage;
use crate::components::calc::NeedMark;

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UiStage, opacity_pass_mark
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

pub fn opacity_pass_mark(
    mut query: Query<( &Opacity, &mut RenderContextMark, Option<&IsLeaf>), With<Opacity>>,
    opacity_change: ComponentChanged<Opacity>,
    opacity_added: ComponentAdded<Opacity>,
    mark_type: OrInitSingleRes<RenderContextMarkType<Opacity>>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // context_attr_del(query_set.p1(), &mut removed, ***mark_type);
    for entity in opacity_change.iter().chain(opacity_added.iter()) {
        if let Ok( (value, mut render_mark_value, is_leaf)) = query.get_mut(*entity) {
            if is_leaf.is_none() && value.need_mark() { // 非叶子节点， 并且opacity值小于1.0, 才需要标记为渲染上下文
                render_mark_true( ***mark_type, &mut render_mark_value);
                // if std::any::type_name::<T>().contains("AsImage") {
                    log::debug!("pass_mark_true,{:?}, {:?}", entity, std::any::type_name::<Opacity>());
                // }
            
            } else {
                render_mark_false( ***mark_type, &mut render_mark_value);
                // if std::any::type_name::<T>().contains("AsImage") {
                    log::debug!("pass_mark_false,{:?}, {:?}", entity, std::any::type_name::<Opacity>());
                // }
            }
        }
    }
}

/// 处理opacity属性
/// 如果Opacity修改， 设置PostProcessList的alpha属性为对应值
/// 如果Opacity修改为1， 设置PostProcessList的alpha属性为None
/// 不可删除Opacity组件， 请设置默认值为1
pub fn opacity_post_process(
    mut instances: OrInitSingleResMut<InstanceContext>,
    mark_type: OrInitSingleRes<RenderContextMarkType<Opacity>>,
    opacity_change: ComponentChanged<Opacity>,
    opacity_added: ComponentAdded<Opacity>,
    mut query: Query<(&Opacity, &mut PostProcess, &mut PostProcessInfo), Without<IsLeaf>>,
    mut query1: Query<(&Opacity, &DrawList), With<IsLeaf>>,
    mut query_draw: Query<&InstanceIndex>,
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
        // 是叶子节点， opacity设置在实例属性上
        // 这里的逻辑不需要考虑与实例分配系统的顺序问题， 实例分配系统会使用正确的opacity值初始化实例
        if let Ok((opacity, draw_list)) = query1.get(*entity) {
            for draw_id in draw_list.iter() {
				if let Ok(instance_index) = query_draw.get(draw_id.id) {
					let alignment = instances.instance_data.alignment;
					let count = instance_index.0.len() / alignment;
					for index in 0..count {
						set_instance_opacity(opacity.0, instance_index.0.start + index * alignment, &mut instances);
					}
				}
			}
        }else if let Ok((opacity, mut post_list, mut post_info)) = query.get_mut(*entity) {
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

fn set_instance_opacity(opacity: f32, instance_start: usize, instances: &mut InstanceContext) {
	let mut instance_data = instances.instance_data.instance_data_mut(instance_start);
    instance_data.set_data(&OpacityUniform(&[opacity]));
}

pub fn opacity_changed(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::Opacity as usize).map_or(false, |display| {*display == true})
}
