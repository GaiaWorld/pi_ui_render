//! sdf2文字功能
use crate::system::draw_obj::calc_text::text_sdf2::FboBindGroups;
use std::collections::hash_map::Entry;

use pi_bevy_ecs_extend::prelude::{Down, Layer, OrInitSingleRes, OrInitSingleResMut, Up};
use pi_bevy_render_plugin::{
    node::{Node, NodeId as GraphNodeId, ParamUsage},
    PiRenderDevice, PiRenderGraph, PiSafeAtlasAllocator, RenderContext,
};
use pi_flex_layout::prelude::{CharNode, Rect};
use pi_futures::BoxFuture;
use pi_hal::{font::sdf2_table::sdf_font_size, svg::SvgInfo};

use pi_hal::font::{font::FontType, sdf2_table::SvgTexInfo};
use pi_hash::XHashMap;
use pi_null::Null;
use pi_polygon::mult_to_triangle;
use pi_share::{Share, ShareRefCell};
use pi_slotmap::KeyData;
use pi_world::filter::Or;
use pi_world::prelude::{Changed, Entity, Plugin, Query, SingleResMut, With};
use pi_world::{event::ComponentRemoved, system_params::Local, world::FromWorld};
// use pi_hal::pi_sdf::glyphy::geometry::aabb::AabbEXT;
use pi_render::{
    components::view::target_alloc::{Fbo, ShareTargetView},
    font::{FontSheet, Glyph, GlyphId},
};
use pi_style::style::{Aabb2, CgColor, ColorAndPosition, LinearGradientColor, Point2, StyleType, TextOverflow};
use pi_world::single_res::SingleRes;
use pi_world::system_params::SystemParam;
use wgpu::{BindGroupLayout, CommandEncoder, RenderPass, Sampler};

use crate::components::user::{SvgColor, SvgLinearGradient, SvgLinearGradientStop, SvgShadowNode};
use crate::resource::{GlobalDirtyMark, IsRun, OtherDirtyType, ShareFontSheet, TextRenderObjType};
use crate::shader::ui_meterial::ColorUniform;
use crate::shader1::batch_meterial::RenderFlagType;


use crate::components::user::{serialize::SCG_STYLE_START, SvgFilter, TextContent, TextOverflowData, TextStyle};
use crate::system::draw_obj::geo_split::OtherInfo;
use crate::{
    components::draw_obj::{BoxType, PolygonType, RenderCount, SvgMark, TempGeo, TextMark, VColor},
    resource::SvgRenderObjType,
};
use crate::{
    components::user::{serialize::SvgType, Color, SvgShadow},
    resource::{SvgOuterGlowRenderObjType, SvgShadowRenderObjType},
};
use crate::{
    components::{
        calc::{style_bit, LayoutResult, NodeState, StyleBit, StyleMarkType, LAYOUT_DIRTY},
        draw_obj::{InstanceSplit, TempGeoBuffer, TextOuterGlowMark, TextShadowMark},
        pass_2d::InstanceDrawState,
        root::DynTargetType,
        user::SvgInnerContent,
    },
    resource::{draw_obj::RenderState, TextOuterGlowRenderObjType, TextShadowRenderObjType},
    shader1::{
        batch_gauss_blur::{BatchGussMeterial, GaussDirecition},
        batch_meterial::{LayoutUniform, TyMeterial, UvUniform},
        batch_sdf_glow::BatchGlowMeterial,
        batch_sdf_gray::BatchGrayMeterial,
    },
    system::draw_obj::root_view_port::create_dyn_target_type,
};

// use super::{text_glyph::text_glyph, TEXT_OUTER_GLOW_ORDER, TEXT_SHADOW_ORDER};
// use super::TEXT_ORDER;
// use super::text_split::text_split;

use crate::components::calc::DrawList;
use crate::components::draw_obj::InstanceIndex;
use crate::resource::draw_obj::InstanceContext;

/// 使用sdf2的方式渲染文字
pub struct Sdf2TextPlugin;

impl Plugin for Sdf2TextPlugin {
    fn build(&self, _app: &mut pi_world::prelude::App) {
        // let font_sheet = ShareFontSheet::new(&mut app.world, FontType::Sdf2);
        // app.world.insert_single_res(font_sheet);
        // app
        //     .add_startup_system(UiStage, init_text_effect_graph)
        //     // 文字劈分为字符
        //     .add_system(UiStage, text_split
        // 		.before(calc_layout)
        // 		.in_set(UiSystemSet::Layout)
        // 		.in_schedule(UiSchedule::Layout)
        // 		.in_schedule(UiSchedule::Calc)
        // 		.in_schedule(UiSchedule::Geo)
        // 		.run_if(text_layout_change))
        //     // 字形计算
        //     .add_system(UiStage, text_glyph
        // 		.after(text_split)
        // 		.in_set(UiSystemSet::Layout)
        // 		.before(update_sdf2_texture)
        // 		.in_schedule(UiSchedule::Calc)
        // 	)
        // 	// 创建drawobj
        // 	.add_system(
        // 		UiStage,
        // 		draw_object_life_new::<
        // 				TextContent,
        // 				TextRenderObjType,
        // 				(TextMark, RenderCount),
        // 				{ TEXT_ORDER },
        // 				{ BoxType::None },>
        // 				.in_set(UiSystemSet::LifeDrawObject)
        // 				.run_if(text_content_change)
        // 				.before(calc_sdf2_text_len),
        // 	)
        // 	// 为阴影创建drawObj
        // 	.add_system(
        // 		UiStage,
        // 		draw_object_life_new::<
        // 				TextShadow,
        // 				TextShadowRenderObjType,
        // 				TextShadowMark,
        // 				{ TEXT_SHADOW_ORDER },
        // 				{ BoxType::None },>
        // 				.in_set(UiSystemSet::LifeDrawObject)
        // 				.run_if(text_shadow_change)
        // 				.before(calc_sdf2_text_len),
        // 	)
        // 	// 为outerglow 创建drawObj
        // 	.add_system(
        // 		UiStage,
        // 		draw_object_life_new::<
        // 				TextOuterGlow,
        // 				TextOuterGlowRenderObjType,
        // 				TextOuterGlowMark,
        // 				{ TEXT_OUTER_GLOW_ORDER },
        // 				{ BoxType::None },>
        // 				.in_set(UiSystemSet::LifeDrawObject)
        // 				.run_if(text_outer_glow_change)
        // 				.before(calc_sdf2_text_len),
        // 	)

        // 	// 统计drawobj的实例长度（文字包含多个字符，每个字符一个实例， 并且可能包含多层阴影， 每阴影每字符也需要一个实例）
        // 	// 由于当前一个文字实例可附带渲染一个阴影，因此最终的实例个数为`text.len() * (shadow.len() > 1? shadow.len() - 1: 1)`个实例
        // 	.add_system(
        // 		UiStage,
        // 		calc_sdf2_text_len

        // 			.after(UiSystemSet::LifeDrawObjectFlush)
        // 			.before(update_render_instance_data)
        // 			.after(calc_layout)
        // 			.run_if(text_len_change)
        // 	)
        // 	// 更新实例数据
        // 	.add_system(
        // 		UiStage,
        // 		calc_sdf2_text
        // 			.in_set(UiSystemSet::PrepareDrawObj)
        // 			.run_if(text_change)
        // 	)
        // ;
    }
}

