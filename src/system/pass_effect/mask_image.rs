use crate::{
    components::{
        calc::{MaskTexture, Quad, WorldMatrix},
        pass_2d::{InstanceDrawState, PostProcess, PostProcessInfo},
        user::{MaskImage, MaskImageClip, Opacity, Point2},
    }, 
    resource::{
        draw_obj::{create_render_pipeline, InstanceContext, LastGraphNode, RenderState}, GlobalDirtyMark, OtherDirtyType, RenderContextMarkType
    }, 
    shader::camera::{ProjectUniform, ViewUniform}, 
    shader1::batch_meterial::{CameraBind, RenderFlagType, TyMeterial}, 
    system::base::pass::{pass_graph_node::create_rp_for_fbo, pass_life, update_graph::init_root_graph},
    system::draw_obj::calc_background_color::set_linear_gradient_instance_data,
};
use crate::system::base::draw_obj::image_texture_load::{load_image, set_texture, ImageAwait};
use crate::resource::IsRun;
use pi_world::{app::App, event::{ComponentAdded, ComponentChanged, ComponentRemoved}, fetch::{Has, OrDefault}, param_set::ParamSet, prelude::{Entity, Query, SingleRes, SingleResMut, World}, schedule::PreUpdate, system_params::Local, world::FromWorld};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::{OrInitSingleRes, OrInitSingleResMut};
use pi_bevy_render_plugin::{
    node::{Node, NodeId as GraphNodeId, ParamUsage},
    PiRenderDevice, PiRenderGraph, PiRenderQueue, PiSafeAtlasAllocator, RenderContext, TextureKeyAlloter,
};
use pi_futures::BoxFuture;
use pi_null::Null;
use pi_postprocess::prelude::{ImageMask, PostprocessTexture};
use pi_render::{
    components::view::target_alloc::{ShareTargetView, TargetDescriptor, TargetType, TextureDescriptor},
    renderer::texture::ETextureViewUsage,
    rhi::{
        asset::{AssetWithId, TextureRes}, dyn_uniform_buffer::BufferGroup, shader::BindLayout, texture::PiRenderDefault
    },
};
use pi_share::{Share, ShareRefCell};
use pi_style::style::{Aabb2, LinearGradientColor, MaskImage as MaskImage1, StyleType};
use smallvec::SmallVec;
use std::{mem::transmute, ops::Range};
use wgpu::{CommandEncoder, CompareFunction};
use crate::system::system_set::UiSystemSet;
use crate::prelude::UiStage;
use pi_world::schedule::Startup;
use pi_world::prelude::Plugin;
use pi_world::prelude::IntoSystemConfigs;

pub struct UiMaskImagePlugin;

impl Plugin for UiMaskImagePlugin {
    fn build(&self, app: &mut App) {
        app
            // 初始化渲染渐变色的图节点
            .add_startup_system(UiStage, init.after(init_root_graph))
            // 标记MaskImage所在节点为一个Pass
            .add_system(UiStage, 
                pass_life::pass_mark::<MaskImage>
                    .in_set(UiSystemSet::PassMark)
                    .run_if(mask_image_changed)
                    .before(pass_life::cal_context)
                    ,
            )
            // 设置mask_image的后处理效果
            .add_system(UiStage, 
                mask_image_path_post_process
                .in_set(UiSystemSet::PassSetting)
                .run_if(mask_image_changed)
            )
            .add_system(UiStage, 
                mask_image_linear_post_process
                .run_if(mask_image_changed)
                .after(crate::system::base::draw_obj::life_drawobj::update_render_instance_data)
                .in_set(UiSystemSet::IsRun)
            );
    }
}

