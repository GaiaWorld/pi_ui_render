//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use std::mem::transmute;

use pi_bevy_render_plugin::{render_cross::GraphId, PiRenderGraph};
use pi_null::Null;
use pi_world::{prelude::{Entity, Mut, Query, SingleRes, Ticker}, single_res::SingleResMut};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};

use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_share::{Share, ShareWeak};

use crate::{
    components::{
        calc::{
            IsShow, OverflowDesc, Quad, TransformWillChangeMatrix, View, WorldMatrix
        }, pass_2d::{
            Camera, DirtyRect, DirtyRectState, IsSteady, ParentPassId, PostProcessInfo, RenderTarget, RenderTargetCache
        }, user::{Aabb2, AsImage, Point2, Vector2, Viewport}
    }, resource::{
        draw_obj::{InstanceContext, TargetCacheMgr}, IsRun, RenderContextMarkType, RenderDirty, ShareFontSheet
    }, shader1::batch_meterial::{ProjectUniform, Sdf2TextureSizeUniform, ViewUniform}, system::utils::{create_project, rotatequad_quad_intersection}, utils::tools::{eq_f32, intersect}
};

// pub fn calc_pass_dirty(
//     // mut global_mark: OrInitSingleRes<GlobalDirtyMark>,
//     mut render_dirty: OrInitSingleResMut<RenderDirty>,
//     style_dirty_list: Event<StyleChange>,
//     quad_changed: ComponentChanged<Quad>,
//     willchange_changed: ComponentChanged<TransformWillChangeMatrix>,

//     bg_image_changed: ComponentChanged<BackgroundImageTexture>,
//     border_image_changed: ComponentChanged<BorderImageTexture>,
//     mask_image_changed: ComponentChanged<MaskTexture>,
//     canvas_changed: ComponentChanged<Canvas>, // canvas一定需要修改

//     mut query: Query<(&mut Camera, &ParentPassId)>,
//     query1: Query<&InPassId>,
// 	r: OrInitSingleRes<IsRun>
// ) {
// 	if r.0 {
// 		return;
// 	}

//     if render_dirty.0 {
//         // 如果渲染脏，则全部脏， 不需要计算各pass的脏
//         style_dirty_list.mark_read();
//         quad_changed.mark_read();
//         bg_image_changed.mark_read();
//         border_image_changed.mark_read();
//         canvas_changed.mark_read();
//         mask_image_changed.mark_read();
//         willchange_changed.mark_read();
//         render_dirty.1 = true;
//         return;
//     }

//     let mut is_dirty = false;

//     // 用户修改，脏区域发生变化
//     // let mut p2 = query_pass.p2();
//     // 样式脏， 引起渲染脏， 设置所在pass和递归父pass的draw_changed
//     for node_id in style_dirty_list.iter() {
//         let in_pass_id = match query1.get(**node_id) {
//             Ok(r) => r,
//             _ => continue,
//         };
//         is_dirty = true;
// 		// log::debug!("dirty========style {:?}, {:?}", node_id, in_pass_id);
//         mark_pass_dirty(***in_pass_id, &mut query);
//     }

//     // 处理包围盒改变前的区域，与脏区域求并
//     if quad_changed.len() > 0 ||
//     bg_image_changed.len() > 0 ||
//     border_image_changed.len() > 0 ||
//     canvas_changed.len() > 0 ||
//     mask_image_changed.len() > 0 {
//         for node_id in quad_changed.iter()
//             .chain(bg_image_changed.iter())
//             .chain(border_image_changed.iter())
//             .chain(canvas_changed.iter())
//             .chain(mask_image_changed.iter())  {
//             let in_pass_id = match query1.get(*node_id) {
//                 Ok(r) => r,
//                 _ => continue,
//             };
//             is_dirty = true;
//             // log::debug!("dirty========other {:?}, {:?}", node_id, in_pass_id);
//             mark_pass_dirty(***in_pass_id, &mut query);
//         }
//     }

//     if willchange_changed.len() > 0 {
//          for node_id in willchange_changed.iter() {
//             mark_pass_dirty(*node_id, &mut query);
//             let in_pass_id = match query1.get(*node_id) {
//                 Ok(r) => r,
//                 _ => continue,
//             };
//             is_dirty = true;
//             // log::debug!("dirty========other {:?}, {:?}", node_id, in_pass_id);
//             mark_pass_dirty(***in_pass_id, &mut query);
//         }
//     }

