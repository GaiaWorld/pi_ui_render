use std::io::Result;

use ordered_float::NotNan;
use pi_assets::asset::{Asset, Handle};
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_ecs::prelude::{Changed, Commands, EntityCommands, Event, Id, Query, Res, Write};
use pi_ecs::prelude::{Deleted, Or, OrDefault, ParamSet, ResMut, With};
use pi_ecs_macros::{listen, setup};
use pi_flex_layout::prelude::Rect;
use pi_polygon::{interp_mult_by_lg, mult_to_triangle, split_by_lg, split_by_radius, split_mult_by_lg, LgCfg};
use pi_render::rhi::asset::{RenderRes, TextureRes};
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::dyn_uniform_buffer::Group;
use pi_share::{Share, ShareMutex};
use pi_style::style::{BackgroundImageMod, ImageRepeatOption};
use smallvec::smallvec;
use wgpu::IndexFormat;

use crate::components::calc::{BackgroundImageTexture, LayoutResult};
use crate::components::draw_obj::{BoxType, DrawGroup, DynDrawGroup};
use crate::components::user::{Aabb2, BackgroundImageClip, BorderRadius, FitType, Point2, Vector2};
use crate::resource::draw_obj::{CommonSampler, DynBindGroupIndex, DynUniformBuffer, ImageStaticIndex, StaticIndex};

use crate::shaders::image::{ImageMaterialBind, ImageMaterialGroup, PositionVertexBuffer, SampTex2DGroup, UvVertexBuffer};
use crate::utils::tools::{calc_float_hash, calc_hash, eq_f32, get_content_radius};
use crate::{
    components::{
        calc::{DrawList, NodeId},
        draw_obj::{DrawObject, DrawState},
        user::{BackgroundImage, Node},
    },
    resource::draw_obj::{Shaders, UnitQuadBuffer},
};

use super::border_image::push_quad;

pub struct CalcBackgroundImage;

