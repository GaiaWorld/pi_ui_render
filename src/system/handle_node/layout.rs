//! 布局系统
//! 1.负责处理布局属性的脏，根据不同的脏，设置flex_layout节点的脏类型
//! 2.负责推动flex_layout节点进行布局
//! 
//! TODO
//! 1. 字符布局完成后，如何更新文字节点的布局属性
//! 

use std::{ops::{Index, IndexMut}, marker::PhantomData, intrinsics::transmute};

use pi_ecs::{prelude::{OrDefault, Query, filter_change::Changed, Or, Write, WriteItem, Local, ChangeTrackers}, entity::Entity};
use pi_ecs_utils::prelude::{EntityTree, Layer};
use pi_flex_layout::{
	prelude::{
		Dimension, FlexLayoutStyle, Get, GetMut, LayoutContext, INode, LayoutR, Layout, Rect, Display, FlexDirection, FlexWrap, AlignContent, AlignSelf, AlignItems, PositionType, Direction, JustifyContent, Overflow, Number,
		TreeStorage, CharNode
	}
};

use pi_dirty::LayerDirty;
use pi_null::Null;
use pi_slotmap_tree::{Storage, Up, Down};
use crate::components::{user::{Node, Size, Margin, Padding, Border, Position, MinMax, FlexContainer, FlexNormal, Show}, calc::{LayoutResult, NodeState}};


/// 根据布局样式，计算布局
pub fn calc_layout(
	query: Query<Node, (
		OrDefault<Size>,
		OrDefault<Margin>,
		OrDefault<Padding>,
		OrDefault<Border>,
		OrDefault<Position>,
		OrDefault<MinMax>,
		OrDefault<FlexContainer>,
		OrDefault<FlexNormal>,
		OrDefault<Show>,
	)>,
	mut inodes: Query<Node, &'static mut NodeState>,
	idtree: EntityTree<Node>,
	dirtys: Query<
		Node, 
		(
			Entity,
			ChangeTrackers<Size>,
			ChangeTrackers<Margin>,
			ChangeTrackers<Padding>,
			ChangeTrackers<Border>,
			ChangeTrackers<Position>,
			ChangeTrackers<MinMax>,
			ChangeTrackers<FlexContainer>,
			ChangeTrackers<FlexNormal>,
			ChangeTrackers<Layer>,
			ChangeTrackers<Show>,
			// ChangeTrackers<CharNode>,
		), 
		Or<(
			Changed<Size>, 
			Changed<Margin>,
			Changed<Padding>,
			Changed<Border>,
			Changed<Position>,
			Changed<MinMax>,
			Changed<FlexContainer>,
			Changed<FlexNormal>,
			Changed<Layer>,
			Changed<Show>,
			// Changed<Show>,
		)>
	>,
	mut layout_r: Query<Node, Write<LayoutResult>>,
	mut layer_dirty: Local<LayerDirty<LayoutKey>>,
	default_style: Local<(
		Size,
		Margin,
		Padding,
		Border,
		Position,
		MinMax,
		FlexContainer,
		FlexNormal,
		Show,
	)>,
) {
	
	let layout_styles = LayoutStyles{query: &query, char_nodes: unsafe{transmute(&mut inodes)} , default: &default_style };
	let mut node_state = INodes(unsafe{transmute(&mut inodes)});
	let mut layout_map = LayoutRs {style: &mut layout_r, default: LayoutResult::default(), char_nodes: unsafe{transmute(&mut inodes)} };
	let tree = Tree {
		tree: &idtree,
		char_nodes: unsafe{transmute(&mut inodes)},
	};

	let mut aa = 0;
	let layout_context= LayoutContext {
		mark: PhantomData,
		i_nodes: &mut node_state,
		layout_map: &mut layout_map,
		notify_arg: &mut aa,
		notify: notify,
		tree: &tree,
		style: &layout_styles,
	};
	let mut layout = Layout(layout_context);

	// 遍历布局脏节点，重新设置脏为层次脏
	for (
		e, 
		size, 
		margin, 
		padding, 
		border,
		position,
		min_max,
		flex_container,
		flex_normal,
		layer,
		show,
		// char_node
	) in dirtys.iter() {
		if size.is_changed() || position.is_changed() || margin.is_changed() || layer.is_changed() || min_max.is_changed() {
			layout.set_rect(&mut layer_dirty, LayoutKey{entity:e, text_index: usize::null()} , true, true);
		}

		if flex_normal.is_changed() {
			layout.set_normal_style(&mut layer_dirty, LayoutKey{entity:e, text_index: usize::null()} );
		}

		if padding.is_changed() || border.is_changed() {
			layout.set_self_style(LayoutKey{entity:e, text_index: usize::null()} , &mut layer_dirty);
		}

		if flex_container.is_changed() {
			layout.set_self_style(LayoutKey{entity:e, text_index: usize::null()} , &mut layer_dirty);
		}

		if show.is_changed() {
			layout.set_display(LayoutKey{entity:e, text_index: usize::null()} , &mut layer_dirty);
		}
	}

	// 计算布局
	layout.compute(&mut layer_dirty);
}

