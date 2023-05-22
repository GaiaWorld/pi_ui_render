//! # 背景
//! 如Opacity、Blur、Hsv、MaskImage等，其效果需要将节点自身及其递归子节点作为一个整体来处理
//! 拥有这类属性的节点需要先将其自身包含的渲染对象和其递归子节点的渲染对象先渲染到一个fbo上，再将该fbo附加对应效果呈现出来。pass: 比如，opacity如果作用在每个渲染对象上，叠加后的效果是错误的
//!
//! # 思路
//! 本模块不关心具体需要成为渲染上下文的属性，而是只关心RenderContextMark组件，该组件存在时，则为渲染上下文，不存在时，就不是渲染上下文
//! 由于只关心RenderContextMark组件，而不关心具体属性（opacity、Blur等），外部可根据需要扩展属性，而不影响本模块的逻辑
//! 为每个渲染上下文节点，单独创建一个`Pass2D`来渲染其自身包含的渲染对象、以及其递归子节点上包含的渲染对象。
//!
//! # 具体逻辑
//! 本模块做以下事情：
//! 1. 为渲染上下文节点，创建Pass2D实体。pass: 通过监听RenderContextMark中的Create、Delete删除或创建Pass2D实体
//! 2. 在节点上，建立由其创建的Pass2D实体的索引(Pass2DId)，pass:当RenderContextMark组件删除，或Node实体销毁时，能够删除其对应的Pass2D实体
//! 3. 创建Pass2D对创建它的节点的索引(NodeId), 使得可以通过Pass2D反向查询到其对应节点上的组件
//! 4. 在节点上创建其所在的Pass2D实体的索引（InPass2DId），表明节点上的渲染对象应该渲染到那个Psss2D上。
//!
//!

use bevy::{
    ecs::{
        prelude::{Component, Entity, EventReader, EventWriter, RemovedComponents},
        query::Changed,
        system::{Commands, ParamSet, Query},
        world::Mut,
    },
    prelude::Local,
};
use pi_bevy_ecs_extend::{
    prelude::{Layer, LayerDirty, Up},
    system_param::{layer_dirty::ComponentEvent, res::OrInitRes},
};
use pi_densevec::DenseVecMap;
use pi_map::Map;
use pi_null::Null;

use crate::{
    components::{
        calc::{EntityKey, InPassId, NeedMark, RenderContextMark},
        pass_2d::{Camera, ChildrenPass, ParentPassId},
        PassBundle,
    },
    resource::RenderContextMarkType,
};

/// 记录RenderContext添加和删除的脏，同时记录节点添加到树上的脏
/// 根据脏，从父向子递归，设置节点所在的渲染上下文（节点的渲染目标）
pub fn cal_context(
    mut command: Commands,
    // mut layer_dirty: Local<LayerDirty<Entity>>,
    mut context_mark1: ParamSet<(
        Query<(Entity, &RenderContextMark, Option<&Camera>)>,
        Query<(&mut InPassId, &RenderContextMark, Option<&mut ParentPassId>)>,
        Query<&mut InPassId>,
    )>,
    // idtree: EntityTree,
    // down: Query<&Down>,
    up: Query<&Up>,
    // mut parent_pass_id: Query<&'static mut ParentPassId>,
    mut event_reader: EventReader<ComponentEvent<Changed<RenderContextMark>>>,
    mut event_writer: EventWriter<ComponentEvent<Changed<ParentPassId>>>,
    mut layer_dirty: LayerDirty<Changed<Layer>>,
    // mut layer_change: EventReader<ComponentEvent<Changed<Layer>>>,
) {
    // layer_dirty.clear();
    let mut pass_2d_init = Vec::new();
    // let mut pass_2d_id_insert = Vec::new();


    // 如果mark修改，加入层脏
    for change in event_reader.iter() {
        let p0 = context_mark1.p0();
        if let Ok((entity, mark, camera)) = p0.get(change.id) {
            if camera.is_some() && mark.not_any() {
                // 删除pass
                layer_dirty.mark(entity);
            } else if camera.is_none() && mark.any() {
                // 不存在对应的pass2D， 则创建(放入层脏，按层创建)
                layer_dirty.mark(entity);
            }
        }
    }

    // // 迭代所有layer改变的节点， 如果layer不为null，则添加到层脏
    // for i in layer_change.iter() {
    // 	layer_dirty.mark(i.id);
    // }

    // 按层迭代
    for node in layer_dirty.iter() {
        let parent_context_id = match up.get(node) {
			Ok(r) if let Ok(in_pass_id) = context_mark1.p2().get(r.parent()) => **in_pass_id,
			_ => EntityKey::null(),
		};

        if let Ok((mut in_pass_id, mark, parent_pass_id)) = context_mark1.p1().get_mut(node) {
            // mark已清空，但相机依然存在，则删除pass, 重新设置pass字节点的in_pass_id
            if parent_pass_id.is_some() && mark.not_any() {
                // // 删除pass
                // if in_pass_id.is_null() {
                // 	continue;
                // }
                // command.entity(***camera.unwrap()).despawn();
                // 修改in_pass_id为父的Pass2D
                *in_pass_id = InPassId(parent_context_id);
                // 移除Pass2D
                command.entity(node).remove::<PassBundle>();
                // 删除后，其子节点的in_pass_id修改为parent_context_id
                parent_context_id.0
            } else if mark.any() {
                // log::warn!("pass======node: {:?}, parent_context_id: {:?}", *node, parent_context_id);
                match parent_pass_id {
                    None => {
                        pass_2d_init.push((node, PassBundle::new(*parent_context_id)));
                        // 父的
                        event_writer.send(ComponentEvent::new(node));
                    }
                    Some(mut parent_pass_id) => {
                        if ***parent_pass_id != *parent_context_id {
                            **parent_pass_id = parent_context_id;
                            event_writer.send(ComponentEvent::new(node));
                        }
                    }
                };
                // 修改in_pass_id为当前Pass2D
                *in_pass_id = InPassId(EntityKey(node));
                // 添加后，其子节点的in_pass_id修改为当前创建的parent_context_id
                node
            } else {
                // 不是一个renderContext， 则其in_pass_id为parent_context_id
                *in_pass_id = InPassId(parent_context_id);
                parent_context_id.0
            };

            // let children_item = match down.get(node) {
            //     Ok(r) => r,
            //     _ => continue,
            // };

            // recursive_set_node_context(
            //     children_item.head(),
            //     &idtree,
            //     &down,
            //     &mut context_mark1.p1(),
            //     &mut parent_pass_id,
            //     EntityKey(in_pass_id),
            // );
        }
    }

    // 批量设置插入指令（PassBundle）
    if pass_2d_init.len() > 0 {
        command.insert_or_spawn_batch(pass_2d_init.into_iter());
    }
}

