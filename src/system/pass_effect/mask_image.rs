// use crate::{
//     components::{
//         calc::Quad,
//         draw_obj::{DrawState, DynTargetType, PipelineMeta},
//         pass_2d::PostProcess,
//         user::{MaskImage, MaskImageClip, Point2},
//     },
//     resource::{
//         draw_obj::{CameraGroup, DepthCache, DepthGroup, PosColorVertexLayout, ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup},
//         BackgroundColorRenderObjType,
//     },
//     shader::{
//         camera::{CameraBind, ProjectUniform, ViewUniform},
//         color::{PositionVert, VertColorVert, VERT_COLOR_DEFINE},
//         depth::DepthBind,
//         ui_meterial::{UiMaterialBind, WorldUniform},
//     },
//     system::{
//         draw_obj::{
//             calc_background_color::linear_gradient_split,
//             image_texture_load::{load_image, set_texture, ImageAwait},
//             pipeline::calc_node_pipeline, calc_text::IsRun,
//         },
//         node::world_matrix::cal_matrix,
//         pass::{pass_graph_node::create_rp_for_fbo, pass_life, update_graph},
//         system_set::UiSystemSet,
//         utils::{create_project, set_index_buffer, set_vert_buffer},
//     },
// };
// use bevy_ecs::{
//     system::{SystemParam, SystemState},
//     prelude::{Changed, Commands, DetectChanges, Entity, Or, Query, Ref, RemovedComponents, Res, ResMut,
//         Resource, World, IntoSystemConfigs, apply_deferred
//     },
// };
// use bevy_app::{Plugin, UiSchedule, App, Startup};
// use guillotiere::Rectangle;
// use ordered_float::NotNan;
// use pi_bevy_asset::ShareAssetMgr;
// use pi_bevy_ecs_extend::{
//     prelude::{Layer, OrDefault},
//     system_param::res::{OrInitRes, OrInitResMut},
// };
// use pi_bevy_render_plugin::{
//     component::GraphId,
//     node::{Node, NodeId as GraphNodeId, ParamUsage},
//     PiIndexBufferAlloter, PiRenderDevice, PiRenderGraph, PiRenderQueue, PiSafeAtlasAllocator, PiVertexBufferAlloter, RenderContext,
// };
// use pi_flex_layout::prelude::Size;
// use pi_futures::BoxFuture;
// use pi_hash::XHashSet;
// use pi_null::Null;
// use pi_postprocess::prelude::{ImageMask, PostprocessTexture};
// use pi_render::{
//     components::view::target_alloc::ShareTargetView,
//     renderer::texture::ETextureViewUsage,
//     rhi::{
//         asset::TextureRes,
//         shader::{BindLayout, Input},
//         texture::PiRenderDefault,
//     },
// };
// use pi_share::ShareRefCell;
// use pi_style::style::{Aabb2, LinearGradientColor, MaskImage as MaskImage1};
// use std::borrow::BorrowMut;
// use wgpu::CommandEncoder;

// pub struct UiMaskImagePlugin;

// impl Plugin for UiMaskImagePlugin {
//     fn build(&self, app: &mut App) {
//         app
//             // 初始化渲染渐变色的图节点
//             .add_systems(Startup, init)
//             // 标记MaskImage所在节点为一个Pass
//             .add_systems(UiSchedule, 
//                 pass_life::pass_mark::<MaskImage>
//                     .in_set(UiSystemSet::PassMark)
//                     .before(pass_life::cal_context)
//                     .in_set(FrameDataPrepare),
//             )
//             // 设置mask_image的后处理效果
//             .add_systems(UiSchedule, 
//                 mask_image_post_process
//                     .after(cal_matrix)
//                     .after(update_graph::update_graph)
//                     .in_set(FrameDataPrepare),
//             )
//             .add_systems(UiSchedule, 
//                 apply_deferred
//                     .after(mask_image_post_process)
//                     .before(calc_node_pipeline)
//                     .in_set(FrameDataPrepare),
//             );
//     }
// }

