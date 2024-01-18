use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::Event;

#[derive(Debug, Clone, Event)]
pub struct PassDirty(pub Entity);

/// 有节点删除
#[derive(Debug, Clone, Event)]
pub struct EntityDelete;

/// 有节点创建
#[derive(Debug, Clone, Event)]
pub struct EntityCreate;

/// 有节点zindex发生改变
#[derive(Debug, Clone, Event)]
pub struct NodeZindexChange;

/// 有节点Dispaly发生改变
#[derive(Debug, Clone, Event)]
pub struct NodeDisplayChange;