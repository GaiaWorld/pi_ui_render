use std::borrow::BorrowMut;

use bevy::ecs::prelude::{DetectChanges, Ref};
use bevy::ecs::query::{Changed, Or, With};
use bevy::prelude::{IntoSystemConfig, Component};
use bevy::ecs::system::{Local, Query, Res, SystemState, SystemParam};
use bevy::prelude::{Without, World, Entity, EventReader, RemovedComponents, ParamSet, EventWriter, ResMut, Plugin};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::Layer;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::component::GraphId;
use pi_bevy_render_plugin::node::{Node, ParamUsage};
use pi_bevy_render_plugin::{PiRenderGraph, SimpleInOut, RenderContext, PiSafeAtlasAllocator, PiRenderDevice, PiRenderQueue};
use pi_futures::BoxFuture;
use pi_postprocess::prelude::{PostprocessTexture, BlurGauss};
use pi_render::components::view::target_alloc::ShareTargetView;
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::renderer::texture::ETextureViewUsage;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderVertices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::pipeline::RenderPipeline;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_render::rhi::texture::PiRenderDefault;
use pi_share::ShareRefCell;
use pi_slotmap::Key;
use wgpu::CommandEncoder;

use crate::components::DrawBundle;
use crate::components::calc::{NodeState, NodeId, DrawList, EntityKey, DrawInfo, RenderContextMark, WorldMatrix};
use crate::components::draw_obj::{PipelineMeta, TextMark, TextShadowMark, BoxType, DynTargetType};
use crate::components::pass_2d::{PostProcess, Camera, ParentPassId};
use crate::components::user::{TextShadow, Matrix4};
use crate::resource::draw_obj::{EmptyVertexBuffer, TextTextureGroup, ProgramMetaRes, PosUvColorVertexLayout, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup, ClearDrawObj, DynFboClearColorBindGroup};
use crate::resource::{ShareFontSheet, TextShadowRenderObjType, RenderContextMarkType, TextRenderObjType};
use crate::shader::camera::CameraBind;
use crate::shader::depth::DepthBind;
use crate::shader::text::{PositionVert, SampBind, UvVert};
use crate::shader::ui_meterial::{ColorUniform, TextureSizeOrBottomLeftBorderUniform, UiMaterialBind, StrokeColorOrURectUniform, WorldUniform};
use crate::components::draw_obj::DrawState;
use crate::system::AddEvent;
use crate::system::pass::pass_life::{render_mark_true, cal_context};
use crate::system::pass::pass_graph_node::create_rp_for_fbo;
use crate::system::pass::update_graph::{get_to, update_graph};
use crate::system::system_set::UiSystemSet;
use crate::shader::text::SHADOW_DEFINE;

use super::text::calc_text;


/// 文字阴影插件
pub struct UiTextShadowPlugin;

impl Plugin for UiTextShadowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_frame_event::<ComponentEvent<Changed<TextShadow>>>()
            .add_system(text_shadow_life.in_set(UiSystemSet::LifeDrawObject).in_set(UiSystemSet::PassMark).before(cal_context),)
            .add_system(calc_text_shadow.in_set(UiSystemSet::PrepareDrawObj).in_set(UiSystemSet::PassSetting).after(calc_text))
			.add_system(calc_graph_depend.in_set(UiSystemSet::PassCalc).after(update_graph))
		;
	}
}

pub const TEXT_SHADOW_ORDER: u8 = 7;

#[derive(Debug, Component)]
pub struct TextShadowColorBindGroup(DrawBindGroup);

