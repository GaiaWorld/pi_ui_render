
use bevy_ecs::change_detection::{DetectChangesMut, DetectChanges};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::{Changed, With};
use bevy_ecs::system::{Local, ParamSet, Query, Res, ResMut, SystemParam, SystemState};
use bevy_ecs::prelude::{Bundle, Commands, Component, EventReader, FromWorld, Resource, World};
use bevy_ecs::world::Ref;
use pi_assets::asset::Handle;
use pi_bevy_ecs_extend::query::or_default::OrDefault;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};
use pi_bevy_ecs_extend::system_param::tree::{Root, Layer};
use pi_bevy_render_plugin::render_cross::DepthRange;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderGraph};
use pi_null::Null;
use pi_render::components::view::target_alloc::ShareTargetView;
use pi_render::depend_graph::graph;
use pi_render::rhi::asset::{AssetWithId, TextureRes};
use pi_share::Share;
use pi_style::style::CgColor;
use wgpu::core::instance;

use crate::components::calc::{DrawInfo, EntityKey, NodeId, InPassId, IsShow, ZRange, RenderContextMark};
use crate::components::draw_obj::{InstanceIndex, GetInstanceSplit, InstanceSplit, RenderCount, Pipeline, FboInfo};
use crate::components::root::RootInstance;
use crate::components::user::{RenderTargetType, Size};
use crate::components::DrawBundleNew;
use crate::components::pass_2d::{Camera, Draw2DList, DrawElement, DrawIndex, InstanceDrawState, ParentPassId, PostProcessInfo};
use crate::events::{ NodeZindexChange, NodeDisplayChange, EntityChange};
use crate::resource::draw_obj::{InstanceContext, CommonSampler};
use crate::resource::{RenderObjType, PassGraphMap};

use crate::components::calc::DrawList;
use crate::shader::camera;
use crate::shader1::{InstanceData, GpuBuffer};
use crate::shader1::meterial::{TextureIndexUniform, DepthUniform, ColorUniform, RenderFlagType, TyUniform, MeterialBind, BoxUniform, QuadUniform};

use super::calc_text::IsRun;

/// 新版本的draw_object生命周期管理
/// 用于创建和销毁drawobj
pub fn draw_object_life_new<
    Src: Component + GetInstanceSplit,
    RenderType: Resource + std::ops::Deref<Target = RenderObjType> + FromWorld,
    With: Bundle + Default, // 初始化时额外需要插入的组件
    const ORDER: u8,
>(
    world: &mut World,

    state: &mut SystemState<(
		EventWriter<EntityChange>, // 有节点创建
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
    let (mut node_change, render_type, mut changed, mut del, mut query_meterial, mut commands, r) =
        state.get_mut(world);
	if r.0 {
		return;
	}
    let render_type = ***render_type;
	let mut node_is_changed = false;

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
					node_is_changed = true;
					log::trace!("despawn drawobj====={:?}", draw_obj.id);
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
        if let Ok((src, mut draw_list)) = query_meterial.get_mut(changed.id) {
            let texture = match src {
                Some(r) => r,
                None => continue,
            };
            // 不存在，才需要创建DrawObject
			match draw_list.get_one(render_type) {
				None => {
					let bundle = DrawBundleNew {
						node_id: NodeId(EntityKey(changed.id)),
						instance_index: InstanceIndex::default(),
						draw_info: DrawInfo::new(ORDER, false), //TODO
						other: With::default(),
					};
					let id = if let Some(r) = texture.get_split()  {
						commands
							.spawn((bundle, r))
							.id()
					} else {
						commands
							.spawn(bundle)
							.id()
					};
					node_is_changed = true;
					
					// spawn_list.push(id);
					log::debug!(target: format!("entity_{:?}", changed.id).as_str(), "create RenderObj {:?} for {} changed, ", &id, std::any::type_name::<Src>());
					draw_list.push(render_type, id);
					log::debug!("create drawobj=================draw={:?}, node={:?}, ty={:?}", id, changed.id, std::any::type_name::<Src>());
				},
				
				Some(r) => if let Some(InstanceSplit::ByTexture(t)) = texture.get_split() {
					// 图片修改， 也需要重新组织实例数据
					node_is_changed = true;
					commands.entity(r.id).insert(InstanceSplit::ByTexture(t));
				},
			};
        }
    }
	if node_is_changed {
		node_change.send(EntityChange);
	}

    state.apply(world);
	// let time3 = pi_time::Instant::now();
}

// 渲染实例数据
#[derive(Clone, Debug, Resource)]
pub struct RenderInstances1(pub GpuBuffer);

impl Default for RenderInstances1 {
    fn default() -> Self {
        Self(GpuBuffer::new(MeterialBind::SIZE, 0))
    }
}