fn notify(_t: &mut i32, _entity: LayoutKey, _layout: &LayoutRItem) {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutKey {
	entity: Entity,
	text_index: usize,
}

impl Null for LayoutKey {
	/// 判断当前值是否空
    fn null() -> Self {
		LayoutKey {
			entity: Entity::null(),
			text_index: usize::null(),
		}
	}
    /// 判断当前值是否空
    fn is_null(&self) -> bool {
		self.text_index.is_null() || self.entity.is_null()
	}
}

pub struct LayoutStyles<'a, 'b> {
	query: &'a Query<Node, (
		OrDefault<Size>,
		OrDefault<Margin>,
		OrDefault<Padding>,
		OrDefault<Border>,
		OrDefault<Position>,
		OrDefault<MinMax>,
		OrDefault<FlexContainer>,
		OrDefault<FlexNormal>,
		OrDefault<Show>,
	)>,
	char_nodes: &'a mut Query<Node, &'b mut NodeState>,
	default: &'a (
		Size,
		Margin,
		Padding,
		Border,
		Position,
		MinMax,
		FlexContainer,
		FlexNormal,
		Show,
	),
}

impl<'a, 'b> Get<LayoutKey> for LayoutStyles<'a, 'b> {
	type Target = LayoutStyle<'a>;
	fn get(&self, k: LayoutKey) -> Self::Target {
		if k.text_index.is_null() {
			LayoutStyle(self.query.get(k.entity).unwrap())
		} else {
			let char_node = &(**self.char_nodes.get(k.entity).unwrap()).text[k.text_index];
			LayoutStyle((
				unsafe{transmute(&char_node.size)},
				unsafe{transmute(&char_node.margin)},
				&self.default.2,
				&self.default.3,
				&self.default.4,
				&self.default.5,
				&self.default.6,
				&self.default.7,
				&self.default.8,
			))
		}
	}
}

struct INodes<'a>(&'a mut Query<Node, &'static mut NodeState>);

impl<'a> Index<LayoutKey> for INodes<'a> {
	type Output = INode;
	fn index(&self, index: LayoutKey) -> &Self::Output {
		unsafe{transmute(& **self.0.get(index.entity).unwrap())}
	}
}

impl<'a> IndexMut<LayoutKey> for INodes<'a> {
	fn index_mut(&mut self, index: LayoutKey) -> &mut Self::Output {
		unsafe{transmute(&mut **self.0.get_mut(index.entity).unwrap())} 
	}
}

pub struct LayoutRs<'a, 'b>{
	style: &'a mut Query<Node, Write<LayoutResult>>,
	default: LayoutResult,
	char_nodes: &'a mut Query<Node, &'b mut NodeState>
}

impl<'a, 'b> GetMut<LayoutKey> for LayoutRs<'a, 'b> {
	type Target = LayoutRItem<'a, 'b>;
	fn get_mut(&mut self, index: LayoutKey) -> Self::Target {
		if index.text_index.is_null() {
			let mut item = self.style.get_mut(index.entity).unwrap();
			let r = unsafe { transmute(item.get_mut_or_default())};
			LayoutRItem::Node(item, r, unsafe { transmute(&mut self.char_nodes)}, index.entity)
		} else {
			LayoutRItem::Text(unsafe {transmute(&mut (**self.char_nodes.get_mut(index.entity).unwrap()).text[index.text_index])}, unsafe { transmute(&self.default.border)})
		}
	}
}