#[setup]
impl CalcBackgroundImage {
    /// 创建RenderObject，用于渲染背景颜色
    #[system]
    pub async fn calc_background_image(
        mut query: ParamSet<(
            // 布局修改、BackgroundImage修改、BackgroundImageClip修改、圆角修改或删除，需要修改或创建背景图片的DrawObject
            Query<
                'static,
                'static,
                Node,
                (
                    Id<Node>,
                    &'static BackgroundImage,
                    Option<&'static BorderRadius>,
                    &'static LayoutResult,
                    Write<BackgroundImageDrawId>,
                    Write<DrawList>,
                    OrDefault<BackgroundImageClip>,
                    OrDefault<BackgroundImageMod>,
                    &'static BackgroundImageTexture,
                ),
                (
                    With<BackgroundImageTexture>,
                    Or<(
                        Changed<BackgroundImageTexture>,
                        Changed<BorderRadius>,
                        Deleted<BorderRadius>,
                        Changed<BackgroundImageClip>,
                        Changed<LayoutResult>,
                    )>,
                ),
            >,
            // BackgroundImage删除，需要删除对应的DrawObject
            Query<
                'static,
                'static,
                Node,
                (Option<&'static BackgroundImage>, Write<BackgroundImageDrawId>, Write<DrawList>),
                Deleted<BackgroundImage>,
            >,
        )>,

        query_draw: Query<'static, 'static, DrawObject, (Write<DrawState>, OrDefault<BoxType>)>,
        mut draw_obj_commands: EntityCommands<DrawObject>,
        mut draw_state_commands: Commands<DrawObject, DrawState>,
        mut node_id_commands: Commands<DrawObject, NodeId>,
        mut shader_static_commands: Commands<DrawObject, StaticIndex>,
        mut is_unit_quad_commands: Commands<DrawObject, BoxType>,

        // load_mgr: ResMut<'a, LoadMgr>,
        device: Res<'static, RenderDevice>,
        static_index: Res<'static, ImageStaticIndex>,
        shader_static: Res<'static, Shaders>,
        unit_quad_buffer: Res<'static, UnitQuadBuffer>,
        common_sampler: Res<'static, CommonSampler>,

        buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
        bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,

        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
        image_material_bind_group: Res<'static, DynBindGroupIndex<ImageMaterialGroup>>,
    ) -> Result<()> {
        for (background_image, mut draw_index, mut render_list) in query.p1_mut().iter_mut() {
            // BackgroundColor不存在时，删除对应DrawObject
            if background_image.is_some() {
                continue;
            };

            // 删除对应的DrawObject
            if let Some(draw_index_item) = draw_index.get() {
                draw_obj_commands.despawn(draw_index_item.0.clone());
                if let Some(r) = render_list.get_mut() {
                    for i in 0..r.len() {
                        let item = &r[i];
                        if item == &draw_index_item.0 {
                            r.swap_remove(i);
                        }
                    }
                }
                draw_index.remove();
            }
        }
        let texture_group_layout = &shader_static.get(static_index.shader).unwrap().bind_group_layout[SampTex2DGroup::id() as usize];

        for (
            node,
            background_image,
            radius,
            layout,
            mut draw_index,
            mut render_list,
            background_image_clip_change,
            background_image_mod,
            background_image_texture,
        ) in query.p0_mut().iter_mut()
        {
            match draw_index.get() {
                // background_color已经存在一个对应的DrawObj， 则修改color group
                Some(r) => {
                    let (mut draw_state_item, old_unit_quad) = query_draw.get_unchecked(**r);
                    let draw_state = draw_state_item.get_mut().unwrap();
                    let new_unit_quad = modify(
                        &background_image,
                        radius,
                        layout,
                        draw_state,
                        &device,
                        &buffer_assets,
                        &unit_quad_buffer,
                        &bind_group_assets,
                        &texture_group_layout,
                        &background_image_texture,
                        background_image_clip_change,
                        background_image_mod,
                        &common_sampler,
                    )
                    .await;
                    draw_state_item.notify_modify();

                    if *old_unit_quad != new_unit_quad {
                        is_unit_quad_commands.insert(**r, new_unit_quad);
                    }
                }
                // 否则，创建一个新的DrawObj，并设置color group;
                // 修改以下组件：
                // * <Node, BackgroundImageDrawId>
                // * <Node, DrawList>
                // * <DrawObject, DrawState>
                // * <DrawObject, NodeId>
                // * <DrawObject, IsUnitQuad>
                None => {
                    // 创建新的DrawObj
                    let new_draw_obj = draw_obj_commands.spawn();
                    // 设置DrawState（包含color group）
                    let mut draw_state = DrawState::default();

                    let image_material_dyn_offset = dyn_uniform_buffer.alloc_binding::<ImageMaterialBind>();
                    let group = DrawGroup::Dyn(DynDrawGroup::new(
                        (*image_material_bind_group).clone(),
                        smallvec![image_material_dyn_offset],
                    ));
                    draw_state.bind_groups.insert_group(ImageMaterialGroup::id(), group);

                    let new_unit_quad = modify(
                        &background_image,
                        radius,
                        layout,
                        &mut draw_state,
                        &device,
                        &buffer_assets,
                        &unit_quad_buffer,
                        &bind_group_assets,
                        &texture_group_layout,
                        &background_image_texture,
                        background_image_clip_change,
                        background_image_mod,
                        &common_sampler,
                    )
                    .await;
                    draw_state_commands.insert(new_draw_obj, draw_state);
                    // 建立DrawObj对Node的索引
                    node_id_commands.insert(new_draw_obj, NodeId(node));
                    is_unit_quad_commands.insert(new_draw_obj, new_unit_quad);
                    shader_static_commands.insert(new_draw_obj, static_index.clone());

                    // 建立Node对DrawObj的索引
                    draw_index.write(BackgroundImageDrawId(new_draw_obj));
                    match render_list.get_mut() {
                        Some(r) => {
                            r.push(new_draw_obj);
                            render_list.notify_modify();
                        }
                        None => {
                            let mut r = DrawList::default();
                            r.push(new_draw_obj);
                            render_list.write(r);
                        }
                    };
                }
            }
        }
        return Ok(());
    }
}

#[derive(Deref, Default)]
pub struct BackgroundImageDrawId(Id<DrawObject>);

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BackgroundImage, Delete), component=(Node, Node, Delete))]
pub fn background_image_delete(e: Event, query: Query<Node, &BackgroundImageDrawId>, mut draw_obj: EntityCommands<DrawObject>) {
    if let Some(index) = query.get_by_entity(e.id) {
        draw_obj.despawn(**index);
    }
}

