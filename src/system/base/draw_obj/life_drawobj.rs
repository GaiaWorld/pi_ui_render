

use pi_bevy_render_plugin::asimage_url::RenderTarget;
use pi_world::alter::Alter;
use pi_world::event::{ComponentAdded, ComponentChanged};
use pi_world::insert::Bundle;
use pi_world::param_set::ParamSet;
use pi_world::prelude::{SystemParam, SingleRes, FromWorld, Insert, With, Query, Entity, OrDefault, Has, Ticker, ComponentRemoved};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes, Layer, Root};

use pi_bevy_render_plugin::PiRenderDevice;
use pi_null::Null;
use pi_render::components::view::target_alloc::ShareTargetView;
use pi_share::Share;
use pi_key_alloter::Key;
use crate::components::draw_obj::{BoxType, DrawCount};

use crate::components::calc::{style_bit, DrawInfo, EntityKey, InPassId, IsShow, NodeId, RenderContextMark, SdfUv, StyleBit, StyleMarkType, ZRange};
use crate::components::draw_obj::{ FboInfo, GetInstanceSplit, HasDraw, InstanceIndex, InstanceSplit, Pipeline, RenderCount};
// use crate::components::root::RootInstance;
use crate::components::user::{IsLeaf, Opacity};
// #[cfg(debug_assertions)]
// use crate::components::user::{BackgroundColor, BackgroundImage, BlendMode, BorderImage, Canvas, TextContent};
// #[cfg(debug_assertions)]
// use crate::components::draw_obj::{BackgroundColorMark, BackgroundImageMark, BorderImageMark, BoxShadowMark, CanvasMark, TextMark, TextShadowMark};

use crate::components::DrawBundleNew;
use crate::components::pass_2d::{Camera, Draw2DList, DrawElement, DrawIndex, InstanceDrawState, ParentPassId, PostProcessInfo};
use crate::resource::draw_obj::{BatchTexture, BatchTextureItem, CommonSampler, DefaultPipelines, InstanceContext};
use crate::resource::{GlobalDirtyMark, IsRun, OtherDirtyType, RenderObjType};

use crate::components::calc::DrawList;
use crate::shader1::GpuBuffer;
use crate::shader1::batch_meterial::{BatchMeterial, DebugInfo, DepthUniform, MeterialBind, OpacityUniform, RenderFlagType, TetxureIndexMeterial, TyMeterial};

/// 新版本的draw_object生命周期管理
/// 用于创建和销毁drawobj
pub fn draw_object_life_new<
    Src: GetInstanceSplit + HasDraw + DrawCount + Send + Sync,
    RenderType: std::ops::Deref<Target = RenderObjType> + FromWorld + Send + Sync,
    Other: Bundle + Default, // 初始化时额外需要插入的组件
    const ORDER: u8,
	const BOX_TYPE: BoxType,
