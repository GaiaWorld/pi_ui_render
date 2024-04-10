use std::borrow::BorrowMut;

use bevy_ecs::prelude::{DetectChanges, Ref};
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::system::{Query, Res, SystemParam, SystemState, SystemChangeTick};
use bevy_app::{Plugin, Update, App};
use bevy_ecs::prelude::{Commands, Component, IntoSystemConfigs};
use bevy_ecs::prelude::{Entity, EventReader, EventWriter, ParamSet, RemovedComponents, ResMut, Without, World};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::Layer;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::render_cross::GraphId;
use pi_bevy_render_plugin::node::NodeId as GraphNodeId;
use pi_bevy_render_plugin::node::{Node, ParamUsage};
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderGraph, PiRenderQueue, PiSafeAtlasAllocator, RenderContext, SimpleInOut};
use pi_futures::BoxFuture;
use pi_null::Null;
use pi_postprocess::image_effect::PostProcessDraw;
use pi_postprocess::prelude::{BlurGauss, PostprocessTexture};
use pi_render::components::view::target_alloc::ShareTargetView;
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::renderer::texture::ETextureViewUsage;
use pi_render::renderer::vertices::{EVerticesBufferUsage, RenderVertices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::pipeline::RenderPipeline;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_render::rhi::texture::PiRenderDefault;
use pi_share::ShareRefCell;
use wgpu::CommandEncoder;

use crate::components::calc::{DrawInfo, DrawList, EntityKey, NodeId, NodeState, RenderContextMark, WorldMatrix};
use crate::components::draw_obj::DrawState;
use crate::components::draw_obj::{BoxType, DynTargetType, PipelineMeta, TextMark, TextShadowMark};
use crate::components::pass_2d::{Camera, ParentPassId, PostProcess};
use crate::components::user::{Matrix4, TextShadow, Animation, TextStyle};
use crate::components::DrawBundle;
use crate::resource::draw_obj::{
    ClearDrawObj, DepthCache, EmptyVertexBuffer, PosUvColorVertexLayout, ProgramMetaRes, ShaderInfoCache,
    ShareGroupAlloter, TextTextureGroup, UiMaterialGroup, DynMark,
};
use crate::resource::{RenderContextMarkType, ShareFontSheet, TextRenderObjType, TextShadowRenderObjType};
use crate::shader1::meterial::CameraBind;
use crate::shader::depth::DepthBind;
use crate::shader::text::SHADOW_DEFINE;
use crate::shader::text::{PositionVert, SampBind, UvVert};
use crate::shader::ui_meterial::{ColorUniform, StrokeColorOrURectUniform, TextureSizeOrBottomLeftBorderUniform, UiMaterialBind, WorldUniform};
use crate::system::pass::pass_graph_node::create_rp_for_fbo;
use crate::system::pass::pass_life::{cal_context, render_mark_true};
use crate::system::pass::update_graph::{get_to, update_graph};
use crate::system::system_set::UiSystemSet;
use bevy_window::AddFrameEvent;

use super::text::calc_text;
use super::IsRun;


/// 文字阴影插件
pub struct UiTextShadowPlugin;

impl Plugin for UiTextShadowPlugin {
    fn build(&self, app: &mut App) {
        app.add_frame_event::<ComponentEvent<Changed<TextShadow>>>()
            .add_systems(Update, 
                text_shadow_life
                    .in_set(UiSystemSet::LifeDrawObject)
                    .in_set(UiSystemSet::PassMark)
                    .before(cal_context),
            )
            .add_systems(Update, 
                calc_text_shadow
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .in_set(UiSystemSet::PassSetting)
                    .after(calc_text)
            )
            .add_systems(Update, calc_graph_depend.in_set(UiSystemSet::PassCalc).after(update_graph));
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
    // mut will_creates: Local<Vec<(Entity, usize, usize)>>, // (节点id， 开始索引，阴影数量)
    // mut will_create_draws: Local<Vec<Entity>>, // drawObj的id
    // mut will_delete: Local<Vec<Entity>>,
    state: &mut SystemState<(
        OrInitRes<TextShadowRenderObjType>,
        EventReader<ComponentEvent<Changed<TextShadow>>>,
        RemovedComponents<TextShadow>,
        ParamSet<(
            Query<(Option<&'static TextShadow>, &'static mut DrawList, &'static mut RenderContextMark, Option<&'static Animation>)>,
            Query<(&'static TextShadow, &'static mut DrawList, &'static mut RenderContextMark, Option<&'static Animation>), Changed<TextShadow>>,
        )>,
        Query<&'static TextShadowMark>,
        (
			OrInitRes<ProgramMetaRes<crate::shader::text::ProgramMeta>>,
			OrInitRes<ProgramMetaRes<crate::shader::text_sdf::ProgramMeta>>,
			Res<ShareFontSheet>,
			RemovedComponents<TextShadowMark>,
		),
        OrInitRes<PosUvColorVertexLayout>,
        OrInitRes<ShaderInfoCache>,
        (OrInitRes<ShareGroupAlloter<UiMaterialGroup>>, OrInitRes<ShareGroupAlloter<UiMaterialGroup, DynMark>>),
		
        ResMut<PiRenderGraph>,
        Query<&'static GraphId>,
        OrInitRes<RenderContextMarkType<TextShadow>>,
        EventWriter<ComponentEvent<Changed<RenderContextMark>>>,
        Commands,
		OrInitRes<IsRun>,
		SystemChangeTick,
    )>,
) {
    let (
        render_type,
        mut changed,
        mut del,
        mut query,
        mark_query,
        (
			program_meta,
			text_sdf_program_meta,
			font_sheet,
			mut text_shadow_mark_del,
		),
        vert_layout,
        shader_catch,
        (group_alloter, dyn_group_alloter),
        mut rg,
        graph_id_query,
        mark_type,
        mut event_writer,
        mut commands,
		r,
		system_change_tick
    ) = state.get_mut(world);
	if r.0 {
		return;
	}
    let group_alloter = group_alloter.clone();
    let render_type = ***render_type;

    // TextShadow组件被移除时，删除对应的DrawObj
    for del in del.iter() {
        if let Ok((text_shadow, mut draw_list, mut render_mark_value, animation)) = query.p0().get_mut(del) {
            if text_shadow.is_some() {
                continue;
            }
            // 删除对应的DrawObject
            draw_list.remove(render_type, |draw_obj| {
                if let Some(mut r) = commands.get_entity(draw_obj.id) {
					log::trace!("despawn shadow2====={:?}", draw_obj.id);
                    r.despawn();
                }
            });

            if unsafe { render_mark_value.replace_unchecked(***mark_type, false) } {
                // 通知（RenderContextMark组件在每个节点上都存在， 但实际上，是渲染上下文的节点不多，基于通知的改变更高效）
                event_writer.send(ComponentEvent::new(del));
            }
        }
    }

	for id in text_shadow_mark_del.iter() {
		let _ = rg.remove_node(format!("TextShadow_{:?}", id));
	}

    let program_meta = program_meta.clone();
	let sdf_program_meta = text_sdf_program_meta.clone();
    let p_state = shader_catch.premultiply.clone();
    let vert_layout = vert_layout.clone();
	let use_sdf = match font_sheet.borrow().font_mgr().font_type {
		pi_hal::font::font::FontType::Bitmap => false,
		pi_hal::font::font::FontType::Sdf1 => true,
		pi_hal::font::font::FontType::Sdf2 => false,
	};

	// let mut spawn_list = Vec::new();
    // 收集需要创建DrawObject的实体
    for changed in changed.iter() {
        if let Ok((shadow, mut draw_list, mut render_mark_value, animation)) = query.p1().get_mut(changed.id) {
			// changed中的id肯呢个重复， 这里判断被system当帧运行是否已经修改过draw_list， 如果已经修改过，则忽略
			if draw_list.last_changed() == system_change_tick.this_run() {
				continue;
			}
            render_mark_true(changed.id, ***mark_type, &mut event_writer, &mut render_mark_value);
            let mut need_count = shadow.len();
            let mut i = 0;
            while i < draw_list.len() {
                if draw_list[i].ty == render_type {
                    let mark = mark_query.get(draw_list[i].id).unwrap();
                    if **mark >= shadow.len() {
                        // 多余的， 删除
                        let draw_obj = draw_list.swap_remove(i);
						if let Some(mut r) = commands.get_entity(draw_obj.id) {
							log::trace!("despawn shadow1====={:?}", draw_obj.id);
							r.despawn();
						}
                        continue;
                    } else {
                        need_count -= 1;
                    }
                }
                i += 1;
            }
            if need_count > 0 {
                // will_creates.push((changed.id, shadow.len() - need_count, shadow.len()));
                // 计算需要为该节点创建的阴影DrawObj的数量
                let mut start = shadow.len() - need_count;
                let count = shadow.len();
                while start < count {
                    let mut draw_state = DrawState::default();
                    let ui_material_group = group_alloter.alloc();
                    draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                    let mut clear_group = group_alloter.alloc();
                    let world_matrix = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
                    let _ = clear_group.set_uniform(&WorldUniform(world_matrix.as_slice()));

                    let mut post = PostProcess::default();
                    post.post.src_preimultiplied = false;

                    let id = commands
                        .spawn((
                            DrawBundle {
                                node_id: NodeId(EntityKey(changed.id)),
                                draw_state,
                                box_type: BoxType::ContentNone,
                                pipeline_meta: PipelineMeta {
                                    type_mark: render_type,
                                    program: if use_sdf {sdf_program_meta.clone()} else {program_meta.clone()},
                                    state: p_state.clone(),
                                    vert_layout: vert_layout.clone(),
                                    defines: Default::default(),
                                },
                                draw_info: DrawInfo::new(TEXT_SHADOW_ORDER, false), //TODO
                                other: TextShadowMark(start),
                            },
                            post,
                            GraphId::default(),
                            TextShadowColorBindGroup(clear_group.into()),
                        ))
                        .id();
					// spawn_list.push(id);
                    draw_list.push(render_type, id);
					// log::warn!("create drawobj=================draw={:?}, node={:?}", id, changed.id);
                    start += 1;
                }
            }
        }
    }

	// if spawn_list.len() > 0 {
	// 	log::warn!("spawn shadow=================draw={:?}", &spawn_list);
	// }

    state.apply(world);
	// log::warn!("text_shadow======{:?}", pi_time::Instant::now() - time);
}

/// 设置文字阴影的顶点、索引、uv，和颜色的Uniform
pub fn calc_text_shadow(
    render_type: Res<TextRenderObjType>,
    query: Query<
        (&NodeState, Ref<NodeState>, Ref<TextShadow>, Ref<WorldMatrix>, &DrawList, &TextStyle),
        // TextContent改变，NodeState必然改变; 存在NodeState， 也必然存在TextContent
        (With<TextShadow>, Or<(Changed<NodeState>, Changed<TextShadow>)>),
    >,

    mut query_draw: Query<
        (
            &mut DrawState,
            &mut BoxType,
            &mut TextShadowColorBindGroup,
            &NodeId,
            &TextShadowMark,
            &mut PostProcess,
            &mut PipelineMeta,
        ),
        Without<TextMark>,
    >,
    query_text_draw: Query<&DrawState, (With<TextMark>, Without<TextShadowMark>)>,

    text_texture_group: OrInitRes<TextTextureGroup>,

    font_sheet: ResMut<ShareFontSheet>,
    empty_vert_buffer: Res<EmptyVertexBuffer>,
	r: OrInitRes<IsRun>,
) {
	if r.0 {
		return;
	}
    let mut font_sheet = (***font_sheet).borrow_mut();

    // 更新纹理尺寸
    let size = font_sheet.texture_size();
    let texture_group = match &text_texture_group.0 {
        Some(r) => r,
        None => panic!(), // 必须要创建TextTextureGroup
    };

    // let mut init_spawn_drawobj = Vec::new();
    for (mut draw_state, mut box_type, mut clear_color_group, node_id, shadow_mark, mut post_process, mut pipeline_meta) in query_draw.iter_mut() {
        if let Ok((node_state, node_state_change, text_shadow, world_matrix, draw_list, text_style)) = query.get(***node_id) {
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
                *box_type = BoxType::ContentRect;
            }

            // 重新设置顶点、索引和uv(与文字渲染一样，直接clone过来)
            if node_state_change.is_changed() || text_shadow.is_changed() {
                if let Some(text_draw_id) = draw_list.get_one(**render_type) {
                    if let Ok(text_draw) = query_text_draw.get(text_draw_id.id) {
                        // 可能文字长度为0，无法渲染
                        if let None = &text_draw.indices {
                            draw_state.indices = None;
                            draw_state.vertices.clear();
							draw_state.vertex = 0..0;
                            continue;
                        }
                        draw_state.vertices.insert(
                            PositionVert::location(),
                            text_draw.vertices.get(PositionVert::location()).unwrap().clone(),
                        );
                        draw_state
                            .vertices
                            .insert(UvVert::location(), text_draw.vertices.get(UvVert::location()).unwrap().clone());
                        draw_state.indices = text_draw.indices.clone();
                    }
                }
            }

            // 设置颜色uniform, h、v uniform
            if text_shadow.is_changed() {
                let color: &pi_style::style::CgColor = &text_shadow[shadow_mark.0].color;
                draw_state.bindgroups.set_uniform(&ColorUniform(&[color.x, color.y, color.z, color.w]));
                draw_state.bindgroups.set_uniform(&StrokeColorOrURectUniform(&[
                    text_shadow[shadow_mark.0].h,
                    text_shadow[shadow_mark.0].v,
                    0.0,
                    0.0,
                ]));

                clear_color_group.0.set_uniform(&ColorUniform(&[color.x, color.y, color.z, 0.0]));
            }

            // 文字阴影修改，或世界矩阵修改，则重新设置模糊半径
            if text_shadow.is_changed() || world_matrix.is_changed() {
                post_process.blur_gauss = Some(BlurGauss {
                    radius: text_shadow[shadow_mark.0].blur * node_state.0.scale,
                });
            }

			// if font_sheet.font_mgr().use_sdf() && (text_shadow.is_changed() || world_matrix.is_changed()) {
			// 	let scale = node_state.0.scale;
			// 	let font_size = get_size(&text_style.font_size) as f32;
			// 	let font_id = font_sheet.font_id(Font::new(
			// 		text_style.font_family.clone(),
			// 		font_size as usize,
			// 		text_style.font_weight,
			// 		text_style.text_stroke.width, // todo 或许应该设置比例
			// 	));
			// 	let font_height = font_sheet.font_height(font_id, font_size as usize);

			// 	let font_info = font_sheet.font_mgr().font_info(font_id);
			// 	let metrics = font_sheet.font_mgr().brush.sdf_brush.metrics_info(&font_info.font_ids[0]);
			// 	let scale0 = scale * font_height / (metrics.ascender - metrics.descender);
			// 	let sw = (scale * *text_style.text_stroke.width).round();
			// 	let distance_px_range = scale0 * metrics.distance_range;
			// 	let fill_bound = 0.5 - (text_style.font_weight as f32 / 500 as f32 - 1.0) / distance_px_range;
			// 	let stroke = sw/2.0/distance_px_range;
			// 	let stroke_bound = fill_bound - stroke;
			// 	draw_state.bindgroups.set_uniform(&ClipSdfOrSdflineUniform(&[distance_px_range, fill_bound, stroke_bound]));
			// }
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
	r: OrInitRes<IsRun>
) {
	if r.0 {
		return;
	}
    for (node, draw_list) in shadow_query.iter() {
        let parent_graph_id = get_to(node, &pass_query);
        if parent_graph_id.is_null() {
            continue;
        }
		
        for draw_id in draw_list.iter() {
            if draw_id.ty == **render_type {
                if let Ok(mut g) = shadow_draw_query.get_mut(draw_id.id) {
                    if g.is_null() {
                        **g = match rg.add_node(format!("TextShadow_{:?}", draw_id.id), TextShadowNode{id: draw_id.id, post_process: None, rt: None }, GraphNodeId::default()) {
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
pub struct BuildParam<'w, 's> {
	atlas_allocator: Res<'w, PiSafeAtlasAllocator>,
	device: Res<'w, PiRenderDevice>,
	queue: Res<'w, PiRenderQueue>,
	post_process_query:  Query<'w, 's,( &'static mut PostProcess, &'static NodeId,)>,
	camera_query: Query<'w, 's, (&'static Camera, &'static Layer)>,
	query_root: Query<'w, 's, &'static DynTargetType>,
	post_resource: Res<'w, PostprocessResource>,
    pipline_assets: Res<'w, ShareAssetMgr<RenderRes<RenderPipeline>>>,
}

#[derive(SystemParam)]
pub struct QueryParam<'w, 's> {
    camera_query: Query<'w, 's, &'static Camera>,
    draw_query: Query<
        'w,
        's,
        (
			&'static DrawState,
			&'static NodeId,
			&'static TextShadowColorBindGroup,
			&'static PostProcess,
		)
    >,
    clear_draw: Res<'w, ClearDrawObj>,
    depth_cache: OrInitRes<'w, DepthCache>,
}

/// 渲染图节点， 用于将文字做模糊处理（draw_front）
pub struct TextShadowNode {
	id: Entity,
	post_process: Option<Vec<PostProcessDraw>>,
	rt: Option<ShareTargetView>,
}

impl Node for TextShadowNode {
    type Input = ();
    type Output = SimpleInOut;

    type RunParam = QueryParam<'static, 'static>;
	type BuildParam = BuildParam<'static, 'static>;
	fn build<'a>(
		&'a mut self,
		world: &'a mut bevy_ecs::world::World,
		query_param_state: &'a mut bevy_ecs::system::SystemState<Self::BuildParam>,
		_context: pi_bevy_render_plugin::RenderContext,
		_input: &'a Self::Input,
		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
		_id: GraphNodeId,
		_from: &'a [GraphNodeId],
		_to: &'a [GraphNodeId],
	) -> Result<Self::Output, String> {
		let mut param = query_param_state.get_mut(world);
		let draw_id = self.id;
		if let Ok((mut post_process, node_id)) = param.post_process_query.get_mut(draw_id) {
			if let Ok((camera, layer)) = param.camera_query.get(***node_id) {
				if camera.is_active {
					if layer.layer() <= 0 {
						return Ok(SimpleInOut::default());
					};

					let t_type = param.query_root.get(layer.root()).unwrap();

					let e: [ShareTargetView; 0] = [];
					let rt = param.atlas_allocator.allocate_not_hold(
						(camera.view_port.maxs.x - camera.view_port.mins.x).ceil() as u32,
						(camera.view_port.maxs.y - camera.view_port.mins.y).ceil() as u32,
						t_type.has_depth,
						e.iter(),
					);
					

					let rect = rt.rect().clone();
                    let (w, h) = ((rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32);

					match post_process.calc(
						16, 
						&param.device, 
						&param.queue, 
						PostprocessTexture::from_share_target(rt.clone(), wgpu::TextureFormat::pi_render_default()),
						(w, h),
						&param.atlas_allocator,
						&param.post_resource.resources,
						&param.pipline_assets,
						t_type.no_depth,
						wgpu::TextureFormat::pi_render_default(),
					) {
						Ok(r) => {
							if let ETextureViewUsage::SRT(target) = &r.1.view {
								self.rt = Some(rt);
								self.post_process = Some(r.0);
								return Ok(SimpleInOut{
									target: Some(target.clone()),
									valid_rect: None,
								});
							}
						},
						_ => {
							self.rt = Some(rt.clone());
							return Ok(SimpleInOut{
								target: Some(rt),
								valid_rect: None,
							})
						},
					};
				}
			}
		}
		return Ok(SimpleInOut::default());
	}

    fn run<'a>(
        &'a mut self,
        world: &'a World,
        query_param_state: &'a mut SystemState<Self::RunParam>,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        _input: &'a Self::Input,
        _usage: &'a ParamUsage,
        _id: GraphNodeId,
        _from: &'a [GraphNodeId],
        _to: &'a [GraphNodeId],
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<(), String>> {
        let RenderContext { device, queue, .. } = context;
        let (draw_id, post_process_draw, rt) = (self.id, self.post_process.take(), self.rt.take());
        Box::pin(async move {
			let rt = match rt {
				Some(r) => r,
				None => return Ok(()), // 没有分配rt， 不需要渲染
			};

            let param: QueryParam<'_, '_> = query_param_state.get(world);
            if let Ok((draw_state, node_id,  clear_color, post_process)) = param.draw_query.get(draw_id) {
				if let Ok(camera) = param.camera_query.get(***node_id) {
					{
						// 创建一个渲染Pass
						let mut c = (*commands).borrow_mut();
						let (mut rp, view_port, clear_port, _) =
							create_rp_for_fbo(&rt, &mut c, &camera.view_port, &camera.view_port, None);
	
						// 设置视口
						// let clear_obj = &param.fbo_clear_color.0;
						rp.set_viewport(clear_port.0, clear_port.1, clear_port.2, clear_port.3, 0.0, 1.0);
						clear_color.0.set(&mut rp, UiMaterialBind::set());
						// clear_obj.0.set(&mut rp, UiMaterialBind::set());
						param.depth_cache.list[0].set(&mut rp, DepthBind::set());
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

					if let Some(post_process_draw) = post_process_draw {
						// 渲染后处理
						post_process.draw_front(
							&mut commands.borrow_mut(),
							&post_process_draw
						)
					}
				}
			}
			return Ok(());
        })
    }
}
