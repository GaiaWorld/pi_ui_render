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
use std::collections::VecDeque;

use pi_bevy_render_plugin::{render_cross::GraphId, NodeId, PiRenderGraph};
use pi_slotmap::SecondaryMap;
use pi_style::style::Aabb2;
use pi_world::{event::{ComponentAdded, ComponentChanged}, fetch::{OrDefault, Ticker}, filter::{Or, With}, prelude::{Alter, Changed, Entity, Mut, ParamSet, Query, SingleRes, SingleResMut}, system_params::Local};
use pi_bevy_ecs_extend::prelude::{Layer, LayerDirty, OrInitSingleRes, OrInitSingleResMut, Root, Up};

use pi_null::Null;

use crate::{
    components::{
        calc::{style_bit, ContentBox, EntityKey, InPassId, NeedMark, OverflowDesc, RenderContextMark, StyleBit, StyleMarkType, TransformWillChangeMatrix, View, WorldMatrix}, draw_obj::InstanceIndex, pass_2d::{Camera, ChildrenPass, ParentPassId, PostProcessInfo}, user::{Point2, Size}, PassBundle
    }, resource::{draw_obj::InstanceContext, EffectRenderContextMark, GlobalDirtyMark, IsRun, OtherDirtyType, RenderContextMarkType}, shader1::batch_meterial::{RenderFlagType, TyMeterial}, system::{base::draw_obj::image_texture_load::AsImageBindList, draw_obj::set_matrix}
};



// pub fn text_layout_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
// 	mark.mark.has_any(&*TEXT_LAYOUT_DIRTY)
// }

/// 根据RenderContext脏，创建或删除Pass2D）
pub fn cal_pass_life(
    mut context_mark1: ParamSet<(
        Alter<(&RenderContextMark, Option<&ParentPassId>), (), PassBundle, ()>, 
        Alter<(), With<PostProcessInfo>, (), PassBundle>,
    )>,
    mark_changed: ComponentChanged<RenderContextMark>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}

    log::debug!("cal_pass_life==========={:?}", mark_changed.len());
    // let cal_context_span = tracing::info_span!("cal_context");
    // 如果mark修改，加入层脏
    for entity in mark_changed.iter() {
        let (mark, parent_pass) = context_mark1.p0().get_mut(*entity).unwrap();
        if mark.any() {
            if parent_pass.is_none() {
                log::debug!("create pass==========={:?}", entity);
                // 添加pass
                let bundle = PassBundle::new(Default::default());
                let _ = context_mark1.p0().alter(*entity, bundle);
            }
        } else {
            // 删除pass
            log::debug!("del pass==========={:?}", entity);
            let _ = context_mark1.p1().alter(*entity,  ());
        }
    }
}