>(
	render_type: OrInitSingleRes<RenderType>,
	mut query_meterial: ParamSet<(
		Query<(&'static Src, &'static mut DrawList, Entity)>,
		Query<(Has<Src>, &'static mut DrawList)>,
		Query<(&'static Src, Entity)>,
	)>,
	changed: ComponentChanged<Src>,
	added: ComponentAdded<Src>,
	removed: ComponentRemoved<Src>,
	mut alter_drawobj: Alter<&DrawInfo, (), InstanceSplit>,
	insert: Insert<(DrawBundleNew<Other>, )>,
	insert1: Insert<(DrawBundleNew<Other>, InstanceSplit)>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
	r: OrInitSingleRes<IsRun>, 
) {
	// let time1 = pi_time::Instant::now();
	if r.0 {
		return;
	}
    let render_type = ***render_type;
	let mut node_is_changed = false;
	let mut rebatch = false;

	// let mut count1 = 0;
	// let mut count2 = 0;

    // 收集需要删除DrawObject的实体
	let mut is_delete = false;
	let p1 = query_meterial.p1();
	for i in removed.iter() {
		if let Ok((has_texture, mut draw_list)) = p1.get_mut(*i) {
			if has_texture {
				continue;
			}
			// 删除对应的DrawObject
			draw_list.remove(render_type, |draw_obj| {
				if let Ok(true) = alter_drawobj.destroy(draw_obj.id) {
					node_is_changed = true;
					log::debug!(target: format!("entity_{:?}", draw_obj.id).as_str(), "remove RenderObj {:?} for {} destroy, ", &draw_obj.id, std::any::type_name::<Src>());
				}
				is_delete = true;
			});
		}
	}
	if is_delete {
		global_mark.mark.set(OtherDirtyType::DrawObjDelete as usize, true);
	}
	
	// let time2 = pi_time::Instant::now();

	// let mut spawn_list = Vec::new();
    // 收集需要创建DrawObject的实体
    // count2 += 1;
	// println!("aaaa============{:?}", std::any::type_name::<Src>());
	// for (src, node) in query_meterial.p2().iter_mut() {
	// 	println!("cccc============{:?}", std::any::type_name::<Src>());
	// }

	// if std::any::type_name::<Src>() == std::any::type_name::<BackgroundColor>() {
	// 	panic!("aaaaaaa=======");
	// }
	let mut is_create = false;
	if changed.len() > 0 || added.len() > 0 {
		let p0 = query_meterial.p0();
		for entity in changed.iter().chain(added.iter()) {
			if let Ok((src, mut draw_list, node)) = p0.get_mut(*entity) {
				if !src.has_draw() {
					continue;
				}

				let mut draw_count = src.draw_count();

				if !draw_count.is_null() && draw_list.count(render_type) != draw_count {
					// 数量不等， 删除原有的
					draw_list.remove(render_type, |draw_obj| {
						if let Ok(true) = alter_drawobj.destroy(draw_obj.id) {
							node_is_changed = true;
							log::debug!(target: format!("entity_{:?}", draw_obj.id).as_str(), "remove RenderObj {:?} for {} destroy, ", &draw_obj.id, std::any::type_name::<Src>());
						}
						is_delete = true;
					});
				}
				
				draw_count = if draw_count.is_null() { 1 } else { draw_count };
				// 不存在，才需要创建DrawObject
				match draw_list.get_one(render_type) {
					None => {
						for _ in 0..draw_count {
							let bundle = DrawBundleNew {
								node_id: NodeId(EntityKey(node)),
								instance_index: InstanceIndex::default(),
								draw_info: DrawInfo::new(ORDER, false), //TODO
								other: Other::default(),
								box_type: BOX_TYPE,
								render_count: Default::default(),
							};
							let id = if let Some(r) = src.get_split()  {
								insert1.insert((bundle, r))
							} else {
								insert.insert((bundle, ))
								
							};
							// spawn_list.push(id);
							log::debug!(target: format!("entity_{:?}", node).as_str(), "create RenderObj {:?} for {} changed, ", &id, std::any::type_name::<Src>());
							draw_list.push(render_type, id);
							log::debug!("create drawobj=================draw={:?}, node={:?}, ty={:?}", id, node, std::any::type_name::<Src>());
						}
						is_create = true;
					},
					
					Some(r) => if let Some(InstanceSplit::ByTexture(t)) = src.get_split() {
						// if node.index() == 159 {
							// println!("rebatch=======node: {:?}, draw: {:?}, texture: {:?}", node, r.id, t.id);
						// }
						// 图片修改， 也需要重新组织实例数据
						rebatch = true;
						let _ = alter_drawobj.alter(r.id, InstanceSplit::ByTexture(t));
					} else if let Some(InstanceSplit::ByFrame(t)) = src.get_split() {
						// 图片修改， 也需要重新组织实例数据
						rebatch = true;
						let _ = alter_drawobj.alter(r.id, InstanceSplit::ByFrame(t));
					},
				};
			}
		}
	}
	
	if is_create {
		global_mark.mark.set(OtherDirtyType::DrawObjCreate as usize, true);
	}
	if is_delete {
		global_mark.mark.set(OtherDirtyType::DrawObjDelete as usize, true);
	}
	if rebatch {
		global_mark.mark.set(OtherDirtyType::Rebatch as usize, true);
	}
}

// 渲染实例数据
#[derive(Clone, Debug)]
pub struct RenderInstances1(pub GpuBuffer);

impl Default for RenderInstances1 {
    fn default() -> Self {
        Self(GpuBuffer::new(MeterialBind::SIZE, 0))
    }
}

lazy_static! {
	pub static ref NODE_DIRTY: StyleMarkType = style_bit()
		.set_bit(OtherDirtyType::NodeTreeAdd as usize)
		.set_bit(OtherDirtyType::NodeTreeDel as usize)
		.set_bit(OtherDirtyType::NodeTreeRemove as usize)
		.set_bit(OtherDirtyType::DrawObjCreate as usize)
		.set_bit(OtherDirtyType::DrawObjDelete as usize)
		.set_bit(OtherDirtyType::InstanceCount as usize)
		.set_bit(OtherDirtyType::PassLife as usize)
		.set_bit(OtherDirtyType::CanvasBylist as usize);
	pub static ref REBATCH_DIRTY: StyleMarkType = NODE_DIRTY.clone()
		.set_bit(OtherDirtyType::Rebatch as usize);
}

pub fn node_change(mark: &GlobalDirtyMark) -> bool {
	mark.mark.has_any(&*NODE_DIRTY)
}
pub fn rebatch_change(mark: &GlobalDirtyMark) -> bool {
	mark.mark.get(OtherDirtyType::Rebatch as usize).map_or(false, |display| {*display == true})
}

/// 创建渲染实例数据
/// 注意， 这里没考虑节点上纹理修改的问题（TODO）
#[allow(suspicious_double_ref_op)]
pub fn update_render_instance_data(
	global_mark: OrInitSingleResMut<GlobalDirtyMark>,
	query_opacity: Query<Option<&Opacity>, With<IsLeaf>>,
	mut pass_query: ParamSet<(
		Query<(&mut Draw2DList, Entity)>,
		Query<&mut Draw2DList>,
	)>,
	post_info_query: Query<(&PostProcessInfo, Option<&Root>)>,
	mut instances : OrInitSingleResMut<InstanceContext>,
	node_query: Query<(Option<&ParentPassId>, &InPassId, &DrawList, &ZRange, &IsShow, Entity, Ticker<&Layer>)>,

	mut instance_index: ParamSet<(
		Query<(&PostProcessInfo, &'static mut InstanceIndex, Entity, &mut FboInfo, &mut RenderTarget)>,
		Query<(&'static mut InstanceIndex, OrDefault<RenderCount>)>
	)>,
	mark_changed: ComponentChanged<RenderContextMark>,

	draw_info: Query<(&DrawInfo, Ticker<&RenderCount>)>,

	mut catche_buffer: OrInitSingleResMut<RenderInstances1>,
	default_sdf_uv: OrInitSingleRes<SdfUv>,
) {
	log::debug!("update_render_instance_data=====================");
	// 如果没有实体创建， 也没有实体删除， zindex也没改变，上下文结构也没改变， 则不需要更新实例数据
	// let mark_changed = query_mark.iter().next().is_some();
	let mut node_change = node_change(&global_mark);
	let instances = &mut **instances;
	log::trace!("life========================node_change={:?}， {:p}", node_change, instances);
	let p0 = instance_index.p0();
	for entity in mark_changed.iter() {
		if let Ok((post_info, mut instance_index, e, mut fbo, mut render_target)) = p0.get_mut(*entity) {
			if !post_info.has_effect() && !instance_index.is_null() {
				*fbo = FboInfo::default();
				*render_target = Default::default();
				*instance_index = Default::default();
				log::debug!("delloc pass1======================={:?}", entity);
			}
			if !node_change && post_info.has_effect() && instance_index.is_null() ||
				(!post_info.has_effect() && !instance_index.is_null())
			{
				node_change = true;
				log::debug!("node_changed6============{:?}", (e, post_info.has_effect(), instance_index.is_null()));
			}
		}
	}
	

	let mut instance_index = instance_index.p1(); 

	instances.rebatch = instances.rebatch || node_change || rebatch_change(&global_mark); // 重新批处理

	if !node_change {
		return;
	}
	log::debug!("life========================node_change={:?}, rebatch: {:?},  pass_toop_list: {:?}, {:p}", node_change, instances.rebatch, &instances.pass_toop_list, instances);
	// node_change.node_changed = false;
	// node_change.rebatch = false;
	
	let catche_buffer = &mut *catche_buffer;

	let p1 = pass_query.p1();
	// 否则，先迭代所有的drawObj,如果drawobj可见,
	for (parent_pass_id, in_pass_id, draw_list, z_range, is_show, id, layer) in node_query.iter() {
		
		// // 如果display为false， 则不需要放入渲染列表 TODO
		// if !is_show.get_display() {
		// 	continue;
		// }

		// 节点从树上移除， 删除对应例索引
		if layer.layer().is_null() {
			if layer.is_changed() {
				if let Ok((mut index, _render_count)) = instance_index.get_mut(id) {
					let index = index.bypass_change_detection();
					index.opacity = Null::null();
					index.transparent = Null::null();
				}
			
				for draw_id in draw_list.iter() {
					if let Ok((mut index, _render_count)) = instance_index.get_mut(draw_id.id) {
						let index = index.bypass_change_detection();
						index.opacity = Null::null();
						index.transparent = Null::null();
					}
				}
			}
			continue;
		}
		let mut draw_2d_list = match p1.get_mut(***in_pass_id) {
            Ok(r) => r,
            _ => continue,
        };
		// log::error!("draw info========id={:?}, is_display={:?}, in_pass_id={:?}, has_draw2d_list={:?}, draw_list={:?}, list: {:?}", id, is_show.get_display(), in_pass_id, p1.get(***in_pass_id).is_ok(),draw_list, &draw_2d_list.all_list);
		
		if draw_list.len() > 0 {
			log::debug!("draw info========id={:?}, in_pass_id={:?}, parent_pass_id={:?}, draw_list={:?}", id, in_pass_id, parent_pass_id, draw_list);
			let list = &mut *draw_2d_list;
			for draw_id in draw_list.iter() {
				let (info, render_count) = draw_info.get(draw_id.id).unwrap();
				// 渲染数量修改， 则list一定会修改
				if render_count.is_changed() {
					list.list_is_change = true;
				}
				let is_visibility = is_show.get_visibility() && is_show.get_display();
				
				if render_count.opacity > 0 {
					let mut info = info.clone();
					info.set_visibility(is_visibility);
					info.set_is_opacity(true);
					list.push_element(
						DrawIndex::DrawObj{
							draw_entity: EntityKey(draw_id.id),
							// #[cfg(debug_assertions)]
							node_entity: EntityKey(id),
						},
						z_range.clone(),
						info,
					);
				}

				if render_count.transparent > 0 {
					let mut info = info.clone();
					info.set_visibility(is_visibility);
					info.set_is_opacity(false);
					list.push_element(
						DrawIndex::DrawObj{
							draw_entity: EntityKey(draw_id.id),
							// #[cfg(debug_assertions)]
							node_entity: EntityKey(id),
						},
						z_range.clone(),
						info,
					);
				}
			}
        }
        // parent_pass_id存在，表示本节点是一个pass2d
        // if camera.is_active {
            if let Some(parent) = parent_pass_id {
                if let Ok(mut p_draw_2d_list) = p1.get_mut(parent.0) {
					log::debug!("draw info1========id={:?}, in_pass_id={:?}, parent_pass_id={:?}, draw_list={:?}", id, in_pass_id, parent_pass_id, draw_list);
					// if p_camera.is_active && p_camera.is_change {
						let mut info = DrawInfo::new(10, false);
						info.set_visibility(is_show.get_visibility() && is_show.get_display() && !layer.layer().is_null());
						p_draw_2d_list.push_element(DrawIndex::Pass2D(EntityKey(id)), z_range.clone(), info);
					// }
                }
            }
        // }
	}

	let new_instances = &mut catche_buffer.0;
	if new_instances.capacity() < instances.instance_data.cur_index() { // 扩容， 避免内存拷贝
		new_instances.data.reserve(instances.instance_data.cur_index() - new_instances.data.len());
	}

	let mut default_metrial = BatchMeterial::default();
	default_metrial.sdf_uv = [default_sdf_uv.0.left, default_sdf_uv.0.top, default_sdf_uv.0.right, default_sdf_uv.0.bottom];
	
	let mut alloc = |draw_index: &DrawIndex, draw_info: &DrawInfo, new_instances: &mut GpuBuffer, instances: &InstanceContext, instance_index: &mut Query<(&'static mut InstanceIndex, OrDefault<RenderCount>)>, pass_id: Entity| {
		let mut alloc:  Option<Entity> = None;
		// #[cfg(debug_assertions)]
		let mut node = EntityKey::null();
		let is_opacity = draw_info.is_opacity();
		match draw_index {
			DrawIndex::DrawObj{
				draw_entity, 
				// #[cfg(debug_assertions)]
				node_entity,
			} => {
				alloc = Some(draw_entity.0);
				// #[cfg(debug_assertions)]
				node = *node_entity;
			},
			DrawIndex::Pass2D(entity) => {
				if let Ok((post_info, _)) = post_info_query.get(entity.0) {
					// 后处理节点留在本层渲染末尾处理
					if post_info.has_effect() {
						// 如果存在后处理特效， 需要分配一个实例， 用于将特效拷贝到gui上
						alloc = Some(entity.0);
					} 
				}
				// #[cfg(debug_assertions)]
				node = *entity;
			},

			_ => {},
		}

		if let Some(entity) = alloc {
			// 为每一个drawObj分配新索引
			let (mut index, render_count) = instance_index.get_mut(entity).unwrap();
			let index_bypass = index.bypass_change_detection();
			let old_index = index_bypass.index(is_opacity).clone();
			let render_count = render_count.count(is_opacity) as usize;
			let new_index;
			if old_index.is_null() || old_index.len() != new_instances.alignment * render_count {
				// 不存在旧的，则分配一个新索引
				new_index = new_instances.alloc_instance_data_mult(render_count);
				let mut ty = 0;
				if !draw_info.is_visibility() {
					ty |=1 << RenderFlagType::NotVisibility as usize;
				}
				let opacity = match query_opacity.get(node.0) {
					Ok(r) => {
						ty |= 1 << RenderFlagType::Opacity as usize;
						match r {
							Some(r) => [r.0],
							None => [1.0],
						}
					},
					Err(_) => [1.0],
				};
				
				// 初始化渲染类型
				for i in 0..render_count {
					let mut instance_data = new_instances.instance_data_mut(new_index.start + i as usize * new_instances.alignment);
					instance_data.set_data(&default_metrial);
					instance_data.set_data(&TyMeterial(&[ty as f32]));
					instance_data.set_data(&OpacityUniform(opacity.as_slice())); // 初始化opacity

					// 用于debug
					// #[cfg(debug_assertions)]
					instance_data.set_data(&DebugInfo(&[node.index() as f32]));
					
				}

				log::debug!("alloc instance_index============entity={:?}, new_index={:?}, render_count={:?}", (node, entity), new_index, render_count);
				index.set_index(is_opacity, new_index.clone());

			} else {
				// 存在旧的，从旧的实例上拷贝过来
				new_index = new_instances.cur_index()..new_instances.cur_index() + render_count * new_instances.alignment;
				log::debug!("change_index============{:?}, new_index: {:?}, old_index: {:?}", (node, entity), new_index, old_index);
				if render_count > 0 {
					new_instances.extend(instances.instance_data.slice(old_index.clone()));

					if new_index.start != old_index.start || new_index.end != old_index.end {
						new_instances.update_dirty_range(new_index.clone());
					}
				}
				index_bypass.set_index(is_opacity, new_index.clone());
				
			}
			// log::trace!("life1========================insatnce_index={:?}, instance_data_start={:?}, draw_index={:?}, split={:?}, cur_index={:?}, render_count: {:?}", new_index, instance_data_start, draw_index, draw_query.get(entity), new_instances.cur_index(), render_count);
		}
	};
	let p0 = pass_query.p0();	
		
	let pass_toop_list = std::mem::take(&mut instances.pass_toop_list);
	for entity in pass_toop_list.iter() {
		let (mut draw_2d_list, _pass_id) = match p0.get_mut(*entity) {
			Ok(r) => r,
			_ => continue
		};

		let draw_2d_list = draw_2d_list.bypass_change_detection();

		draw_2d_list.shrink();

		log::debug!("draw_2d_list instance============entity: {:?}, list_is_change: {:?}, old_instance_range: {:?}, cur_index: {:?}, \nall_list: {:?}", entity, draw_2d_list.list_is_change, &draw_2d_list.instance_range, new_instances.cur_index(), draw_2d_list.all_list.as_slice());
		// 渲染列表未改变， 拷贝旧数据到新的实例数据中， 如果数据偏移发生变化， 还需要标记脏区域
		if !draw_2d_list.list_is_change {
			let instance_data_range = &draw_2d_list.instance_range;
			let mut cur_index = new_instances.cur_index();
			new_instances.extend(instances.instance_data.slice(instance_data_range.clone()));
			// 如果新的索引和原有索引不同，需要更新每个draw_obj的实例索引, 如果深度值不同， 需要更新深度值
			if cur_index != instance_data_range.start {
				new_instances.update_dirty_range(cur_index..cur_index + instance_data_range.len());
				for el in draw_2d_list.all_list_sort.iter() {
					if let DrawIndex::DrawObj{draw_entity, ..} | DrawIndex::Pass2D(draw_entity)  = &el.0 {
						let is_opacity = el.2.is_opacity();
						let (mut index, render_count) = instance_index.get_mut(draw_entity.0).unwrap();
						let end = cur_index + render_count.count(is_opacity) as usize * new_instances.alignment;
						index.set_index(is_opacity, cur_index..end);
						cur_index = end;
					};
				}
				draw_2d_list.instance_range = (new_instances.cur_index() - instance_data_range.len()) ..new_instances.cur_index();
				instances.rebatch = true; // 需要重新批处理
			}
			continue;
		} else {
			instances.rebatch = true; // 需要重新批处理
		}

		
		draw_2d_list.all_list_sort.clear();
		draw_2d_list.all_list_sort.extend_from_slice(draw_2d_list.all_list.as_slice());
		draw_2d_list.all_list_sort.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
			// let a_sort_opacity_order = a_sort.opacity_order();
			// let b_sort_opacity_order = a_sort.opacity_order();

			// if a_sort_opacity_order < b_sort_opacity_order {
			// 	return std::cmp::Ordering::Less
			// } else if a_sort_opacity_order > b_sort_opacity_order {
			// 	return std::cmp::Ordering::Greater
			// }

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

		draw_2d_list.render_list.clear();

		for i in draw_2d_list.all_list_sort.iter() {
			if i.2.opacity_order() == 0 {
				draw_2d_list.render_list.push_opaque(i.clone());
			} else {
				draw_2d_list.render_list.push_transparent(i.clone());
			}
			if i.2.is_by_cross() {
				draw_2d_list.render_list.split()
			}
		}
		draw_2d_list.render_list.split();
		// 设置all_list长度为0（数据还在，数据用于下次列表与新元素对比，来确定列表是否发生改变）
		draw_2d_list.reset();
		
		// draw_2d_list.instance_range.clear();
		// draw_2d_list.need_dyn_fbo_index.clear();

		log::trace!("life2========================{:?}, {:?}, {:?}, all_list_len: {}", entity, draw_2d_list.all_list_sort.len(), &draw_2d_list.all_list_sort, draw_2d_list.all_list.len()); 

		let instance_data_start = new_instances.cur_index();
		draw_2d_list.render_list.iter(| (draw_index, _, draw_info)| {
			alloc(draw_index, draw_info, new_instances, &instances, &mut instance_index, *entity);
		});
		// let mut pipeline;
		// for (draw_index, _, draw_info) in draw_2d_list.opaque.iter().rev().chain(draw_2d_list.transparent.iter()) {
		// 	log::debug!("draw_index============{:?}", draw_index);
		// 	alloc(draw_index, draw_info, new_instances, &instances, &mut instance_index);
		// }

		// 设置当前pass对应的实例范围（当一些节点发生改变， 而当前pass的节点未发生变动， 则根据该范围从旧的实例数据拷贝到新的实例数据）
		draw_2d_list.instance_range = instance_data_start..new_instances.cur_index();

		// }
	}

	// // 为根节点分配实例， 用于将根节点拷贝到屏幕上
	// for (root_entity, render_target_type, post_process_info, is_show, layer) in query_root1.iter() {
	// 	log::trace!("alloc root========================{:?}", root_entity); 
	// 	if post_process_info.has_effect() && RenderTargetType::Screen == *render_target_type {
	// 		// 有后处理效果， 并且最终会渲染到屏幕上， 则需要分配一个实例用于将其渲染到屏幕
	// 		let mut info = DrawInfo::new(10, false);
	// 		info.set_visibility(is_show.get_visibility() && is_show.get_display() && !layer.layer().is_null());
	// 		alloc(&DrawIndex::Pass2D(EntityKey(root_entity)), &info, new_instances, &instances, &mut instance_index, root_entity);

	// 		// 否则， 不需要这个实例渲染
	// 	} else {
	// 		let (mut index, _render_count) = instance_index.get_mut(root_entity).unwrap();
	// 		*index = InstanceIndex::default();
	// 	}
	// }
	// // 分配清屏所需实例（清屏需要批渲染，因此将其分配在一起）
	// for entity in pass_toop_list.iter() {
	// 	let (mut draw_2d_list, pass_id) = match p0.get_mut(*entity) {
	// 		Ok(r) => r,
	// 		_ => continue
	// 	}; 
	// 	match post_info_query.get(pass_id) {
	// 		Ok((post_info, layer)) if post_info.has_effect() || layer.is_some() => {
	// 			// 清屏数据
	// 			let index = if !draw_2d_list.clear_instance.is_null() {
	// 				let cur_index = new_instances.cur_index();
					
	// 				new_instances.extend(instances.instance_data.slice(draw_2d_list.clear_instance..draw_2d_list.clear_instance + new_instances.alignment));
	// 				if cur_index != draw_2d_list.clear_instance {
	// 					let end = new_instances.cur_index();
	// 					new_instances.update_dirty_range(cur_index..end);
	// 				}
	// 				cur_index
	// 			} else {
	// 				set_clear_screen_instance(&CgColor::new(0.0, 0.0, 0.0, 0.0), new_instances)
	// 			};
	// 			// 分配清屏数据
	// 			// draw_2d_list.clear_instance = index;
	// 			draw_2d_list.need_render_pass = true;
	// 		},
	// 		_ => {
	// 			// 不清屏
	// 			// draw_2d_list.clear_instance = pi_null::Null::null();
	// 			draw_2d_list.need_render_pass = false;
	// 		}
	// 	}
	// }
	instances.pass_toop_list = pass_toop_list;

	instances.instance_data.clear();
	// 用新的实例数据替换旧的实例数据
	std::mem::swap(&mut instances.instance_data, &mut *new_instances);

	log::debug!("len============={:?}, {:?}", instances.instance_data.cur_index(), &instances.pass_toop_list );
}


/// 批处理实例
/// 在渲染图的build之后， 渲染之前运行
/// 只将需要渲染的节点节点批处理
/// 按照pass_toop_list的顺序，批处理实例数据
/// 批处理draw列表顺序形如： [
/// DrawClear(pass0, pass1..)
/// PassDrawList(pass0)
/// PassDrawList(pass1)
/// ...
/// DrawClear(pass2, pass3)
/// PassDrawList(pass2)
/// PassDrawList(pass3)
/// ]
pub fn batch_instance_data(
	mut query: BatchQuery,
	query_root: Query<(Entity, &InstanceIndex), With<Root>>, // 只有gui的Root才会有Size
	mut query_camera: Query<&'static mut Camera>,
	mut instances : OrInitSingleResMut<InstanceContext>,
) {
	

	let instances = &mut **instances;
	// println!("batch_instance_data=========={:?}", instances.rebatch);
	log::debug!("batch_instance_data, rebatch={:?}, {:p}", instances.rebatch, instances);
	if !instances.rebatch {
		return;
	}
	log::debug!("batch_instance_data, pass_toop_list={:?}", &instances.pass_toop_list);
	instances.draw_list.clear();
	instances.posts.clear();
	instances.rebatch = false;

	// #[cfg(debug_assertions)]
	// instances.debug_info.clear();
	

	let mut global_state = BatchGlobalState {
		post_start: 0,
		pre_group: 0,
		last_group: None,
		last_fbo: None,
		pre_is_single_split: false,
	};

	// 当前剩余未批处理的数据合批
	// 将排序好的绘制对象劈分成多段, 劈分规则为：
	// 1. 绘制为DrawIndex::Pass2D类型，其直接成为一个劈分点， 把该点的《前面部分》，《自身》，《剩余部分》劈分成三段（剩余部分可能继续被劈分）
	// 2. 如果DrawObject存在UiTexture，由于着色器一次最多接收16个纹理， 因此根据当前纹理是否超出16个为一个新的劈分点，将《前一部分》，《自身和后续部分》劈分成两段
	// 3. pipeline不同，会将《前一部分》，《自身和后续部分》劈分成两段
	let mut batch_state = BatchRootState {
		next_node_with_depend: instances.next_node_with_depend.get(0).map_or(std::usize::MAX, |r| {*r}),
		// toop_list_len: root_instance.pass_toop_list.len(),  // pass的最大数量
		pre_pipeline: instances.default_pipelines.common_pipeline.clone(),
		// next_node_with_depend_list: &root_instance.next_node_with_depend,
		next_node_with_depend_index: 0,
	};
	
	let pass_toop_list = std::mem::take(&mut instances.pass_toop_list);
	log::debug!("pass_toop_list!!!!!===={:?}", pass_toop_list);
	

	let mut start_draw_index = instances.draw_list.len();

	for (pass_index, pass_id) in pass_toop_list.iter().enumerate() {
		let pass_index = pass_index + 1;

		let mut draw_2d_list= match query.pass_query.get_mut(*pass_id) {
			Ok(r) => r,
			_ => continue
		};
		let (_, _, fbo_info, _render_target) = query.draw_query.get(*pass_id).unwrap();
		

		let mut fbo_changed = false;
		let draw_2d_list = draw_2d_list.bypass_change_detection();
		let mut camera = query_camera.get_mut(*pass_id).unwrap();

		let (is_fbo, is_root) = if let Ok((post_info, layer)) = query.post_info_query.get(*pass_id) {
			(post_info.has_effect(), layer.root() == *pass_id) 
		} else {
			(false, false)
		};

		if is_fbo || is_root {
			// 如果pass需要清屏
			match (&fbo_info.fbo, &global_state.last_fbo){
				(Some(r), Some(r1)) => {
					if !Share::ptr_eq(&r.target().colors[0].0, &r1.target().colors[0].0) {
						global_state.last_fbo = Some(r.clone());
						fbo_changed = true;
					}
				},
				_ => (),
			};	

			let is_render_own = camera.is_render_own;
			// let active_changed = if is_render_own != global_state.is_active {
			// 	global_state.is_active = is_render_own;
			// 	true
			// } else {
			// 	false
			// };
			if is_render_own && fbo_changed {
				// 如果fbo发生改变， 需要结束之前的纹理批处理， 避免出现渲染源和目标冲突
				take_group(&mut instances.batch_texture, &query.device, global_state.pre_group, &mut instances.draw_list, global_state.pre_is_single_split);
				global_state.pre_group = instances.draw_list.len();
			}

			if is_fbo {
				instances.posts.push(*pass_id);// 后处理节点留在本层渲染末尾处理
			}
			// 当需要渲染时， 才需要劈分数据
			if is_render_own {
				// let mut instance_data_start = draw_2d_list.instance_range.start;
				// let mut instance_data_end =  draw_2d_list.instance_range.start;
				let mut draw_2d_list = std::mem::take(draw_2d_list);
				log::debug!("pass_index================{:?}", (pass_index, pass_id, &draw_2d_list));
				
				batch_pass(&mut query, &mut batch_state, &mut global_state, instances, &mut draw_2d_list, *pass_id, *pass_id);
				batch_depth(&mut query, instances, &mut draw_2d_list, &mut 1);

				// 还回列表
				let mut draw_2d_list1= match query.pass_query.get_mut(*pass_id) {
					Ok(r) => r,
					_ => unreachable!()
				};
				*(draw_2d_list1.bypass_change_detection()) = draw_2d_list;
			}

		}
		
		// 已经到达下一个"有依赖未就绪"的节点
		// 在draw_list中push一个DrawPost， 用于绘制当前需要绘制的后处理效果
		if pass_index >= batch_state.next_node_with_depend {
			if pass_index >= batch_state.next_node_with_depend {
				batch_state.next_node_with_depend_index += 1;
				batch_state.next_node_with_depend = instances.next_node_with_depend.get(batch_state.next_node_with_depend_index).map_or(std::usize::MAX, |r| {*r});
				if global_state.post_start < instances.posts.len() {
					log::trace!("DrawPost====={:?}, index={}, {:?}, {:?}, {:?}, {:?}", pass_id, instances.draw_list.len(), pass_index, batch_state.next_node_with_depend, global_state.post_start..instances.posts.len(), instances.next_node_with_depend);
					let post = (DrawElement::DrawPost(global_state.post_start..instances.posts.len()), *pass_id);
					instances.draw_list.push(post);
					global_state.post_start = instances.posts.len();
				}
				camera.bypass_change_detection().draw_range = start_draw_index..instances.draw_list.len();
				start_draw_index = instances.draw_list.len();
			}
			
			// 如果处理了当前层的后处理， group需要重新生成（不能确定后处理的fbo的依赖关系）
			take_group(&mut instances.batch_texture, &query.device, global_state.pre_group, &mut instances.draw_list, global_state.pre_is_single_split);
			global_state.pre_group = instances.draw_list.len();
			// // 最后一个Pass, 需要设置前面批数据的texture_bind_group
			// if pass_index == batch_state.toop_list_len {
				
			// }
		}
		
		// draw_2d_list.draw_list.push();
		// max_depth_count = (start..cursor).len().max(max_depth_count);

		// instance_data_start = instance_data_end;
	}
	instances.pass_toop_list = pass_toop_list;

	for (root, instance_index) in query_root.iter() {
		if !instance_index.transparent.is_null() {

			let (_, _, _fbo_info, render_target) = query.draw_query.get(root).unwrap();
			if let Some(target) = &render_target.0 {
				let texture = &target.target().colors[0].0;
				let (texture_index, group, name) = instances.batch_texture.push(BatchTextureItem::Fbo(texture.clone()), &query.common_sampler.pointer, &query.device);
				instances.instance_data.instance_data_mut(instance_index.transparent.start).set_data(&TetxureIndexMeterial(&[texture_index as f32])); // 设置drawobj的纹理索引
				if let Some(group) = group {
					let group = Share::new(group);
					// 设置之前的批渲染的纹理group
					for i in global_state.pre_group..instances.draw_list.len() {
						if let DrawElement::DrawInstance { draw_state, .. } = &mut instances.draw_list[i].0 {
							log::debug!("texture flow group============={:?}, {:?}, {:p}", global_state.pre_group, &draw_state.instance_data_range, &*group);
							draw_state.texture_bind_group = Some(group.clone());
							#[cfg(debug_assertions)]
							{
								draw_state.texture_bind_group_type = name;
							}

						}
						global_state.pre_group = instances.draw_list.len();
					}
				}
			}

			let p = instances.default_pipelines.copy_pipeline.clone();
			// instance_index.start不为null， 则需要将对应的fbo渲染到屏幕上
			instances.draw_list.push((DrawElement::DrawInstance {
				draw_state: InstanceDrawState { 
					instance_data_range: instance_index.transparent.start..instance_index.transparent.end, 
					pipeline: Some(p),
					texture_bind_group: None,
					#[cfg(debug_assertions)]
					pipeline_type: "copy_pipeline",
					#[cfg(debug_assertions)]
					texture_bind_group_type: "none",
				},
				depth_start: 0,
				draw_range: 0..0,
				pass: root,
			}, EntityKey::null().0));

		}
		
	}
	instances.draw_screen_range =  start_draw_index..instances.draw_list.len();
	
	// 最后一个Pass, 需要设置前面批数据的texture_bind_group
	take_group(&mut instances.batch_texture, &query.device, global_state.pre_group, &mut instances.draw_list, true);
}

fn select_pipeline<'a>(is_opacity: bool, is_fbo: bool, pipeline: Option<&'a Pipeline>, default_pipeline: &'a DefaultPipelines) -> &'a Share<wgpu::RenderPipeline> {
	if let Some(pipeline) = pipeline {
		pipeline.value(is_opacity)
	} else if is_opacity{
		if is_fbo {
			&default_pipeline.common_fbo_opacity_pipeline
		} else {
			&default_pipeline.common_opacity_pipeline
		}
		
	} else {
		if is_fbo { 
			&default_pipeline.common_fbo_pipeline
		}  else {
			&default_pipeline.common_pipeline
		}
	}
}

fn take_group(batch_texture: &mut BatchTexture, device: &wgpu::Device, pre_group: usize, draw_list: &mut Vec<(DrawElement, Entity/*fbo passid*/)>, is_single: bool) {
	let (group, name) =	match batch_texture.take_group(device) {
		(Some(group), name) => {
			let r = Share::new(group);
			log::debug!("some==================={:p}", &*r);
			(Some(r), name)
		},
		(None, _) => {
			let r = batch_texture.default_group(is_single);
			log::debug!("default==================={is_single}, {:p}", &*r);
			(Some(r), if is_single{"single texture bindgroup"} else {"batch texture bindgroup"})
		},
	};
	for i in pre_group..draw_list.len() {
		if let DrawElement::DrawInstance { draw_state, .. } = &mut draw_list[i].0 {
			if let Some(group) = &group {
				log::debug!("set_group==================={:?}, {:p}", (pre_group, &draw_state.instance_data_range), &**group);
			}
			
			draw_state.texture_bind_group = group.clone();
			#[cfg(debug_assertions)]
			{draw_state.texture_bind_group_type = name;}
		}
	}
}


/// 更新深度， 返回消耗的深度空间
pub fn update_depth(
	depth_count: &mut usize,
	instances: &mut InstanceContext,
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
			DrawElement::GraphDrawList { .. } => {
				// let depth = calc_depth(*depth_count);

				// let mut count = 1;
				// if let Ok((mut depth_range, list)) = render_cross_query.get_mut(**id) {
				// 	count = (list.require_depth / DEPTH_SPACE).ceil() as usize;
				// 	if depth_range.start != depth || depth + list.require_depth != depth_range.end { // 修改深度范围
				// 		depth_range.start = depth;
				// 		depth_range.end = depth + list.require_depth;
				// 	}
				// }
				*depth_count += 1;
			},
			DrawElement::DrawPost(_) => (),
			// DrawElement::Clear { .. } => (),
		}
	}
}

pub fn calc_depth(index: usize) -> f32 {
	index as f32 * DEPTH_SPACE
}

fn batch_depth(
	query: &mut BatchQuery,
	instances: &mut InstanceContext,
	draw_list: &mut Draw2DList,
	// all_list_sort: &Vec<(DrawIndex, ZRange, DrawInfo)>, 
	// instance_data_start: &mut usize,  
	// instance_data_end: &mut usize,

	depth_count: &mut usize,
) {
	for (draw_index, _, draw_info) in draw_list.all_list_sort.iter() {
		let is_opacity = draw_info.is_opacity();
		match draw_index.clone() {
			DrawIndex::DrawObj{ 
				draw_entity, 
				..
			} => {
				// 为每一个drawObj分配新索引
				let index = query.instance_index.get_mut(draw_entity.0).unwrap();
				instances.instance_data.set_data_mult1(index.index(is_opacity).clone(), &DepthUniform(&[calc_depth(*depth_count)]));// 设置drawobj的深度
				*depth_count +=1;
			},
				
			DrawIndex::Pass2D(r, ..) => match query.post_info_query.get(r.0) {
				Ok((post_info, _)) if  post_info.has_effect() => {
					let index = query.instance_index.get_mut(r.0).unwrap();
					instances.instance_data.set_data_mult1(index.index(is_opacity).clone(), &DepthUniform(&[calc_depth(*depth_count)]));// 设置drawobj的深度
					*depth_count +=1;
				},
				_ => {
					let mut draw_2d_list = match query.pass_query.get_mut(*r) {
						Ok(r) => r,
						_ => continue
					};
					let draw_2d_list = draw_2d_list.bypass_change_detection();
					let mut draw_2d_list = std::mem::take(draw_2d_list);
					batch_depth(query, instances, &mut draw_2d_list, depth_count);
					let mut draw_2d_list1: pi_world::fetch::Mut<'_, Draw2DList>= match query.pass_query.get_mut(*r) {
						Ok(r) => r,
						_ => continue
					};
					*(draw_2d_list1.bypass_change_detection()) = draw_2d_list;
					continue;
				},
			},
			_ => (),
		}
	}
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
	log::debug!("pass_toop_list!!!!!3333===={:?}", (pass_id, instances.draw_list.len()));
	let mut start = 0;
	let mut cursor = 0;

	let mut instance_data_start = draw_list.instance_range.start;
	let mut instance_data_end =  draw_list.instance_range.start;

	// let mut pipeline;
	draw_list.render_list.iter(|(draw_index, _, draw_info)| {
		let mut last_pipeline = None;
		let mut split_by_texture:  Option<(std::ops::Range<usize>, BatchTextureItem, &Share<wgpu::Sampler>)> = None;
		let mut instance_data_end1 = instance_data_end;
		let mut cross_list: Option<EntityKey> = None;
		let is_opacity = draw_info.is_opacity();
		let mut is_single_texture = false;
		let cur_pipeline  = match draw_index.clone() {
			DrawIndex::DrawObj{ 
				draw_entity, 
				..
			} => if let Ok((instance_split, pipeline, _fbo_info, render_target)) = query.draw_query.get(*draw_entity) {
				// 为每一个drawObj分配新索引
				let index = query.instance_index.get_mut(draw_entity.0).unwrap().index(is_opacity);
				instance_data_end1 = instance_data_end;
				instance_data_end = index.end;
				let mut cur_pipeline = select_pipeline(is_opacity, false, pipeline, &instances.default_pipelines);
				log::debug!("DrawIndex::DrawObj======pass_id: {:?}, instance_split: {:?}", pass_id, instance_split);

				if let Some(instance_split) = instance_split {
					match instance_split {
						InstanceSplit::ByEntity(entity) => if let Ok((_instance_split, _pipeline, _fbo_info, render_target)) = query.draw_query.get(entity.clone()) {
							// as_image
							if let Some(t) = &render_target.0 {
								log::debug!("ByEntity======{:?}", (pass_id, entity, &t.target().colors[0].0));
								split_by_texture = Some(((*index).clone(), BatchTextureItem::Fbo(t.target().colors[0].0.clone()), &query.common_sampler.pointer));
								cur_pipeline = select_pipeline(is_opacity, true, pipeline, &instances.default_pipelines);
							}
						},
						InstanceSplit::ByTexture(ui_texture) => {
							// #[cfg(debug_assertions)]
							// if !index.start.is_null() {
							// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("image: {:?}", draw_entity));
							// }
							// if node_entity.index() == 159 {
								// println!("split_by_texture=======node_entity:{:?}, draw_entity:{:?}, {:?}, {:?}", node_entity, draw_entity,  ui_texture.id, a.1);
							// }
							log::debug!("ByTexture======{:?}", draw_entity);
							split_by_texture = Some(((*index).clone(), BatchTextureItem::Texture(ui_texture.clone()), &query.common_sampler.default));
						},
						InstanceSplit::ByFrame(ui_texture) => {
							// #[cfg(debug_assertions)]
							// if !index.start.is_null() {
							// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("image: {:?}", draw_entity));
							// }
							// if node_entity.index() == 159 {
								// println!("split_by_texture=======node_entity:{:?}, draw_entity:{:?}, {:?}, {:?}", node_entity, draw_entity,  ui_texture.id, a.1);
							// }
							log::debug!("ByFrame======draw_entity: {:?}, frame: {:?}", &draw_entity, ui_texture.texture().texture);
							split_by_texture = Some(((*index).clone(), BatchTextureItem::Frame(ui_texture.clone()), &query.common_sampler.default));

							if ui_texture.frame().is_none() {
								is_single_texture = true;
								// blendMod, TODO
								cur_pipeline = select_pipeline(is_opacity, true, pipeline, &instances.default_pipelines);
							}
						},
						InstanceSplit::ByFbo(ui_texture) => {
							// #[cfg(debug_assertions)]
							// if !index.start.is_null() {
							// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("image: {:?}", draw_entity));
							// }
							// if node_entity.index() == 159 {
								// println!("split_by_texture=======node_entity:{:?}, draw_entity:{:?}, {:?}, {:?}", node_entity, draw_entity,  ui_texture.id, a.1);
							// }
							log::debug!("ByFbo=========={:?}", (pass_id, draw_entity, &ui_texture.as_ref().unwrap().target().colors[0].1));
							split_by_texture = Some((index.clone(), BatchTextureItem::Fbo(ui_texture.as_ref().unwrap().target().colors[0].0.clone()), &query.common_sampler.default));
							cur_pipeline = select_pipeline(is_opacity, true, pipeline, &instances.default_pipelines);
							is_single_texture = true;
						},
						InstanceSplit::ByCross(id, is_list) =>  {
							if *is_list {
								log::debug!("ByCross draw_list==========pass_id: {:?}, entity: {:?}", pass_id, id);
								cross_list = Some(EntityKey(*id));
								// is_list为true时， 必须劈分
								last_pipeline = Some(root_state.pre_pipeline.clone())
							} else {
								// 设置实例是否需要还原预乘
								let mut ty = instances.instance_data.instance_data_mut(index.start).get_render_ty();
								
								// log::error!("pipeline================{:?}", (draw_entity, pipeline.is_some()));
								match pipeline{
									Some(r) if !Share::ptr_eq(&r.transparent, &instances.default_pipelines.fbo_premultiply_pipeline) => ty &= !(1 << RenderFlagType::Premulti as usize),
									_ => ty |= 1 << RenderFlagType::Premulti as usize,
								};
								let mut instance_data = instances.instance_data.instance_data_mut(index.start);
								instance_data.set_data(&TyMeterial(&[ty as f32]));
								if let Some(r) = &render_target.0 {
									split_by_texture = Some((index.clone(), BatchTextureItem::Fbo(r.target().colors[0].0.clone()), &query.common_sampler.pointer)); // TODO， 根据纹理尺寸目标尺寸选择混合模式
								}
								log::debug!("ByCross fbo==========pass_id: {:?}, entity: {:?}", pass_id, id);
								is_single_texture = true;

								// #[cfg(debug_assertions)]
								// if !index.start.is_null() {
								// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("canvas: {:?}", draw_entity));
								// }
							}
						},
						
					}
				} else {
					log::debug!("ByNone=========={:?}", draw_entity);
					// #[cfg(debug_assertions)]
					// if !index.start.is_null() {
					// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("node: {:?}, draw: {:?}", node_entity, draw_entity));
					// }
					
				}

				cur_pipeline
			} else if is_opacity {
				is_single_texture = true;
				&instances.default_pipelines.common_fbo_opacity_pipeline
			} else {
				is_single_texture = true;
				&instances.default_pipelines.common_fbo_pipeline
			},
			DrawIndex::Pass2D(cur_pass ) => match query.post_info_query.get(cur_pass.0) {
				Ok((post_info, _)) if  post_info.has_effect() => {
					log::debug!("ByPass Fbo=========={:?}", cur_pass);
					let (_, _, _fbo_info, render_target) = query.draw_query.get(cur_pass.0).unwrap();
					let camera = query.camera_query.get(cur_pass.0).unwrap();
					let index = query.instance_index.get_mut(cur_pass.0).unwrap().index(is_opacity);
					instance_data_end1 = instance_data_end;
					instance_data_end = index.end;
					if let Some(r) = &render_target.0 {
						if camera.is_render_to_parent { 
							// 如果是fbo， 必须可以可以渲染到父，才能设置texture， 否则，该节点不会作为图节点输出到下一个依赖， 可能导致纹理既作为源又作为目标
							log::debug!("pass=========={:?}", (pass_id, cur_pass, index.start/224, &r.target().colors[0].1));
							split_by_texture = Some((index.clone(), BatchTextureItem::Fbo(r.target().colors[0].0.clone()), &query.common_sampler.pointer)); // fbo拷贝使用点采样

							// debug版本， 判断纹理是否冲突
							#[cfg(debug_assertions)]
							{
								let (_, _, parent_fbo_info, _) = query.draw_query.get(parent_pass_id).unwrap();
								
								if parent_fbo_info.fbo.is_none() {
									log::debug!("parent texture none, parent_pass_id: {:?}, cur_pass: {:?}", parent_pass_id, cur_pass.0);
								} else  if r.target().colors[0].0.id == parent_fbo_info.fbo.as_ref().unwrap().target().colors[0].0.id {
									log::error!("texture conflicting, {:?}, {:?}", (cur_pass.0, parent_pass_id), (r.target().colors[0].0.id, parent_fbo_info.fbo.as_ref().unwrap().target().colors[0].0.id));
								}
							}
						}

						// #[cfg(debug_assertions)]
						// if !index.start.is_null() {
						// 	instances.debug_info.insert(index.start / MeterialBind::SIZE, format!("pass:{:?}", r));
						// }
						// #[cfg(debug_assertions)]
						// instances.instance_data_mut(index.start + i as usize * index.alignment).set_data(&DebugInfo(&[r.0.index() as f32]));
					}
					log::debug!("pass pipeline=========={:?}", pass_id);
					is_single_texture = true;
					// instances.posts.push(*r);// 后处理节点留在本层渲染末尾处理
					if is_opacity {
						&instances.default_pipelines.common_fbo_opacity_pipeline // 可能是不透明的？
					} else {
						&instances.default_pipelines.common_fbo_pipeline // 可能是不透明的？
					}
				},
				_ => {
					log::debug!("ByPass Not Fbo=========={:?}", cur_pass);
					// 将当前剩余未批处理的数据合批(没有fbo的Passd， 相机发生改变，需要分批)
					if instance_data_start < instance_data_end {
						instances.draw_list.push((DrawElement::DrawInstance {
							draw_state: InstanceDrawState { 
								instance_data_range: instance_data_start..instance_data_end, 
								pipeline: Some(root_state.pre_pipeline.clone()),
								texture_bind_group: None,
								#[cfg(debug_assertions)]
								pipeline_type: pipeline_type(&root_state.pre_pipeline, instances),
								#[cfg(debug_assertions)]
								texture_bind_group_type: "none",
							},
							depth_start: 0,
							draw_range: start..cursor,
							pass: pass_id,
						}, parent_pass_id));
						instance_data_start = instance_data_end;
					}

					let mut draw_2d_list = match query.pass_query.get_mut(*cur_pass) {
						Ok(r) => r,
						_ => return
					};
					let draw_2d_list = draw_2d_list.bypass_change_detection();
					let mut draw_2d_list = std::mem::take(draw_2d_list);
					batch_pass(query, root_state, global_state, instances, &mut draw_2d_list, *cur_pass, parent_pass_id);
					let mut draw_2d_list1= match query.pass_query.get_mut(*cur_pass) {
						Ok(r) => r,
						_ => return
					};
					*(draw_2d_list1.bypass_change_detection()) = draw_2d_list;
					return;
				},
			},
			_ => {
				log::debug!("ByPass Other==========");
				is_single_texture = global_state.pre_is_single_split;
				&root_state.pre_pipeline
			},
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
		
		log::debug!("b================{:?}", (global_state.pre_is_single_split, is_single_texture, instance_data_start, instance_data_end1, global_state.pre_group, instances.draw_list.len()));
		// 将前一部分劈分出去
		if let Some(p)= &last_pipeline {
			if instance_data_end1 > instance_data_start {
				instances.draw_list.push((DrawElement::DrawInstance {
					draw_state: InstanceDrawState { 
						instance_data_range: instance_data_start..instance_data_end1, 
						pipeline: Some(p.clone()),
						texture_bind_group: None,
						#[cfg(debug_assertions)]
						pipeline_type: pipeline_type(p, instances),
						#[cfg(debug_assertions)]
						texture_bind_group_type: "none",
					},
					depth_start: 0,
					draw_range: start..cursor,
					pass: pass_id,
				}, parent_pass_id));
				// draw_2d_list.draw_list.push();
				// max_depth_count = (start..cursor).len().max(max_depth_count);

				instance_data_start = instance_data_end1;
				start = cursor;
			}
			if is_single_texture != global_state.pre_is_single_split {
				log::debug!(" take_group a================{:?}", (is_single_texture, instance_data_start, instance_data_end1));
				// 纹理类型发生改变， take_group
				take_group(&mut instances.batch_texture, &query.device, global_state.pre_group, &mut instances.draw_list, global_state.pre_is_single_split);
				global_state.pre_group = instances.draw_list.len();
				global_state.pre_is_single_split = is_single_texture;
			}
		}

		// 其他框架提供的渲染列表
		if let Some(draw_entity) = cross_list {
			instances.draw_list.push((DrawElement::GraphDrawList{ 
				id: draw_entity, 
				pass: pass_id,
				depth_start: 0.0
			}, parent_pass_id));
			start = cursor + 1;
			instance_data_end1 = instance_data_end;
			instance_data_start = instance_data_end; // 渲染实例跳过当前实例不渲染（通过drawlist的方式渲染， 不需要将fbo拷贝到gui上了）
		}

		// 添加渲染所需纹理， 如果纹理溢出， 需要结束批处理
		if let Some((index, texture, sampler)) = split_by_texture {
			let (texture_index, group, name) = instances.batch_texture.push(texture, sampler, &query.device);
			instances.instance_data.set_data_mult(index.start, (index.end-index.start)/instances.instance_data.alignment, &TetxureIndexMeterial(&[texture_index as f32]));// 设置drawobj的纹理索引
			if let Some(group) = group {
				let group = Share::new(group);
				if instance_data_end1 > instance_data_start {
					// batch_texture中纹理已经超出16个，因此需要劈分
					instances.draw_list.push((DrawElement::DrawInstance {
						draw_state: InstanceDrawState { 
							#[cfg(debug_assertions)]
							pipeline_type: pipeline_type(&match &last_pipeline {
								Some(r) => r.clone(),
								None => root_state.pre_pipeline.clone(),
							}, instances),
							#[cfg(debug_assertions)]
							texture_bind_group_type: "none",

							instance_data_range: instance_data_start..instance_data_end1, 
							texture_bind_group: Some(group.clone()),
							pipeline: match last_pipeline {
								Some(r) => Some(r),
								None => Some(root_state.pre_pipeline.clone()),
							},
						},
						depth_start: 0,
						draw_range: start..cursor,
						pass: pass_id,
					}, parent_pass_id));
					global_state.last_group = Some(group.clone());
					// max_depth_count = (start..cursor).len().max(max_depth_count);
					// instances.extend_count(cursor - start);
					instance_data_start = instance_data_end1;
					start = cursor;
				}
				// 设置之前的批渲染的纹理group
				for i in global_state.pre_group..instances.draw_list.len() {
					if let DrawElement::DrawInstance { draw_state, .. } = &mut instances.draw_list[i].0 {
						log::debug!("texture flow group============={:?}, {:p}", global_state.pre_group, &*group);
						draw_state.texture_bind_group = Some(group.clone());
						#[cfg(debug_assertions)]
						{draw_state.texture_bind_group_type = name;}
					}
				}
				global_state.pre_group = instances.draw_list.len();
			}
		};


		
		cursor += 1;
	});	

	// 将当前剩余未批处理的数据合批
	if instance_data_start < instance_data_end {
		instances.draw_list.push((DrawElement::DrawInstance {
			draw_state: InstanceDrawState { 
				instance_data_range: instance_data_start..instance_data_end, 
				pipeline: Some(root_state.pre_pipeline.clone()),
				texture_bind_group: None,
				#[cfg(debug_assertions)]
				pipeline_type: pipeline_type(&root_state.pre_pipeline, instances),
				#[cfg(debug_assertions)]
				texture_bind_group_type: "none",
			},
			depth_start: 0,
			draw_range: start..cursor,
			pass: pass_id,
		}, parent_pass_id));
	}
}

fn pipeline_type(cur_pipeline: &Share<wgpu::RenderPipeline>, instances: &InstanceContext) -> &'static str {
	if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.common_fbo_pipeline) {
		"common_fbo_pipeline"
	} else if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.common_fbo_opacity_pipeline) {
		"common_fbo_opacity_pipeline"
	} else if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.common_opacity_pipeline) {
		"common_opacity_pipeline"
	} else if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.common_pipeline) {
		"common_pipeline"
	} else if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.premultiply_pipeline) {
		"premultiply_pipeline"
	} else if Share::ptr_eq(cur_pipeline, &instances.default_pipelines.fbo_premultiply_pipeline) {
		"fbo_premultiply_pipeline"
	} else {
		"other"
	}
}

