//! 处理渲染相关逻辑
//! 1. canvas修改后，添加正确的图依赖关系（修改后不知道原值，无法移除原值的依赖关系，需要原值对应的图节点正确的销毁，如果为销毁，仍然存在依赖关系，如何正确处理？TODO）
//! 2. canvas删除后，移除依赖关系
//! 3. 为后续渲染准备正确的Camera数据
//! 4. 为pass2D创建对应的图节点，并添加依赖关系
//! 5. 为删除的pass2D删除图节点，并建立正确的依赖关系

use bevy::{
    ecs::{
        prelude::Entity,
        system::{ParamSet, Query, Res, ResMut},
    },
    prelude::{With, Without},
};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::{
    prelude::{Layer, OrDefault},
    system_param::res::{OrInitRes, OrInitResMut},
};
use pi_bevy_post_process::PostprocessResource;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, PiVertexBufferAlloter, PiIndexBufferAlloter};
use pi_render::{
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup, buffer::Buffer},
};
use pi_sparialtree::quad_helper::intersects;

use crate::{
    components::{
        calc::{DrawInfo, DrawList, EntityKey, InPassId, IsShow, Quad, RootDirtyRect, TransformWillChangeMatrix, View, WorldMatrix, ZRange, OverflowDesc},
        draw_obj::DrawState,
        pass_2d::{Camera, DirtyRectState, Draw2DList, DrawIndex, LastDirtyRect, ParentPassId, PostProcess, PostProcessInfo, ViewMatrix},
        user::{Aabb2, Matrix4, Point2, RenderDirty, Vector2, Viewport},
    },
    resource::{
        draw_obj::{CameraGroup, DepthCache, GroupAlloterCenter, ShareGroupAlloter, ShareLayout},
        QuadTree, 
    },
    shader::camera::{ProjectUniform, ViewUniform},
    system::utils::{rotatequad_quad_intersection, create_project},
    utils::tools::{box_aabb, calc_aabb, intersect},
};