// /// 绘制渐变颜色的 DrawObject
// /// 每帧清空，并重新收集
// #[derive(Debug, Default, Resource)]
// pub struct LinearMaskDrawList(Vec<(Entity, ShareTargetView)>);

// /// 设置遮罩的后处理效果
// /// 如果MaskImage为url, 则加载该纹理，并设置在后处理上
// /// 如果MaskImage为渐变色，则创建fbo，将该fbo作为MaskImage设置在后处理效果上；同时创建渲染节点，用于在fbo上渲染该渐变颜色
// pub fn mask_image_post_process(
//     q: (
//         Query<
//             (Entity, Ref<MaskImage>, OrDefault<MaskImageClip>, &Quad, &Layer, &GraphId),
//             Or<(Changed<MaskImage>, Changed<Layer>, Changed<MaskImageClip>, Changed<Quad>)>,
//         >,
//         Query<(Entity, &MaskImage)>,
//         Query<OrDefault<MaskImageClip>>,
//         Query<&mut PostProcess>,
//         Query<&DynTargetType>,
//     ),

//     mut del: RemovedComponents<MaskImage>,

//     mut mask_draw_list: OrInitResMut<LinearMaskDrawList>,
//     image_await: OrInitRes<ImageAwait<Entity, MaskImage>>,
//     texture_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
//     queue: Res<PiRenderQueue>,
//     device: Res<PiRenderDevice>,
//     program_meta: OrInitRes<ProgramMetaRes<crate::shader::color::ProgramMeta>>,
//     vert_layout: OrInitRes<PosColorVertexLayout>,
//     shader_catch: OrInitRes<ShaderInfoCache>,
//     vertex_buffer_alloter: OrInitRes<PiVertexBufferAlloter>,
//     index_buffer_alloter: OrInitRes<PiIndexBufferAlloter>,
//     atlas_allocator: Res<PiSafeAtlasAllocator>,
//     group_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
//     camera_material_alloter: OrInitRes<ShareGroupAlloter<CameraGroup>>,
//     other: (
//         OrInitRes<BackgroundColorRenderObjType>,
//         OrInitRes<LinearMaskNodeId>,
//         Commands,
//         ResMut<PiRenderGraph>,
//         OrInitResMut<DepthCache>,
//         OrInitRes<ShareGroupAlloter<DepthGroup>>,
// 		OrInitRes<IsRun>,
//     ),
//     // cur_depth: usize, device: &'a RenderDevice, bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>
// 	// r: OrInitRes<IsRun>
// ) {
//     let (color_render_type, mask_node_id, mut commands, mut rg, mut depth_cache, depth_alloter, r) = other;
// 	if r.0 {
// 		return;
// 	}
//     let (mut query, query_src, query_clip, mut query_dst, query_target_ty) = q;
//     // 图片删除，则删除对应的遮罩效果
//     for del in del.iter() {
//         if let Ok(mut r) = query_dst.get_mut(del) {
//             r.image_mask = None;
//         };
//     }

//     // 清理
//     // 渐变色fbo在MaskImage、MaskImageClip、 Quad不变的情况下，永远不会重新绘制，因此总是每帧检查，并删除
//     // 如果这些属性发生改变， 后续或重新创建新的DrawObj（这种情况应该很少发生）
//     for (entity, _) in mask_draw_list.0.drain(..) {
// 		log::warn!("despawn mask====={:?}", entity);
//         // 删除对应的RenderObj（由于绘制渐变色的RenderObj没有放入DrawList中， 常规处理无法销毁该Obj， 因此在此处对其销毁）
//         commands.entity(entity).despawn();
//     }