// 返回当前需要的StaticIndex
async fn modify<'a>(
    image: &BackgroundImage,
    radius: Option<&BorderRadius>,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    unit_quad_buffer: &UnitQuadBuffer,
    group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
    texture_group_layout: &BindGroupLayout,
    texture: &BackgroundImageTexture,
    clip: &BackgroundImageClip,
    background_image_mod: &BackgroundImageMod,
    common_sampler: &CommonSampler,
) -> BoxType {
    let radius = get_content_radius(radius, layout);

    let (vertex_buffer, uv_buffer, index_buffer, is_unit) = if radius.is_some()
        || background_image_mod.object_fit != FitType::Fill
        || background_image_mod.repeat.x != ImageRepeatOption::Stretch
        || background_image_mod.repeat.y != ImageRepeatOption::Stretch
    {
        let (vertex, uvs, indices) = get_pos_uv(texture, &clip.0, background_image_mod, layout);
        // modify_radius_linear_geo
        let vertex_key = calc_float_hash(vertex.as_slice(), calc_hash(&("image vert", radius), 0));
        let index_key = calc_hash(&("image index", radius, indices.as_slice()), 0);
        let uv_key = calc_float_hash(vertex.as_slice(), calc_hash(&("image uv", radius), 0));

        let v_buffer = match buffer_assets.get(&vertex_key) {
            Some(r) => r,
            None => {
                // if radius.is_some() {
                //     (vertex, uv, indices) = use_layout_pos(uv_aabb, layout, radius.as_ref().unwrap());
                // }

                let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                    label: Some("background image vert buffer init"),
                    contents: bytemuck::cast_slice(vertex.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                buffer_assets.insert(vertex_key, RenderRes::new(buf, vertex.len() * 4)).unwrap()
            }
        };
        let uv_buffer = match buffer_assets.get(&uv_key) {
            Some(r) => r,
            None => {
                let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                    label: Some("background image uv buffer init"),
                    contents: bytemuck::cast_slice(uvs.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                buffer_assets.insert(uv_key, RenderRes::new(buf, uvs.len() * 2)).unwrap()
            }
        };
        let index_buffer = match buffer_assets.get(&index_key) {
            Some(r) => r,
            None => {
                let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                    label: Some("background image index buffer init"),
                    contents: bytemuck::cast_slice(indices.as_slice()),
                    usage: wgpu::BufferUsages::INDEX,
                });
                buffer_assets.insert(index_key, RenderRes::new(buf, indices.len() * 2)).unwrap()
            }
        };
        (v_buffer, uv_buffer, index_buffer, BoxType::None)
    } else {
        (
            unit_quad_buffer.vertex.clone(),
            unit_quad_buffer.vertex.clone(),
            unit_quad_buffer.index.clone(),
            BoxType::Content,
        )
    };

    draw_state.vbs.insert(PositionVertexBuffer::id() as usize, (vertex_buffer, 0));
    draw_state.vbs.insert(UvVertexBuffer::id() as usize, (uv_buffer, 0));
    let len = index_buffer.size() / 2;
    draw_state.ib = Some((index_buffer, len as u64, IndexFormat::Uint16));

    let texture_group_key = calc_hash(&image.0.get_hash(), calc_hash(&"image texture", 0));
    // texture BindGroup
    let texture_group = match group_assets.get(&texture_group_key) {
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
                label: Some("bg image group create"),
            });
            group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
        }
    };

    draw_state
        .bind_groups
        .insert_group(SampTex2DGroup::id(), DrawGroup::Static(texture_group));

    is_unit
}

