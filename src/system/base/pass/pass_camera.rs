//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use std::mem::transmute;

use pi_bevy_render_plugin::{render_cross::GraphId, PiRenderGraph};
use pi_null::Null;
use pi_style::{style::StyleType, style_type::AsImageType};
use pi_world::{event::{ComponentAdded, ComponentChanged, Event}, prelude::{Changed, Entity, Mut, Query, SingleRes, Ticker}, single_res::SingleResMut};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};

use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_share::{Share, ShareWeak};

use crate::{
    components::{
        calc::{
            BackgroundImageTexture, BorderImageTexture, InPassId, IsShow, MaskTexture, OverflowDesc, Quad, StyleBit, TransformWillChangeMatrix, View, WorldMatrix
        }, pass_2d::{
            Camera, IsSteady, ParentPassId, PostProcessInfo, RenderTarget, RenderTargetCache, StrongTarget
        }, user::{Aabb2, AsImage, Canvas, Point2, Vector2, Viewport}
    }, resource::{
        draw_obj::{InstanceContext, TargetCacheMgr}, GlobalDirtyMark, IsRun, OtherDirtyType, RenderDirty, ShareFontSheet
    }, shader1::batch_meterial::{ProjectUniform, Sdf2TextureSizeUniform, ViewUniform}, system::{base::node::user_setting::StyleChange, utils::{create_project, rotatequad_quad_intersection}}, utils::tools::intersect
};
use crate::components::calc::{StyleMarkType, style_bit};
lazy_static! {
	pub static ref OTHER_RENDER_DIRTY: StyleMarkType = style_bit()
        .set_bit(OtherDirtyType::MaskImageTexture as usize)
        .set_bit(OtherDirtyType::BorderImageTexture as usize)
		.set_bit(OtherDirtyType::BackgroundImageTexture as usize);
}

