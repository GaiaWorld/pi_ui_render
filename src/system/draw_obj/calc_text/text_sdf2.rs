//! sdf2文字功能

use std::collections::hash_map::Entry;

use pi_bevy_render_plugin::{
    node::{Node, NodeId as GraphNodeId, ParamUsage}, PiRenderDevice, PiRenderGraph, PiSafeAtlasAllocator, RenderContext
};
use pi_flex_layout::prelude::CharNode;
use pi_futures::BoxFuture;
use pi_hal::font::sdf2_table::sdf_font_size;
use pi_hal::font::sdf_table::MetricsInfo;
use pi_hash::XHashMap;
use pi_null::Null;
use pi_polygon::mult_to_triangle;
use pi_share::{Share, ShareRefCell};
use pi_slotmap::KeyData;
use pi_world::{event::ComponentRemoved, system_params::Local, world::FromWorld};
use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, SingleResMut, Entity, Plugin, OrDefault, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes, Up, Layer};
use pi_hal::font::font::FontType;
// use pi_hal::pi_sdf::glyphy::geometry::aabb::AabbEXT;
use pi_render::{components::view::target_alloc::{Fbo, ShareTargetView}, font::{FontSheet, Glyph, GlyphId}};
use pi_style::style::{Aabb2, CgColor, Point2, StyleType, TextOverflow};
use pi_world::single_res::SingleRes;
use pi_world::system_params::SystemParam;
use wgpu::{ BindGroupLayout, CommandEncoder, RenderPass, Sampler};

use crate::{components::{calc::{style_bit, LayoutResult, NodeState, StyleBit, StyleMarkType, LAYOUT_DIRTY}, draw_obj::{InstanceSplit, TempGeoBuffer, TextOuterGlowMark, TextShadowMark}, pass_2d::InstanceDrawState, root::DynTargetType}, resource::{draw_obj::RenderState, TextOuterGlowRenderObjType, TextShadowRenderObjType}, shader1::{batch_gauss_blur::{BatchGussMeterial, GaussDirecition}, batch_meterial::{LayoutUniform, TyMeterial, UvUniform}, batch_sdf_glow::BatchGlowMeterial, batch_sdf_gray::BatchGrayMeterial}, system::draw_obj::root_view_port::create_dyn_target_type};
use crate::components::draw_obj::{BoxType, PolygonType, RenderCount, TempGeo, TextMark, VColor};
use crate::components::user::{get_size, TextContent, TextOuterGlow, TextOverflowData, TextShadow, TextStyle};
use crate::components::user::Color;
use crate::resource::{GlobalDirtyMark, IsRun, OtherDirtyType, ShareFontSheet, TextRenderObjType};
use crate::shader::ui_meterial::ColorUniform;
use crate::shader1::batch_meterial::RenderFlagType;
use crate::system::base::draw_obj::life_drawobj::{draw_object_life_new, update_render_instance_data};
use crate::system::base::draw_obj::sdf_gen::update_sdf2_texture;
use crate::prelude::UiStage;
use crate::system::base::node::layout::calc_layout;
use crate::system::draw_obj::geo_split::OtherInfo;

use super::{text_glyph::text_glyph, TEXT_OUTER_GLOW_ORDER, TEXT_SHADOW_ORDER};
use super::TEXT_ORDER;
use super::text_split::text_split;

use crate::components::calc::DrawList;
use crate::components::draw_obj::InstanceIndex;
use crate::resource::draw_obj::InstanceContext;
use crate::system::system_set::{UiSchedule, UiSystemSet};

/// 使用sdf2的方式渲染文字
pub struct Sdf2TextPlugin;

impl Plugin for Sdf2TextPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		let font_sheet = ShareFontSheet::new(&mut app.world, FontType::Sdf2);
		app.world.insert_single_res(font_sheet);
        app
            .add_startup_system(UiStage, init_text_effect_graph)
            // 文字劈分为字符
            .add_system(UiStage, text_split
				.before(calc_layout)
				.in_set(UiSystemSet::Layout)
				.in_schedule(UiSchedule::Layout)
				.in_schedule(UiSchedule::Calc)
				.in_schedule(UiSchedule::Geo)
				.run_if(text_layout_change))
            // 字形计算
            .add_system(UiStage, text_glyph
				.after(text_split)
				.in_set(UiSystemSet::Layout)
				.before(update_sdf2_texture)
				.in_schedule(UiSchedule::Calc)
			)
			// 创建drawobj 
			.add_system(
				UiStage, 
				draw_object_life_new::<
						TextContent,
						TextRenderObjType,
						(TextMark, RenderCount),
						{ TEXT_ORDER },
						{ BoxType::None },>
						.in_set(UiSystemSet::LifeDrawObject)
						.run_if(text_content_change)
						.before(calc_sdf2_text_len),
			)
			// 为阴影创建drawObj
			.add_system(
				UiStage, 
				draw_object_life_new::<
						TextShadow,
						TextShadowRenderObjType,
						TextShadowMark,
						{ TEXT_SHADOW_ORDER },
						{ BoxType::None },>
						.in_set(UiSystemSet::LifeDrawObject)
						.run_if(text_shadow_change)
						.before(calc_sdf2_text_len),
			)
			// 为outerglow 创建drawObj
			.add_system(
				UiStage, 
				draw_object_life_new::<
						TextOuterGlow,
						TextOuterGlowRenderObjType,
						TextOuterGlowMark,
						{ TEXT_OUTER_GLOW_ORDER },
						{ BoxType::None },>
						.in_set(UiSystemSet::LifeDrawObject)
						.run_if(text_outer_glow_change)
						.before(calc_sdf2_text_len),
			)

			
			// 统计drawobj的实例长度（文字包含多个字符，每个字符一个实例， 并且可能包含多层阴影， 每阴影每字符也需要一个实例）
			// 由于当前一个文字实例可附带渲染一个阴影，因此最终的实例个数为`text.len() * (shadow.len() > 1? shadow.len() - 1: 1)`个实例
			.add_system(
				UiStage, 
				calc_sdf2_text_len
					
					.after(UiSystemSet::LifeDrawObjectFlush)
					.before(update_render_instance_data)
					.after(UiSystemSet::Layout)
					.run_if(text_len_change)
			)
			// 更新实例数据
			.add_system(
				UiStage, 
				calc_sdf2_text
					.in_set(UiSystemSet::PrepareDrawObj)
					.run_if(text_change)
			)
		;
    }
}