pub enum LayoutRItem<'s, 'b> {
	Node(WriteItem<LayoutResult>, &'s mut LayoutResult, &'s mut Query<Node, &'b mut NodeState>, Entity),
	Text(&'s mut CharNode, &'s Rect<f32>),
} 

// pub struct LayoutRItem<'s>(WriteItem<LayoutResult>, &'s mut LayoutResult);

impl<'s, 'b> LayoutR for LayoutRItem<'s, 'b> {
	fn rect(&self) -> &Rect<f32> {
		match self {
			LayoutRItem::Node(_, r, _, _) => &r.rect,
			LayoutRItem::Text(char_node, _) => &char_node.pos
		}
		
	}
	fn border(&self) -> &Rect<f32> {
		match self {
			LayoutRItem::Node(_, r, _, _) => &r.border,
			LayoutRItem::Text(_, r) => r
		}
	}
	fn padding(&self) -> &Rect<f32> {
		match self {
			LayoutRItem::Node(_, r, _, _) => &r.padding,
			LayoutRItem::Text(_, r) => r
		}
	}
	
	// 设置布局属性
	fn set_rect(&mut self, v: Rect<f32>) {
		match self {
			LayoutRItem::Node(_, r, _, _) => r.padding = v,
			LayoutRItem::Text(char_node, _r) =>  {
				char_node.pos = v;
			}
		};
	}
	fn set_border(&mut self, v: Rect<f32>) {
		if let LayoutRItem::Node(_, r, _, _) = self {
			r.border = v;
		}
	}
	fn set_padding(&mut self, v: Rect<f32>) {
		if let LayoutRItem::Node(_, r, _, _) = self {
			r.padding = v;
		}
	}

	fn set_finish(&mut self) {
		if let LayoutRItem::Node(r, l, node_states, e) = self {
			let state = node_states.get(e.clone()).unwrap();
			if state.is_vnode() && state.text.len() > 0 {
				// 
				let mut rect = Rect{
					left: std::f32::MAX,
					right: 0.0,
					top: std::f32::MAX,
					bottom: 0.0,
				};
				for c in state.text.iter() {
					let l = &c.pos;
					if l.left < rect.left {
						rect.left = l.left;
					}
					if l.top < rect.top {
						rect.top = l.top;
					}
			
					if l.right > rect.right {
						rect.right = l.right;
					}
					if l.bottom > rect.bottom {
						rect.bottom = l.bottom;
					}
				}
				l.rect = rect;
				
			}
			r.notify_modify();
		}
	}
}

pub struct LayoutStyle<'a>((
	&'a Size, 
	&'a Margin, 
	&'a Padding, 
	&'a Border, 
	&'a Position, 
	&'a MinMax, 
	&'a FlexContainer,
	&'a FlexNormal,
	&'a Show,
));

impl<'a> FlexLayoutStyle for LayoutStyle<'a>  {
	fn width(&self) -> Dimension {
		self.0.0.width
	}
	fn height(&self) -> Dimension {
		self.0.0.height
	}

	fn margin_top(&self) -> Dimension {
		self.0.1.top
	}
	fn margin_right(&self) -> Dimension {
		self.0.1.right
	}
	fn margin_bottom(&self) -> Dimension{
		self.0.1.bottom
	}
	fn margin_left(&self) -> Dimension {
		self.0.1.left
	}
	
	fn padding_top(&self) -> Dimension {
		self.0.2.top
	}
	fn padding_right(&self) -> Dimension {
		self.0.2.right
	}
	fn padding_bottom(&self) -> Dimension{
		self.0.2.bottom
	}
	fn padding_left(&self) -> Dimension {
		self.0.2.left
	}

	fn position_top(&self) -> Dimension {
		self.0.4.top
	}
	fn position_right(&self) -> Dimension {
		self.0.4.right
	}
	fn position_bottom(&self) -> Dimension {
		self.0.4.bottom
	}
	fn position_left(&self) -> Dimension {
		self.0.4.left
	}

	fn border_top(&self) -> Dimension {
		self.0.3.top
	}
	fn border_right(&self) -> Dimension{
		self.0.3.right
	}
	fn border_bottom(&self) -> Dimension{
		self.0.3.bottom
	}
	fn border_left(&self) -> Dimension {
		self.0.3.left
	}

	fn display(&self) -> Display {
		self.0.8.get_display()
	}

    fn position_type(&self) -> PositionType {
		self.0.7.position_type
	}
    fn direction(&self) -> Direction {
		self.0.6.direction
	}