//     // 处理图片路径修改，尝试加载图片（异步加载，加载完成后，放入image_await中）
//     // 如果MaskImage是一个渐变颜色，则创建绘制该渐变颜色的DrawObj
//     for (entity, mask_image, mask_image_clip, quad, layer, graph_id) in query.iter_mut() {
//         if mask_image.is_added() && !mask_node_id.is_null() {
//             rg.add_depend(****mask_node_id, **graph_id).unwrap();
//         }
//         match &mask_image.0 {
//             MaskImage1::Path(key) => {
//                 load_image(
//                     entity,
//                     key,
//                     &image_await,
//                     &device,
//                     &queue,
//                     None,
//                     &mut query_dst,
//                     &texture_assets_mgr,
//                     |d, s, _| {
// 						let is_null = d.image_mask.is_null();
//                         d.image_mask = Some(ImageMask::new(PostprocessTexture {
//                             use_x: (mask_image_clip.left * s.width as f32).round() as u32,
//                             use_y: (mask_image_clip.top * s.height as f32).round() as u32,
//                             use_w: ((mask_image_clip.right - mask_image_clip.left) * s.width as f32).round() as u32,
//                             use_h: ((mask_image_clip.bottom - mask_image_clip.top) * s.height as f32).round() as u32,
//                             width: s.width,
//                             height: s.height,
//                             format: s.format,
//                             view: ETextureViewUsage::Tex(s),
//                         }));
// 						is_null
//                     },
//                 );
//             }
//             MaskImage1::LinearGradient(color) => {
//                 let mut post_process = query_dst.get_mut(entity).unwrap();
//                 // 创建fbo
//                 let size = calc_size(&quad, color) as u32;

//                 let mut render_target = None;
//                 if let Some(mask) = &post_process.image_mask {
//                     if let ETextureViewUsage::SRT(r) = &mask.image.view {
//                         let rect = r.rect();
//                         if rect.width() < size as i32 && rect.height() < size as i32 {
//                             if mask_image.is_changed() {
//                                 // mask_image改变，绘制渐变色到原有纹理上
//                                 render_target = Some(r.clone());
//                             } else {
//                                 // mask_image未改变， 不需要重新绘制渐变纹理
//                                 continue;
//                             }
//                         }
//                     }
//                 }

//                 // 以下用于创建绘制用渐变颜色描述的MaskImage的RenderObj
//                 let render_target = match render_target {
//                     Some(r) => r,
//                     None => {
//                         let ty = query_target_ty.get(layer.root()).unwrap(); // 必须存在target_ty
//                         let e: [ShareTargetView; 0] = [];
//                         atlas_allocator.allocate(size, size, ty.no_depth, e.iter())
//                     }
//                 };


//                 let rect = render_target.rect();

//                 let mut t = PostprocessTexture::from_share_target(render_target.clone(), wgpu::TextureFormat::pi_render_default());
//                 t.use_x += 1;
//                 t.use_y += 1;
//                 t.use_w -= 2;
//                 t.use_h -= 2;
//                 post_process.image_mask = Some(ImageMask::new(t));
//                 let mut draw_state = DrawState::default();
//                 let ui_material_group = group_alloter.alloc();
//                 draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);
//                 let camera_group = camera_material_alloter.alloc();
//                 draw_state.bindgroups.insert_group(CameraBind::set(), camera_group);
//                 depth_cache.or_create_depth(0, &depth_alloter); // 其深度将为0， 在图节点渲染时会使用
//                                                                          // draw_state.bindgroups.insert_group(DepthBind::set(), depth_cache.list[0].clone());

//                 // 设置顶点
//                 let (positions, colors, indices) = create_linear_gradient_verts(rect, color);
//                 set_vert_buffer(
//                     PositionVert::location(),
//                     8,
//                     bytemuck::cast_slice(&positions),
//                     &vertex_buffer_alloter,
//                     &mut draw_state,
//                 );
//                 set_vert_buffer(
//                     VertColorVert::location(),
//                     16,
//                     bytemuck::cast_slice(&colors),
//                     &vertex_buffer_alloter,
//                     &mut draw_state,
//                 );
//                 set_index_buffer(bytemuck::cast_slice(&indices), &index_buffer_alloter, &mut draw_state);

