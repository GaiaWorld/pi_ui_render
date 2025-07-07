//! 计算show
//! 该系统默认为所有已经创建的Entity创建Show组件， 并监听Show和Display的创建修改， 以及监听idtree上的创建事件， 计算已经在idtree上///! 存在的实体的Enable和Visibility

use pi_style::style::StyleType;
use pi_world::{event::{ComponentAdded, ComponentChanged, Event}, filter::With, prelude::{IntoSystemConfigs, OrDefault, Plugin, Query}, single_res::SingleRes, world::Entity};
use pi_bevy_ecs_extend::prelude::{EntityTree, Layer, LayerDirty, OrInitSingleRes, OrInitSingleResMut, Up};

use pi_flex_layout::style::Display;
use pi_null::Null;

use crate::{components::{
    calc::{style_bit, DrawList, IsShow, StyleBit, StyleMarkType}, draw_obj::InstanceIndex, pass_2d::Draw2DList, user::{Enable, Show}
}, resource::{draw_obj::InstanceContext, GlobalDirtyMark, IsRun, OtherDirtyType}, shader1::batch_meterial::{RenderFlagType, TyMeterial}, system::{base::draw_obj::life_drawobj::update_render_instance_data, system_set::UiSystemSet}};

use crate::prelude::UiStage;

use super::{user_setting::{AddEvent, RemoveEvent}, world_matrix::Empty};
pub struct ShowPlugin;

impl Plugin for ShowPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<NodeVisibilityChange>()
			.add_system(UiStage, calc_show.in_set(UiSystemSet::BaseCalc).run_if(show_change).in_set(UiSystemSet::IsRun))
			.add_system(UiStage, 
				set_show_data
					.run_if(show_data_change)
					.after(update_render_instance_data)
					.after(UiSystemSet::PrepareDrawObj) // 这里是为了确保与其他设置实例数据的system不并行， 因为设置的数据冲突（TyUniform）
					.in_set(UiSystemSet::IsRun)
					)
		;
	}
}

#[derive(Debug, Default)]
pub struct ShowDirty(pub Vec<Entity>);

/// 计算节点的显示属性
pub fn calc_show(
	show_changed: ComponentChanged<Show>,
	show_added: ComponentAdded<Show>,
	add_events: Event<AddEvent>,
    remove_events: Event<RemoveEvent>,
    mut layer_dirty: LayerDirty<With<Empty>>,
	tree: EntityTree,
	
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
    // mut dirty: LayerDirty<(Changed<Layer>, Changed<Show>)>,
    query: Query<(OrDefault<Show>, Option<&Up>)>,
    mut write: Query<&mut IsShow>,

	mut is_show_changed: OrInitSingleResMut<ShowDirty>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
	if show_changed.len() > 0 || show_added.len() > 0 {
		for entity in show_changed.iter().chain(show_added.iter()) {
			layer_dirty.mark(*entity)
		}
	}
	if add_events.len() > 0 {
		for entity in add_events.iter() {
			layer_dirty.mark(entity.0)
		}
	}

	// 已经移除的节点， 设置display为false
	if remove_events.len() > 0 {
		for entity in remove_events.iter() {
			if let Ok(mut is_show) = write.get_mut(entity.0) {
				is_show_changed.0.push(entity.0);
				let head = tree.get_down(entity.0).unwrap().head;
				is_show.set_display(false);
				for i in tree.recursive_iter(head) {
					if let Ok(mut is_show) = write.get_mut(i) {
						is_show_changed.0.push(i);
						is_show.set_display(false);
					}
				}
			}

		}
	}

	// for (layer, show, entity) in query_dirty.iter() {
	// 	if layer.is_changed() {
	// 		show_changed.0.push(i.0);
	// 	}
	// 	if !layer.layer().is_null() && (layer.is_changed() || show.as_ref().map_or(false, |r| {r.is_changed()})) {
	// 		layer_dirty.mark(i.0);
	// 	}
	// }
	if layer_dirty.count() > 0 {
		let mut display_change = false;
		let mut visibility_change = false;

		for node in layer_dirty.iter() {
			let mut parent_c_visibility = true;
			let mut parent_c_display = true;
			let mut parent_c_enable = true;

			let item = match query.get(node) {
				Ok(r) => r,
				_ => continue,
			};
			if let Some(up) = item.1 {
				let parent = up.parent();
				if let Ok(w) = write.get(parent) {
					parent_c_visibility = w.get_visibility();
					parent_c_display = w.get_display();
					parent_c_enable = w.get_enable();
				}
			}

			let show_value = item.0;
			let display_value = match show_value.get_display() {
				Display::Flex => true,
				Display::None => false,
				// Display::Grid => true,
			};
			let visibility_value = show_value.get_visibility();
			let enable_value = show_value.get_enable();

			let c_visibility =  visibility_value && parent_c_visibility;
			let c_display = display_value && parent_c_display;

			let c_enable = match enable_value {
				Enable::Visible => true,
				Enable::Auto => parent_c_enable,
				Enable::None => false,
			};
			let c_enable = c_visibility && c_enable;
			let mut write_item = write.get_mut(node).unwrap();
			let write_item1 = write_item.bypass_change_detection();
			if c_display != write_item1.get_display() {
				display_change = true;
				write_item1.set_display(c_display);
			}

			if c_visibility != write_item1.get_visibility() {
				visibility_change = true;
				write_item1.set_visibility(c_visibility);
			}

			if display_change || visibility_change {
				is_show_changed.0.push(node);
			}
			
			// log::debug!("c_enable: {}", c_enable);
			// log::warn!("show=============entity: {:?}, c_enable: {:?}, parent: {:?}, enable_value: {:?}", node, c_enable, parent_c_enable, enable_value);
			// println!("show=============entity: {:?}, c_display: {:?}, c_visibility: {:?}, c_enable: {:?}, {:?}", node, c_display, c_visibility, c_enable, visibility_change);
			write_item.set_enable(c_enable);
		}

		// display改变， 则发出通知，如果是实例化渲染， 需要重新组织实例化数据（display为None的实例， 不应该在实例化数据中）
		if display_change || visibility_change {
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
			log::debug!("node_changed3============");
		}
	}
}