/// system， 用于添加LinearMaskNode节点到渲染图中，该节点将MaskImage的渐变颜色渲染成纹理
/// LinearMaskNode图节点在LastGraphNode节点之前运行
pub fn init(
	mut rg: SingleResMut<PiRenderGraph>, 
	last_graph_id: SingleRes<LastGraphNode>,
	
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    match rg.add_node("MaskImageLinear".to_string(), LinearMaskNode, GraphNodeId::default()) {
        Ok(r) => {
            rg.add_depend(r, last_graph_id.0).unwrap();
        },
        Err(e) => log::error!("node: {:?}, {:?}", "MaskImageLinear".to_string(), e),
    };
}
/// 处理Path类型的MaskImage
/// 1. 加载MaskImage纹理
/// 2. 加载成功后， 设置MaskImage后处理
/// 3. MaskImage为MaskImage::Path(Atom::from(""))时， 删除MaskImage后处理效果
/// maskimage不可删除， 需要删除时， 设置值为MaskImage::Path(Atom::from(""))
pub fn mask_image_path_post_process(
    mark_type: OrInitSingleRes<RenderContextMarkType<MaskImage>>,
    mask_image_changed: ComponentChanged<MaskImage>,
    mask_image_added: ComponentAdded<MaskImage>,
    mut query0: Query<(&MaskImage, &mut PostProcessInfo)>,
    mut query1: Query<(&mut PostProcess, OrDefault<MaskImageClip>)>,

    mask1: Query<(Entity, &MaskImage)>,
    mut mask_texture: Query<&mut MaskTexture>,

    // remove: ComponentRemoved<MaskImage>,

    image_await: OrInitSingleRes<ImageAwait<Entity, MaskImage>>,
    texture_assets_mgr: SingleRes<ShareAssetMgr<AssetWithId<TextureRes>>>,
    queue: SingleRes<PiRenderQueue>,
    device: SingleRes<PiRenderDevice>,
    key_alloter: OrInitSingleRes<TextureKeyAlloter>,
    mut global_mark: SingleResMut<GlobalDirtyMark>,
) {
    // MaskImage删除，则删除对应的遮罩效果
    // for i in remove.iter() {
    //     if let Ok((mut post_list, mut post_info, has_mask_image)) = query.p1().get_mut(*i) {
    //         if has_mask_image {
    //             continue;
    //         }
    //         post_list.image_mask = None;
    //         render_mark_false(***mark_type, &mut render_mark_value);
    //     }
    // }

    // 保证安全
    let query2: &mut Query<(&mut PostProcess, OrDefault<MaskImageClip>)>= unsafe { transmute(&mut query1) };
    // 加载遮罩纹理
    // 设置后处理效果标记
    let mut f = |d: &mut MaskTexture, s: MaskTexture, entity| {
		let is_null = d.is_null();
        
        if let Ok((mut post_list, mask_image_clip)) = query2.get_mut(entity){
            let s = s.clone().0.unwrap();
            post_list.image_mask = Some(ImageMask::new(PostprocessTexture {
                use_x: (mask_image_clip.left * s.width as f32).round() as u32,
                use_y: (mask_image_clip.top * s.height as f32).round() as u32,
                use_w: ((mask_image_clip.right - mask_image_clip.left) * s.width as f32).round() as u32,
                use_h: ((mask_image_clip.bottom - mask_image_clip.top) * s.height as f32).round() as u32,
                width: s.width,
                height: s.height,
                format: s.format,
                view: ETextureViewUsage::TexWithId(s),
            }));  
        }
        *d = s;
        is_null
    };

    for entity in mask_image_changed.iter().chain(mask_image_added.iter()) {
       if let Ok((mask_image, mut post_info)) = query0.get_mut(*entity) {
            match &mask_image.0 {
                MaskImage1::Path(key) => {
                    if key.as_str() == "" {
                        // 如果是空路径， 表示删除MaskImage
                        post_info.effect_mark.set(***mark_type, false);
                        if let Ok((mut post_list, mask_image_clip)) = query1.get_mut(*entity){
                            post_list.image_mask = None;
                        }
                    }
                    load_image::<{OtherDirtyType::MaskImageTexture}, _, _, _>(
                        *entity,
                        key,
                        &image_await,
                        &device,
                        &queue,
                        &mut mask_texture,
                        &texture_assets_mgr,
                        &key_alloter,
                        &mut f,
                        &mut global_mark,
                    );
                }
                MaskImage1::LinearGradient(_) => (),
            };
            post_info.effect_mark.set(***mark_type, true);
       }
    }
    set_texture::<{OtherDirtyType::MaskImageTexture}, _, _, _>(&image_await, &mask1, &mut mask_texture, f, &mut global_mark);
}


pub fn mask_image_changed(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::MaskImage as usize | StyleType::MaskImageClip as usize).map_or(false, |display| {*display == true})
}

