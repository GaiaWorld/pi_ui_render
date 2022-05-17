//! 计算show
//! 该系统默认为所有已经创建的Entity创建Show组件， 并监听Show和Display的创建修改， 以及监听idtree上的创建事件， 计算已经在idtree上///! 存在的实体的Enable和Visibility

use pi_ecs::prelude::{Query, Write, Changed, OrDefault};
use pi_ecs_utils::prelude::{LayerDirty, NodeUp};
use pi_flex_layout::style::Display;

use crate::{
	components::{
		user::{Node, Show, Enable},
		calc::{IsEnable, Visibility as CVisibility}
	}
};

/// 计算节点的显示属性
pub fn calc_show(
	dirty: &LayerDirty<Node, Changed<Show>>,
	query: Query<Node, (OrDefault<Show>, Option<&NodeUp<Node>>)>,
	write: Query<Node, (Write<CVisibility>, Write<IsEnable>)>
) {
	for node in dirty.iter() {
		let mut parent_c_visibility = true;
		let mut parent_c_enable = true;
		let item = query.get_unchecked(node);
		if let Some(up) = item.1 {
			let parent = up.parent();
			let w = write.get_unchecked(parent);

			parent_c_visibility = w.0.get_or_default().0;
			parent_c_enable = w.1.get_or_default().0;
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
		let mut write_item = write.get_unchecked(node);
		write_item.0.write(CVisibility(c_visibility));
		write_item.1.write(IsEnable(c_enable));
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn test() {

	}


}
