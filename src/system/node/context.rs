//! # 背景
//! 如Opacity、Blur、Hsv、MaskImage等，其效果需要将节点自身及其递归子节点作为一个整体来处理
//! 拥有这类属性的节点需要先将其自身包含的渲染对象和其递归子节点的渲染对象先渲染到一个fbo上，再将该fbo附加对应效果呈现出来。pass: 比如，opacity如果作用在每个渲染对象上，叠加后的效果是错误的
//!
//! # 思路
//! 本模块不关心具体需要成为渲染上下文的属性，而是只关心RenderContextMark组件，该组件存在时，则为渲染上下文，不存在时，就不是渲染上下文
//! 由于只关心RenderContextMark组件，而不关心具体属性（opacity、Blur等），外部可根据需要扩展属性，而不影响本模块的逻辑
//! 为每个渲染上下文节点，单独创建一个`Pass2D`来渲染其自身包含的渲染对象、以及其递归子节点上包含的渲染对象。
//!
//! # 具体逻辑
//! 本模块做以下事情：
//! 1. 为渲染上下文节点，创建Pass2D实体。pass: 通过监听RenderContextMark中的Create、Delete删除或创建Pass2D实体
//! 2. 在节点上，建立由其创建的Pass2D实体的索引(Pass2DId)，pass:当RenderContextMark组件删除，或Node实体销毁时，能够删除其对应的Pass2D实体
//! 3. 创建Pass2D对创建它的节点的索引(NodeId), 使得可以通过Pass2D反向查询到其对应节点上的组件
//! 4. 在节点上创建其所在的Pass2D实体的索引（InPass2DId），表明节点上的渲染对象应该渲染到那个Psss2D上。
//!
//!

use pi_dirty::LayerDirty;
use pi_ecs::storage::SecondaryMap;
use pi_ecs::{
    monitor::EventType,
    prelude::{Added, Commands, Deleted, EntityCommands, EntityDelete, EntityInsert, Event, Id, Or, ParamSet, Query, ResMut, Write},
};
use pi_ecs_macros::{listen, setup};
use pi_ecs_utils::prelude::{EntityTree, Layer, LayerDirty as LayerDirtyParam, NodeDown, NodeUp, Root};
use pi_null::Null;

use crate::components::user::Node;
use crate::{
    components::{
        calc::{InPassId, NodeId, Pass2DId, RenderContextMark},
        pass_2d::{ParentPassId, Pass2D},
        user::{Aabb2, Point2},
    },
    resource::draw_obj::LayerPass2D,
};

pub struct CalcContext;

#[setup]
impl CalcContext {
    /// system
    /// 记录RenderContext添加和删除的脏，同时记录节点添加到树上的脏
    /// 根据脏，从父向子递归，设置节点所在的渲染上下文（节点的渲染目标）
    #[system]
    pub fn cal_in_context_id(
        mut layer_pass_2d: ResMut<LayerPass2D>,
        mut command: EntityCommands<Pass2D>,
        dirty: LayerDirtyParam<Node, Or<(Added<RenderContextMark>, Deleted<RenderContextMark>)>>,
        idtree: EntityTree<Node>,
        query: Query<Node, (Option<&NodeDown<Node>>, Option<&RenderContextMark>, Option<&Pass2DId>)>,
        up: Query<Node, &NodeUp<Node>>,
        mut in_context: Query<Node, Write<InPassId>>,
        mut query_pass: Commands<Pass2D, ParentPassId>,

        // context_id: Query<Node, Write<Pass2DId>>,
        mut mark_context: Query<Node, (Option<&RenderContextMark>, Write<Pass2DId>)>,
        mut node_id: Commands<Pass2D, NodeId>,
    ) {
        // 当节点被挂在主树上，或者
        for (node, mark, layer) in dirty.iter_manual() {
            if let (Some(_mark), mut pass2d_id_item) = mark_context.get_unchecked_mut(node) {
                // 创建Pass2D
                if pass2d_id_item.get().is_none() {
                    let context = command.spawn();
                    // 建立Node到RenderContext的索引关系
                    pass2d_id_item.write(Pass2DId(context));
                    // 建立RenderContext到Node的索引关系
                    node_id.insert(context, NodeId(node));
                    layer_pass_2d.mark(context, layer);
                }
            }

            let parent_context_id = match up.get(node) {
                Some(r) =>
                // 自身已经不是一个Pass，则取父节点的InPassId
                {
                    in_context.get_unchecked(r.parent()).get().unwrap().clone()
                }
                None => InPassId(Id::<Pass2D>::null()),
            };
            recursive_set_node_context(node, &idtree, &query, &mut query_pass, &mut in_context, parent_context_id, mark);
        }
    }

    // /// 监听RenderContextMark和Layer的创建方法，创建渲染上下文（未考虑层修改的情况，是否需要？ TODO）
    // #[listen(component=(Node, RenderContextMark, Create), component=(Node, Layer, Create))]
    // pub fn context_mark_create(
    // 	e: Event,
    // 	context_id: Query<Node, Write<Pass2DId>>,
    // 	mark_layer: Query<Node, (&RenderContextMark, &Layer)>,
    // 	mut command: EntityCommands<Pass2D>,
    // 	mut node_id: Commands<NodeId>,