// 文字阴影的生命周期管理
// PosUvColorVertexLayout,
//                     crate::shader::text::ProgramMeta
pub fn text_shadow_life(
    world: &mut World,
    mut will_creates: Local<Vec<(Entity, usize, usize)>>, // (节点id， 开始索引，阴影数量)
	mut will_create_draws: Local<Vec<Entity>>, // drawObj的id
    mut will_delete: Local<Vec<Entity>>,

    state: &mut SystemState<(
        OrInitRes<TextShadowRenderObjType>,
        EventReader<ComponentEvent<Changed<TextShadow>>>,
        RemovedComponents<TextShadow>,
		ParamSet<(
			Query<(Option<&'static TextShadow>, &'static mut DrawList, &'static mut RenderContextMark)>,
			Query<(&'static TextShadow, &'static mut DrawList, &'static mut RenderContextMark), Changed<TextShadow>>,
			Query<&'static mut DrawList>,
		)>,
		Query<&'static TextShadowMark>,

        OrInitRes<ProgramMetaRes<crate::shader::text::ProgramMeta>>,
        OrInitRes<PosUvColorVertexLayout>,
        OrInitRes<ShaderInfoCache>,
        OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
		ResMut<PiRenderGraph>,
		Query<&'static GraphId>,
		OrInitRes<RenderContextMarkType<TextShadow>>,
		EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
		
    )>,

    query_draw_list: &mut SystemState<Query<&'static mut DrawList>>,
) {
	let (render_type, mut changed, mut del, mut query, mark_query, program_meta, vert_layout, shader_catch, group_alloter, mut rg, graph_id_query, mark_type,mut  event_writer) = state.get_mut(world);
    let group_alloter = group_alloter.clone();
	let render_type = ***render_type;

    // 收集需要删除DrawObject的实体
    for del in del.iter() {
        if let Ok((text_shadow, mut draw_list, mut render_mark_value)) = query.p0().get_mut(del) {
            if text_shadow.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            if let Some(draw_obj) = draw_list.remove(render_type) {
				will_delete.push(draw_obj.id);

				// 删除渲染图节点
				if let Ok(r) = graph_id_query.get(draw_obj.id) {
					let _ = rg.remove_node(**r);
				}
            }

			if unsafe { render_mark_value.replace_unchecked(***mark_type, false) } {
                // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
                event_writer.send(ComponentEvent::new(del));
            }
        }
    }

    // 收集需要创建DrawObject的实体
    for changed in changed.iter() {
        if let Ok((shadow, mut draw_list, mut render_mark_value)) = query.p1().get_mut(changed.id) {
			render_mark_true(changed.id, ***mark_type, &mut event_writer, &mut render_mark_value);

			let mut need_count = shadow.len();
			let mut i = 0;
			while i < draw_list.len() {
				if draw_list[i].ty == render_type {
					let mark = mark_query.get(draw_list[i].id).unwrap();
					if **mark >= shadow.len() {
						// 多余的， 删除
						draw_list.swap_remove(i);
						break;
					} else {
						need_count -= 1;
					}
				}
				i += 1;
			}
			if need_count > 0 {
				// 计算需要为该节点创建的阴影DrawObj的数量
				will_creates.push((changed.id, shadow.len() - need_count, shadow.len()));
			}
           
        }
    }

    let program_meta = program_meta.clone();
    let state = shader_catch.premultiply.clone();
    let vert_layout = vert_layout.clone();

    // 删除DrawObject实体
    for del in will_delete.drain(..) {
        world.despawn(del);
    }

    // 创建DrawObject
    for (create, start, count) in will_creates.iter_mut() {
		// let old_count = 0;
		// for i in 
		let mut start = *start;
		while start < *count {
			let mut draw_state = DrawState::default();
			let ui_material_group = group_alloter.alloc();
			draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

			let mut clear_group = group_alloter.alloc();
			let world_matrix = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
			let _ = clear_group.set_uniform(&WorldUniform(world_matrix.as_slice()));

			let mut post = PostProcess::default();
			post.post.src_preimultiplied = false;

			will_create_draws.push(
				world
					.spawn((DrawBundle {
						node_id: NodeId(EntityKey(*create)),
						draw_state,
						box_type: BoxType::ContentNone,
						pipeline_meta: PipelineMeta {
							program: program_meta.clone(),
							state: state.clone(),
							vert_layout: vert_layout.clone(),
							defines: Default::default(),
						},
						draw_info: DrawInfo::new(TEXT_SHADOW_ORDER, false), //TODO
						other: TextShadowMark(start),
					}, post, GraphId::default(), TextShadowColorBindGroup(clear_group.into())))
					.id(),
			);
			start += 1;
		}
    }

    let mut query_draw_list = query_draw_list.get_mut(world);
	let mut index = 0;
    // 创建Node到DrawObject的映射
    for (create, start, count) in will_creates.drain(..) {
        if let Ok(mut draw_list) = query_draw_list.get_mut(create) {
			for _ in 0..count - start {
				draw_list.push(render_type, will_create_draws[index]);
				index += 1;
			}
        }
    }
	will_create_draws.clear();
}

/// 设置文字阴影的顶点、索引、uv，和颜色的Uniform
pub fn calc_text_shadow(
	render_type: Res<TextRenderObjType>,
    query: Query<
		(
			&NodeState,
			Ref<NodeState>,
			Ref<TextShadow>,
			Ref<WorldMatrix>,
			&DrawList,
		),
		// TextContent改变，NodeState必然改变; 存在NodeState， 也必然存在TextContent
		(With<TextShadow>, Or<(Changed<NodeState>, Changed<TextShadow>)>),
	>,

    mut query_draw: Query<(&mut DrawState, &mut TextShadowColorBindGroup, &NodeId, &TextShadowMark, &mut PostProcess, &mut PipelineMeta), Without<TextMark>>,
	query_text_draw: Query<&DrawState, (With<TextMark>, Without<TextShadowMark>)>,

    text_texture_group: OrInitRes<TextTextureGroup>,

	font_sheet: Res<ShareFontSheet>,
	empty_vert_buffer: Res<EmptyVertexBuffer>,
	mut post_resource: ResMut<PostprocessResource>,
	device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let font_sheet = font_sheet.borrow();

    // 更新纹理尺寸
    let size = font_sheet.texture_size();
	let texture_group = match &***text_texture_group {
		Some(r) => r,
		None => panic!(), // 必须要创建TextTextureGroup
	};

    // let mut init_spawn_drawobj = Vec::new();
	for (mut draw_state, mut clear_color_group, node_id, shadow_mark, mut post_process, mut pipeline_meta) in query_draw.iter_mut() {
		if let Ok((node_state, node_state_change, text_shadow, world_matrix, draw_list)) = query.get(***node_id) {
			if node_state.0.scale < 0.000001 {
				continue;
			}

			// 如果不存在，插入默认值（只有刚创建时不存在）
            if draw_state.vertices.get(2).is_none() {
                draw_state.insert_vertices(RenderVertices {
                    slot: 2,
                    buffer: EVerticesBufferUsage::GUI((*empty_vert_buffer).clone()),
                    buffer_range: None,
                    size_per_value: 8,
                });
                draw_state
                    .bindgroups
                    .insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group.clone()));
                draw_state
                    .bindgroups
                    .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[size.width as f32, size.height as f32]));
				pipeline_meta.defines.insert(SHADOW_DEFINE.clone());
            }

			// 重新设置顶点、索引和uv(与文字渲染一样，直接clone过来)
			if node_state_change.is_changed() || text_shadow.is_changed() {
				if let Some(text_draw_id) = draw_list.get_one(**render_type) {
					if let Ok(text_draw) = query_text_draw.get(text_draw_id.id) {
						// 可能文字长度为0，无法渲染
						if let None = &text_draw.indices {
							draw_state.indices = None;
							draw_state.vertices.clear();
							continue;
						}
						draw_state.vertices.insert(PositionVert::location(), text_draw.vertices.get(PositionVert::location()).unwrap().clone());
						draw_state.vertices.insert(UvVert::location(), text_draw.vertices.get(UvVert::location()).unwrap().clone());
						draw_state.indices = text_draw.indices.clone();
					}
				}
			}
			
			// 设置颜色uniform, h、v uniform
			if text_shadow.is_changed() {
				let color: &pi_style::style::CgColor = &text_shadow[shadow_mark.0].color;
				draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));draw_state.bindgroups.set_uniform(&StrokeColorOrURectUniform(&[text_shadow[shadow_mark.0].h, text_shadow[shadow_mark.0].v, 0.0, 0.0]));

				clear_color_group.0.set_uniform(&ColorUniform(&[color.x, color.y, color.z, 0.0]));
			}

			// 文字阴影修改，或世界矩阵修改，则重新设置模糊半径
			if text_shadow.is_changed() || world_matrix.is_changed() {
				post_process.blur_gauss = Some(BlurGauss{radius: text_shadow[shadow_mark.0].blur * node_state.0.scale});
				post_process.calc(16, &device, &queue, &mut post_resource.vballocator);
			}
		}
	}
    
}

