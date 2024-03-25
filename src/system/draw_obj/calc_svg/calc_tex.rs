//! 文字字形系统
//! 为字符分配纹理位置，得到字符的位置索引关联到CharNode中的ch_id_or_count字段上
//! 在fontsheet中，文字最多缓存一张纹理。为字符分配纹理，可能存在空间不足的情况。此时，本系统将清空fontsheet中所有缓存的字符，并重新为当前所有显示节点上的文字重新绘制纹理。
use crate::{
    components::{
        calc::{DrawList, WorldMatrix},
        draw_obj::{InstanceIndex, SvgMark},
    },
    resource::SvgRenderObjType,
    shader1::{
        meterial::{
            ColorUniform, GradientColorUniform, GradientEndUniform, GradientPositionUniform, RenderFlagType, Sdf2InfoUniform, TextOuterGlowUniform,
            TextOutlineUniform, TextWeightUniform, TyUniform,
        },
        InstanceData, RenderInstances,
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
    },
};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_hal::{font::sdf2_table::TexInfo, pi_sdf, runtime::MULTI_MEDIA_RUNTIME};

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

pub struct SvgShapeAwaitList(pub Share<ShareMutex<Vec<(Vec<Entity>, Share<ShareMutex<(usize, Vec<(u64, TexInfo, Vec<u8>, Vec<u8>)>)>>)>>>);

impl Default for SvgShapeAwaitList {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Vec::new()))) }
}