    fn flex_direction(&self) -> FlexDirection {
		self.0.6.flex_direction
	}
    fn flex_wrap(&self) -> FlexWrap {
		self.0.6.flex_wrap
	}
    fn justify_content(&self) -> JustifyContent {
		self.0.6.justify_content
	}
    fn align_items(&self) -> AlignItems {
		self.0.6.align_items
	}
    fn align_content(&self) -> AlignContent {
		self.0.6.align_content
	}

    fn order(&self) -> isize {
		self.0.7.order
	}
    fn flex_basis(&self) -> Dimension {
		self.0.7.flex_basis
	}
    fn flex_grow(&self) -> f32 {
		self.0.7.flex_grow
	}
    fn flex_shrink(&self) -> f32 {
		self.0.7.flex_shrink
	}
    fn align_self(&self) -> AlignSelf {
		self.0.7.align_self
	}

    fn overflow(&self) -> Overflow {
		unimplemented!()
	}
    fn min_width(&self) -> Dimension {
		self.0.5.min.width
	}
	fn min_height(&self) -> Dimension {
		self.0.5.min.height
	}
	fn max_width(&self) -> Dimension {
		self.0.5.max.height
	}
	fn max_height(&self) -> Dimension {
		self.0.5.max.height
	}
    fn aspect_ratio(&self) -> Number {
		self.0.7.aspect_ratio
	}
}

pub struct Tree<'a, 'b>{
	tree: &'a EntityTree<Node>,
	char_nodes: &'a Query<Node, &'b mut NodeState>
}

impl<'a, 'b> TreeStorage<LayoutKey> for Tree<'a, 'b> {
	fn get_up(&self, k: LayoutKey) -> Option<Up<LayoutKey>> {
		if k.text_index.is_null() {
			// 普通节点
			match self.tree.get_up(k.entity) {
				Some(r) => {
					Some(
						Up::new(
							LayoutKey {entity: r.parent(), text_index: usize::null()},
							LayoutKey {entity: r.prev(), text_index: usize::null()},
							LayoutKey {entity: r.next(), text_index: usize::null()}
						),
					)
				},
				None => None
			}
		} else {
			// 文字 
			let char_node = self.char_nodes.get_unchecked(k.entity);
			let char = &char_node.text[k.text_index];

			let prev = if k.text_index == 0 {
				usize::null()
			} else {
				let p = k.text_index - 1;
				let prev_char = &char_node.text[p];
				if prev_char.context_id != char.context_id {
					prev_char.context_id as usize
				} else {
					p
				}
			};

			let n = k.text_index + 1;
			let next = if n >= char_node.text.len() {
				usize::null()
			} else {
				let next_char = &char_node.text[n];
				if next_char.context_id != char.context_id {
					if next_char.context_id == n as isize { // 后面节点的context_id是自己
						k.text_index + char.count
					} else {
						usize::null()
					}
				} else {
					n
				}
			};
			// 父节点是一个普通节点
			return Some(Up::new(
				LayoutKey {entity: k.entity, text_index: usize::null()},
				LayoutKey {entity: if prev.is_null() {Entity::null()}else{k.entity}, text_index: prev},
				LayoutKey {entity: if next.is_null() {Entity::null()}else{k.entity}, text_index: next}
			));
		}
	}
	fn up(&self, k: LayoutKey) -> Up<LayoutKey> {
		self.get_up(k).unwrap()
	}

	fn get_layer(&self, k: LayoutKey) -> Option<usize> {
		if k.text_index.is_null() {
			match self.tree.get_layer(k.entity) {
				Some(r) => Some(*r),
				None => None
			}
		} else {
			return None;
		}
	}
	fn layer(&self, k: LayoutKey) -> usize {
		self.get_layer(k).unwrap()
	}