lazy_static! {
    pub static ref SVG_DIRTY: StyleMarkType = SVG_LEN_DIRTY.clone();
    pub static ref SVG_LEN_DIRTY: StyleMarkType = SVG_LAYOUT_DIRTY.clone()
        | LAYOUT_DIRTY
            .set_bit(OtherDirtyType::NodeTreeAdd as usize)
            .set_bit(SvgType::SvgColor as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgStrokeColor as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgStrokeWidth as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgShadowBlurLevel as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgShadowOffsetX as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgShadowColor as usize + SCG_STYLE_START)
            .set_bit(SvgType::SvgShadowOffsetY as usize + SCG_STYLE_START);
    pub static ref SVG_LAYOUT_DIRTY: StyleMarkType = style_bit()
        .set_bit(SvgType::SvgShapeAX as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeAY as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeBX as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeBY as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeCX as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeCY as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeHeight as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeWidth as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeRadius as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeRadiusX as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeRadiusY as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapePath as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapePoints as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShape as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapePoints as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeX as usize + SCG_STYLE_START)
        .set_bit(SvgType::SvgShapeY as usize + SCG_STYLE_START)
        .set_bit(OtherDirtyType::NodeTreeAdd as usize);
}

pub fn text_layout_change(mark: SingleRes<GlobalDirtyMark>) -> bool { mark.mark.has_any(&*SVG_LAYOUT_DIRTY) }

// pub fn text_len_change(mark: SingleRes<GlobalDirtyMark>) -> bool { mark.mark.has_any(&*TEXT_LEN_DIRTY) }
pub fn svg_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
    let r = mark.mark.has_any(&*SVG_DIRTY);
    // println!("========== svg_change: {}", r);
    r
}
pub fn text_content_change(mark: SingleRes<GlobalDirtyMark>, removed: ComponentRemoved<TextContent>) -> bool {
    let r = removed.len() > 0 || mark.mark.get(StyleType::TextContent as usize).map_or(false, |display| *display == true);
    removed.mark_read();
    r
}

pub fn text_shadow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
    let r = mark.mark.get(StyleType::TextShadow as usize).map_or(false, |display| *display == true);
    r
}

pub fn text_outer_glow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
    let r = mark.mark.get(StyleType::TextOuterGlow as usize).map_or(false, |display| *display == true);
    r
}

