use bevy::ecs::prelude::{Entity, RemovedComponents};
use bevy::ecs::query::{Changed, Or, With};
use bevy::ecs::system::{Commands, Local, ParamSet, Query, Res};
use bytemuck::{Pod, Zeroable};
use ordered_float::NotNan;
use pi_assets::asset::Asset;
use pi_assets::mgr::AssetMgr;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::renderer::vertices::{RenderVertices, EVerticesBufferUsage, RenderIndices};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_share::Share;
use pi_style::style::ImageRepeatOption;
use wgpu::IndexFormat;

use crate::components::calc::{BorderImageTexture, DrawInfo, EntityKey, LayoutResult};
use crate::components::draw_obj::{BoxType, PipelineMeta};
use crate::components::user::{BorderImage, BorderImageClip, BorderImageSlice, ImageRepeat, Point2};
use crate::components::DrawBundle;
use crate::components::{
    calc::{DrawList, NodeId},
    draw_obj::DrawState,
    user::BorderImageRepeat,
};
use crate::resource::draw_obj::{CommonSampler, PosUv2VertexLayout, ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup};
use crate::resource::RenderObjType;
use crate::shader::image::{PositionVert, ProgramMeta, SampBind};
use crate::shader::ui_meterial::UiMaterialBind;
use crate::system::utils::clear_draw_obj;
use crate::utils::tools::{calc_hash, eq_f32};

