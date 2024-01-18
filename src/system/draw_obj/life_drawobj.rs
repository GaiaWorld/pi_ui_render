use bevy_ecs::change_detection::DetectChangesMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::Changed;
use bevy_ecs::system::{Query, SystemState, ResMut};
use bevy_ecs::prelude::{Bundle, Commands, Component, EventReader, FromWorld, Resource, World};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_null::Null;
use pi_render::rhi::shader::{BindLayout, ShaderProgram};
use pi_share::Share;

use crate::components::calc::{DrawInfo, EntityKey, NodeId, InPassId, IsShow, ZRange};
use crate::components::draw_obj::{BoxType, PipelineMeta, InstanceIndex};
use crate::components::{DrawBundle, DrawBundleNew};
use crate::components::pass_2d::{Draw2DList, ParentPassId, Camera, DrawIndex, DrawElement};
use crate::events::{EntityCreate, EntityDelete, NodeZindexChange, NodeDisplayChange};
use crate::resource::draw_obj::{ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup, VertexBufferLayoutWithHash};
use crate::resource::RenderObjType;

use crate::components::{calc::DrawList, draw_obj::DrawState};
use crate::shader::ui_meterial::UiMaterialBind;
use crate::shader1::RenderInstances;

use super::calc_text::IsRun;

// 创建或删除DrawObject
pub fn draw_object_life<
    Src: Component,
    RenderType: Resource + std::ops::Deref<Target = RenderObjType> + FromWorld,
    With: Bundle + Default, // 初始化时额外需要插入的组件
    VertLayout: Resource + std::ops::Deref<Target = Share<VertexBufferLayoutWithHash>> + FromWorld,
    Program: ShaderProgram,
    const ORDER: u8,
>(
    world: &mut World,

    state: &mut SystemState<(
        OrInitRes<RenderType>,
        EventReader<ComponentEvent<Changed<Src>>>,
        RemovedComponents<Src>,
        Query<(Option<&'static Src>, &'static mut DrawList)>,
        OrInitRes<ProgramMetaRes<Program>>,
        OrInitRes<VertLayout>,
        OrInitRes<ShaderInfoCache>,
        OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
        Commands,
		OrInitRes<IsRun>,
    )>,

	// #[allow(dead_code)]
    // query_draw_list: &mut SystemState<Query<&'static mut DrawList>>, 
) {
	// let time1 = pi_time::Instant::now();
    let (render_type, mut changed, mut del, mut query_texture, program_meta, vert_layout, shader_catch, group_alloter, mut commands, r) =
        state.get_mut(world);
	if r.0 {
		return;
	}
    let group_alloter = group_alloter.clone();
    let render_type = ***render_type;

	// let mut count1 = 0;
	// let mut count2 = 0;

    // 收集需要删除DrawObject的实体
    for del in del.iter() {
		// count1 += 1;
        if let Ok((texture, mut draw_list)) = query_texture.get_mut(del) {
            if texture.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            draw_list.remove(render_type, |draw_obj| {
				if let Some(mut r) = commands.get_entity(draw_obj.id) {
					r.despawn();
					log::warn!("despawn drawobj====={:?}", draw_obj.id);
					log::debug!(target: format!("entity_{:?}", del).as_str(), "remove RenderObj {:?} for {} destroy, ", &draw_obj.id, std::any::type_name::<Src>());
				}
			});
        }
    }

    let program_meta = program_meta.clone();
    let p_state = shader_catch.common.clone();
    let vert_layout = vert_layout.clone();
	// let time2 = pi_time::Instant::now();

	// let mut spawn_list = Vec::new();
    // 收集需要创建DrawObject的实体
    for changed in changed.iter() {
		// count2 += 1;
        if let Ok((texture, mut draw_list)) = query_texture.get_mut(changed.id) {
            if texture.is_none() {
                continue;
            }
            // 不存在，才需要创建DrawObject
            if let None = draw_list.get_one(render_type) {
                let mut draw_state = DrawState::default();
                let ui_material_group = group_alloter.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                let id = commands
                    .spawn(DrawBundle {
                        node_id: NodeId(EntityKey(changed.id)),
                        draw_state,
                        box_type: BoxType::ContentNone,
                        pipeline_meta: PipelineMeta {
                            type_mark: render_type,
                            program: program_meta.clone(),
                            state: p_state.clone(),
                            vert_layout: vert_layout.clone(),
                            defines: Default::default(),
                        },
                        draw_info: DrawInfo::new(ORDER, false), //TODO
                        other: With::default(),
                    })
                    .id();
				// spawn_list.push(id);
                log::debug!(target: format!("entity_{:?}", changed.id).as_str(), "create RenderObj {:?} for {} changed, ", &id, std::any::type_name::<Src>());
                draw_list.push(render_type, id);
				// log::warn!("create drawobj=================draw={:?}, node={:?}", id, changed.id);
            }
        }
    }
	// if spawn_list.len() > 0 {
	// 	log::warn!("spawn drawobj=================draw={:?}", &spawn_list);
	// }

    state.apply(world);
	// let time3 = pi_time::Instant::now();
	// log::warn!("life======{:?}, {:?}, {:?}, {:?}, {:?}", std::any::type_name::<Src>(), time2 - time1, time3 - time2, count1, count2);
}