pub fn calc_pass_dirty(
    // mut global_mark: OrInitSingleRes<GlobalDirtyMark>,
    mut render_dirty: OrInitSingleResMut<RenderDirty>,
    style_dirty_list: Event<StyleChange>,
    quad_changed: ComponentChanged<Quad>,

    bg_image_changed: ComponentChanged<BackgroundImageTexture>,
    border_image_changed: ComponentChanged<BorderImageTexture>,
    mask_image_changed: ComponentChanged<MaskTexture>,
    canvas_changed: ComponentChanged<Canvas>, // canvas一定需要修改

    mut query: Query<(&mut Camera, &ParentPassId)>,
    query1: Query<&InPassId>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

    if render_dirty.0 {
        // 如果渲染脏，则全部脏， 不需要计算各pass的脏
        style_dirty_list.mark_read();
        quad_changed.mark_read();
        bg_image_changed.mark_read();
        border_image_changed.mark_read();
        canvas_changed.mark_read();
        mask_image_changed.mark_read();
        render_dirty.1 = true;
        return;
    }

    let mut is_dirty = false;

    // 用户修改，脏区域发生变化
    // let mut p2 = query_pass.p2();
    // 样式脏， 引起渲染脏， 设置所在pass和递归父pass的draw_changed
    for node_id in style_dirty_list.iter() {
        let in_pass_id = match query1.get(**node_id) {
            Ok(r) => r,
            _ => continue,
        };
        is_dirty = true;
		// log::debug!("dirty========style {:?}, {:?}", node_id, in_pass_id);
        mark_pass_dirty(***in_pass_id, &mut query);
    }

    // 处理包围盒改变前的区域，与脏区域求并
    if quad_changed.len() > 0 ||
    bg_image_changed.len() > 0 ||
    border_image_changed.len() > 0 ||
    canvas_changed.len() > 0 ||
    mask_image_changed.len() > 0 {
        for node_id in quad_changed.iter().chain(bg_image_changed.iter()).chain(border_image_changed.iter()).chain(canvas_changed.iter()).chain(mask_image_changed.iter()) {
            let in_pass_id = match query1.get(*node_id) {
                Ok(r) => r,
                _ => continue,
            };
            is_dirty = true;
            // log::debug!("dirty========other {:?}, {:?}", node_id, in_pass_id);
            mark_pass_dirty(***in_pass_id, &mut query);
        }
    }

    render_dirty.1 = is_dirty;
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera(
    mut query_pass: Query<
        (
            Entity,
            &mut Camera,
            Ticker<&View>,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            &IsSteady,
            &mut RenderTarget,
            &Quad,
            Ticker<&IsShow>,
        ),
    >,

    query_root: Query<Ticker<&Viewport>>,
    font_sheet: SingleRes<ShareFontSheet>,
	mut instance_context: SingleResMut<InstanceContext>,
    mut render_dirty: OrInitSingleResMut<RenderDirty>,
    assets: SingleRes<TargetCacheMgr>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
    // 没有任何脏， 不需要计算相机
    if !render_dirty.2 && !render_dirty.1 {
        return;
    }
    // let p0 = query_root.p0();
	let font_sheet = font_sheet.0.borrow();
	let font_texture_size = font_sheet.texture_size();
	let font_type =  font_sheet.font_mgr().font_type;

	// 所有根共同的脏区域
    // let mut all_dirty_rect = Aabb2::new(Point2::new(std::f32::MAX, std::f32::MAX), Point2::new(std::f32::MIN, std::f32::MIN));

    let mut view_port_is_dirty = false;
    // 迭代根节点，得到最大脏包围盒
    for view_port in query_root.iter() {
        if view_port.is_changed() {
            view_port_is_dirty = true;
        }
        // box_aabb(&mut all_dirty_rect, &view_port.0);
    }
    let render_dirty1 = render_dirty.0;
    render_dirty.0 = false;

    let mut is_render_own_changed = false;

    // log::debug!("calc camera=========", );

    let calc_camera = |
        (
            entity, 
            mut camera, 
            overflow_aabb, 
            willchange_matrix, 
            layer, 
            as_image,
            is_steady, 
            mut render_target, 
            quad, 
            is_show): 
        (
            Entity,
            Mut<Camera>,
            Ticker<&View>,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            &IsSteady,
            Mut<RenderTarget>,
            &Quad,
            Ticker<&IsShow>,
        )
    | -> bool {

        let camera_bypass = camera.bypass_change_detection();
        let old_is_render_own = camera_bypass.is_render_own;
        let old_is_render_to_parent = camera_bypass.is_render_to_parent;
        camera_bypass.is_render_own = false;
        // log::debug!("camera.is_render_own = {:?};", (entity, camera_bypass.draw_changed, render_dirty1, view_port_is_dirty, as_image, &render_target.cache));
        let local_dirty_mark = camera_bypass.draw_changed || render_dirty1 || view_port_is_dirty;
        // local_dirty_mark = true;
        camera_bypass.draw_changed = false;

        log::debug!("change==========={:?}", (entity, camera_bypass.draw_changed, render_dirty.0, is_show.get_visibility(), is_show.get_display()));
        // 检查render_target的缓存情况， 设置rendertarget
        check_render_target(&mut render_target, as_image, is_steady.0);

        if !is_show.get_visibility() || !is_show.get_display() {
            // 如果设置为隐藏，之前的渲染结果也需要释放
            // 因为， 如果将其缓存，直到重新设置为可见， 中间发生了哪些改变不可知，也不知道fbo是否需要重新渲染， 因此，直接释放掉
            render_target.target = StrongTarget::None;
            render_target.cache = RenderTargetCache::None;
			return old_is_render_own != camera_bypass.is_render_own;
		}

        let view_port = match query_root.get(layer.root()) {
            Ok(r) => r,
            Err(_) => return old_is_render_own != camera_bypass.is_render_own,
        };

        let overflow_aabb = &*overflow_aabb;
        if let StrongTarget::None = render_target.target {
            
        } else {
            
            if !local_dirty_mark {
                
                // 存在fbo缓存， 且本地不脏，则不需要渲染
                return old_is_render_own != camera_bypass.is_render_own;
            }
        }

        let dirty_rect = &*view_port;
        // 
        let no_rotate_view_aabb = if let OverflowDesc::Rotate(oveflow_rotate) = &overflow_aabb.desc {
            // let mins = oveflow_rotate.rotate_matrix_invert * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
            // 脏区域变化到当前上下文的非旋转坐标系，与当前上下文的视图aabb相交，得到最终视口区域
            let rr = rotatequad_quad_intersection(
                &(
                    Vector2::new(view_port.mins.x, view_port.mins.y),
                    Vector2::new(view_port.mins.x, view_port.maxs.y),
                    Vector2::new(view_port.maxs.x, view_port.maxs.y),
                    Vector2::new(view_port.maxs.x, view_port.mins.y),
                ),
                &oveflow_rotate.world_rotate_invert,
                &overflow_aabb.view_box.aabb,
            );

            // let r = calc_bound_box(&aabb, &oveflow_rotate.rotate_matrix_invert);
            // let rr = intersect(&overflow, &r).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0),
            // Point2::new(0.0, 0.0)));
            // log::debug!("rr=====id: {:?} \nrotate_matrix_invert: {:?}, \nview_port: {:?}, \nview_box.aabb: {:?}, \n rr: {:?}, ", entity, &oveflow_rotate.world_rotate_invert, view_port, overflow_aabb.view_box.aabb, rr);
            rr
        } else {
            intersect(&overflow_aabb.view_box.aabb, &view_port).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
        };

                // log::debug!("viewport======={:?}, \nview_aabb={:?}, \noverflow_aabb={:?}, \ndirty_rect={:?}", entity, no_rotate_view_aabb, overflow_aabb, dirty_rect);

        log::debug!("pass_id2 22========={:?}, {:?}", entity, (&*dirty_rect, overflow_aabb, no_rotate_view_aabb, !is_show.get_visibility(), !is_show.get_display()));
        if no_rotate_view_aabb.mins.x >= no_rotate_view_aabb.maxs.x || no_rotate_view_aabb.mins.y >= no_rotate_view_aabb.maxs.y {
            // 如果视口为0， 则不需要渲染
            return old_is_render_own != camera_bypass.is_render_own;
        }

        // 计算视图区域（世界坐标系）
        // let aabb_temp;
        // let view_world_aabb = match &overflow_aabb.desc {
        //     OverflowDesc::Rotate(r) => {
        //         aabb_temp = calc_bound_box(&no_rotate_view_aabb, &r.world_rotate);
        //         &aabb_temp
        //     }
        //     _ => &no_rotate_view_aabb,
        // };

		// log::debug!("last_dirty======{:?}, {:?}", entity, cull_aabb);
        let aabb = Aabb2::new(
            Point2::new(no_rotate_view_aabb.mins.x.floor(), no_rotate_view_aabb.mins.y.floor()),
            Point2::new(no_rotate_view_aabb.maxs.x.ceil(), no_rotate_view_aabb.maxs.y.ceil()),
        );

        // 计算投影矩阵（投影矩阵将view_aabb范围内的对象投影到-1~1， 注意view_aabb所在坐标系为当前节点的非旋转坐标系）
        let project_matrix = create_project(aabb.mins.x, aabb.maxs.x, aabb.mins.y, aabb.maxs.y);

        // 计算视图矩阵
        let view_temp;
        // 将willchange应用到视图矩阵中
        let mut view_matrix = match &willchange_matrix.0 {
            Some(r) => &r.will_change.0,
            None => {
                view_temp = WorldMatrix::default().0;
                &view_temp
            }
        };
        let view_temp1;
        if let OverflowDesc::Rotate(r) = &overflow_aabb.desc {
            // 由于投影矩阵是将非旋转坐标系中的一个区域投影到-1~1，因此需要将渲染物体变换到非旋转坐标系中
            view_temp1 = r.world_rotate_invert * view_matrix;
            view_matrix = &view_temp1;
        }
        
        // log::debug!("pass_id=========\nentity: {:?}, \nproject_matrix: {:?}, \nview_matrix: {}, \nwillchange_matrix:{:?} \naabb:{:?}, \noverflow_aabb: {:?}", entity, project_matrix, view_matrix, willchange_matrix, aabb, overflow_aabb);
        // log::debug!("pass_id2=========\nentity: {:?}, \nproject_matrix: {:?}, \nview_matrix: {}, \nwillchange_matrix:{:?} \naabb:{:?}, \noverflow_aabb: {:?}", entity, project_matrix, view_matrix, willchange_matrix, aabb, overflow_aabb);

        let mut camera_group = instance_context.camera_alloter.alloc();
        camera_group.set_uniform(&ProjectUniform(project_matrix.as_slice()));
        camera_group.set_uniform(&ViewUniform(view_matrix.as_slice()));
		let data_texture_size = if let pi_hal::font::font::FontType::Sdf2 = font_type { 
			font_sheet.font_mgr().table.sdf2_table.data_packer_size().clone()
		} else {
			pi_hal::font::font::Size {
				width: 0,
				height: 0,
			}
		};
        log::debug!("font_texture_size==============={:?}", (font_texture_size.width as f32, font_texture_size.height as f32));
		camera_group.set_uniform(&Sdf2TextureSizeUniform(&[font_texture_size.width as f32, font_texture_size.height as f32]));

        *camera = Camera {
            // view: view_matrix.clone(),
            // project: project_matrix,
            bind_group: Some(DrawBindGroup::Offset(camera_group)),
            view_port: aabb,
            // world_matrix: world_matrix.clone(),
            is_render_own: true,
            draw_changed: false,
            is_render_to_parent: old_is_render_to_parent,
        };

        // 删除原有缓冲（内容发生改变会重新渲染， 原有缓冲没有作用）
        match &render_target.cache {
            RenderTargetCache::None => (),
            RenderTargetCache::Strong(droper) => {assets.0.pop_by_filter(|r| {
                Share::as_ptr(&r.0) == Share::as_ptr(droper)
            });},
            RenderTargetCache::Weak(droper) => {assets.0.pop_by_filter(|r| {
                Share::as_ptr(&r.0) == ShareWeak::as_ptr(droper)
            });},
        };
        render_target.target = StrongTarget::None;
        render_target.cache = RenderTargetCache::None;


        let width = camera.view_port.maxs.x - camera.view_port.mins.x;
        let height = camera.view_port.maxs.y - camera.view_port.mins.y;
        if layer.root() == entity {
            // 根节点必须分配与根节点overflow_aabb等大的fbo
            // 因为根节点fbo要缓冲上一帧的内容，其fbo大小必须包含整个视口内容
            let overflow_aabb = &overflow_aabb.view_box.aabb;
            render_target.bound_box = Aabb2::new(
                Point2::new(overflow_aabb.mins.x.floor(), overflow_aabb.mins.y.floor()),
                Point2::new(overflow_aabb.maxs.x.ceil(), overflow_aabb.maxs.y.ceil()),
            );
        } else {
            // 非根节点，在没有旧的fbo的情况下，只需要开与渲染区域等大的fbo
            render_target.bound_box = camera.view_port.clone();
            render_target.accurate_bound_box = Aabb2::new(
                Point2::new((no_rotate_view_aabb.mins.x - camera.view_port.mins.x) / width, (no_rotate_view_aabb.mins.y - camera.view_port.mins.y) / height),
                Point2::new((no_rotate_view_aabb.maxs.x - camera.view_port.maxs.x) / width, (no_rotate_view_aabb.maxs.y - camera.view_port.maxs.y) / height),
            );
        }
        

        return old_is_render_own != camera.is_render_own;

        // if let &StrongTarget::None = &render_target.target {
		// 	// if bg.is_some() {
		// 	// 	log::debug!("aaaa1================={:?}, {:?}, {:?}, {}", entity, render_target.bound_box, &camera.view_port, render_target.bound_box.maxs.x - render_target.bound_box.mins.x);
		// 	// 	log::debug!("aaaa0================={:?}, {:?}, {:?}, {}", entity, view_port, );
		// 	// }
		// 	if layer.root() == entity {
		// 		// 根节点必须分配与根节点overflow_aabb等大的fbo
		// 		// 因为根节点fbo要缓冲上一帧的内容，其fbo大小必须包含整个视口内容
		// 		let overflow_aabb = &overflow_aabb.view_box.aabb;
		// 		render_target.bound_box = Aabb2::new(
		// 			Point2::new(overflow_aabb.mins.x.floor(), overflow_aabb.mins.y.floor()),
		// 			Point2::new(overflow_aabb.maxs.x.ceil(), overflow_aabb.maxs.y.ceil()),
		// 		);
		// 	} else {
		// 		// 非根节点，在没有旧的fbo的情况下，只需要开与渲染区域等大的fbo
		// 		render_target.bound_box = camera.view_port.clone();
		// 	}
		// } else {
		// 	// 能进入该分支， 说明节点内容fbo需要强制缓冲（强制缓冲的内容应该包含节点下的所有内容，而不仅仅是当前脏区域的内容， 因此bound_box应为节点内容大小）
		// 	let overflow_aabb = &overflow_aabb.view_box.aabb;
		// 	let overflow_aabb = Aabb2::new(
		// 		Point2::new(overflow_aabb.mins.x.floor(), overflow_aabb.mins.y.floor()),
		// 		Point2::new(overflow_aabb.maxs.x.ceil(), overflow_aabb.maxs.y.ceil()),
		// 	);
		// 	// log::debug!("target_size_change========{:?}, {:?}, {:?}, {:?}", entity, &render_target.bound_box, overflow_aabb.view_box.aabb.clone(), &camera.view_port);
			

			
        //     // let target_size_change = !(eq_f32(
        //     //     render_target.bound_box.maxs.x - render_target.bound_box.mins.x,
        //     //     overflow_aabb.maxs.x - overflow_aabb.mins.x,
        //     // ) && eq_f32(
        //     //     render_target.bound_box.maxs.y - render_target.bound_box.mins.y,
        //     //     overflow_aabb.maxs.y - overflow_aabb.mins.y,
        //     // ));

		// 	render_target.bound_box = overflow_aabb; 

        //     // if target_size_change {
				
		// 	// 	// if bg.is_some() {
		// 	// 	// 	log::debug!("aaaa2================={:?}, {:?}, {:?}, {}", entity, render_target.bound_box, &camera.view_port, render_target.bound_box.maxs.x - render_target.bound_box.mins.x);
		// 	// 	// }
        //     //     // 从资源管理器中删除原有的渲染目标（TODO， 另外还需要在RenderTarget销毁时， 从资源管理器中删除）
        //     //     render_target.target = StrongTarget::None; // 设置为None， 等待渲染时重新分配
        //     //     render_target.cache = RenderTargetCache::None;
        //     // }
        //     // // 如果本地脏区域面积为0，并且渲染目标尺寸未改变， 则该camera下的物体不需要改变
        //     // if !local_dirty_mark && !target_size_change {
        //     //     camera.is_change = false;
        //     // }
        // }
    };
    // 当视口发生变化时， 需要遍历所有的pass2d从新计算相机
    for r in query_pass.iter_mut() {
        is_render_own_changed |= calc_camera(r);
    }
    instance_context.rebatch |= is_render_own_changed;

    // if view_port_is_dirty {
    //     // 当视口发生变化时， 需要遍历所有的pass2d从新计算相机
    //     for r in query_pass.p0().iter_mut() {
    //         calc_camera(r);
    //     }
    // } else {
    //     // 当视口未发生改变时， 只需要重新计算脏了的pass
        
    //     for r in query_pass.p1().iter_mut() {
    //         calc_camera(r);
    //     }
    // }
    
}


pub fn check_render_target(render_target: &mut RenderTarget, as_image: Option<&AsImage>, is_steady: bool) {
    let as_image = match as_image {
        Some(r) => r.level,
        None => pi_style::style::AsImage::None,
    };
    
    match (as_image, is_steady) {
        (pi_style::style::AsImage::None, false) => {
            // 设置render_target.cache为none，在渲染时动态分配rendertarget
            render_target.cache = RenderTargetCache::None;
        }
        (pi_style::style::AsImage::Advise, _) | (_, true) => {
            match &render_target.cache {
                RenderTargetCache::None => return,
                RenderTargetCache::Strong(r) => {
                    render_target.target = StrongTarget::Asset(r.clone());
                    // 缓存修改为弱引用
                    let weak = Share::downgrade(r);
                    render_target.cache = RenderTargetCache::Weak(weak);
                }
                RenderTargetCache::Weak(r) => {
                    match ShareWeak::upgrade(r) {
                        Some(r) => {
                            // 弱引用升级成功，返回强引用，如果相机被激活，外部应该将其放在render_target.target上， 避免在渲染时， 该弱引用对应的值已被销毁
                            render_target.target = StrongTarget::Asset(r.clone());
                        }
                        None => {
                            // 弱引用升级不成功，清理掉弱引用
                            render_target.cache = RenderTargetCache::None;
                        }
                    };
                }
            }
        }
        (pi_style::style::AsImage::Force, _) => {
            match &render_target.cache {
                RenderTargetCache::None => return,
                RenderTargetCache::Strong(r) => {
                    // 返回强引用
                    render_target.target = StrongTarget::Asset(r.clone());
                }
                RenderTargetCache::Weak(r) => {
                    match ShareWeak::upgrade(r) {
                        Some(r) => {
                            render_target.target = StrongTarget::Asset(r.clone());
                            // 缓存强引用
                            render_target.cache = RenderTargetCache::Strong(r);
                        }
                        None => {
                            // 弱引用升级不成功，清理掉弱引用
                            render_target.cache = RenderTargetCache::None;
                        }
                    };
                }
            }
        }
    }
}


pub fn calc_pass_active(
    mut query: Query<(&mut Camera, &PostProcessInfo, &ParentPassId, &GraphId)>,
	r: OrInitSingleRes<IsRun>,
    mut render_dirty: OrInitSingleResMut<RenderDirty>,
    instance_context: OrInitSingleRes<InstanceContext>,
    mut rg: SingleResMut<PiRenderGraph>,
) {
	if r.0 {
		return;
	}
    
    let is_dirty = render_dirty.1 || render_dirty.2;
    render_dirty.0 = false;
    render_dirty.2 = render_dirty.1; // 设置为上一帧是否脏
    render_dirty.1 = false;
    if !is_dirty {
        return;
    }

    let q1: &Query<(&mut Camera, &PostProcessInfo, &ParentPassId, &GraphId)> = unsafe {transmute(&query)};
    // log::debug!("calc_pass_active======{:?}", (instance_context.pass_toop_list.len(), render_dirty.1, render_dirty.2));
    for node in instance_context.pass_toop_list.iter().rev() {
        if let Ok((mut camera, _post_info, mut parent_pass, graph_id)) = query.get_mut(*node) {
            let old_is_render_to_parent = camera.is_render_to_parent;
            let camera = camera.bypass_change_detection();
            if parent_pass.0.is_null() {
                camera.is_render_to_parent = true;
            } else {
                while let Ok((c1, p1, pp1, _)) = q1.get(parent_pass.0.0) {
                    if !p1.has_effect() {
                        parent_pass = pp1;
                        continue;
                    }

                    camera.is_render_to_parent = c1.is_render_own;
                    // log::debug!("set======{:?}", (node, parent_pass.0.0, graph_id.0, camera.is_render_to_parent, camera.is_render_own, c1.is_render_own, ));
                    camera.is_render_own = camera.is_render_own && c1.is_render_own;
                    break;
                    
                } 
            }
            // log::debug!("set_enable======{:?}", (node, graph_id.0, camera.is_render_to_parent, camera.is_render_own ));
            if old_is_render_to_parent != camera.is_render_to_parent {
                let _ = rg.set_enable(graph_id.0, camera.is_render_to_parent);
            }
        }
    }
    // let r = pi_time::Instant::now();
    // let mut i = 0;
    // for entity in query_root.iter() {
    //     mark_children_active(entity, true, &mut query, &query_c, &mut i);
    // }
    // println!("calc_pass_active {:?}", (pi_time::Instant::now() - r, i));
}


#[inline]
fn mark_pass_dirty(mut pass_id: Entity, query: &mut Query<(&mut Camera, &ParentPassId)>,) {
    while let Ok((mut camera, parent_pass)) = query.get_mut(pass_id) {
        if camera.draw_changed {
            break;
        }

        camera.bypass_change_detection().draw_changed = true;
        pass_id = parent_pass.0.0;
    }
}
