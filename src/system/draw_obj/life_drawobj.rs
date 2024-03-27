
use bevy_ecs::change_detection::{DetectChangesMut, DetectChanges};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::{Changed, With};
use bevy_ecs::system::{Query, SystemState, Res, Local};
use bevy_ecs::prelude::{Bundle, Commands, Component, EventReader, FromWorld, Resource, World};
use bevy_ecs::world::Ref;
use pi_bevy_ecs_extend::query::or_default::OrDefault;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};
use pi_bevy_ecs_extend::system_param::tree::{Root, Layer};
use pi_bevy_render_plugin::render_cross::DepthRange;
use pi_bevy_render_plugin::{PiRenderDevice, PiVertexBufferAlloter};
use pi_cg2d::Rectangle;
use pi_null::Null;
use pi_render::rhi::shader::{BindLayout, ShaderProgram};
use pi_share::Share;
use pi_style::style::{CgColor, Aabb2};

use crate::components::calc::{DrawInfo, EntityKey, NodeId, InPassId, IsShow, ZRange, RenderContextMark, WorldMatrix};
use crate::components::draw_obj::{BoxType, PipelineMeta, InstanceIndex, GetInstanceSplit, InstanceSplit, RenderCount, Pipeline};
use crate::components::user::{Size, Vector4, BackgroundImage};
use crate::components::{DrawBundle, DrawBundleNew};
use crate::components::pass_2d::{Draw2DList, ParentPassId, Camera, DrawIndex, DrawElement, PostProcessInfo, InstanceDrawState};
use crate::events::{ NodeZindexChange, NodeDisplayChange, EntityChange};
use crate::resource::draw_obj::{ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup, VertexBufferLayoutWithHash, InstanceContext, CommonSampler};
use crate::resource::RenderObjType;

use crate::components::{calc::DrawList, draw_obj::DrawState};
use crate::shader::ui_meterial::UiMaterialBind;
use crate::shader1::{RenderInstances, InstanceData};
use crate::shader1::meterial::{TextureIndexUniform, DepthUniform, ColorUniform, RenderFlagType, TyUniform, MeterialBind, TextWeightUniform, BoxUniform, QuadUniform};

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
#[derive(Clone, Debug)]
pub struct RenderInstances1(pub RenderInstances);

impl Default for RenderInstances1 {
    fn default() -> Self {
        Self(RenderInstances::new(MeterialBind::SIZE, 0))
    }
}

