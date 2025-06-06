// pub mod filter;
// pub mod gradient;
// pub mod text_glyph;
pub mod text_sdf2;
// mod text_split;

use crate::components::draw_obj::{RenderCount, SvgOuterGlowMark, SvgShadowMark};
use crate::components::user::{SvgColor, SvgShadow};
use crate::prelude::UiStage;
use crate::resource::{SvgOuterGlowRenderObjType, SvgShadowRenderObjType};
use crate::system::base::node::layout::calc_layout;
use crate::{
    components::{
        draw_obj::{BoxType, SvgMark},
        user::{Shape, SvgInnerContent},
    },
    resource::{IsRun, SvgRenderObjType},
    system::system_set::UiSystemSet,
};

// use self::svg_main::{calc_sdf2_svg, svg_glyph};
// use self::filter::{flter_blur, flter_offset};
// use self::gradient::{gradient_offset, gradient_stop};
use crate::system::base::draw_obj::life_drawobj::{draw_object_life_new, update_render_instance_data};

use std::collections::HashMap;


// use pi_world::change_detection::DetectChanges;
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};
use pi_hal::svg::SvgInfo;
use pi_hal::{
    font::sdf2_table::TexInfo,
    // pi_sdf::{self, glyphy::geometry::aabb::AabbEXT},
    runtime::MULTI_MEDIA_RUNTIME,
};
use pi_world::{
    filter::Or,
    prelude::{App, Changed, Entity, Local, ParamSet, Plugin, Query, SingleResMut, With},
    schedule_config::IntoSystemConfigs,
};

use pi_render::font::FontType;
use pi_share::{Share, ShareMutex};
use pi_style::style::Color;
use text_sdf2::{calc_sdf2_svg, calc_sdf2_text_len, init_svg_effect_graph, svg_change};

use crate::resource::ShareFontSheet;

pub const SVG_ORDER: u8 = 8;
pub const SVG_OUTER_GLOW_ORDER: u8 = 12;
pub const SVG_SHADOW_ORDER: u8 = 13;
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        println!("add SvgPlugin");
        app
            // .add_frame_event::<ComponentEvent<Changed<SvgInnerContent>>>()
            .add_startup_system(UiStage, init_svg_effect_graph)
            .add_system(
                UiStage,
                svg_glyph.in_set(UiSystemSet::Layout), // .before(update_sdf2_texture)
            )
            // 创建drawobj
            .add_system(
                UiStage,
                draw_object_life_new::<SvgInnerContent, SvgRenderObjType, (SvgMark, RenderCount), { SVG_ORDER }, { BoxType::None }>
                    .in_set(UiSystemSet::LifeDrawObject)
                    .after(svg_glyph),
            )
            // .add_system(
            //     UiStage,
            //     draw_object_life_new::<SvgInnerContent, SvgOuterGlowRenderObjType, SvgOuterGlowMark, { SVG_OUTER_GLOW_ORDER }, { BoxType::None }>
            //         .in_set(UiSystemSet::LifeDrawObject)
            //         .after(svg_glyph),
            // )
            .add_system(
                UiStage,
                draw_object_life_new::<SvgShadow, SvgShadowRenderObjType, SvgShadowMark, { SVG_SHADOW_ORDER }, { BoxType::None }>
                    .in_set(UiSystemSet::LifeDrawObject)
                    .after(svg_glyph),
            )
            // 更新实例数据
            // .add_system(UiStage, calc_svg.in_set(UiSystemSet::PrepareDrawObj))
            // .add_system(UiStage, flter_blur.in_set(UiSystemSet::PrepareDrawObj))
            // .add_system(UiStage, flter_offset.in_set(UiSystemSet::PrepareDrawObj))
            // .add_system(UiStage, gradient_offset.in_set(UiSystemSet::PrepareDrawObj))
            // .add_system(UiStage, gradient_stop.in_set(UiSystemSet::PrepareDrawObj))
            .add_system(
                UiStage,
                calc_sdf2_text_len
                    .after(UiSystemSet::LifeDrawObjectFlush)
                    .before(update_render_instance_data)
                    .after(UiSystemSet::Layout)
                    .in_set(UiSystemSet::IsRun),
            )
            .add_system(UiStage, calc_sdf2_svg.in_set(UiSystemSet::PrepareDrawObj).run_if(svg_change));
    }
}