#[derive(SystemParam)]
pub struct BatchQuery<'w> {
	pass_query: Query<'w, &'static mut Draw2DList>,
	post_info_query: Query<'w, (&'static PostProcessInfo, &'static Layer)>,
	draw_query: Query<'w, (Option<&'static InstanceSplit>, Option<&'static Pipeline>, OrDefault<FboInfo>, OrDefault<RenderTarget>)>,
	camera_query:  Query<'w, &'static Camera>,
	instance_index: Query<'w, &'static InstanceIndex>,
	common_sampler: OrInitSingleRes<'w,CommonSampler>,
	device: SingleRes<'w,PiRenderDevice>,

	// #[cfg(debug_assertions)]
	// debug_node_query: Query<'w, 's, (
	// 	Option< &'static BackgroundColor>,
	// 	Option< &'static BackgroundImage>,
	// 	Option< &'static BorderImage>,
	// 	Option< &'static Canvas>,
	// 	Option< &'static BlendMode>,
	// 	Option< &'static TextContent>,
	// 	// Option< &'static Svg>,
	// )>,

	// #[cfg(debug_assertions)]
	// debug_draw_query: Query<'w, 's, (
	// 	Option< &'static BackgroundColorMark>,
	// 	Option< &'static BackgroundImageMark>,
	// 	Option< &'static BorderImageMark>,
	// 	Option< &'static CanvasMark>,
	// 	Option< &'static TextMark>,
	// 	Option< &'static TextShadowMark>,
	// 	Option< &'static BoxShadowMark>,
	// )>,
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
	pre_is_single_split: bool,
	// 在按顺序进行批处理的过程中，相邻pass分配的fbo可能相同也可能不同，当不同时， 记录当前遍历到的最新的pass的fbo
	// 这用于比较， 当前处理的pass是否切换了fbo
	// 当前对fbo的清屏操作，是批量清理多个区域，因此，当fbo切换时，clear实例需要放入新的批次
	last_fbo: Option<ShareTargetView>,
}


const DEPTH_SPACE: f32 = 0.0001;

// fn set_clear_screen_instance(color: &CgColor, instances: &mut GpuBuffer) -> usize{
// 	let new_index = instances.alloc_instance_data();
// 	log::debug!("alloc clear_screen instance_index============{:?}", new_index);
// 	let mut instance_data = instances.instance_data_mut(new_index);

// 	let mut batch_meterial = BatchMeterial::default();
// 	batch_meterial.color = [color.x, color.y, color.z, color.w];

// 	instance_data.set_data(&batch_meterial);
// 	instance_data.set_data(&WorldMatrixMeterial(WorldMatrix::default().as_slice()));
// 	instance_data.set_data(&DepthUniform(&[0.0]));

// 	instance_data.set_data(&LayoutUniform(&[0.0, 0.0, -1.0, -1.0]));
// 	let render_flag = 1 << RenderFlagType::IgnoreCamera as usize;
// 	instance_data.set_data(&TyMeterial(&[render_flag as f32]));
// 	// instance_data.set_data(&QuadUniform(&[
// 	// 	-1.0, 1.0,
// 	// 	-1.0, -1.0,
// 	// 	1.0, -1.0,
// 	// 	1.0, 1.0,
// 	// ]));

// 	new_index
// }