/// 创建RenderObject，用于渲染背景颜色
pub fn calc_border_image(
    render_type: Local<RenderObjType>,
    del: RemovedComponents<BorderImageTexture>,
    mut query: ParamSet<(
        // 布局修改、BorderImage修改、圆角修改或删除，需要修改或创建BorderImage的DrawObject
        Query<
            (
                Entity,
                &BorderImage,
                &BorderImageTexture,
                OrDefault<BorderImageClip>,
                OrDefault<BorderImageSlice>,
                OrDefault<BorderImageRepeat>,
                &LayoutResult,
                &mut DrawList,
            ),
            (
                With<BorderImageTexture>,
                Or<(
                    Changed<BorderImageTexture>,
                    Changed<BorderImageClip>,
                    Changed<BorderImageSlice>,
                    Changed<BorderImageRepeat>,
                    Changed<LayoutResult>,
                )>,
            ),
        >,
        // BorderImage删除，需要删除对应的DrawObject
        Query<(Option<&BorderImageTexture>, &mut DrawList)>,
    )>,

    mut query_draw: Query<&'static mut DrawState>,

    mut commands: Commands,

    device: Res<PiRenderDevice>,

    ui_material_group: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,

    buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: Res<ShareAssetMgr<RenderRes<BindGroup>>>,
    common_sampler: Res<CommonSampler>,
    program_meta: OrInitRes<ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitRes<PosUv2VertexLayout>,
    shader_catch: OrInitRes<ShaderInfoCache>,
) {
    // 删除对应的DrawObject
    clear_draw_obj(*render_type, del, query.p1(), &mut commands);

    let texture_group_layout = &program_meta.bind_group_layout[SampBind::set() as usize];
    let mut init_spawn_drawobj = Vec::new();
    for (node_id, border_image, border_texture, border_image_clip, border_image_slice, border_image_repeat, layout, mut draw_list) in
        query.p0().iter_mut()
    {
        match draw_list.get(**render_type as u32) {
            // borderimage已经存在一个对应的DrawObj， 则修改color group
            Some(r) => {
                let mut draw_state = match query_draw.get_mut(*r) {
                    Ok(r) => r,
                    _ => continue,
                };

                modify(
                    &border_image,
                    &border_texture,
                    &border_image_clip,
                    &border_image_slice,
                    &border_image_repeat,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &bind_group_assets,
                    texture_group_layout,
                    &common_sampler,
                );
            }
            // 否则，创建一个新的DrawObj，并设置color group;
            // 修改以下组件：
            // * <Node, BackgroundDrawId>
            // * <Node, DrawList>
            // * <DrawObject, DrawState>
            // * <DrawObject, NodeId>
            // * <DrawObject, IsUnitQuad>
            None => {
                // 创建新的DrawObj
                let new_draw_obj = commands.spawn_empty().id();
                // 设置DrawState（包含color group）
                let mut draw_state = DrawState::default();

                let ui_material_group = ui_material_group.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                modify(
                    &border_image,
                    &border_texture,
                    &border_image_clip,
                    &border_image_slice,
                    &border_image_repeat,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &bind_group_assets,
                    texture_group_layout,
                    &common_sampler,
                );

                init_spawn_drawobj.push((
                    new_draw_obj,
                    DrawBundle {
                        node_id: NodeId(EntityKey(node_id)),
                        draw_state,
                        box_type: BoxType::default(),
                        pipeline_meta: PipelineMeta {
                            program: program_meta.clone(),
                            state: shader_catch.common.clone(),
                            vert_layout: vert_layout.clone(),
                            defines: Default::default(),
                        },
                        draw_info: DrawInfo::new(5, border_texture.is_opacity), //TODO
                    },
                ));
                // 建立Node对DrawObj的索引
                draw_list.insert(**render_type as u32, new_draw_obj);
            }
        }
    }
    if init_spawn_drawobj.len() > 0 {
        commands.insert_or_spawn_batch(init_spawn_drawobj.into_iter());
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

// 返回当前需要的StaticIndex
fn modify<'a>(
    image: &BorderImage,
    texture: &BorderImageTexture,
    clip: &BorderImageClip,
    slice: &BorderImageSlice,
    repeat: &ImageRepeat,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
    texture_group_layout: &BindGroupLayout,
    common_sampler: &CommonSampler,
) {
    // key TODO
    // &layout.border, &layout.rect
    let layout_data = [
        NotNan::new(layout.rect.top).unwrap(),
        NotNan::new(layout.rect.right).unwrap(),
        NotNan::new(layout.rect.bottom).unwrap(),
        NotNan::new(layout.rect.left).unwrap(),
        NotNan::new(layout.border.top).unwrap(),
        NotNan::new(layout.border.right).unwrap(),
        NotNan::new(layout.border.bottom).unwrap(),
        NotNan::new(layout.border.left).unwrap(),
    ];
    // TODO, layout 使用NotNan
    let buffer_key = calc_hash(&("border image", image, clip, slice, repeat, &layout_data), 0);
    let index_key = calc_hash(&("border image index", image, clip, slice, repeat, &layout_data), 0);
    let (vertex_buffer, index_buffer) = match (buffer_assets.get(&buffer_key), buffer_assets.get(&index_key)) {
        (Some(r1), Some(r2)) => (r1, r2),
        (buffer, index) => {
            let (vertex, indices) = get_border_image_stream(texture, clip, slice, repeat, layout, Vec::new(), Vec::new());
            let index = match index {
                Some(r1) => r1,
                _ => {
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("border image index buffer init"),
                        contents: bytemuck::cast_slice(&indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });
                    buffer_assets.insert(index_key, RenderRes::new(buf, indices.len() * 2)).unwrap()
                }
            };
            (
                match buffer {
                    Some(r1) => r1,
                    None => {
                        let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                            label: Some("border image vert buffer init"),
                            contents: bytemuck::cast_slice(&vertex),
                            usage: wgpu::BufferUsages::VERTEX,
                        });
                        buffer_assets.insert(buffer_key, RenderRes::new(buf, vertex.len() * 4)).unwrap()
                    }
                },
                index,
            )
        }
    };

	draw_state.vertex = 0..(vertex_buffer.size()/8) as u32;
	draw_state.insert_vertices(RenderVertices { slot: PositionVert::location(), buffer: EVerticesBufferUsage::GUI(vertex_buffer), buffer_range: None, size_per_value: 8 });
	draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(index_buffer), buffer_range: None, format: IndexFormat::Uint16 } );

    // texture BindGroup
    let texture_group = match group_assets.get(&buffer_key) {
        Some(r) => r,
        None => {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.default),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture.texture_view),
                    },
                ],
                label: Some("border image group create"),
            });
            group_assets.insert(buffer_key, RenderRes::new(group, 5)).unwrap()
        }
    };
    draw_state.bindgroups.insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group));
}

