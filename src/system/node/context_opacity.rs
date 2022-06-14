//! 处理opacity属性，对opacity设置小于1.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_assets::mgr::AssetMgr;
use pi_ecs::{monitor::Event, prelude::{Query, Write, Or, Changed, Deleted, FromWorld, Res, Commands, EntityCommands}, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout, asset::RenderRes, buffer::Buffer, bind_group::BindGroup};
use pi_share::Share;
use pi_slotmap::{DefaultKey, KeyData};
use wgpu::IndexFormat;

use crate::{components::{user::{Node, Opacity}, calc::{RenderContextMark, Pass2DId, NodeId}, pass_2d::{PostProcessList, Pass2D, PostProcess}, draw_obj::{DrawObject, DrawState, ShaderKey, PipelineKey, VertexBufferLayoutKey, FSDefines}}, resource::{RenderContextMarkType, draw_obj::{UnitQuadBuffer, Shaders}}, system::shader_utils::post_process::{PostProcessStaticIndex, POST_TEXTURE_GROUP, POST_UV_LOCATION, OPACITY_GROUP}, utils::tools::calc_float_hash};

pub struct CalcOpacity;

#[derive(Deref)]
pub struct OpacityRenderContextMarkType(RenderContextMarkType);

impl FromWorld for OpacityRenderContextMarkType{
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
        Self(RenderContextMarkType::from_world(world))
    }
}

#[setup]
impl CalcOpacity {
	#[listen(component=(Node, Opacity, (Create, Modify, Delete)))]
	pub fn opacity_change(
		e: Event,
		opacity: Query<Node, &Opacity>,
		render_mark: Query<Node, Write<RenderContextMark>>,
		mark_type: Res<OpacityRenderContextMarkType>,
	) {
		let opacity_item = opacity.get_by_entity(e.id);

		let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
		let mut render_mark_value = render_mark_item.get_or_default().clone();

		match opacity_item {
			Some(opacity_item) if **opacity_item < 1.0 => {
				render_mark_value.set(***mark_type, true);
			},
			_ => {
				render_mark_value.set(***mark_type, false);
				if render_mark_value.not_any() {
					render_mark_item.remove();
					return;
				}
			},
		};

		render_mark_item.write(render_mark_value);
		
	}
}

/// 计算半透明后处理
pub struct CalcOpacityPostProcess;

#[setup]
impl CalcOpacityPostProcess {
	#[system]
	pub fn opacity_change(
		opacity_dirty: Query<Node, (Id<Node>, Option<&Opacity>, Option<&Pass2DId>), Or<(Changed<Opacity>, Deleted<Opacity>)>>,
		mark_type: Res<OpacityRenderContextMarkType>,
		device: Res<RenderDevice>,
		mut pass_query: Query<Pass2D, Write<PostProcessList>>,
		mut query_draw: Query<DrawObject, Write<DrawState>>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut shader_id_commands: Commands<DrawObject, ShaderKey>,
		mut pipeline_state_commands: Commands<DrawObject, PipelineKey>,
		mut vertex_buffer_layout_commands: Commands<DrawObject, VertexBufferLayoutKey>,
		mut fs_defines_commands: Commands<DrawObject, FSDefines>,

		unit_quad_buffer: Res<UnitQuadBuffer>,
		static_index: Res<PostProcessStaticIndex>,
		shader_static: Res<'static, Shaders>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
	) {
		for (node, opacity, pass2d_id) in opacity_dirty.iter() {
			let pass2d_id = match pass2d_id {
				Some(r) => r,
				None => continue
			};

			match (opacity, pass_query.get_mut(pass2d_id.0)) {
				(Some(opacity), Some(mut post_list)) if opacity.0 < 1.0 => {
					let post_list = post_list.get_mut_or_default();
					let post_key = DefaultKey::from(KeyData::from_ffi(***mark_type as u64));

					let opacity_group_layout = match shader_static.get(static_index.shader) {
						Some(r) => r.bind_group.get(OPACITY_GROUP).unwrap(),
						None => return,
					};

					match post_list.0.get(post_key) {
						Some(r) => {
							let mut darw_obj = query_draw.get_unchecked_mut(r.draw_obj_key);
							modify_opacity_group(
								opacity, 
								darw_obj.get_mut().unwrap(), 
								&device, 
								opacity_group_layout, 
								&buffer_assets, 
								&bind_group_assets);
						},
						None => {
							let new_draw_obj = draw_obj_commands.spawn();
							// 设置DrawState（包含color group）
							let mut draw_state = DrawState::default();
							draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
							draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
							// opacity
							modify_opacity_group(
								opacity, 
								&mut draw_state,
								&device, 
								opacity_group_layout,
								&buffer_assets, 
								&bind_group_assets);
							
							draw_state_commands.insert(new_draw_obj, draw_state);
							// 建立DrawObj对Node的索引
							node_id_commands.insert(new_draw_obj, NodeId(node));
							//shader
							shader_id_commands.insert(new_draw_obj, ShaderKey(static_index.shader));
							// pipeline
							pipeline_state_commands.insert(new_draw_obj, PipelineKey(static_index.pipeline_state));
							// vertex_buffer_layout
							vertex_buffer_layout_commands.insert(new_draw_obj, VertexBufferLayoutKey(static_index.vertex_buffer_index));
							// fs defines - OPACITY
							let mut fs_defines = FSDefines::default();
							fs_defines.insert("OPACITY".to_string());
							fs_defines_commands.insert(new_draw_obj, fs_defines);

							// 创建PostPprocess,并插入后处理列表中
							let post_process = PostProcess::new(
								new_draw_obj,
								POST_TEXTURE_GROUP,
								POST_UV_LOCATION,
								0,
								0,
							);
							post_list.0.insert(post_key, post_process);
						},
					};
				},
				(_, None) => {},
				(_, Some(mut post_list)) =>  {
					// opacity不存在，或者opacity为1.0，则删除对应的后处理
					if let Some(post_list) = post_list.get_mut() {
						let post_key = DefaultKey::from(KeyData::from_ffi(***mark_type
							 as u64));
						if let Some(post_process) = post_list.0.remove(post_key) {
							draw_obj_commands.despawn(post_process.draw_obj_key);
						}
					}
				},
			}
		}
	}
}


fn modify_opacity_group(
	opacity: &Opacity,
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	opacity_group_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) {
	let key = calc_float_hash(&[opacity.0]);
	let group = match bind_group_assets.get(&key) {
		Some(r) => r,
		None => {
			let uniform_buf = match buffer_assets.get(&key) {
				Some(r) => r,
				None => {
					let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("color buffer init"),
						contents: bytemuck::cast_slice(&[opacity.0]),
						usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
					});
					buffer_assets.cache(key, RenderRes::new(uniform_buf, 5));
					buffer_assets.get(&key).unwrap()
				}
			};
			let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: opacity_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: uniform_buf.as_entire_binding(),
					},
				],
				label: Some("opacity group create"),
			});
			bind_group_assets.cache(key, RenderRes::new(group, 5));
			bind_group_assets.get(&key).unwrap()
		}
	};
	// 插入到drawstate中
	draw_state.bind_groups.insert(OPACITY_GROUP, group);
	
}