/// 更新sdf2的纹理
pub fn update_sdf2_texture(
    mut instances: OrInitResMut<InstanceContext>,
    font_sheet: ResMut<ShareFontSheet>,
    device: Res<PiRenderDevice>,
    common_sampler: Res<crate::resource::draw_obj::CommonSampler>,
) {
    let font_sheet = font_sheet.0.borrow();
    if let (Some(sdf2_index_texture_view), Some(sdf2_data_texture_view)) = (&font_sheet.sdf2_index_texture_view, &font_sheet.sdf2_data_texture_view) {
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
                        resource: wgpu::BindingResource::TextureView(&sdf2_data_texture_view.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
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
    mut query: ParamSet<(Query<(Entity, &'static mut SvgContent)>, Query<&mut SvgContent>)>,
    font_sheet: ResMut<ShareFontSheet>,
    mut event_writer: EventWriter<ComponentEvent<Changed<SvgContent>>>,
    r: OrInitRes<IsRun>,
    await_list: Local<SvgShapeAwaitList>,
) {
    if r.0 {
        return;
    }
    // println!("=========1text_svg");
    let mut font_sheet = font_sheet.borrow_mut();

    let mut await_set_gylph = Vec::new();
    for (entity, mut node_state) in query.p0().iter_mut() {
        if node_state.shape.is_some() {
            if let Some(shape) = node_state.shape.take() {
                await_set_gylph.push(entity);
                println!("add_shape");
                let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;
                let hash = match shape {
                    Shape::Rect { x, y, width, height } => sdf2_table.add_shape(Box::new(pi_sdf::shape::Rect::new(x, y, width, height))),
                    Shape::Circle { cx, cy, radius } => sdf2_table.add_shape(Box::new(pi_sdf::shape::Circle::new(cx, cy, radius).unwrap())),
                    Shape::Ellipse { cx, cy, rx, ry } => sdf2_table.add_shape(Box::new(pi_sdf::shape::Ellipse::new(cx, cy, rx, ry))),
                    Shape::Segment { ax, ay, bx, by } => {
                        sdf2_table.add_shape(Box::new(pi_sdf::shape::Segment::new(Point2::new(ax, ay), Point2::new(bx, by))))
                    }
                    Shape::Polygon { points } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(Box::new(pi_sdf::shape::Polygon::new(points)))
                    }
                    Shape::Polyline { points } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(Box::new(pi_sdf::shape::Polyline::new(points)))
                    }
                    Shape::Path { points, verb } => {
                        let points = points.into_iter().map(|v| Point2::new(v[0], v[1])).collect::<Vec<Point2>>();
                        sdf2_table.add_shape(Box::new(pi_sdf::shape::Path::new(verb, points)))
                    }
                };
                node_state.hash = hash;
            }
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
            event_writer.send(ComponentEvent::<Changed<SvgContent>>::new(*entity));
        }
        log::debug!("await_set_gylph================{:?}", await_set_gylph);
    }
    // }
}


/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_text(
    mut instances: OrInitResMut<InstanceContext>,
    query: Query<(Entity, Ref<WorldMatrix>, Ref<SvgContent>, Ref<LayoutResult>, &DrawList, &Layer), Changed<SvgContent>>,
    mut query_draw: Query<&InstanceIndex, With<SvgMark>>,
    r: OrInitRes<IsRun>,
    render_type: OrInitRes<SvgRenderObjType>,
    font_sheet: ResMut<ShareFontSheet>,
) {
    if r.0 {
        return;
    }
    // println!("calc_sdf2_text1");
    let render_type = ***render_type;

    let mut font_sheet = font_sheet.borrow_mut();


    for (entity, world_matrix, node_state, layout, draw_list, layer) in query.iter() {
        // println!("calc_sdf2_text2");
        let draw_id = match draw_list.get_one(render_type) {
            Some(r) => r.id,
            None => continue,
        };
        // println!("calc_sdf2_text211111");
        if let Ok(instance_index) = query_draw.get_mut(draw_id) {
            // println!("calc_sdf2_text22");
            // 节点可能设置为dispaly none， 此时instance_index可能为Null
            if pi_null::Null::is_null(&instance_index.0.start) {
                continue;
            }
            // println!("calc_sdf2_text222");
            if layer.layer() == 0 {
                continue;
            }
            // println!("calc_sdf2_text3");
            let mut _n = entity;
            let mut _state = &*node_state;
            let matrix = &*world_matrix;

            let is_added = node_state.is_changed();

            let (text_style_change, text_style) = (is_added, &node_state.style); // TextStyle组件在设计上不会被删除， 当TextStyle为None时， TextStyle一定没有改变过

            let instance_data = instance_data(text_style_change, is_added, world_matrix.is_changed(), text_style, matrix.clone());

            text_vert(
                &node_state,
                &layout,
                &mut font_sheet,
                &node_state,
                entity,
                instance_data,
                instance_index.clone(),
                &mut instances.instance_data,
            );
        }
    }
}

#[inline]
fn instance_data(
    is_style_change: bool,
    is_content_change: bool,
    is_matrix_change: bool,
    svg_style: &SvgStyle,
    // layout: &LayoutResult,
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
        },
        // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
        Color::LinearGradient(color) => {
            // TODO， 渐变端点
            let mut colors: [f32; 16] = [0.0; 16];
            let mut positions: [f32; 4] = [1.0; 4];
            if color.list.len() > 0 {
                for i in 0..4 {
                    match color.list.get(i) {
                        Some(r) => {
                            positions[i] = r.position;
                            let j = i * 4;
                            colors[j] = r.rgba.x;
                            colors[j + 1] = r.rgba.y;
                            colors[j + 2] = r.rgba.z;
                            colors[j + 3] = r.rgba.w;
                        }
                        None => {
                            positions[i] = 1.0;
                            let j = i * 4;
                            colors[j] = colors[j - 4];
                            colors[j + 1] = colors[j - 3];
                            colors[j + 2] = colors[j - 2];
                            colors[j + 3] = colors[j - 1];
                        }
                    }
                }
            }
            // log::trace!("sdf2 LinearGradient======{:?}, {:?}", color, positions);
            // let normalize_direction = Vector2::new(color.direction.cos(), color.direction.sin());
            // let r = [
            //     Vector2::new(layout.border.left, layout.border.top).dot(&normalize_direction),
            //     Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.border.top).dot(&normalize_direction),
            //     Vector2::new(
            //         layout.rect.right - layout.border.right - layout.rect.left,
            //         layout.rect.bottom - layout.border.bottom - layout.rect.top,
            //     )
            //     .dot(&normalize_direction),
            //     Vector2::new(layout.border.left, layout.rect.bottom - layout.border.bottom - layout.rect.top).dot(&normalize_direction),
            // ];
            // let (min, max) = (r[0].min(r[1]).min(r[2]).min(r[3]), r[0].max(r[1]).max(r[2]).max(r[3]));
            // let end = (normalize_direction * min, normalize_direction * max);
            // let end = [end.0.x, end.0.y, end.1.x, end.1.y];

            // log::trace!(
            //     "sdf2 LinearGradient======{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            //     normalize_direction,
            //     r,
            //     min,
            //     max,
            //     end,
            //     [
            //         Vector2::new(layout.border.left, layout.border.top),
            //         Vector2::new(layout.rect.right - layout.border.right - layout.rect.left, layout.border.top),
            //         Vector2::new(
            //             layout.rect.right - layout.border.right - layout.rect.left,
            //             layout.rect.bottom - layout.border.bottom - layout.rect.top
            //         ),
            //         Vector2::new(layout.border.left, layout.rect.bottom - layout.border.bottom - layout.rect.top),
            //     ]
            // );

            println!("LinearGradient");
            UniformData {
                stroke,
                stroke_dasharray,
                is_style_change,
                is_content_change,
                is_matrix_change,
                font_style: FontStyle::Normal,
                color: ColorData::LinearGradient {
                    colors,
                    positions,
                    end: [0.0, 0.0, 0.0, 0.0],
                },
                world_matrix,
            }
        }
    }
}

