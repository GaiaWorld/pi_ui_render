use bevy::prelude::{EventReader, Changed, Query, Entity, Local};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_densevec::DenseVecMap;
use pi_map::Map;
use pi_null::Null;

use crate::components::{calc::{RenderContextMark, EntityKey}, pass_2d::{ChildrenPass, ParentPassId}};

/// Pass2D设置children
pub fn calc_pass_children_and_clear(
    event_reader: EventReader<ComponentEvent<Changed<RenderContextMark>>>,
    mut query: Query<&mut ChildrenPass>,
    query_pass: Query<(Entity, &ParentPassId)>,
    mut local: Local<DenseVecMap<(Entity, ChildrenPass)>>,
) {
    if event_reader.len() > 0 {
        event_reader.clear();
        // 重新组织渲染上下文的树
        for (entity, parent) in query_pass.iter() {
			if parent.0.is_null() {
				continue;
			}
            match local.get_mut(&(parent.index() as usize)) {
                Some(r) => r.1.push(EntityKey(entity)),
                None => {
                    let mut c = ChildrenPass::default();
                    c.push(EntityKey(entity));
                    local.insert(parent.index() as usize, ((***parent).clone(), c));
                }
            }
        }

        for item in local.values() {
            if let Ok(mut children) = query.get_mut(item.0) {
                *children = item.1.clone(); // 不clone, TODO
            }
        }

        local.clear();
    }
}