	fn get_down(&self, k: LayoutKey) -> Option<Down<LayoutKey>> {
		if k.text_index.is_null() {
			let char_node = self.char_nodes.get(k.entity);
			match char_node {
				Some(chars) => {
					if chars.text.len() == 0 {
						return None;
					}
 					let last = &chars.text[chars.text.len() - 1];
					let last_index = if last.context_id.is_null() {
						chars.text.len() - 1
					} else {
						last.context_id as usize
					};
					Some(
						Down::new(
							LayoutKey {entity: k.entity, text_index: 0},
							LayoutKey {entity: k.entity, text_index: last_index},
							0, // 在flex布局中，未使用到len
							0 // 在flex布局中，未使用到count
						),
					)
				},
				None => {
					// 普通节点
					match self.tree.get_down(k.entity) {
						Some(r) => {
							Some(
								Down::new(
									LayoutKey {entity: r.head(), text_index: usize::null()},
									LayoutKey {entity: r.tail(), text_index: usize::null()},
									r.len(),
									r.count()
								),
							)
						},
						None => None
					}
				},
			}
		} else {
			// 文字
			// 字符节点无子节点
			// 单词字符节点不需要布局，其位置就是其初始化时的位置
			return None;
		}
	}
	fn down(&self, k: LayoutKey) -> Down<LayoutKey> {
		self.get_down(k).unwrap()
	}
}


// /// 布局大小
// #[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
// pub struct Size(FlexSize<Dimension>);

// /// 布局外边距
// #[derive(Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
// pub struct Margin(Rect<Dimension>);

// /// 布局内边距
// #[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
// pub struct Padding(Rect<Dimension>);

// /// 布局边框尺寸
// #[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
// pub struct Border(Rect<Dimension>);