/// 设置parentPassId和InPassId
pub fn cal_context(
    // mut command: Commands,
    // mut layer_dirty: Local<LayerDirty<Entity>>,
    mut context_mark1: ParamSet<(
        Query<(Entity, &RenderContextMark, Option<&PostProcessInfo>), Changed<RenderContextMark>>,
        Query<(
            &mut InPassId,
            &RenderContextMark,
            Option<&mut ParentPassId>,
            Option<&mut PostProcessInfo>,
        )>, 
        Query<&mut InPassId>,
        Query<(Ticker<&RenderContextMark>, OrDefault<ParentPassId>)>,
    )>,
    up: Query<&Up>,
    mut layer_dirty: LayerDirty<(With<RenderContextMark>, Changed<Layer>)>,
    // dirty_list: Event<StyleChange>,
    mark_changed: ComponentChanged<RenderContextMark>,
    
    effect_mark: SingleRes<EffectRenderContextMark>,
    mut global_mark: SingleResMut<GlobalDirtyMark>,
    // mut layer_change: EventReader<ComponentEvent<Changed<Layer>>>,
	r: OrInitSingleRes<IsRun>,
    // mut node_change: OrInitSingleResMut<NodeChanged>,

    // mut del: Local<Vec<Entity>>,
) {
	if r.0 {
		return;
	}

    let mut pass_life_change = false;
    log::debug!("cal_context==========={:?}", mark_changed.len());
    // 如果mark修改，加入层脏
    if mark_changed.len() > 0 {
        let p3 = context_mark1.p3();
        for entity in mark_changed.iter() {
            let entity = *entity;
            if let Ok((mark, parent_pass_id)) = p3.get(entity) { 
                if mark.not_any() {
                    pass_life_change = true;
                    log::debug!("pass_life del========================{:?}", entity);
                    // 删除pass
                    layer_dirty.mark(entity);
                } else if parent_pass_id.0.is_null() && mark.any() {
                    pass_life_change = true;
                    log::debug!("pass_life add========================{:?}", entity);
                    // 不存在对应的pass2D， 则创建(放入层脏，按层创建)
                    layer_dirty.mark(entity);
                }
            }
        }
    }

    if pass_life_change { 
        global_mark.mark.set(OtherDirtyType::PassLife as usize, true);
        log::debug!("pass_life_change==================={:?}", (OtherDirtyType::PassLife as usize, &global_mark.mark, global_mark.mark.get(OtherDirtyType::PassLife as usize)));
    }
    log::debug!("cal_context2==========={:?}", mark_changed.len());
    

    // 按层迭代
    if layer_dirty.count() > 0 {
        for node in layer_dirty.iter() {
            let parent_context_id = match up.get(node) {
                Ok(r) if let Ok((in_pass_id, _, _, _)) = context_mark1.p1().get(r.parent()) => **in_pass_id, // TODO
                _ => {
                    // log::warn!("null in pass=========={:?}, up = {:?}", node, up.get(node));
                    EntityKey::null()
                },
            };

            log::debug!("pass!!!======node: {:?}, has: {:?}, up: {:?}", node, context_mark1.p1().get_mut(node).is_ok(), up.get(node));
            if let Ok((mut in_pass_id, mark, parent_pass_id, post_info)) = context_mark1.p1().get_mut(node) {
                // mark已清空，但相机依然存在，则删除pass, 重新设置pass字节点的in_pass_id
                log::debug!("pass0======node: {:?}, {:?}, {:?}", node, mark.not_any(), parent_pass_id);
                if mark.not_any() {
                    log::debug!("del pass0======node: {:?}, {:?}", node, parent_pass_id);
                    // 修改in_pass_id为父的Pass2D
                    *in_pass_id = InPassId(parent_context_id);
                } else {
                    // 修改in_pass_id为当前Pass2D
                    *in_pass_id = InPassId(EntityKey(node));
                    //  post_info
                    log::debug!("pass======node: {:?}, parent_pass_id: {:?}, parent_context_id: {:?}, effect_mark{:?} {:?}, {:?}", node, parent_pass_id,  parent_context_id, **effect_mark & **mark, mark, **effect_mark);
                    match parent_pass_id {
                        Some(mut parent_pass_id) => {
                            if ***parent_pass_id != *parent_context_id {
                                **parent_pass_id = parent_context_id;
                                // event_writer.send(ComponentEvent::new(node));
                            }

                            if let Some(mut info) = post_info {
                                let effect = **effect_mark & **mark;
                                info.effect_mark = info.effect_mark & **mark | effect;
                            }
                        },
                        _ => unreachable!()
                    };
                };
            }
        }
    }
}

lazy_static! {
	pub static ref PASS_LIFE_DIRTY: StyleMarkType = style_bit()
		.set_bit(OtherDirtyType::NodeTreeAdd as usize);
    pub static ref PASS_CHILDREN_DIRTY: StyleMarkType = style_bit()
		.set_bit(OtherDirtyType::NodeTreeAdd as usize)
        .set_bit(OtherDirtyType::NodeTreeDel as usize)
        .set_bit(OtherDirtyType::NodeTreeRemove as usize);
}

pub fn pass_life_change(mark: SingleRes<GlobalDirtyMark>, changed: ComponentChanged<RenderContextMark>) -> bool {
	let r = mark.mark.has_any(&*PASS_LIFE_DIRTY) || changed.len() > 0;
    changed.mark_read();
    // log::warn!("pass_life_change============{:?}", r);
    r
}

