//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use std::collections::HashMap;

use crate::{
    components::{
        calc::{DrawList, WorldMatrix},
        draw_obj::{InstanceIndex, SvgMark},
        user::{Point3, SvgFilterBlurLevel, SvgInnerContent, Vector2, Vector4},
    },
    resource::SvgRenderObjType,
    shader1::{
        meterial::{
            ColorUniform, GradientColorUniform, GradientEndUniform, GradientPositionUniform, RenderFlagType, Sdf2InfoUniform, ShadowUniform,
            TextOuterGlowUniform, TextOutlineUniform, TextWeightUniform, TyUniform,
        },
        InstanceData, GpuBuffer,
    },
    system::draw_obj::set_box,
};
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::{
    change_detection::DetectChangesMut,
    prelude::{Entity, EventWriter},
    query::{Changed, With},
    system::{Local, ParamSet, Query, Res, ResMut},
    world::Ref,
};
use pi_bevy_ecs_extend::{
    prelude::Layer,
    system_param::{
        layer_dirty::ComponentEvent,
        res::{OrInitRes, OrInitResMut},
        tree::Up,
    },
};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_hal::{
    font::sdf2_table::TexInfo,
    pi_sdf::{self, glyphy::geometry::aabb::AabbEXT},
    runtime::MULTI_MEDIA_RUNTIME,
};

use pi_render::font::{FontSheet, FontType};
use pi_share::{Share, ShareMutex};
use pi_style::style::{Aabb2, Color, FontStyle, Point2};

use crate::resource::Shape;
use crate::{
    components::{
        calc::LayoutResult,
        user::{SvgContent, SvgStyle},
    },
    resource::{draw_obj::InstanceContext, ShareFontSheet},
    system::draw_obj::calc_text::IsRun,
};
use pi_async_rt::prelude::AsyncRuntime;
// use super::IsRun;

pub struct SvgShapeAwaitList(
    pub  Share<
        ShareMutex<
            Vec<(
                Vec<Entity>,
                Share<ShareMutex<(usize, Vec<(u64, TexInfo, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>)>>,
            )>,
        >,
    >,
    ShareMutex<HashMap<u64, u64>>,
);

impl Default for SvgShapeAwaitList {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Vec::new())), ShareMutex::new(HashMap::default())) }
}


/// 更新sdf2的纹理
pub fn update_sdf2_texture(
    mut instances: OrInitResMut<InstanceContext>,
    font_sheet: ResMut<ShareFontSheet>,
    device: Res<PiRenderDevice>,
    common_sampler: Res<crate::resource::draw_obj::CommonSampler>,
) {
    let font_sheet = font_sheet.0.borrow();
    if let (Some(sdf2_index_texture_view), Some(sdf2_data_texture_view), Some(sdf2_shadow_texture_view)) = (
        &font_sheet.sdf2_index_texture_view,
        &font_sheet.sdf2_data_texture_view,
        &font_sheet.sdf2_shadow_texture_view,
    ) {
        if instances.sdf2_texture_group.is_none() {
            let group = (***device).create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &instances.sdf2_texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&sdf2_index_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&sdf2_data_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&sdf2_shadow_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.default),
                    },
                ],
                label: Some("sdf2 texture bind group create"),
            });

            instances.sdf2_texture_group = Some(Share::new(group));
        }
    }
}