lazy_static! {
	pub static ref TEXT_DIRTY: StyleMarkType = TEXT_LEN_DIRTY.clone()
		.set_bit(OtherDirtyType::NodeState as usize);
	pub static ref TEXT_LEN_DIRTY: StyleMarkType = TEXT_LAYOUT_DIRTY.clone() | LAYOUT_DIRTY
		.set_bit(StyleType::Color as usize)
		.set_bit(StyleType::TextStroke as usize)
		.set_bit(StyleType::TextOuterGlow as usize)
		.set_bit(StyleType::TextShadow as usize);
	pub static ref TEXT_LAYOUT_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::FontStyle as usize)
		.set_bit(StyleType::FontWeight as usize)
		.set_bit(StyleType::FontSize as usize)
		.set_bit(StyleType::FontFamily as usize)
		.set_bit(StyleType::LetterSpacing as usize)
		.set_bit(StyleType::WordSpacing as usize)
		.set_bit(StyleType::LineHeight as usize)
		.set_bit(StyleType::TextIndent as usize)
		.set_bit(StyleType::WhiteSpace as usize)
		.set_bit(StyleType::TextAlign as usize)
		.set_bit(StyleType::VerticalAlign as usize)
		.set_bit(StyleType::TextContent as usize)
		.set_bit(StyleType::TextOverflow as usize)
		.set_bit(OtherDirtyType::NodeTreeAdd as usize);
}

pub fn text_layout_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*TEXT_LAYOUT_DIRTY)
}

pub fn text_len_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*TEXT_LEN_DIRTY)
}
pub fn text_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*TEXT_DIRTY)
}
pub fn text_content_change(mark: SingleRes<GlobalDirtyMark>, removed: ComponentRemoved<TextContent>) -> bool {
	let r = removed.len() > 0 || mark.mark.get(StyleType::TextContent as usize).map_or(false, |display| {*display == true});
	removed.mark_read();
	r
}

pub fn text_shadow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	let r = mark.mark.get(StyleType::TextShadow as usize).map_or(false, |display| {*display == true});
	r
}

pub fn text_outer_glow_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	let r = mark.mark.get(StyleType::TextOuterGlow as usize).map_or(false, |display| {*display == true});
	r
}