/// 新版本的draw_object生命周期管理
/// 用于创建和销毁drawobj
pub fn draw_object_life_new<
    Src: Component,
    RenderType: Resource + std::ops::Deref<Target = RenderObjType> + FromWorld,
    With: Bundle + Default, // 初始化时额外需要插入的组件
    // VertLayout: Resource + std::ops::Deref<Target = Share<VertexBufferLayoutWithHash>> + FromWorld,
    // Program: ShaderProgram,
    const ORDER: u8,
>(
    world: &mut World,

    state: &mut SystemState<(
        OrInitRes<RenderType>,
        EventReader<ComponentEvent<Changed<Src>>>,
        RemovedComponents<Src>,
        Query<(Option<&'static Src>, &'static mut DrawList)>,
        Commands,
		OrInitRes<IsRun>,
    )>,

	// #[allow(dead_code)]
    // query_draw_list: &mut SystemState<Query<&'static mut DrawList>>, 
) {
	// let time1 = pi_time::Instant::now();
    let (render_type, mut changed, mut del, mut query_meterial, mut commands, r) =
        state.get_mut(world);
	if r.0 {
		return;
	}
    let render_type = ***render_type;

	// let mut count1 = 0;
	// let mut count2 = 0;

    // 收集需要删除DrawObject的实体
    for del in del.iter() {
		// count1 += 1;
        if let Ok((texture, mut draw_list)) = query_meterial.get_mut(del) {
            if texture.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            draw_list.remove(render_type, |draw_obj| {
				if let Some(mut r) = commands.get_entity(draw_obj.id) {
					r.despawn();
					log::warn!("despawn drawobj====={:?}", draw_obj.id);
					log::debug!(target: format!("entity_{:?}", del).as_str(), "remove RenderObj {:?} for {} destroy, ", &draw_obj.id, std::any::type_name::<Src>());
				}
			});
        }
    }
	// let time2 = pi_time::Instant::now();

	// let mut spawn_list = Vec::new();
    // 收集需要创建DrawObject的实体
    for changed in changed.iter() {
		// count2 += 1;
        if let Ok((texture, mut draw_list)) = query_meterial.get_mut(changed.id) {
            if texture.is_none() {
                continue;
            }
            // 不存在，才需要创建DrawObject
            if let None = draw_list.get_one(render_type) {
                // let mut draw_state = DrawState::default();
                // let ui_material_group = group_alloter.alloc();
                // draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                let id = commands
                    .spawn(DrawBundleNew {
                        node_id: NodeId(EntityKey(changed.id)),
                        // draw_state,
                        box_type: BoxType::ContentNone,
                        // pipeline_meta: PipelineMeta {
                        //     type_mark: render_type,
                        //     program: program_meta.clone(),
                        //     state: p_state.clone(),
                        //     vert_layout: vert_layout.clone(),
                        //     defines: Default::default(),
                        // },
						instance_index: InstanceIndex::default(),
                        draw_info: DrawInfo::new(ORDER, false), //TODO
                        other: With::default(),
                    })
                    .id();
				// spawn_list.push(id);
                log::debug!(target: format!("entity_{:?}", changed.id).as_str(), "create RenderObj {:?} for {} changed, ", &id, std::any::type_name::<Src>());
                draw_list.push(render_type, id);
				// log::warn!("create drawobj=================draw={:?}, node={:?}", id, changed.id);
            }
        }
    }
	// if spawn_list.len() > 0 {
	// 	log::warn!("spawn drawobj=================draw={:?}", &spawn_list);
	// }

    state.apply(world);
	// let time3 = pi_time::Instant::now();
	// log::warn!("life======{:?}, {:?}, {:?}, {:?}, {:?}", std::any::type_name::<Src>(), time2 - time1, time3 - time2, count1, count2);
}