lazy_static! {
    // 布局脏
    pub static ref SHOW_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::Display as usize)
		.set_bit(StyleType::Enable as usize)
		.set_bit(StyleType::Visibility as usize)
        .set_bit(OtherDirtyType::NodeTreeAdd as usize)
		.set_bit(OtherDirtyType::NodeTreeRemove as usize);
}


pub fn show_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*SHOW_DIRTY)
}

pub fn show_data_change(show_changed: OrInitSingleRes<ShowDirty>) -> bool {
	show_changed.0.len() > 0
}

/// 设置渲染数据
/// visibility为true时， 设置渲染实例可见
/// visibility为false时， 设置渲染实例不可见
pub fn set_show_data(
	mut instances: OrInitSingleResMut<InstanceContext>,
	query: Query<(&DrawList, &IsShow, &Layer, Option<&InstanceIndex>, Option<&Draw2DList>)>,
	mut show_changed: OrInitSingleResMut<ShowDirty>,
    query_draw: Query<&InstanceIndex>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
	if show_changed.0.len() == 0 {
		return;
	}
	for node in show_changed.0.drain(..) {
		if let Ok((draw_list, is_show, layer, instance_index, draw_2d_list)) = query.get(node) {
			let visibility = is_show.get_visibility() && is_show.get_display() && !layer.layer().is_null();
			for draw_id in draw_list.iter() {
				if let Ok(instance_index) = query_draw.get(draw_id.id) {

					let alignment = instances.instance_data.alignment;
					for index in 0..instance_index.opacity.len() / alignment {
						set_instance_visibility(visibility, instance_index.opacity.start + index * alignment, &mut instances);
					}
					for index in 0..instance_index.transparent.len() / alignment {
						set_instance_visibility(visibility, instance_index.transparent.start + index * alignment, &mut instances);
					}
				}
			}
	
			if let Some(instance_index) = instance_index {
				if !instance_index.transparent.start.is_null() {
					set_instance_visibility(visibility, instance_index.transparent.start, &mut instances);
				} else if !instance_index.opacity.start.is_null() {
					set_instance_visibility(visibility, instance_index.opacity.start, &mut instances);
				}
			}

			if let Some(draw_2d_list) = draw_2d_list {
				if !draw_2d_list.clear_instance.is_null() {
					set_instance_visibility(visibility, draw_2d_list.clear_instance, &mut instances);
				}
			}
		}
	}
}

fn set_instance_visibility(visibility: bool, instance_start: usize, instances: &mut InstanceContext) {
	let mut instance_data = instances.instance_data.instance_data_mut(instance_start);
	let mut ty = instance_data.get_render_ty();

	let old_visibility = (ty & (1 << RenderFlagType::NotVisibility as usize) ) == 0;
	if old_visibility != visibility {
		if visibility {
			ty &= !(1 << RenderFlagType::NotVisibility as usize);
		} else {
			ty |= 1 << RenderFlagType::NotVisibility as usize;
		}
		// log::trace!("set show=============entity: {:?}, visibility: {:?}", entity, (visibility, ty));
		instance_data.set_data(&TyMeterial(&[ty as f32]));
	}
}