/// 计算sdf文字的的实例数量
/// 
pub fn calc_sdf2_text_len(
	mut events: OrInitSingleResMut<TextTemp>,
    query: Query<(
		Entity, 
		&NodeState, 
		Option<&TextOverflowData>, 
		&DrawList, 
		&LayoutResult,
		Option<&TextShadow>, 
		OrDefault<TextStyle>,
		Option<&TextOuterGlow>,
		&Up,
		&Layer,
	), (
		Or<(Changed<NodeState>, 
		Changed<TextOverflowData>, 
		Changed<LayoutResult>, 
		Changed<TextStyle>,
		Changed<TextShadow>)>, 
		With<TextContent>,
	)>,
	font_sheet: SingleResMut<ShareFontSheet>,
    mut query_draw: Query<&mut RenderCount, With<TextMark>>,
	query_up: Query<(&NodeState, &LayoutResult, &'static Up)>,
	// query_up: Query<(&'static LayoutResult, &'static Up, &'static NodeState)>,
	r: OrInitSingleRes<IsRun>,
	mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
	render_type: OrInitSingleRes<TextRenderObjType>,
) {
	if r.0 {
		return;
	}
	let mut font_sheet = font_sheet.borrow_mut();
	for (
		entity,
		node_state, 
		text_overflow_data,
		draw_list,
		layout,
		text_shadow,
		text_style,
		text_outer_glow,
		mut up,
		layer,
	) in query.iter() {
		if layer.layer().is_null() {
			continue;
		}
		

		let render_type = ***render_type;
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		let mut render_count = match query_draw.get_mut(draw_id) {
			Ok(render_count) => render_count,
			_ => continue	
		};
		let count =  if node_state.0.text.len() > 0 {
			let fontsize = get_size(&text_style.font_size) as f32;
		
			let mut layout1 = &*layout;
			if node_state.is_vnode() {
				let mut n ;
				loop {
					n = up.parent();
					// 虚拟节点，现阶段只有图文混排的文字节点，直接使用父节点的世界矩阵
					if let Ok((s, l, u)) = query_up.get(n) {
						if s.is_vnode() {
							up = u;
							continue;
						}
						layout1 = l;
						break;
					} else {
						log::warn!("can not find parent node==={:?}", (n, entity));
						break;
					}
				}
			}
			
			let data = UniformData {
				node_state,
				layout: layout1,
				text_style,
				text_shadow,
				text_outer_glow,
				text_overflow_data,
				font_sheet: &mut *font_sheet,
				own_layout: layout,

				fontsize,
				half_stroke: (*text_style.text_stroke.width) / 2.0,
			};

			data.geo(entity, &mut events)
		} else {
			0
		};
		
		let diff = render_count.0 as i32 - count as i32;
		if diff < 0 || diff > 10 { // 这里， 为文字数量保有一定的变动空间，防止像倒计时这类的文字，数量发生变化后，使得批渲数据重新分配
			render_count.0 = count as u32;
			global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
			log::debug!("node_changed2============");
		}
	}
}

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_sdf2_text(
	mut events: OrInitSingleResMut<TextTemp>,
	// sdf2_texture_version
	mut instances: OrInitSingleResMut<InstanceContext>,
	query_shadow: Query<(&DrawList, &TextShadow)>,
	query_outer_glow: Query<(&DrawList, &TextOuterGlow)>,
	query_text: Query<(&DrawList, &TextStyle)>,
    mut query_draw: Query<&InstanceIndex>,
	mut query_draw_other: Query<(&InstanceIndex, &mut InstanceSplit)>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<TextRenderObjType>,
	shadow_render_type: OrInitSingleRes<TextShadowRenderObjType>,
	outer_glow_render_type: OrInitSingleRes<TextOuterGlowRenderObjType>,

	atlas_allocator: SingleRes<PiSafeAtlasAllocator>,
	device: SingleRes<PiRenderDevice>,
	dyn_target_type: Local<U8DynTargetType>,
	mut bacth_text_effect: OrInitSingleResMut<BatchTextEffect>,
	mut bindgroup_map: Local<FboBindGroups>,
	mut tem_gray_target: Local<Vec<ShareTargetView>>,
	mut tem_shadow_h_target: Local<Vec<ShareTargetView>>,
) {
	if r.0 {
		return;
	}
	bacth_text_effect.shadow_gray.clear();
	bacth_text_effect.shadow_h.clear();
	bacth_text_effect.shadow_v.clear();
	bacth_text_effect.outer_glows.clear();

	instances.text_gray_instance_data.clear();
	instances.text_shadow_h_instance_data.clear();
	instances.text_shadow_v_instance_data.clear();
	instances.text_glow_instance_data.clear();

	let render_type = ***render_type;
	let shadow_render_type = ***shadow_render_type;
	let outer_glow_render_type = ***outer_glow_render_type;

	let e: [ShareTargetView; 0] = [];

	let mut pre_outer_glow_target = None;
	let mut pre_outer_glow_src = None;

	let mut target_rect;

	let is_batch = |
		pre_target: &mut Option<Share<Fbo>>, 
		pre_src: &mut Option<Share<Fbo>>,
		next_target: &Share<Fbo>, 
		next_src: Option<&Share<Fbo>>,

	| {
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
			},
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
			let text = &events.text[*index];
			let max_blur = *max_blur;
			let max_blur2 = (max_blur + max_blur) as u32;
			let rect = &text.rect;
			let (width, height) = ((text.rect.maxs.x - text.rect.mins.x).ceil() as u32, (text.rect.maxs.y - text.rect.mins.y).ceil() as u32);
			let (width_gray, height_gray) = (width + max_blur2, height + max_blur2);
				
			// 用于绘制高斯模糊所用的原图像（灰度图）
			let gray_target = atlas_allocator.allocate(width_gray, height_gray, dyn_target_type.0.no_depth, e.iter());
			target_rect = gray_target.rect();
	
			let range = match &geo.polygons {
				PolygonType::Rect(range) => range,
				_ => unreachable!(),
			};
			let instance_start = instances.text_gray_instance_data.alloc_instance_data();
			let mut i = range.start;
			while i < range.end {
				let instance_start = instances.text_gray_instance_data.alloc_instance_data();
				let pindex0 = i;
				let pindex1 = i + 2;
				let mut instance_data = instances.text_gray_instance_data.instance_data_mut(instance_start);
				
				// 0~1范围
				let box_layout = [
					(buffer.positions[pindex0] - rect.mins.x + target_rect.min.x as f32 + max_blur as f32) / gray_target.target().width as f32, (buffer.positions[pindex0 + 1] - rect.mins.y  + target_rect.min.y as f32 + max_blur as f32) / gray_target.target().height as f32,
					(buffer.positions[pindex1] - buffer.positions[pindex0]) / gray_target.target().width as f32, (buffer.positions[pindex1 + 1] - buffer.positions[pindex0 + 1]) / gray_target.target().height as f32,
				];
				let sdf_uv = [
					buffer.sdf_uvs[pindex0], buffer.sdf_uvs[pindex0 + 1],
					buffer.sdf_uvs[pindex1], buffer.sdf_uvs[pindex1 + 1],
				];

				log::debug!("shadow gray============{:?}", (width_gray, height_gray, max_blur, instance_start, box_layout,  rect));

				instance_data.set_data(&BatchGrayMeterial {
					box_layout,
					sdf_uv,
					px_range: text.px_range,
					fill_bound: text.fill_bound,
				});
				
				log::debug!("geo color======={:?}", (instance_start, box_layout, sdf_uv));
				i += 4;
			}
			if is_batch(&mut pre_target, &mut pre_src, gray_target.target(), None)  {
				log::debug!("shadow bacth============");
				let b = InstanceDrawState {
					instance_data_range: instance_start..instances.text_gray_instance_data.cur_index(),
					pipeline: Some(instances.text_gray_pipeline.clone()),
					texture_bind_group: instances.sdf2_texture_group.clone(),
				};
				bacth_text_effect.shadow_gray.push((b, gray_target.target().clone()));
			} else {
				let last = bacth_text_effect.shadow_gray.len() - 1;
				bacth_text_effect.shadow_gray[last].0.instance_data_range.end = instances.text_gray_instance_data.cur_index();
			}

			tem_gray_target.push(gray_target);
		}
		// shadow_h
		for (ii, (_geo, index, max_blur)) in events.shadow.iter().enumerate() {
			let text = &events.text[*index];
			let (_draw_list, text_shadow) = match query_shadow.get(text.entity) {
				Ok(r) => r,
				_ => continue,
			};
			let max_blur = (*max_blur) as f32;
			let (width, height) = ((text.rect.maxs.x - text.rect.mins.x).ceil() as u32, (text.rect.maxs.y - text.rect.mins.y).ceil() as u32);
			
			let shadow_len = text_shadow.len();
			for i in 0..shadow_len {
				let blur2 = text_shadow[i].blur as u32 * 2;
				let (width, height) = (width + blur2, height + blur2);
				let gray_target = &tem_gray_target[ii];
				// 该target作为绘制guass 水平方向模糊的目标target
				let h_target = atlas_allocator.allocate(width, height, dyn_target_type.0.no_depth, tem_gray_target[ii..ii + 1].iter());
					
				// 水平模糊实例
				let target_rect: [f32; 4] = h_target.rect_normalize();
				let rect = gray_target.rect();
				let instance_start = instances.text_shadow_h_instance_data.alloc_instance_data();
				let meterial = BatchGussMeterial {
					box_layout: [target_rect[0], target_rect[1], target_rect[2] - target_rect[0], target_rect[3] - target_rect[1]],
					uv: [
						(rect.min.x as f32 + max_blur - text_shadow[i].blur + 0.5) / gray_target.target().width as f32,
						(rect.min.y as f32 + max_blur - text_shadow[i].blur + 0.5) / gray_target.target().height as f32,
						(rect.max.x as f32 + max_blur - text_shadow[i].blur - 0.5) / gray_target.target().width as f32,
						(rect.max.y as f32 + max_blur - text_shadow[i].blur - 0.5) / gray_target.target().height as f32,
					],
					texture_size: [gray_target.target().width as f32, gray_target.target().height as f32],
					blur_radius: text_shadow[i].blur,
					direcition: GaussDirecition::Horizontal as u8 as f32,
				};
				instances.text_shadow_h_instance_data.instance_data_mut(instance_start).set_data(&meterial);
				log::debug!("shadow h============{:?}", (width, height, instance_start, meterial, &text.rect));
				if is_batch(&mut pre_shadow_h_target, &mut pre_shadow_h_src, h_target.target(), Some(&gray_target.target())) {
					let b = InstanceDrawState {
						instance_data_range: instance_start..instances.text_shadow_h_instance_data.cur_index(),
						pipeline: Some(instances.text_shadow_pipeline.clone()),
						texture_bind_group: Some(bindgroup_map.bind_group(gray_target.target(), &device, &instances.sdf2_texture_layout, &instances.batch_texture.default_sampler).clone()),
					};
					bacth_text_effect.shadow_h.push((b, h_target.target().clone()));
				} else {
					let last = bacth_text_effect.shadow_h.len() - 1;
					bacth_text_effect.shadow_h[last].0.instance_data_range.end = instances.text_shadow_h_instance_data.cur_index();
				}
				tem_shadow_h_target.push(h_target);
			}
			
		}
		tem_gray_target.clear();
		// shadow_v
		let mut shadow_h_i = 0;
		for (_geo, index, _max_blur) in events.shadow.drain(..) {
			let text = &events.text[index];
			let (draw_list, text_shadow) = match query_shadow.get(text.entity) {
				Ok(r) => r,
				_ => continue,
			};

			let rect = &text.rect;
			let (width, height) = ((text.rect.maxs.x - text.rect.mins.x).ceil() as u32, (text.rect.maxs.y - text.rect.mins.y).ceil() as u32);
			
			let draw_id_first = draw_list.get_first(shadow_render_type);
			let shadow_len = text_shadow.len();
			for i in 0..shadow_len {
				let draw_id = &draw_list.0[draw_id_first + i];
				if let Ok((instance_index, mut split)) = query_draw_other.get_mut(draw_id.id) {

					let blur2 = text_shadow[i].blur as u32 * 2;
					let (width, height) = (width + blur2, height + blur2);
					let h_target = &tem_shadow_h_target[shadow_h_i];
					// 该target作为绘制guass 水平方向模糊的目标target
					let v_target = atlas_allocator.allocate(width, height, dyn_target_type.0.no_depth, tem_shadow_h_target[shadow_h_i..shadow_h_i + 1].iter());

					{
						*split = InstanceSplit::ByFbo(Some(v_target.clone()));
						let target_rect = v_target.rect_normalize();
						// 垂直模糊实例
						let instance_start = instances.text_shadow_v_instance_data.alloc_instance_data();
						instances.text_shadow_v_instance_data.instance_data_mut(instance_start).set_data(&BatchGussMeterial {
							box_layout: [target_rect[0], target_rect[1], target_rect[2] - target_rect[0], target_rect[3] - target_rect[1]],
							uv: h_target.uv_box(),
							texture_size: [h_target.target().width as f32, h_target.target().height as f32],
							blur_radius: text_shadow[i].blur,
							direcition: GaussDirecition::Vertical as u8 as f32,
						});
						if is_batch(&mut pre_shadow_v_target, &mut pre_shadow_v_src, v_target.target(), Some(&h_target.target())) {
							let b = InstanceDrawState {
								instance_data_range: instance_start..instances.text_shadow_v_instance_data.cur_index(),
								pipeline: Some(instances.text_shadow_pipeline.clone()),
								texture_bind_group: Some(bindgroup_map.bind_group(h_target.target(), &device, &instances.sdf2_texture_layout, &instances.batch_texture.default_sampler).clone()), // TODO
							};
							bacth_text_effect.shadow_v.push((b, v_target.target().clone()));
						} else {
							let last = bacth_text_effect.shadow_v.len() - 1;
							bacth_text_effect.shadow_v[last].0.instance_data_range.end = instances.text_shadow_v_instance_data.cur_index();
						}
					}

					// 最后的高斯模糊结果渲染到屏幕或目标target上
					{
						let mut instance_data = instances.instance_data.instance_data_mut(instance_index.start);
						let mut ty = instance_data.get_render_ty();
						ty |= 1 << RenderFlagType::R8 as usize;
						instance_data.set_data(&TyMeterial(&[ty as f32]));
						instance_data.set_data(&ColorUniform(text_shadow[i].color.as_slice()));
						instance_data.set_data(&UvUniform(h_target.uv_box().as_slice()));
						instance_data.set_data(&LayoutUniform(&[
							rect.mins.x - text_shadow[i].blur + text_shadow[i].h, 
							rect.mins.y - text_shadow[i].blur + text_shadow[i].v, 
							width as f32, 
							height as f32
						]));
						
					}
					
				}
				shadow_h_i += 1;
			}

		}
		tem_shadow_h_target.clear();
	}

	for (geo, index) in events.outer_glow.drain(..) {
		let text = &events.text[index];
		let (draw_list, text_outer_glow) = match query_outer_glow.get(text.entity) {
			Ok(r) => r,
			_ => continue,
		};
		let rect = &text.rect;
		let distance2 = text_outer_glow.distance as u32 + text_outer_glow.distance as u32;
		let (width, height) = ((text.rect.maxs.x - text.rect.mins.x + distance2 as f32).ceil() as u32, (text.rect.maxs.y - text.rect.mins.y + distance2 as f32).ceil() as u32);
		let range = match geo.polygons {
			PolygonType::Rect(range) => range,
			_ => unreachable!(),
		};

		let outer_glow_target = atlas_allocator.allocate(width, height, dyn_target_type.0.no_depth, e.iter());	
		let instance_start = instances.text_glow_instance_data.cur_index();

		
		let mut i = range.start;
		while  i < range.end {
			let instance_start = instances.text_glow_instance_data.alloc_instance_data();
			let pindex0 = i;
			let pindex1 = i + 2;

			let target_rect = outer_glow_target.rect();
			let meterial = BatchGlowMeterial {
				box_layout: [
					(outer_glow_buffer.positions[pindex0] - rect.mins.x + text_outer_glow.distance + target_rect.min.x as f32) / outer_glow_target.target().width as f32, (outer_glow_buffer.positions[pindex0 + 1] - rect.mins.y + text_outer_glow.distance + target_rect.min.y as f32) / outer_glow_target.target().height as f32,
					(outer_glow_buffer.positions[pindex1] - outer_glow_buffer.positions[pindex0]) / outer_glow_target.target().width as f32, (outer_glow_buffer.positions[pindex1 + 1] - outer_glow_buffer.positions[pindex0 + 1]) / outer_glow_target.target().height as f32,
				],
				sdf_uv: [
					outer_glow_buffer.sdf_uvs[pindex0], outer_glow_buffer.sdf_uvs[pindex0 + 1],
					outer_glow_buffer.sdf_uvs[pindex1], outer_glow_buffer.sdf_uvs[pindex1 + 1],
				],
				fill_bound: text.fill_bound,
			};

			log::debug!("text_glow============{:?}", (width, height, instance_start, &meterial, &outer_glow_buffer.positions[i..i + 4], &text.rect));

			instances.text_glow_instance_data.instance_data_mut(instance_start).set_data(&meterial);

			
			i += 4;
		}

		{
			if is_batch(&mut pre_outer_glow_target, &mut pre_outer_glow_src, outer_glow_target.target(), None) {
				let b = InstanceDrawState {
					instance_data_range: instance_start..instances.text_glow_instance_data.cur_index(),
					pipeline: Some(instances.text_glow_pipeline.clone()),
					texture_bind_group: instances.sdf2_texture_group.clone(), // TODO
				};
				bacth_text_effect.outer_glows.push((b, outer_glow_target.target().clone()));
			} else {
				let last = bacth_text_effect.outer_glows.len() - 1;
				bacth_text_effect.outer_glows[last].0.instance_data_range.end = instances.text_glow_instance_data.cur_index();
			}
		}

		
		// 外发光实例渲染在屏幕上或目标fbo上的实例数据填充
		{
			let draw_id = match draw_list.get_one(outer_glow_render_type) {
				Some(r) => r.id,
				None => continue,
			};

			if let Ok((instance_index, mut split)) = query_draw_other.get_mut(draw_id) {
				let mut instance_data = instances.instance_data.instance_data_mut(instance_index.start);
				let mut ty = instance_data.get_render_ty();
				ty |= 1 << RenderFlagType::R8 as usize;
				instance_data.set_data(&TyMeterial(&[ty as f32]));
				instance_data.set_data(&ColorUniform(text_outer_glow.color.as_slice()));
				instance_data.set_data(&UvUniform(outer_glow_target.uv_box().as_slice()));
				instance_data.set_data(&LayoutUniform(&[
					rect.mins.x - text_outer_glow.distance, 
					rect.mins.y - text_outer_glow.distance, 
					width as f32, 
					height as f32
				]));
				log::debug!("outer_glow render to screen======{:?}", (instance_index.start, &rect, text_outer_glow.distance, width, height));
				*split = InstanceSplit::ByFbo(Some(outer_glow_target.clone()));
			}
		}
	}

	for text in events.text.drain(..) {
		let (draw_list, text_style) = match query_text.get(text.entity) {
			Ok(r) => r,
			_ => continue,
		};
		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};

		let stroke_color = &text_style.text_stroke.color;
		
		let instance_index = match query_draw.get_mut(draw_id) {
			Ok(r) => r,
			_ => continue,
		};
		// 节点可能设置为dispaly none， 此时instance_index可能为Null
		if pi_null::Null::is_null(&instance_index.0.start) {
			continue;
		}
		let start = instance_index.start;

		let mut ty = instances.instance_data.instance_data_mut(start).get_render_ty();
		ty &= !(1 << RenderFlagType::Invalid as usize);
		if *text_style.text_stroke.width > 0.0 {
			ty |= 1 << RenderFlagType::Stroke as usize;
		} else {
			ty &= !(1 << RenderFlagType::Stroke as usize);
		}
		let end = text.text_geo.set_instance_data(start, &mut instances, Some(&OtherInfo {
			sdf_info: [text.px_range, text.fill_bound, text.stroke_bound],
			stroke_color: [stroke_color.x, stroke_color.y, stroke_color.z, stroke_color.w],
			ty: ty as f32,
		}), buffer);

		if end < instance_index.end {
			// 设多余的实例为无效实例
			ty |= 1 << RenderFlagType::Invalid as usize;
			instances.instance_data.set_data_mult1(end..instance_index.end, &TyMeterial(&[ty as f32]));
		}
	}
	
	buffer.clear();
	outer_glow_buffer.clear();
}