pub fn pass_life_children(mark: SingleRes<GlobalDirtyMark>, changed: ComponentChanged<RenderContextMark>) -> bool {
	let r = mark.mark.has_any(&*PASS_CHILDREN_DIRTY) || changed.len() > 0;
    changed.mark_read();
    r
}

/// Pass2D设置children
pub fn calc_pass_children_and_clear(
    mut query: ParamSet<(
		Query<&mut ChildrenPass>,
        Query<(&mut ChildrenPass, Entity)>,
	)>,
    query_pass: Query<(Entity, &ParentPassId)>,
    // mut query_root: Query<&mut RootInstance>,
    // mut temp: Local<(Vec<Entity>, Vec<Entity>)>,
	r: OrInitSingleRes<IsRun>
) {
    
	if r.0 {
		return;
	}
    log::debug!("calc_pass_children_and_clear===================");
    
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
}

// 对gui中的Pass进行拓扑排序
// RenderContextMark修改， AsImageBindList修改或添加、Camera删除，都需要冲洗计算toop排序
pub fn calc_pass_toop_sort(
    query_root: Query<&GraphId, (With<Root>, With<Size>)>,
    query_graph_id: Query<&GraphId>,
    query_post: Query<&PostProcessInfo, With<Size>>,
    mut instances: SingleResMut<InstanceContext>,
    mut rg: SingleResMut<PiRenderGraph>,
	r: OrInitSingleRes<IsRun>,
    mut temp: Local<(Vec<NodeId>, Vec<NodeId>, Vec<NodeId>, SecondaryMap<NodeId, (usize, bool)>)>,

    mark_change: ComponentChanged<RenderContextMark>,
    as_image_changed: ComponentChanged<AsImageBindList>,
    as_image_added: ComponentAdded<AsImageBindList>,

    mark: SingleRes<GlobalDirtyMark>,

) {
    if r.0 {
		return;
	}

    if !mark.mark.has_any(&*PASS_CHILDREN_DIRTY) && mark_change.len() == 0 && as_image_changed.len() == 0 && as_image_added.len() == 0 {
        return;
    }
    mark_change.mark_read();
    as_image_changed.mark_read();
    as_image_added.mark_read();

    let temp = &mut *temp;
    let rg = &mut *rg;
    
    let InstanceContext {pass_toop_list,  next_node_with_depend, ..} = &mut *instances;
    // 从叶子节点开始排序
    pass_toop_list.clear();
    next_node_with_depend.clear();
    // log::debug!("calc_pass_toop_sort, temp_len:{:?}", temp.0.len());
    
    let mut temp_before = VecDeque::new();
    for i in query_root.iter() {
        temp.1.push(i.0.clone());
    }
    temp_before.push_back(&temp.1[0..temp.1.len()]);
    log::debug!("temp_before======{:?}", &temp_before);

    // 遍历所有节点，在temp.0中放入所有的叶子节点，并将每个节点的前置依赖数量放入temp.3中缓存
    loop  {
        let node_ids = match  temp_before.pop_front() {
            Some(node_ids) => node_ids,
            None => break,
        };
        log::debug!("temp_before1======{:?}", &node_ids);
        for node_id in node_ids {
            match temp.3.get_mut(node_id.clone()) {
                Some(_count) => continue, // 存在索引，表示已经迭代过了， 不需要处理 
                None => {
                    let before = rg.before_nodes(node_id.clone()).unwrap();
                    log::debug!("temp_before2======node_id:{:?}, before: {:?}", node_id, &before);
                    temp.3.insert(node_id.clone(), (before.len(), false));
                    if before.len() > 0 {
                        temp_before.push_back(before);
                    } else {
                        temp.0.push(node_id.clone()); // 如果没有后续节点， 则加入当前列表(叶子节点列表)
                    }
                },
            };
        }
    }
    temp.1.clear();

    let mut set_depend = |pass_toop_list: &Vec<Entity>, last_effect: usize, last_depend: &mut usize, rg: &mut PiRenderGraph| {
        if last_effect != *last_depend {
            let last_effect_index = last_effect - 1;
            for i in *last_depend .. pass_toop_list.len() {
                if let Ok(graph_id) = query_graph_id.get(pass_toop_list[i]) {
                    let r = if last_effect_index != i {
                        false
                    } else {
                        true
                    };
                    let _ = rg.set_is_run(graph_id.0, r);
                    log::debug!("set_is_run==============={:?}", (pass_toop_list[i], r));
                }
            }
            next_node_with_depend.push(last_effect); // 下一个存在依赖的节点在toop排序中的索引
            *last_depend = last_effect;
        }
    };


    let mut last_depend = 0;
    let mut last_effect = 0;
    while temp.0.len() > 0 { // 循环开始时， temp.0是所有的pass叶子节点
      
        log::debug!("after!!!!======{:?}", &temp.0);
        for node_id in temp.0.drain(..) {
            let entity = rg.get_bind(node_id.clone());
            log::debug!("after1======{:?}", (node_id, entity));
            let mut has_effect = false;
            let mut is_gui = false;
            if query_post.contains(entity) { // 对应节点为gui节点
                pass_toop_list.push(entity); // 加入到pass_toop_list
                if let Ok(post_info) = query_post.get(entity) {
                    has_effect = post_info.has_effect();
                    if has_effect {
                        last_effect = pass_toop_list.len();
                    }
                    
                    log::debug!("after2======has_effect{:?}", (entity, node_id, last_effect, has_effect));
                }
                is_gui = true;
                
            } else {
                // log::warn!("zzzz======================{:?}", (entity, pass_toop_list.len()));
                // next_node_with_depend.push(pass_toop_list.len()); // 下一个存在依赖的节点在toop排序中的索引
                // last_depend = pass_toop_list.len();
                has_effect = true; // 不是gui节点， 统统当做是有特效节点
            };

            let after = rg.after_nodes(node_id).unwrap(); //
            let mut is_gui_to_other = false;
            log::debug!("after3======{:?}", after);
            for node_id in after {
                if let Some((count, before_has_effect)) = temp.3.get_mut(node_id.clone()) {
                    if is_gui {
                        let entity = rg.get_bind(node_id.clone());
                        if !query_post.contains(entity) { 
                            // 存在一个gui到其他系统的的链接
                            is_gui_to_other = true;
                        }
                    }
                    

                    *count = *count - 1;
                    *before_has_effect |= has_effect;
                    log::debug!("after4======{:?}", (node_id, *count));
                    if *count == 0 { // 依赖已经分析完毕
                        if *before_has_effect { // 前置节点存在fbo依赖
                            temp.1.push(node_id.clone());
                        } else { // 前置节点不存在fbo依赖
                            temp.2.push(node_id.clone());
                        }
                    }
                }
            }

            // 如果当前节点是gui节点， 并连接到gui之外的系统， 立即设置下一个依赖索引
            // 因为gui的图渲染不是每pass节点渲染自身内容，而是下个依赖之前的图节点中， 跟其他无依赖的pass节点批量清屏，在逐个渲染， 再逐个后处理
            // 由于无法分析其他系统的图节点的组织方式， 唯一明确的是， gui与外部系统相连， gui必然将渲染结果输出到后续的外部系统节点
            // 这里立即设置下个依赖， 保证当前pass的图节点运行后，其渲染结果也已经呈现， 使得外部节点能正常渲染
            if is_gui_to_other {
                set_depend(pass_toop_list, last_effect, &mut last_depend, rg); 
            }
        }

        log::debug!("next======================{:?}", (&temp.1, &temp.2));
        if temp.2.len() > 0 { // 非fbo节点
            // log::warn!("2222======================{:?}", (&temp.2));
            std::mem::swap(&mut temp.0, &mut temp.2);
        } else {
            
            if temp.1.len() > 0 {
                std::mem::swap(&mut temp.0, &mut temp.1);
            }
            set_depend(pass_toop_list, last_effect, &mut last_depend, rg);
        } 
    }

    log::debug!("111======================{:?}, \n{:?}", pass_toop_list, next_node_with_depend.iter().map(|i| {pass_toop_list[i - 1]}).collect::<Vec<Entity>>());

    temp.0.clear();
    temp.1.clear();
    temp.2.clear();
    temp.3.clear();
    // log::warn!("2222======================{:?}, {:?}", pass_toop_list, next_node_with_depend);
}


