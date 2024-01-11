use bevy_ecs::prelude::RemovedComponents;
use bevy_ecs::query::Changed;
use bevy_ecs::system::{Query, SystemState};
use bevy_ecs::prelude::{Bundle, Commands, Component, EventReader, FromWorld, Resource, World};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_render::rhi::shader::{BindLayout, ShaderProgram};
use pi_share::Share;

use crate::components::calc::{DrawInfo, EntityKey, NodeId};
use crate::components::draw_obj::{BoxType, PipelineMeta};
use crate::components::DrawBundle;
use crate::resource::draw_obj::{ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup, VertexBufferLayoutWithHash};
use crate::resource::RenderObjType;

use crate::components::{calc::DrawList, draw_obj::DrawState};
use crate::shader::ui_meterial::UiMaterialBind;

use super::calc_text::IsRun;

// 创建或删除DrawObject
pub fn draw_object_life<
    Src: Component,
    RenderType: Resource + std::ops::Deref<Target = RenderObjType> + FromWorld,
    With: Bundle + Default, // 初始化时额外需要插入的组件
    VertLayout: Resource + std::ops::Deref<Target = Share<VertexBufferLayoutWithHash>> + FromWorld,
    Program: ShaderProgram,
    const ORDER: u8,
>(
    world: &mut World,

    state: &mut SystemState<(
        OrInitRes<RenderType>,
        EventReader<ComponentEvent<Changed<Src>>>,
        RemovedComponents<Src>,
        Query<(Option<&'static Src>, &'static mut DrawList)>,
        OrInitRes<ProgramMetaRes<Program>>,
        OrInitRes<VertLayout>,
        OrInitRes<ShaderInfoCache>,
        OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
        Commands,
		OrInitRes<IsRun>,
    )>,

	// #[allow(dead_code)]
    // query_draw_list: &mut SystemState<Query<&'static mut DrawList>>, 
) {
	// let time1 = pi_time::Instant::now();
    let (render_type, mut changed, mut del, mut query_texture, program_meta, vert_layout, shader_catch, group_alloter, mut commands, r) =
        state.get_mut(world);
	if r.0 {
		return;
	}
    let group_alloter = group_alloter.clone();
    let render_type = ***render_type;

	// let mut count1 = 0;
	// let mut count2 = 0;

    // 收集需要删除DrawObject的实体
    for del in del.iter() {
		// count1 += 1;
        if let Ok((texture, mut draw_list)) = query_texture.get_mut(del) {
            if texture.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            draw_list.remove(render_type, |draw_obj| {
				if let Some(mut r) = commands.get_entity(draw_obj.id) {
					r.despawn();
					log::warn!("despawn drawobj====={:?}", draw_obj.id);
					log::debug!(target: format!("entity_{:?}", del).as_str(), "remove RenderObj {:?} for {} destroy, ", &draw_obj.id, std::any::type_name::<Src>());
				}
			});
        }
    }

    let program_meta = program_meta.clone();
    let p_state = shader_catch.common.clone();
    let vert_layout = vert_layout.clone();
	// let time2 = pi_time::Instant::now();

	// let mut spawn_list = Vec::new();
    // 收集需要创建DrawObject的实体
    for changed in changed.iter() {
		// count2 += 1;
        if let Ok((texture, mut draw_list)) = query_texture.get_mut(changed.id) {
            if texture.is_none() {
                continue;
            }
            // 不存在，才需要创建DrawObject
            if let None = draw_list.get_one(render_type) {
                let mut draw_state = DrawState::default();
                let ui_material_group = group_alloter.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                let id = commands
                    .spawn(DrawBundle {
                        node_id: NodeId(EntityKey(changed.id)),
                        draw_state,
                        box_type: BoxType::ContentNone,
                        pipeline_meta: PipelineMeta {
                            type_mark: render_type,
                            program: program_meta.clone(),
                            state: p_state.clone(),
                            vert_layout: vert_layout.clone(),
                            defines: Default::default(),
                        },
                        draw_info: DrawInfo::new(ORDER, false), //TODO
                        other: With::default(),
                    })
                    .id();
				// spawn_list.push(id);
                log::debug!(target: format!("entity_{:?}", changed.id).as_str(), "create RenderObj {:?} for {} changed, ", &id, std::any::type_name::<Src>());
                draw_list.push(render_type, id);
				// log::warn!("create drawobj=================draw={:?}, node={:?}", id, changed.id);
            }
        }
    }
	// if spawn_list.len() > 0 {
	// 	log::warn!("spawn drawobj=================draw={:?}", &spawn_list);
	// }

    state.apply(world);
	// let time3 = pi_time::Instant::now();
	// log::warn!("life======{:?}, {:?}, {:?}, {:?}, {:?}", std::any::type_name::<Src>(), time2 - time1, time3 - time2, count1, count2);
}