//                 // 设置uniform
//                 let matrix = vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
//                 let project_matrix = create_project(rect.min.x as f32, rect.max.x as f32, rect.min.y as f32, rect.max.y as f32);
//                 draw_state.bindgroups.set_uniform(&ViewUniform(&matrix));
//                 draw_state.bindgroups.set_uniform(&WorldUniform(&matrix));
//                 draw_state.bindgroups.set_uniform(&ProjectUniform(project_matrix.as_slice()));

//                 let mut defines = XHashSet::default();
//                 defines.insert(VERT_COLOR_DEFINE.clone());
//                 mask_draw_list.0.push((
//                     commands
//                         .spawn((
//                             draw_state,
//                             PipelineMeta {
//                                 type_mark: ***color_render_type,
//                                 program: program_meta.clone(),
//                                 state: shader_catch.common_no_depth.clone(),
//                                 vert_layout: vert_layout.clone(),
//                                 defines,
//                             },
//                         ))
//                         .id(),
//                     render_target,
//                 ));
//             }
//         }
//     }

//     set_texture(&image_await, None, &query_src, &mut query_dst, |d, s, entity| {
//         let mask_image_clip = query_clip.get(entity).unwrap();
// 		let is_null = d.image_mask.is_null();
//         d.image_mask = Some(ImageMask::new(PostprocessTexture {
//             use_x: (mask_image_clip.left * s.width as f32).round() as u32,
//             use_y: (mask_image_clip.top * s.height as f32).round() as u32,
//             use_w: ((mask_image_clip.right - mask_image_clip.left) * s.width as f32).round() as u32,
//             use_h: ((mask_image_clip.bottom - mask_image_clip.top) * s.height as f32).round() as u32,
//             width: s.width,
//             height: s.height,
//             format: s.format,
//             view: ETextureViewUsage::Tex(s),
//         }));
		
// 		is_null
//     });
// }

// #[derive(Debug, Deref, Resource, Default)]
// pub struct LinearMaskNodeId(GraphId);

// /// system， 用于添加LinearMaskNode节点到渲染图中，该节点将MaskImage的渐变颜色渲染成纹理
// pub fn init(
// 	mut rg: ResMut<PiRenderGraph>, 
// 	mut id: OrInitResMut<LinearMaskNodeId>,
	
// 	r: OrInitRes<IsRun>
// ) {
// 	if r.0 {
// 		return;
// 	}
//     match rg.add_node("MaskImageLinear".to_string(), LinearMaskNode, GraphNodeId::default()) {
//         Ok(r) => {
//             ****id = r;
//         }
//         Err(e) => log::error!("node: {:?}, {:?}", "MaskImageLinear".to_string(), e),
//     };
// }

// #[derive(SystemParam)]
// pub struct QueryParam<'w, 's> {
//     mask_draw_list: OrInitRes<'w, LinearMaskDrawList>,
//     query: Query<'w, 's, &'static DrawState>,
//     depth_cache: OrInitRes<'w, DepthCache>,
//     // // // 清屏相关参数
//     // fbo_clear_color: Res<'w, DynFboClearColorBindGroup>,
//     // clear_draw: Res<'w, ClearDrawObj>,
// }

// // 用于绘制MaskImage
// pub struct LinearMaskNode;

// impl Node for LinearMaskNode {
//     type Input = ();
//     type Output = ();

// 	type BuildParam = QueryParam<'static, 'static>;
//     type RunParam = QueryParam<'static, 'static>;

// 	fn build<'a>(
// 		&'a mut self,
// 		_world: &'a mut bevy_ecs::world::World,
// 		_param: &'a mut bevy_ecs::system::SystemState<Self::BuildParam>,
// 		_context: pi_bevy_render_plugin::RenderContext,
// 		_input: &'a Self::Input,
// 		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
// 		_id: GraphNodeId,
// 		_from: &'a [GraphNodeId],
// 		_to: &'a [GraphNodeId],
// 	) -> Result<Self::Output, String> {
// 		Ok(())
// 	}

