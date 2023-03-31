use bevy::ecs::prelude::{Entity, RemovedComponents};
use bevy::ecs::query::{Changed, Or, With};
use bevy::ecs::system::{Commands, Local, ParamSet, Query, Res};
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::prelude::OrDefault;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::renderer::vertices::{RenderVertices, EVerticesBufferUsage, RenderIndices};
use pi_render::rhi::asset::{RenderRes, TextureRes};
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::rhi::shader::{BindLayout, Input};
use pi_share::{Share, ShareMutex};
use pi_style::style::ImageRepeatOption;
use wgpu::IndexFormat;

use crate::components::calc::{BackgroundImageTexture, DrawInfo, EntityKey, LayoutResult, NodeId};
use crate::components::draw_obj::{BoxType, PipelineMeta};
use crate::components::user::{Aabb2, BackgroundImageClip, BackgroundImageMod, FitType, NotNanRect, Point2, Vector2};
use crate::components::DrawBundle;
use crate::resource::draw_obj::{CommonSampler, PosUv1VertexLayout, ProgramMetaRes, ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup};
use crate::resource::RenderObjType;

use crate::shader::color::PositionVert;
use crate::shader::image::{ProgramMeta, SampBind, UvVert};
use crate::shader::ui_meterial::UiMaterialBind;
use crate::system::utils::{clear_draw_obj, push_quad};
use crate::utils::tools::{calc_float_hash, calc_hash, eq_f32};
use crate::{
    components::{calc::DrawList, draw_obj::DrawState, user::BackgroundImage},
    resource::draw_obj::UnitQuadBuffer,
};

pub struct CalcBackgroundImage;

pub fn calc_background_image(
    render_type: Local<RenderObjType>,
    del: RemovedComponents<BackgroundImageTexture>,
    mut query: ParamSet<(
        // 布局修改、BackgroundImage修改、BackgroundImageClip修改、圆角修改或删除，需要修改或创建背景图片的DrawObject
        Query<
            (
                Entity,
                &'static BackgroundImage,
                &'static LayoutResult,
                &mut DrawList,
                OrDefault<BackgroundImageClip>,
                OrDefault<BackgroundImageMod>,
                &'static BackgroundImageTexture,
            ),
            (
                With<BackgroundImageTexture>,
                Or<(Changed<BackgroundImageTexture>, Changed<BackgroundImageClip>, Changed<LayoutResult>)>,
            ),
        >,
        // BackgroundImage删除，需要删除对应的DrawObject
        Query<(Option<&'static BackgroundImageTexture>, &'static mut DrawList)>,
    )>,
    // mut query_draw: Query<(&'static mut DrawState, &mut BoxType, &'static mut StaticIndex, &'static mut FSDefines, &'static mut VSDefines)>,
    mut query_draw: Query<(&mut DrawState, &mut BoxType)>,
    mut commands: Commands,

    device: Res<PiRenderDevice>,

    unit_quad_buffer: Res<UnitQuadBuffer>,
    ui_meterial_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,

    buffer_assets: Res<ShareAssetMgr<RenderRes<Buffer>>>,
    bind_group_assets: Res<ShareAssetMgr<RenderRes<BindGroup>>>,
    common_sampler: Res<CommonSampler>,
    program_meta: OrInitRes<ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitRes<PosUv1VertexLayout>,
    shader_catch: OrInitRes<ShaderInfoCache>,
) {
    // 删除对应的DrawObject
    clear_draw_obj(*render_type, del, query.p1(), &mut commands);

    let texture_group_layout = &program_meta.bind_group_layout[SampBind::set() as usize];

    let mut init_spawn_drawobj = Vec::new();
    for (node_id, background_image, layout, mut draw_list, background_image_clip, background_image_mod, background_image_texture) in
        query.p0().iter_mut()
    {
        match draw_list.get(**render_type) {
            // background_color已经存在一个对应的DrawObj， 则修改color group
            Some(r) => {
                let (mut draw_state, mut old_box_type) = match query_draw.get_mut(*r) {
                    Ok(r) => r,
                    _ => continue,
                };

                let box_type = modify(
                    &background_image,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &unit_quad_buffer,
                    &bind_group_assets,
                    &texture_group_layout,
                    &background_image_texture,
                    background_image_clip,
                    background_image_mod,
                    &common_sampler,
                );

                if *old_box_type != box_type {
                    *old_box_type = box_type;
                }
            }
            None => {
                // 创建新的DrawObj
                let new_draw_obj = commands.spawn_empty().id();
                // 设置DrawState（包含color group）
                let mut draw_state = DrawState::default();

                let ui_material_group = ui_meterial_alloter.alloc();
                draw_state.bindgroups.insert_group(UiMaterialBind::set(), ui_material_group);

                let box_type = modify(
                    &background_image,
                    layout,
                    &mut draw_state,
                    &device,
                    &buffer_assets,
                    &unit_quad_buffer,
                    &bind_group_assets,
                    &texture_group_layout,
                    &background_image_texture,
                    background_image_clip,
                    background_image_mod,
                    &common_sampler,
                );
                init_spawn_drawobj.push((
                    new_draw_obj,
                    DrawBundle {
                        node_id: NodeId(EntityKey(node_id)),
                        draw_state,
                        box_type,
                        pipeline_meta: PipelineMeta {
                            program: program_meta.clone(),
                            state: shader_catch.common.clone(),
                            vert_layout: vert_layout.clone(),
                            defines: Default::default(),
                        },
                        draw_info: DrawInfo::new(3, false), //TODO
                    },
                ));
                // 建立Node对DrawObj的索引
                draw_list.insert(**render_type, new_draw_obj);
            }
        }
    }

    if init_spawn_drawobj.len() > 0 {
        commands.insert_or_spawn_batch(init_spawn_drawobj.into_iter());
    }
}