// // 返回当前需要的StaticIndex
// fn modify<'a>(
//     image: &BorderImage,
//     texture: &BorderImageTexture,
//     clip: &BorderImageClip,
//     slice: &BorderImageSlice,
//     repeat: &ImageRepeat,
//     layout: &LayoutResult,
//     draw_state: &mut DrawState,
//     device: &RenderDevice,
//     buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
//     group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
//     texture_group_layout: &BindGroupLayout,
//     common_sampler: &CommonSampler,
// ) {
//     // key TODO
//     // &layout.border, &layout.rect
//     let layout_data = [
//         NotNan::new(layout.rect.top).unwrap(),
//         NotNan::new(layout.rect.right).unwrap(),
//         NotNan::new(layout.rect.bottom).unwrap(),
//         NotNan::new(layout.rect.left).unwrap(),
//         NotNan::new(layout.border.top).unwrap(),
//         NotNan::new(layout.border.right).unwrap(),
//         NotNan::new(layout.border.bottom).unwrap(),
//         NotNan::new(layout.border.left).unwrap(),
//     ];
//     // TODO, layout 使用NotNan
//     let buffer_key = calc_hash(&("border image", image, clip, slice, repeat, &layout_data), 0);
//     let index_key = calc_hash(&("border image index", image, clip, slice, repeat, &layout_data), 0);
//     let (index_len, vertex_buffer, index_buffer) = match (AssetMgr::load(buffer_assets, &buffer_key), AssetMgr::load(buffer_assets, &index_key)) {
//         (LoadResult::Ok(r1), LoadResult::Ok(r2)) => (r2.size() / 2, r1, r2),
//         (buffer, index) => {
//             let (vertex, indices) = get_border_image_stream(texture, clip, slice, repeat, layout, Vec::new(), Vec::new());
//             let index = match index {
//                 LoadResult::Ok(r1) => r1,
//                 LoadResult::Wait(r1) => r1.await.unwrap(),
//                 LoadResult::Receiver(r1) => {
//                     let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                         label: Some("border image index buffer init"),
//                         contents: bytemuck::cast_slice(&indices),
//                         usage: wgpu::BufferUsages::INDEX,
//                     });
//                     r1.receive(index_key, Ok(RenderRes::new(buf, indices.len() * 2))).await.unwrap()
//                 }
//             };
//             (
//                 index.size() / 2,
//                 match buffer {
//                     LoadResult::Ok(r1) => r1,
//                     LoadResult::Wait(r1) => r1.await.unwrap(),
//                     LoadResult::Receiver(r1) => {
//                         let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//                             label: Some("border image vert buffer init"),
//                             contents: bytemuck::cast_slice(&vertex),
//                             usage: wgpu::BufferUsages::VERTEX,
//                         });
//                         r1.receive(buffer_key, Ok(RenderRes::new(buf, vertex.len() * 4))).await.unwrap()
//                     }
//                 },
//                 index,
//             )
//         }
//     };

//     draw_state.vertices.insert(PositionVertexBuffer::id() as usize, (vertex_buffer, 0));
//     draw_state.indices = Some((index_buffer, index_len as u64, IndexFormat::Uint16));

//     // texture BindGroup
//     let texture_group = match group_assets.get(&buffer_key) {
//         Some(r) => r,
//         None => {
//             let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//                 layout: texture_group_layout,
//                 entries: &[
//                     wgpu::BindGroupEntry {
//                         binding: 0,
//                         resource: wgpu::BindingResource::Sampler(&common_sampler.default),
//                     },
//                     wgpu::BindGroupEntry {
//                         binding: 1,
//                         resource: wgpu::BindingResource::TextureView(&texture.texture_view),
//                     },
//                 ],
//                 label: Some("border image group create"),
//             });
//             group_assets.insert(buffer_key, RenderRes::new(group, 5)).unwrap()
//         }
//     };
//     draw_state
//         .bindgroups
//         .insert_group(SampTex2DGroup::id(), DrawBindGroup::Static(texture_group));
// }