// // 
// pub fn calc_pass_toop_sort(
//     // query_mark: Query<&RenderContextMark, Changed<RenderContextMark>>,
//     mut query_children: Query<&mut ChildrenPass>,
//     query_pass: Query<(Entity, &ParentPassId, &PostProcessInfo)>,
//     // mut query_root: Query<&mut RootInstance>,
//     mut instances: SingleResMut<InstanceContext>,
// 	r: OrInitSingleRes<IsRun>
// ) {
//     if r.0 {
// 		return;
// 	}
    
//     let InstanceContext {pass_toop_list,  next_node_with_depend, temp, ..} = &mut *instances;
//     // 从叶子节点开始排序
//     pass_toop_list.clear();
//     next_node_with_depend.clear();
//     log::debug!("calc_pass_toop_sort, temp_len:{:?}", temp.0.len());
//     // for mut root_instance in query_root.iter_mut() {
//         // root_instance.pass_toop_list.clear();
//         // root_instance.next_node_with_depend.clear();
//         // let root_instance = root_instance.bypass_change_detection();
//     while temp.0.len() > 0 { // 循环开始时， temp.0是所有的pass叶子节点
//         for entity in temp.0.drain(..) {
//             if let Ok((_, parent, post_info)) = query_pass.get(entity) {
//                 if let Ok(mut children) = query_children.get_mut(*parent.0) {
//                     children.temp_count -= 1; // temp_count的初值为子pass数量
//                     children.temp_has_effect |= post_info.has_effect();
//                     if children.temp_count == 0 {
//                         if !children.temp_has_effect {
//                             temp.1.push(*parent.0);
//                         } else {
//                             temp.2.push(*parent.0);
//                         }
//                         children.temp_has_effect = false;
//                     }
//                 }
//             }
//             pass_toop_list.push(entity);
//         }
//         if temp.1.len() > 0 {
//             std::mem::swap(&mut temp.0, &mut temp.1);
//         } else {
//             let l = pass_toop_list.len();
//             next_node_with_depend.push(l); // 下一个存在依赖的节点在toop排序中的索引
//             std::mem::swap(&mut temp.0, &mut temp.2);
//         } 
//     }