/// 文字字形计算
pub fn text_svg(
    mut query: ParamSet<(
        Query<(Entity, &'static mut SvgInnerContent), Changed<SvgInnerContent>>,
        Query<&mut SvgInnerContent>,
    )>,
    font_sheet: ResMut<ShareFontSheet>,
    mut event_writer: EventWriter<ComponentEvent<Changed<SvgInnerContent>>>,
    r: OrInitRes<IsRun>,
    await_list: Local<SvgShapeAwaitList>,
    // query_view_box: Query<&SvgContent>,
) {
    if r.0 {
        return;
    }
    // println!("=========1text_svg");
    let mut font_sheet = font_sheet.borrow_mut();

    let mut await_set_gylph = Vec::new();
    // let
    for (entity, mut node_state) in query.p0().iter_mut() {
        if node_state.shape.is_ready() {
            // if let Some(shape) = node_state.shape.take() {
            let hash = node_state.shape.hash();
            let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;
            let mut map = await_list.1.lock().unwrap();
            if map.get(&hash).is_none() {
                map.insert(hash, 0);
                await_set_gylph.push(entity);
                log::debug!("add_shape!! hash: {}", hash);
                match node_state.shape.clone() {
                    Shape::Rect { x, y, width, height } => sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Rect::new(x, y, width, height))),
                    Shape::Circle { cx, cy, radius } => sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Circle::new(cx, cy, radius).unwrap())),
                    Shape::Ellipse { cx, cy, rx, ry } => sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Ellipse::new(cx, cy, rx, ry))),
                    Shape::Segment { ax, ay, bx, by } => {
                        sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Segment::new(Point2::new(ax, ay), Point2::new(bx, by))))
                    }
                    Shape::Polygon { points } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Polygon::new(points)))
                    }
                    Shape::Polyline { points } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Polyline::new(points)))
                    }
                    Shape::Path { points, verb } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(hash, Box::new(pi_sdf::shape::Path::new(verb, points)))
                    }
                };
            }
            node_state.hash = hash;
            // }
        }
    }

    // 如果是sdf2， 则设置就绪字形对应节点的NodeState的修改版本
    // if let FontType::Sdf2 = font_type {
    if await_set_gylph.len() > 0 {
        let list = (*await_list).0.clone();
        let cur_await = font_sheet.draw_sdf_await();
        MULTI_MEDIA_RUNTIME
            .spawn(async move {
                let r = cur_await.await;
                println!("draw_sdf_await");
                list.lock().unwrap().push((await_set_gylph, r));
            })
            .unwrap();
    }

    let mut p2 = query.p1();
    for (await_set_gylph, result) in await_list.0.lock().unwrap().drain(..) {
        println!("update_svg_sdf2, await_set_gylph: {:?}", await_set_gylph);
        font_sheet.update_svg_sdf2(result); // 更新纹理
        for entity in await_set_gylph.iter() {
            if let Ok(mut node_state) = p2.get_mut(*entity) {
                node_state.set_changed();
            }
            event_writer.send(ComponentEvent::<Changed<SvgInnerContent>>::new(*entity));
        }
        log::debug!("await_set_gylph================{:?}", await_set_gylph);
    }
    // }
}


/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_text(
    mut instances: OrInitResMut<InstanceContext>,
    query: Query<(Entity, Ref<WorldMatrix>, Ref<SvgInnerContent>, Ref<LayoutResult>, &DrawList, &Layer), Changed<SvgInnerContent>>,
    mut query_draw: Query<&InstanceIndex, With<SvgMark>>,
    r: OrInitRes<IsRun>,
    render_type: OrInitRes<SvgRenderObjType>,
    font_sheet: ResMut<ShareFontSheet>,
) {
    if r.0 {
        return;
    }
    // log::debug!("calc_sdf2_text1");
    let render_type = ***render_type;

    let mut font_sheet = font_sheet.borrow_mut();

    let mut i = 0;
    for (entity, world_matrix, node_state, layout, draw_list, layer) in query.iter() {
        log::debug!("calc_sdf2_text2");
        let draw_id = match draw_list.get_one(render_type) {
            Some(r) => r.id,
            None => continue,
        };
        log::debug!("calc_sdf2_text211111");
        if let Ok(instance_index) = query_draw.get_mut(draw_id) {
            log::debug!("calc_sdf2_text22, instance_index.0.start,{}", instance_index.0.start);
            // 节点可能设置为dispaly none， 此时instance_index可能为Null
            if pi_null::Null::is_null(&instance_index.0.start) {
                continue;
            }
            log::debug!("calc_sdf2_text222");
            if layer.layer() == 0 {
                continue;
            }
            log::debug!("calc_sdf2_text3");
            let mut _n = entity;
            let mut _state = &*node_state;
            let matrix = &*world_matrix;

            let is_added = node_state.is_changed();

            let (text_style_change, text_style) = (is_added, &node_state.style); // TextStyle组件在设计上不会被删除， 当TextStyle为None时， TextStyle一定没有改变过

            let font_type = font_sheet.font_mgr().font_type;
            let tex_info = match font_type {
                FontType::Bitmap => todo!(),
                FontType::Sdf1 => todo!(),
                FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.shapes.get(&node_state.hash).unwrap(),
            };

            let instance_data = instance_data(
                text_style_change,
                is_added,
                world_matrix.is_changed(),
                text_style,
                tex_info,
                matrix.clone(),
            );

            text_vert(
                &layout,
                tex_info,
                &node_state.style,
                entity,
                instance_data,
                instance_index.clone(),
                &mut instances.instance_data,
            );
        }
        log::debug!("instance: {}", i);
        i += 1;
    }
}