#[derive(Clone, DerefMut, Deref)]
pub struct BackgroundImageAwait(Share<ShareMutex<Vec<(Id<Node>, Atom, Handle<TextureRes>)>>>);

impl Default for BackgroundImageAwait {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Vec::new()))) }
}

// pub struct CalcBackgroundImageLoad;

// #[setup]
// impl CalcBackgroundImageLoad {
//     /// BorderImage创建，加载对应的图片
//     /// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
//     /// 因为BorderImageTexture未加锁，其他线程可能正在使用
//     /// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
//     #[listen(component=(Node, BackgroundImage, Create))]
//     pub fn background_image_change(
//         e: Event,
//         mut query: Query<Node, (&BackgroundImage, Write<BackgroundImageTexture>)>,
//         texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
//         image_assets_mgr: Res<Share<AssetMgr<ImageRes>>>,
//         border_image_await: Res<BackgroundImageAwait>,
//         queue: Res<RenderQueue>,
//         device: Res<RenderDevice>,
//     ) {
//         println!("=============== background_image_change");
//         let (key, mut texture) = query.get_unchecked_mut_by_entity(e.id);
//         match AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64)) {
//             LoadResult::Ok(r) => texture.write(BackgroundImageTexture(r)),
//             LoadResult::Wait(f) => {
//                 let awaits = (*border_image_await).clone();
//                 let (id, key) = (unsafe { Id::new(e.id.local()) }, key.clone());
//                 MULTI_RUNTIME
//                     .spawn(MULTI_RUNTIME.alloc(), async move {
//                         let r = f.await.unwrap();
//                         awaits.lock().unwrap().push((id, (*key).clone(), r))
//                     })
//                     .unwrap();
//             }
//             LoadResult::Receiver(recv) => {
//                 let (awaits, device, queue) = (
//                     (*border_image_await).clone(),
//                     (*device).clone(),
//                     (*queue).clone(),
//                 );
//                 let image_assets_mgr = (*image_assets_mgr).clone();
//                 let (id, key) = (unsafe { Id::new(e.id.local()) }, (*key).clone());
//                 MULTI_RUNTIME
//                     .spawn(MULTI_RUNTIME.alloc(), async move {
//                         let image =
//                             pi_multimedia::image::load_from_path(&image_assets_mgr, &*key).await;
//                         let image = match image {
//                             Ok(r) => r,
//                             Err(_) => {
//                                 log::error!("load image fail: {:?}", key.as_str());
//                                 panic!();
//                             }
//                         };

//                         let texture = create_texture_from_image(
//                             &image,
//                             &device,
//                             &queue,
//                             (*key).clone(),
//                             recv,
//                         )
//                         .await;
//                         awaits.lock().unwrap().push((id, (*key).clone(), texture))
//                     })
//                     .unwrap();
//             }
//         }
//     }

//     //
//     #[system]
//     pub fn check_await_texture(
//         border_image_await: Res<BackgroundImageAwait>,
//         mut query: Query<Node, (&BackgroundImage, Write<BackgroundImageTexture>)>,
//     ) {
//         let awaits = {
//             let mut border_image_await = border_image_await.0.lock().unwrap();
//             std::mem::replace(&mut *border_image_await, Vec::new())
//         };

//         for (id, key, texture) in awaits.into_iter() {
//             let mut texture_item = match query.get_mut(id) {
//                 Some((img, texture_item)) => {
//                     // borderimage已经修改，不需要设置texture
//                     if **img != key {
//                         continue;
//                     }
//                     texture_item
//                 }
//                 // 节点已经销毁，或borderimage已经被删除，不需要设置texture
//                 None => continue,
//             };
//             println!("=========== texture_item write");
//             texture_item.write(BackgroundImageTexture(texture));
//             println!("=========== texture_item write end");
//         }
//     }
// }