#[inline]
fn get_border_image_stream(
    texture: &BorderImageTexture,
    clip: &BorderImageClip,
    slice: &BorderImageSlice,
    repeat: &ImageRepeat,
    layout: &LayoutResult,
    mut vert_arr: Vec<f32>,
    mut index_arr: Vec<u16>,
) -> (Vec<f32>, Vec<u16>) {
    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;
    let p1 = Point2::new(0.0, 0.0);
    let p2 = Point2::new(width, height);
    let left = layout.border.left;
    let right = width - layout.border.right;
    let top = layout.border.top;
    let bottom = height - layout.border.bottom;
    let uvw = *clip.right - *clip.left;
    let uvh = *clip.bottom - *clip.top;
    let (uv_left, uv_right, uv_top, uv_bottom) = (
        *clip.left + *slice.left * uvw,
        *clip.right - *slice.right * uvw,
        *clip.top + *slice.top * uvh,
        *clip.bottom - *slice.bottom * uvh,
    );

    //  p1, p2, w, h, left, right, top, bottom, "UV::", uv1, uv2, uvw, uvh, uv_left, uv_right, uv_top, uv_bottom);
    // TODO 在仅使用左或上的边框时， 应该优化成8个顶点
    // 先将16个顶点和uv放入数组，记录偏移量
    let mut pi = (vert_arr.len() / 3) as u16;
    // 左上的4个点
    let p_x1_y1 = push_vertex(&mut vert_arr, p1.x, p1.y, *clip.left, *clip.top, &mut pi);
    let p_x1_top = push_vertex(&mut vert_arr, p1.x, top, *clip.left, uv_top, &mut pi);
    let mut p_left_top = push_vertex(&mut vert_arr, left, top, uv_left, uv_top, &mut pi);
    let p_left_y1 = push_vertex(&mut vert_arr, left, p1.y, uv_left, *clip.top, &mut pi);
    push_quad(&mut index_arr, p_x1_y1, p_x1_top, p_left_top, p_left_y1);

    // 左下的4个点
    let p_x1_bottom = push_vertex(&mut vert_arr, p1.x, bottom, *clip.left, uv_bottom, &mut pi);
    let p_x1_y2 = push_vertex(&mut vert_arr, p1.x, p2.y, *clip.left, *clip.bottom, &mut pi);
    let p_left_y2 = push_vertex(&mut vert_arr, left, p2.y, uv_left, *clip.bottom, &mut pi);
    let mut p_left_bottom = push_vertex(&mut vert_arr, left, bottom, uv_left, uv_bottom, &mut pi);
    push_quad(&mut index_arr, p_x1_bottom, p_x1_y2, p_left_y2, p_left_bottom);

    // 右下的4个点calc_step
    let mut p_right_bottom = push_vertex(&mut vert_arr, right, bottom, uv_right, uv_bottom, &mut pi);
    let p_right_y2 = push_vertex(&mut vert_arr, right, p2.y, uv_right, *clip.bottom, &mut pi);
    let p_x2_y2 = push_vertex(&mut vert_arr, p2.x, p2.y, *clip.right, *clip.bottom, &mut pi);
    let p_x2_bottom = push_vertex(&mut vert_arr, p2.x, bottom, *clip.right, uv_bottom, &mut pi);
    push_quad(&mut index_arr, p_right_bottom, p_right_y2, p_x2_y2, p_x2_bottom);

    // 右上的4个点
    let p_right_y1 = push_vertex(&mut vert_arr, right, p1.y, uv_right, *clip.top, &mut pi);
    let mut p_right_top = push_vertex(&mut vert_arr, right, top, uv_right, uv_top, &mut pi);
    let p_x2_top = push_vertex(&mut vert_arr, p2.x, top, *clip.right, uv_top, &mut pi);
    let p_x2_y1 = push_vertex(&mut vert_arr, p2.x, p1.y, *clip.right, *clip.top, &mut pi);
    push_quad(&mut index_arr, p_right_y1, p_right_top, p_x2_top, p_x2_y1);

    // 根据图像大小和uv计算
    let texture_center_width = texture.0.width as f32 * (uv_right - uv_left);
    let texture_center_height = texture.0.height as f32 * (uv_bottom - uv_top);

    let (texture_left_width, texture_right_width, texture_top_height, texture_bottom_height) = (
        texture.0.width as f32 * (uv_left - *clip.left),
        texture.0.width as f32 * (*clip.right - uv_right),
        texture.0.height as f32 * (uv_top - *clip.top),
        texture.0.height as f32 * (*clip.bottom - uv_bottom),
    );

    let (mut uoffset_top, mut uspace_top, mut ustep_top) = (0.0, 0.0, 0.0);
    if texture_top_height > 0.0 {
        (uoffset_top, uspace_top, ustep_top) = calc_step(right - left, top / texture_top_height * texture_center_width, repeat.x);

        if ustep_top > 0.0 {
            push_u_arr(
                &mut vert_arr,
                &mut index_arr,
                p_left_y1,
                p_left_top,
                p_right_top,
                p_right_y1,
                uv_left,
                *clip.top,
                uv_right,
                uv_top,
                ustep_top,
                uoffset_top,
                uspace_top,
                &mut pi,
            ); // 上边
        }
    }

    if texture_bottom_height > 0.0 {
        let (uoffest_bottom, uspace_bottom, ustep_bottom) = calc_step(
            right - left,
            layout.border.bottom / texture_bottom_height * texture_center_width,
            repeat.x,
        );
        if ustep_bottom > 0.0 {
            push_u_arr(
                &mut vert_arr,
                &mut index_arr,
                p_left_bottom,
                p_left_y2,
                p_right_y2,
                p_right_bottom,
                uv_left,
                uv_bottom,
                uv_right,
                *clip.bottom,
                ustep_bottom,
                uoffest_bottom,
                uspace_bottom,
                &mut pi,
            ); // 下边
        }
    }

    let (mut voffset_left, mut vspace_left, mut vstep_left) = (0.0, 0.0, 0.0);
    if texture_left_width > 0.0 {
        (voffset_left, vspace_left, vstep_left) = calc_step(bottom - top, left / texture_left_width * texture_center_height, repeat.y);
        if vstep_left > 0.0 {
            push_v_arr(
                &mut vert_arr,
                &mut index_arr,
                p_x1_top,
                p_x1_bottom,
                p_left_bottom,
                p_left_top,
                *clip.left,
                uv_top,
                uv_left,
                uv_bottom,
                vstep_left,
                voffset_left,
                vspace_left,
                &mut pi,
            ); // 左边
        }
    }

    if texture_right_width > 0.0 {
        let (voffset_right, vspace_bottom, vstep_right) =
            calc_step(bottom - top, layout.border.right / texture_right_width * texture_center_height, repeat.y);
        if vstep_right > 0.0 {
            push_v_arr(
                &mut vert_arr,
                &mut index_arr,
                p_right_top,
                p_right_bottom,
                p_x2_bottom,
                p_x2_top,
                uv_right,
                uv_top,
                *clip.right,
                uv_bottom,
                vstep_right,
                voffset_right,
                vspace_bottom,
                &mut pi,
            ); // 右边
        }
    }

    // 处理中间
    if slice.fill {
        if repeat.x == ImageRepeatOption::Stretch {
            ustep_top = right - left;
        }
        if repeat.y == ImageRepeatOption::Stretch {
            vstep_left = bottom - top;
        }
        if vstep_left > 0.0 && ustep_top > 0.0 {
            let mut cur_y = top;
            let mut y_end = bottom;
            push_v_box(
                &mut vert_arr,
                &mut p_left_top,
                &mut p_left_bottom,
                &mut p_right_bottom,
                &mut p_right_top,
                &mut cur_y,
                &mut y_end,
                uv_left,
                uv_top,
                uv_right,
                uv_bottom,
                vstep_left,
                voffset_left,
                vspace_left,
                &mut pi, // point_arr, index_arr, &mut p1, &mut p2, &mut p3, &mut p4, &mut cur, &mut max, u1, v1, u2, u2, step, offset, i,
            );
            let (mut v1, v2) = (vert_arr[(p_left_top * 4 + 3) as usize], vert_arr[(p_right_bottom * 4 + 3) as usize]);
            cur_y += vstep_left;

            while !(cur_y > y_end || eq_f32(cur_y, y_end)) {
                let p_left_bottom = push_vertex(&mut vert_arr, left, cur_y, uv_left, uv_bottom, &mut pi);
                let p_right_bottom = push_vertex(&mut vert_arr, right, cur_y, uv_right, uv_bottom, &mut pi);

                push_u_arr(
                    &mut vert_arr,
                    &mut index_arr,
                    p_left_top,
                    p_left_bottom,
                    p_right_bottom,
                    p_right_top,
                    uv_left,
                    v1,
                    uv_right,
                    uv_bottom,
                    ustep_top,
                    uoffset_top,
                    uspace_top,
                    &mut pi,
                );
                cur_y += vspace_left;
                p_left_top = push_vertex(&mut vert_arr, left, cur_y, uv_left, uv_top, &mut pi);
                p_right_top = push_vertex(&mut vert_arr, right, cur_y, uv_right, uv_top, &mut pi);
                v1 = uv_top;
                cur_y += vstep_left;
            }
            push_u_arr(
                &mut vert_arr,
                &mut index_arr,
                p_left_top,
                p_left_bottom,
                p_right_bottom,
                p_right_top,
                uv_left,
                uv_top,
                uv_right,
                v2,
                ustep_top,
                uoffset_top,
                uspace_top,
                &mut pi,
            );
        }
    }

    (vert_arr, index_arr)
}
// 将四边形放进数组中
pub fn push_vertex(point_arr: &mut Vec<f32>, x: f32, y: f32, u: f32, v: f32, i: &mut u16) -> u16 {
    point_arr.extend_from_slice(&[x, y]);
    point_arr.extend_from_slice(&[u, v]);
    // uv_arr.extend_from_slice(&[u, v]);
    let r = *i;
    *i += 1;
    r
}