#[allow(unused_variables)]
fn text_vert(
    node_state: &SvgContent,
    layout: &LayoutResult,
    font_sheet: &mut FontSheet,
    svg_content: &SvgContent,
    entity: Entity,
    uniform_data: UniformData,
    instance_index: InstanceIndex,
    instances: &mut RenderInstances,
) {
    let font_size = 1.0;
    let line_height = 1.0;
    let text_style = &svg_content.style;
    let font_type = font_sheet.font_mgr().font_type;
    let word_pos = (0.0, 0.0);
    let offset = (layout.border.left + layout.padding.left, layout.border.top + layout.padding.top);
    let count = 0;
    let half_stroke = *text_style.stroke.width / 2.0;


    let line_max = 0.0;

    let mut cur_instance_index = instance_index.0.start;

    let tex_info = match font_type {
        FontType::Bitmap => todo!(),
        FontType::Sdf1 => todo!(),
        FontType::Sdf2 => font_sheet.font_mgr().table.sdf2_table.shapes.get(&svg_content.hash).unwrap(),
    };

    // let face_id = fontface_ids[font_sheet.font_mgr().table.sdf2_table.glyphs[c1.ch_id].font_face_index];
    let render_range = &font_sheet.font_mgr().table.sdf2_table.svg.view_box;
    // let offset_y = (line_height - font_height) / 2.0;
    uniform_data.set_data(
        instances.instance_data_mut(cur_instance_index),
        tex_info,
        render_range,
        (0.0, 0.0),
        font_size,
    );
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
    is_style_change: bool,
    is_content_change: bool,
    is_matrix_change: bool,
    font_style: FontStyle,
    color: ColorData,
    world_matrix: WorldMatrix,
}

enum ColorData {
    Rgba([f32; 4]),
    LinearGradient { colors: [f32; 16], positions: [f32; 4], end: [f32; 4] },
}

impl UniformData {
    #[inline]
    fn set_data(&self, mut instance_data: InstanceData, tex_info: &TexInfo, render_range: &Aabb2, offset: (f32, f32), font_size: f32) {
        // println!(
        //     "set_data===================={:?}, {:?}, offset={:?}, font_size={}",
        //     instance_data, tex_info, offset, font_size
        // );
        let mut render_flag = instance_data.get_render_ty();
        render_flag |= 1 << RenderFlagType::Sdf2 as usize;
        render_flag |= 1 << RenderFlagType::Svg as usize;

        if self.is_style_change {
            println!("stroke: {:?}", self.stroke);
            instance_data.set_data(&TextOutlineUniform(&self.stroke));
            instance_data.set_data(&TextWeightUniform(&[0.0]));
            if self.stroke_dasharray[2] < 100000. && self.stroke_dasharray[3] > 0. {
                println!("set stroke_dasharray: {:?}", self.stroke_dasharray);
                instance_data.set_data(&TextOuterGlowUniform(&self.stroke_dasharray));
                render_flag |= 1 << RenderFlagType::SvgStrokeDasharray as usize;
                render_flag &= !(1 << RenderFlagType::Sdf2OutGlow as usize);
            }
            match &self.color {
                ColorData::Rgba(r) => {
                    render_flag |= 1 << RenderFlagType::Color as usize;
                    render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
                    println!("color: {:?}", r);
                    instance_data.set_data(&ColorUniform(r))
                }
                ColorData::LinearGradient { colors, positions, end } => {
                    render_flag |= 1 << RenderFlagType::LinearGradient as usize;
                    render_flag &= !(1 << RenderFlagType::Color as usize);

                    instance_data.set_data(&GradientColorUniform(colors));
                    instance_data.set_data(&GradientPositionUniform(positions));
                    instance_data.set_data(&GradientEndUniform(end));
                }
            }
        }


        if self.is_style_change || self.is_content_change || self.is_matrix_change {
            let (mut scope_factor, mut scope_y) = (0.0, 0.0);
            if self.font_style == FontStyle::Oblique {
                scope_y = -render_range.mins.y * font_size; // 基线位置的y
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
            println!("view_box: {:?}", render_range);
            let rect = Aabb2::new(
                Point2::new(offset.0, offset.1),
                Point2::new(
                    (render_range.maxs.x - render_range.mins.x) + offset.0,
                    (render_range.maxs.y - render_range.mins.y) + offset.1,
                ),
            );
            println!("set_box: {:?}， world_matrix: {:?}", rect, self.world_matrix);
            set_box(&self.world_matrix, &rect, &mut instance_data);

            // 设置渲染类型
            instance_data.set_data(&TyUniform(&[render_flag as f32]));
        }
    }
}