/// 处理LinearGradient类型的maskimage
/// 1. 为MaskImage的渐变色渲染分配纹理
/// 2. 为MaskImage的渐变色渲染准备渲染数据（填充实例数据）
/// 3. 设置后处理对象
/// 由于渲染LinearGradient对应的渐变颜色， 采用实例化渲染， 需要修改InstanceContext，
/// 因此此system需要在实例数据分配后运行
pub fn mask_image_linear_post_process(
    mask_image_changed: ComponentChanged<MaskImage>,
    mask_image_added: ComponentAdded<MaskImage>,
    // mask_image_changed1: ComponentAdded<MaskImage>,
    mut query: Query<(&MaskImage, &Quad, &mut PostProcess)>,

    mut instances: OrInitSingleResMut<InstanceContext>,
    atlas_allocator: SingleRes<PiSafeAtlasAllocator>,

    mut target_ty: Local<Option<TargetType>>, 
    mut mask_range: OrInitSingleResMut<MaskRenderRange>,
) {
    mask_range.render_list.clear();
     

    // 绘制MaskImage渐变效果的渲染目标类型（与普通的fbo不共用）
    let target_ty = match &*target_ty {
        Some(r) => *r,
        None => {
            let r = atlas_allocator.get_or_create_type(TargetDescriptor {
                colors_descriptor: SmallVec::from_slice(&[TextureDescriptor {
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::pi_render_default(),
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    base_mip_level: 0,
                    base_array_layer: 0,
                    array_layer_count: None,
                    view_dimension: None,
                }]),
                depth_descriptor: None,
                need_depth: false,
                default_width: 200,
                default_height: 200,
            });
            *target_ty = Some(r);
            r
        }
    };

    for entity in mask_image_changed.iter().chain(mask_image_added.iter()) {
        if let Ok((mask_image, quad, mut post_process)) = query.get_mut(*entity) {
            if let MaskImage1::LinearGradient(color) = &mask_image.0 {
                let mut render_range = instances.instance_data.cur_index()..instances.instance_data.cur_index();
//                 // 创建fbo
                let size = calc_size(&quad, color) as u32;

                let mut render_target = None;
                if let Some(mask) = &post_process.image_mask {
                    if let ETextureViewUsage::SRT(r) = &mask.image.view {
                        let rect = r.rect();
                        if rect.width() >= size as i32 && rect.height() >= size as i32 {
                            // mask_image改变，绘制渐变色到原有纹理上
                            render_target = Some(r.clone());
                        }
                    }
                }

                // 以下用于创建绘制用渐变颜色描述的MaskImage的RenderObj
                let render_target = match render_target {
                    Some(r) => r,
                    None => {
                        let e: [ShareTargetView; 0] = [];
                        atlas_allocator.allocate(size, size, target_ty, e.iter())
                    }
                };

                // 设置后处理纹理
                let mut t = PostprocessTexture::from_share_target(render_target.clone(), wgpu::TextureFormat::pi_render_default());
                t.use_x += 1;
                t.use_y += 1;
                t.use_w -= 2;
                t.use_h -= 2;
                post_process.image_mask = Some(ImageMask::new(t));  

                // 填充实例数据  
                let instance_id: usize = instances.instance_data.alloc_instance_data();
                render_range.end = instances.instance_data.cur_index(); 
                mask_range.render_list.push((render_range, render_target));

                let mut instance_data = instances.instance_data.instance_data_mut(instance_id);
                let mut render_flag = instance_data.get_render_ty();
                set_linear_gradient_instance_data(
					color, 
					&Aabb2::new(
						Point2::new( 0.0, 0.0), 
						Point2::new( 1.0, 1.0)
					), 
					&mut instance_data, 
					&mut render_flag
				);
                instance_data.set_data(&BoxUniform([0.0, 0.0, 1.0, 1.0].as_slice()));
                instance_data.set_data(&QuadUniform(&[
                    -1.0, 1.0,
                    -1.0, -1.0,
                    1.0, -1.0,
                    1.0, 1.0,
                ]));

                render_flag &= !(1 << RenderFlagType::NotVisibility as usize);
                render_flag &= !(1 << RenderFlagType::IgnoreCamera as usize);  //（不需要乘视图矩， 投影矩阵）
                instance_data.set_data(&TyMeterial([render_flag as f32].as_slice()));         
            }
        }
    }
  
}

pub struct UnitCamera (pub BufferGroup);

/// 用于标记哪些实例是MaskImage渐变颜色渲染
#[derive(Debug, Default)]
pub struct MaskRenderRange {
    pub render_list: Vec<(Range<usize>, ShareTargetView)>,
}

impl FromWorld for UnitCamera {
    fn from_world(world: &mut World) -> Self {
        let matrix = WorldMatrix::default();
        let instances = world.get_single_res_mut::<InstanceContext>().unwrap();
        let mut camera_group = instances.camera_alloter.alloc();
        let _ = camera_group.set_uniform(&ProjectUniform(matrix.as_slice()));
        let _ = camera_group.set_uniform(&ViewUniform(matrix.as_slice()));
        Self(camera_group)
    }
}