/// 创建渲染实例数据
/// 注意， 这里没考虑节点上纹理修改的问题（TODO）
#[allow(suspicious_double_ref_op)]
pub fn update_render_instance_data(
	mut events: (
		EventReader<EntityChange>,// 有节点创建
		EventReader<NodeZindexChange>, // 有节点zIndex修改
		EventReader<NodeDisplayChange>, // 有display发生改变
		EventReader<ComponentEvent<Changed<RenderContextMark>>>, // 有pass2d修改（子pass2d或父pass2d修改）
	), 
	

	mut pass_query: ParamSet<(
		Query<(&mut Draw2DList, Entity)>,
		Query<&mut Draw2DList>,
	)>,
	mut post_info_query: Query<(&PostProcessInfo, Option<&Root>)>,
	mut render_cross_query: Query<(&mut DepthRange, &pi_bevy_render_plugin::render_cross::DrawList)>,
	mut instances : OrInitResMut<InstanceContext>,
	node_query: Query<(Option<&ParentPassId>, &InPassId, &DrawList, &ZRange, &IsShow, Entity, &Layer)>,
	draw_query: Query<(Option<&InstanceSplit>, Option<&Pipeline>)>,

	mut instance_index: Query<(&'static mut InstanceIndex, OrDefault<RenderCount>)>,

	draw_info: Query<(&DrawInfo, Option<Ref<RenderCount>>)>,

	common_sampler: OrInitRes<CommonSampler>,
	device: Res<PiRenderDevice>,

	query_root: Query<Entity, (With<Root>, With<Size>)>, // 只有gui的Root才会有Size
	query_root1: Query<(&RootInstance, Entity, OrDefault<RenderTargetType>, &PostProcessInfo, &IsShow, &Layer)>, // 只有gui的Root才会有Size
	mut catche_buffer: OrInitResMut<RenderInstances1>,
	mut pass_graph_map: OrInitResMut<PassGraphMap>,
	mut rg: ResMut<PiRenderGraph>,
) {
	log::trace!("life========================node_change={:?}, node_zindex_change={:?}, pass2d_change={:?}, node_display_change={:?}", events.0.len(), events.1.len(), events.2.len(), events.3.len());
	// 如果没有实体创建， 也没有实体删除， zindex也没改变，山下文结构也没改变， 则不需要更新实例数据
	if events.0.len() == 0 &&
	events.1.len() == 0 && 
	events.2.len() == 0 &&
	events.3.len() == 0
	{
		return;
	}
	events.0.clear();
	events.1.clear();
	events.2.clear();
	events.3.clear();
	let catche_buffer = &mut *catche_buffer;

	let mut p1 = pass_query.p1();
	// 否则，先迭代所有的drawObj,如果drawobj可见,
	for (parent_pass_id, in_pass_id, draw_list, z_range, is_show, id, layer) in node_query.iter() {
		// log::debug!("draw info========id={:?}, is_display={:?}, has_draw2d_list={:?}, in_pass_id={:?}, draw_list={:?}", id, is_show.get_display(), in_pass_id, p1.get_mut(***in_pass_id).is_ok(),draw_list);
		// // 如果display为false， 则不需要放入渲染列表 TODO
		// if !is_show.get_display() {
		// 	continue;
		// }

		let mut draw_2d_list = match p1.get_mut(***in_pass_id) {
            Ok(r) => r,
            _ => continue,
        };
        if draw_list.len() > 0 {
			let list = &mut *draw_2d_list;
			for draw_id in draw_list.iter() {
				let (info, render_count) = draw_info.get(draw_id.id).unwrap();
				// 渲染数量修改， 则list一定会修改
				if let Some(render_count) = render_count {
					if render_count.is_changed() {
						list.list_is_change = true;
					}
				}
				let mut info = info.clone();
				info.set_visibility(is_show.get_visibility() && is_show.get_display() && layer.layer() > 0);
				list.push_element(
					DrawIndex::DrawObj(EntityKey(draw_id.id)),
					z_range.clone(),
					info,
				);
			}
        }
        // parent_pass_id存在，表示本节点是一个pass2d
        // if camera.is_active {
            if let Some(parent) = parent_pass_id {
                if let Ok(mut p_draw_2d_list) = p1.get_mut(*parent.0) {
					// if p_camera.is_active && p_camera.is_change {
						p_draw_2d_list.push_element(DrawIndex::Pass2D(EntityKey(id)), z_range.clone(), DrawInfo::new(10, false));
					// }
                }
            }
        // }
	}

	let new_instances = &mut catche_buffer.0;
	if new_instances.capacity() < instances.instance_data.cur_index() { // 扩容， 避免内存拷贝
		new_instances.data.reserve(instances.instance_data.cur_index() - new_instances.data.len());
	}


	let mut alloc = |draw_index: &DrawIndex, draw_info: &DrawInfo, new_instances: &mut GpuBuffer, instances: &InstanceContext, instance_index: &mut Query<(&'static mut InstanceIndex, OrDefault<RenderCount>)>| {
		let mut alloc:  Option<Entity> = None;
		match draw_index {
			DrawIndex::DrawObj(draw_entity) => {
				alloc = Some(draw_entity.0);
			},
			DrawIndex::Pass2D(entity) => {
				if let Ok((post_info, _)) = post_info_query.get(entity.0) {
					// 后处理节点留在本层渲染末尾处理
					if post_info.has_effect() {
						// 如果存在后处理特效， 需要分配一个实例， 用于将特效拷贝到gui上
						alloc = Some(entity.0);
					}
				}
			},

			_ => {},
		}

		if let Some(entity) = alloc {
			// 为每一个drawObj分配新索引
			let (mut index, render_count) = instance_index.get_mut(entity).unwrap();
			let old_index = index.bypass_change_detection().0.clone();
			let new_index;
			if old_index.is_null() || old_index.len() != new_instances.alignment * render_count.0 as usize {
				// 不存在旧的，则分配一个新索引
				new_index = new_instances.alloc_instance_data_mult(render_count.0 as usize);
				let mut ty = 0;
				if !draw_info.is_visibility() {
					ty |=1 << RenderFlagType::NotVisibility as usize;
				}
				
				// 初始化渲染类型
				for i in 0..render_count.0 {
					new_instances.instance_data_mut(new_index.start + i as usize * new_instances.alignment).set_data(&TyUniform(&[ty as f32]));

					// // 用于debug， 实际上是其他信息
					// instances.instance_data_mut(new_index.start + i as usize * instances.alignment).set_data(&TextWeightUniform(&[entity.index() as f32]));
				}

				log::trace!("instance_data_mut1============{:?}, {:?}", new_index, old_index);
				index.0 = new_index.clone();

			} else {
				// 存在就的，从旧的实例上拷贝过来
				new_index = new_instances.cur_index()..new_instances.cur_index() + render_count.0 as usize * new_instances.alignment;
				log::trace!("new_index============{:?}, {:?}, {:?}", entity, new_index, old_index);
				if render_count.0 > 0 {
					new_instances.extend(instances.instance_data.slice(old_index.clone()));

					if new_index.start != old_index.start || new_index.end != old_index.end {
						new_instances.update_dirty_range(new_index.clone());
					}
				}
				index.bypass_change_detection().0 = new_index.clone();
				
			}
			// log::trace!("life1========================insatnce_index={:?}, instance_data_start={:?}, draw_index={:?}, split={:?}, cur_index={:?}, render_count: {:?}", new_index, instance_data_start, draw_index, draw_query.get(entity), new_instances.cur_index(), render_count);
		}
	};
	let mut p0 = pass_query.p0();
	// let mut new_instances = RenderInstances::new(instances.instance_data.alignment, instances.instance_data.cur_index());
	// for (root_instance, root_entity, render_target_type, post_process_info, is_show, layer) in query_root1.iter() {
		// // root节点特殊处理， 需要将自身push在渲染列表中
		// let (mut draw_2d_list, pass_id) = match p0.get_mut(root_entity) {
		// 	Ok(r) => r,
		// 	_ => continue
		// };
		// draw_2d_list.push_element(DrawIndex::Pass2D(EntityKey(root_entity)), ZRange(std::usize::MIN + 1..std::usize::MIN),  DrawInfo::new(10, false));
		
		
		
	let pass_toop_list = std::mem::take(&mut instances.pass_toop_list);
	for entity in pass_toop_list.iter() {
		let (mut draw_2d_list, pass_id) = match p0.get_mut(*entity) {
			Ok(r) => r,
			_ => continue
		};

		let draw_2d_list = draw_2d_list.bypass_change_detection();

		draw_2d_list.shrink();

		// 渲染列表未改变， 拷贝旧数据到新的实例数据中， 如果数据偏移发生变化， 还需要标记脏区域
		if !draw_2d_list.list_is_change {
			let instance_data_range = &draw_2d_list.instance_range;
			let mut cur_index = new_instances.cur_index();
			new_instances.extend(instances.instance_data.slice(instance_data_range.clone()));
			// 如果新的索引和原有索引不同，需要更新每个draw_obj的实例索引, 如果深度值不同， 需要更新深度值
			if cur_index != instance_data_range.start {
				new_instances.update_dirty_range(cur_index..cur_index + instance_data_range.len());
				for el in draw_2d_list.all_list_sort.iter() {
					if let DrawIndex::DrawObj(entity) | DrawIndex::Pass2D(entity)  = &el.0 {
						let (mut index, render_count) = instance_index.get_mut(entity.0).unwrap();
						let end = cur_index + render_count.0 as usize * new_instances.alignment;
						index.bypass_change_detection().0 = cur_index..end;
						cur_index = end;
					};
				}
				instances.rebatch = true; // 需要重新批处理
			}
		} else {
			instances.rebatch = true; // 需要重新批处理
		}

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
		
		// draw_2d_list.instance_range.clear();
		// draw_2d_list.need_dyn_fbo_index.clear();

		log::trace!("life2========================{:?}, {:?}, {:?}", entity, draw_2d_list.all_list_sort.len(), &draw_2d_list.all_list_sort); 

		let instance_data_start = new_instances.cur_index();
		// let mut pipeline;
		for (draw_index, _, draw_info) in draw_2d_list.all_list_sort.iter() {
			alloc(draw_index, draw_info, new_instances, &instances, &mut instance_index);
		}

		// log::warn!("all====={:?}", (entity, &draw_2d_list.all_list_sort));
		// 设置当前pass对应的实例范围（当一些节点发生改变， 而当前pass的节点未发生变动， 则根据该范围从旧的实例数据拷贝到新的实例数据）
		draw_2d_list.instance_range = instance_data_start..new_instances.cur_index();

		// }
	}

	// 为根节点分配实例， 用于将根节点拷贝到屏幕上
	for (root_instance, root_entity, render_target_type, post_process_info, is_show, layer) in query_root1.iter() {
		if post_process_info.has_effect() && RenderTargetType::Screen == *render_target_type {
			// 有后处理效果， 并且最终会渲染到屏幕上， 则需要分配一个实例用于将其渲染到屏幕
			let mut info = DrawInfo::new(10, false);
			info.set_visibility(is_show.get_visibility() && is_show.get_display() && layer.layer() > 0);
			alloc(&DrawIndex::Pass2D(EntityKey(root_entity)), &info, new_instances, &instances, &mut instance_index);

			// 否则， 不需要这个实例渲染
		} else {
			let (mut index, _render_count) = instance_index.get_mut(root_entity).unwrap();
			*index = InstanceIndex::default();
		}
	}
	
	// 分配清屏所需实例（清屏需要批渲染，因此将其分配在一起）
	for entity in pass_toop_list.iter() {
		let (mut draw_2d_list, pass_id) = match p0.get_mut(*entity) {
			Ok(r) => r,
			_ => continue
		}; 

		match post_info_query.get(pass_id) {
			Ok((post_info, layer)) if post_info.has_effect() || layer.is_some() => {
				// 清屏数据
				let index = if !draw_2d_list.clear_instance.is_null() {
					let cur_index = new_instances.cur_index();
					new_instances.extend(instances.instance_data.slice(draw_2d_list.clear_instance..draw_2d_list.clear_instance + new_instances.alignment));
					if cur_index != draw_2d_list.clear_instance {
						let end = new_instances.cur_index();
						new_instances.update_dirty_range(cur_index..end);
					}
					cur_index
				} else {
					set_clear_screen_instance(&CgColor::new(0.0, 0.0, 0.0, 0.0), new_instances)
				};
				// 分配清屏数据
				draw_2d_list.clear_instance = index;
			},
			_ => {
				// 不清屏
				draw_2d_list.clear_instance = pi_null::Null::null();
			}
		}
	}
	instances.pass_toop_list = pass_toop_list;

	instances.instance_data.clear();
	// 用新的实例数据替换旧的实例数据
	std::mem::swap(&mut instances.instance_data, &mut *new_instances);
}

fn batch_pass(
	query: &mut BatchQuery,
	root_state: &mut BatchRootState,
	global_state: &mut BatchGlobalState,
	instances: &mut InstanceContext,
	draw_list: &mut Draw2DList,
	// all_list_sort: &Vec<(DrawIndex, ZRange, DrawInfo)>, 
	// instance_data_start: &mut usize,  
	// instance_data_end: &mut usize,
	pass_id: Entity,

	parent_pass_id: Entity,
) {
	let mut start = 0;
	let mut cursor = 0;

	let mut instance_data_start = draw_list.instance_range.start;
	let mut instance_data_end =  draw_list.instance_range.start;
	// let mut pipeline;
	// log::warn!("batch_pass======={:?}", (pass_id, &draw_list.all_list_sort));
	for (draw_index, _, draw_info) in draw_list.all_list_sort.iter() {
		let mut last_pipeline = None;
		let mut split_by_texture:  Option<(InstanceIndex, &Handle<AssetWithId<TextureRes>>, &wgpu::Sampler)> = None;
		let mut instance_data_end1 = instance_data_end;
		let mut cross_list: Option<EntityKey> = None;

		let cur_pipeline = match draw_index.clone() {
			DrawIndex::DrawObj(draw_entity) => if let Ok((instance_split, pipeline, fbo_info)) = query.draw_query.get(*draw_entity) {
				// 为每一个drawObj分配新索引
				let index = query.instance_index.get_mut(draw_entity.0).unwrap();
				instance_data_end1 = instance_data_end;
				instance_data_end = index.end;
				let cur_pipeline = if let Some(pipeline) = pipeline {
					&pipeline.0
				} else {
					&instances.common_pipeline
				};

				if let Some(instance_split) = instance_split {
					match instance_split {
						InstanceSplit::ByTexture(ui_texture) => {
							split_by_texture = Some(((*index).clone(), ui_texture, &query.common_sampler.default));
						},
						InstanceSplit::ByCross(is_list) =>  {
							if *is_list {
								cross_list = Some(draw_entity);
								// is_list为true时， 必须劈分
								last_pipeline = Some(root_state.pre_pipeline.clone())
							} else {
								// 设置实例是否需要还原预乘
								let mut ty = instances.instance_data.instance_data_mut(index.start).get_render_ty();
								
								match pipeline{
									Some(r) if !Share::ptr_eq(r, &instances.premultiply_pipeline) => ty &= !(1 << RenderFlagType::Premulti as usize),
									_ => ty |= 1 << RenderFlagType::Premulti as usize,
								};
								let mut instance_data = instances.instance_data.instance_data_mut(index.start);
								instance_data.set_data(&TyUniform(&[ty as f32]));
								if let Some(r) = &fbo_info.out {
									split_by_texture = Some((index.clone(), &r.target().colors[0].0, &query.common_sampler.default)); // TODO， 根据纹理尺寸目标尺寸选择混合模式
								};
							}
						},
					}
				}

				cur_pipeline
			} else {
				&instances.common_pipeline
			},
			DrawIndex::Pass2D(r) => match query.post_info_query.get(r.0) {
				Ok(post_info) if  post_info.has_effect() => {
					let (_, _, fbo_info) = query.draw_query.get(r.0).unwrap();
					let index = query.instance_index.get_mut(r.0).unwrap();
					instance_data_end1 = instance_data_end;
					instance_data_end = index.end;
					if let Some(r) = &fbo_info.out {
						split_by_texture = Some((index.clone(), &r.target().colors[0].0, &query.common_sampler.default));
					}
					
					instances.posts.push(*r);// 后处理节点留在本层渲染末尾处理
					&instances.common_pipeline
				},
				_ => {
					// 将当前剩余未批处理的数据合批
					if instance_data_start < instance_data_end {
						instances.draw_list.push((DrawElement::DrawInstance {
							draw_state: InstanceDrawState { 
								instance_data_range: instance_data_start..instance_data_end, 
								pipeline: Some(instances.common_pipeline.clone()),
								texture_bind_group: None,
							},
							depth_start: 0,
							draw_range: start..cursor,
							pass: pass_id,
						}, parent_pass_id));
						instance_data_start = instance_data_end;
					}

					let mut draw_2d_list = match query.pass_query.get_mut(*r) {
						Ok(r) => r,
						_ => continue
					};
					let draw_2d_list = draw_2d_list.bypass_change_detection();
					let mut draw_2d_list = std::mem::take(draw_2d_list);
					batch_pass(query, root_state, global_state, instances, &mut draw_2d_list, *r, parent_pass_id);
					let mut draw_2d_list1= match query.pass_query.get_mut(*r) {
						Ok(r) => r,
						_ => continue
					};
					*(draw_2d_list1.bypass_change_detection()) = draw_2d_list;
					continue;
				},
			},
			_ => &root_state.pre_pipeline,
		};

		// 当前pipeline与上一个pipeline不相等， 需要劈分之前的
		if !Share::ptr_eq(&root_state.pre_pipeline, &cur_pipeline) {
			let r = root_state.pre_pipeline.clone();
			root_state.pre_pipeline = cur_pipeline.clone();
			last_pipeline = Some(r);
		}

		// if last_pipeline.is_none() {
		// 	last_pipeline = Some(root_state.pre_pipeline.clone());
		// }

		// 将前一部分劈分出去
		if let Some(p)= &last_pipeline {
			if instance_data_end1 > instance_data_start {
				instances.draw_list.push((DrawElement::DrawInstance {
					draw_state: InstanceDrawState { 
						instance_data_range: instance_data_start..instance_data_end1, 
						pipeline: Some(p.clone()),
						texture_bind_group: None,
					},
					depth_start: 0,
					draw_range: start..cursor,
					pass: pass_id,
				}, parent_pass_id));
				// draw_2d_list.draw_list.push();
				// max_depth_count = (start..cursor).len().max(max_depth_count);

				// log::warn!("pipeline, {:?}", (start..cursor, instance_data_start..new_index.start, render_count.0));
				instance_data_start = instance_data_end1;
				start = cursor;
			}
		}

		// 其他框架提供的渲染列表
		if let Some(draw_entity) = cross_list {
			instances.draw_list.push((DrawElement::GraphDrawList{ 
				id: draw_entity, 
				depth_start: 0.0
			}, parent_pass_id));
			start = cursor + 1;
		}

		// 添加渲染所需纹理， 如果纹理溢出， 需要结束批处理
		if let Some((index, texture, sampler)) = split_by_texture {
			let (texture_index, group) = instances.batch_texture.push(texture, &query.common_sampler.default, &query.device);
			instances.instance_data.instance_data_mut(index.start/*TODO,这里默认只有一个实例*/).set_data(&TextureIndexUniform(&[texture_index as f32])); // 设置drawobj的纹理索引
			if let Some(group) = group {
				let group = Share::new(group);
				if instance_data_end1 > instance_data_start {
					// batch_texture中纹理已经超出16个，因此需要劈分
					instances.draw_list.push((DrawElement::DrawInstance {
						draw_state: InstanceDrawState { 
							instance_data_range: instance_data_start..instance_data_end1, 
							
							pipeline: match last_pipeline {
								Some(r) => Some(r),
								None => Some(root_state.pre_pipeline.clone()),
							},
							texture_bind_group: Some(group.clone()),
						},
						depth_start: 0,
						draw_range: start..cursor,
						pass: pass_id,
					}, parent_pass_id));
					global_state.last_group = Some(group.clone());
					// log::warn!("ByTexture, {:?}", (start..cursor, instance_data_start..new_index.end));
					// max_depth_count = (start..cursor).len().max(max_depth_count);
					// instances.extend_count(cursor - start);
					instance_data_start = instance_data_end1;
					start = cursor;
				}
				// 设置之前的批渲染的纹理group
				for i in global_state.pre_group..instances.draw_list.len() {
					if let DrawElement::DrawInstance { draw_state, .. } | DrawElement::Clear { draw_state, .. } = &mut instances.draw_list[i].0 {
						draw_state.texture_bind_group = Some(group.clone());
					}
				}
				global_state.pre_group = instances.draw_list.len();
			}
		}
		
		cursor += 1;
	}	

	// log::warn!("aa====={:?}", (instance_data_start, instance_data_end, instances.draw_list.len()));
	// 将当前剩余未批处理的数据合批
	if instance_data_start < instance_data_end {
		instances.draw_list.push((DrawElement::DrawInstance {
			draw_state: InstanceDrawState { 
				instance_data_range: instance_data_start..instance_data_end, 
				pipeline: Some(instances.common_pipeline.clone()),
				texture_bind_group: None,
			},
			depth_start: 0,
			draw_range: start..cursor,
			pass: pass_id,
		}, parent_pass_id));
		instance_data_start = instance_data_end;
	}

	// 设置all_list长度为0（数据还在，数据用于下次列表与新元素对比，来确定列表是否发生改变）
	draw_list.reset();
}

#[derive(SystemParam)]
pub struct BatchQuery<'s, 'w> {
	pass_query: Query<'w, 's, &'static mut Draw2DList>,
	post_info_query: Query<'w, 's, &'static PostProcessInfo>,
	render_cross_query: Query<'w, 's, (&'static mut DepthRange, &'static pi_bevy_render_plugin::render_cross::DrawList)>,
	draw_query: Query<'w, 's, (Option<&'static InstanceSplit>, Option<&'static Pipeline>, OrDefault<FboInfo>)>,
	instance_index: Query<'w, 's, &'static InstanceIndex>,
	common_sampler: OrInitRes<'w,CommonSampler>,
	device: Res<'w,PiRenderDevice>,
}

// 批处理状态维护
struct BatchRootState {
	pre_pipeline: Share<wgpu::RenderPipeline>,
	// next_node_with_depend_list: &'a Vec<usize>,
	next_node_with_depend_index: usize,
	next_node_with_depend: usize,
	// toop_list_len: usize, // pass的最大数量
}

// 批处理全局状态维护
struct BatchGlobalState{
	post_start: usize,
	pre_group: usize,
	last_group: Option<Share<wgpu::BindGroup>>,
	last_fbo: Option<ShareTargetView>,
}

/// 批处理实例
/// 在渲染图的build之后， 渲染之前运行
/// 只将需要渲染的节点节点批处理
pub fn batch_instance_data(
	mut query: BatchQuery,
	mut query_root: Query<(&mut RootInstance, Entity, &InstanceIndex), With<Root>>, // 只有gui的Root才会有Size
	mut instances : OrInitResMut<InstanceContext>,
) {

	let instances = &mut *instances;
	if !instances.rebatch {
		return;
	}
	instances.draw_list.clear();
	instances.posts.clear();
	instances.rebatch = false;
	

	let mut global_state = BatchGlobalState {
		post_start: 0,
		pre_group: 0,
		last_group: None,
		last_fbo: None,
	};

	// for (mut root_instance, root, instance_index) in query_root.iter_mut() {
		// log::warn!("len====={:?}", (&root, instances.batch_texture.temp_textures.len()));
		// 当前剩余未批处理的数据合批
		// 将排序好的绘制对象劈分成多段, 劈分规则为：
		// 1. 绘制为DrawIndex::Pass2D类型，其直接成为一个劈分点， 把该点的《前面部分》，《自身》，《剩余部分》劈分成三段（剩余部分可能继续被劈分）
		// 2. 如果DrawObject存在UiTexture，由于着色器一次最多接收16个纹理， 因此根据当前纹理是否超出16个为一个新的劈分点，将《前一部分》，《自身和后续部分》劈分成两段
		// 3. pipeline不同，会将《前一部分》，《自身和后续部分》劈分成两段
		let mut batch_state = BatchRootState {
			next_node_with_depend: instances.next_node_with_depend.get(0).map_or(std::usize::MAX, |r| {*r}),
			// toop_list_len: root_instance.pass_toop_list.len(),  // pass的最大数量
			pre_pipeline: instances.common_pipeline.clone(),
			// next_node_with_depend_list: &root_instance.next_node_with_depend,
			next_node_with_depend_index: 0,
		};

		let mut pre_clear_index = 0;
		
		let pass_toop_list = std::mem::take(&mut instances.pass_toop_list);
		for (pass_index, pass_id) in pass_toop_list.iter().enumerate() {
			let pass_index = pass_index + 1;

			let mut draw_2d_list= match query.pass_query.get_mut(*pass_id) {
				Ok(r) => r,
				_ => continue
			};

			let (_, _, fbo_info) = query.draw_query.get(*pass_id).unwrap();
			

			let mut fbo_changed = false;
			let draw_2d_list = draw_2d_list.bypass_change_detection();
			if !draw_2d_list.clear_instance.is_null() {
				let fbo_changed1 = match (&fbo_info.fbo, &global_state.last_fbo){
					(Some(r), Some(r1)) => {
						if !Share::ptr_eq(&r.target().colors[0].0, &r1.target().colors[0].0) {
							global_state.last_fbo = Some(r.clone());
							fbo_changed = true;
							true
						} else {
							false
						}
					},
					(None, None) => {
						true
					},
					(r, _) => {
						global_state.last_fbo = r.clone();
						true
					},
				};	

				let (split_index, end) = if fbo_changed1 {
					// 如果fbo发生了改变， 重新劈分clear
					let c = (DrawElement::Clear {
						draw_state: InstanceDrawState { 
							instance_data_range: draw_2d_list.clear_instance..draw_2d_list.clear_instance + instances.instance_data.alignment, 
							pipeline: Some(instances.clear_pipeline.clone()),
							texture_bind_group: None,
						},
						pass: *pass_id,
					}, *pass_id);
					let last_index = pre_clear_index;
					pre_clear_index = instances.draw_list.len(); // 记录当前清屏所需drawcall（实例化渲染， 渲染多个清屏）的索引
					instances.draw_list.push(c);
					batch_state.pre_pipeline = instances.clear_pipeline.clone();
					if instances.draw_list.len() > 1 {
						(last_index, draw_2d_list.clear_instance)
					} else {
						(Null::null(), 0)
					}
				} else {
					if instances.draw_list.len() > 0 && pass_index >= batch_state.next_node_with_depend {
						(pre_clear_index, draw_2d_list.clear_instance + instances.instance_data.alignment)
					} else {
						(Null::null(), draw_2d_list.clear_instance)
					}
				};
				if !split_index.is_null() {
					// fbo未改变， 并且迭代结束了，则设置上一个清屏drawcall的实例范围
					if let DrawElement::Clear {draw_state, ..} = &mut instances.draw_list[split_index].0 {
						draw_state.instance_data_range.end = end;
						log::warn!("is_split_clear========={:?}", (pass_id, split_index, pre_clear_index, draw_2d_list.clear_instance, fbo_changed1, &draw_state.instance_data_range, draw_state.instance_data_range.len() / 224));
					}
				}

				// let mut instance_data_start = draw_2d_list.instance_range.start;
				// let mut instance_data_end =  draw_2d_list.instance_range.start;
				let mut draw_2d_list = std::mem::take(draw_2d_list);
				
				batch_pass(&mut query, &mut batch_state, &mut global_state, instances, &mut draw_2d_list, *pass_id, *pass_id);

				// 还回列表
				let mut draw_2d_list1= match query.pass_query.get_mut(*pass_id) {
					Ok(r) => r,
					_ => unreachable!()
				};
				*(draw_2d_list1.bypass_change_detection()) = draw_2d_list;
				// log::warn!("effect================{:?}", pass_id);
			}	
			
			// 已经到达下一个"有依赖未就绪"的节点
			// 在draw_list中push一个DrawPost， 用于绘制当前多有需要绘制的后处理效果
			if pass_index >= batch_state.next_node_with_depend || fbo_changed {
				if pass_index >= batch_state.next_node_with_depend {
					batch_state.next_node_with_depend_index += 1;
					batch_state.next_node_with_depend = instances.next_node_with_depend.get(batch_state.next_node_with_depend_index).map_or(std::usize::MAX, |r| {*r});
					if global_state.post_start < instances.posts.len() {
						let post = (DrawElement::DrawPost(global_state.post_start..instances.posts.len()), *pass_id);
						instances.draw_list.push(post);
						global_state.post_start = instances.posts.len();
					}
				}
				
				// 如果处理了当前层的后处理， group需要重新生成（不能确定后处理的fbo的依赖关系）
				// let group = if pass_index == batch_state.toop_list_len {
				let group =	match instances.batch_texture.take_group(&query.device) {
					Some(group) => Some(Share::new(group)),
					None => match global_state.last_group.clone() {
						Some(r) => Some(r),
						None => Some(Share::new(instances.batch_texture.default_group(&query.device))),
					}
				};
				// } else {
				// 	match global_state.last_group.clone() {
				// 		Some(r) => Some(r),
				// 		None => Some(Share::new(instances.batch_texture.default_group(&query.device))),
				// 	}
				// };
	
				// log::warn!("pass_index====={:?}", (pass_id, fob_change, pass_index, batch_state.next_node_with_depend, pass_toop_list.len(), global_state.pre_group, instances.draw_list.len(),  &instances.next_node_with_depend));
				for i in global_state.pre_group..instances.draw_list.len() {
					if let DrawElement::DrawInstance { draw_state, .. } | DrawElement::Clear { draw_state, .. } = &mut instances.draw_list[i].0 {
						draw_state.texture_bind_group = group.clone();
					}
				}
				global_state.pre_group = instances.draw_list.len();
				// // 最后一个Pass, 需要设置前面批数据的texture_bind_group
				// if pass_index == batch_state.toop_list_len {
					
				// }
			}
			
			// draw_2d_list.draw_list.push();
			// max_depth_count = (start..cursor).len().max(max_depth_count);

			// log::warn!("pipeline, {:?}", (start..cursor, instance_data_start..new_index.start, render_count.0));

			// instance_data_start = instance_data_end;
		}
		instances.pass_toop_list = pass_toop_list;
		// root_instance.draw_range = element_start..instances.draw_list.len();

		// update_depth(root, &mut 1, &mut query.render_cross_query, instances, &mut root_instance);
	// }

	for (mut root_instance, root, instance_index) in query_root.iter_mut() {
		// 将当前剩余未批处理的数据合批
		// log::warn!("root======={:?}", root);
		if !instance_index.start.is_null() {
			root_instance.draw_index = instances.draw_list.len();
			let p = instances.common_pipeline.clone();
			instances.draw_list.push((DrawElement::DrawInstance {
				draw_state: InstanceDrawState { 
					instance_data_range: instance_index.start..instance_index.end, 
					pipeline: Some(p),
					texture_bind_group: None,
				},
				depth_start: 0,
				draw_range: 0..0,
				pass: root,
			}, EntityKey::null().0));

			let (_, _, fbo_info) = query.draw_query.get(root).unwrap();
			if let Some(target) = &fbo_info.out {
				let texture = &target.target().colors[0].0;
				let (texture_index, group) = instances.batch_texture.push(texture, &query.common_sampler.default, &query.device);
				instances.instance_data.instance_data_mut(instance_index.start).set_data(&TextureIndexUniform(&[texture_index as f32])); // 设置drawobj的纹理索引
				
				if let Some(group) = group {
					let group = Share::new(group);
					// 设置之前的批渲染的纹理group
					for i in global_state.pre_group..instances.draw_list.len() {
						if let DrawElement::DrawInstance { draw_state, .. } | DrawElement::Clear { draw_state, .. } = &mut instances.draw_list[i].0 {
							draw_state.texture_bind_group = Some(group.clone());
						}
						global_state.pre_group = instances.draw_list.len();
					}
				}
			}
		}
		
	}
	
	let group = match instances.batch_texture.take_group(&query.device) {
		Some(group) => Some(Share::new(group)),
		None => match global_state.last_group.clone() {
			Some(r) => Some(r),
			None => Some(Share::new(instances.batch_texture.default_group(&query.device))),
		}
	};
	// 最后一个Pass, 需要设置前面批数据的texture_bind_group
	for i in global_state.pre_group..instances.draw_list.len() {
		if let DrawElement::DrawInstance { draw_state, .. } | DrawElement::Clear { draw_state, .. } = &mut instances.draw_list[i].0 {
			draw_state.texture_bind_group = group.clone();
		}
	}

	update_depth( &mut 1, &mut query.render_cross_query, instances);
}


/// 更新深度， 返回消耗的深度空间
pub fn update_depth(
	depth_count: &mut usize,
	// pass_query: &mut Query<(&mut Draw2DList, Entity)>,
	// post_info_query: &mut Query<(&PostProcessInfo, Option<&Root>)>,
	render_cross_query: &mut Query<(&mut DepthRange, &pi_bevy_render_plugin::render_cross::DrawList)>,
	
	// mut pass_query: &mut Query<&mut Draw2DList>,
	instances: &mut InstanceContext,
	// root_instance: &mut RootInstance,
	// vertex_buffer_alloter: &PiVertexBufferAlloter,
) {
	for i in instances.draw_list.iter_mut() {
		match &mut i.0 {
			DrawElement::DrawInstance { draw_state: InstanceDrawState{ instance_data_range, .. }, depth_start, .. } => {
				if *depth_start != *depth_count {
					*depth_start = *depth_count;
					// list发生改变， 则重设depth
					for i in 0..instance_data_range.len() / instances.instance_data.alignment {
						let index = instance_data_range.start + i * instances.instance_data.alignment;
						instances.instance_data.instance_data_mut(index).set_data(&DepthUniform(&[calc_depth(*depth_count)]));
						*depth_count += 1;
					}
				} else {
					*depth_count += instance_data_range.len() / instances.instance_data.alignment;
				}
				
			},
			DrawElement::GraphDrawList {id, .. } => {
				let depth = calc_depth(*depth_count);

				let mut count = 1;
				if let Ok((mut depth_range, list)) = render_cross_query.get_mut(**id) {
					count = (list.require_depth / DEPTH_SPACE).ceil() as usize;
					if depth_range.start != depth || depth + list.require_depth != depth_range.end { // 修改深度范围
						depth_range.start = depth;
						depth_range.end = depth + list.require_depth;
					}
				}
				*depth_count += count;
			},
			DrawElement::DrawPost(_) => (),
			DrawElement::Clear { draw_state, pass } => (),
		}
	}
}

pub fn calc_depth(index: usize) -> f32 {
	index as f32 * DEPTH_SPACE
}

const DEPTH_SPACE: f32 = 0.0001;

fn set_clear_screen_instance(color: &CgColor, instances: &mut GpuBuffer) -> usize{
	let new_index = instances.alloc_instance_data();
	let mut instance_data = instances.instance_data_mut(new_index);

	let mut render_flag = 0;
	render_flag |= 1 << RenderFlagType::Color as usize;

	instance_data.set_data(&ColorUniform(&[color.x, color.y, color.z, color.w]));
	instance_data.set_data(&TyUniform(&[render_flag as f32]));
	// let world = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
	// instance_data.set_data(&WorldUniform(WorldMatrix::default().as_slice()));
	instance_data.set_data(&DepthUniform(&[0.0]));

	instance_data.set_data(&BoxUniform(&[0.0, 0.0, 1.0, 1.0]));
	instance_data.set_data(&QuadUniform(&[
		-1.0, 1.0,
		-1.0, -1.0,
		1.0, -1.0,
		1.0, 1.0,
	]));

	new_index
}