// 获得图片的4个点(逆时针)的坐标和uv的Aabb
fn get_pos_uv(img: &BackgroundImageTexture, clip: &Aabb2, image_mod: &BackgroundImageMod, layout: &LayoutResult) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    let src = img.0.as_ref();
    let size = Vector2::new(
        src.width as f32 * (clip.maxs.x - clip.mins.x).abs(),
        src.height as f32 * (clip.maxs.y - clip.mins.y).abs(),
    );
    let (mut uv1, mut uv2) = (clip.mins, clip.maxs);

    let w = layout.rect.right - layout.rect.left - layout.border.right - layout.border.left;
    let h = layout.rect.bottom - layout.rect.top - layout.border.bottom - layout.border.top;

    let mut p1 = Point2::new(layout.border.left, layout.border.top);
    let mut p2 = Point2::new(p1.x + w, p1.y + h);
    // let w = p2.x - p1.x;
    // let h = p2.y - p1.y;
    // 如果不是填充，总是居中显示。 如果在范围内，则修改点坐标。如果超出的部分，会进行剪切，剪切会修改uv坐标。
    if image_mod.repeat.x == ImageRepeatOption::Stretch && image_mod.repeat.y == ImageRepeatOption::Stretch {
        match image_mod.object_fit {
            FitType::None => {
                // 保持原有尺寸比例。同时保持内容原始尺寸大小。 超出部分会被剪切
                if size.x <= w {
                    let x = (w - size.x) / 2.0;
                    p1.x += x;
                    p2.x -= x;
                } else {
                    let x = (size.x - w) * (uv2.x - uv1.x) * 0.5 / size.x;
                    uv1.x += x;
                    uv2.x -= x;
                }
                if size.y <= h {
                    let y = (h - size.y) / 2.0;
                    p1.y += y;
                    p2.y -= y;
                } else {
                    let y = (size.y - h) * (uv2.y - uv1.y) * 0.5 / size.y;
                    uv1.y += y;
                    uv2.y -= y;
                }
            }
            FitType::Contain => {
                // 保持原有尺寸比例。保证内容尺寸一定可以在容器里面放得下。因此，此参数可能会在容器内留下空白。
                fill(&size, &mut p1, &mut p2, w, h);
            }
            FitType::Cover => {
                // 保持原有尺寸比例。保证内容尺寸一定大于容器尺寸，宽度和高度至少有一个和容器一致。超出部分会被剪切
                let rw = size.x / w;
                let rh = size.y / h;
                if rw > rh {
                    let x = (size.x - w * rh) * (uv2.x - uv1.x) * 0.5 / size.x;
                    uv1.x += x;
                    uv2.x -= x;
                } else {
                    let y = (size.y - h * rw) * (uv2.y - uv1.y) * 0.5 / size.y;
                    uv1.y += y;
                    uv2.y -= y;
                }
            }
            FitType::ScaleDown => {
                // 如果内容尺寸小于容器尺寸，则直接显示None。否则就是Contain
                if size.x <= w && size.y <= h {
                    let x = (w - size.x) / 2.0;
                    let y = (h - size.y) / 2.0;
                    p1.x += x;
                    p1.y += y;
                    p2.x -= x;
                    p2.y -= y;
                } else {
                    fill(&size, &mut p1, &mut p2, w, h);
                }
            }
            FitType::Fill => (), // 填充。 内容拉伸填满整个容器，不保证保持原有的比例
        };
    } else {
        let mut vert_arr = Vec::new();
        let mut uv_arr = Vec::new();
        let mut index_arr = Vec::new();
        let mut index = 0;

        // 上边线两个顶点

        // push_quad(&mut index_arr, p_x1_y1, p_x1_top, p_left_top, p_left_y1);


        let (uoffset, uspace, ustep) = calc_step(w, size.x, image_mod.repeat.x);
        let (voffset, vspace, vstep) = calc_step(h, size.y, image_mod.repeat.y);

        let (mut cur_y, mut next_y) = (p1.y, p1.y + vstep);
        let mut v2 = clip.maxs.y;
        let mut v_end = p2.y;

        let mut u2 = clip.maxs.x;
        if uoffset > 0.0 {
            u2 = clip.mins.x + uoffset / ustep * (clip.maxs.x - clip.mins.x);
        }
        let mut u_end = p2.x;

        if uspace > 0.0 && w < ustep * 2.0 {
            u_end -= uspace;
        }
        if vspace > 0.0 && h < vstep * 2.0 {
            v_end -= vspace;
        }

        loop {
            if next_y > v_end {
                next_y = v_end;
                v2 = clip.mins.y + voffset / vstep * (clip.maxs.y - clip.mins.y);
            }

            let p_left_top = push_vertex(&mut vert_arr, p1.x, cur_y, &mut index);
            let p_right_top = push_vertex(&mut vert_arr, u_end, cur_y, &mut index);
            uv_arr.extend_from_slice(&[clip.mins.x, clip.mins.y]);
            uv_arr.extend_from_slice(&[u2, clip.mins.y]);

            let p_left_bootom = push_vertex(&mut vert_arr, p1.x, next_y, &mut index);
            let p_right_bottom = push_vertex(&mut vert_arr, u_end, next_y, &mut index);
            uv_arr.extend_from_slice(&[clip.mins.x, v2]);
            uv_arr.extend_from_slice(&[u2, v2]);

            push_u_arr(
                &mut vert_arr,
                &mut uv_arr,
                &mut index_arr,
                p_left_top,
                p_left_bootom,
                p_right_bottom,
                p_right_top,
                clip.mins.x,
                clip.mins.y,
                clip.maxs.x,
                v2,
                ustep,
                uspace,
                &mut index,
            ); // 上边
            if next_y > v_end || eq_f32(next_y, v_end) {
                break;
            }

            cur_y = next_y + vspace;
            next_y = cur_y + vstep;
        }

        return (vert_arr, uv_arr, index_arr);
    }

    return (
        vec![p1.x, p1.y, p1.x, p2.y, p2.x, p2.y, p2.x, p1.y],
        vec![uv1.x, uv1.y, uv1.x, uv2.y, uv2.x, uv2.y, uv2.x, uv1.y],
        vec![0, 1, 2, 0, 2, 3],
    );
}