#[inline]
fn instance_data(
    is_style_change: bool,
    is_content_change: bool,
    is_matrix_change: bool,
    svg_style: &SvgStyle,
    tex_info: &TexInfo,
    world_matrix: WorldMatrix,
) -> UniformData {
    let stroke = if *svg_style.stroke.width > 0.0 {
        [
            svg_style.stroke.color.x,
            svg_style.stroke.color.y,
            svg_style.stroke.color.z,
            *svg_style.stroke.width,
        ]
    } else {
        [0.0, 0.0, 0.0, *svg_style.stroke.width]
    };
    let stroke_dasharray = [
        svg_style.stroke_dasharray.start_x,
        svg_style.stroke_dasharray.start_y,
        svg_style.stroke_dasharray.real,
        svg_style.stroke_dasharray.empty,
    ];
    let shadow_color = [
        svg_style.shadow.color.x,
        svg_style.shadow.color.y,
        svg_style.shadow.color.z,
        svg_style.shadow.color.w,
    ];
    let shadow_offset = [svg_style.shadow.offset_x, svg_style.shadow.offset_y];
    let shadow_blur_level = svg_style.shadow.blur_level;
    // let weight = [-0.0];

    match &svg_style.fill_color {
        // 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
        Color::RGBA(color) => UniformData {
            stroke,
            stroke_dasharray,
            is_style_change,
            is_content_change,
            is_matrix_change,
            font_style: FontStyle::Normal,
            color: ColorData::Rgba([color.x, color.y, color.z, color.w]),
            world_matrix,
            shadow_color,
            shadow_offset,
            outer_glow_color_and_dist: svg_style.outer_glow_color_and_dist,
            shadow_blur_level,
        },
        // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
        Color::LinearGradient(color) => {
            // TODO， 渐变端点
            let mut colors: [f32; 12] = [0.0; 12];
            let mut positions: [f32; 4] = [1.0; 4];
            if color.list.len() > 0 {
                for i in 0..4 {
                    match color.list.get(i) {
                        Some(r) => {
                            positions[i] = r.position;
                            let j = i * 3;
                            colors[j] = r.rgba.x;
                            colors[j + 1] = r.rgba.y;
                            colors[j + 2] = r.rgba.z;
                            // colors[j + 3] = r.rgba.w;
                        }
                        None => {
                            positions[i] = 1.0;
                            let j = i * 3;
                            colors[j] = colors[j - 3];
                            colors[j + 1] = colors[j - 2];
                            colors[j + 2] = colors[j - 1];
                            // colors[j + 3] = colors[j - 1];
                        }
                    }
                }
            }
            let binding_box = tex_info.binding_box;
            log::trace!(
                "sdf2 LinearGradient======; color: {:?}, positions: {:?}, binding_box: {:?}",
                color,
                positions,
                binding_box
            );
            let normalize_direction = Vector2::new(color.direction.cos(), color.direction.sin());
            let r = [
                Vector2::new(binding_box.mins.x, binding_box.mins.y).dot(&normalize_direction),
                Vector2::new(binding_box.mins.x, binding_box.maxs.y).dot(&normalize_direction),
                Vector2::new(binding_box.maxs.x, binding_box.mins.y).dot(&normalize_direction),
                Vector2::new(binding_box.maxs.x, binding_box.maxs.y).dot(&normalize_direction),
            ];
            let (min, max) = (r[0].min(r[1]).min(r[2]).min(r[3]), r[0].max(r[1]).max(r[2]).max(r[3]));
            let end = (normalize_direction * min, normalize_direction * max);
            let end = [end.0.x, end.0.y, end.1.x, end.1.y];
            // let end = [20.0, 0.0, 120.0, 0.0];
            // log::trace!("sdf2 layout {:?}",layout);
            log::trace!(
                "sdf2 LinearGradient======{:?}, {:?}, {:?}, {:?}, {:?}",
                normalize_direction,
                r,
                min,
                max,
                end,
            );

            println!("LinearGradient");
            UniformData {
                stroke,
                stroke_dasharray,
                is_style_change,
                is_content_change,
                is_matrix_change,
                font_style: FontStyle::Normal,
                color: ColorData::LinearGradient { colors, positions, end },
                world_matrix,
                shadow_color,
                shadow_offset,
                outer_glow_color_and_dist: svg_style.outer_glow_color_and_dist,
                shadow_blur_level,
            }
        }
    }
}