// // 创建或删除DrawObject
// pub fn draw_object_life<
// 	Src: Component,
// 	RenderType: Resource + std::ops::Deref<Target=RenderObjType> + FromWorld,
// 	With: Bundle + Default, // 初始化时额外需要插入的组件
// 	VertLayout: Resource + std::ops::Deref<Target=Share<VertexBufferLayoutWithHash>> + FromWorld,
// 	Program: ShaderProgram,
// 	const ORDER: u8,
// >(
// 	// world: &mut World,
// 	// mut will_creates: Local<Vec<(Entity, EntityKey)>>,
// 	// mut will_delete: Local<Vec<Entity>>,

// 	// state: &mut SystemState<(
// 	// 	OrInitRes<RenderType>,
// 	// 	EventReader<ComponentEvent<Changed<Src>>>,
// 	// 	RemovedComponents<Src>,
// 	// 	Query<(Option<&'static Src>, &'static mut DrawList)>,

// 	// 	OrInitRes<ProgramMetaRes<Program>>,
// 	// 	OrInitRes<VertLayout>,
// 	// 	OrInitRes<ShaderInfoCache>,
// 	// 	OrInitRes<ShareGroupAlloter<UiMaterialGroup>>
// 	// )>,

// 	// query_draw_list: &mut SystemState<Query<&'static mut DrawList>>,

// 	mut entity_creator: EntityCreator<DrawBundle<With>>,
// 	mut will_creates: Local<Vec<(Entity, EntityKey)>>,
// 	mut will_delete: Local<Vec<Entity>>,
// 	render_type: OrInitRes<RenderType>,
// 	mut changed: EventReader<ComponentEvent<Changed<Src>>>,
// 	mut del: RemovedComponents<Src>,

// 	program_meta: OrInitRes<ProgramMetaRes<Program>>,
// 	vert_layout: OrInitRes<VertLayout>,
// 	shader_catch: OrInitRes<ShaderInfoCache>,
// 	group_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
// 	mut query: ParamSet<(
// 		Query<(Option<&'static Src>, &'static mut DrawList)>,
// 		Query<&'static mut DrawList>,)>,
// 	mut commands: Commands,

// ) {
// 	// let (render_type, mut changed, mut del, mut query_texture, program_meta, vert_layout, shader_catch, group_alloter) = state.get_mut(world);
// 	let render_type = ****render_type as u32;
// 	let group_alloter = group_alloter.clone();

// 	// 收集需要删除DrawObject的实体
// 	let mut query_texture = query.p0();
// 	for del in del.iter() {
// 		if let Ok((texture, mut draw_list)) = query_texture.get_mut(del) {
//             if texture.is_some() {
//                 continue;
//             }
//             // 删除对应的DrawObject
//             if let Some(draw_obj) = draw_list.remove(render_type) {
// 				will_delete.push(draw_obj);
//             }
//         }
//     }

// 	// 收集需要创建DrawObject的实体
// 	for changed in changed.iter() {
// 		if let Ok((texture, draw_list)) = query_texture.get(changed.id) {
//             if texture.is_none() {
//                 continue;
//             }
//             // 不存在，才需要创建DrawObject
//             if let None = draw_list.get(render_type) {
// 				will_creates.push((changed.id, EntityKey::null()));
//             }
//         }
//     }

// 	let program_meta = program_meta.clone();
// 	let state = shader_catch.common.clone();
// 	let vert_layout = vert_layout.clone();

// 	// 删除DrawObject实体
// 	for del in will_delete.drain(..) {
// 		if let Some(mut e) = commands.get_entity(del) {
// 			e.despawn();
// 		}
//     }

// 	// 创建DrawObject
// 	for (create, draw_obj) in will_creates.iter_mut() {
// 		let mut draw_state = DrawState::default();
// 		let ui_material_group = group_alloter.alloc();
// 		draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

// 		*draw_obj = EntityKey(entity_creator.spawn(
// 			DrawBundle {
// 				node_id: NodeId(EntityKey(*create)),
// 				draw_state,
// 				box_type: BoxType::ContentNone,
// 				pipeline_meta: PipelineMeta {
// 					program: program_meta.clone(),
// 					state: state.clone(),
// 					vert_layout: vert_layout.clone(),
// 					defines: Default::default(),
// 				},
// 				draw_info: DrawInfo::new(ORDER, false), //TODO
// 				other: With::default(),
// 			}));
//     }

// 	// let mut query_draw_list = query_draw_list.get_mut(world);
// 	let mut query_draw_list = query.p1();
// 	// 创建Node到DrawObject的映射
// 	for (create, draw_obj) in will_creates.drain(..) {
// 		if let Ok(mut draw_list) = query_draw_list.get_mut(create) {
// 			draw_list.insert(render_type, draw_obj.0);
// 		}
// 	}
// }
