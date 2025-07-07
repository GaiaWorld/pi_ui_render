//! 计算递归子节点是否存在动画， 以优化渲染
//! 不存在动画的节点， 如果是fbo， 缓存起来， 后续不需要再重复渲染
use pi_bevy_ecs_extend::prelude::{Layer, Root, OrInitSingleResMut, Up, OrInitSingleRes};
use pi_world::{event::{ComponentChanged, EventReader, ComponentAdded}, fetch::Ticker, prelude::{App, Entity, IntoSystemConfigs, Plugin, Query, With}};

use pi_null::Null;

use crate::{
    components::{calc::{HasAnimation, RenderContextMark}, pass_2d::{ChildrenPass, IsSteady, PostProcessInfo}, user::Animation}, resource::{GlobalDirtyMark, OtherDirtyType}, system::{base::node::user_setting::AddEvent, system_set::UiSystemSet}
};

use crate::prelude::UiStage;


pub struct RenderSteadyPlugin;

impl Plugin for RenderSteadyPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_system(UiStage, calc_has_animation.in_set(UiSystemSet::NextSetting)
				// .before(calc_animation)
			)
            .add_system(UiStage, calc_is_steady
                .in_set(UiSystemSet::PassCalc)
            )
			
			
		;
	}
}

/// * 
pub fn calc_has_animation(
    mut query: Query<(Option<&Animation>, &Up, &mut HasAnimation, Ticker<&Layer>)>,
    changed: ComponentChanged<Animation>, // Animation组件一旦创建， 不会再删除, 因此只处理改变
    added: ComponentAdded<Animation>, // Animation组件一旦创建， 不会再删除, 因此只处理改变
    mounted: EventReader<AddEvent>,
    mut has_animation_chaned: OrInitSingleResMut<HasAnimationChanged>,
) {

    let has_animation_chaned = &mut *has_animation_chaned;
    if changed.len() == 0 && mounted.len() == 0 && added.len() == 0 {
        return;
    }
    // 被挂到树上， 可能animation没有改变， 也需要迭代设置
    for i in changed.iter().chain(added.iter()).chain(mounted.iter().map(|r| { &mut r.0 })) {
        if let Ok((animation, up, mut has_animation, layer)) = query.get_mut(*i) {
            let animation_count = match animation {
                Some(animation) => animation.name.value.len(),
                None => 0,
            };

            log::debug!("calc_has_animation======={:?}", (i, animation_count));
            
            let has_animation = has_animation.bypass_change_detection();

            if animation_count == 0 {
                if has_animation.old_has_animation {
                    has_animation.child_count_width_animation -= 1;
                    has_animation.old_has_animation = false;
                    if has_animation.child_count_width_animation == 0 || layer.is_changed() {
                        has_animation_chaned.0 = true;
                    }
                } 
            } else if !has_animation.old_has_animation {
                has_animation.child_count_width_animation += 1;
                has_animation.old_has_animation = false;
                if has_animation.child_count_width_animation == 1 || layer.is_changed() {
                    has_animation_chaned.0 = true;
                }
            }
            

            let mut parent = up.parent();
            if has_animation.child_count_width_animation > 0 && has_animation.old_set_parent != parent { 
                // 有动画的节点数量大于0，且未给当前父节点贡献一个动画节点数量，则向上递归设置父节点的动画数量——1
                has_animation.old_set_parent = parent.clone();
                while !parent.is_null() {
                    if let Ok((_animation, up, mut has_animation, _layer)) = query.get_mut(parent) {
                        let has_animation = has_animation.bypass_change_detection();
                        has_animation.child_count_width_animation += 1; // 父节点动画数量+1
                        parent = up.parent();

                        if has_animation.old_set_parent == parent {
                            has_animation.old_set_parent = parent;
                            // 父节点已经向上贡献过节点数量，结束循环
                            break;
                        }
                        has_animation.old_set_parent = parent.clone();
                    }
                }
            }else if has_animation.child_count_width_animation == 0 && has_animation.old_set_parent == parent {
                // 有动画的节点数量等于0，且已经给当前父节点贡献一个动画节点数量，则向上递归设置父节点的动画数量-1
                has_animation.old_set_parent = Entity::null();
                while !parent.is_null() {
                    if let Ok((_animation, up, mut has_animation, _layer)) = query.get_mut(parent) {
                        let has_animation = has_animation.bypass_change_detection();
                        has_animation.child_count_width_animation -= 1; // 父节点动画数量-1
                        parent = up.parent();

                        if has_animation.child_count_width_animation > 0 || has_animation.old_set_parent != parent {
                            // 动画节点数量依然大于0 或 父节点没有向上贡献过节点数量，结束循环
                            has_animation.old_set_parent = Entity::null();
                            break;
                        }
                        has_animation.old_set_parent = Entity::null();
                    }
                }
            }
        }
    }
}

/// 计算哪些Pass2d节点，是根部的、无动画的节点(这些节点标记为稳定节点)
/// 稳定节点在渲染后缓存，可优化渲染性能
/// 注： 当前Pass2d节点C的child_count_width_animation == 0，并且当前节点是一个存在fbo的节点， 其父Pass2d节点A的child_count_width_animation > 0, 则认为当前节点C是稳定
pub fn calc_is_steady(
    query: Query<(&ChildrenPass, &PostProcessInfo, &HasAnimation)>,
    mut query_steady: Query<&mut IsSteady, With<ChildrenPass>>,
    query_root: Query<Entity, With<Root>>,
    mut has_animation_chaned: OrInitSingleResMut<HasAnimationChanged>,
    mark_changed: ComponentChanged<RenderContextMark>,
    global_mark: OrInitSingleRes<GlobalDirtyMark>,
) {
    // HasAnimation不改变， 上下文标记也不改变，节点也未被删除， 不需要重新计算
    if !has_animation_chaned.0 && mark_changed.len() == 0 && global_mark.get(OtherDirtyType::NodeTreeDel as usize).as_deref() == Some(&false) {
        return;
    }

    has_animation_chaned.0 = false;
    mark_changed.mark_read();
    
    for entity in query_root.iter() {
        recursive_set_is_steady(entity, false, &mut query_steady, &query);
    }
}

// 递归设置IsSteady
fn recursive_set_is_steady(
    entity: Entity,
    mut parent_steady: bool,
    // parent_pass: &ParentPassId,
    // children_pass: &ChildrenPass,
    query_steady: &mut Query<&mut IsSteady, With<ChildrenPass>>,
    query: &Query<(&ChildrenPass, &PostProcessInfo, &HasAnimation)>,
) {

    let children_pass = if let (Ok(mut steady), Ok((children_pass, post_info, has_animation))) = (query_steady.get_mut(entity), query.get(entity)) {
        // let steady = steady.bypass_
        if parent_steady {
            steady.0 = false; // 父是稳定渲染， 则设置自身为不稳定渲染（稳定渲染的用途是缓存渲染结果， 缓存了父，则不需要缓存子）
        } else {
            steady.0 = if post_info.has_effect() && has_animation.child_count_width_animation == 0 {
                // 有特效，则有fbo， 并且child_count_width_animation为0， 标记为稳定渲染
                log::debug!("steady true================{:?}", entity);
                true
            } else {
                false
            };
            parent_steady = steady.0;
        }
        children_pass
    } else {
        return;
    };
    
    
    // 递归设置
    for child_pass in children_pass.iter() {
        recursive_set_is_steady(**child_pass, parent_steady, query_steady, query);
    }
}

#[derive(Default)]
pub struct HasAnimationChanged(pub bool);