//! 1. 收集渲染上下文节点，为该节点创建对应的渲染上下文
//! 2. 为所有节点设置其所在的渲染上下文的引用
//! 
//! 渲染上下文表示，节点自身及其递归子节点需要渲染到一个单独的fbo上，后续会将该fbo渲染到父上下文上
//! 如Opacity、Blur、Hsv、MaskImage等，其效果需要将节点自身及其递归子节点作为一个整体来处理
//! 拥有这类属性的节点应该成为一个渲染上下文
//! 本模块不关心具体需要成为渲染上下文的属性，而是只关心RenderContextMark组件，该组件存在时，则为渲染上下文，不存在时，就不是渲染上下文
//! 
//! 节点拥有一个属性，如果设置该属性，意味着节点需要是一个渲染上下文，请插入RenderContextMark组件

use pi_dirty::LayerDirty;
use pi_ecs::{entity::Entity, prelude::{EntityCommands, Query, Write, Commands, ResMut, Or}, monitor::Event, query::filter_change::{Modifyed, Added}, storage::LocalVersion};
use pi_ecs_macros::listen;
use pi_ecs_utils::prelude::{Layer, NodeUp, LayerDirty as LayerDirtyParam, EntityTree, NodeDown};
use pi_null::Null;
use pi_slotmap::SecondaryMap;

use crate::components::{calc::RenderContextMark, user::{Aabb2, Point2}};
use crate::components::user::Node;

/// 渲染上下文实体
pub struct RenderContext;

/// 渲染上下文列表，组织为层的结构，渲染时，按照层的顺序，从大到小渲染（因为父上下文的渲染依赖子上下文的渲染）
#[derive(Deref, DerefMut, Default)]
pub struct RenderContextLayerList(LayerDirty<Entity>);

/// 上下文的实体ID，作为Node的组件，关联由其创建的渲染上下文
#[derive(Deref, DerefMut, Default)]
pub struct ContextId(Entity);

/// 作为Node的组件，表示节点所在的渲染上下文的实体
#[derive(Clone, Copy, Deref, DerefMut, Default, PartialEq, Eq)]
pub struct InContextId(Entity);

/// 节点的实体id，作为RenderContext的组件，引用创建该渲染上下文的节点
#[derive(Deref, DerefMut, Default)]
pub struct NodeId(Entity);

/// 脏区域
#[derive(Deref, DerefMut)]
pub struct DirtyRect(Aabb2);

impl Default for DirtyRect {
    fn default() -> Self {
        Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
    }
}

/// 监听RenderContextMark的修改
/// 检查： 标记全部为空时，删除RenderContextMark组件
#[listen(component=(Node, RenderContextMark, Modify))]
pub fn context_mark_modify(
	e: Event,
	mut render_mark: Query<Node, Write<RenderContextMark>>,
) {
	let mut render_context_mark_item = match render_mark.get_mut(e.id) {
		Some(r) => r,
		// 正常情况不会进入该分支，除非e.id实体在Node中不存在
		None => return,
	};

	// 如果标记全部为空，则删除RenderContextMark组件
	if let Some(v) = render_context_mark_item.get() {
		if v.not_any() {
			render_context_mark_item.remove();
		}
	}
}

/// 监听RenderContextMark和Layer的创建方法，创建渲染上下文（为考虑层修改的情况，是否需要？ TODO）
#[listen(component=(Node, RenderContextMark, Create), component=(Node, Layer, Create))]
pub fn context_mark_create(
	e: Event,
	context_id: Query<Node, Write<ContextId>>,
	mark_layer: Query<Node, (&RenderContextMark, &Layer)>,
	mut command: EntityCommands<RenderContext>,
	mut node_id: Commands<NodeId>,

	mut render_context_layer_list: ResMut<RenderContextLayerList>,
) {
	// 不存在RenderContextMark或者不存在Layer，都不处理
	// 成为渲染上下文的充要条件是：RenderContextMark组件存在，并且节点在渲染树上
	if let Some((_, layer)) = mark_layer.get(e.id) {
		// 创建一个RenderContext
		let context = command.spawn();
		// 建立Node到RenderContext的索引关系
		context_id.get_unchecked(e.id).write(ContextId(context));
		// 建立RenderContext到Node的索引关系
		node_id.insert(context, NodeId(e.id));

		// 添加到渲染列表中（伴随着层信息，可以按层进行迭代）
		render_context_layer_list.mark(context, **layer);
	}
}

