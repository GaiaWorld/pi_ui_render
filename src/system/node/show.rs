//! 计算show
//! 该系统默认为所有已经创建的Entity创建Show组件， 并监听Show和Display的创建修改， 以及监听idtree上的创建事件， 计算已经在idtree上///! 存在的实体的Enable和Visibility

use bevy_ecs::{prelude::Entity, query::Changed, system::Query};
use pi_bevy_ecs_extend::{prelude::{Layer, LayerDirty, OrDefault, Up}, system_param::res::OrInitRes};
use pi_flex_layout::style::Display;

use crate::{components::{
    calc::IsShow,
    user::{Enable, Show},
}, system::draw_obj::calc_text::IsRun};

pub struct CalcShow;

/// 计算节点的显示属性
pub fn calc_show(
    mut dirty: LayerDirty<Changed<Layer>>,
    show_change: Query<Entity, Changed<Show>>,
    query: Query<(OrDefault<Show>, Option<&Up>)>,
    mut write: Query<&mut IsShow>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    for entity in show_change.iter() {
        dirty.mark(entity)
    }

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
        write_item.set_visibility(c_visibility);
		write_item.set_display(parent_c_display);
        // log::warn!("show=============entity: {:?}, c_enable: {:?}, parent: {:?}, enable_value: {:?}", node, c_enable, parent_c_enable, enable_value);
        write_item.set_enable(c_enable);
    }
}