// 建立图依赖
pub fn calc_graph_depend(
	render_type: Res<TextShadowRenderObjType>,
    pass_query: Query<(&ParentPassId, &GraphId), Without<TextShadowMark>>,
	shadow_query: Query<(Entity, &DrawList), (With<TextShadow>, Or<(Changed<ParentPassId>, Changed<TextShadow>)>)>,
	mut shadow_draw_query: Query<&mut GraphId, With<TextShadowMark>>,
    mut rg: ResMut<PiRenderGraph>,
) {
	for (node, draw_list) in shadow_query.iter() {
		let parent_graph_id = get_to(node, &pass_query);
		if parent_graph_id.is_null() {
			continue;
		}
		for draw_id in draw_list.iter() {
			if draw_id.ty == **render_type {
				if let Ok(mut g) = shadow_draw_query.get_mut(draw_id.id) {
					if g.is_null() {
						**g = match rg.add_node(format!("TextShadow_{:?}", draw_id.id), TextShadowNode(draw_id.id)) {
							Ok(r) => r,
							Err(e) => {
								log::error!("node: {:?}, {:?}", format!("TextShadow_{:?}", draw_id.id), e);
								return;
							}
						};
					}
					if let Err(e) = rg.add_depend(**g, parent_graph_id) {
						log::error!("{:?}", e);
					}
				}
			}
		}
	}
}