//     render_dirty.1 = is_dirty;
// }

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera(
    mut query_pass: Query<
        (
            Entity,
            &ParentPassId,
            &mut Camera,
            Ticker<&View>,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            &IsSteady,
            &mut RenderTarget,
            &Quad,
            &IsShow,
            &PostProcessInfo,
        )
    >,

    mut query_dirty_rect: Query<&mut DirtyRect>,
    // mut query_pass_root: Query<
    //     (
    //         Entity,
    //         &mut Camera,
    //         Ticker<&View>,
    //         &TransformWillChangeMatrix,
    //         &Layer,
    //         Option<&AsImage>,
    //         &IsSteady,
    //         &mut RenderTarget,
    //         &Quad,
    //         &IsShow,
    //     ),
    //     With<Root>,
    // >,

    mut query_root: Query<Ticker<&Viewport>>,
    font_sheet: SingleRes<ShareFontSheet>,
	mut instance_context: SingleResMut<InstanceContext>,
    mut render_dirty: OrInitSingleResMut<RenderDirty>,
    assets: SingleRes<TargetCacheMgr>,
	r: OrInitSingleRes<IsRun>,
    as_image_mark_type: OrInitSingleRes<RenderContextMarkType<AsImage>>,
) {
    log::debug!("calc_camera===============================");
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
    // // 迭代根节点，得到最大脏包围盒
    for view_port in query_root.iter() {
        if view_port.is_changed() {
            view_port_is_dirty = true;
        }
        // box_aabb(&mut all_dirty_rect, &view_port.0);
    }
    let render_dirty1 = render_dirty.0;
    render_dirty.0 = false;

    let mut is_render_own_changed = false;

    let calc_camera = |
        (
            entity, 
            parent_pass_id,
            mut camera, 
            overflow_aabb, 
            willchange_matrix, 
            layer, 
            as_image,
            is_steady, 
            mut render_target, 
            quad, 
            is_show,
            post_info): 
        (
            Entity,
            &ParentPassId,
            Mut<Camera>,
            Ticker<&View>,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            &IsSteady,
            Mut<RenderTarget>,
            &Quad,
            &IsShow,
            &PostProcessInfo,
        ),
        parent_dirty_rect: Aabb2,
        cur_dirty_rect:  &mut DirtyRect,
    | -> bool {

        let camera_bypass = camera.bypass_change_detection();
        let old_is_render_own = camera_bypass.is_render_own;
        camera_bypass.is_render_own = false;
        // log::debug!("camera.is_render_own = {:?};", (entity, camera_bypass.draw_changed, render_dirty1, view_port_is_dirty, as_image, &render_target.cache));
        let local_dirty_mark = cur_dirty_rect.draw_changed || render_dirty1 || view_port_is_dirty;
        // local_dirty_mark = true;
        log::debug!("change==========={:?}", (entity, &cur_dirty_rect.state, render_dirty.0, is_show.get_visibility(), is_show.get_display()));
        let dirty_state = cur_dirty_rect.state.clone();
        cur_dirty_rect.state = DirtyRectState::UnInit;
        cur_dirty_rect.draw_changed = false;
        
        if !is_show.get_visibility() || !is_show.get_display() {
            // 如果设置为隐藏，之前的渲染结果也需要释放
            // 因为， 如果将其缓存，直到重新设置为可见， 中间发生的任何改变， 都不会渲染为fbo，此时曾经缓冲的fbo的像素结果， 可能已经不正确了， 因此，直接释放掉缓存的fbo
            render_target.target = None;
            render_target.cache = RenderTargetCache::None;
            if camera_bypass.is_visible {
                camera_bypass.is_visible = false; // 设置为不可见
                camera_bypass.view_port = Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0));
                return true; // 返回true，表示需要重新批处理
            }
            // 如果曾经不可见， 现在也不可见， 则不需要重新批处理
			return false;
		}

        let overflow_aabb = &*overflow_aabb;
        let view_aabb = &overflow_aabb.view_box.aabb;
        let view_aabb_int = Aabb2::new(
            Point2::new(view_aabb.mins.x.floor(), view_aabb.mins.y.floor()),
            Point2::new(view_aabb.maxs.x.ceil(), view_aabb.maxs.y.ceil()),
        );
        let target_size_change = !(eq_f32(
            render_target.bound_box.maxs.x - render_target.bound_box.mins.x,
            view_aabb_int.maxs.x - view_aabb_int.mins.x,
        ) && eq_f32(
            render_target.bound_box.maxs.y - render_target.bound_box.mins.y,
            view_aabb_int.maxs.y - view_aabb_int.mins.y,
        ));

        fn clear_cache(render_target: &mut RenderTarget, assets: &TargetCacheMgr) {
             // 缓存大小和新的节点大小不一致， 删除缓存
            match &render_target.cache {
                RenderTargetCache::None => (),
                RenderTargetCache::Strong(droper) => {assets.0.pop_by_filter(|r| {
                    Share::as_ptr(&r.0) == Share::as_ptr(droper)
                });},
                RenderTargetCache::Weak(droper) => {assets.0.pop_by_filter(|r| {
                    Share::as_ptr(&r.0) == ShareWeak::as_ptr(droper)
                });},
            };
            render_target.target = None;
            render_target.cache = RenderTargetCache::None;
        }
        if target_size_change {
           clear_cache(&mut render_target, &assets);
        }

        let as_image = match as_image {
            Some(r) => r.level,
            None => pi_style::style::AsImage::None,
        };
        // 检查render_target的缓存情况， 如果存在缓存， 将缓存从弱引用变为强引用， 设置到rendertarget
        check_render_target(&mut render_target, as_image, is_steady.0);

        if let None = render_target.target {
            
        } else if !local_dirty_mark && dirty_state == DirtyRectState::UnInit {  
            // // 如果缓冲fbo的视口区域范围大于当前视口区域， 则不需要重新渲染 , 
            // // TODO， 如果 A->B->C形成Pass父子关系， 脏区域小于A的缓冲， 也小于C的缓冲， B从Pass变为非Pass， 可能会纹理冲突????(该顾虑可能不存在)
            // if camera_bypass.view_port.mins.x <= aabb.mins.x &&
            //     camera_bypass.view_port.mins.y <= aabb.mins.y &&
            //     camera_bypass.view_port.maxs.x >= aabb.maxs.x &&
            //     camera_bypass.view_port.maxs.y >= aabb.maxs.y {
                // 存在fbo缓存， 且当前pass不脏，则不需要渲染
                return false;
            // }
            // return old_is_render_own != camera_bypass.is_render_own;
        } 

        let mut dirty_rect = &cur_dirty_rect.value;
        if render_dirty1 || view_port_is_dirty || !post_info.has_effect() || render_target.target.is_none() {
            // 如果设置了全屏脏， 设置脏区域为父的脏区域（父又递归继承了父的脏区域， 实际上是视口区域）
            // 如果当前pass没有fbo，则也继承父的脏区域（通常， 这是一个overflow的pass， 如果不设置为父的脏区域， 节点内部的修改范围小于父的修改范围， 此时会渲染不全）
            // 节点未缓存， 设置脏区域为父的脏区域（最多只会渲染父上下文的范围）
            dirty_rect = &parent_dirty_rect;
        }

        // 为了保证根节点在其视口范围的渲染结果的正确性， 根节点的脏区域需要特殊处理
        // 如果节点的target被缓存， 则只需要重新渲染脏区域的内容
        // 如果节点的target未被缓存， 需要将整个视口范围重新渲染
        // 本方法中会先处理所有的根节点， 再处理子节点，在处理根节点时， 就将脏区域更新， 使得子节点使用正确的脏区域
        // if entity == layer.root() {
        //     // 为了保证根节点在其视口范围的渲染结果的正确性， 根节点的脏区域需要特殊处理
        //     // 如果根节点的target被缓存， 则只需要重新渲染脏区域的内容
        //     // 如果根节点的target未被缓存， 需要将整个视口范围重新渲染
        //     // 本方法中会先处理所有的根节点， 再处理子节点，在处理根节点时， 就将脏区域更新， 使得子节点使用正确的脏区域
        //     if let StrongTarget::None = &render_target.target {
        //         dirty_rect = &view_port.0;
        //         root_dirty_rect.0.value = view_port.0.clone();
        //     }
        // }

        
        let no_rotate_view_aabb = if let OverflowDesc::Rotate(oveflow_rotate) = &overflow_aabb.desc {
            // let mins = oveflow_rotate.rotate_matrix_invert * Vector4::new(aabb.mins.x, aabb.mins.y, 0.0, 1.0);
            // 脏区域变化到当前上下文的非旋转坐标系，与当前上下文的视图aabb相交，得到最终视口区域
            let rr = rotatequad_quad_intersection(
                &(
                    Vector2::new(dirty_rect.mins.x, dirty_rect.mins.y),
                    Vector2::new(dirty_rect.mins.x, dirty_rect.maxs.y),
                    Vector2::new(dirty_rect.maxs.x, dirty_rect.maxs.y),
                    Vector2::new(dirty_rect.maxs.x, dirty_rect.mins.y),
                ),
                &oveflow_rotate.world_rotate_invert,
                view_aabb,
            );

            // let r = calc_bound_box(&aabb, &oveflow_rotate.rotate_matrix_invert);
            // let rr = intersect(&overflow, &r).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0),
            // Point2::new(0.0, 0.0)));
            // log::debug!("rr=====id: {:?} \nrotate_matrix_invert: {:?}, \nview_port: {:?}, \nview_box.aabb: {:?}, \n rr: {:?}, ", entity, &oveflow_rotate.world_rotate_invert, view_port, overflow_aabb.view_box.aabb, rr);
            rr
        } else {
            intersect(view_aabb, &dirty_rect).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
        };
        let mut aabb = Aabb2::new(
            Point2::new(no_rotate_view_aabb.mins.x.floor(), no_rotate_view_aabb.mins.y.floor()),
            Point2::new(no_rotate_view_aabb.maxs.x.ceil(), no_rotate_view_aabb.maxs.y.ceil()),
        );

        
        
        // log::debug!("viewport======={:?}, \nview_aabb={:?}, \noverflow_aabb={:?}, \ndirty_rect={:?}", entity, no_rotate_view_aabb, overflow_aabb, dirty_rect);

        log::debug!("pass_id2 22========={:?}, {:?}", entity, (overflow_aabb, no_rotate_view_aabb, !is_show.get_visibility(), !is_show.get_display()));
        if no_rotate_view_aabb.mins.x >= no_rotate_view_aabb.maxs.x || no_rotate_view_aabb.mins.y >= no_rotate_view_aabb.maxs.y {
            // 如果视口为0， 则不需要渲染
            // 与 visiblity=false和display=none的区别是， 该fbo依然可见，任何改变， 都会触发缓存的fbo重新渲染， 缓存的fbo总会是最新结果， 因此不需要将fbo释放，原有缓存依然可用
            if camera_bypass.is_visible {
                camera_bypass.is_visible = false; // 设置为不可见
                camera_bypass.view_port = no_rotate_view_aabb;
                return true; // 返回true，表示需要重新批处理
            }
            return old_is_render_own != camera_bypass.is_render_own;
        }

        if post_info.is_not_only_as_image(&as_image_mark_type) {
            // 存在后处理效果，当前fbo存在脏， 则缓冲无效（缓冲的是后处理结果）， 清除缓冲
            clear_cache(&mut render_target, &assets);
        }

        // is_steady.0为true或as_image为Force， 视为强行缓冲， 强行缓冲需要缓冲整个fbo， fbo的大小应该为当前节点的可视区域的大小
        
        let is_full_fbo = match (as_image, is_steady.0) {
            (_, true) => true,
            (pi_style::style::AsImage::Force, _) => true,
            (pi_style::style::AsImage::Advise, _) => if render_target.target.is_some() {
                true // 建议缓冲， 并且有旧的缓冲（旧的缓冲一定是节点的可视区域大小）， 则设置fbo为可视区域大小
            } else {
                false // 建议缓冲， 但没有旧的缓冲区域， fbo开视口大小
            },
            _ => false
        };
        if is_full_fbo {
            // 能进入该分支， 说明节点内容fbo需要缓冲（缓冲的内容应该包含节点下的所有内容，而不仅仅是当前脏区域的内容， 因此bound_box应为节点内容大小）
            render_target.bound_box = view_aabb_int;
            if let &None = &render_target.target {
                // 如果需要强行缓冲(整个节点内容都必须缓冲下来)， 又没有旧的缓存， 则视口应该是整个节点的可视区域（重新渲染整个节点内容）
                aabb = view_aabb_int;
                // 重设了视口， 修改脏区域为视口区域， 使子pass知道， 父的pass需要渲染的最大范围（否则， 子pass如果脏范围较小， 并且子pass没有后处理， 其渲染会被裁剪到小范围）
                dirty_rect = view_aabb;
            }
            let width = view_aabb_int.maxs.x - view_aabb_int.mins.x;
            let height = view_aabb_int.maxs.y - view_aabb_int.mins.y;
            render_target.accurate_bound_box = Aabb2::new(
                Point2::new((view_aabb.mins.x - view_aabb_int.mins.x) / width, (view_aabb.mins.y - view_aabb_int.mins.y) / height),
                Point2::new((view_aabb.maxs.x - view_aabb_int.maxs.x) / width, (view_aabb.maxs.y - view_aabb_int.maxs.y) / height),
            );
		} else {
			render_target.bound_box = aabb.clone();
            let width = aabb.maxs.x - aabb.mins.x;
            let height = aabb.maxs.y - aabb.mins.y;
            render_target.accurate_bound_box = Aabb2::new(
                Point2::new((no_rotate_view_aabb.mins.x - aabb.mins.x) / width, (no_rotate_view_aabb.mins.y - aabb.mins.y) / height),
                Point2::new((no_rotate_view_aabb.maxs.x - aabb.maxs.x) / width, (no_rotate_view_aabb.maxs.y - aabb.maxs.y) / height),
            );
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
		camera_group.set_uniform(&Sdf2TextureSizeUniform(&[font_texture_size.width as f32, font_texture_size.height as f32]));

        *camera = Camera {
            // view: view_matrix.clone(),
            // project: project_matrix,
            bind_group: Some(DrawBindGroup::Offset(camera_group)),
            old_view_port: camera.view_port.clone(),
            view_port: aabb,
            draw_range: camera.draw_range.clone(),
            // world_matrix: world_matrix.clone(),
            is_render_own: true,
            is_render_to_parent: camera.is_render_to_parent,
            is_visible: true,
        };

       

        log::debug!("camera, entity: {:?}, parent_pass: {:?},  is_not_only_as_image: {:?}, view_port:{:?}, bound_box: {:?}, accurate_bound_box: {:?}, \ncur_dirty_rect:{:?}, \ndirty_rect:{:?}, \noverflow_aabb:{:?}, \nno_rotate_view_aabb: {:?}, ", 
            entity, 
            parent_pass_id.0, 
            post_info.is_not_only_as_image(&as_image_mark_type), 
            &camera.view_port,   
            &render_target.bound_box, 
            &render_target.accurate_bound_box, 
            cur_dirty_rect.value,
            dirty_rect,
            view_aabb,
            &no_rotate_view_aabb,);
        
        cur_dirty_rect.value = dirty_rect.clone(); // 更新当前脏区域， 后续子Pass会使用

        // // 删除原有缓冲（内容发生改变会重新渲染， 原有缓冲没有作用）
        // // 除了根节点和asimageurl节点
        // // asimageurl节点 TODO
        // fn is_full_screen_render(camera: &Camera, view_port: &Aabb2) -> bool {
        //     if 
        //         eq_f32(camera.view_port.mins.x, view_port.mins.x) && 
        //         eq_f32(camera.view_port.maxs.x, view_port.maxs.x) && 
        //         eq_f32(camera.view_port.mins.y, view_port.mins.y) && 
        //         eq_f32(camera.view_port.maxs.y, view_port.maxs.y) {
        //         return true;
        //     } else {
        //         return false; 
        //     }
        // }



        // let width = camera.view_port.maxs.x - camera.view_port.mins.x;
        // let height = camera.view_port.maxs.y - camera.view_port.mins.y;
        // if layer.root() == entity {
        //     // 根节点必须分配与根节点overflow_aabb等大的fbo
        //     // 因为根节点fbo要缓冲上一帧的内容，其fbo大小必须包含整个视口内容
        //     let overflow_aabb = &overflow_aabb.view_box.aabb;
        //     render_target.bound_box = Aabb2::new(
        //         Point2::new(overflow_aabb.mins.x.floor(), overflow_aabb.mins.y.floor()),
        //         Point2::new(overflow_aabb.maxs.x.ceil(), overflow_aabb.maxs.y.ceil()),
        //     );
        // } else {
        //     // 非根节点，在没有旧的fbo的情况下，只需要开与渲染区域等大的fbo
        //     render_target.bound_box = camera.view_port.clone();
        //     render_target.accurate_bound_box = Aabb2::new(
        //         Point2::new((no_rotate_view_aabb.mins.x - camera.view_port.mins.x) / width, (no_rotate_view_aabb.mins.y - camera.view_port.mins.y) / height),
        //         Point2::new((no_rotate_view_aabb.maxs.x - camera.view_port.maxs.x) / width, (no_rotate_view_aabb.maxs.y - camera.view_port.maxs.y) / height),
        //     );
        // }

        
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

        return old_is_render_own != camera.is_render_own;
    };
    // 当视口发生变化时， 需要遍历所有的pass2d重新计算相机
    // 计算时， 从跟节点开始遍历
    for r in instance_context.pass_toop_list.iter().rev() {
        if let Ok(r) = query_pass.get_mut(*r) {
            let parent_dirty_rect = match query_dirty_rect.get(r.1.0) {
                Ok(r) => r.value.clone(),
                Err(_) => match query_root.get_mut(r.5.root()) {
                    Ok(r) => r.0.clone(),
                    Err(_) => continue,
                },
            };
            let mut dirty_rect = query_dirty_rect.get_mut(r.0).unwrap();
            is_render_own_changed |= calc_camera(r, parent_dirty_rect, dirty_rect.bypass_change_detection());
        }
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


pub fn check_render_target(render_target: &mut RenderTarget, as_image: pi_style::style::AsImage, is_steady: bool) {
    match (as_image, is_steady) {
        (pi_style::style::AsImage::None, false) => {
            // 设置render_target.cache为none，在渲染时动态分配rendertarget
            render_target.cache = RenderTargetCache::None;
        }
        (pi_style::style::AsImage::Advise, _) | (_, true) => {
            match &render_target.cache {
                RenderTargetCache::None => return,
                RenderTargetCache::Strong(r) => {
                    render_target.target = Some(r.clone());
                    // 缓存修改为弱引用
                    let weak = Share::downgrade(r);
                    render_target.cache = RenderTargetCache::Weak(weak);
                }
                RenderTargetCache::Weak(r) => {
                    match ShareWeak::upgrade(r) {
                        Some(r) => {
                            // 弱引用升级成功，返回强引用，如果相机被激活，外部应该将其放在render_target.target上， 避免在渲染时， 该弱引用对应的值已被销毁
                            render_target.target = Some(r.clone());
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
                    render_target.target = Some(r.clone());
                }
                RenderTargetCache::Weak(r) => {
                    match ShareWeak::upgrade(r) {
                        Some(r) => {
                            render_target.target = Some(r.clone());
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
    mut instance_context: OrInitSingleResMut<InstanceContext>,
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
    let instance_context = &mut **instance_context;

    let q1: &Query<(&mut Camera, &PostProcessInfo, &ParentPassId, &GraphId)> = unsafe {transmute(&query)};
    // log::debug!("calc_pass_active======{:?}", (instance_context.pass_toop_list.len(), render_dirty.1, render_dirty.2));
    for node in instance_context.pass_toop_list.iter().rev() {
        if let Ok((mut camera, _post_info, mut parent_pass, graph_id)) = query.get_mut(*node) {
            let old_is_render_to_parent = camera.is_render_to_parent;
            let camera = camera.bypass_change_detection();
            if parent_pass.0.is_null() {
                camera.is_render_to_parent = true;
            } else if !camera.is_visible {
                camera.is_render_to_parent = false;
            } else {
                while let Ok((c1, p1, pp1, _)) = q1.get(parent_pass.0) {
                    if !p1.has_effect() {
                        parent_pass = pp1;
                        continue;
                    }

                    camera.is_render_to_parent = c1.is_render_own;
                    // log::debug!("set======{:?}", (node, parent_pass.0.0, graph_id.0, camera.is_render_to_parent, camera.is_render_own, c1.is_render_own, ));
                    camera.is_render_own = camera.is_render_own && c1.is_render_own;
                    // if !camera.is_render_own {
                    //     log::debug!("camera.is_render_own= false================={:?}", (node, parent_pass.0.0, camera.is_render_own, c1.is_render_own));
                    // }
                    break;
                    
                } 
            }
            // log::debug!("set_enable======{:?}", (node, graph_id.0, camera.is_render_to_parent, camera.is_render_own ));
            if old_is_render_to_parent != camera.is_render_to_parent {
                log::debug!("set_enable======{:?}", (node, graph_id.0, camera.is_render_to_parent));
                let _ = rg.set_is_build(graph_id.0, camera.is_render_to_parent);
                instance_context.rebatch = true;
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


// #[inline]
// fn mark_pass_dirty(mut pass_id: Entity, query: &mut Query<(&mut Camera, &ParentPassId)>,) {
//     while let Ok((mut camera, parent_pass)) = query.get_mut(pass_id) {
//         if camera.draw_changed {
//             break;
//         }

//         camera.bypass_change_detection().draw_changed = true;
//         pass_id = parent_pass.0;
//     }
// }
