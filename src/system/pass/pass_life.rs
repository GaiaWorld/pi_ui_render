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
use nalgebra::Point2;
use pi_style::style::Aabb2;
use pi_world::{prelude::{Alter, Changed, Entity, Has, Mut, ParamSet, Query, Removed, SingleRes, SingleResMut}, system_params::Local};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes, Up, Layer, LayerDirty};

use pi_null::Null;

use crate::{
    components::{
        calc::{ContentBox, EntityKey, InPassId, NeedMark, OverflowDesc, RenderContextMark, View, WorldMatrix}, draw_obj::InstanceIndex, pass_2d::{Camera, ChildrenPass, ParentPassId, PostProcessInfo}, PassBundle
    }, resource::{draw_obj::InstanceContext, EffectRenderContextMark, RenderContextMarkType}, shader1::meterial::{BoxUniform, QuadUniform, RenderFlagType, TyUniform}, system::draw_obj::{calc_text::IsRun, set_box}
};

/// 记录RenderContext添加和删除的脏，同时记录节点添加到树上的脏
/// 根据脏，从父向子递归，设置节点所在的渲染上下文（节点的渲染目标）
pub fn cal_context(
    // mut command: Commands,
    // mut layer_dirty: Local<LayerDirty<Entity>>,
    mut context_mark1: ParamSet<(
        Query<(Entity, &RenderContextMark, Option<&Camera>), Changed<RenderContextMark>>,
        Alter<(
            &mut InPassId,
            &RenderContextMark,
            Option<&mut ParentPassId>,
            Option<&mut PostProcessInfo>,
        ), (), PassBundle, ()>,
        Query<&mut InPassId>,
    )>,
    // idtree: EntityTree,
    // down: Query<&Down>,
    up: Query<&Up>,
    // mut parent_pass_id: Query<&'static mut ParentPassId>,
    // mut event_reader: EventReader<ComponentEvent<Changed<RenderContextMark>>>,
    // mut event_writer: EventWriter<ComponentEvent<Changed<ParentPassId>>>,
    // mut mark_change: Query<Entity, Changed<RenderContextMark>>,
    mut layer_dirty: LayerDirty<Changed<Layer>>,
    effect_mark: SingleRes<EffectRenderContextMark>,
    // mut layer_change: EventReader<ComponentEvent<Changed<Layer>>>,
	r: OrInitSingleRes<IsRun>,

    mut del: Local<Vec<Entity>>,
) {
	if r.0 {
		return;
	}
    // layer_dirty.clear();
    // let mut pass_2d_init = Vec::new();
    // let mut pass_2d_id_insert = Vec::new();


    // 如果mark修改，加入层脏
    for (entity, mark, camera) in context_mark1.p0().iter() {
        log::trace!("pass_life========================{:?}", entity);
        if camera.is_some() && mark.not_any() {
            // 删除pass
            layer_dirty.mark(entity);
        } else if camera.is_none() && mark.any() {
            // 不存在对应的pass2D， 则创建(放入层脏，按层创建)
            layer_dirty.mark(entity);
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

        let p1 = context_mark1.p1();
        if let Ok((mut in_pass_id, mark, parent_pass_id, post_info)) = p1.get_mut(node) {
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
                del.push(node);
                // 删除后，其子节点的in_pass_id修改为parent_context_id
                parent_context_id.0
            } else if mark.any() {
                // 修改in_pass_id为当前Pass2D
                *in_pass_id = InPassId(EntityKey(node));
                //  post_info
                log::trace!("pass======node: {:?}, parent_pass_id: {:?}, parent_context_id: {:?}, effect_mark{:?} {:?}, {:?}", node, parent_pass_id,  parent_context_id, **effect_mark & **mark, mark, **effect_mark);
                match parent_pass_id {
                    None => {
                        println!("PassBundle======={:?}", (node));
                        let mut bundle = PassBundle::new(*parent_context_id);
                        bundle.post_list_info.effect_mark = bundle.post_list_info.effect_mark | (**effect_mark & **mark);
                        let _ = p1.alter(node, bundle);
                        // 父的
                        // event_writer.send(ComponentEvent::new(node));
                    }
                    Some(mut parent_pass_id) => {
                        if ***parent_pass_id != *parent_context_id {
                            **parent_pass_id = parent_context_id;
                            // event_writer.send(ComponentEvent::new(node));
                        }

                        if let Some(mut info) = post_info {
                            let effect = **effect_mark & **mark;
                            info.effect_mark = info.effect_mark & **mark | effect;
                        }
                    }
                };
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

    // // 批量设置插入指令（PassBundle）
    // if pass_2d_init.len() > 0 {
    //     command.insert_or_spawn_batch(pass_2d_init.into_iter());
    // }
}

/// Pass2D设置children
pub fn calc_pass_children_and_clear(
    // mut event_reader: EventReader<ComponentEvent<Changed<RenderContextMark>>>,
    query_mark: Query<&RenderContextMark, Changed<RenderContextMark>>,
    mut query: ParamSet<(
		Query<&mut ChildrenPass>,
        Query<(&mut ChildrenPass, Entity)>,
	)>,
    query_pass: Query<(Entity, &ParentPassId)>,
    // mut query_root: Query<&mut RootInstance>,
    // mut temp: Local<(Vec<Entity>, Vec<Entity>)>,
    mut instances: SingleResMut<InstanceContext>,
	r: OrInitSingleRes<IsRun>
) {
    
	if r.0 {
		return;
	}
    
    if !query_mark.iter().next().is_some() {
        return;
    }
    log::trace!("calc_pass_children_and_clear===================");
    
    // 先清理旧的子节点
    let query_children = query.p0();
    for mut children in query_children.iter_mut() {
        children.clear();
    }

    // 重新组织渲染上下文的树
    for (entity, parent) in query_pass.iter() {
        if parent.0.is_null() {
            continue;
        }
        if let Ok(mut children) = query_children.get_mut(*parent.0) {
            children.push(EntityKey(entity));
        }
    }

    // 找到叶子节点
    for (mut children, entity) in query.p1().iter_mut() {
        // if let Ok(mut root_instance) = query_root.get_mut(layer.root()) {
            if children.len() == 0 {
                instances.temp.0.push(entity);
            }
            children.temp_count = children.len();
        // }
    }
}


// 
pub fn calc_pass_toop_sort(
    query_mark: Query<&RenderContextMark, Changed<RenderContextMark>>,
    mut query_children: Query<&mut ChildrenPass>,
    query_pass: Query<(Entity, &ParentPassId, &PostProcessInfo)>,
    // mut query_root: Query<&mut RootInstance>,
    mut instances: SingleResMut<InstanceContext>,
	r: OrInitSingleRes<IsRun>
) {
    if r.0 {
		return;
	}
    if !query_mark.iter().next().is_some() {
        return;
    }
    
    
    let InstanceContext {pass_toop_list,  next_node_with_depend, temp, ..} = &mut *instances;
    // 从叶子节点开始排序
    pass_toop_list.clear();
    next_node_with_depend.clear();
    log::trace!("calc_pass_toop_sort, temp_len:{:?}", temp.0.len());
    // for mut root_instance in query_root.iter_mut() {
        // root_instance.pass_toop_list.clear();
        // root_instance.next_node_with_depend.clear();
        // let root_instance = root_instance.bypass_change_detection();
    while temp.0.len() > 0 {
        for entity in temp.0.drain(..) {
            if let Ok((_, parent, post_info)) = query_pass.get(entity) {
                if let Ok(mut children) = query_children.get_mut(*parent.0) {
                    children.temp_count -= 1;
                    children.temp_has_effect |= post_info.has_effect();
                    if children.temp_count == 0 {
                        if !children.temp_has_effect {
                            temp.1.push(*parent.0);
                        } else {
                            temp.2.push(*parent.0);
                        }
                        children.temp_has_effect = false;
                    }
                }
            }
            pass_toop_list.push(entity);
        }
        if temp.1.len() > 0 {
            std::mem::swap(&mut temp.0, &mut temp.1);
        } else {
            let l = pass_toop_list.len();
            next_node_with_depend.push(l); // 下一个存在依赖的节点在toop排序中的索引
            std::mem::swap(&mut temp.0, &mut temp.2);
        } 
    }

    temp.0.clear();
    temp.1.clear();
    temp.2.clear();
}


/// 标记RenderContextMark
/// Opacity、Blur、Hsi等属性，需要标记RenderContextMark
/// RenderContextMark中的位标记不全为0时，后续阶段后将该节点设置为Pass节点（添加PassBundle）
pub fn pass_mark<T: NeedMark + Send + Sync>(
    mut query_set: ParamSet<(
        Query<(Entity, &T, &mut RenderContextMark), Changed<T>>,
        Query<(&'static mut RenderContextMark, Has<T>), Removed<T>>,
    )>,
    // del: RemovedComponents<T>,
    // render_mark: Query<Write<>>,
    mark_type: OrInitSingleRes<RenderContextMarkType<T>>,

    // mut event_writer: EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // let mut render_context = query_set.p1();
    // 组件删除，取消渲染上下文标记
    context_attr_del(query_set.p1(), ***mark_type);
    // println!("pass_mark!!!!!!, {:?}", std::any::type_name::<T>());

    for (entity, value, mut render_mark_value) in query_set.p0().iter_mut() {
        if value.need_mark() {
            log::debug!("pass_mark_true,{:?}, {:?}", entity, std::any::type_name::<T>());
            render_mark_true( ***mark_type, &mut render_mark_value);
        } else {
			log::debug!("pass_mark_false,{:?}, {:?}", entity, std::any::type_name::<T>());
            render_mark_false( ***mark_type, &mut render_mark_value);
        }
    }
}

/// 为Pass设置渲染实例数据（将后处理结果拷贝到gui上）
pub fn calc_pass(
	mut instances: OrInitSingleResMut<InstanceContext>,
	query: Query<
		(
            &InstanceIndex,
            &ParentPassId,
            &Camera,
            &View,
		),
		(Changed<PostProcessInfo>, Changed<WorldMatrix>, Changed<ContentBox>),
	>,
    query1: Query<
		(
            &ParentPassId,
            &Camera,
            &PostProcessInfo,
		),
	>,
	r: OrInitSingleRes<IsRun>,
) {
    if r.0 {
		return;
	}

    for (instance_index, mut parent_pass_id, camera, overflow_aabb) in query.iter() {
		// 节点可能设置为dispaly none， 此时instance_index可能为Null
        // 节点可能没有后处理效果， 此时instance_index为Null
        if pi_null::Null::is_null(&instance_index.0.start) {
            continue;
        }

        log::debug!("set pass instance data, parent_pass_id={:?},  instance_index={:?}", parent_pass_id, instance_index);
        
        let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
        let mut render_flag = instance_data.get_render_ty();
        render_flag |= 1 << RenderFlagType::Uv as usize;
        render_flag |= 1 << RenderFlagType::Premulti as usize;
        // instance_data.set_data(&BoxUniform(&[p1.x, p1.y, p2.x - p1.x, p2.y - p1.y]));
        if parent_pass_id.0.is_null() {
            // 如果是根节点， 渲染时设置的投影矩阵和视图矩阵都是单位阵
            instance_data.set_data(&BoxUniform(&[0.0, 0.0, 1.0, 1.0]));
            instance_data.set_data(&QuadUniform(&[
                -1.0, 1.0,
                -1.0, -1.0,
                1.0, -1.0,
                1.0, 1.0,
            ]));
        } else {
            render_flag |= 1 << RenderFlagType::Fbo as usize;
            // if content_box.layout.width() >= 700.0 && content_box.layout.height() >= 910.0 {
                // println!("right_bottom.x >= 788, {:?}, \n{:?}", (entity, post_info.has_effect(), content_box.layout.width(), content_box.layout.height()), world_matrix);
            // }
            
            // let aabb = &camera.view_port;
            if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
                // 注意， 此处设置的BoxUniform并不正确， TODO
                set_box(&matrix.from_context_rotate, &camera.view_port, &mut instance_data);
            } else {
                // if bg.is_some() {
                	// log::warn!("aaaa================={:?}, {:?}", entity, &camera.view_port);
                // }
                // post_info.matrix = WorldMatrix(world_matrix, false);
                
                let mut view_port = &camera.view_port;
                let t;
                while let Ok((p, p_camera, post, )) = query1.get(***parent_pass_id) {
                    if post.has_effect() {
                        let min = Point2::new(
                            p_camera.view_port.mins.x + (camera.view_port.mins.x - p_camera.view_port.mins.x),
                            p_camera.view_port.mins.y + (camera.view_port.mins.y - p_camera.view_port.mins.y)
                        );
                        let max = Point2::new(
                            min.x + camera.view_port.maxs.x - camera.view_port.mins.x,
                            min.y + camera.view_port.maxs.y - camera.view_port.mins.y,
                        );
                        t = Aabb2::new(min, max) ;
                        view_port = &t;
                        break;
                    }
                    parent_pass_id = p;
                }

                // println!("calc_pass!!!!!!==={:?}", (instance_index,  parent_pass_id, view_port));

                instance_data.set_data(&QuadUniform(&[
                    view_port.mins.x, view_port.mins.y,
                    view_port.mins.x, view_port.maxs.y,
                    view_port.maxs.x, view_port.maxs.y,
                    view_port.maxs.x, view_port.mins.y,
                ]));
                // if instance_index.start == 480 {
                //     instance_data.set_data(&QuadUniform(&[
                //         100.0, 200.0,
                //         100.0, 300.0,
                //         150.0, 300.0,
                //         150.0, 200.0,
                //     ]));
                // } 
                
            }

            // // 存在旋转，需要旋转回父上下文
            // if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
            //     post_info.matrix = WorldMatrix(&matrix.from_context_rotate * world_matrix, true);
            // } else {
            //     // if bg.is_some() {
            //     // 	log::warn!("aaaa================={:?}, {:?}, {:?}", entity, aabb, world_matrix);
            //     // }
            //     post_info.matrix = WorldMatrix(world_matrix, false);
            // }

            // let layout_rect = Aabb2::new(Point2::new(0.0, 0.0), Point2::new(content_box.layout.width(), content_box.layout.height()));
            // instance_data.set_data(&BoxUniform(&[layout_rect.mins.x, layout_rect.mins.y, layout_rect.maxs.x - layout_rect.mins.x, layout_rect.maxs.y - layout_rect.mins.y]));
            // instance_data.set_data(&QuadUniform(&[
            //     left_top.x, left_top.y,
            //     left_bottom.x, left_bottom.y,
            //     right_bottom.x, right_bottom.y,
            //     right_top.x, right_top.y,
            // ]));
            // set_box(&world_matrix, &Aabb2::new(Point2::new(0.0, 0.0), Point2::new(content_box.layout.width(), content_box.layout.height())), &mut instance_data);
        }
        instance_data.set_data(&TyUniform(&[render_flag as f32]));
	}
}

fn context_attr_del<T>(
    dels: &mut Query<(&'static mut RenderContextMark, Has<T>), Removed<T>>,
    mark_type: usize,
    // event_writer: &mut EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
    // render_context: &mut Query<&'static mut RenderContextMark>,
) {
    // Opacity组件删除，取消渲染上下文标记
    for (mut render_mark_value, has_t) in dels.iter_mut() {
        if has_t {
            continue;
        }
        unsafe { render_mark_value.replace_unchecked(mark_type, false) };
        // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
        // log::debug!("pass_mark_del,{:?}, {:?}", del, std::any::type_name::<T>());
        // if unsafe { render_mark_value.replace_unchecked(mark_type, false) } {
        //     // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
        //     log::debug!("pass_mark_del,{:?}, {:?}", del, std::any::type_name::<T>());
        //     event_writer.send(ComponentEvent::new(del));
        // }
    }
}


#[inline]
pub fn render_mark_true(
    mark_type: usize,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    unsafe { render_mark_value.replace_unchecked(mark_type, true) };
}

#[inline]
pub fn render_mark_false(
    mark_type: usize,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    unsafe { render_mark_value.replace_unchecked(mark_type, false) };
}
