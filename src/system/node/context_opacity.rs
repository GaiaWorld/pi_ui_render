//! 处理opacity属性，对opacity设置小于1.0的节点，标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_assets::mgr::AssetMgr;
use pi_ecs::{monitor::Event, prelude::{Query, Write, Or, Changed, Deleted, FromWorld, Res, Commands, EntityCommands, ResMut}, entity::Id};
use pi_ecs_macros::{listen, setup};
use pi_render::rhi::{device::RenderDevice, asset::RenderRes, buffer::Buffer, bind_group::BindGroup, dyn_uniform_buffer::{Bind, Group}};
use pi_share::Share;
use pi_slotmap::{DefaultKey, KeyData};
use wgpu::IndexFormat;
use smallvec::smallvec;

use crate::{components::{user::{Node, Opacity}, calc::{RenderContextMark, Pass2DId, NodeId}, pass_2d::{PostProcessList, Pass2D, PostProcess}, draw_obj::{DrawObject, DrawState, FSDefines, DrawGroup, DynDrawGroup}}, resource::{RenderContextMarkType, draw_obj::{UnitQuadBuffer, Shaders, StaticIndex, ImageStaticIndex, DynUniformBuffer, DynBindGroupIndex, CommonPipelineState}}, shaders::image::{ImageMaterialBind, ImageMaterialGroup, PositionVertexBuffer, OpacityUniform, SampTex2DGroup, UvVertexBuffer}};

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
		mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		mut fs_defines_commands: Commands<DrawObject, FSDefines>,

		unit_quad_buffer: Res<UnitQuadBuffer>,
		static_index: Res<ImageStaticIndex>,
		common_state: Res<CommonPipelineState>,
		shader_static: Res<'static, Shaders>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,

		mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
		image_material_bind_group: Res<'static, DynBindGroupIndex<ImageMaterialGroup>>,
	) {
		let mut static_index = (*static_index).clone();
		static_index.pipeline_state = common_state.premultiply.clone();

		for (node, opacity, pass2d_id) in opacity_dirty.iter() {
			let pass2d_id = match pass2d_id {
				Some(r) => r,
				None => continue
			};

			match (opacity, pass_query.get_mut(pass2d_id.0)) {
				(Some(opacity), Some(mut post_list)) if opacity.0 < 1.0 => {
					let post_list = post_list.get_mut_or_default();
					let post_key = DefaultKey::from(KeyData::from_ffi(***mark_type as u64));
					match post_list.0.get(post_key) {
						Some(r) => {
							let mut darw_obj = query_draw.get_unchecked_mut(r.draw_obj_key);
							modify_opacity_group(
								opacity, 
								darw_obj.get_mut().unwrap(),
								&mut dyn_uniform_buffer);
						},
						None => {
							let new_draw_obj = draw_obj_commands.spawn();
							// 设置DrawState（包含color group）
							let mut draw_state = DrawState::default();

							let image_material_dyn_offset = dyn_uniform_buffer.alloc_binding::<ImageMaterialBind>();
							let group = DrawGroup::Dyn(
								DynDrawGroup::new(
									(*image_material_bind_group).clone(),
									smallvec![image_material_dyn_offset]
								));
							draw_state.bind_groups.insert_group(ImageMaterialGroup::id(), group);

							draw_state.vbs.insert(PositionVertexBuffer::id() as usize, (unit_quad_buffer.vertex.clone(), 0));
							draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
							// opacity
							modify_opacity_group(
								opacity, 
								&mut draw_state,
								&mut dyn_uniform_buffer);
							
							draw_state_commands.insert(new_draw_obj, draw_state);
							// 建立DrawObj对Node的索引
							node_id_commands.insert(new_draw_obj, NodeId(node));

							shader_static_commands.insert(new_draw_obj, static_index.clone());
							// fs defines - OPACITY
							let mut fs_defines = FSDefines::default();
							fs_defines.insert("OPACITY".to_string());
							fs_defines_commands.insert(new_draw_obj, fs_defines);

							// 创建PostPprocess,并插入后处理列表中
							let post_process = PostProcess::new(
								new_draw_obj,
								SampTex2DGroup::id() as usize,
								UvVertexBuffer::id() as usize,
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
	dyn_uniform_buffer: &mut DynUniformBuffer,
) {
	let dyn_offset = draw_state.bind_groups.get_group(ImageMaterialGroup::id()).unwrap().get_offset(ImageMaterialBind::index()).unwrap();
	dyn_uniform_buffer.set_uniform(dyn_offset, &OpacityUniform(&[opacity.0]));
	
}