struct UniformData<'a> {
	node_state: &'a NodeState,
	layout: &'a LayoutResult,
	own_layout: &'a LayoutResult,
	text_shadow: Option<&'a TextShadow>,
	text_overflow_data: Option<&'a TextOverflowData>,
	text_outer_glow: Option<&'a TextOuterGlow>,
	text_style: &'a TextStyle,
	font_sheet: &'a mut FontSheet,

	fontsize: f32, // 字体大小
	half_stroke: f32, // 描边的一半宽度
}

#[derive(Debug)]
pub struct TextEvent {
	entity: Entity,
	// not_vnode: Entity, // 非虚拟节点的父节点

	text_geo: TempGeo,
	rect: Aabb2, // 文本区域

	px_range: f32, fill_bound: f32, stroke_bound: f32,
}

#[derive(Debug, Default)]
pub struct TextTemp {
	buffer: TempGeoBuffer,
	outer_glow_buffer: TempGeoBuffer,
	text: Vec<TextEvent>,
	shadow: Vec<(TempGeo, usize, usize/*max_blur*/)>, // (阴影，文字轮廓渲染在第几个索引：text数组中的索引)
	outer_glow: Vec<(TempGeo, usize)>, // (外发光，文字轮廓渲染在第几个索引：text数组中的索引)
}

