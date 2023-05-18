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

use bevy::{ecs::{
    prelude::{Component, Entity, EventReader, EventWriter, RemovedComponents},
    query::Changed,
    system::{Commands, ParamSet, Query},
    world::Mut,
}, prelude::Local};
use pi_bevy_ecs_extend::{
    prelude::{Up, Layer, LayerDirty},
    system_param::layer_dirty::ComponentEvent,
};
use pi_densevec::DenseVecMap;
use pi_map::Map;
use pi_null::Null;

use crate::{
    components::{
        calc::{EntityKey, InPassId, RenderContextMark},
        pass_2d::{Camera, ParentPassId, ChildrenPass},
        PassBundle,
    },
};

/// system
/// 记录RenderContext添加和删除的脏，同时记录节点添加到树上的脏
/// 根据脏，从父向子递归，设置节点所在的渲染上下文（节点的渲染目标）
pub fn cal_context(
    mut command: Commands,
    // mut layer_dirty: Local<LayerDirty<Entity>>,
    mut context_mark1: ParamSet<(
        Query<(Entity, &RenderContextMark, Option<&Camera>)>,
        Query<(&mut InPassId, &RenderContextMark, Option<&mut ParentPassId>)>,
		Query<&mut InPassId>
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
				match parent_pass_id{
					None => {
						pass_2d_init.push((node, PassBundle::new(*parent_context_id)));
						// 父的
						event_writer.send(ComponentEvent::new(node));
					},
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


// /// 监听RenderContextMark和Layer的创建方法，创建渲染上下文（未考虑层修改的情况，是否需要？ TODO）
// #[listen(component=(Node, RenderContextMark, Create), component=(Node, Layer, Create))]
// pub fn context_mark_create(
// 	e: Event,
// 	context_id: Query<Node, Write<Pass2DId>>,
// 	mark_layer: Query<Node, (&RenderContextMark, &Layer)>,
// 	mut command: EntityCommands<Pass2D>,
// 	mut node_id: Commands<NodeId>,

// 	// mut render_context_layer_list: ResMut<RenderContextLayerList>,
// ) {
// 	// 不存在RenderContextMark或者不存在Layer，都不处理
// 	// 成为渲染上下文的充要条件是：RenderContextMark组件存在，并且节点在渲染树上
// 	if let Some((_, layer)) = mark_layer.get(e.id) {
// 		// 创建一个RenderContext
// 		let context = command.spawn();
// 		// 建立Node到RenderContext的索引关系
// 		context_id.get_unchecked(e.id).write(Pass2DId(context));
// 		// 建立RenderContext到Node的索引关系
// 		node_id.insert(context, NodeId(e.id));

// 		// // 添加到渲染列表中（伴随着层信息，可以按层进行迭代）
// 		// render_context_layer_list.mark(context, **layer);
// 	}
// }

// /// 监听RenderContextMark、Layer、Node的移除事件，移除对应的渲染上下文
// #[listen(component=(Node, RenderContextMark, Delete), component=(Node, Layer<Node>, Delete), entity=(Node, Delete))]
// pub fn context_mark_remove(
//     e: Event,
//     context_id: Query<Node, Write<Pass2DId>>,
//     layer: Query<Node, &Layer<Node>>,
//     mut command: EntityDelete<Pass2D>,
//     mut layer_pass_2d: ResMut<LayerPass2D>,
//     // mut render_context_layer_list: ResMut<RenderContextLayerList>,
// ) {
//     if let (Some(context), Some(layer)) = (
//         context_id.get_unchecked_by_entity(e.id).remove(),
//         layer.get(unsafe { Id::<Node>::new(e.id.local()) }),
//     ) {
//         // 删除RenderContext实体
//         command.despawn(*context);
//         layer_pass_2d.delete(*context, layer.layer());
//         // render_context_layer_list.delete(*context, **layer);
//     }
// }

// #[listen(component=(Node, Root, (Create, Delete)))]
// pub fn root_change(
//     e: Event,
//     mut pass2d_query: ParamSet<(EntityInsert<Pass2D>, EntityDelete<Pass2D>, Query<Pass2D, Write<NodeId>>)>,

//     mut node_query: Query<Node, Write<Pass2DId>>,
// ) {
//     match e.ty {
//         EventType::Create => {
//             let id = pass2d_query.p0_mut().spawn();
//             node_query.get_unchecked_mut_by_entity(e.id).write(Pass2DId(id));
//             let mut node_id = pass2d_query.p2_mut().get_unchecked_mut(id);
//             node_id.write(NodeId(unsafe { Id::<Node>::new(e.id.local()) }));
//         }
//         EventType::Delete => {
//             let id = node_query.get_unchecked_mut_by_entity(e.id);
//             pass2d_query.p1_mut().despawn(id.get().unwrap().0);
//         }
//         _ => (),
//     }
// }


// /// 递归设置节点的上下文
// fn recursive_set_node_context(
//     head: Entity,
//     idtree: &EntityTree,
//     down: &Query<&Down>,
//     in_context: &mut Query<(&'static mut InPassId, Option<&Camera>, &RenderContextMark)>,
//     parent_pass_id: &mut Query<&'static mut ParentPassId>,
//     parent_context_id: EntityKey,
// ) {
//     if EntityKey(head).is_null() {
//         return;
//     }
//     // 递归设置子节点
//     for node in idtree.iter(head) {
//         if let Ok((mut in_pass_id, _, mark)) = in_context.get_mut(node) {
//             if mark.any() {
//                 // 如果存在context_mark, 设置其对应的pass的parent_id
//                 if let Ok(mut parent_pass_id) = parent_pass_id.get_mut(***in_pass_id) {
//                     parent_pass_id.0 = parent_context_id;
//                 }
//                 // 如果存在context_mark,则返回，将在接下来的迭代中处理
//                 continue;
//             } else {
//                 in_pass_id.0 = parent_context_id;
//             }
//         }

//         let children_item = match down.get(node) {
//             Ok(r) => r,
//             _ => continue,
//         };

//         recursive_set_node_context(children_item.head(), idtree, down, in_context, parent_pass_id, parent_context_id);
//     }
// }

pub fn context_attr_del<T: Component>(
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