#[inline]
pub fn push_vertex(point_arr: &mut Vec<f32>, x: f32, y: f32, i: &mut u16) -> u16 {
    point_arr.extend_from_slice(&[x, y]);
    let r = *i;
    *i += 1;
    r
}

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
        ImageRepeatOption::Repeat => (csize % img_size, 0.0, img_size),
        ImageRepeatOption::Round => (0.0, 0.0, if c > 1.0 { csize / c.round() } else { csize }),
        ImageRepeatOption::Space => {
            let space = csize % img_size; // 空白尺寸
            let pre_space = if c > 2.0 { space / (c.floor() - 1.0) } else { space };
            (0.0, pre_space, img_size)
        }
        _ => (0.0, 0.0, csize),
    }
}

pub fn push_u_arr(
    point_arr: &mut Vec<f32>,
    uv_arr: &mut Vec<f32>,
    index_arr: &mut Vec<u16>,
    p1: u16,
    p2: u16,
    p3: u16,
    p4: u16,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    space: f32,
    i: &mut u16,
) {
    let y1 = point_arr[p1 as usize * 2 + 1];
    let y2 = point_arr[p2 as usize * 2 + 1];
    let mut cur = point_arr[p1 as usize * 2] + step;
    let max = point_arr[p3 as usize * 2];

    let mut pt1 = p1;
    let mut pt2 = p2;
    while !(cur > max || eq_f32(max, cur)) {
        let i3 = push_vertex(point_arr, cur, y2, i);
        let i4 = push_vertex(point_arr, cur, y1, i);
        uv_arr.extend_from_slice(&[u2, v2]);
        uv_arr.extend_from_slice(&[u2, v1]);
        push_quad(index_arr, pt1, pt2, i3, i4);
        // 因为uv不同，新插入2个顶点
        cur += space;
        // if cur
        pt1 = push_vertex(point_arr, cur, y1, i);
        pt2 = push_vertex(point_arr, cur, y2, i);
        uv_arr.extend_from_slice(&[u1, v1]);
        uv_arr.extend_from_slice(&[u1, v2]);
        cur += step;
    }
    push_quad(index_arr, pt1, pt2, p3, p4);
}