pub struct TextResult {
    text: TextEvent,
    outer_glow: TempGeo,
	instance_count: usize,
}


impl<'a> UniformData<'a> {
	fn geo(&self, entity: Entity, temp: &mut TextTemp) -> usize {
		let font_size = get_size(&self.text_style.font_size) as f32;		
		let font_size_scale = font_size.max(0.0001) / sdf_font_size(font_size as usize) as f32;
		let px_range = pi_hal::font::sdf2_table::PXRANGE as f32 * font_size_scale * 2.0;
		let fill_bound = 0.5 - (self.text_style.font_weight as f32 / 500 as f32 - 1.0) / px_range;
		let stroke_bound = fill_bound - (*self.text_style.text_stroke.width)/2.0/font_size_scale/(pi_hal::font::sdf2_table::PXRANGE as f32 * 2.0);

		let mut calc_result = TextResult {
			
			text: TextEvent {
				entity,

				text_geo: Default::default(),
				rect: Aabb2::new(Point2::new(std::f32::INFINITY, std::f32::INFINITY), Point2::new(-std::f32::INFINITY, -std::f32::INFINITY)),
				px_range,
				fill_bound,
				stroke_bound,
			},
			outer_glow: Default::default(),
			instance_count: 0,
		};

		let text_start = temp.buffer.positions.len();
		let ouetr_glow_start = temp.outer_glow_buffer.positions.len();

		let offset = (self.layout.border.left + self.layout.padding.left, self.layout.border.top + self.layout.padding.top);
		let mut word_pos = offset.clone();
		let mut count = 0;

		// 文字是否存在换行（如果存在换行， text_overflow无效）
		let start_y = self.node_state.0.text[0].pos.top + offset.1; 
		let text_overflow = calc_text_overflow_data(self.text_overflow_data, self.text_style);

		let mut line_max = 0.0;
		if text_overflow.0 {
			line_max = self.layout.rect.right - self.layout.border.right - self.layout.padding.right - self.layout.border.left - self.layout.padding.left - self.layout.rect.left;
		}
		
		for c in self.node_state.0.text.iter() {
			if c.ch == char::from(0) {
				if c.count > 0 {
					word_pos = (c.pos.left + offset.0, c.pos.top + offset.1);
					// log::debug!("pos==================={:?}", c.pos);
					count = c.count - 1;
				}
				continue;
			}
			if c.ch <= ' ' {
				continue;
			}

			let mut left = word_pos.0 + c.pos.left;
			let w = c.pos.right - c.pos.left;
			let h = c.pos.bottom - c.pos.top;
			let top = word_pos.1 + c.pos.top;

			if text_overflow.0 && c.pos.top == start_y && left + w + text_overflow.1 > line_max {
				if let Some(text_overflow_data) = &self.text_overflow_data {
					let mut max = 1;
					if let TextOverflow::Ellipsis = text_overflow_data.text_overflow {
						max = 3;
					}
					let mut i = 0;
					while i < max {
						for c1 in text_overflow_data.text_overflow_char.iter() {
							self.push_pos_uv(
								&mut calc_result,
								temp,
								left + self.text_style.letter_spacing, 
								top,
								h,
								c,
							);
							
							left += c1.width + self.text_style.letter_spacing;
						}
						i += 1;
					}
				}
				break;
			}

			
			self.push_pos_uv(
				&mut calc_result,
				temp,
				left, 
				top,
				h,
				c,
			);

			if count > 0 {
				count -= 1;
				if count == 0 {
					word_pos = offset;
				}
			}
		}

		let char_len = (temp.buffer.positions.len() - text_start) / 4;
		let text_end = temp.buffer.positions.len();

		// calc_result.outer_glow_geo.polygons = PolygonType::Rect;  // 规则的四边形
		if let Some(text_shadow) = self.text_shadow {
			let mut max_blur = 0.0_f32;
			for i in text_shadow.iter() {
				max_blur = max_blur.max(i.blur.ceil());
			}
			temp.shadow.push((
				TempGeo {
					polygons: PolygonType::Rect(text_start..temp.buffer.positions.len()),
					colors: VColor::CgColor(CgColor::new(0.0, 0.0, 0.0, 1.0)),
					sdf_px_range: 1.0,
				},
				temp.text.len(),
				max_blur as usize,
			));
		}

		if let Some(_outer_glow) = self.text_outer_glow {
			calc_result.outer_glow.polygons = PolygonType::Rect(ouetr_glow_start..temp.outer_glow_buffer.positions.len());
			temp.outer_glow.push((calc_result.outer_glow, temp.text.len()));
		}

		
		calc_result.text.text_geo.polygons = PolygonType::Rect(text_start..text_end);  // 规则的四边形
		match &self.text_style.color {
			Color::RGBA(r) => {
				calc_result.instance_count += char_len;
				log::debug!("text cgcolor============char_len: {:?}", char_len);
				calc_result.text.text_geo.colors = VColor::CgColor(r.clone());
			},
			Color::LinearGradient(color) => {
				calc_result.text.text_geo.linear_gradient_split(color, &self.own_layout.padding_rect(), &mut temp.buffer);
				let out_indices = match &calc_result.text.text_geo.polygons {
					PolygonType::NoRule(indices) => {
						let mut out_indices = Vec::with_capacity(indices.counts.len() * 4); // 预计多边形为四边形
						mult_to_triangle(&indices, &mut out_indices);
						out_indices
					},
					_ => todo!(), // 不会是三角形和规则多边形
				};
				calc_result.instance_count += out_indices.len() / 3;
				log::debug!("text LinearGradient============instance_count: {:?}", calc_result.instance_count);
				calc_result.text.text_geo.polygons = PolygonType::Triangle(out_indices);
			},
		}
		temp.text.push(calc_result.text);
		calc_result.instance_count
		

	}