/// 监听RenderContextMark、Layer、Node的移除事件，移除对应的渲染上下文
#[listen(component=(Node, RenderContextMark, Delete), component=(Node, Layer, Delete), entity=(Node, Delete))]
pub fn context_mark_remove(
	e: Event,
	context_id: Query<Node, Write<ContextId>>,
	layer: Query<Node, &Layer>,
	mut command: EntityCommands<RenderContext>,
	mut render_context_layer_list: ResMut<RenderContextLayerList>,
) {
	if let (Some(context), Some(layer)) = (context_id.get_unchecked(e.id).remove(), layer.get(e.id)) {
		// 删除RenderContext实体
		command.despawn(context.0);
		render_context_layer_list.delete(*context, **layer);
	}
}

/// system
/// 记录RenderContext添加和删除的脏，同时记录节点添加到树上的脏
/// 根据脏，从父向子递归，设置节点所在的渲染上下文（节点的渲染目标）
pub fn context_create(
	dirty: LayerDirtyParam<Node, Or<(Modifyed<NodeUp>, Added<RenderContext>)>>,
	idtree: EntityTree<Node>,
	query: Query<Node, (Option<&NodeDown>, Option<&RenderContext>)>,
	up: Query<Node, Option<&NodeUp>>,
	mut in_context: Query<Node, Write<InContextId>>,
) {
	// NodeUp 修改，表示节点挂载主树上
	// 当节点被挂载主树上，或者
	for (node, mark) in dirty.iter_manual() {
		let up_item = up.get_unchecked(node);
		let parent_context_id = match up_item {
			// 取到父的context_id
			Some(r) if r.parent().is_null() => in_context.get_unchecked(r.parent()).get().unwrap().clone(),
			_ => InContextId(Entity::null())
		};

		let (children_item, context_item) = query.get_unchecked(node);
		let (child_context_id, cur_context_id) = match context_item {
			Some(_) => (in_context.get_unchecked(node).get().unwrap().clone(), parent_context_id),
			None => {
				mark.remove(node.local()); // 移除脏标记
				(parent_context_id, parent_context_id)
			},
		};

		// 设置当前节点的InContext
		set_node_context(node, &mut in_context, cur_context_id);

		if let Some(c) = children_item {
			recursive_set_node_context(c.head, &idtree, &query, &mut in_context, child_context_id, mark);
		}
	}
}

/// 递归设置节点的上下文
fn recursive_set_node_context<'s>(
	node: Entity,
	idtree: &EntityTree<Node>,
	query: &Query<Node, (Option<&NodeDown>, Option<&RenderContext>)>,
	in_context: &mut Query<Node, Write<InContextId>>,
	context_id: InContextId,
	dirty_mark: &mut SecondaryMap<LocalVersion, usize>,
) {
	if node.is_null() {
		return;
	}
	set_node_context(node, in_context, context_id);
	let (children_item, context_item) = query.get_unchecked(node);

	// 不存在RenderContext, 才会继续递归下去
	if context_item.is_some() {
		return;
	}

	// 如果不是RenderContext，才会移除脏标记
	dirty_mark.remove(node.local());

	// 递归设置子节点
	if let Some(children_item) = children_item {
		for c in idtree.iter(children_item.head()) {
			recursive_set_node_context(
				c,
				idtree,
				query,
				in_context,
				context_id,
				dirty_mark,
			);
		}
	}
	
}

fn set_node_context(
	node: Entity,
	in_context: &mut Query<Node, Write<InContextId>>,
	context_id: InContextId,
) {
	let mut in_context_id_item = in_context.get_unchecked(node);
	match in_context_id_item.get() {
		Some(r) if *r == context_id => {},
		_ => {
			// 如果与旧的context_id不相等，则重新写入
			in_context_id_item.write(context_id);
		}
	};
}