#[derive(SystemParam)]
pub struct QueryParam<'w, 's> {
	query_post_info: Query<'w, 's, &'static Camera>,
	draw_query: Query<'w, 's, (&'static DrawState, &'static NodeId, &'static PostProcess, &'static TextShadowColorBindGroup)>,
	atlas_allocator: Res<'w, PiSafeAtlasAllocator>,
	// 清屏相关参数
    fbo_clear_color: Res<'w, DynFboClearColorBindGroup>,
    clear_draw: Res<'w, ClearDrawObj>,
	query_root: Query<'w, 's, &'static DynTargetType>,
	query_layer: Query<'w, 's, &'static Layer>,
	post_resource: Res<'w, PostprocessResource>,
    pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,
}

/// 渲染图节点， 用于将文字做模糊处理（draw_front）
pub struct TextShadowNode(Entity);

impl Node for TextShadowNode {
    type Input = ();
    type Output = SimpleInOut;

    type Param = QueryParam<'static, 'static>;

    fn run<'a>(
        &'a mut self,
        world: &'a World,
        query_param_state: &'a mut SystemState<Self::Param>,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        _input: &'a Self::Input,
        _usage: &'a ParamUsage,
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
		let RenderContext { device, queue } = context;
        let draw_id = self.0;
		Box::pin(async move {
			let param = query_param_state.get(world);
			if let Ok((draw_state, node_id, post_process, clear_color)) = param.draw_query.get(draw_id) {
				if let Ok(camera) = param.query_post_info.get(***node_id) {
					if camera.is_active {
						let layer = match param.query_layer.get(***node_id) {
							Ok(r) if r.layer() > 0 => r.clone(),
							_ => return Ok(SimpleInOut { target: None }),
						};

						let t_type = param.query_root.get(layer.root()).unwrap();
						let e: [ShareTargetView; 0] = [];
						let rt = param.atlas_allocator.allocate(
							(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
							(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
							t_type.has_depth,
							e.iter(),
						);
						{
							// 创建一个渲染Pass
							let (mut rp, view_port, clear_port) = create_rp_for_fbo(
								&rt,
								commands.borrow_mut(),
								&camera.view_port,
								None
							);

							// 设置视口
							let clear_obj = &param.fbo_clear_color.0;
							rp.set_viewport(clear_port.0, clear_port.1, clear_port.2, clear_port.3, 0.0, 1.0);
							clear_color.0.set(&mut rp, UiMaterialBind::set());
							// clear_obj.0.set(&mut rp, UiMaterialBind::set());
							clear_obj.1.set(&mut rp, DepthBind::set());
							param.clear_draw.0.draw(&mut rp);

							// 设置视口
							rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

							// 渲染
							if let Some(r) = &camera.bind_group {
								r.set(&mut rp, CameraBind::set());
							}

							draw_state.draw(&mut rp);
						}

						// 后处理
						let rect = rt.rect().clone();
						let (w, h) = ((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32);
						// 渲染后处理
						if let Ok(r) = post_process.draw_front(
							&device,
							&queue,
							commands.borrow_mut(),
							PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default()),
							(w, h),
							&param.atlas_allocator,
							&param.post_resource.resources,
							&param.pipline_assets,
							t_type.no_depth,
							wgpu::TextureFormat::pi_render_default(),
						) {
							if let ETextureViewUsage::SRT(r) = r.view {
								return Ok(SimpleInOut { target: Some(r) });
							}
						};
					}
				}
			}
			return Ok(SimpleInOut { target: None });
		})
	}
}