pub struct SvgShapeAwaitList(
    pub  Share<
        ShareMutex<
            Vec<(
                Vec<Entity>,
                Share<ShareMutex<(usize, Vec<(u64, TexInfo, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>)>>,
            )>,
        >,
    >,
    ShareMutex<HashMap<u64, SvgInfo>>,
);

impl Default for SvgShapeAwaitList {
    fn default() -> Self { Self(Share::new(ShareMutex::new(Vec::new())), ShareMutex::new(HashMap::default())) }
}

/// svg形状计算（贝塞尔曲线晶格化）
pub fn svg_glyph(
    mut query: Query<(Entity, &'static mut SvgInnerContent), Changed<SvgInnerContent>>,
    // query2: Query<(Entity, &'static mut SvgShadow)>,
    font_sheet: SingleResMut<ShareFontSheet>,
    // mut event_writer: EventWriter<ComponentEvent<Changed<SvgInnerContent>>>,
    r: OrInitSingleRes<IsRun>,
    await_list: Local<SvgShapeAwaitList>,
    // query_view_box: Query<&SvgContent>,
) {
    // if r.0 {
    //     return;
    // }
    // println!("=========1text_svg");
    let mut font_sheet = font_sheet.borrow_mut();

    // let mut await_set_gylph = Vec::new();
    // for (e, s) in  query2.iter(){
    //     println!("============== 1: {:?}", (e, s));
    // }
    // let
    for (entity, mut node_state) in query.iter_mut() {
        println!("============== 1");
        log::error!(
            "entity: {:?}, node_state: {:?}, {:?}",
            entity,
            node_state.style,
            node_state.shape
        );
        if node_state.shape.is_ready() {
            // if let Some(shape) = node_state.shape.take() {
            // let hash = node_state.shape.hash();
            let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;
            
            let info= match node_state.shape.clone() {
                Shape::Rect { x, y, width, height } => pi_hal::svg::Rect::new(x, y, width, height).get_svg_info(),
                Shape::Circle { cx, cy, radius } => pi_hal::svg::Circle::new(cx, cy, radius).unwrap().get_svg_info(),
                Shape::Ellipse { cx, cy, rx, ry } => pi_hal::svg::Ellipse::new(cx, cy, rx, ry).get_svg_info(),
                Shape::Segment { ax, ay, bx, by } => {
                    let step = if node_state.style.stroke_dasharray.real < 100000.0 && node_state.style.stroke_dasharray.empty > 0.001 {
                        Some(vec![node_state.style.stroke_dasharray.real, node_state.style.stroke_dasharray.empty])
                    } else {
                        None
                    };
                    pi_hal::svg::Segment::new(ax, ay, bx, by, step).get_svg_info()
                }
                Shape::Polygon { points } => pi_hal::svg::Polygon::new(points).get_svg_info(),
                Shape::Polyline { points } => pi_hal::svg::Polyline::new(points).get_svg_info(),
                Shape::Path { points, verb } => pi_hal::svg::Path::new(verb, points).get_svg_info(),
            };

            let binding_box= &info.binding_box;
            node_state.bbox = (binding_box[0], binding_box[1], binding_box[2], binding_box[3]);
            // 如果不是闭合曲线，将描边宽度除以2，并将填充颜色设置为描边颜色;适配shader算法
            if !info.is_area {
                let sc = node_state.style.stroke.color.clone();
                if let SvgColor::Color(Color::RGBA(c))  = &mut node_state.style.fill_color {
                    *c = sc;
                };
                node_state.style.stroke.width *= 0.5;
            }

            node_state.is_area = info.is_area;
            println!("info.binding_box: {:?}", binding_box);
            let size = (binding_box[2] - binding_box[0])
                .max(binding_box[3] - binding_box[1])
                .ceil();

            let tex_size = info.tex_size;

            let hash = info.hash;
            node_state.hash = info.hash;
           
            // node_state.scale = size / info.tex_size;
            println!("=========== node_state.scale: {:?}", (node_state.scale, size, tex_size, hash));
            let mut map = await_list.1.lock().unwrap();
            if let Some(info) = map.get(&hash) {
                node_state.svg_info = info.clone();
                node_state.scale = size / info.tex_size;
            } else {
                node_state.svg_info = info.clone();
                map.insert(hash, info.clone());
                node_state.scale = size / info.tex_size;
                // await_set_gylph.push(entity);
                log::error!("add_shape!! hash: {:?}", (hash, &node_state.shape));

                let pxrang = 5;
                let cut_off = 2;
                sdf2_table.add_shape(hash, info, tex_size as usize, pxrang, cut_off);
            }
        }
    }
}