// #[derive(SystemParam)]
// pub struct QueryParam<'w, 's> {
//     mask_draw_list: OrInitSingleRes<'w, LinearMaskDrawList>,
//     query: Query<'w, 's, &'static DrawState>,
//     depth_cache: OrInitSingleRes<'w, DepthCache>,
//     // // // 清屏相关参数
//     // fbo_clear_color: SingleRes<'w, DynFboClearColorBindGroup>,
//     // clear_draw: SingleRes<'w, ClearDrawObj>,
// }

// 用于绘制渐变颜色声明的MaskImage
pub struct LinearMaskNode;
impl Node for LinearMaskNode {
    type Input = ();
    type Output = ();

	type BuildParam = ();
    type RunParam = (
        SingleRes<'static, InstanceContext>,
        OrInitSingleResMut<'static, MaskRenderRange>,
    );

	fn build<'a>(
		&'a mut self,
		// world: &'a mut pi_world::world::World,
		_param: &'a mut Self::BuildParam,
		_context: pi_bevy_render_plugin::RenderContext,
		_input: &'a Self::Input,
		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
		_id: GraphNodeId,
		_from: &'a [GraphNodeId],
		_to: &'a [GraphNodeId],
	) -> Result<Self::Output, String> {
		Ok(())
	}

    fn run<'a>(
        &'a mut self,
        // world: &'a World,
        param: &'a Self::RunParam,
        _context: RenderContext,
        commands: ShareRefCell<CommandEncoder>,
        _input: &'a Self::Input,
        _usage: &'a ParamUsage,
        _id: GraphNodeId,
        _from: &'a [GraphNodeId],
        _to: &'a [GraphNodeId],
    ) -> BoxFuture<'a, Result<(), String>> {
        Box::pin(async move {
            if param.1.render_list.len() == 0 {
                return Ok(());
            }

            let mut render_state = RenderState {
				reset: true,
				pipeline: param.0.mask_image_pipeline.clone(),
				texture: param.0.batch_texture.default_texture_group.clone(),
			};
            let mut draw_state: InstanceDrawState = InstanceDrawState {
                instance_data_range: 0..0,
                pipeline: Some(param.0.mask_image_pipeline.clone()),
                texture_bind_group: Some(param.0.batch_texture.default_texture_group.clone()),
            };
            let mut commands = commands.borrow_mut();
            for (range, target_view) in param.1.render_list.iter() {
                draw_state.instance_data_range = range.clone();
                let view_port = target_view.rect();
                // 创建一个渲染Pass
                let view_port = Aabb2::new(
                    Point2::new(0.0, 0.0),
                    Point2::new((view_port.max.x - view_port.min.x) as f32, (view_port.max.y - view_port.min.y) as f32),
                );
                
                let (mut rp, view_port, _clear_port, _) = create_rp_for_fbo(target_view, &mut commands, &view_port, &view_port, None); 
                rp.set_viewport(view_port.0, view_port.1, view_port.2, view_port.3, 0.0, 1.0);
                param.0.set_pipeline(&mut rp, &draw_state, &mut render_state);
                let group = param.0.default_camera.get_group();
				rp.set_bind_group(CameraBind::set(), group.bind_group, group.offsets);
                param.0.draw(&mut rp, &draw_state, &mut render_state);

                render_state.reset = true;
            }
            Ok(())
        })
    }
}

fn calc_size(quad: &Aabb2, linear: &LinearGradientColor) -> u32 {
    let width = quad.maxs.x - quad.mins.x;
    let height = quad.maxs.y - quad.mins.y;

    let l = (width * width + height * height).sqrt();
    let mut min: f32 = 1.0;
    let mut pre_pos: f32 = 0.0;
    for item in linear.list.iter() {
        let diff = item.position - pre_pos;
        if diff != 0.0 {
            min = min.min(diff);
            pre_pos = item.position;
        }
    }

    if min == 1.0 {
        return 10;
    }

    // 保证渐变百分比中，渐变端点之间的距离至少两个像素
    let at_least = (2.0_f32.min((min * l).ceil() + 1.0) / min).min(width.max(height) / 4.0);
    // 渐变颜色渲染尺寸为20的整数倍，使得不同大小的渐变色，可以共用同一张纹理
    // 加2，使得分配的纹理四周可以扩充一个像素，避免采样问题导致边界模糊 TODO
    return ((at_least / 10.0).ceil() * 10.0) as u32;
}


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