    // 	// mut render_context_layer_list: ResMut<RenderContextLayerList>,
    // ) {
    // 	// 不存在RenderContextMark或者不存在Layer，都不处理
    // 	// 成为渲染上下文的充要条件是：RenderContextMark组件存在，并且节点在渲染树上
    // 	if let Some((_, layer)) = mark_layer.get(e.id) {
    // 		// 创建一个RenderContext
    // 		let context = command.spawn();
    // 		// 建立Node到RenderContext的索引关系
    // 		context_id.get_unchecked(e.id).write(Pass2DId(context));
    // 		// 建立RenderContext到Node的索引关系
    // 		node_id.insert(context, NodeId(e.id));

    // 		// // 添加到渲染列表中（伴随着层信息，可以按层进行迭代）
    // 		// render_context_layer_list.mark(context, **layer);
    // 	}
    // }

    /// 监听RenderContextMark、Layer、Node的移除事件，移除对应的渲染上下文
    #[listen(component=(Node, RenderContextMark, Delete), component=(Node, Layer, Delete), entity=(Node, Delete))]
    pub fn context_mark_remove(
        e: Event,
        context_id: Query<Node, Write<Pass2DId>>,
        layer: Query<Node, &Layer>,
        mut command: EntityDelete<Pass2D>,
        mut layer_pass_2d: ResMut<LayerPass2D>,
        // mut render_context_layer_list: ResMut<RenderContextLayerList>,
    ) {
        if let (Some(context), Some(layer)) = (
            context_id.get_unchecked_by_entity(e.id).remove(),
            layer.get(unsafe { Id::<Node>::new(e.id.local()) }),
        ) {
            // 删除RenderContext实体
            command.despawn(*context);
            layer_pass_2d.delete(*context, **layer);
            // render_context_layer_list.delete(*context, **layer);
        }
    }

    #[listen(component=(Node, Root, (Create, Delete)))]
    pub fn root_change(
        e: Event,
        mut pass2d_query: ParamSet<(EntityInsert<Pass2D>, EntityDelete<Pass2D>, Query<Pass2D, Write<NodeId>>)>,

        mut node_query: Query<Node, Write<Pass2DId>>,
    ) {
        match e.ty {
            EventType::Create => {
                let id = pass2d_query.p0_mut().spawn();
                node_query.get_unchecked_mut_by_entity(e.id).write(Pass2DId(id));
                let mut node_id = pass2d_query.p2_mut().get_unchecked_mut(id);
                node_id.write(NodeId(unsafe { Id::<Node>::new(e.id.local()) }));
            }
            EventType::Delete => {
                let id = node_query.get_unchecked_mut_by_entity(e.id);
                pass2d_query.p1_mut().despawn(id.get().unwrap().0);
            }
            _ => (),
        }
    }
}


/// 渲染上下文列表，组织为层的结构，渲染时，按照层的顺序，从大到小渲染（因为父上下文的渲染依赖子上下文的渲染）
#[derive(Deref, DerefMut, Default)]
pub struct RenderContextLayerList(LayerDirty<Id<Node>>);

/// 脏区域
#[derive(Deref, DerefMut)]
pub struct DirtyRect(Aabb2);

impl Default for DirtyRect {
    fn default() -> Self { Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0))) }
}

/// 递归设置节点的上下文
fn recursive_set_node_context<'s>(
    node: Id<Node>,
    idtree: &EntityTree<Node>,
    query: &Query<Node, (Option<&NodeDown<Node>>, Option<&RenderContextMark>, Option<&Pass2DId>)>,
    query_pass: &mut Commands<Pass2D, ParentPassId>,
    in_context: &mut Query<Node, Write<InPassId>>,
    parent_context_id: InPassId,
    dirty_mark: &mut SecondaryMap<Id<Node>, usize>,
) {
    if node.is_null() {
        return;
    }
    let (children_item, mark, pass2d_id) = query.get_unchecked(node);
    let in_context_id = match pass2d_id {
        Some(r) => {
            query_pass.insert(**r, ParentPassId(*parent_context_id));

            let in_context_item = in_context.get_unchecked(node);
            if let Some(in_pass_id) = in_context_item.get() {
                // 已经正确设置过in_pass_id, 则返回
                if **in_pass_id == **r {
                    return;
                }
            }
            InPassId(**r)
        }
        None => {
            if let Some(_r) = mark {
                // 如果存在context_mark,则返回，将在接下来的迭代中处理
                return;
            } else {
                parent_context_id
            }
        }
    };
    set_node_context(node, in_context, in_context_id);

    // 如果不是RenderContext，才会移除脏标记
    dirty_mark.remove(node);


    // 递归设置子节点
    if let Some(children_item) = children_item {
        for c in idtree.iter(children_item.head()) {
            recursive_set_node_context(c, idtree, query, query_pass, in_context, in_context_id, dirty_mark);
        }
    }
}

fn set_node_context(node: Id<Node>, in_context: &mut Query<Node, Write<InPassId>>, context_id: InPassId) {
    let mut in_context_id_item = in_context.get_unchecked(node);
    match in_context_id_item.get() {
        Some(r) if *r == context_id => {}
        _ => {
            // 如果与旧的context_id不相等，则重新写入
            in_context_id_item.write(context_id);
        }
    };
}
