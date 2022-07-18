//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{Res, ResMut, res::WriteRes};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, asset::RenderRes, bind_group::BindGroup, buffer::Buffer};
use pi_share::Share;
use wgpu::IndexFormat;

use crate::{components::{user::Matrix4, calc::{ WorldMatrix}, pass_2d::RenderTarget, draw_obj::{VSDefines, FSDefines}}, resource::{draw_obj::{PipelineMap, ShaderInfoMap, UnitQuadBuffer, Shaders, ShaderCatch, VertexBufferLayoutMap, StateMap, ShareLayout, CopyFboToScreen, CommonSampler, EmptyBind}}, system::{pass::{ pass_graph_node::PostBindGroupLayout, pass_render::DepthCache}, draw_obj::{pipeline::CalcPipeline, world_marix::modify_world_matrix}, shader_utils::{create_camera_bind_group, post_process::{POST_TEXTURE_GROUP, CalcPostProcessShader, PostProcessStaticIndex}, create_depth_group}}, utils::{shader_helper::{PROJECT_GROUP, DEPTH_GROUP}, tools::calc_hash}};


pub struct CalcRoot;

#[setup]
impl CalcRoot {
	#[system]
	pub fn render_change(
		mut pipeline_map: ResMut<PipelineMap>,
		mut shader_map: ResMut<ShaderInfoMap>,

		unit_quad_buffer: Res<UnitQuadBuffer>,
		post_static_index: Res<PostProcessStaticIndex>,
		shader_statics: Res<Shaders>,
		device: Res<RenderDevice>,
		shader_catch: Res<ShaderCatch>,
		vertex_buffer_layout_map: Res<VertexBufferLayoutMap>,
		state_map: Res<StateMap>,
		share_layout: Res<ShareLayout>,

		buffer_assets: Res<Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<Share<AssetMgr<RenderRes<BindGroup>>>>,
		
		mut copy_draw_obj: WriteRes<CopyFboToScreen>,
		post_bind_group_layout: Res<PostBindGroupLayout>,
		common_sampler: Res<CommonSampler>,
		mut depth_cache: ResMut<DepthCache>,

		render_target: Res<RenderTarget>,
		empty_group: Res<EmptyBind>,
	) {
		if !render_target.is_changed() {
			return;
		}

		let target = if let RenderTarget::OffScreen(target) = &*render_target {
			target
		} else {
			return;
		};

		let mut draw_state = CalcPostProcessShader::create_draw_state(&empty_group);
		draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
		draw_state.vbs.insert(1, (unit_quad_buffer.uv.clone(), 0));
		draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));

		let pipeline = CalcPipeline::calc_pipeline(
			&VSDefines::default(),
			&FSDefines::default(),
			&post_static_index,

			&shader_statics,
			&device,
			&vertex_buffer_layout_map,
			&state_map,
			&shader_catch,

			&mut pipeline_map,
			&mut shader_map,
			&share_layout,
		);
		draw_state.pipeline = Some(pipeline);

		let project_bind_group = create_camera_bind_group(
			&WorldMatrix::default().0, 
			&share_layout.project, 
			&device, 
			&buffer_assets,
			&bind_group_assets,);
		draw_state.bind_groups.insert(PROJECT_GROUP, project_bind_group);

		// 世界矩阵
		let view = Matrix4::new(
			2.0, 0.0, 0.0, -1.0,
			0.0, 2.0, 0.0, -1.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		);
		modify_world_matrix(
			&WorldMatrix(view, false),
			&mut draw_state,
			&device,
			&share_layout.matrix,
			&buffer_assets,
			&bind_group_assets,
		);

		let depth_bind_group = create_depth_group(
			0, 
			&buffer_assets, 
			&bind_group_assets, 
			&mut depth_cache,
			&device,
			&share_layout);
		draw_state.bind_groups.insert(DEPTH_GROUP, depth_bind_group);

		let group_key = calc_hash(&("bind", target.target().colors[0].0.key() ), 0);

		let texture_bind = match bind_group_assets.get(&group_key) {
			Some(r) => r,
			None => {
				let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
					layout: &post_bind_group_layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::TextureView(&target.target().colors[0].0),
						},
					],
					label: Some("post process texture bind group create"),
				});
				bind_group_assets.insert(group_key, RenderRes::new(group, 5)).unwrap()
			},
		};
		draw_state.bind_groups.insert(POST_TEXTURE_GROUP, texture_bind);

		
		copy_draw_obj.write(CopyFboToScreen(draw_state));
	}

	// #[listen(component=(Node, Root, (Create, Delete)))]
	// pub fn root_change(
	// 	e: Event,
	// 	root: Query<Node, &Root>,
	// 	render_mark: Query<Node, Write<RenderContextMark>>,
	// 	local: Local<RenderContextMarkType>,
	// ) {
	// 	let root_item = root.get_by_entity(e.id);

	// 	let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
	// 	let mut render_mark_value = render_mark_item.get_or_default().clone();

	// 	match root_item {
	// 		Some(_) => {
	// 			render_mark_value.set(**local, true);
	// 		},
	// 		_ => {
	// 			render_mark_value.set(**local, false);
	// 			// 如果所有的位标记都被清除，则调用remove方法
	// 			if render_mark_value.not_any() {
	// 				render_mark_item.remove();
	// 				return;
	// 			}
	// 		},
	// 	};

	// 	render_mark_item.write(render_mark_value);
		
	// }
}