// 按比例缩放到容器大小，居中显示
fn fill(size: &Vector2, p1: &mut Point2, p2: &mut Point2, w: f32, h: f32) {
    let rw = size.x / w;
    let rh = size.y / h;
    if rw > rh {
        let y = (h - size.y / rw) / 2.0;
        p1.y += y;
        p2.y -= y;
    } else {
        let x = (w - size.x / rh) / 2.0;
        p1.x += x;
        p2.x -= x;
    }
}

fn use_layout_pos(uv: Aabb2, layout: &LayoutResult, radius: &Rect<NotNan<f32>>) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;
    let start_x = layout.border.left;
    let start_y = layout.border.top;
    let end_x = width - layout.border.right;
    let end_y = height - layout.border.bottom;
    let (positions, indices) = if *radius.left == 0.0 || width == 0.0 || height == 0.0 {
        (vec![start_x, start_y, start_x, end_y, end_x, end_y, end_x, start_y], vec![0, 1, 2, 3])
    } else {
        split_by_radius(start_x, start_y, end_x - start_x, end_y - start_y, *radius.left - start_x, None)
    };
    // debug_println!("indices: {:?}", indices);
    // debug_println!("split_by_lg,  positions:{:?}, indices:{:?}, top_percent: {}, bottom_percent: {}, start: ({}, {}) , end: ({}, {})", positions, indices, 0.0, 1.0, 0.0, 0.0, 0.0, layout.height);
    let (positions, indices_arr) = split_by_lg(positions, indices, &[0.0, 1.0], (0.0, 0.0), (0.0, height));
    // debug_println!("split_mult_by_lg, positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [{}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.x, uv.max.x]}], 0.0, 1.0, 0.0, 0.0, layout.width, 0.0);
    let (positions, indices_arr) = split_mult_by_lg(positions, indices_arr, &[0.0, 1.0], (0.0, 0.0), (width, 0.0));
    let indices = mult_to_triangle(&indices_arr, Vec::new());
    // debug_println!("u positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [{}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.x, uv.max.x]}], 0.0, 1.0, 0.0, 0.0, layout.width, 0.0);
    let u = interp_mult_by_lg(
        &positions,
        &indices_arr,
        vec![Vec::new()],
        vec![LgCfg {
            unit: 1,
            data: vec![uv.mins.x, uv.maxs.x],
        }],
        &[0.0, 1.0],
        (0.0, 0.0),
        (width, 0.0),
    );
    let v = interp_mult_by_lg(
        &positions,
        &indices_arr,
        vec![Vec::new()],
        vec![LgCfg {
            unit: 1,
            data: vec![uv.mins.y, uv.maxs.y],
        }],
        &[0.0, 1.0],
        (0.0, 0.0),
        (0.0, height),
    );
    // debug_println!("v positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.y, uv.max.y]}], 0.0, 1.0, 0.0, 0.0, 0.0, layout.height);
    let mut uvs = Vec::with_capacity(u[0].len());
    for i in 0..u[0].len() {
        uvs.push(u[0][i]);
        uvs.push(v[0][i]);
    }

    (positions, uvs, indices)
    // render_obj.geometry = Some(engine.create_geo_res(
    //     0,
    //     indices.as_slice(),
    //     &[
    //         AttributeDecs::new(AttributeName::Position, positions.as_slice(), 2),
    //         AttributeDecs::new(AttributeName::UV0, uvs.as_slice(), 2),
    //     ],
    // ));
}