#[allow(unused_variables)]
fn text_vert(
    layout: &LayoutResult,
    tex_info: &TexInfo,
    svg_style: &SvgStyle,
    entity: Entity,
    uniform_data: UniformData,
    instance_index: InstanceIndex,
    instances: &mut GpuBuffer,
) {
    let font_size = 1.0;
    let line_height = 1.0;
    // let text_style = &svg_content.style;

    let word_pos = (0.0, 0.0);
    let offset = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
    let count = 0;
    let half_stroke = *svg_style.stroke.width / 2.0;


    let line_max = 0.0;

    let mut cur_instance_index = instance_index.0.start;


    // let face_id = fontface_ids[font_sheet.font_mgr().table.sdf2_table.glyphs[c1.ch_id].font_face_index];
    let extents = &tex_info.extents;
    // let offset_y = (line_height - font_height) / 2.0;
    uniform_data.set_data(instances.instance_data_mut(cur_instance_index), tex_info, extents, font_size);
    // left += c1.width + text_style.letter_spacing;
    cur_instance_index = instances.next_index(cur_instance_index);


    if cur_instance_index > instance_index.0.end {
        panic!(
            "text len error, cur_instance_index: {}, instance_index: {:?}",
            cur_instance_index, &instance_index.0,
        );
    }
}


struct UniformData {
    stroke: [f32; 4],
    stroke_dasharray: [f32; 4],
    shadow_color: [f32; 4],
    shadow_offset: [f32; 2],
    outer_glow_color_and_dist: [f32; 4],
    shadow_blur_level: f32,
    is_style_change: bool,
    is_content_change: bool,
    is_matrix_change: bool,
    font_style: FontStyle,
    color: ColorData,
    world_matrix: WorldMatrix,
}

enum ColorData {
    Rgba([f32; 4]),
    LinearGradient { colors: [f32; 12], positions: [f32; 4], end: [f32; 4] },
}