//     temp.0.clear();
//     temp.1.clear();
//     temp.2.clear();
// }


/// 标记RenderContextMark
/// Opacity、Blur、Hsi等属性，需要标记RenderContextMark
/// RenderContextMark中的位标记不全为0时，后续阶段后将该节点设置为Pass节点（添加PassBundle）
/// 此system不处理删除T的情况， 不允许外部删除T， 通常应该设置为默认值来代表删除行为
pub fn pass_mark<T: NeedMark + Send + Sync>(
    // mut query_set: ParamSet<(
    //     Query<(Entity, &T, &mut RenderContextMark), Changed<T>>,
    //     Query<(&'static mut RenderContextMark, Has<T>)>,
    // )>,
    mut query: Query<(Entity, &T, &mut RenderContextMark), Changed<T>>,
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
    // context_attr_del(query_set.p1(), &mut removed, ***mark_type);
    for (entity, value, mut render_mark_value) in query.iter_mut() {
        if value.need_mark() {
            render_mark_true( ***mark_type, &mut render_mark_value);
            // if std::any::type_name::<T>().contains("AsImage") {
                log::debug!("pass_mark_true,{:?}, {:?}", entity, std::any::type_name::<T>());
            // }
           
        } else {
            render_mark_false( ***mark_type, &mut render_mark_value);
            // if std::any::type_name::<T>().contains("AsImage") {
                log::debug!("pass_mark_false,{:?}, {:?}", entity, std::any::type_name::<T>());
            // }
        }
    }
}