/// 创建渲染实例数据
/// 注意， 这里没考虑节点上纹理修改的问题（TODO）
#[allow(suspicious_double_ref_op)]
pub fn update_render_instance_data(
	mut node_change: EventReader<EntityChange>, // 有节点创建
	mut node_zindex_change: EventReader<NodeZindexChange>, // 有节点zIndex修改
	mut node_display_change: EventReader<NodeDisplayChange>, // 有display发生改变
	mut pass2d_change: EventReader<ComponentEvent<Changed<RenderContextMark>>>, // 有pass2d修改（子pass2d或父pass2d修改）

	mut pass_query: Query<(&mut Draw2DList, Entity)>,
	mut post_info_query: Query<(&PostProcessInfo, Option<&Root>)>,
	mut render_cross_query: Query<(&mut DepthRange, &pi_bevy_render_plugin::render_cross::DrawList)>,
	mut instances : OrInitResMut<InstanceContext>,
	node_query: Query<(Option<&ParentPassId>, &InPassId, &DrawList, &ZRange, &IsShow, Entity, &Layer)>,
	draw_query: Query<(Option<&InstanceSplit>, Option<&Pipeline>)>,

	mut instance_index: Query<(&mut InstanceIndex, OrDefault<RenderCount>)>,

	draw_info: Query<(&DrawInfo, Option<Ref<RenderCount>>)>,

	common_sampler: OrInitRes<CommonSampler>,
	device: Res<PiRenderDevice>,

	query_root: Query<Entity, (With<Root>, With<Size>)>, // 只有gui的Root才会有Size
	mut catche_buffer: Local<RenderInstances1>,
) {
	// log::trace!("life========================node_change={:?}, node_zindex_change={:?}, pass2d_change={:?}, node_display_change={:?}", node_change.len(), node_zindex_change.len(), pass2d_change.len(), node_display_change.len());
	// 如果没有实体创建， 也没有实体删除， zindex也没改变，山下文结构也没改变， 则不需要更新实例数据
	if node_change.len() == 0 &&
		node_zindex_change.len() == 0 && 
		pass2d_change.len() == 0 &&
		node_display_change.len() == 0
	{
		return;
	}
	node_change.clear();
	node_zindex_change.clear();
	pass2d_change.clear();
	node_display_change.clear();

	// 否则，先迭代所有的drawObj,如果drawobj可见,
	for (parent_pass_id, in_pass_id, draw_list, z_range, is_show, id, layer) in node_query.iter() {
		log::debug!("draw info========id={:?}, is_display={:?}, has_draw2d_list={:?}, in_pass_id={:?}, draw_list={:?}", id, is_show.get_display(), in_pass_id, pass_query.get_mut(***in_pass_id).is_ok(),draw_list);
		// // 如果display为false， 则不需要放入渲染列表 TODO
		// if !is_show.get_display() {
		// 	continue;
		// }

		let (mut draw_2d_list, _) = match pass_query.get_mut(***in_pass_id) {
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
                if let Ok((mut p_draw_2d_list, _)) = pass_query.get_mut(*parent.0) {
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

	// let mut new_instances = RenderInstances::new(instances.instance_data.alignment, instances.instance_data.cur_index());
	let mut max_depth_count = 1;
	// 对list变化的pass，从新排序， 并组织实例数据
	for (mut draw_2d_list, pass_id) in pass_query.iter_mut() {
		let draw_2d_list = draw_2d_list.bypass_change_detection();

		draw_2d_list.shrink();

		// 渲染列表未改变， 拷贝旧数据到新的实例数据中， 如果数据偏移发生变化， 还需要标记脏区域
		if !draw_2d_list.list_is_change {
			// 清屏数据
			if !draw_2d_list.clear_instance.is_null() {
				let cur_index = new_instances.cur_index();
				new_instances.extend(instances.instance_data.slice(draw_2d_list.clear_instance..draw_2d_list.clear_instance + new_instances.alignment));
				draw_2d_list.clear_instance = cur_index;
			}

			for draw_element in draw_2d_list.draw_list.iter() {
				if let DrawElement::DrawInstance{draw_state: InstanceDrawState { instance_data_range, ..}, draw_range, .. } = draw_element {
					let mut cur_index = new_instances.cur_index();
					new_instances.extend(instances.instance_data.slice(instance_data_range.clone()));
					// 如果新的索引和原有索引不同，需要更新每个draw_obj的实例索引, 如果深度值不同， 需要更新深度值
					if cur_index != instance_data_range.start {
						new_instances.update_dirty_range(cur_index..cur_index + instance_data_range.len());
						for i in draw_range.clone() {
							if let DrawIndex::DrawObj(draw_entity) = &draw_2d_list.all_list_sort[i].0 {
								let (mut index, render_count) = instance_index.get_mut(draw_entity.0).unwrap();
								let end = cur_index + render_count.0 as usize * new_instances.alignment;
								log::debug!("update_render_instance_data3: {:?}", cur_index..end);
								index.bypass_change_detection().0 = cur_index..end;
								cur_index = end;
							}
						}
						draw_2d_list.list_is_change = false; // 在列表中的位置发生改变， 设置脏， 后续会重新设置深度（如果深度值是局部排序， 似乎不需要？TODO）
					}
					max_depth_count = draw_range.len().max(max_depth_count);

				}

			}
			continue;
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
		
		draw_2d_list.draw_list.clear();
		draw_2d_list.need_dyn_fbo_index.clear();

		// 将排序好的绘制对象劈分成多段, 劈分规则为：
		// 1. 绘制为DrawIndex::Pass2D类型，其直接成为一个劈分点， 把该点的《前面部分》，《自身》，《剩余部分》劈分成三段（剩余部分可能继续被劈分）
		// 2. 如果DrawObject存在UiTexture，由于着色器一次最多接收16个纹理， 因此根据当前纹理是否超出16个为一个新的劈分点，将《前一部分》，《自身和后续部分》劈分成两段
		// 3. pipeline不同，会将《前一部分》，《自身和后续部分》劈分成两段
		let mut start = 0;
		let mut cursor = 0;
		let mut pre_pipeline = instances.common_pipeline.clone();

		match post_info_query.get(pass_id) {
			Ok((post_info, layer)) if post_info.has_effect() || layer.is_some() => {
				// 分配清屏数据
				let index = set_clear_screen_instance(&CgColor::new(0.0, 0.0, 0.0, 0.0), new_instances);
				draw_2d_list.clear_instance = index;

				// draw_2d_list.draw_list.push(DrawElement::DrawInstance {
				// 	draw_state: InstanceDrawState { 
				// 		instance_data_range: index..(index + new_instances.alignment), 
				// 		pipeline: Some(instances.clear_pipeline.clone()),
				// 		texture_bind_group: instances.batch_texture.take_group(&device),
				// 	},
				// 	depth_start: 0,
				// 	draw_range: start..start,
				// });
			},
			_ => {
				// 不清屏
				draw_2d_list.clear_instance = pi_null::Null::null();
			}
		}

		log::trace!("life2========================{:?}",draw_2d_list.all_list_sort.len());

		let mut instance_data_start = new_instances.cur_index();

		// let mut pipeline;
        for (draw_index, _, draw_info) in draw_2d_list.all_list_sort.iter() {
			match draw_index {
				DrawIndex::DrawObj(draw_entity) => {

					if let Ok((instance_split, pipeline)) = draw_query.get(**draw_entity) {
						let p = if let Some(pipeline) = pipeline {
							&pipeline.0
						} else if let Some(InstanceSplit::ByCross(_)) = instance_split {
							&instances.premultiply_pipeline
						} else {
							&instances.common_pipeline
						};
						// 当前pipeline与上一个pipeline不相等， 需要让之前的
						let pipeline = if !Share::ptr_eq(&pre_pipeline, &p) {
							let r = pre_pipeline.clone();
							pre_pipeline = p.clone();
							Some(r)
						}  else if let Some(InstanceSplit::ByCross(_)) = instance_split {
							// 目前跨引擎渲染不合并， 单独一个drawcall (TODO, 由于纹理在渲染图build阶段才能拿到， 这里无法正确创建bindgroup)
							Some(p.clone())
						} else {
							None
							// Some(pre_pipeline.clone()) // 一个一个渲染
						};

						// let pipeline =  if let Some(InstanceSplit::ByCross(_)) = instance_split {
						// 	pre_pipeline = instances.premultiply_pipeline.clone();
						// 	Some(instances.premultiply_pipeline.clone())
						// } else {
						// 	let p = if let Some(pipeline) = pipeline {
						// 		&pipeline.0
						// 	} else {
						// 		&instances.common_pipeline
						// 	};
						// 	// 当前pipeline与上一个pipeline不相等， 需要让之前的
						// 	if !Share::ptr_eq(&pre_pipeline, &p) {
						// 		let r = pre_pipeline.clone();
						// 		pre_pipeline = p.clone();
						// 		Some(r)
						// 	} else {
						// 		None
						// 		// Some(pre_pipeline.clone()) // 一个一个渲染
						// 	}
						// };

						if let Some(p)= &pipeline {
							// 将前一部分劈分出去
							if new_instances.cur_index() > instance_data_start {
								draw_2d_list.draw_list.push(DrawElement::DrawInstance {
									draw_state: InstanceDrawState { 
										instance_data_range: instance_data_start..new_instances.cur_index(), 
										pipeline: Some(p.clone()),
										texture_bind_group: instances.batch_texture.take_group(&device),
									},
									depth_start: 0,
									draw_range: start..cursor,
								});
								max_depth_count = (start..cursor).len().max(max_depth_count);

								// log::warn!("pipeline, {:?}", (start..cursor, instance_data_start..new_index.start, render_count.0));
	
								instance_data_start = new_instances.cur_index();
								start = cursor;
							}
							
						}

						// 为每一个drawObj分配新索引
						let (mut index, render_count) = instance_index.get_mut(draw_entity.0).unwrap();
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

								// 用于debug， 实际上是其他信息
								new_instances.instance_data_mut(new_index.start + i as usize * new_instances.alignment).set_data(&TextWeightUniform(&[draw_entity.0.index() as f32]));
							}
							log::debug!("update_render_instance_data1: {:?}", new_index);
							index.0 = new_index.clone();

						} else {
							new_index = new_instances.cur_index()..new_instances.cur_index() + render_count.0 as usize * new_instances.alignment;
							log::trace!("new_index============{:?}", new_index);
							if render_count.0 > 0 {
								new_instances.extend(instances.instance_data.slice(old_index.clone()));

								if new_index.start != old_index.start || new_index.end != old_index.end {
									new_instances.update_dirty_range(new_index.clone());
								}
							}
							log::debug!("update_render_instance_data2: {:?}", new_index);
							index.bypass_change_detection().0 = new_index.clone();
							
						}
						log::trace!("life1========================insatnce_index={:?}, instance_data_start={:?}, draw_index={:?}, split={:?}, cur_index={:?}, render_count: {:?}, cursor: {}, start: {}", new_index, instance_data_start, draw_index, draw_query.get(**draw_entity), new_instances.cur_index(), render_count, cursor, start);
						

						
						if let Some(instance_split) = instance_split {
							match instance_split {
								InstanceSplit::ByTexture(ui_texture) => {
									// 设置drawobj的纹理索引
									let (texture_index, group) = instances.batch_texture.push(ui_texture, &common_sampler.default, &device);
									new_instances.instance_data_mut(new_index.start/*TODO,这里默认只有一个实例*/).set_data(&TextureIndexUniform(&[texture_index as f32])); // 设置drawobj的纹理索引
									if let Some(group) = group {
										if new_index.start > instance_data_start {
											// batch_texture中纹理已经超出16个，因此需要劈分
											draw_2d_list.draw_list.push(DrawElement::DrawInstance {
												draw_state: InstanceDrawState { 
													instance_data_range: instance_data_start..new_index.start, 
													
													pipeline: match pipeline {
														Some(r) => Some(r),
														None => Some(pre_pipeline.clone()),
													},
													texture_bind_group: Some(group),
												},
												depth_start: 0,
												draw_range: start..cursor,
											});
											// log::warn!("ByTexture, {:?}", (start..cursor, instance_data_start..new_index.end));
											max_depth_count = (start..cursor).len().max(max_depth_count);
											// new_instances.extend_count(cursor - start);
											instance_data_start = new_index.end;
											start = cursor;
										}

									}
								},
								InstanceSplit::ByCross(is_graph) =>  {
									if *is_graph {
										draw_2d_list.draw_list.push(DrawElement::GraphDrawList{ 
											id: draw_entity.clone(), 
											depth_start: 0.0
										});
									} else {
										draw_2d_list.need_dyn_fbo_index.push(draw_2d_list.draw_list.len());
										// canvas需要此DrawInstance将fbo渲染到gui， is_graph为true的时候不需要， 因为完全由某个图节点渲染
										draw_2d_list.draw_list.push(DrawElement::GraphFbo {
											id: draw_entity.clone(), 
											draw_state: InstanceDrawState { 
												instance_data_range: instance_data_start..new_index.end, 
												pipeline: Some(instances.premultiply_pipeline.clone()), // canvas使用不同的混合模式
												texture_bind_group: None, // 由于canvas的fbo在渲染图的build阶段才能分配， 此字段不会设置， 直接在渲染时创建
											},
											depth_start: 0,
											draw_range: start..cursor + 1,
										});
										// log::warn!("ByCross, {:?}", (start..cursor, instance_data_start..new_index.end));
										max_depth_count = (start..cursor + 1).len().max(max_depth_count);
										new_instances.instance_data_mut(new_index.start).set_data(&TextureIndexUniform(&[0.0])); // 设置drawobj的纹理索引(永远是0)
										instance_data_start = new_index.end;
									}
									start = cursor + 1;
								},
							}
						}
					}
				},
				DrawIndex::Pass2D(r) => {
					if new_instances.cur_index() > instance_data_start {
						draw_2d_list.draw_list.push(DrawElement::DrawInstance {
							draw_state: InstanceDrawState { 
								instance_data_range: instance_data_start..new_instances.cur_index(),
								pipeline: Some(pre_pipeline.clone()),
								texture_bind_group: instances.batch_texture.take_group(&device),
							},
							depth_start: 0,
							draw_range: start..cursor,
						});
						new_instances.extend_count(cursor - start);
						max_depth_count = (start..cursor).len().max(max_depth_count);
					}
					// log::warn!("Pass2D, {:?}", (start..cursor, instance_data_start..new_instances.cur_index()));
					draw_2d_list.need_dyn_fbo_index.push(draw_2d_list.draw_list.len());

					draw_2d_list.draw_list.push(DrawElement::Pass2D{ id: r.clone(), depth: -1.0,});
					instance_data_start = new_instances.cur_index();
					start = cursor + 1;
				},
				DrawIndex::DrawObjPost(_) => todo!(),
			}
			
			cursor += 1;
		}

		if new_instances.cur_index() > instance_data_start {
			draw_2d_list.draw_list.push(DrawElement::DrawInstance {
				draw_state: InstanceDrawState { 
					instance_data_range: instance_data_start..new_instances.cur_index(), 
					pipeline: Some(pre_pipeline.clone()),
					texture_bind_group: instances.batch_texture.take_group(&device),
				},
				depth_start: 0,
				draw_range: start..cursor,
			});
			// log::trace!("end, {:?}", (start..cursor, instance_data_start..new_instances.cur_index()));
			max_depth_count = (start..cursor).len().max(max_depth_count);
			new_instances.extend_count(cursor - start);
		}
		
		// 设置all_list长度为0（数据还在，数据用于下次列表与新元素对比，来确定列表是否发生改变）
		draw_2d_list.reset();
	}

	// 深度buffer不够长
	let old_depth_count = instances.depth_data.cur_index() / instances.depth_data.alignment;
	if max_depth_count > old_depth_count {
		instances.depth_data.extend_count(max_depth_count - old_depth_count);
		for i in max_depth_count..old_depth_count {
			instances.depth_data.instance_data_mut(i * 4).set_data(&DepthUniform(&[ -1.0 + i as f32 * DEPTH_SPACE]));
		}
	}

	instances.instance_data.clear();
	// 用新的实例数据替换旧的实例数据
	std::mem::swap(&mut instances.instance_data, &mut *new_instances);

	// for root in query_root.iter() {
	// 	update_depth(root, &mut 0, &mut pass_query, &mut post_info_query, &mut render_cross_query, &mut instances, &vert_allotor);
	// }

	for root in query_root.iter() {
		update_depth(root, &mut 1, &mut pass_query, &mut post_info_query, &mut render_cross_query, &mut instances);
	}
}

// /// 更新深度， 返回消耗的深度空间
// pub fn update_depth(
// 	entity: Entity,
// 	depth_count: &mut usize,
// 	mut pass_query: &mut Query<&mut Draw2DList>,
// 	mut post_info_query: &mut Query<&PostProcessInfo>,
// 	mut render_cross_query: &mut Query<(&mut DepthRange, &pi_bevy_render_plugin::render_cross::DrawList)>,
	
// 	// mut pass_query: &mut Query<&mut Draw2DList>,
// 	instances: &mut InstanceDrawState,
// 	vertex_buffer_alloter: &PiVertexBufferAlloter,
// ) -> usize {
// 	if let Ok(mut list) = pass_query.get_mut(entity) {
// 		let mut list = std::mem::take(&mut list.draw_list);

// 		for i in list.iter_mut() {
// 			match i {
// 				DrawElement::DrawInstance { instance_data_range, draw_range, depth_vert,.. } => {
// 					let depth = *depth_count as f32 * DEPTH_SPACE;
// 					vertex_buffer_alloter.update(depth_vert, bytemuck::cast_slice(&[depth, depth, depth, depth]));
// 					*depth_count += draw_range.len();
// 				},
// 				DrawElement::Pass2D{depth, id} => {
// 					*depth = *depth_count as f32 * DEPTH_SPACE;
// 					*depth_count += 1;

// 					if let Ok(r) = post_info_query.get(**id) {
// 						if r.has_effect() {
// 							update_depth(**id, &mut 0, pass_query, post_info_query, render_cross_query, instances, vertex_buffer_alloter);
// 							continue;
// 						}
// 					}
// 					update_depth(**id, depth_count, pass_query, post_info_query, render_cross_query, instances, vertex_buffer_alloter);

// 				},
// 				DrawElement::GraphDrawList {id, .. } => {
// 					let depth = *depth_count as f32 * DEPTH_SPACE;

// 					let mut count = 1;
// 					if let Ok((mut depth_range, list)) = render_cross_query.get_mut(**id) {
// 						count = (list.require_depth / DEPTH_SPACE).ceil() as usize;
// 						if depth_range.start != depth || depth + list.require_depth != depth_range.end { // 修改深度范围
// 							depth_range.start = depth;
// 							depth_range.end = depth + list.require_depth;
// 						}
// 					}
// 					*depth_count += count;
// 				},
// 				DrawElement::GraphFbo {depth_vert, id, .. } => {
// 					let depth = *depth_count as f32 * DEPTH_SPACE;
// 					vertex_buffer_alloter.update(depth_vert, bytemuck::cast_slice(&[depth, depth, depth, depth]));
// 					*depth_count += 1;
// 				},
// 			}
// 		}

// 		pass_query.get_mut(entity).unwrap().draw_list = list;
// 	}
// 	0
// }


/// 更新深度， 返回消耗的深度空间
pub fn update_depth(
	entity: Entity,
	depth_count: &mut usize,
	pass_query: &mut Query<(&mut Draw2DList, Entity)>,
	post_info_query: &mut Query<(&PostProcessInfo, Option<&Root>)>,
	render_cross_query: &mut Query<(&mut DepthRange, &pi_bevy_render_plugin::render_cross::DrawList)>,
	
	// mut pass_query: &mut Query<&mut Draw2DList>,
	instances: &mut InstanceContext,
	// vertex_buffer_alloter: &PiVertexBufferAlloter,
) -> usize {
	if let Ok((mut list, _)) = pass_query.get_mut(entity) {
		let list_is_change = list.list_is_change;
		let mut list = std::mem::take(&mut list.draw_list);
		

		for i in list.iter_mut() {
			match i {
				DrawElement::DrawInstance { draw_state: InstanceDrawState{ instance_data_range, .. }, depth_start, .. } => {
					if list_is_change || *depth_start != *depth_count {
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
				DrawElement::Pass2D{depth, id} => {
					*depth = calc_depth(*depth_count);
					*depth_count += 1;

					if let Ok((r, _)) = post_info_query.get(**id) {
						if r.has_effect() {
							update_depth(**id, &mut 1, pass_query, post_info_query, render_cross_query, instances);
							continue;
						}
					}
					update_depth(**id, depth_count, pass_query, post_info_query, render_cross_query, instances);

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
				DrawElement::GraphFbo {draw_state: InstanceDrawState{ instance_data_range, .. }, depth_start, .. } => {
					if list_is_change || *depth_start != *depth_count {
						*depth_start = *depth_count;
						instances.instance_data.instance_data_mut(instance_data_range.start).set_data(&DepthUniform(&[calc_depth(*depth_count)]));
					}
					*depth_count += 1;
				},
			}
		}

		let (mut l, _) = pass_query.get_mut(entity).unwrap();
		l.list_is_change = false;
		l.draw_list = list;
	}
	0
}

pub fn calc_depth(index: usize) -> f32 {
	index as f32 * DEPTH_SPACE
}

const DEPTH_SPACE: f32 = 0.0001;

fn set_clear_screen_instance(color: &CgColor, instances: &mut RenderInstances) -> usize{
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

