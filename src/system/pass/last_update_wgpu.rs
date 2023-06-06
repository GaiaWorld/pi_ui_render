use bevy::prelude::{Query, Entity, Res, DetectChangesMut, With, ResMut};
use pi_assets::mgr::AssetMgr;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::{system_param::res::{OrInitRes, OrInitResMut}, prelude::Root};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, PiVertexBufferAlloter, PiIndexBufferAlloter};
use pi_render::{rhi::{device::RenderDevice, asset::RenderRes, bind_group::BindGroup, shader::BindLayout, RenderQueue}, renderer::draw_obj::DrawBindGroup};
use pi_share::Share;

use crate::{components::{pass_2d::{Draw2DList, DrawIndex, ChildrenPass, PostProcess}, draw_obj::DrawState, user::Size}, resource::draw_obj::{GroupAlloterCenter, DepthCache}, shader::depth::DepthBind};

pub fn last_update_wgpu(
	query_root: Query<(Entity, &ChildrenPass), (With<Root>, With<Size>)>, // 只有gui的Root才会有Size

	mut draw_list_query: Query<&mut Draw2DList>,
	mut postprocess_query: Query<&mut PostProcess>,
	mut draw_state: Query<&mut DrawState>,
	device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
	bind_group_assets: Res<ShareAssetMgr<RenderRes<BindGroup>>>,
	vertbuffer_alloter: OrInitRes<PiVertexBufferAlloter>,
	index_alloter: OrInitRes<PiIndexBufferAlloter>,
	group_alloc_center: Res<GroupAlloterCenter>,
	mut depth_cache: OrInitResMut<DepthCache>,
	mut post_resource: ResMut<PostprocessResource>,
) {
	for (root, children) in query_root.iter() {
		alloc_depth(
			root,
            &device,
			&queue,
			&mut post_resource,
            &mut draw_list_query,
            &mut postprocess_query,
            &mut draw_state,
            &bind_group_assets,
            &mut depth_cache,
        );
		for entity in children.iter() {
			alloc_depth(
				**entity,
				&device,
				&queue,
				&mut post_resource,
				&mut draw_list_query,
				&mut postprocess_query,
				&mut draw_state,
				&bind_group_assets,
				&mut depth_cache,
			);
		}
	}

	group_alloc_center.write_buffer(&device, &queue);
    vertbuffer_alloter.write_buffer();
	index_alloter.write_buffer();

}

fn alloc_depth<'a1, 'a2, 'a3, 'a6, 'a7>(
	pass2d_id: Entity,
    device: &RenderDevice,
	queue: &RenderQueue,
	post_resource: &mut PostprocessResource,
    draw_list_query: &mut Query<&'a1 mut Draw2DList>,
    post_process_query: &mut Query<&'a6 mut PostProcess>,
    draw_state_query: &mut Query<&'a7 mut DrawState>,
    bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
    depth_cache: &mut DepthCache,
    // geometrys: &mut PostProcessGeometryManager,
    // postprocess_pipelines: &mut PostProcessMaterialMgr,
) {
	if let Ok(mut list) = draw_list_query.get_mut(pass2d_id) {
		let mut post = post_process_query.get_mut(pass2d_id).unwrap();
		post.calc(
			16,
			&device,
			&queue,
			&mut post_resource.vballocator,
		);

		let mut cur_depth = post.depth as usize;
		for index in list.all_list.drain(..) {
			match &index.0 {
				// 如果绘制索引是一个DrawObj，则设置该DrawObj的depth group
				DrawIndex::DrawObj(draw_key) => {
					depth_cache.or_create_depth(cur_depth, device, bind_group_assets);
					alloc_depth_one(**draw_key, draw_state_query, &mut cur_depth, depth_cache);
				}
				DrawIndex::DrawObjPost(draw_key) => {
					if let Ok(mut r) = post_process_query.get_mut(**draw_key) {
						r.depth = cur_depth;
						cur_depth += 1;
					};
				}
				// 如果绘制索引是一个pass2d，则为该pass2d中的渲染对象设置depth group
				DrawIndex::Pass2D(pass2d_id) => {
					if let Ok(mut post) = post_process_query.get_mut(**pass2d_id) {
						post.depth = cur_depth;
						cur_depth += 1;
					}
				}
			}
		}

		
	}
}

fn alloc_depth_one<'a, 'w>(draw_key: Entity, draw_state: &'a mut Query<&'w mut DrawState>, cur_depth: &'a mut usize, depth_cache: &'a DepthCache) {
    let mut draw_state = match draw_state.get_mut(draw_key) {
        Ok(r) => r,
        _ => return,
    };
    draw_state
        .bypass_change_detection()
        .bindgroups
        .insert_group(DepthBind::set(), DrawBindGroup::Independ(depth_cache.list[*cur_depth].clone()));

    *cur_depth += 1;
}
