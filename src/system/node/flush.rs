use std::{any::TypeId, mem::replace};

use pi_ecs::{prelude::{world::WorldMut, ArchetypeId, Local}, world::FromWorld};
use pi_ecs_macros::{setup};

use crate::{
    components::user::Node,
    resource::{UserCommands, UserCommandsCache}
};

/// 资源索引

pub struct CmdCache{
	node_archetype_id: ArchetypeId,
	// cmd_id: ResourceId,
}
impl FromWorld for CmdCache {
    fn from_world(world: &mut pi_ecs::world::World) -> Self {
		world.get_or_insert_resource::<UserCommandsCache>();
        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();
		Self{
			node_archetype_id,
			// cmd_id: world.archetypes().get_archetype_resource_id::<UserCommandsCache>().unwrap().clone()
		}
		
    }
}
// let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

pub struct CalcFlush;

#[setup]
impl CalcFlush {
    #[system]
    pub fn user_setting(
        mut world: WorldMut,
		local: Local<CmdCache>,
    ) {
        world.archetypes_mut()[local.node_archetype_id].flush();
        let commands = replace(world.get_resource_mut::<UserCommandsCache>().unwrap(), UserCommandsCache::default());
		let old = replace(world.get_resource_mut::<UserCommands>().unwrap(), commands.0);
		*world.get_resource_mut::<UserCommandsCache>().unwrap() = UserCommandsCache(old);
    }
}