impl UniformData {
    #[inline]
    fn set_data(&self, mut instance_data: InstanceData, tex_info: &TexInfo, extents: &Aabb2, font_size: f32) {
        // println!(
        //     "set_data===================={:?}, {:?}, offset={:?}, font_size={}",
        //     instance_data, tex_info, offset, font_size
        // );
        let mut render_flag = instance_data.get_render_ty();
        render_flag |= 1 << RenderFlagType::Sdf2 as usize;
        render_flag |= 1 << RenderFlagType::Svg as usize;

        // let width = extents.width();
        // let height = extents.height();
        let mut extents = extents.clone();
        if self.is_style_change {
            log::debug!("stroke: {:?}", self.stroke);
            instance_data.set_data(&TextOutlineUniform(&self.stroke));
            instance_data.set_data(&TextWeightUniform(&[0.0]));
            if self.stroke_dasharray[2] < 100000. && self.stroke_dasharray[3] > 0. {
                let start_pos = self
                    .world_matrix
                    .transform_point(&Point3::new(self.stroke_dasharray[0], self.stroke_dasharray[1], 0.0));
                let step = [
                    self.world_matrix[0] * self.stroke_dasharray[2],
                    self.world_matrix[5] * self.stroke_dasharray[3],
                ];
                let stroke_dasharray = [start_pos.x, start_pos.y, step[0], step[1]];
                log::debug!("set stroke_dasharray: {:?}", stroke_dasharray);
                instance_data.set_data(&TextOuterGlowUniform(&stroke_dasharray));
                render_flag |= 1 << RenderFlagType::SvgStrokeDasharray as usize;
                render_flag &= !(1 << RenderFlagType::Sdf2OutGlow as usize);
                render_flag &= !(1 << RenderFlagType::Shadow as usize);
            } else if self.shadow_color[3] > 0.0 {
                log::debug!("set shadow_color: {:?}", self.shadow_color);
                instance_data.set_data(&TextOuterGlowUniform(&self.shadow_color));
                if self.shadow_offset[0] > 0.0{
                    extents.maxs.x = extents.maxs.x + self.shadow_offset[0];
                }else{
                    extents.mins.x = extents.mins.x + self.shadow_offset[0];
                }

                if self.shadow_offset[1] > 0.0{
                    extents.maxs.y = extents.maxs.y + self.shadow_offset[1];
                }else{
                    extents.mins.y = extents.mins.y + self.shadow_offset[1];
                }
                log::debug!("set shadow_offset_and_blur_level: {}, {}", self.shadow_offset[0] - extents.mins.x,  extents.width());
                let shadow_offset_and_blur_level = [(self.shadow_offset[0])  / extents.width(), (self.shadow_offset[1] ) / extents.height(), self.shadow_blur_level];
                log::debug!("set shadow_offset_and_blur_level: {:?}", shadow_offset_and_blur_level);
                instance_data.set_data(&ShadowUniform(&shadow_offset_and_blur_level));
                render_flag |= 1 << RenderFlagType::Shadow as usize;
                render_flag &= !(1 << RenderFlagType::Sdf2OutGlow as usize);
                render_flag &= !(1 << RenderFlagType::SvgStrokeDasharray as usize);
            } else if !self.outer_glow_color_and_dist[3].is_infinite() {
                log::debug!("set outer_glow_color_and_dist: {:?}", self.outer_glow_color_and_dist);
                instance_data.set_data(&TextOuterGlowUniform(&self.outer_glow_color_and_dist));
                render_flag |= 1 << RenderFlagType::Sdf2OutGlow as usize;
                render_flag &= !(1 << RenderFlagType::Shadow as usize);
                render_flag &= !(1 << RenderFlagType::SvgStrokeDasharray as usize);
            }
            log::debug!("index: {:?}", instance_data.index());
            match &self.color {
                ColorData::Rgba(r) => {
                    render_flag |= 1 << RenderFlagType::Color as usize;
                    render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
                    log::debug!("color: {:?}", r);


                    instance_data.set_data(&ColorUniform(r))
                }
                ColorData::LinearGradient { colors, positions, end } => {
                    render_flag |= 1 << RenderFlagType::LinearGradient as usize;
                    render_flag &= !(1 << RenderFlagType::Color as usize);
                    log::debug!("LinearGradient color: {:?}, positions: {:?}, end: {:?}", colors, positions, end);
                    instance_data.set_data(&GradientColorUniform(colors));
                    instance_data.set_data(&GradientPositionUniform(positions));
                    instance_data.set_data(&GradientEndUniform(end));
                }
            }
        }


        if self.is_style_change || self.is_content_change || self.is_matrix_change {
            let (mut scope_factor, mut scope_y) = (0.0, 0.0);
            if self.font_style == FontStyle::Oblique {
                scope_y = -extents.mins.y * font_size; // 基线位置的y
                scope_factor = 0.35;
            }

            // sdf信息[max_offset, min_sdf, sdf_step, check, index_offset_x, index_offset_y, index_w, index_h, data_offset_x, data_offset_y, scope_factor, scope_y]
            let data = [
                tex_info.max_offset as f32,
                tex_info.min_sdf as f32,
                tex_info.sdf_step as f32,
                tex_info.cell_size * 0.5 * 2.0f32.sqrt(),
                tex_info.index_offset.0 as f32,
                tex_info.index_offset.1 as f32,
                tex_info.grid_w,
                tex_info.grid_w,
                tex_info.data_offset.0 as f32,
                tex_info.data_offset.1 as f32,
                scope_factor,
                scope_y,
            ];
            instance_data.set_data(&Sdf2InfoUniform(&data));

            // 设置文字在布局空间的偏移和宽高
            // instance_data.set_data(&BoxUniform(&[offset.0, offset.1, (render_range.maxs.x - render_range.mins.x) * font_size, (render_range.maxs.y - render_range.mins.y) * font_size]));
            // log::debug!("view_box: {:?}", render_range);
            // let matrix = self.world_matrix.scale(1.0).translate(x, y, z)

            let rect = extents;
            log::debug!("set_box: {:?}, world_matrix: {:?}", rect, self.world_matrix);
            set_box(&self.world_matrix, &rect, &mut instance_data);

            // 设置渲染类型
            instance_data.set_data(&TyUniform(&[render_flag as f32]));
        }
    }
}