/// 计算sdf文字的的实例数量
///
pub fn calc_sdf2_text_len(
    mut events: OrInitSingleResMut<SvgTemp>,
    mut query: Query<
        (
            Entity,
            &NodeState,
            // Option<&TextOverflowData>,
            &DrawList,
            &LayoutResult,
            Option<&SvgShadow>,
            // OrDefault<TextStyle>,
            // Option<&TextOuterGlow>,
            &mut SvgInnerContent,
            &Up,
            &Layer,
        ),
        (
            Or<(
                // Changed<NodeState>,
                // Changed<TextOverflowData>,
                // Changed<LayoutResult>,
                Changed<SvgInnerContent>,
            )>,
            // Changed<TextStyle>,
            // Changed<TextShadow>)>,
            // With<SvgInnerContent>,
        ),
    >,
    query_filter: Query<(&SvgFilter, &Down)>,
    query_liner: Query<(&SvgLinearGradient, &Down)>,
    query_stop: Query<(&SvgLinearGradientStop, &Up)>,
    query_shadow: Query<&SvgShadow>,
    font_sheet: SingleResMut<ShareFontSheet>,
    mut query_draw: Query<&mut RenderCount, With<SvgMark>>,
    query_up: Query<(&NodeState, &LayoutResult, &'static Up)>,
    // query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
    r: OrInitSingleRes<IsRun>,
    mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
    render_type: OrInitSingleRes<SvgRenderObjType>,
) {
    if r.0 {
        return;
    }
    // println!("========== calc_sdf2_text_len1");
    let mut font_sheet = font_sheet.borrow_mut();
    for (entity, node_state, draw_list, layout, svg_shadow, mut svg, up, layer) in query.iter_mut() {
        log::error!("========== calc_sdf2_text_len");
        if layer.layer() == 0 {
            continue;
        }

        log::error!("========== calc_sdf2_text_len1");
        let render_type = ***render_type;
        let draw_id = match draw_list.get_one(render_type) {
            Some(r) => r.id,
            None => continue,
        };
        log::error!("========== calc_sdf2_text_len2");
        let mut render_count = match query_draw.get_mut(draw_id) {
            Ok(render_count) => render_count,
            _ => continue,
        };
        log::error!("========== calc_sdf2_text_len3");
        // let count = if node_state.0.text.len() > 0 {
        //     // let fontsize = get_size(&text_style.font_size) as f32;
        // 	log::error!("========== calc_sdf2_text_len4");
        let mut layout1 = &*layout;
        log::error!("========== calc_sdf2_text_len4: {:?}", (svg.style.filter, svg.style.filter.is_null()));
        //     if node_state.is_vnode() {
        //         let mut n;
        //         loop {
        //             n = up.parent();
        //             // 虚拟节点，现阶段只有图文混排的文字节点，直接使用父节点的世界矩阵
        //             if let Ok((s, l, u)) = query_up.get(n) {
        //                 if s.is_vnode() {
        //                     up = u;
        //                     continue;
        //                 }
        //                 layout1 = l;
        // 				println!("=============1111");
        //                 break;
        //             }
        //         }
        //     }
        let mut svg_shadow = svg_shadow;
        let mut shadow_e = Entity::null();
        if !svg.style.filter.is_null() {
            query_filter.iter().for_each(|item| {
                log::error!("========== calc_sdf2_text_len5: {:?}", item);
                if item.0 .0 == svg.style.filter {
                    svg_shadow = query_shadow.get(item.1.head()).map(|o| o).ok();
                    if svg_shadow.is_some() {
                        shadow_e = item.1.head();
                    }
                }
            });
        }
        log::error!("========== svg_shadow: {:?}", svg_shadow);
        if let SvgColor::ID(id) = svg.style.fill_color {
            let mut linear_gradient = LinearGradientColor::default();
            log::error!("========== query_liner: {:?}", id);
            query_liner.iter().for_each(|item| {
                linear_gradient.direction = (item.0.gradient_transform / 360.0) % 1.0 * std::f32::consts::TAU;
                log::error!("========== query_liner: {:?}", (&item));
                if item.0.id == id {
                    if let Some((SvgLinearGradientStop { offset, color }, up)) = query_stop.get(item.1.head()).ok() {
                        linear_gradient.list.push(ColorAndPosition {
                            position: *offset,
                            rgba: color.clone(),
                        });
                        let mut up = up;
                        loop {
                            let e = up.next();
                            log::error!("========== query_liner3333: {:?}", (&e));
                            if e.is_null() {
                                break;
                            }

                            let (SvgLinearGradientStop { offset, color }, up1) = query_stop.get(e).unwrap();
                            log::error!("========== query_liner3333: {:?}", (&e, offset, color, up1));
                            up = up1;

                            linear_gradient.list.push(ColorAndPosition {
                                position: *offset,
                                rgba: color.clone(),
                            });
                        }
                    }
                }
            });
            svg.style.fill_color = SvgColor::Color(Color::LinearGradient(linear_gradient));
        }

        log::error!("========== calc_sdf2_text_len6: {:?}", (&svg_shadow, &svg.style.fill_color));
        let data = UniformData {
            node_state,
            layout: layout1,
            // text_style,
            svg_shadow,
            // text_outer_glow,
            // text_overflow_data,
            font_sheet: &mut *font_sheet,
            own_layout: layout,
            svg: &svg,
            half_stroke: (*svg.style.stroke.width) / 2.0,
            fontsize: 128.0,
        };
        log::error!("===========data.geo111");
        log::error!("===========data.geo, {:?}", svg);
        let count = data.geo(entity, shadow_e, &mut events) as u32;
        log::error!("================data.geo2{:?}", (&events, count));
        // } else {
        //     0
        // };
        render_count.transparent = count as u32;
        // let diff = render_count.0 as i32 - count as i32;
        // if diff < 0 || diff > 10 {
        //     // 这里， 为文字数量保有一定的变动空间，防止像倒计时这类的文字，数量发生变化后，使得批渲数据重新分配
        //     render_count.0 = count as u32;
        //     global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
        //     log::debug!("node_changed2============");
        // }
    }
}

#[derive(Default)]
pub struct BatchSvgEffect {
    shadow_gray: Vec<(InstanceDrawState, Share<Fbo>)>,
    shadow_h: Vec<(InstanceDrawState, Share<Fbo>)>,
    shadow_v: Vec<(InstanceDrawState, Share<Fbo>)>,

    outer_glows: Vec<(InstanceDrawState, Share<Fbo>)>,
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_svg(
    mut events: OrInitSingleResMut<SvgTemp>,
    // sdf2_texture_version
    mut instances: OrInitSingleResMut<InstanceContext>,
    query_shadow: Query<(&DrawList, &SvgShadow)>,
    // query_outer_glow: Query<(&DrawList, &SvgInnerContent)>,
    query_text: Query<(&DrawList, &SvgInnerContent)>,
    mut query_draw: Query<&InstanceIndex>,
    mut query_draw_other: Query<(Entity, &InstanceIndex, &mut InstanceSplit)>,
    r: OrInitSingleRes<IsRun>,
    render_type: OrInitSingleRes<SvgRenderObjType>,
    shadow_render_type: OrInitSingleRes<SvgShadowRenderObjType>,
    // outer_glow_render_type: OrInitSingleRes<SvgOuterGlowRenderObjType>,
    atlas_allocator: SingleRes<PiSafeAtlasAllocator>,
    device: SingleRes<PiRenderDevice>,
    dyn_target_type: Local<U8DynTargetType>,
    mut bacth_svg_effect: OrInitSingleResMut<BatchSvgEffect>,
    mut bindgroup_map: Local<FboBindGroups>,
    mut tem_gray_target: Local<Vec<ShareTargetView>>,
    mut tem_shadow_h_target: Local<Vec<ShareTargetView>>,
) {
    if r.0 {
        return;
    }
    bacth_svg_effect.shadow_gray.clear();
    bacth_svg_effect.shadow_h.clear();
    bacth_svg_effect.shadow_v.clear();
    bacth_svg_effect.outer_glows.clear();

    instances.svg_gray_instance_data.clear();
    instances.svg_shadow_h_instance_data.clear();
    instances.svg_shadow_v_instance_data.clear();
    instances.svg_glow_instance_data.clear();

    let render_type = ***render_type;
    let shadow_render_type = ***shadow_render_type;
    // let outer_glow_render_type = ***outer_glow_render_type;

    let e: [ShareTargetView; 0] = [];

    // let mut pre_outer_glow_target = None;
    // let mut pre_outer_glow_src = None;

    let mut target_rect;

    let is_batch =
        |pre_target: &mut Option<Share<Fbo>>, pre_src: &mut Option<Share<Fbo>>, next_target: &Share<Fbo>, next_src: Option<&Share<Fbo>>| {
            let mut is_batch = false;
            if let Some(pre_target1) = pre_target {
                if !Share::ptr_eq(pre_target1, next_target) {
                    is_batch = true;
                    *pre_target = Some(next_target.clone());
                }
            } else {
                is_batch = true;
                *pre_target = Some(next_target.clone());
            }

            is_batch |= match (&pre_src, next_src) {
                (None, None) => false,
                (Some(r1), Some(r2)) => {
                    let rr = !Share::ptr_eq(r1, r2);
                    *pre_src = Some(r2.clone());
                    rr
                }
                (None, Some(r2)) => {
                    *pre_src = Some(r2.clone());
                    true
                }
                _ => true,
            };

            is_batch
        };

    let events = &mut **events;
    let buffer = &mut events.buffer;
    let outer_glow_buffer = &mut events.outer_glow_buffer;

    if events.shadow.len() > 0 {
        let mut pre_target = None;
        let mut pre_src = None;

        let mut pre_shadow_h_target = None;
        let mut pre_shadow_h_src = None;

        let mut pre_shadow_v_target = None;
        let mut pre_shadow_v_src = None;
        // gray
        for (geo, index, max_blur) in events.shadow.iter() {
            log::error!("svg shadow gray============");
            let text = &events.svg_events[*index];
            let max_blur = *max_blur;
            let max_blur2 = (max_blur + max_blur) as u32;
            let rect = &text.rect;
            let (width, height) = (
                (text.rect.maxs.x - text.rect.mins.x).ceil() as u32,
                (text.rect.maxs.y - text.rect.mins.y).ceil() as u32,
            );
            let (width_gray, height_gray) = (width + max_blur2, height + max_blur2);

            // 用于绘制高斯模糊所用的原图像（灰度图）
            let gray_target = atlas_allocator.allocate(width_gray, height_gray, dyn_target_type.0.no_depth, e.iter());
            target_rect = gray_target.rect();

            let range = match &geo.polygons {
                PolygonType::Rect(range) => range,
                _ => unreachable!(),
            };
            let instance_start = instances.svg_gray_instance_data.alloc_instance_data();
            let mut i = range.start;
            while i < range.end {
                let instance_start = instances.svg_gray_instance_data.alloc_instance_data();
                let pindex0 = i;
                let pindex1 = i + 2;
                let mut instance_data = instances.svg_gray_instance_data.instance_data_mut(instance_start);

                // 0~1范围
                let box_layout = [
                    (buffer.positions[pindex0] - rect.mins.x + target_rect.min.x as f32 + max_blur as f32) / gray_target.target().width as f32,
                    (buffer.positions[pindex0 + 1] - rect.mins.y + target_rect.min.y as f32 + max_blur as f32) / gray_target.target().height as f32,
                    (buffer.positions[pindex1] - buffer.positions[pindex0]) / gray_target.target().width as f32,
                    (buffer.positions[pindex1 + 1] - buffer.positions[pindex0 + 1]) / gray_target.target().height as f32,
                ];
                let sdf_uv = [
                    buffer.sdf_uvs[pindex0],
                    buffer.sdf_uvs[pindex0 + 1],
                    buffer.sdf_uvs[pindex1],
                    buffer.sdf_uvs[pindex1 + 1],
                ];

                log::error!(
                    "shadow gray============{:?}",
                    (width_gray, height_gray, max_blur, instance_start, box_layout, rect)
                );

                instance_data.set_data(&BatchGrayMeterial {
                    box_layout,
                    sdf_uv,
                    px_range: text.px_range,
                    fill_bound: text.fill_bound,
                });

                log::error!("geo color======={:?}", (instance_start, box_layout, sdf_uv));
                i += 4;
            }
            if is_batch(&mut pre_target, &mut pre_src, gray_target.target(), None) {
                log::error!("shadow bacth============");
                let b = InstanceDrawState {
                    instance_data_range: instance_start..instances.svg_gray_instance_data.cur_index(),
                    pipeline: Some(instances.default_pipelines.text_gray_pipeline.clone()),
                    texture_bind_group: instances.sdf2_texture_group.clone(),
                    #[cfg(debug_assertions)]
                    pipeline_type: "text_gray_pipeline",
                    #[cfg(debug_assertions)]
                    texture_bind_group_type: "sdf2_texture_group",
                };
                bacth_svg_effect.shadow_gray.push((b, gray_target.target().clone()));
            } else {
                let last = bacth_svg_effect.shadow_gray.len() - 1;
                bacth_svg_effect.shadow_gray[last].0.instance_data_range.end = instances.svg_gray_instance_data.cur_index();
            }

            tem_gray_target.push(gray_target);
        }
        // shadow_h
        for (ii, (_geo, index, max_blur)) in events.shadow.iter().enumerate() {
            let text = &events.svg_events[*index];
            let (_draw_list, shadow) = match query_shadow.get(text.shadow_entity) {
                Ok(r) => r,
                _ => continue,
            };
            let max_blur = (*max_blur) as f32;
            let (width, height) = (
                (text.rect.maxs.x - text.rect.mins.x).ceil() as u32,
                (text.rect.maxs.y - text.rect.mins.y).ceil() as u32,
            );

            let blur2 = shadow.blur_level as u32 * 2;
            let (width, height) = (width + blur2, height + blur2);
            let gray_target = &tem_gray_target[ii];
            // 该target作为绘制guass 水平方向模糊的目标target
            let h_target = atlas_allocator.allocate(width, height, dyn_target_type.0.no_depth, tem_gray_target[ii..ii + 1].iter());

            // 水平模糊实例
            let target_rect: [f32; 4] = h_target.rect_normalize();
            let rect = gray_target.rect();
            let instance_start = instances.svg_shadow_h_instance_data.alloc_instance_data();
            let meterial = BatchGussMeterial {
                box_layout: [
                    target_rect[0],
                    target_rect[1],
                    target_rect[2] - target_rect[0],
                    target_rect[3] - target_rect[1],
                ],
                uv: [
                    (rect.min.x as f32 + max_blur - shadow.blur_level + 0.5) / gray_target.target().width as f32,
                    (rect.min.y as f32 + max_blur - shadow.blur_level + 0.5) / gray_target.target().height as f32,
                    (rect.max.x as f32 + max_blur - shadow.blur_level - 0.5) / gray_target.target().width as f32,
                    (rect.max.y as f32 + max_blur - shadow.blur_level - 0.5) / gray_target.target().height as f32,
                ],
                texture_size: [gray_target.target().width as f32, gray_target.target().height as f32],
                blur_radius: shadow.blur_level,
                direcition: GaussDirecition::Horizontal as u8 as f32,
            };
            instances.svg_shadow_h_instance_data.instance_data_mut(instance_start).set_data(&meterial);
            log::error!("shadow h============{:?}", (width, height, instance_start, meterial, &text.rect));
            if is_batch(
                &mut pre_shadow_h_target,
                &mut pre_shadow_h_src,
                h_target.target(),
                Some(&gray_target.target()),
            ) {
                let b = InstanceDrawState {
                    instance_data_range: instance_start..instances.svg_shadow_h_instance_data.cur_index(),
                    pipeline: Some(instances.default_pipelines.text_shadow_pipeline.clone()),
                    texture_bind_group: Some(
                        bindgroup_map
                            .bind_group(
                                gray_target.target(),
                                &device,
                                &instances.sdf2_texture_layout,
                                &instances.batch_texture.default_sampler,
                            )
                            .clone(),
                    ),
                    #[cfg(debug_assertions)]
                    pipeline_type: "text_shadow_pipeline",
                    #[cfg(debug_assertions)]
                    texture_bind_group_type: "sdf2_texture_group",
                };
                bacth_svg_effect.shadow_h.push((b, h_target.target().clone()));
            } else {
                let last = bacth_svg_effect.shadow_h.len() - 1;
                bacth_svg_effect.shadow_h[last].0.instance_data_range.end = instances.svg_shadow_h_instance_data.cur_index();
            }
            tem_shadow_h_target.push(h_target);
        }
        tem_gray_target.clear();
        // shadow_v
        let mut shadow_h_i = 0;
        for (_geo, index, _max_blur) in events.shadow.drain(..) {
            log::error!("shadow_v =============== 1111: {:?}", &events.svg_events[index]);
            let text = &events.svg_events[index];
            let (draw_list, shadow) = match query_shadow.get(text.shadow_entity) {
                Ok(r) => r,
                _ => continue,
            };
            log::error!("shadow_v =============== 2222");
            let rect = &text.rect;
            let (width, height) = (
                (text.rect.maxs.x - text.rect.mins.x).ceil() as u32,
                (text.rect.maxs.y - text.rect.mins.y).ceil() as u32,
            );

            let draw_id_first = draw_list.get_first(shadow_render_type);

            let draw_id = &draw_list.0[draw_id_first];
            log::error!("shadow_v =============== 3333: {:?}", (draw_id, query_draw_other.len()));
            query_draw_other.iter().for_each(|e| {
                log::error!("shadow_v =============== 4444: {:?}", e);
            });
            if let Ok((_e, instance_index, mut split)) = query_draw_other.get_mut(draw_id.id) {
                log::error!("shadow_v =============== 555: {:?}", _e);
                let blur2 = shadow.blur_level as u32 * 2;
                let (width, height) = (width + blur2, height + blur2);
                let h_target = &tem_shadow_h_target[shadow_h_i];
                // 该target作为绘制guass 水平方向模糊的目标target
                let v_target = atlas_allocator.allocate(
                    width,
                    height,
                    dyn_target_type.0.no_depth,
                    tem_shadow_h_target[shadow_h_i..shadow_h_i + 1].iter(),
                );

                {
                    *split = InstanceSplit::ByFbo(Some(v_target.clone()));
                    let target_rect = v_target.rect_normalize();
                    // 垂直模糊实例
                    let instance_start = instances.svg_shadow_v_instance_data.alloc_instance_data();
                    instances
                        .svg_shadow_v_instance_data
                        .instance_data_mut(instance_start)
                        .set_data(&BatchGussMeterial {
                            box_layout: [
                                target_rect[0],
                                target_rect[1],
                                target_rect[2] - target_rect[0],
                                target_rect[3] - target_rect[1],
                            ],
                            uv: h_target.uv_box(),
                            texture_size: [h_target.target().width as f32, h_target.target().height as f32],
                            blur_radius: shadow.blur_level,
                            direcition: GaussDirecition::Vertical as u8 as f32,
                        });
                    if is_batch(
                        &mut pre_shadow_v_target,
                        &mut pre_shadow_v_src,
                        v_target.target(),
                        Some(&h_target.target()),
                    ) {
                        log::error!("shadow_v =============== 1111");
                        let b = InstanceDrawState {
                            instance_data_range: instance_start..instances.svg_shadow_v_instance_data.cur_index(),
                            pipeline: Some(instances.default_pipelines.text_shadow_pipeline.clone()),
                            texture_bind_group: Some(
                                bindgroup_map
                                    .bind_group(
                                        h_target.target(),
                                        &device,
                                        &instances.sdf2_texture_layout,
                                        &instances.batch_texture.default_sampler,
                                    )
                                    .clone(),
                            ), // TODO
                            #[cfg(debug_assertions)]
                            pipeline_type: "text_shadow_pipeline",
                            #[cfg(debug_assertions)]
                            texture_bind_group_type: "sdf2_texture_group",
                        };
                        bacth_svg_effect.shadow_v.push((b, v_target.target().clone()));
                    } else {
                        let last = bacth_svg_effect.shadow_v.len() - 1;
                        bacth_svg_effect.shadow_v[last].0.instance_data_range.end = instances.svg_shadow_v_instance_data.cur_index();
                    }
                }

                // 最后的高斯模糊结果渲染到屏幕或目标target上
                {
                    let mut instance_data = instances.instance_data.instance_data_mut(instance_index.transparent.start);
                    let mut ty = instance_data.get_render_ty();
                    ty |= 1 << RenderFlagType::R8 as usize;
                    instance_data.set_data(&TyMeterial(&[ty as f32]));
                    instance_data.set_data(&ColorUniform(shadow.color.as_slice()));
                    instance_data.set_data(&UvUniform(h_target.uv_box().as_slice()));
                    instance_data.set_data(&LayoutUniform(&[
                        rect.mins.x - shadow.blur_level + shadow.offset_x,
                        rect.mins.y - shadow.blur_level + shadow.offset_y,
                        width as f32,
                        height as f32,
                    ]));
                }
            }
            shadow_h_i += 1;
        }
        tem_shadow_h_target.clear();
    }

    // for (geo, index) in events.outer_glow.drain(..) {
    //     let text = &events.text[index];
    //     let (draw_list, svg) = match query_outer_glow.get(text.entity) {
    //         Ok(r) => r,
    //         _ => continue,
    //     };
    //     let rect = &text.rect;
    //     let distance2 = svg.style.outer_glow.distance as u32 + svg.style.outer_glow.distance as u32;
    //     let (width, height) = (
    //         (text.rect.maxs.x - text.rect.mins.x + distance2 as f32).ceil() as u32,
    //         (text.rect.maxs.y - text.rect.mins.y + distance2 as f32).ceil() as u32,
    //     );
    //     let range = match geo.polygons {
    //         PolygonType::Rect(range) => range,
    //         _ => unreachable!(),
    //     };

    //     let outer_glow_target = atlas_allocator.allocate(width, height, dyn_target_type.0.no_depth, e.iter());
    //     let instance_start = instances.svg_glow_instance_data.cur_index();

    //     let mut i = range.start;
    //     while i < range.end {
    //         let instance_start = instances.svg_glow_instance_data.alloc_instance_data();
    //         let pindex0 = i;
    //         let pindex1 = i + 2;

    //         let target_rect = outer_glow_target.rect();
    //         let meterial = BatchGlowMeterial {
    //             box_layout: [
    //                 (outer_glow_buffer.positions[pindex0] - rect.mins.x + svg.style.outer_glow.distance + target_rect.min.x as f32)
    //                     / outer_glow_target.target().width as f32,
    //                 (outer_glow_buffer.positions[pindex0 + 1] - rect.mins.y + svg.style.outer_glow.distance + target_rect.min.y as f32)
    //                     / outer_glow_target.target().height as f32,
    //                 (outer_glow_buffer.positions[pindex1] - outer_glow_buffer.positions[pindex0]) / outer_glow_target.target().width as f32,
    //                 (outer_glow_buffer.positions[pindex1 + 1] - outer_glow_buffer.positions[pindex0 + 1]) / outer_glow_target.target().height as f32,
    //             ],
    //             sdf_uv: [
    //                 outer_glow_buffer.sdf_uvs[pindex0],
    //                 outer_glow_buffer.sdf_uvs[pindex0 + 1],
    //                 outer_glow_buffer.sdf_uvs[pindex1],
    //                 outer_glow_buffer.sdf_uvs[pindex1 + 1],
    //             ],
    //             fill_bound: text.fill_bound,
    //         };

    //         log::debug!(
    //             "text_glow============{:?}",
    //             (
    //                 width,
    //                 height,
    //                 instance_start,
    //                 &meterial,
    //                 &outer_glow_buffer.positions[i..i + 4],
    //                 &text.rect
    //             )
    //         );

    //         instances.svg_glow_instance_data.instance_data_mut(instance_start).set_data(&meterial);

    //         i += 4;
    //     }

    //     {
    //         if is_batch(&mut pre_outer_glow_target, &mut pre_outer_glow_src, outer_glow_target.target(), None) {
    //             let b = InstanceDrawState {
    //                 instance_data_range: instance_start..instances.svg_glow_instance_data.cur_index(),
    //                 pipeline: Some(instances.text_glow_pipeline.clone()),
    //                 texture_bind_group: instances.sdf2_texture_group.clone(), // TODO
    //             };
    //             bacth_text_effect.outer_glows.push((b, outer_glow_target.target().clone()));
    //         } else {
    //             let last = bacth_text_effect.outer_glows.len() - 1;
    //             bacth_text_effect.outer_glows[last].0.instance_data_range.end = instances.svg_glow_instance_data.cur_index();
    //         }
    //     }

    //     // 外发光实例渲染在屏幕上或目标fbo上的实例数据填充
    //     {
    //         let draw_id = match draw_list.get_one(outer_glow_render_type) {
    //             Some(r) => r.id,
    //             None => continue,
    //         };

    //         if let Ok((instance_index, mut split)) = query_draw_other.get_mut(draw_id) {
    //             let mut instance_data = instances.instance_data.instance_data_mut(instance_index.start);
    //             let mut ty = instance_data.get_render_ty();
    //             ty |= 1 << RenderFlagType::R8 as usize;
    //             instance_data.set_data(&TyMeterial(&[ty as f32]));
    //             instance_data.set_data(&ColorUniform(svg.style.outer_glow.color.as_slice()));
    //             instance_data.set_data(&UvUniform(outer_glow_target.uv_box().as_slice()));
    //             instance_data.set_data(&LayoutUniform(&[
    //                 rect.mins.x - svg.style.outer_glow.distance,
    //                 rect.mins.y - svg.style.outer_glow.distance,
    //                 width as f32,
    //                 height as f32,
    //             ]));
    //             log::debug!(
    //                 "outer_glow render to screen======{:?}",
    //                 (instance_index.start, &rect, svg.style.outer_glow.distance, width, height)
    //             );
    //             *split = InstanceSplit::ByFbo(Some(outer_glow_target.clone()));
    //         }
    //     }

    // println!("calc_sdf2_text111");
    for text in events.svg_events.drain(..) {
        log::error!("calc_sdf2_text,  events.text.drain");
        let (draw_list, svg) = match query_text.get(text.entity) {
            Ok(r) => r,
            _ => continue,
        };
        log::error!("calc_sdf2_text,  events.text.drain2");
        let draw_id = match draw_list.get_one(render_type) {
            Some(r) => r.id,
            None => continue,
        };
        log::error!("calc_sdf2_text,  events.text.drain3");
        let stroke_color = &svg.style.stroke.color;

        let instance_index = match query_draw.get_mut(draw_id) {
            Ok(r) => r,
            _ => continue,
        };

        // 节点可能设置为dispaly none， 此时instance_index可能为Null
		if pi_null::Null::is_null(&instance_index.transparent.start) {
			continue;
		}

        log::error!("calc_sdf2_text,  events.text.drain4, {:?}", instance_index);
        let start = instance_index.transparent.start;
        let mut ty = instances.instance_data.instance_data_mut(start).get_render_ty();
        ty &= !(1 << RenderFlagType::Invalid as usize);
        if *svg.style.stroke.width > 0.0 {
            ty |= 1 << RenderFlagType::Stroke as usize;
        } else {
            ty &= !(1 << RenderFlagType::Stroke as usize);
        }
        log::error!(
            "calc_sdf2_text,  events.text.drain5, text.stroke_bound: {:?}",
            (text.px_range, text.fill_bound, text.stroke_bound, &text.text_geo, &buffer)
        );
        let end = text.text_geo.set_instance_data(
            start,
            &mut instances,
            Some(&OtherInfo {
                sdf_info: [text.px_range, text.fill_bound, text.stroke_bound],
                stroke_color: [stroke_color.x, stroke_color.y, stroke_color.z, stroke_color.w],
                ty: ty as f32,
            }),
            buffer,
        );
        log::error!("calc_sdf2_text,  events.text.drain6");
        if end < instance_index.transparent.end {
            // 设多余的实例为无效实例
            ty |= 1 << RenderFlagType::Invalid as usize;
            instances.instance_data.set_data_mult1(end..instance_index.transparent.end, &TyMeterial(&[ty as f32]));
        }
    }

    buffer.clear();
    outer_glow_buffer.clear();
}


struct UniformData<'a> {
    node_state: &'a NodeState,
    layout: &'a LayoutResult,
    own_layout: &'a LayoutResult,
    svg_shadow: Option<&'a SvgShadow>,
    // text_overflow_data: Option<&'a TextOverflowData>,
    svg: &'a SvgInnerContent,
    // text_outer_glow: Option<&'a TextOuterGlow>,
    // text_style: &'a TextStyle,
    font_sheet: &'a mut FontSheet,

    fontsize: f32, // 字体大小
    half_stroke: f32,
}

#[derive(Debug)]
pub struct SvgEvent {
    entity: Entity,
    shadow_entity: Entity,
    // not_vnode: Entity, // 非虚拟节点的父节点
    text_geo: TempGeo,
    rect: Aabb2, // 文本区域

    px_range: f32,
    fill_bound: f32,
    stroke_bound: f32,
}

#[derive(Debug, Default)]
pub struct SvgTemp {
    buffer: TempGeoBuffer,
    outer_glow_buffer: TempGeoBuffer,
    svg_events: Vec<SvgEvent>,
    shadow: Vec<(TempGeo, usize, usize /*max_blur*/)>, // (阴影，文字轮廓渲染在第几个索引：text数组中的索引)
    outer_glow: Vec<(TempGeo, usize)>,                 // (外发光，文字轮廓渲染在第几个索引：text数组中的索引)
}

pub struct TextResult {
    text: SvgEvent,
    outer_glow: TempGeo,
    instance_count: usize,
}


impl<'a> UniformData<'a> {
    fn geo(&self, entity: Entity, shadow_entity: Entity, temp: &mut SvgTemp) -> usize {
        // let font_size = 50;
        let font_size_scale = self.svg.scale;

        let px_range = pi_hal::font::sdf2_table::PXRANGE as f32 * font_size_scale * 2.0;
        //
        let fill_bound = 0.5 - (500 as f32 /* 大于500粗体，小于500细体，500正常 */ / 500 as f32 - 1.0) / px_range;
        let stroke_bound = fill_bound - (*self.svg.style.stroke.width) / 2.0 / font_size_scale / (pi_hal::font::sdf2_table::PXRANGE as f32 * 2.0);
        log::error!(
            "================geo, .stroke_bound: {:?}",
            (
                stroke_bound,
                fill_bound,
                self.svg.style.stroke.width,
                (*self.svg.style.stroke.width) / 2.0 / font_size_scale / (pi_hal::font::sdf2_table::PXRANGE as f32 * 2.0)
            )
        );
        let mut calc_result = TextResult {
            text: SvgEvent {
                entity,
                shadow_entity,
                text_geo: Default::default(),
                rect: Aabb2::new(
                    Point2::new(std::f32::INFINITY, std::f32::INFINITY),
                    Point2::new(-std::f32::INFINITY, -std::f32::INFINITY),
                ),
                px_range,
                fill_bound,
                stroke_bound,
            },
            outer_glow: Default::default(),
            instance_count: 0,
        };

        let text_start = temp.buffer.positions.len();

        self.push_pos_uv(&mut calc_result, temp);
        log::error!("push_pos_uv ============= : {:?}", (temp.buffer));

        let char_len = (temp.buffer.positions.len() - text_start) / 4;
        let text_end = temp.buffer.positions.len();

        // calc_result.outer_glow_geo.polygons = PolygonType::Rect;  // 规则的四边形
        if let Some(shadow) = self.svg_shadow {
            let mut max_blur = 0.0_f32;
            max_blur = max_blur.max(self.svg.style.shadow.blur_level.ceil());
            temp.shadow.push((
                TempGeo {
                    polygons: PolygonType::Rect(text_start..temp.buffer.positions.len()),
                    colors: VColor::CgColor(CgColor::new(0.0, 0.0, 0.0, 1.0)),
                    sdf_px_range: 1.0,
                },
                temp.svg_events.len(),
                max_blur as usize,
            ));
        }

        // if let Some(_outer_glow) = &self.svg.style.outer_glow {
        //     calc_result.outer_glow.polygons = PolygonType::Rect(ouetr_glow_start..temp.outer_glow_buffer.positions.len());
        //     temp.outer_glow.push((calc_result.outer_glow, temp.text.len()));
        // }


        calc_result.text.text_geo.polygons = PolygonType::Rect(text_start..text_end); // 规则的四边形
        match &self.svg.style.fill_color {
            SvgColor::Color(Color::RGBA(r)) => {
                // 如果不是封闭图形，拆分曲面
                // 当颜色为渐变时暂时不拆分；todo
                if !self.svg.svg_info.is_area {
                    
                    let ps = temp.buffer.positions[text_start..text_end].to_vec(); 
                    let uvs = temp.buffer.sdf_uvs[text_start..text_end].to_vec(); 
                    let mut indices = Vec::new(); 
                    let start = temp.buffer.positions.len();
                    println!("=========== temp.buffer.positions: {:?}",( &temp.buffer.positions.len(), start));
                    self.svg.svg_info.compute_positions_and_uv(&ps, &uvs, self.half_stroke * 2.0, &mut temp.buffer.positions, &mut temp.buffer.sdf_uvs, &mut indices);
                    // let color = 
                    let mut size = (temp.buffer.positions.len() - start) / 2;
                    if start / 2  > temp.buffer.colors.len() / 4{
                        size += start / 2  - temp.buffer.colors.len() / 4;
                    }
                    temp.buffer.colors.append(&mut vec![[r.x, r.y, r.z, r.w]; size].into_iter().flatten().collect::<Vec<f32>>());
                    // temp.buffer.positions = ps;// vec![19.0, 99.0, 121.0, 99.0, 121.0, 201.0, 19.0, 201.0];
                    // temp.buffer.sdf_uvs = uvs;//vec![1.0, 43.0, 53.0, 43.0, 53.0, 95.0, 1.0, 95.0];
                    println!("====== compute_positions_and_uv: {:?}", (&temp.buffer.positions.len(), &temp.buffer.sdf_uvs, &temp.buffer.colors.len(), &indices));
                    calc_result.instance_count += indices.len() / 3;
                    calc_result.text.text_geo.polygons = PolygonType::Triangle(indices); 
                } else {
                    calc_result.instance_count += 1;
                    log::error!("text cgcolor============char_len: {:?}", char_len);
                    calc_result.text.text_geo.colors = VColor::CgColor(r.clone());
                }
            }
            SvgColor::Color(Color::LinearGradient(color)) => {
                // log::error!("============= LinearGradient: {:?}", (color, &calc_result.text.text_geo, &self.own_layout.padding_rect()));
                // let rect = self.own_layout.padding_rect();
                let rect = Rect::new(self.svg.bbox.0, self.svg.bbox.2, self.svg.bbox.1, self.svg.bbox.3);
                // log::error!("push_pos_uv222222 ============= : {:?}", (temp.buffer));
                calc_result.text.text_geo.linear_gradient_split(color, &rect, &mut temp.buffer);
                log::error!("============= LinearGradient000: {:?}", (&calc_result.text, &temp.buffer));
                let out_indices = match &calc_result.text.text_geo.polygons {
                    PolygonType::NoRule(indices) => {
                        let mut out_indices = Vec::with_capacity(indices.counts.len() * 4); // 预计多边形为四边形
                        mult_to_triangle(&indices, &mut out_indices);
                        out_indices
                    }
                    _ => todo!(), // 不会是三角形和规则多边形
                };
                // log::error!("out_indices: {:?}", out_indices);

                calc_result.instance_count += out_indices.len() / 3;
                log::error!("text LinearGradient============instance_count: {:?}", calc_result.instance_count);
                calc_result.text.text_geo.polygons = PolygonType::Triangle(out_indices);
            }
            SvgColor::ID(_id) => {}
        }
        log::error!("============= LinearGradient1111: {:?}", (&calc_result.text, &temp.buffer));
        temp.svg_events.push(calc_result.text);
        calc_result.instance_count
    }

    fn push_pos_uv(&self, result: &mut TextResult, temp: &mut SvgTemp) {
        // log::debug!("glyph!!!==================={:?}, {:?}, {left:?}, {top:?}", c.ch_id, c.ch);
        let svg = self.svg;
        let info = self.font_sheet.font_mgr().table.sdf2_table.shapes_tex_info.get(&svg.hash).unwrap();

        push_pos_uv(&mut temp.buffer.positions, &mut temp.buffer.sdf_uvs, self.half_stroke, info, svg.bbox, &self.svg.svg_info);

        result.text.rect.mins.x = svg.bbox.0;
        result.text.rect.mins.y = svg.bbox.1;
        result.text.rect.maxs.x = svg.bbox.2;
        result.text.rect.maxs.y = svg.bbox.3;
    }
}


fn push_pos_uv(
    positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    // x: f32,
    // y: f32,
    // line_height: f32,
    // fontsize: f32,
    half_extend: f32, // 一半的扩展宽度（描边， 阴影， 外法光都需要额外扩展）
    info: &SvgTexInfo,
    // metrics: &MetricsInfo,
    // is_linear: bool,
    bbox: (f32, f32, f32, f32),
    svg_info: &SvgInfo
) {

    // 否则， push aabb
    let ps = [bbox.0 - half_extend, bbox.1 - half_extend, bbox.2 + half_extend, bbox.3 + half_extend];

    let uv = [
        info.x - half_extend,
        info.y - half_extend, //??? todo * 0.5
        info.x + info.width as f32 + half_extend,
        info.y + info.height as f32 + half_extend,
    ];

    uvs.extend_from_slice(&uv);
    positions.extend_from_slice(&ps[..]);
}


#[derive(SystemParam)]
pub struct QueryParam<'w> {
    // camera_query: Query<'w, 's, &'static Camera>,
    instance_draw: OrInitSingleRes<'w, InstanceContext>,
    bacth_text_effect: SingleRes<'w, BatchSvgEffect>,
    // clear_draw: SingleRes<'w, ClearDrawObj>,
    // depth_cache: OrInitSingleRes<'w, DepthCache>,
}