// 返回当前需要的StaticIndex
fn modify(
    image: &BackgroundImage,
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
    // let border_radius = cal_content_border_radius(&cal_border_radius(border_radius, layout), (pos.mins.y, pos.maxs.x, pos.maxs.y, pos.mins.x));

    let (vertex_buffer, uv_buffer, index_buffer, is_unit) = if (background_image_mod.object_fit == FitType::Fill
        || background_image_mod.object_fit == FitType::Cover)
        && background_image_mod.repeat.x == ImageRepeatOption::Stretch
        && background_image_mod.repeat.y == ImageRepeatOption::Stretch
        && layout.border.left == 0.0
        && layout.border.right == 0.0
        && layout.border.top == 0.0
        && layout.border.bottom == 0.0
    {
        (
            unit_quad_buffer.vertex.clone(),
            if clip.is_unit() {
                unit_quad_buffer.vertex.clone()
            } else {
                let uv_key = calc_hash(&"texture uv", calc_float_hash(&[*clip.top, *clip.right, *clip.bottom, *clip.left], 0));
                if let Some(r) = buffer_assets.get(&uv_key) {
                    r
                } else {
                    let uvs = [
                        *clip.left,
                        *clip.top,
                        *clip.right,
                        *clip.top,
                        *clip.right,
                        *clip.bottom,
                        *clip.left,
                        *clip.bottom,
                    ];
                    let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                        label: Some("background image uv buffer init"),
                        contents: bytemuck::cast_slice(&uvs),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    buffer_assets.insert(uv_key, RenderRes::new(buf, uvs.len() * 2)).unwrap()
                }
            },
            unit_quad_buffer.index.clone(),
            BoxType::ContentRect,
        )
    } else {
        let hash = calc_hash(
            background_image_mod,
            calc_float_hash(&[layout.rect.top, layout.rect.right, layout.rect.bottom, layout.rect.left], 0),
        );
        let vertex_key = calc_hash(&"image vert", hash);
        let index_key = calc_hash(&"index vert", hash);
        let uv_key = calc_hash(&"texture uv", calc_float_hash(&[*clip.top, *clip.right, *clip.bottom, *clip.left], hash));

        match (buffer_assets.get(&vertex_key), buffer_assets.get(&uv_key), buffer_assets.get(&index_key)) {
            (Some(vert), Some(uv), Some(index)) => (vert, uv, index, BoxType::ContentNone),
            (vert_buffer, uv_buffer, index_buffer) => {
                let (pos, uv, texture_size, _is_part) = get_pos_uv(texture, clip, background_image_mod, layout);
                let (vertex, uvs, indices) = get_pos_uv_buffer(&pos, &uv, texture_size, background_image_mod);
                (
                    match vert_buffer {
                        Some(r) => r,
                        None => {
                            let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("background image vert buffer init"),
                                contents: bytemuck::cast_slice(vertex.as_slice()),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                            buffer_assets.insert(vertex_key, RenderRes::new(buf, vertex.len() * 4)).unwrap()
                        }
                    },
                    match uv_buffer {
                        Some(r) => r,
                        None => {
                            let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("background image uv buffer init"),
                                contents: bytemuck::cast_slice(uvs.as_slice()),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                            buffer_assets.insert(uv_key, RenderRes::new(buf, uvs.len() * 2)).unwrap()
                        }
                    },
                    match index_buffer {
                        Some(r) => r,
                        None => {
                            let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                                label: Some("background image index buffer init"),
                                contents: bytemuck::cast_slice(indices.as_slice()),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                            buffer_assets.insert(index_key, RenderRes::new(buf, indices.len() * 2)).unwrap()
                        }
                    },
                    BoxType::ContentNone,
                )
            }
        }
    };
	
	draw_state.vertex = 0..(vertex_buffer.size()/8) as u32;
	draw_state.insert_vertices(RenderVertices { slot: PositionVert::location(), buffer: EVerticesBufferUsage::GUI(vertex_buffer), buffer_range: None, size_per_value: 8 });
	draw_state.insert_vertices(RenderVertices { slot: UvVert::location(), buffer: EVerticesBufferUsage::GUI(uv_buffer), buffer_range: None, size_per_value: 8 });
	draw_state.indices = Some(RenderIndices { buffer: EVerticesBufferUsage::GUI(index_buffer), buffer_range: None, format: IndexFormat::Uint16 } );

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

    draw_state.bindgroups.insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group));

    is_unit
}