/// 为Pass设置渲染实例数据（将后处理结果拷贝到gui上）
pub fn calc_pass(
	mut instances: OrInitSingleResMut<InstanceContext>,
	query: Query<( 
        &InstanceIndex, 
        &ParentPassId,
        &Camera,
        &View,
        &TransformWillChangeMatrix,
        // OrDefault<Overflow>,
        &crate::components::calc::LayoutResult,
        &ContentBox,
        Entity,
    ), Or<(Changed<PostProcessInfo>, Changed<ContentBox>, Changed<TransformWillChangeMatrix>, Changed<Camera>)>>,
    // query1: Query<
	// 	(
    //         &ParentPassId,
    //         &Camera,
    //         &PostProcessInfo,
	// 	),
	// >,
	r: OrInitSingleRes<IsRun>,
) {
    if r.0 {
		return;
	}
    for (instance_index, parent_pass_id, camera, view, will_change, layout, content_box, entity, ) in query.iter() {
        log::debug!("passs1==============={:?}", instance_index.0.start);
        // 节点可能设置为dispaly none， 此时instance_index可能为Null
        // 节点可能没有后处理效果， 此时instance_index为Null
        if pi_null::Null::is_null(&instance_index.0.start) {
            continue;
        }
        
        let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
        let mut render_flag = instance_data.get_render_ty();
        render_flag |= 1 << RenderFlagType::Uv as usize;
        render_flag |= 1 << RenderFlagType::Premulti as usize;

        if parent_pass_id.0.is_null() {
            // 如果是根节点， 渲染时设置的投影矩阵和视图矩阵都是单位阵
            // instance_data.set_data(&BoxUniform(&[0.0, 0.0, 1.0, 1.0]));
            // instance_data.set_data(&QuadUniform(&[
            //     -1.0, 1.0,
            //     -1.0, -1.0,
            //     1.0, -1.0,
            //     1.0, 1.0,
            // ]));
            // 0,0点在左上角
            set_matrix(&WorldMatrix::default(), &Aabb2::new(Point2::new(-1.0, 1.0), Point2::new(1.0, -1.0)), &mut instance_data);

            render_flag |= 1 << RenderFlagType::IgnoreCamera as usize;
            instance_data.set_data(&TyMeterial(&[render_flag as f32]));
        } else {
            // let (left, top, width, height) = if **overflow {
            //     // oveflow需要裁剪子节点到内容区域（注意，同时也将自身裁剪到内容区域，这与浏览器标准不符）
            //     (
            //         layout.border.left + layout.padding.left,
            //         layout.border.top + layout.padding.top,
            //         layout.rect.right - (layout.border.right + layout.padding.right) - layout.rect.left,
            //         layout.rect.bottom - (layout.border.top + layout.padding.top) - layout.rect.top,
            //     )
            // } else {
            //     // 如果子节点设有transform， 并且使得超出了本节点的布局范围会有问题（如何解决？TODO）
            //     (
            //         0.0,
            //         0.0,
            //         content_box.layout.maxs.x - content_box.layout.mins.x,
            //         content_box.layout.maxs.y - content_box.layout.mins.y,
            //     )
            // };
            // instance_data.set_data(&BoxUniform(&[left, top, width, height]));
            instance_data.set_data(&TyMeterial(&[render_flag as f32]));
            // 设置quad到世界位置
            match (&view.desc, &will_change.0) {
                (OverflowDesc::NoRotate(_), None) => {
                    set_matrix(&WorldMatrix::default(), &camera.view_port, &mut instance_data);
                    log::trace!("no rotate=================={:?}", (instance_index.start / 224, entity, &camera.view_port, layout, content_box));
                    // instance_data.set_data(&QuadUniform(&[
                    //     view_port.mins.x, view_port.mins.y,
                    //     view_port.mins.x, view_port.maxs.y,
                    //     view_port.maxs.x, view_port.maxs.y,
                    //     view_port.maxs.x, view_port.mins.y,
                    // ]));
                    continue;
                },
                (OverflowDesc::NoRotate(_), Some(will_change)) => {
                    set_matrix(&will_change.will_change_invert, &camera.view_port, &mut instance_data)
                },
                (OverflowDesc::Rotate(matrix), Some(will_change)) => {
                    set_matrix(&(&will_change.will_change_invert * &matrix.world_rotate), &camera.view_port, &mut instance_data);
                },
                (OverflowDesc::Rotate(matrix), None) => set_matrix(&matrix.world_rotate, &camera.view_port, &mut instance_data),
            }

            

            // if let OverflowDesc::Rotate(matrix) = &view.desc {
            //     set_box(&matrix.world_rotate, &camera.view_port, &mut instance_data);
            // } else {
            //     let view_port = &camera.view_port;
            //     instance_data.set_data(&QuadUniform(&[
            //         view_port.mins.x, view_port.mins.y,
            //         view_port.mins.x, view_port.maxs.y,
            //         view_port.maxs.x, view_port.maxs.y,
            //         view_port.maxs.x, view_port.mins.y,
            //     ]));
            // }
        }

        

        // let aabb_temp;
        // let view_world_aabb = match &overflow_aabb.desc {
        //     OverflowDesc::Rotate(r) => {
        //         aabb_temp = calc_bound_box(&no_rotate_view_aabb, &r.world_rotate);
        //         &aabb_temp
        //     }
        //     _ => &no_rotate_view_aabb,
        // };

       

        // // instance_data.set_data(&BoxUniform(&[p1.x, p1.y, p2.x - p1.x, p2.y - p1.y]));
        // if parent_pass_id.0.is_null() {
        //     // 如果是根节点， 渲染时设置的投影矩阵和视图矩阵都是单位阵
        //     instance_data.set_data(&BoxUniform(&[0.0, 0.0, 1.0, 1.0]));
        //     instance_data.set_data(&QuadUniform(&[
        //         -1.0, 1.0,
        //         -1.0, -1.0,
        //         1.0, -1.0,
        //         1.0, 1.0,
        //     ]));
        // } else {
        //     // let view_port = &camera.view_port;
        //     // instance_data.set_data(&BoxUniform(&[0.0, 0.0, 1.0, 1.0]));
        //     // instance_data.set_data(&QuadUniform(&[
        //     //     view_port.mins.x, view_port.mins.y,
        //     //     view_port.mins.x, view_port.maxs.y,
        //     //     view_port.maxs.x, view_port.maxs.y,
        //     //     view_port.maxs.x, view_port.mins.y,
        //     // ]));


        //     // if content_box.layout.width() >= 700.0 && content_box.layout.height() >= 910.0 {
        //         // println!("right_bottom.x >= 788, {:?}, \n{:?}", (entity, post_info.has_effect(), content_box.layout.width(), content_box.layout.height()), world_matrix);
        //     // }
        //     let aabb = &camera.view_port;
        //     let scale_x = (aabb.maxs.x - aabb.mins.x) / 2.0;
        //     let scale_y = (aabb.maxs.y - aabb.mins.y) / 2.0;
        //     // 后处理效果与gui坐标系使用不一致，所以缩放为-scale_y
        //     // 这里的aabb是指当前非旋转坐标系
        //     let quad = [
        //         Vector4::new(aabb.mins.x - scale_x, scale_y, 0.0, 0.0),
        //         Vector4::new(aabb.mins.x - scale_x, scale_y, 0.0, 0.0),
        //     ];
        //     Matrix4::new(
        //         scale_x,
        //         0.0,
        //         0.0,
        //         aabb.mins.x + scale_x,
        //         0.0,
        //         -scale_y,
        //         0.0,
        //         aabb.mins.y + scale_y,
        //         0.0,
        //         0.0,
        //         1.0,
        //         0.0,
        //         0.0,
        //         0.0,
        //         0.0,
        //         1.0,
        //     );

        //     // let aabb = &camera.view_port;
        //     if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
        //         // 注意， 此处设置的BoxUniform并不正确， TODO
        //         set_box(&matrix.from_context_rotate, &camera.view_port, &mut instance_data);
        //     } else {
        //         // if bg.is_some() {
        //         	// log::warn!("aaaa================={:?}, {:?}", entity, &camera.view_port);
        //         // }
        //         // post_info.matrix = WorldMatrix(world_matrix, false);
                
        //         let mut view_port = &camera.view_port;
        //         let t;
        //         while let Ok((p, p_camera, post, )) = query1.get(***parent_pass_id) {
        //             if post.has_effect() {
        //                 let min = Point2::new(
        //                     p_camera.view_port.mins.x + (camera.view_port.mins.x - p_camera.view_port.mins.x),
        //                     p_camera.view_port.mins.y + (camera.view_port.mins.y - p_camera.view_port.mins.y)
        //                 );
        //                 let max = Point2::new(
        //                     min.x + camera.view_port.maxs.x - camera.view_port.mins.x,
        //                     min.y + camera.view_port.maxs.y - camera.view_port.mins.y,
        //                 );
        //                 t = Aabb2::new(min, max) ;
        //                 view_port = &t;
        //                 break;
        //             }
        //             parent_pass_id = p;
        //         }

        //         // println!("calc_pass!!!!!!==={:?}", (instance_index,  parent_pass_id, view_port));

        //         instance_data.set_data(&QuadUniform(&[
        //             view_port.mins.x, view_port.mins.y,
        //             view_port.mins.x, view_port.maxs.y,
        //             view_port.maxs.x, view_port.maxs.y,
        //             view_port.maxs.x, view_port.mins.y,
        //         ]));
        //         // if instance_index.start == 480 {
        //         //     instance_data.set_data(&QuadUniform(&[
        //         //         100.0, 200.0,
        //         //         100.0, 300.0,
        //         //         150.0, 300.0,
        //         //         150.0, 200.0,
        //         //     ]));
        //         // } 
                
        //     }

        //     // 存在旋转，需要旋转回父上下文
        //     if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
        //         post_info.matrix = WorldMatrix(&matrix.from_context_rotate * world_matrix, true);
        //     } else {
        //         // if bg.is_some() {
        //         // 	log::warn!("aaaa================={:?}, {:?}, {:?}", entity, aabb, world_matrix);
        //         // }
        //         post_info.matrix = WorldMatrix(world_matrix, false);
        //     }

        //     let layout_rect = Aabb2::new(Point2::new(0.0, 0.0), Point2::new(content_box.layout.width(), content_box.layout.height()));
        //     instance_data.set_data(&BoxUniform(&[layout_rect.mins.x, layout_rect.mins.y, layout_rect.maxs.x - layout_rect.mins.x, layout_rect.maxs.y - layout_rect.mins.y]));
        //     instance_data.set_data(&QuadUniform(&[
        //         left_top.x, left_top.y,
        //         left_bottom.x, left_bottom.y,
        //         right_bottom.x, right_bottom.y,
        //         right_top.x, right_top.y,
        //     ]));
        //     set_box(&world_matrix, &Aabb2::new(Point2::new(0.0, 0.0), Point2::new(content_box.layout.width(), content_box.layout.height())), &mut instance_data);
        // }
       
	}
}