#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn calc_camera_depth_and_renderlist(
    query_draw2d_list: Query<&'static mut Draw2DList>,
    mut query_pass: ParamSet<(
        Query<(
            Entity,
            &mut Camera,
            &mut ViewMatrix,
            &mut LastDirtyRect,
            Option<&mut PostProcess>,
			&mut PostProcessInfo,
            &View,
            &TransformWillChangeMatrix,
            &Layer,
        ), Without<DrawState>>,
        Query<(Option<&ParentPassId>, &LastDirtyRect, &Camera)>,
        (Query<(&Camera, &View)>, Query<&'static mut PostProcessInfo>),
    )>,
    node_query: Query<(
        &'static InPassId,
        &'static DrawList,
        &'static Quad,
        &'static ZRange,
        &'static IsShow,
        Entity,
    )>,
	draw_obj_post_query: Query<(), (With<PostProcess>, With<DrawState>)>,
    mut query_root: ParamSet<(Query<(&RootDirtyRect, OrDefault<RenderDirty>, &Viewport)>, Query<&mut RenderDirty>)>,
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
) {
    let (share_layout, device, queue, buffer_assets, bind_group_assets, group_alloc_center, vertbuffer_alloter, index_alloter, quad_tree) = res;
    let p0: Query<(&RootDirtyRect, OrDefault<RenderDirty>, &Viewport)> = query_root.p0();
    let mut all_dirty_rect = Aabb2::new(Point2::new(std::f32::MAX, std::f32::MAX), Point2::new(std::f32::MIN, std::f32::MIN));

    // 迭代根节点，得到最大脏包围盒
    for (global_dirty_rect, render_dirty_mark, _) in p0.iter() {
        if render_dirty_mark.0 || global_dirty_rect.state != DirtyRectState::UnInit {
            box_aabb(&mut all_dirty_rect, &global_dirty_rect.value);
        }
    }

    for (entity, mut camera, _view_matrix, mut last_dirty, postprocess_list, mut post_info, overflow_aabb, willchange_matrix, layer) in
        query_pass.p0().iter_mut()
    {
        camera.is_active = false;

        let (global_dirty_rect, render_dirty_mark, viewport) = match p0.get(layer.root()) {
            Ok(r) => r,
            _ => continue,
        };

        // 不脏，不需要组织渲染图， 也不需要渲染脏
        if global_dirty_rect.state == DirtyRectState::UnInit && !render_dirty_mark.0 {
            continue;
        }

        // 如果render_dirty_mark.0, 表示全屏zz
        let mut dirty_rect = global_dirty_rect.value.clone();
        if render_dirty_mark.0 {
            dirty_rect = viewport.0;
        }

        // log::warn!("pass_id1========={:?}, {:?}", dirty_rect, willchange_matrix);
        // 计算视图区域（坐标系为本节点的非旋转坐标系）
        let view_aabb = if let OverflowDesc::Rotate(oveflow_rotate) = &overflow_aabb.desc {
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

        // log::info!("viewport======={:?}, {:?}, {:?}", entity, view_aabb, overflow_aabb);


        if view_aabb.mins.x >= view_aabb.maxs.x || view_aabb.mins.y >= view_aabb.maxs.y {
            continue;
        }

        // 计算视图区域（世界坐标系）
        let aabb_temp;
        let view_world_aabb = match &overflow_aabb.desc {
            OverflowDesc::Rotate(r) => {
                aabb_temp = calc_aabb(&view_aabb, &r.world_rotate);
                &aabb_temp
            }
            _ => &view_aabb,
        };

        // 计算用于剔除的aabb
        // 如果存在transformwillchange，则需要算上脏区域
        // no_will_change用于包围盒剔除渲染对象（渲染对象使用quad来剔除，quad是没有willchange_matrix的参与的）
        let cull_aabb = if let Some(r) = &willchange_matrix.0 {
            if post_info.has_effect() {
                calc_aabb(&view_world_aabb, &r.will_change_invert)
            } else {
                view_aabb.clone()
            }
        } else {
            view_aabb.clone()
        };
        *last_dirty = LastDirtyRect {
            // last: view_aabb.clone(),
            no_will_change: cull_aabb,
        };
        let aabb = Aabb2::new(
            Point2::new(view_aabb.mins.x.floor(), view_aabb.mins.y.floor()),
            Point2::new(view_aabb.maxs.x.ceil(), view_aabb.maxs.y.ceil()),
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
        };

		// 一些不需要后处理的，可以不用计算view_port和matrix， TODO
		post_info.view_port = aabb;
		// 存在旋转，需要旋转回父上下文
		if let OverflowDesc::Rotate(matrix) = &overflow_aabb.desc {
			post_info.matrix = WorldMatrix(&matrix.from_context_rotate * &post_info.matrix.0, true);
		} else {
			post_info.matrix = WorldMatrix(world_matrix, false);
		}
    }

    // 没有渲染脏
    if (all_dirty_rect.maxs.x - all_dirty_rect.mins.x) <= 0.0 || (all_dirty_rect.maxs.y - all_dirty_rect.mins.y) <= 0.0 {
        return;
    }

    // log::warn!("all_dirty_rect====={:?}", all_dirty_rect);
    // 组织渲染列表
    let mut args = AbQueryArgs {
        node_query,
        pass_query: query_pass.p1(),
        draw_list: query_draw2d_list,
        draw_info,
        post_process: draw_obj_post_query,
    };
	
    quad_tree.query(&all_dirty_rect, intersects, &mut args, ab_query_func);

    // 遍历所有的pass，设置不透明渲染列表和透明渲染列表
    for mut list in args.draw_list.iter_mut() {
        let list = &mut list;
        list.opaque.clear();
        list.transparent.clear();
        if list.all_list.len() == 0 {
            continue;
        }

        // 按深度从小到大排序
        list.all_list.sort_by(|(_a, a_z_depth, a_sort), (_b, b_z_depth, b_sort)| {
            if a_z_depth.start < b_z_depth.start {
                std::cmp::Ordering::Less
            } else if a_z_depth.start > b_z_depth.start {
                std::cmp::Ordering::Greater
            } else {
                if a_sort.order() < b_sort.order() {
                    std::cmp::Ordering::Less
                } else if a_sort.order() > b_sort.order() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            // 用渲染管线排序，TODO
            // draw_state.get(a)
        });

        for i in 0..list.all_list.len() {
            let (entity, _, draw_info) = list.all_list[i];
            // 暂时放入不透明列表，TODO
            if draw_info.is_opacity() {
                list.opaque.push(entity);
            } else {
                list.transparent.push(entity);
            }
        }
    }

    // 重置渲染脏
    for mut i in query_root.p1().iter_mut() {
        **i = false;
    }
}

pub struct AbQueryArgs<'s, 'a> {
    node_query: Query<'a, 'a, (&'s InPassId, &'s DrawList, &'s Quad, &'s ZRange, &'s IsShow, Entity)>,
    pass_query: Query<'a, 'a, (Option<&'s ParentPassId>, &'s LastDirtyRect, &'s Camera)>,
    draw_list: Query<'a, 'a, &'s mut Draw2DList>,
	post_process: Query<'a, 'a, (), (With<PostProcess>, With<DrawState>)>,
    draw_info: Query<'a, 'a, &'s DrawInfo>,
}

fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, _aabb: &Aabb2, _bind: &()) {
    // quad_tree.
    if let Ok((in_pass_id, draw_list, quad, z_range, is_show, entity)) = arg.node_query.get(*id) {
		

        // log::warn!("draw_list1==================entity: {:?}, draw_list: {:?}, {}, {:?}", entity, draw_list, is_show.get_visibility(), quad, );
        let (parent_pass_id, context_dirty, camera) = match arg.pass_query.get(***in_pass_id) {
            Ok(r) => r,
            _ => return,
        };
        // log::warn!("draw_list2==================id: {:?}, {:?}, {:?}, quad: {:?}", id, in_pass_id, draw_list, quad);
        // global_dirty_rect应该是pass内部的aadd，（与TransformWillChange有关）
        if draw_list.len() > 0 {
            if is_show.get_visibility() && intersects(quad, &context_dirty.no_will_change) {
                let mut list = arg.draw_list.get_mut(***in_pass_id).unwrap();
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
                if let Ok(mut p) = arg.draw_list.get_mut(*parent.0) {
                    p.all_list
                        .push((DrawIndex::Pass2D(EntityKey(entity)), z_range.clone(), DrawInfo::new(10, false)));
                }
            }
        }
    }
}
