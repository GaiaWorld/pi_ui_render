//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use bevy_ecs::{
    system::{ParamSet, Query, Res, ResMut},
    prelude::{DetectChanges, DetectChangesMut, Ref, With, Without, Entity},
};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::{
    prelude::{Layer, OrDefault},
    system_param::res::{OrInitRes, OrInitResMut},
};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{PiIndexBufferAlloter, PiRenderDevice, PiRenderQueue, PiVertexBufferAlloter};
use pi_render::{
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer},
};
use pi_share::{Share, ShareWeak};
use pi_spatial::quad_helper::intersects;

use crate::{
    components::{
        calc::{
            DrawInfo, DrawList, EntityKey, InPassId, IsShow, OverflowDesc, Quad, RootDirtyRect, TransformWillChangeMatrix, View, WorldMatrix, ZRange,
        },
        draw_obj::DrawState,
        pass_2d::{
            Camera, DirtyMark, DirtyRectState, Draw2DList, DrawIndex, LastDirtyRect, ParentPassId, PostProcess, PostProcessInfo, RenderTarget,
            RenderTargetCache, ViewMatrix, StrongTarget,
        },
        user::{Aabb2, AsImage, Matrix4, Point2, RenderDirty, Vector2, Viewport, BackgroundImage},
    },
    resource::{
        draw_obj::{CameraGroup, DepthCache, GroupAlloterCenter, ShareGroupAlloter, ShareLayout},
        QuadTree,
    },
    shader::camera::{ProjectUniform, ViewUniform},
    system::{utils::{create_project, rotatequad_quad_intersection}, draw_obj::calc_text::IsRun},
    utils::tools::{box_aabb, calc_bound_box, eq_f32, intersect},
};

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera_depth_and_renderlist(
    // query_draw2d_list: Query<&'static mut Draw2DList>,
    mut query_pass: ParamSet<(
        Query<
            (
                Entity,
                &mut Camera,
                &mut ViewMatrix,
                &mut LastDirtyRect,
                &mut DirtyMark, // 本地脏区域
                Option<&mut PostProcess>,
                &mut PostProcessInfo,
                &View,
                &TransformWillChangeMatrix,
                &Layer,
                Option<&AsImage>,
                &mut RenderTarget,
				Option<&BackgroundImage>,
				&Quad,
            ),
            Without<DrawState>,
        >,
        (Query<(&LastDirtyRect, &Camera)>, Query<(&'static mut Draw2DList, &Camera)>),
        (Query<(&Camera, &View)>, Query<&'static mut PostProcessInfo>),
		Query<(&'static mut Draw2DList, Entity, &Camera)>
    )>,
    node_query: Query<(
		Option<&'static ParentPassId>,
        &'static InPassId,
        &'static DrawList,
        &'static Quad,
        &'static ZRange,
        &'static IsShow,
        Entity,
    )>,
    draw_obj_post_query: Query<(), (With<PostProcess>, With<DrawState>)>,
    mut query_root: ParamSet<(Query<(&RootDirtyRect, OrDefault<RenderDirty>, Ref<Viewport>)>, Query<&mut RenderDirty>)>,
    draw_state: Query<&'static mut DrawState>,
    draw_info: Query<&'static DrawInfo>,

    res: (
        Res<ShareLayout>,
        Res<PiRenderDevice>,
        Res<PiRenderQueue>,
        Res<ShareAssetMgr<RenderRes<Buffer>>>,
        Res<ShareAssetMgr<RenderRes<BindGroup>>>,
        Res<GroupAlloterCenter>,
        OrInitRes<PiVertexBufferAlloter>,
        OrInitRes<PiIndexBufferAlloter>,
        Res<QuadTree>,
        // Res<NotDrawListMark>,
    ),
    depth_cache: OrInitResMut<DepthCache>,
    camera_material_alloter: OrInitRes<ShareGroupAlloter<CameraGroup>>,

    post_resource: ResMut<PostprocessResource>,
    // mut geometrys: ResMut<PiPostProcessGeometryManager>,
    // mut postprocess_pipelines: ResMut<PiPostProcessMaterialMgr>,
	r: OrInitResMut<IsRun>
) {
	if r.0 {
		return;
	}
    let (share_layout, device, queue, buffer_assets, bind_group_assets, group_alloc_center, vertbuffer_alloter, index_alloter, quad_tree) = res;
    let p0 = query_root.p0();
	// 所有根共同的脏区域
    let mut all_dirty_rect = Aabb2::new(Point2::new(std::f32::MAX, std::f32::MAX), Point2::new(std::f32::MIN, std::f32::MIN));

    // 迭代根节点，得到最大脏包围盒
    for (global_dirty_rect, render_dirty_mark, view_port) in p0.iter() {
        if global_dirty_rect.state != DirtyRectState::UnInit {
            box_aabb(&mut all_dirty_rect, &global_dirty_rect.value);
        } else if render_dirty_mark.0 {
			box_aabb(&mut all_dirty_rect, &view_port.0);
		}
    }

    for (
        entity,
        mut camera,
        _view_matrix,
        mut last_dirty,
        mut local_dirty,
        postprocess_list,
        mut post_info,
        overflow_aabb,
        willchange_matrix,
        layer,
        as_image,
        mut render_target,
		bg,
		quad,
    ) in query_pass.p0().iter_mut()
    {
        camera.is_active = false;

        let mut local_dirty_mark = local_dirty.0;
        local_dirty.0 = false;

        let (global_dirty_rect, render_dirty_mark, viewport) = match p0.get(layer.root()) {
            Ok(r) => {
                if r.2.is_changed() {
                    local_dirty_mark = true; // root的视口脏了， 需要全部重新渲染
                }
                r
            }
            _ => continue,
        };
		// log::warn!("local_dirty_mark============{:?}, {:?}, {:?}, {:?}, {:?}", entity, local_dirty_mark, as_image, global_dirty_rect, quad);

        // 不脏，不需要组织渲染图， 也不需要渲染脏
        if global_dirty_rect.state == DirtyRectState::UnInit && !render_dirty_mark.0 {
            continue;
        }

        // 检查render_target的缓存情况， 设置rendertarget
        check_render_target(&mut render_target, as_image);

        // 如果render_dirty_mark.0, 表示全屏脏
        let mut dirty_rect = global_dirty_rect.value.clone();
		// 如果该pass2d是根节点， 则其脏区域始终为视口区域
        if render_dirty_mark.0 { 
            dirty_rect = viewport.0;
        }


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


        if no_rotate_view_aabb.mins.x >= no_rotate_view_aabb.maxs.x || no_rotate_view_aabb.mins.y >= no_rotate_view_aabb.maxs.y {
            continue;
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

        // 计算用于剔除的aabb
        // 剔除使用quad和节点的脏区域判断相交来剔除， 注意坐标系都是世界坐标系
        // 如果存在transformwillchange，则需要算上脏区域
        // no_will_change用于包围盒剔除渲染对象（渲染对象使用quad来剔除，quad是没有willchange_matrix的参与的）
        let cull_aabb = if let Some(r) = &willchange_matrix.0 {
            // log::warn!("cull_aabb willchange_matrix======================{:?}, {:?}", entity, willchange_matrix);
            if post_info.has_effect() {
                let mm;
                let m = match &overflow_aabb.desc {
                    OverflowDesc::Rotate(rotate) => {
                        mm = &r.will_change_invert.0 * &rotate.world_rotate;
                        &mm
                    }
                    _ => &r.will_change_invert,
                };
                let r = calc_bound_box(&no_rotate_view_aabb, m);
                r
            } else {
                view_world_aabb.clone()
            }
        } else {
            view_world_aabb.clone()
        };
        // log::warn!("cull_aabb======{:?}, {:?}, {:?}", entity, cull_aabb, no_rotate_view_aabb);
        *last_dirty = LastDirtyRect {
            // last: view_aabb.clone(),
            no_will_change: cull_aabb,
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

        let mut camera_group = camera_material_alloter.alloc();
        camera_group.set_uniform(&ProjectUniform(project_matrix.as_slice()));
        camera_group.set_uniform(&ViewUniform(view_matrix.as_slice()));


        let scale_x = (aabb.maxs.x - aabb.mins.x) / 2.0;
        let scale_y = (aabb.maxs.y - aabb.mins.y) / 2.0;
        // 后处理效果与gui坐标系使用不一致，所以缩放为-scale_y
        // 这里的aabb是指当前非旋转坐标系
        let world_matrix = Matrix4::new(
            scale_x,
            0.0,
            0.0,
            aabb.mins.x + scale_x,
            0.0,
            -scale_y,
            0.0,
            aabb.mins.y + scale_y,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        *camera = Camera {
            view: view_matrix.clone(),
            project: project_matrix,
            bind_group: Some(DrawBindGroup::Offset(camera_group)),
            view_port: aabb,
            world_matrix: world_matrix.clone(),
            is_active: true,
            is_change: true,
        };

		// log::warn!("pass==================={:?}, {:?}, {:?}, {:?}", entity, global_dirty_rect, overflow_aabb, aabb);
        // 一些不需要后处理的，可以不用计算view_port和matrix， TODO
        let post_info = post_info.bypass_change_detection();
        post_info.view_port = aabb;
        // 存在旋转，需要旋转回父上下文
        if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
            post_info.matrix = WorldMatrix(&matrix.from_context_rotate * world_matrix, true);
        } else {
			// if bg.is_some() {
			// 	log::warn!("aaaa================={:?}, {:?}, {:?}", entity, aabb, world_matrix);
			// }
            post_info.matrix = WorldMatrix(world_matrix, false);
        }

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
    }

    // 没有渲染脏
    if (all_dirty_rect.maxs.x - all_dirty_rect.mins.x) <= 0.0 || (all_dirty_rect.maxs.y - all_dirty_rect.mins.y) <= 0.0 {
        return;
    }

    // 组织渲染列表
	let p1 = query_pass.p1();
    let mut args = AbQueryArgs {
        node_query,
        pass_query: p1.0,
        draw_list: p1.1,
        draw_info,
        post_process: draw_obj_post_query,
		is_run: r,
    };

	ab_query_func(&mut args);
    // quad_tree.query(&all_dirty_rect, intersects, &mut args, ab_query_func);

	// log::warn!("all_dirty_rect====={:?}", all_dirty_rect);

    // // 遍历所有的pass，设置不透明渲染列表和透明渲染列表
    // for mut list in args.draw_list.iter_mut() {
    //     let list = &mut list;
    //     list.opaque.clear();
    //     list.transparent.clear();
    //     if list.all_list.len() == 0 {
    //         continue;
    //     }

    //     // 按深度从小到大排序
    //     list.all_list.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
    //         if a_z_depth.start < b_z_depth.start {
    //             std::cmp::Ordering::Less
    //         } else if a_z_depth.start > b_z_depth.start {
    //             std::cmp::Ordering::Greater
    //         } else {
    //             if a_sort.order() < b_sort.order() {
    //                 std::cmp::Ordering::Less
    //             } else if a_sort.order() > b_sort.order() {
    //                 std::cmp::Ordering::Greater
    //             } else {
    //                 std::cmp::Ordering::Equal
    //             }
    //         }
    //         // 用渲染管线排序，TODO
    //         // draw_state.get(a)
    //     });

    //     for i in 0..list.all_list.len() {
    //         let (entity, _, draw_info) = list.all_list[i];
    //         // 暂时放入不透明列表，TODO
    //         if draw_info.is_opacity() {
    //             list.opaque.push((entity, 0));
    //         } else {
    //             list.transparent.push((entity, 0));
    //         }
    //     }
    // }

	// for (list, entity, camera) in query_pass.p3().iter() {
	// 	log::warn!("list======={:?}, {:?}, {:?}", entity, list.all_list, camera);
	// }

    // 重置渲染脏
    for mut i in query_root.p1().iter_mut() {
        **i = false;
    }
}

pub struct AbQueryArgs<'s, 'a> {
    node_query: Query<'a, 'a, (Option<&'s ParentPassId>, &'s InPassId, &'s DrawList, &'s Quad, &'s ZRange, &'s IsShow, Entity)>,
    pass_query: Query<'a, 'a, (&'s LastDirtyRect, &'s Camera)>,
    draw_list: Query<'a, 'a, (&'s mut Draw2DList, &'s Camera)>,
    post_process: Query<'a, 'a, (), (With<PostProcess>, With<DrawState>)>,
    draw_info: Query<'a, 'a, &'s DrawInfo>,
	is_run: OrInitResMut<'a, IsRun>,
}

/// 不能通过四叉树命中， 因为可能存在transformwillchange， 此时包围是不正确的， 不能正确查询
fn ab_query_func(arg: &mut AbQueryArgs) {
	if arg.is_run.0 {
		return;
	}
	for (parent_pass_id, in_pass_id, draw_list, quad, z_range, is_show, id) in arg.node_query.iter() {
		let (context_dirty, camera) = match arg.pass_query.get(***in_pass_id) {
            Ok(r) => r,
            _ => return,
        };

        // log::trace!(target: format!("entity_{:?}", id.0).as_str(), "try collect render all_list, is_show: {:?}, quad: {:?}, context_dirty: {:?}, intersects={:?}", is_show.get_visibility(), quad, context_dirty.no_will_change, intersects(quad, &context_dirty.no_will_change));
        // log::trace!(
        //     "try collect render all_list, entity: {:?}, is_show: {:?}, quad: {:?}, context_dirty: {:?}, is_change:{:?}, is_active: {:?}",
        //     id.0,
        //     is_show.get_visibility(),
        //     quad,
        //     context_dirty.no_will_change,
		// 	camera.is_change,
		// 	camera.is_active,
        // );
        // log::warn!("draw_list2==================id: {:?}, {:?}, {:?}, quad: {:?}", id, in_pass_id, draw_list, quad);
        // global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
        if draw_list.len() > 0 && camera.is_change && is_show.get_visibility() {
            // log::warn!("ab_query_func======={:?}, {:?}, {:?}, {:?}, {:?}", id, in_pass_id, quad, context_dirty.no_will_change, intersects(quad, &context_dirty.no_will_change));
            if intersects(quad, &context_dirty.no_will_change) {
                let (mut list, _) = arg.draw_list.get_mut(***in_pass_id).unwrap();
                let list = &mut list;
                for draw_id in draw_list.iter() {
                    if let Ok(_) = arg.post_process.get(draw_id.id) {
                        list.single_list.push(DrawIndex::DrawObj(EntityKey(draw_id.id)));
                        list.all_list.push((
                            DrawIndex::DrawObjPost(EntityKey(draw_id.id)),
                            z_range.clone(),
                            *arg.draw_info.get(draw_id.id).unwrap(),
                        ));
                    } else {
						if arg.draw_info.get(draw_id.id).is_err() {
							log::warn!("is_err================={:?}, {:?}, {:?}", draw_id, id, draw_list);
							arg.is_run.0 = true;
							return;
						}
                        list.all_list.push((
                            DrawIndex::DrawObj(EntityKey(draw_id.id)),
                            z_range.clone(),
                            *arg.draw_info.get(draw_id.id).unwrap(),
                        ));
                    }
                }
            } else {
                // log::warn!("cull======{:?}, {:?}, is_show: {:?}", quad, &context_dirty.no_will_change, is_show.get_visibility());
            }
        }
        // parent_pass_id存在，表示本节点是一个pass2d
        if camera.is_active {
            if let Some(parent) = parent_pass_id {
				
                if let Ok((mut p, p_camera)) = arg.draw_list.get_mut(*parent.0) {
					if p_camera.is_active && p_camera.is_change {
						p.all_list
                        .push((DrawIndex::Pass2D(EntityKey(id)), z_range.clone(), DrawInfo::new(10, false)));
					}
                }
            }
        }
	}
}


// fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, _aabb: &Aabb2, _bind: &()) {
// 	if arg.is_run.0 {
// 		return;
// 	}
//     // quad_tree.
//     if let Ok((parent_pass_id, in_pass_id, draw_list, quad, z_range, is_show, entity)) = arg.node_query.get(*id) {
//         // log::warn!("draw_list1==================entity: {:?}, draw_list: {:?}, {}, {:?}", entity, draw_list, is_show.get_visibility(), quad, );
//         let (context_dirty, camera) = match arg.pass_query.get(***in_pass_id) {
//             Ok(r) => r,
//             _ => return,
//         };

//         // log::trace!(target: format!("entity_{:?}", id.0).as_str(), "try collect render all_list, is_show: {:?}, quad: {:?}, context_dirty: {:?}, intersects={:?}", is_show.get_visibility(), quad, context_dirty.no_will_change, intersects(quad, &context_dirty.no_will_change));
//         // log::trace!(
//         //     "try collect render all_list, entity: {:?}, is_show: {:?}, quad: {:?}, context_dirty: {:?}, is_change:{:?}, is_active: {:?}",
//         //     id.0,
//         //     is_show.get_visibility(),
//         //     quad,
//         //     context_dirty.no_will_change,
// 		// 	camera.is_change,
// 		// 	camera.is_active,
//         // );
//         // log::warn!("draw_list2==================id: {:?}, {:?}, {:?}, quad: {:?}", id, in_pass_id, draw_list, quad);
//         // global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
//         if draw_list.len() > 0 && camera.is_change && is_show.get_visibility() {
//             // log::warn!("ab_query_func======={:?}, {:?}, {:?}, {:?}, {:?}", id, in_pass_id, quad, context_dirty.no_will_change, intersects(quad, &context_dirty.no_will_change));
//             if intersects(quad, &context_dirty.no_will_change) {
//                 let (mut list, _) = arg.draw_list.get_mut(***in_pass_id).unwrap();
//                 let list = &mut list;
//                 for draw_id in draw_list.iter() {
//                     if let Ok(_) = arg.post_process.get(draw_id.id) {
//                         list.single_list.push(DrawIndex::DrawObj(EntityKey(draw_id.id)));
//                         list.all_list.push((
//                             DrawIndex::DrawObjPost(EntityKey(draw_id.id)),
//                             z_range.clone(),
//                             *arg.draw_info.get(draw_id.id).unwrap(),
//                         ));
//                     } else {
// 						if arg.draw_info.get(draw_id.id).is_err() {
// 							log::warn!("is_err================={:?}, {:?}, {:?}", draw_id, id, draw_list);
// 							arg.is_run.0 = true;
// 							return;
// 						}
//                         list.all_list.push((
//                             DrawIndex::DrawObj(EntityKey(draw_id.id)),
//                             z_range.clone(),
//                             *arg.draw_info.get(draw_id.id).unwrap(),
//                         ));
//                     }
//                 }
//             } else {
//                 // log::warn!("cull======{:?}, {:?}, is_show: {:?}", quad, &context_dirty.no_will_change, is_show.get_visibility());
//             }
//         }
// 		log::warn!("parent============{:?}, {:?}, {:?}, {:?}",id, parent_pass_id, camera.is_active, quad);
//         // parent_pass_id存在，表示本节点是一个pass2d
//         if camera.is_active {
//             if let Some(parent) = parent_pass_id {
				
//                 if let Ok((mut p, p_camera)) = arg.draw_list.get_mut(*parent.0) {
// 					if p_camera.is_active && p_camera.is_change {
// 						p.all_list
//                         .push((DrawIndex::Pass2D(EntityKey(entity)), z_range.clone(), DrawInfo::new(10, false)));
// 					}
//                 }
//             }
//         }
//     }
// }

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
