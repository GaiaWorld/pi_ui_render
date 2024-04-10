
use bevy_ecs::prelude::Event;


/// 有节点创建，删除， Append
#[derive(Debug, Clone, Event)]
pub struct EntityChange;

/// 有节点zindex发生改变
#[derive(Debug, Clone, Event)]
pub struct NodeZindexChange;

/// 有节点Dispaly发生改变
#[derive(Debug, Clone, Event)]
pub struct NodeDisplayChange;

/// 有节点Visibility发生改变
#[derive(Debug, Clone, Event)]
pub struct NodeVisibilityChange;