// 将四边形放进数组中
pub fn push_quad(index_arr: &mut Vec<u16>, p1: u16, p2: u16, p3: u16, p4: u16) { index_arr.extend_from_slice(&[p1, p2, p3, p1, p3, p4]); }

/// 根据参数计算uv的step
/// 返回初始偏移和不掉宽度
pub fn calc_step(csize: f32, img_size: f32, rtype: ImageRepeatOption) -> (f32, f32, f32) {
    if let ImageRepeatOption::Stretch = rtype {
        return (0.0, 0.0, csize);
    }
    if img_size == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let c = csize / img_size;
    let f = c.round();
    if eq_f32(f, c) {
        // 整数倍的情况（这里消除了浮点误差，大致为整数倍，都认为是整数倍）
        return (0.0, 0.0, img_size);
    }

    match rtype {
        ImageRepeatOption::Repeat => (-(csize % img_size) / 2.0, 0.0, img_size),
        ImageRepeatOption::Round => (0.0, 0.0, if f > 0.0 { csize / f } else { csize }),
        ImageRepeatOption::Space => {
            let space = csize % img_size; // 空白尺寸
            let pre_space = space / (c.floor() + 1.0);
            (0.0, pre_space, if c >= 1.0 { img_size } else { 0.0 })
        }
        _ => (0.0, 0.0, csize),
    }
}