	fn push_pos_uv(
		&self, 
		result: &mut TextResult,
		temp: &mut TextTemp,
		x: f32, 
		y: f32,
		h: f32,
		c: &CharNode,
	) {

		// log::debug!("glyph!!!==================={:?}, {:?}, {x:?}, {y:?}", c.ch_id, c.ch);
		let glyph = self.font_sheet.font_mgr().table.sdf2_table.glyph(GlyphId(c.ch_id));
		let metrics = match self.font_sheet.font_mgr().metrics(GlyphId(c.ch_id)) {
			Some(r) => r,
			None => return
		};
		// log::warn!("calc_result========c: {:?}, ch_id: {:?}, glyph: {:?}", c.ch, c.ch_id, glyph,  );


		if let Some(text_outer_glow) = self.text_outer_glow {
			let glyph_outer_glow = self.font_sheet.font_mgr().table.sdf2_table.font_outer_glow_info.get(&(GlyphId(c.ch_id), text_outer_glow.distance as u32)).unwrap();
			log::debug!("push_pos_uv outer_glow============={:?}", glyph_outer_glow);
			push_pos_uv(
				&mut temp.outer_glow_buffer.positions,
				&mut temp.outer_glow_buffer.sdf_uvs,
				x, 
				y,
				h,
				self.fontsize,
				text_outer_glow.distance,
				glyph_outer_glow,
				metrics,
				// false
			);
		}

		push_pos_uv(
			&mut temp.buffer.positions,
			&mut temp.buffer.sdf_uvs,
			x, 
			y,
			h,
			self.fontsize,
			self.half_stroke,
			glyph,
			metrics,
			// self.is_linear
		);
		let font_line_height = metrics.line_height * self.fontsize;
		let plane_min_x = x + glyph.plane_min_x * self.fontsize;
		let plane_min_y = y + ((h - font_line_height) / 2.0)/*上下一半剩余行高的空间*/ + (metrics.ascender - glyph.plane_max_y) * self.fontsize;
	
		let plane_width = (glyph.plane_max_x - glyph.plane_min_x) * self.fontsize;
		let plane_height = (glyph.plane_max_y - glyph.plane_min_y) * self.fontsize;

		result.text.rect.mins.x = result.text.rect.mins.x.min(plane_min_x);
		result.text.rect.mins.y = result.text.rect.mins.y.min(plane_min_y);
		result.text.rect.maxs.x = result.text.rect.maxs.x.max(plane_min_x + plane_width);
		result.text.rect.maxs.y = result.text.rect.maxs.y.max(plane_min_y + plane_height);
	}
	
}