//     fn run<'a>(
//         &'a mut self,
//         world: &'a World,
//         query_param_state: &'a mut SystemState<Self::RunParam>,
//         _context: RenderContext,
//         mut commands: ShareRefCell<CommandEncoder>,
//         _input: &'a Self::Input,
//         _usage: &'a ParamUsage,
//         _id: GraphNodeId,
//         _from: &'a [GraphNodeId],
//         _to: &'a [GraphNodeId],
//         // context: RenderContext,
//         // mut commands: ShareRefCell<CommandEncoder>,
//         // inputs: &'a [Self::Output],
//     ) -> BoxFuture<'a, Result<Self::Output, String>> {
//         Box::pin(async move {
//             let param = query_param_state.get(world);
//             for (entity, rt) in param.mask_draw_list.0.iter() {
//                 if let Ok(draw_state) = param.query.get(*entity) {
//                     let view_port = rt.rect();
//                     // 创建一个渲染Pass
//                     let view_port = Aabb2::new(
//                         Point2::new(0.0, 0.0),
//                         Point2::new((view_port.max.x - view_port.min.x) as f32, (view_port.max.y - view_port.min.y) as f32),
//                     );
//                     let (mut rp, view_port, _clear_port, _) = create_rp_for_fbo(&rt, commands.borrow_mut(), &view_port, &view_port, None);

//                     // // 清屏
//                     // let clear_color = &param.fbo_clear_color.0;
//                     // rp.set_viewport(clear_port.0, clear_port.1, clear_port.2, clear_port.3, 0.0, 1.0);
//                     // clear_color.0.set(&mut rp, UiMaterialBind::set());
//                     // clear_color.1.set(&mut rp, DepthBind::set());
//                     // param.clear_draw.0.draw(&mut rp);

//                     // 设置视口
//                     rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);

//                     param.depth_cache.list[0].set(&mut rp, DepthBind::set());
//                     draw_state.draw(&mut rp);
//                 }
//             }
//             Ok(())
//         })
//     }
// }

// fn calc_size(quad: &Aabb2, linear: &LinearGradientColor) -> u32 {
//     let width = quad.maxs.x - quad.mins.x;
//     let height = quad.maxs.y - quad.mins.y;

//     let l = (width * width + height * height).sqrt();
//     let mut min: f32 = 1.0;
//     let mut pre_pos: f32 = 0.0;
//     for item in linear.list.iter() {
//         let diff = item.position - pre_pos;
//         if diff != 0.0 {
//             min = min.min(diff);
//             pre_pos = item.position;
//         }
//     }

//     if min == 1.0 {
//         return 10;
//     }

//     // 保证渐变百分比中，渐变端点之间的距离至少两个像素
//     let at_least = (2.0_f32.min((min * l).ceil() + 1.0) / min).min(width.max(height) / 4.0);
//     // 渐变颜色渲染尺寸为20的整数倍，使得不同大小的渐变色，可以共用同一张纹理
//     // 加2，使得分配的纹理四周可以扩充一个像素，避免采样问题导致边界模糊 TODO
//     return ((at_least / 10.0).ceil() * 10.0) as u32;
// }


// fn create_linear_gradient_verts(rect: &Rectangle, color: &LinearGradientColor) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
//     let size = Size {
//         width: NotNan::new((rect.max.x - rect.min.x) as f32).unwrap(),
//         height: NotNan::new((rect.max.y - rect.min.y) as f32).unwrap(),
//     };
//     let (positions, indices) = (
//         vec![
//             rect.min.x as f32,
//             rect.min.y as f32, // left_top
//             rect.min.x as f32,
//             rect.max.y as f32, // left_bootom
//             rect.max.x as f32,
//             rect.max.y as f32, // right_bootom
//             rect.max.x as f32,
//             rect.min.x as f32, // right_top
//         ],
//         vec![0, 1, 2, 3],
//     );
//     linear_gradient_split(color, positions, indices, &size)
// }