// 获得图片的4个点(逆时针)的坐标和uv的Aabb
fn get_pos_uv(texture: &BackgroundImageTexture, clip: &NotNanRect, fit: &BackgroundImageMod, layout: &LayoutResult) -> (Aabb2, Aabb2, Vector2, bool) {
    let width = layout.rect.right - layout.rect.left - layout.border.right - layout.border.left;
    let height = layout.rect.bottom - layout.rect.top - layout.border.bottom - layout.border.top;
    let mut p1 = Point2::new(layout.border.left, layout.border.top);
    let mut p2 = Point2::new(p1.x + width, p1.y + height);
    let src = &texture.0;
    let texture_size = Vector2::new(
        src.width as f32 * (clip.right - clip.left).abs(),
        src.height as f32 * (clip.bottom - clip.top).abs(),
    );
    let mut uv1 = Point2::new(*clip.left, *clip.top);
    let mut uv2 = Point2::new(*clip.right, *clip.bottom);

    // 如果不是填充，总是居中显示。 如果在范围内，则修改点坐标。如果超出的部分，会进行剪切，剪切会修改uv坐标。
    match fit.object_fit {
        FitType::None => {
            // 保持原有尺寸比例。同时保持内容原始尺寸大小。 超出部分会被剪切
            if texture_size.x <= width {
                let x = (width - texture_size.x) / 2.0;
                p1.x += x;
                p2.x -= x;
            } else {
                let x = (texture_size.x - width) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
                uv1.x += x;
                uv2.x -= x;
            }
            if texture_size.y <= height {
                let y = (height - texture_size.y) / 2.0;
                p1.y += y;
                p2.y -= y;
            } else {
                let y = (texture_size.y - height) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
                uv1.y += y;
                uv2.y -= y;
            }
        }
        FitType::Contain => {
            // 保持原有尺寸比例。保证内容尺寸一定可以在容器里面放得下。因此，此参数可能会在容器内留下空白。
            fill(&texture_size, &mut p1, &mut p2, width, height);
        }
        FitType::Cover => {
            // 保持原有尺寸比例。保证内容尺寸一定大于容器尺寸，宽度和高度至少有一个和容器一致。超出部分会被剪切
            if width != 0.0 && height != 0.0 {
                let rw = texture_size.x / width;
                let rh = texture_size.y / height;

                if rw > rh {
                    let x = (texture_size.x - width * rh) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
                    uv1.x += x;
                    uv2.x -= x;
                } else {
                    let y = (texture_size.y - height * rw) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
                    uv1.y += y;
                    uv2.y -= y;
                }
            }
        }
        FitType::ScaleDown => {
            // 如果内容尺寸小于容器尺寸，则直接显示None。否则就是Contain
            if texture_size.x <= width && texture_size.y <= height {
                let x = (width - texture_size.x) / 2.0;
                let y = (height - texture_size.y) / 2.0;
                p1.x += x;
                p1.y += y;
                p2.x -= x;
                p2.y -= y;
            } else {
                fill(&texture_size, &mut p1, &mut p2, width, height);
            }
        }
        FitType::Fill => (), // 填充。 内容拉伸填满整个容器，不保证保持原有的比例
    };
    (Aabb2::new(p1, p2), Aabb2::new(uv1, uv2), texture_size, false)
}