#[inline]
fn calc_text_overflow_data(text_overflow_data: Option<&TextOverflowData>, text_style: &TextStyle) -> (bool, f32) {
	if let Some(text_overflow_data) = text_overflow_data {
		let width = match &text_overflow_data.text_overflow {
			pi_style::style::TextOverflow::None => return (false, 0.0),
			pi_style::style::TextOverflow::Clip => return (true, 0.0),
			pi_style::style::TextOverflow::Ellipsis => text_overflow_data.text_overflow_char[0].width * 3.0 + text_style.letter_spacing * 3.0,
			pi_style::style::TextOverflow::Custom(_) => {
				let mut width = text_style.letter_spacing * text_overflow_data.text_overflow_char.len() as f32;
				for c in text_overflow_data.text_overflow_char.iter() {
					width += c.width;
				}
				width
			},
		};
		return (true, width)
	}
	(false, 0.0)
}

fn push_pos_uv(
	positions: &mut Vec<f32>,
	uvs: &mut Vec<f32>,
	x: f32, 
	y: f32,
	line_height: f32,
	fontsize: f32,
	half_extend: f32, // 一半的扩展宽度（描边， 阴影， 外法光都需要额外扩展）
	glyph: &Glyph,
	metrics: &MetricsInfo,
	// is_linear: bool,
) {
	log::debug!("push_pos_uv============={:?}", glyph);
	
	// let layout_atlas_ratio = width/(glyph.advance * metrics.font_size); // `布局尺寸`与`固定字号字体宽度的比值`
	let font_line_height = metrics.line_height * fontsize;
	let stroke_width = half_extend * 2.0;

	let plane_min_x = x + glyph.plane_min_x * fontsize - half_extend;
	// y + ((line_height - font_line_height) / 2.0)为top线的位置
	let plane_min_y = y + ((line_height - font_line_height) / 2.0)/*上下一半剩余行高的空间*/ + (metrics.ascender - glyph.plane_max_y) * fontsize - half_extend;
	

	let scale = fontsize / metrics.font_size;
	let plane_width = (glyph.plane_max_x - glyph.plane_min_x) * fontsize + stroke_width;
	let plane_height = (glyph.plane_max_y - glyph.plane_min_y) * fontsize + stroke_width;

	let plane_max_x = plane_min_x + plane_width;
	let plane_max_y = plane_min_y + plane_height;

	let half_stroke_uv = half_extend*scale;

	// 否则， push aabb
	let ps = [
		plane_min_x,
		plane_min_y,
		plane_max_x,
		plane_max_y,
	];

	let uv = [
		glyph.x - half_stroke_uv,
		glyph.y - half_stroke_uv,
		glyph.x + glyph.width + half_stroke_uv,
		glyph.y + glyph.height + half_stroke_uv,
	];

	// log::warn!("push_pos_uv1============={:?}, {:?}, uv: {:?}, uv_start_index: {:?}， ps: {:?}, ps_start_index: {:?}", [
	// 	plane_min_x,
	// 	plane_min_y,
	// 	plane_max_x,
	// 	plane_max_y,
	// 	fontsize,
	// ],  [
	// 	half_stroke_uv,
	// 	fontsize,
	// 	metrics.font_size,
		
	// ], uv, uvs.len(), ps,  positions.len());
	uvs.extend_from_slice(&uv);
	positions.extend_from_slice(&ps[..]);
}



#[derive(SystemParam)]
pub struct QueryParam<'w> {
    // camera_query: Query<'w, 's, &'static Camera>,

	instance_draw: OrInitSingleRes<'w, InstanceContext>,
	bacth_text_effect: SingleRes<'w, BatchTextEffect>,
    // clear_draw: SingleRes<'w, ClearDrawObj>,
    // depth_cache: OrInitSingleRes<'w, DepthCache>,
}

pub fn init_text_effect_graph(
	mut rg: SingleResMut<PiRenderGraph>,
) {
	let effect_graph_id = rg.add_sub_graph("gui_effect_graph").unwrap();
	let node = TextEffectNode;
	let _id = rg.add_node("GuiTextEffectNode", node, effect_graph_id).unwrap();

	// 将其设置在所有gui节点之前运行
	let main_graph_id = rg.main_graph_id();
	let _ = rg.add_depend(effect_graph_id, main_graph_id);
}

/// 渲染图节点， 用于将文字做模糊处理（draw_front）
pub struct TextEffectNode;

impl Node for TextEffectNode {
    type Input = ();
    type Output = ();