// #[derive(Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
// pub struct Position(Rect<Dimension>);

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct MinMax{
// 	pub min: FlexSize<Dimension>,
// 	pub max: FlexSize<Dimension>,
// }

// // 描述子节点行为的flex布局属性
// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct FlexContainer {
// 	pub flex_direction: FlexDirection,
//     pub flex_wrap: FlexWrap,
//     pub justify_content: JustifyContent,
//     pub align_items: AlignItems,
//     pub align_content: AlignContent,
// 	pub direction: Direction,
// }

// // 描述节点自身行为的flex布局属性
// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct FlexNormal {
// 	pub order: isize,
//     pub flex_basis: Dimension,
//     pub flex_grow: f32,
//     pub flex_shrink: f32,
//     pub align_self: AlignSelf,
// 	pub position_type: PositionType,
// 	pub aspect_ratio: Number,
// }

// // 矩形区域脏，绝对定位下，设自身self_dirty，相对定位下，设自身self_dirty后，还要设父child_dirty
// pub const RECT_DIRTY: usize = StyleType2::Width as usize 
// 							| StyleType2::Height as usize
// 							| LAYOUT_POSITION_MARK
// 							| LAYOUT_MARGIN_MARK;

// // 普通脏及子节点添加或移除， 设父child_dirty
// pub const NORMAL_DIRTY: usize = //StyleType2::FlexBasis as usize 
// 							//| StyleType1::Order as usize
// 							StyleType2::FlexShrink as usize
// 							| StyleType2::FlexGrow as usize
// 							| StyleType2::AlignSelf as usize
// 							| StyleType2::PositionType as usize;

// // 自身脏， 仅设自身self_dirty
// pub const SELF_DIRTY: usize = LAYOUT_PADDING_MARK 
// 							| LAYOUT_BORDER_MARK;

// // 子节点脏， 仅设自身child_dirty
// pub const CHILD_DIRTY: usize = StyleType2::FlexDirection as usize
// 							| StyleType2::FlexWrap as usize
// 							| StyleType2::AlignItems as usize
// 							| StyleType2::JustifyContent as usize
// 							| StyleType2::AlignContent as usize;


// pub const DIRTY2: usize = RECT_DIRTY | NORMAL_DIRTY | SELF_DIRTY | CHILD_DIRTY;


// #[derive(Default)]
// pub struct LayoutSys{
// 	dirty: LayerDirty<usize>,
// }

// impl<'a> Runner<'a> for LayoutSys {
// 	type ReadData = ( 
// 		&'a MultiCaseImpl<Node, RectLayoutStyle>,
// 		&'a MultiCaseImpl<Node, OtherLayoutStyle>,
// 		&'a SingleCaseImpl<IdTree>, 
// 		&'a SingleCaseImpl<DirtyList>);
// 	type WriteData = (
// 		&'a mut MultiCaseImpl<Node, LayoutR>,
// 		&'a mut MultiCaseImpl<Node, NodeState>,
// 		&'a mut MultiCaseImpl<Node, StyleMark>, );
//     fn run(&mut self, (rect_layout_styles, other_layout_styles, tree, dirty_list, ): Self::ReadData, (layouts, node_states, style_marks): Self::WriteData) {
// 		let time = cross_performance::now();
// 		if dirty_list.0.len() == 0 {
//             return;
// 		}
		
// 		let flex_rect_styles = unsafe {&mut *(rect_layout_styles.get_storage() as *const VecMapWithDefault<RectLayoutStyle> as usize as *mut VecMapWithDefault<flex_layout::RectStyle>)};
// 		let flex_other_styles = unsafe {&mut *(other_layout_styles.get_storage() as *const VecMapWithDefault<OtherLayoutStyle> as usize as *mut VecMapWithDefault<flex_layout::OtherStyle>)};
// 		let flex_layouts = unsafe {&mut *(layouts.get_storage() as *const VecMap<LayoutR> as usize as *mut VecMap<flex_layout::LayoutR>)};
// 		let node_states = unsafe {&mut *(node_states.get_storage() as *const VecMap<NodeState> as usize as *mut VecMap<flex_layout::INode>)};
		
// 		// log::info!("dirty_list============={:?}", dirty_list.0);
// 		for id in dirty_list.0.iter() {
// 			let style_mark = match style_marks.get_mut(*id) {
//                 Some(r) => r,
//                 None => continue,
//             };
// 			match tree.get(*id) {
//                 Some(r) => if r.layer() == 0 {continue},
//                 None => continue,
//             };
// 			let dirty2 = style_mark.dirty2;
// 			let dirty1 = style_mark.dirty1;
// 			// log::info!("layout dirty============={}, {}, {}", dirty2, dirty1, dirty2 & RECT_DIRTY);

//             // 不存在LayoutTree关心的脏, 跳过
//             if dirty2 & DIRTY2 == 0 && dirty1 & StyleType1::Display as usize == 0 && dirty1 & StyleType1::FlexBasis as usize == 0 && dirty1 & StyleType1::Create as usize == 0 {
//                 continue;
// 			}

// 			// log::info!("layout dirty1============={}", id);

// 			// println!("dirty======{:?}, {:?}", id, &flex_styles[*id]);
// 			let rect_style = &flex_rect_styles[*id];
// 			let other_style = &flex_other_styles[*id];

// 			if dirty2 & RECT_DIRTY != 0 || dirty1 & StyleType1::Create as usize != 0 {
// 				set_rect(tree, node_states, &mut self.dirty, *id, rect_style, other_style, true, true);
// 			}

// 			if dirty2 & NORMAL_DIRTY != 0 || dirty1 & StyleType1::FlexBasis as usize != 0 {
// 				// println!("dirty NORMAL_DIRTY======{:?}", id);
// 				set_normal_style(tree, node_states, &mut self.dirty, *id, other_style);
// 			}

// 			if dirty2 & SELF_DIRTY != 0 {
// 				// println!("dirty SELF_DIRTY======{:?}", id);
// 				set_self_style(tree, node_states, &mut self.dirty, *id, other_style);
// 			}

// 			if dirty2 & CHILD_DIRTY as usize != 0 {
// 				set_children_style(tree, node_states, &mut self.dirty, *id, other_style);
// 			}

// 			if dirty1 & StyleType1::Display as usize != 0 {
// 				set_display(*id, other_style.display, &mut self.dirty, tree, node_states, rect_style, other_style);
// 			}
// 			style_mark.dirty2 &= !DIRTY2;
// 			style_mark.dirty1 &= !(StyleType1::Display as usize | StyleType1::FlexBasis as usize | StyleType1::Create as usize);
// 		}
// 		let count = self.dirty.count();
// 		compute(&mut self.dirty, tree, node_states, flex_rect_styles, flex_other_styles, flex_layouts, notify, layouts);
// 		// if count > 0 {
// 		// 	log::info!("layout======={:?}", cross_performance::now() - time);
// 		// }
// 	}
// }


// //节点创建时， 默认为节点创建LayoutStyle组件
// impl<'a> EntityListener<'a, Node, CreateEvent> for LayoutSys {
// 	type ReadData = &'a SingleCaseImpl<IdTree>;
// 	type WriteData = (
// 		&'a mut MultiCaseImpl<Node, RectLayoutStyle>, 
// 		&'a mut MultiCaseImpl<Node, OtherLayoutStyle>, 
// 		&'a mut MultiCaseImpl<Node, LayoutR>, 
// 		&'a mut MultiCaseImpl<Node, NodeState>);
// 	fn listen(&mut self, event: &Event, _tree: Self::ReadData, (rect_layout_styles, other_layout_styles, layouts, node_states): Self::WriteData) {
// 		// rect_layout_styles.insert(event.id, RectLayoutStyle::default());
// 		// other_layout_styles.insert(event.id, OtherLayoutStyle::default());
// 		layouts.insert(event.id, LayoutR::default());
// 		node_states.insert(event.id, NodeState::default());
// 	}
// }

// // impl<'a> SingleCaseListener<'a, IdTree, ModifyEvent> for LayoutSys {
// //     type ReadData = &'a SingleCaseImpl<IdTree>;
// //     type WriteData = (&'a mut  MultiCaseImpl<Node, NodeState>, &'a mut  MultiCaseImpl<Node, RectLayoutStyle>, &'a mut  MultiCaseImpl<Node, OtherLayoutStyle>);
// //     fn listen(&mut self, event: &Event, tree: Self::ReadData, (node_states, _rect_layout_styles, other_layout_styles): Self::WriteData) {
// // 		// let flex_rect_styles = unsafe {&mut *(rect_layout_styles.get_storage() as *const VecMap<RectLayoutStyle> as usize as *mut VecMap<flex_layout::RectStyle>)};
		
// // 		// if event.field == "add" {
// // 		// let flex_other_styles = unsafe {&mut *(other_layout_styles.get_storage() as *const VecMapWithDefault<OtherLayoutStyle> as usize as *mut VecMapWithDefault<flex_layout::OtherStyle>)};
// // 		// 	set_normal_style(tree, node_states, &mut self.dirty, event.id, &flex_other_styles[event.id]);
// // 		// }
// // 		 if event.field == "remove"{

// // 			let node_states = unsafe {&mut *(node_states.get_storage() as *const VecMap<NodeState> as usize as *mut VecMap<flex_layout::INode>)};
// // 			let parent = tree[event.id].parent();
// // 			if parent > 0 {
// // 				mark_children_dirty(tree, node_states, &mut self.dirty, parent);
// // 			}
// // 		}
// //     }
// // }

// // impl<'a> SingleCaseListener<'a, IdTree, CreateEvent> for LayoutSys {
// //     type ReadData = &'a SingleCaseImpl<IdTree>;
// //     type WriteData = (&'a mut  MultiCaseImpl<Node, NodeState>, &'a mut  MultiCaseImpl<Node, RectLayoutStyle>, &'a mut  MultiCaseImpl<Node, OtherLayoutStyle>);
// //     fn listen(&mut self, event: &Event, tree: Self::ReadData, (node_states, _rect_layout_styles, other_layout_styles): Self::WriteData) {
// // 		// log::info!("idtree create============={}", event.id);
// // 		let flex_other_styles = unsafe {&mut *(other_layout_styles.get_storage() as *const VecMapWithDefault<OtherLayoutStyle> as usize as *mut VecMapWithDefault<flex_layout::OtherStyle>)};
// // 		let node_states = unsafe {&mut *(node_states.get_storage() as *const VecMap<NodeState> as usize as *mut VecMap<flex_layout::INode>)};
// // 		set_normal_style(tree, node_states, &mut self.dirty, event.id, &flex_other_styles[event.id]);
// //     }
// // }

// impl<'a> SingleCaseListener<'a, IdTree, DeleteEvent> for LayoutSys {
//     type ReadData = &'a SingleCaseImpl<IdTree>;
//     type WriteData = &'a mut MultiCaseImpl<Node, NodeState>;
//     fn listen(&mut self, event: &Event, tree: Self::ReadData, node_states: Self::WriteData) {
// 		let node_states = unsafe {&mut *(node_states.get_storage() as *const VecMap<NodeState> as usize as *mut VecMap<flex_layout::INode>)};
// 		let parent = tree[event.id].parent();
// 		if parent > 0 {
// 			mark_children_dirty(tree, node_states, &mut self.dirty, parent);
// 		}
//     }
// }

// fn notify(context: usize, id: usize, _layout:&pi_flex_layout::LayoutR) {
// 	// println!("notify======================={}, layout:{:?}", id, layout);
// 	// context.get_notify_ref().modify_event(id, "", 0);
// }