pub fn init_svg_effect_graph(mut rg: SingleResMut<PiRenderGraph>) {
    let effect_graph_id = rg.add_sub_graph("gui_svg_effect_graph").unwrap();
    let node = SvgEffectNode;
    let _id = rg.add_node("GuiSvgEffectNode", node, effect_graph_id, Null::null()).unwrap();

    // 将其设置在所有gui节点之前运行
    let main_graph_id = rg.main_graph_id();
    let _ = rg.add_depend(effect_graph_id, main_graph_id);
}

/// 渲染图节点， 用于将文字做模糊处理（draw_front）
pub struct SvgEffectNode;

impl Node for SvgEffectNode {
    type RunParam = QueryParam<'static>;
    type BuildParam = ();
    type ResetParam = ();
    // // 释放纹理占用
    // fn reset<'a>(&'a mut self) {
    //     // self.out_put_target = None;
    //     // self.target = None;
    // }

    /// 用于给pass2d分配fbo
    fn build<'a>(
        &'a mut self,
        // world: &'a mut pi_world::world::World,
        _param: &'a mut Self::BuildParam,
        _context: pi_bevy_render_plugin::RenderContext,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> Result<(), String> {
        Ok(())
    }

    fn run<'a>(
        &'a mut self,
        param: &'a Self::RunParam,
        _context: RenderContext,
        commands: ShareRefCell<CommandEncoder>,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> BoxFuture<'a, Result<(), String>> {
        Box::pin(async move {
            let mut commands = commands.borrow_mut();
            let mut pre = None;
            let mut rp: Option<RenderPass> = None;

            let mut state = RenderState {
                reset: true,
                pipeline: param.instance_draw.default_pipelines.common_pipeline.clone(),
                texture: param.instance_draw.batch_texture.default_group(false),
            };

            for (draw, fbo) in param.bacth_text_effect.shadow_gray.iter() {
                if is_change(&mut pre, fbo) {
                    {
                        let _a = rp;
                    }
                    rp = Some(c_rp(&mut commands, fbo));
                    let rp = rp.as_mut().unwrap();
                    state.reset = true;
                    param.instance_draw.set_pipeline(rp, draw, &mut state);
                    let group = param.instance_draw.default_camera.get_group();
                    rp.set_bind_group(0, &**group.bind_group, group.offsets);
                    state.reset = true;
                }
                let rp = rp.as_mut().unwrap();
                param.instance_draw.draw_effect(
                    rp,
                    draw,
                    &param.instance_draw.svg_gray_instance_buffer,
                    &param.instance_draw.svg_gray_instance_data,
                    &mut state,
                );
            }

            for (draw, fbo) in param.bacth_text_effect.shadow_h.iter() {
                if is_change(&mut pre, fbo) {
                    {
                        let _a = rp;
                    }
                    rp = Some(c_rp(&mut commands, fbo));
                    state.reset = true;
                    let rp = rp.as_mut().unwrap();
                    param.instance_draw.set_pipeline(rp, draw, &mut state);
                    let group = param.instance_draw.default_camera.get_group();
                    rp.set_bind_group(0, &**group.bind_group, group.offsets);
                }
                let rp = rp.as_mut().unwrap();
                param.instance_draw.set_pipeline(rp, draw, &mut state);
                param.instance_draw.draw_effect(
                    rp,
                    draw,
                    &param.instance_draw.svg_shadow_h_instance_buffer,
                    &param.instance_draw.svg_shadow_h_instance_data,
                    &mut state,
                );
            }

            state.reset = true;
            for (draw, fbo) in param.bacth_text_effect.shadow_v.iter() {
                if is_change(&mut pre, fbo) {
                    {
                        let _a = rp;
                    }
                    rp = Some(c_rp(&mut commands, fbo));
                    let rp = rp.as_mut().unwrap();
                    state.reset = true;
                    param.instance_draw.set_pipeline(rp, draw, &mut state);
                    let group = param.instance_draw.default_camera.get_group();
                    rp.set_bind_group(0, &**group.bind_group, group.offsets);
                }
                let rp = rp.as_mut().unwrap();
                param.instance_draw.set_pipeline(rp, draw, &mut state);
                param.instance_draw.draw_effect(
                    rp,
                    draw,
                    &param.instance_draw.svg_shadow_v_instance_buffer,
                    &param.instance_draw.svg_shadow_v_instance_data,
                    &mut state,
                );
            }

            state.reset = true;
            for (draw, fbo) in param.bacth_text_effect.outer_glows.iter() {
                if is_change(&mut pre, fbo) {
                    {
                        let _a = rp;
                    }
                    rp = Some(c_rp(&mut commands, fbo));
                    state.reset = true;
                    let rp = rp.as_mut().unwrap();
                    param.instance_draw.set_pipeline(rp, draw, &mut state);
                    let group = param.instance_draw.default_camera.get_group();
                    rp.set_bind_group(0, &**group.bind_group, group.offsets);
                }

                let rp = rp.as_mut().unwrap();
                param.instance_draw.set_pipeline(rp, draw, &mut state);
                param.instance_draw.draw_effect(
                    rp,
                    draw,
                    &param.instance_draw.text_glow_instance_buffer,
                    &param.instance_draw.svg_glow_instance_data,
                    &mut state,
                );
            }

            Ok(())
        })
    }
    
    fn reset<'a>(
        &'a mut self,
        param: &'a mut Self::ResetParam,
        context: RenderContext,
        id: Entity,
    ) {

    }
}