    type RunParam = QueryParam<'static>;
	type BuildParam = ();
	// 释放纹理占用
	fn reset<'a>(
		&'a mut self,
	) {
		// self.out_put_target = None;
		// self.target = None;
	}

	/// 用于给pass2d分配fbo
	fn build<'a>(
		&'a mut self,
		// world: &'a mut pi_world::world::World,
		_param: &'a mut Self::BuildParam,
		_context: pi_bevy_render_plugin::RenderContext,
		_input: &'a Self::Input,
		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
		_id: GraphNodeId,
		_from: &'a [GraphNodeId],
		_to: &'a [GraphNodeId],
	) -> Result<Self::Output, String> {
		Ok(())
	}

	fn run<'a>(
        &'a mut self,
        param: &'a Self::RunParam,
        _context: RenderContext,
        commands: ShareRefCell<CommandEncoder>,
        _input: &'a Self::Input,
        _usage: &'a ParamUsage,
        _id: GraphNodeId,
        _from: &'a [GraphNodeId],
        _to: &'a [GraphNodeId],

    ) -> BoxFuture<'a, Result<(), String>> {
		Box::pin(async move {
			let mut commands = commands.borrow_mut();
			let mut pre = None;
			let mut rp: Option<RenderPass> = None;

			let mut state = RenderState {
				reset: true,
				pipeline: param.instance_draw.common_pipeline.clone(),
				texture: param.instance_draw.batch_texture.default_texture_group.clone(),
			};			

			for (draw, fbo) in param.bacth_text_effect.shadow_gray.iter() {
				if is_change(&mut pre, fbo) {
					{let _a = rp;}
					rp = Some(c_rp(&mut commands, fbo));
					let rp = rp.as_mut().unwrap();
					state.reset = true;
					param.instance_draw.set_pipeline(rp, draw, &mut state);
					let group = param.instance_draw.default_camera.get_group();
					rp.set_bind_group(0, &group.bind_group, group.offsets);
					state.reset = true;
				}
				let rp = rp.as_mut().unwrap();
				param.instance_draw.draw_effect(rp, draw,  &param.instance_draw.text_gray_instance_buffer, &param.instance_draw.text_gray_instance_data, &mut state);
			}

			for (draw, fbo) in param.bacth_text_effect.shadow_h.iter() {
				if is_change(&mut pre, fbo) {
					{let _a = rp;}
					rp = Some(c_rp(&mut commands, fbo));
					state.reset = true;
					let rp = rp.as_mut().unwrap();
					param.instance_draw.set_pipeline(rp, draw, &mut state);
					let group = param.instance_draw.default_camera.get_group();
					rp.set_bind_group(0, &group.bind_group, group.offsets);
					
				}
				let rp = rp.as_mut().unwrap();
				param.instance_draw.set_pipeline(rp, draw, &mut state);
				param.instance_draw.draw_effect(rp, draw,  &param.instance_draw.text_shadow_h_instance_buffer, &param.instance_draw.text_shadow_h_instance_data, &mut state);
			}

			state.reset = true;
			for (draw, fbo) in param.bacth_text_effect.shadow_v.iter() {
				if is_change(&mut pre, fbo) {
					{let _a = rp;}
					rp = Some(c_rp(&mut commands, fbo));
					let rp = rp.as_mut().unwrap();
					state.reset = true;
					param.instance_draw.set_pipeline(rp, draw, &mut state);
					let group = param.instance_draw.default_camera.get_group();
					rp.set_bind_group(0, &group.bind_group, group.offsets);
					
				}
				let rp = rp.as_mut().unwrap();
				param.instance_draw.set_pipeline(rp, draw, &mut state);
				param.instance_draw.draw_effect(rp, draw,  &param.instance_draw.text_shadow_v_instance_buffer,  &param.instance_draw.text_shadow_v_instance_data,&mut state);
			}

			state.reset = true;
			for (draw, fbo) in param.bacth_text_effect.outer_glows.iter() {
				if is_change(&mut pre, fbo) {
					{let _a = rp;}
					rp = Some(c_rp(&mut commands, fbo));
					state.reset = true;
					let rp = rp.as_mut().unwrap();
					param.instance_draw.set_pipeline(rp, draw, &mut state);
					let group = param.instance_draw.default_camera.get_group();
					rp.set_bind_group(0, &group.bind_group, group.offsets);
					
				}
				
				let rp = rp.as_mut().unwrap();
				param.instance_draw.set_pipeline(rp, draw, &mut state);
				param.instance_draw.draw_effect(rp, draw,  &param.instance_draw.text_glow_instance_buffer, &param.instance_draw.text_glow_instance_data, &mut state);
			}

			Ok(())
		})
	}
}

fn is_change<'a: 'b, 'b> (pre: &'b mut Option<Share<Fbo>>, fbo: &'a Share<Fbo>) -> bool{
	match &pre {
		Some(pre_fbo) => {
			if Share::ptr_eq(pre_fbo, fbo) {
				return false;
			}
		},
		None => (),
	};
	*pre = Some(fbo.clone());
	true
}

fn c_rp<'a: 'b, 'b> (commands: &'a mut CommandEncoder, fbo: &'a Share<Fbo>) -> RenderPass<'a> {
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

#[derive(Default)]
pub struct BatchTextEffect {
	shadow_gray: Vec<(InstanceDrawState, Share<Fbo>)>,
	shadow_h: Vec<(InstanceDrawState, Share<Fbo>)>,
	shadow_v: Vec<(InstanceDrawState, Share<Fbo>)>,

	outer_glows: Vec<(InstanceDrawState, Share<Fbo>)>,
}


#[derive(Default)]
pub struct FboBindGroups {
	map: XHashMap<KeyData, Share<wgpu::BindGroup>>,
}

impl FboBindGroups {
	pub fn bind_group(&mut self, fbo: &Share<Fbo>, device: &wgpu::Device, layout: &BindGroupLayout, common_sampler: &Sampler) -> &Share<wgpu::BindGroup> {
		let target = &fbo.colors[0];
		match self.map.entry(target.0.id.clone()) {
			Entry::Occupied(e) => e.into_mut(),
			Entry::Vacant(e) => {
			    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
					label: Some("fbo bind group"),
					layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::TextureView(&target.0.texture_view),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::Sampler(&common_sampler),
						}
					],
				});
				e.insert(Share::new(bind_group))
			}
		}
	}
}