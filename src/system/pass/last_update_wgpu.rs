use std::mem::replace;

use bevy::prelude::{DetectChangesMut, Entity, Query, Res, ResMut, With};
use pi_bevy_ecs_extend::{
    prelude::Root,
    system_param::res::{OrInitRes, OrInitResMut},
};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{PiIndexBufferAlloter, PiRenderDevice, PiRenderQueue, PiVertexBufferAlloter};
use pi_render::rhi::{device::RenderDevice, RenderQueue};

use crate::{
    components::{
        draw_obj::DrawState,
        pass_2d::{Draw2DList, DrawIndex, PostProcess},
        user::Size,
    },
    resource::draw_obj::{DepthCache, DepthGroup, GroupAlloterCenter, ShareGroupAlloter}, system::draw_obj::calc_text::IsRun,
};

pub fn last_update_wgpu(
    query_root: Query<Entity, (With<Root>, With<Size>)>, // 只有gui的Root才会有Size

    mut draw_list_query: Query<&mut Draw2DList>,
    mut postprocess_query: Query<&mut PostProcess>,
    mut draw_state: Query<&mut DrawState>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    vertbuffer_alloter: OrInitRes<PiVertexBufferAlloter>,
    index_alloter: OrInitRes<PiIndexBufferAlloter>,
    group_alloc_center: Res<GroupAlloterCenter>,
    mut depth_cache: OrInitResMut<DepthCache>,
    mut post_resource: ResMut<PostprocessResource>,
    depth_group_alloter: OrInitRes<ShareGroupAlloter<DepthGroup>>,
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    // let depeth_group = group_alloter.alloc();
    // 			draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);
    for root in query_root.iter() {
        alloc_depth(
            &mut 0,
            root,
            &device,
            &queue,
            &mut post_resource,
            &mut draw_list_query,
            &mut postprocess_query,
            &mut draw_state,
            &mut depth_cache,
            &depth_group_alloter,
        );
    }

    group_alloc_center.write_buffer(&device, &queue);
    vertbuffer_alloter.write_buffer();
    index_alloter.write_buffer();
}


fn alloc_depth<'a1, 'a2, 'a3, 'a4, 'a6, 'a7>(
    cur_depth: &mut usize,
    pass2d_id: Entity,
    device: &RenderDevice,
    queue: &RenderQueue,
    post_resource: &mut PostprocessResource,
    draw_list_query: &mut Query<&'a1 mut Draw2DList>,
    post_process_query: &mut Query<&'a6 mut PostProcess>,
    draw_state_query: &mut Query<&'a7 mut DrawState>,
    depth_cache: &mut DepthCache,
    depth_alloter: &ShareGroupAlloter<DepthGroup>,
    // geometrys: &mut PostProcessGeometryManager,
    // postprocess_pipelines: &mut PostProcessMaterialMgr,
) {
    let mut old_all_list = None;
    if let Ok(mut list) = draw_list_query.get_mut(pass2d_id) {
        let mut post = post_process_query.get_mut(pass2d_id).unwrap();
        let post = post.bypass_change_detection();
        post.calc(16, &device, &queue, &mut post_resource.vballocator);
        post.depth = *cur_depth;
        *cur_depth += 1;

        list.opaque.clear();
        list.transparent.clear();
        if list.all_list.len() == 0 {
            return;
        }

        let mut all_list = replace(&mut list.all_list, Vec::new());
        let mut opaque = replace(&mut list.opaque, Vec::new());
        let mut transparent = replace(&mut list.transparent, Vec::new());
		// log::trace!("all_list======={:?}, {:?}", pass2d_id, all_list, );

        // 按深度从小到大排序
        all_list.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
            if a_z_depth.start < b_z_depth.start {
                std::cmp::Ordering::Less
            } else if a_z_depth.start > b_z_depth.start {
                std::cmp::Ordering::Greater
            } else {
                if a_sort.order() < b_sort.order() {
                    std::cmp::Ordering::Less
                } else if a_sort.order() > b_sort.order() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            // 用渲染管线排序，TODO
            // draw_state.get(a)
        });

        // for i in 0..all_list.len() {
        //     let (entity, _, draw_info) = list.all_list[i];
        //     // 暂时放入不透明列表，TODO
        //     if draw_info.is_opacity() {
        //         list.opaque.push((entity, 0));
        //     } else {
        //         list.transparent.push((entity, 0));
        //     }
        // }
        for (entity, _, draw_info) in all_list.drain(..) {
            depth_cache.or_create_depth(*cur_depth, depth_alloter);
            // 暂时放入不透明列表，TODO
            if draw_info.is_opacity() {
                opaque.push((entity, *cur_depth));
            } else {
                transparent.push((entity, *cur_depth));
            }
            *cur_depth += 1;

            // 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
            if let DrawIndex::Pass2D(pass2d_id) = entity {
                alloc_depth(
                    cur_depth,
                    *pass2d_id,
                    device,
                    queue,
                    post_resource,
                    draw_list_query,
                    post_process_query,
                    draw_state_query,
                    depth_cache,
                    depth_alloter,
                );
            }
        }
		// 清理列表
		all_list.clear();
        old_all_list = Some((all_list, opaque, transparent))
    }
    if let (Ok(mut list), Some((old_all_list, old_opaque, old_transparent))) = (draw_list_query.get_mut(pass2d_id), old_all_list) {
        list.all_list = old_all_list;

		
        list.opaque = old_opaque;
        list.transparent = old_transparent;
    }

    // if let Ok(children) = query_children.get(pass2d_id) {
    // 	for entity in children.iter() {
    // 		alloc_depth(
    // 			**entity,
    // 			query_children,
    // 			device,
    // 			queue,
    // 			post_resource,
    // 			draw_list_query,
    // 			post_process_query,
    // 			draw_state_query,
    // 			bind_group_assets,
    // 			depth_cache,
    // 		);
    // 	}
    // }
}

// fn alloc_depth_one<'a, 'w>(draw_key: Entity, draw_state: &'a mut Query<&'w mut DrawState>, cur_depth: &'a mut usize, depth_cache: &'a DepthCache) {
//     let mut draw_state = match draw_state.get_mut(draw_key) {
//         Ok(r) => r,
//         _ => return,
//     };
//     draw_state
//         .bypass_change_detection()
//         .bindgroups
//         .insert_group(DepthBind::set(), DrawBindGroup::Independ(depth_cache.list[*cur_depth].clone()));

//     *cur_depth += 1;
// }