#[derive(Clone, DerefMut, Deref)]
pub struct BackgroundImageAwait(Share<ShareMutex<Vec<(Entity, Atom, Handle<TextureRes>)>>>);

impl Default for BackgroundImageAwait {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Vec::new()))) }
}

fn get_pos_uv_buffer(pos: &Aabb2, clip: &Aabb2, texture_size: Vector2, image_mod: &BackgroundImageMod) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    let (p1, p2) = (&pos.mins, &pos.maxs);
    let (uv1, uv2) = (&clip.mins, &clip.maxs);
    let w = p2.x - p1.x;
    let h = p2.y - p1.y;

    let (uoffset, uspace, ustep) = calc_step(w, texture_size.x, image_mod.repeat.x);
    let (voffset, vspace, vstep) = calc_step(h, texture_size.y, image_mod.repeat.y);

    let mut vert_arr = Vec::default();
    let mut uv_arr = Vec::default();
    let mut index_arr = Vec::default();
    let mut index = index_arr.len() as u16;

    let (mut cur_y, mut next_y) = (p1.y, p1.y + vstep);
    let mut v2 = uv2.y;

    // 第一个四边形的u2
    let mut u2 = uv2.x;
    if uoffset > 0.0 {
        u2 = uv1.x + uoffset / ustep * (uv2.x - uv1.x);
    }

    let mut u_end = pos.maxs.x;
    let mut v_end = pos.maxs.y;
    if uspace > 0.0 && w < ustep * 2.0 {
        u_end = (pos.maxs.x - uspace).min(u_end);
    }
    if vspace > 0.0 && h < vstep * 2.0 {
        v_end = (pos.maxs.y - vspace).min(v_end);
    }

    loop {
        if next_y > v_end {
            next_y = v_end;
            v2 = uv1.y + voffset / vstep * (uv2.y - uv1.y);
        }

        let p_left_top = push_vertex(&mut vert_arr, p1.x, cur_y, &mut index);
        let p_right_top = push_vertex(&mut vert_arr, u_end, cur_y, &mut index);
        uv_arr.extend_from_slice(&[uv1.x, uv1.y]);
        uv_arr.extend_from_slice(&[u2, uv1.y]);

        let p_left_bootom = push_vertex(&mut vert_arr, p1.x, next_y, &mut index);
        let p_right_bottom = push_vertex(&mut vert_arr, u_end, next_y, &mut index);
        uv_arr.extend_from_slice(&[uv1.x, v2]);
        uv_arr.extend_from_slice(&[u2, v2]);

        push_u_arr(
            &mut vert_arr,
            &mut uv_arr,
            &mut index_arr,
            p_left_top,
            p_left_bootom,
            p_right_bottom,
            p_right_top,
            uv1.x,
            uv1.y,
            uv2.x,
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