fn is_change<'a: 'b, 'b>(pre: &'b mut Option<Share<Fbo>>, fbo: &'a Share<Fbo>) -> bool {
    match &pre {
        Some(pre_fbo) => {
            if Share::ptr_eq(pre_fbo, fbo) {
                return false;
            }
        }
        None => (),
    };
    *pre = Some(fbo.clone());
    true
}

fn c_rp<'a: 'b, 'b>(commands: &'a mut CommandEncoder, fbo: &'a Share<Fbo>) -> RenderPass<'a> {
    let ops = wgpu::Operations::default();

    commands.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: fbo
            .colors
            .iter()
            .map(|view| {
                Some(wgpu::RenderPassColorAttachment {
                    resolve_target: None,
                    ops,
                    view: &view.0,
                })
            })
            .collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>()
            .as_slice(),
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    })
}

#[derive(Debug, Clone)]
pub struct U8DynTargetType(pub DynTargetType);

impl FromWorld for U8DynTargetType {
    fn from_world(world: &mut pi_world::world::World) -> Self {
        let atlas_allocator = world.get_single_res::<PiSafeAtlasAllocator>().unwrap();
        Self(create_dyn_target_type(&atlas_allocator, 1024, 1024, wgpu::TextureFormat::R8Unorm))
    }
}