/// Pass2D设置children
pub fn calc_pass_children_and_clear(
    mut event_reader: EventReader<ComponentEvent<Changed<RenderContextMark>>>,
    mut query: Query<&mut ChildrenPass>,
    query_pass: Query<(Entity, &ParentPassId)>,
    mut local: Local<DenseVecMap<(Entity, ChildrenPass)>>,
) {
    if event_reader.len() > 0 {
        event_reader.clear();
        // 重新组织渲染上下文的树
        for (entity, parent) in query_pass.iter() {
            if parent.0.is_null() {
                continue;
            }
            match local.get_mut(&(parent.index() as usize)) {
                Some(r) => r.1.push(EntityKey(entity)),
                None => {
                    let mut c = ChildrenPass::default();
                    c.push(EntityKey(entity));
                    local.insert(parent.index() as usize, ((***parent).clone(), c));
                }
            }
        }

        for item in local.values() {
            if let Ok(mut children) = query.get_mut(item.0) {
                *children = item.1.clone(); // 不clone, TODO
            }
        }

        local.clear();
    }
}

/// 标记RenderContextMark
/// Opacity、Blur、Hsi等属性，需要标记RenderContextMark
/// RenderContextMark中的位标记不全为0时，后续阶段后将该节点设置为Pass节点（添加PassBundle）
pub fn pass_mark<T: Component + NeedMark>(
    mut query_set: ParamSet<(
        Query<(Entity, &T, &mut RenderContextMark), Changed<T>>,
        Query<&'static mut RenderContextMark>,
    )>,
    del: RemovedComponents<T>,
    // render_mark: Query<Write<>>,
    mark_type: OrInitRes<RenderContextMarkType<T>>,

    mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
) {
    let mut render_context = query_set.p1();
    // Opacity组件删除，取消渲染上下文标记
    context_attr_del(del, ***mark_type, &mut event_writer, &mut render_context);

    // Opacity修改，如果<1.0, 设置渲染上下文标记， 否则取消渲染上下文标记
    for (entity, value, mut render_mark_value) in query_set.p0().iter_mut() {
        if value.need_mark() {
            render_mark_true(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        } else {
            render_mark_false(entity, ***mark_type, &mut event_writer, &mut render_mark_value);
        }
    }
}

fn context_attr_del<T: Component>(
    mut dels: RemovedComponents<T>,
    mark_type: usize,
    event_writer: &mut EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
    render_context: &mut Query<&'static mut RenderContextMark>,
) {
    // Opacity组件删除，取消渲染上下文标记
    for del in dels.iter() {
        if let Ok(mut render_mark_value) = render_context.get_mut(del) {
            if unsafe { render_mark_value.replace_unchecked(mark_type, false) } {
                // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
                event_writer.send(ComponentEvent::new(del));
            }
        }
    }
}


#[inline]
pub fn render_mark_true(
    id: Entity,
    mark_type: usize,
    event_writer: &mut EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    if !unsafe { render_mark_value.replace_unchecked(mark_type, true) } {
        event_writer.send(ComponentEvent::new(id));
    }
}

#[inline]
pub fn render_mark_false(
    id: Entity,
    mark_type: usize,
    event_writer: &mut EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    if unsafe { render_mark_value.replace_unchecked(mark_type, false) } {
        event_writer.send(ComponentEvent::new(id));
    }
}