// 将指定区域按u切开
pub fn push_u_arr(
    point_arr: &mut Vec<f32>,
    index_arr: &mut Vec<u16>,
    mut p1: u16,
    mut p2: u16,
    mut p3: u16,
    mut p4: u16,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    offset: f32,
    space: f32,
    i: &mut u16,
) {
    let y1 = point_arr[p1 as usize * 4 + 1];
    let y2 = point_arr[p2 as usize * 4 + 1];
    let mut max = point_arr[p3 as usize * 4];

    let mut cur = point_arr[p1 as usize * 4];
    if offset != 0.0 {
        // repeat
        let u_diff = offset / step * (u2 - u1);
        let (u_start, u_end) = (u1 - u_diff, u2 + u_diff);
        p1 = push_vertex(point_arr, cur, y1, u_start, v1, i);
        p2 = push_vertex(point_arr, cur, y2, u_start, v2, i);
        p3 = push_vertex(point_arr, max, y2, u_end, v2, i);
        p4 = push_vertex(point_arr, max, y1, u_end, v1, i);
        cur += offset;
    }

    if space != 0.0 {
        max = max - space;
        cur = cur + space;
        p1 = push_vertex(point_arr, cur, y1, u1, v1, i);
        p2 = push_vertex(point_arr, cur, y2, u1, v2, i);
        p3 = push_vertex(point_arr, max, y2, u2, v2, i);
        p4 = push_vertex(point_arr, max, y1, u2, v1, i);
    }

    cur += step;

    let mut pt1 = p1;
    let mut pt2 = p2;
    while !(cur > max || eq_f32(cur, max)) {
        let i3 = push_vertex(point_arr, cur, y2, u2, v2, i);
        let i4 = push_vertex(point_arr, cur, y1, u2, v1, i);
        push_quad(index_arr, pt1, pt2, i3, i4);
        // 因为uv不同，新插入2个顶点
        cur += space;
        pt1 = push_vertex(point_arr, cur, y1, u1, v1, i);
        pt2 = push_vertex(point_arr, cur, y2, u1, v2, i);
        cur += step;
    }
    push_quad(index_arr, pt1, pt2, p3, p4);
}
// 将指定区域按v切开
pub fn push_v_arr(
    point_arr: &mut Vec<f32>,
    index_arr: &mut Vec<u16>,
    mut p1: u16,
    mut p2: u16,
    mut p3: u16,
    mut p4: u16,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    offset: f32,
    space: f32,
    i: &mut u16,
) {
    let x1 = point_arr[p1 as usize * 4];
    let x2 = point_arr[p4 as usize * 4];

    let (mut cur, mut max) = (0.0, 0.0);
    push_v_box(
        point_arr, &mut p1, &mut p2, &mut p3, &mut p4, &mut cur, &mut max, u1, v1, u2, v2, step, offset, space, i,
    );
    cur += step;

    let mut pt1 = p1;
    let mut pt4 = p4;
    while !(cur > max || eq_f32(cur, max)) {
        let i2 = push_vertex(point_arr, x1, cur, u1, v2, i);
        let i3 = push_vertex(point_arr, x2, cur, u2, v2, i);
        push_quad(index_arr, pt1, i2, i3, pt4);
        cur += space;
        // 因为uv不同，新插入2个顶点
        pt1 = push_vertex(point_arr, x1, cur, u1, v1, i);
        pt4 = push_vertex(point_arr, x2, cur, u2, v1, i);
        cur += step;
    }
    push_quad(index_arr, pt1, p2, p3, pt4);
}

