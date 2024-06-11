//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use pi_world::{filter::Or, prelude::{Changed, Entity, Mut, ParamSet, Query, SingleRes, Ticker}, single_res::SingleResMut};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, Layer};

use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_share::{Share, ShareWeak};

use crate::{
    components::{
        calc::{
            IsShow, OverflowDesc, Quad, TransformWillChangeMatrix, View, WorldMatrix,
        },
        pass_2d::{
            Camera, DirtyMark, RenderTarget, RenderTargetCache, StrongTarget
        },
        user::{Aabb2, AsImage, Point2, Vector2, Viewport},
    },
    resource::{
        draw_obj::InstanceContext,
        ShareFontSheet,
    },
    shader1::meterial::{ProjectUniform, Sdf2TextureSizeUniform, ViewUniform},
    system::{draw_obj::calc_text::IsRun, utils::{create_project, rotatequad_quad_intersection}},
    utils::tools::{box_aabb, calc_bound_box, eq_f32, intersect},
};

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera_depth_and_renderlist(
    mut query_pass: ParamSet<(
        Query<
            (
                Entity,
                &mut Camera,
                &mut DirtyMark, // 本地脏区域
                &View,
                &TransformWillChangeMatrix,
                &Layer,
                Option<&AsImage>,
                &mut RenderTarget,
                &Quad,
                &IsShow,
            ),
        >,
        Query<
        (
            Entity,
            &mut Camera,
            &mut DirtyMark, // 本地脏区域
            &View,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            &mut RenderTarget,
            &Quad,
            &IsShow,
        ),
        Or<(Changed<View>, Changed<AsImage>, Changed<IsShow>)>
    >)>,
    // mut query_root: ParamSet<(Query<(&RootDirtyRect, OrDefault<RenderDirty>, Ref<Viewport>)>, Query<&mut RenderDirty>)>,
    query_root: Query<Ticker<&Viewport>>,
    font_sheet: SingleRes<ShareFontSheet>,
	instance_context: SingleResMut<InstanceContext>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    // let p0 = query_root.p0();
	let font_sheet = font_sheet.0.borrow();
	let font_texture_size = font_sheet.texture_size();
	let font_type =  font_sheet.font_mgr().font_type;

	// 所有根共同的脏区域
    let mut all_dirty_rect = Aabb2::new(Point2::new(std::f32::MAX, std::f32::MAX), Point2::new(std::f32::MIN, std::f32::MIN));

    let mut view_port_is_dirty = false;
    // 迭代根节点，得到最大脏包围盒
    for view_port in query_root.iter() {
        if view_port.is_changed() {
            view_port_is_dirty = true;
        }
        box_aabb(&mut all_dirty_rect, &view_port.0);
    }
    
    let calc_camera = |
        (
            entity, 
            mut camera, 
            mut local_dirty, 
            overflow_aabb, 
            willchange_matrix, 
            layer, 
            as_image, 
            mut render_target, 
            quad, 
            is_show): 
        (
            Entity,
            Mut<Camera>,
            Mut<DirtyMark>, // 本地脏区域
            &View,
            &TransformWillChangeMatrix,
            &Layer,
            Option<&AsImage>,
            Mut<RenderTarget>,
            &Quad,
            &IsShow,
        )
    | {
        camera.is_active = false;

		if !is_show.get_visibility() || !is_show.get_display() {
			return;
		}

        let local_dirty_mark = local_dirty.0;
        local_dirty.0 = false;

        let dirty_rect = match query_root.get(layer.root()) {
            Ok(r) => r,
            Err(_) => return,
        };

        // 检查render_target的缓存情况， 设置rendertarget
        check_render_target(&mut render_target, as_image);


        // log::warn!("pass_id1========={:?}, {:?}", dirty_rect, willchange_matrix);
        // 计算视图区域（坐标系为本节点的非旋转坐标系）
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
                &overflow_aabb.view_box.aabb,
            );

            // let r = calc_bound_box(&aabb, &oveflow_rotate.rotate_matrix_invert);
            // let rr = intersect(&overflow, &r).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0),
            // Point2::new(0.0, 0.0)));
            // log::warn!("rr=====id: {:?} \nrotate_matrix_invert: {:?}, \ndirty_rect: {:?}, \nview_box.aabb: {:?}, \n rr: {:?}, ", entity, &oveflow_rotate.world_rotate_invert, dirty_rect, overflow_aabb.view_box.aabb, rr);
            rr
        } else {
            intersect(&overflow_aabb.view_box.aabb, &dirty_rect).unwrap_or(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)))
        };

        // log::warn!("viewport======={:?}, \nview_aabb={:?}, \noverflow_aabb={:?}, \ndirty_rect={:?}", entity, no_rotate_view_aabb, overflow_aabb, dirty_rect);

        log::trace!("pass_id2 22========={:?}, {:?}", entity, (&*dirty_rect, overflow_aabb, !is_show.get_visibility(), !is_show.get_display()));
        if no_rotate_view_aabb.mins.x >= no_rotate_view_aabb.maxs.x || no_rotate_view_aabb.mins.y >= no_rotate_view_aabb.maxs.y {
            return;
        }

        // 计算视图区域（世界坐标系）
        let aabb_temp;
        let view_world_aabb = match &overflow_aabb.desc {
            OverflowDesc::Rotate(r) => {
                aabb_temp = calc_bound_box(&no_rotate_view_aabb, &r.world_rotate);
                &aabb_temp
            }
            _ => &no_rotate_view_aabb,
        };

		// log::warn!("last_dirty======{:?}, {:?}", entity, cull_aabb);
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

        // log::warn!("pass_id2=========\nentity: {:?}, \nproject_matrix: {:?}, \nview_matrix: {}, \nwillchange_matrix:{:?} \naabb:{:?}, \noverflow_aabb: {:?}", entity, project_matrix, view_matrix, willchange_matrix, aabb, overflow_aabb);

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
		camera_group.set_uniform(&Sdf2TextureSizeUniform(&[font_texture_size.width as f32, font_texture_size.height as f32, font_texture_size.width as f32, font_texture_size.height as f32]));

        *camera = Camera {
            // view: view_matrix.clone(),
            // project: project_matrix,
            bind_group: Some(DrawBindGroup::Offset(camera_group)),
            view_port: aabb,
            // world_matrix: world_matrix.clone(),
            is_active: true,
            is_change: true,
        };

        if let &StrongTarget::None = &render_target.target {
			// if bg.is_some() {
			// 	log::warn!("aaaa1================={:?}, {:?}, {:?}, {}", entity, render_target.bound_box, &camera.view_port, render_target.bound_box.maxs.x - render_target.bound_box.mins.x);
			// 	log::warn!("aaaa0================={:?}, {:?}, {:?}, {}", entity, dirty_rect, );
			// }
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
			}
		} else {
			// 能进入该分支， 说明节点内容fbo需要强制缓冲（强制缓冲的内容应该包含节点下的所有内容，而不仅仅是当前脏区域的内容， 因此bound_box应为节点内容大小）
			let overflow_aabb = &overflow_aabb.view_box.aabb;
			let overflow_aabb = Aabb2::new(
				Point2::new(overflow_aabb.mins.x.floor(), overflow_aabb.mins.y.floor()),
				Point2::new(overflow_aabb.maxs.x.ceil(), overflow_aabb.maxs.y.ceil()),
			);
			// log::warn!("target_size_change========{:?}, {:?}, {:?}, {:?}", entity, &render_target.bound_box, overflow_aabb.view_box.aabb.clone(), &camera.view_port);
			

			
            let target_size_change = !(eq_f32(
                render_target.bound_box.maxs.x - render_target.bound_box.mins.x,
                overflow_aabb.maxs.x - overflow_aabb.mins.x,
            ) && eq_f32(
                render_target.bound_box.maxs.y - render_target.bound_box.mins.y,
                overflow_aabb.maxs.y - overflow_aabb.mins.y,
            ));

			render_target.bound_box = overflow_aabb; 

            if target_size_change {
				
				// if bg.is_some() {
				// 	log::warn!("aaaa2================={:?}, {:?}, {:?}, {}", entity, render_target.bound_box, &camera.view_port, render_target.bound_box.maxs.x - render_target.bound_box.mins.x);
				// }
                // 从资源管理器中删除原有的渲染目标（TODO， 另外还需要在RenderTarget销毁时， 从资源管理器中删除）
                render_target.target = StrongTarget::None; // 设置为None， 等待渲染时重新分配
                render_target.cache = RenderTargetCache::None;
            }
            // 如果本地脏区域面积为0，并且渲染目标尺寸未改变， 则该camera下的物体不需要改变
            if !local_dirty_mark && !target_size_change {
                camera.is_change = false;
            }
        }
    };
    if view_port_is_dirty {
        // 当视口发生变化时， 需要遍历所有的pass2d从新计算相机
        for r in query_pass.p0().iter_mut() {
            calc_camera(r);
        }
    } else {
        // 当视口未发生改变时， 只需要冲洗计算脏了的pass
        for r in query_pass.p1().iter_mut() {
            calc_camera(r);
        }
    }
    
}

pub fn check_render_target(render_target: &mut RenderTarget, as_image: Option<&AsImage>) {
    match as_image {
        Some(as_image) => match as_image.level {
            pi_style::style::AsImage::None => {
                // 设置render_target.cache为none，在渲染时动态分配rendertarget
                render_target.cache = RenderTargetCache::None;
            }
            pi_style::style::AsImage::Advise => {
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
            pi_style::style::AsImage::Force => {
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
        },
        None => {
            // 设置render_target.cache为none，在渲染时动态分配rendertarget
            render_target.cache = RenderTargetCache::None;
        }
    }
}