// fn context_attr_del<T>(
//     dels: &mut Query<(&'static mut RenderContextMark, Has<T>)>,
//     removed: &mut ComponentRemoved<T>,
//     mark_type: usize,
//     // event_writer: &mut EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
//     // render_context: &mut Query<&'static mut RenderContextMark>,
// ) {
//     // Opacity组件删除，取消渲染上下文标记
//     for i in removed.iter() {
//         if let Ok((mut render_mark_value, has_t)) = dels.get_mut(*i) {
//             if has_t {
//                 continue;
//             }
//             unsafe { render_mark_value.replace_unchecked(mark_type, false) };
//             // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
//             // log::debug!("pass_mark_del,{:?}, {:?}", del, std::any::type_name::<T>());
//             // if unsafe { render_mark_value.replace_unchecked(mark_type, false) } {
//             //     // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
//             //     log::debug!("pass_mark_del,{:?}, {:?}", del, std::any::type_name::<T>());
//             //     event_writer.send(ComponentEvent::new(del));
//             // }
//         }
//     }
    
// }


#[inline]
pub fn render_mark_true(
    mark_type: usize,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    if !unsafe {render_mark_value.bypass_change_detection().replace_unchecked(mark_type, true) } {
        render_mark_value.set_changed();
    }
}

#[inline]
pub fn render_mark_false(
    mark_type: usize,
    render_mark_value: &mut Mut<RenderContextMark>,
) {
    if unsafe {render_mark_value.bypass_change_detection().replace_unchecked(mark_type, false) } {
        render_mark_value.set_changed();
    }
}