#[inline]
pub fn push_v_box(
    point_arr: &mut Vec<f32>,
    p1: &mut u16,
    p2: &mut u16,
    p3: &mut u16,
    p4: &mut u16,
    cur: &mut f32,
    max: &mut f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    offset: f32,
    space: f32,
    i: &mut u16,
) {
    let x1 = point_arr[*p1 as usize * 4];
    let x2 = point_arr[*p4 as usize * 4];

    *max = point_arr[*p3 as usize * 4 + 1];
    *cur = point_arr[*p1 as usize * 4 + 1];
    if offset != 0.0 {
        // repeat
        let v_diff = offset / step * (v2 - v1);
        let (v_start, v_end) = (v1 - v_diff, v2 + v_diff);
        *p1 = push_vertex(point_arr, x1, *cur, u1, v_start, i);
        *p2 = push_vertex(point_arr, x1, *max, u1, v_end, i);
        *p3 = push_vertex(point_arr, x2, *max, u2, v_end, i);
        *p4 = push_vertex(point_arr, x2, *cur, u2, v_start, i);
        *cur += offset;
    }

    if space != 0.0 {
        *cur = *cur + space;
        *max = *max - space;
        *p1 = push_vertex(point_arr, x1, *cur, u1, v1, i);
        *p2 = push_vertex(point_arr, x1, *max, u1, v2, i);
        *p3 = push_vertex(point_arr, x2, *max, u2, v2, i);
        *p4 = push_vertex(point_arr, x2, *cur, u2, v1, i);
    }
}
