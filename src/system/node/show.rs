//! 计算show
//! 该系统默认为所有已经创建的Entity创建Show组件， 并监听Show和Display的创建修改， 以及监听idtree上的创建事件， 计算已经在idtree上///! 存在的实体的Enable和Visibility

use bevy::ecs::{prelude::Entity, query::Changed, system::Query};
use pi_bevy_ecs_extend::prelude::{Layer, LayerDirty, OrDefault, Up};
use pi_flex_layout::style::Display;

use crate::components::{
    calc::IsShow,
    user::{Enable, Show},
};

pub struct CalcShow;

/// 计算节点的显示属性色
pub fn calc_show(
    mut dirty: LayerDirty<Changed<Layer>>,
    show_change: Query<Entity, Changed<Show>>,
    query: Query<(OrDefault<Show>, Option<&Up>)>,
    mut write: Query<&mut IsShow>,
) {
    for entity in show_change.iter() {
        dirty.mark(entity)
    }

    for node in dirty.iter() {
        let mut parent_c_visibility = true;
        let mut parent_c_enable = true;
        let item = match query.get(node) {
            Ok(r) => r,
            _ => continue,
        };
        if let Some(up) = item.1 {
            let parent = up.parent();
            if let Ok(w) = write.get(parent) {
                parent_c_visibility = w.get_visibility();
                parent_c_enable = w.get_visibility();
            }
        }

        let show_value = item.0;
        let display_value = match show_value.get_display() {
            Display::Flex => true,
            Display::None => false,
        };
        let visibility_value = show_value.get_visibility();
        let enable_value = show_value.get_enable();

        let c_visibility = display_value && visibility_value && parent_c_visibility;
        let c_enable = match enable_value {
            Enable::Visible => true,
            Enable::Auto => parent_c_enable,
            Enable::None => false,
        };
        let c_enable = c_visibility && c_enable;
        let mut write_item = write.get_mut(node).unwrap();
        write_item.set_visibility(c_visibility);
        write_item.set_enable(c_enable);
    }
}
