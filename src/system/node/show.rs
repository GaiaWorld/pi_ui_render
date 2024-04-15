//! 计算show
//! 该系统默认为所有已经创建的Entity创建Show组件， 并监听Show和Display的创建修改， 以及监听idtree上的创建事件， 计算已经在idtree上///! 存在的实体的Enable和Visibility

use bevy_app::{Plugin, Update};
use bevy_ecs::{prelude::Entity, query::Changed, system::Query, event::{EventWriter, EventReader}, change_detection::DetectChangesMut, schedule::IntoSystemConfigs};
use bevy_window::AddFrameEvent;
use pi_bevy_ecs_extend::{prelude::{Layer, LayerDirty, OrDefault, Up}, system_param::res::{OrInitRes, OrInitResMut}};
use pi_bevy_render_plugin::FrameDataPrepare;
use pi_flex_layout::style::Display;
use pi_null::Null;

use crate::{components::{
    calc::{IsShow, DrawList},
    user::{Enable, Show}, draw_obj::InstanceIndex,
}, system::{draw_obj::{calc_text::IsRun, life_drawobj::update_render_instance_data}, system_set::UiSystemSet}, events::{NodeDisplayChange, NodeVisibilityChange}, resource::draw_obj::InstanceContext, shader1::meterial::{TyUniform, RenderFlagType}};

pub struct ShowPlugin;

impl Plugin for ShowPlugin {
    fn build(&self, app: &mut bevy_app::App) {
		app
			.add_frame_event::<NodeVisibilityChange>()
			.add_systems(Update, calc_show.in_set(UiSystemSet::BaseCalc))
			.add_systems(Update, 
				set_show_data
					.after(update_render_instance_data)
					.after(UiSystemSet::PrepareDrawObj) // 这里是为了确保与其他设置实例数据的system不并行， 因为设置的数据冲突（TyUniform）
					.in_set(FrameDataPrepare))
		;
	}
}

/// 计算节点的显示属性
pub fn calc_show(
    mut dirty: LayerDirty<Changed<Layer>>,
    show_change: Query<Entity, Changed<Show>>,
    query: Query<(OrDefault<Show>, Option<&Up>)>,
    mut write: Query<&mut IsShow>,
	r: OrInitRes<IsRun>,
	mut events: EventWriter<NodeDisplayChange>,
	mut visisble_events: EventWriter<NodeVisibilityChange>,
) {
	if r.0 {
		return;
	}
    for entity in show_change.iter() {
        dirty.mark(entity)
    }

	let mut display_change = false;
	let mut visibility_change = false;

    for node in dirty.iter() {
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
		// log::debug!("c_enable: {}", c_enable);
		// log::warn!("show=============entity: {:?}, c_enable: {:?}, parent: {:?}, enable_value: {:?}", node, c_enable, parent_c_enable, enable_value);
        log::trace!("show=============entity: {:?}, c_display: {:?}, c_visibility: {:?}, c_enable: {:?}, {:?}", node, c_display, c_visibility, c_enable, visibility_change);
		write_item.set_enable(c_enable);
    }

	// display改变， 则发出通知，如果是实例化渲染， 需要重新组织实例化数据（display为None的实例， 不应该在实例化数据中）
	if display_change {
		events.send(NodeDisplayChange);
	}

	if visibility_change {
		visisble_events.send(NodeVisibilityChange);
	}
}

/// 设置渲染数据
/// visibility为true时， 设置渲染实例可见
/// visibility为false时， 设置渲染实例不可见
pub fn set_show_data(
	mut visisble_events: EventReader<NodeVisibilityChange>,
	mut display_events: EventReader<NodeDisplayChange>,
	mut instances: OrInitResMut<InstanceContext>,
	query: Query<(&DrawList, &IsShow, Option<&InstanceIndex>, Entity), Changed<IsShow>>,
    query_draw: Query<&InstanceIndex>,
	r: OrInitRes<IsRun>,
) {
	if r.0 {
		return;
	}
	if visisble_events.len() == 0 && display_events.len() == 0{
		return;
	}
	visisble_events.clear();
	display_events.clear();

	for (draw_list, is_show, instance_index, entity) in query.iter() {
		let visibility = is_show.get_visibility() || is_show.get_display();
		for draw_id in draw_list.iter() {
			if let Ok(instance_index) = query_draw.get(draw_id.id) {
				let alignment = instances.instance_data.alignment;
				let count = instance_index.0.len() / alignment;
				for index in 0..count {
					let mut instance_data = instances.bypass_change_detection().instance_data.instance_data_mut(instance_index.0.start + index * alignment);
					let mut ty = instance_data.get_render_ty();

					let old_visibility = (ty | (1 << RenderFlagType::NotVisibility as usize) ) == 0;
					if old_visibility != visibility {
						if visibility {
							ty &= !(1 << RenderFlagType::NotVisibility as usize);
						} else {
							ty |= 1 << RenderFlagType::NotVisibility as usize;
						}
						instance_data.set_data(&TyUniform(&[ty as f32]));
					}
				}
			}
		}

		if let Some(instance_index) = instance_index {
			if instance_index.start.is_null() {
				return;
			}
			let mut instance_data = instances.bypass_change_detection().instance_data.instance_data_mut(instance_index.0.start);
			let mut ty = instance_data.get_render_ty();

			let old_visibility = (ty | (1 << RenderFlagType::NotVisibility as usize) ) == 0;
			if old_visibility != visibility {
				if visibility {
					ty &= !(1 << RenderFlagType::NotVisibility as usize);
				} else {
					ty |= 1 << RenderFlagType::NotVisibility as usize;
				}
				// log::trace!("set show=============entity: {:?}, visibility: {:?}", entity, (visibility, ty));
				instance_data.set_data(&TyUniform(&[ty as f32]));
			}
		}
	}
}