/// 创建渲染实例数据
pub fn update_render_instance_data(
	node_create: EventReader<EntityCreate>, // 有节点创建
	node_delete: EventReader<EntityDelete>, // 有节点删除
	node_zindex_change: EventReader<NodeZindexChange>, // 有节点zIndex修改
	pass2d_change: EventReader<NodeZindexChange>, // 有pass2d修改（子pass2d或父pass2d修改）
	node_display_change: EventReader<NodeDisplayChange>, // 有pass2d修改（子pass2d或父pass2d修改）

	mut pass_query: Query<(&mut Camera, &mut Draw2DList)>,
	mut instances : ResMut<RenderInstances>,
	node_query: Query<(Option<&ParentPassId>, &InPassId, &DrawList, &ZRange, &IsShow, Entity)>,


	mut instance_index: Query<&mut InstanceIndex>,

	draw_info: Query<&DrawInfo>,

) {
	// 如果没有实体创建， 也没有实体删除， zindex也没改变，山下文结构也没改变， 则不需要更新实例数据
	if node_create.len() == 0 &&
		node_delete.len() == 0 &&
		node_zindex_change.len() == 0 && 
		pass2d_change.len() == 0 &&
		node_display_change.len() == 0
	{
		return;
	}

	// 否则，先迭代所有的drawObj
	for (parent_pass_id, in_pass_id, draw_list, z_range, is_show, id) in node_query.iter() {
		// 如果display为false， 则不需要放入渲染列表
		if !is_show.get_display() {
			continue;
		}

		let (camera, mut draw_2d_list) = match pass_query.get_mut(***in_pass_id) {
            Ok(r) => r,
            _ => return,
        };
        if draw_list.len() > 0 && is_show.get_visibility() {
			let list = &mut *draw_2d_list;
			for draw_id in draw_list.iter() {
				let info = draw_info.get(draw_id.id).unwrap();
				list.push_element(
					DrawIndex::DrawObj(EntityKey(draw_id.id)),
					z_range.clone(),
					info.clone(),
				);
			}
        }
        // parent_pass_id存在，表示本节点是一个pass2d
        if camera.is_active {
            if let Some(parent) = parent_pass_id {
                if let Ok((p_camera, mut p_draw_2d_list)) = pass_query.get_mut(*parent.0) {
					if p_camera.is_active && p_camera.is_change {
						p_draw_2d_list.push_element(DrawIndex::Pass2D(EntityKey(id)), z_range.clone(), DrawInfo::new(10, false));
					}
                }
            }
        }
	}


	let mut new_instances = RenderInstances::new(208, instances.cur_index());
	// 对list变化的pass，从新排序， 并组织实例数据
	for (_camera, mut draw_2d_list) in pass_query.iter_mut() {
		let draw_2d_list = draw_2d_list.bypass_change_detection();

		draw_2d_list.shrink();

		// 渲染列表未改变， 不做处理
		if !draw_2d_list.list_is_change {
			for draw_element in draw_2d_list.draw_list.iter() {
				if let DrawElement::DrawInstance{instance_data_range, draw_range } = draw_element {
					let mut cur_index = new_instances.cur_index();
					new_instances.extend(instances.slice(instance_data_range.clone()));
					// 如果新的索引和原有索引不同，需要更新每个draw_obj的实例索引
					if cur_index != instance_data_range.start {
						for i in draw_range.clone() {
							if let DrawIndex::DrawObj(draw_entity) = &draw_2d_list.all_list_sort[i].0 {
								let mut index = instance_index.get_mut(draw_entity.0).unwrap();
								index.0 = cur_index;
								cur_index = new_instances.next_index(cur_index);
							}
						}
					}
				}
			}
			continue;
		}
		draw_2d_list.list_is_change = false;

		// 优先按是否透明排序， 把不透明排在最前面， 其次按深度从小到大排序
		draw_2d_list.all_list_sort.clear();
		draw_2d_list.all_list_sort.extend_from_slice(draw_2d_list.all_list.as_slice());
		draw_2d_list.all_list_sort.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
			let a_sort_opacity_order = a_sort.opacity_order();
			let b_sort_opacity_order = a_sort.opacity_order();

			if a_sort_opacity_order < b_sort_opacity_order {
				return std::cmp::Ordering::Less
			} else if a_sort_opacity_order > b_sort_opacity_order {
				return std::cmp::Ordering::Greater
			}

            if a_z_depth.start < b_z_depth.start {
                std::cmp::Ordering::Less
            } else if a_z_depth.start > b_z_depth.start {
                std::cmp::Ordering::Greater
            } else {
                if a_sort.order() < b_sort.order() {
                    std::cmp::Ordering::Less
                } else if a_sort.order() > b_sort.order() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });

		draw_2d_list.draw_list.clear();
		let mut start = 0;
		let instance_data_start = new_instances.cur_index();
        for (index, (draw_index, _, _)) in draw_2d_list.all_list_sort.iter().enumerate() {
			match draw_index {
				DrawIndex::DrawObj(draw_entity) => {
					// 为每一个drawObj分配新索引
					let mut index = instance_index.get_mut(draw_entity.0).unwrap();
					let old_index = index.bypass_change_detection().0;
					let new_index;
					if old_index.is_null() {
						// 不存在旧的，则分配一个新索引
						new_index = new_instances.alloc_instance_data();
					} else {
						new_index = new_instances.cur_index();
						new_instances.extend(instances.slice(old_index..instances.next_index(old_index)));
					}
					index.bypass_change_detection().0 = new_index;
				},
				DrawIndex::Pass2D(r) => {
					if index > start {
						draw_2d_list.draw_list.push(DrawElement::DrawInstance {
							instance_data_range: instance_data_start..new_instances.cur_index(), 
							draw_range: start..index
						});
						new_instances.extend_count(index - start);
					}

					draw_2d_list.draw_list.push(DrawElement::Pass2D(r.clone()));
					start = index + 1;
				},
				DrawIndex::DrawObjPost(_) => todo!(),
			}
		}
	}

	new_instances.reserve();

	// 用新的实例数据替换旧的实例数据
	*instances = new_instances;
}


