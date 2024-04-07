use crate::components::user::{SvgFilter, SvgFilterBlurLevel, SvgFilterOffset, SvgInnerContent};

use bevy_ecs::{prelude::Entity, query::Changed, system::Query};
use pi_bevy_ecs_extend::system_param::{res::OrInitRes, tree::Up};
use pi_style::style::CgColor;


use crate::system::draw_obj::calc_text::IsRun;
// use super::IsRun;


pub fn flter_blur(
    query: Query<(Entity, &'static SvgFilterBlurLevel), Changed<SvgFilterBlurLevel>>,
    r: OrInitRes<IsRun>,
    query_parent: Query<&Up>,
    query_flter: Query<&SvgFilter>,
    mut query_svg: Query<&mut SvgInnerContent>,
) {
    if r.0 {
        return;
    }

    for (entity, blur_level) in query.iter() {
        log::debug!("========== flter_blur000: {:?}", entity);
        if let Ok(flter_id) = query_parent.get(entity) {
            log::debug!("========== flter_blur11: {:?}", flter_id);
            if let Ok(flter) = query_flter.get(flter_id.parent()) {
                log::debug!("========== flter_blur: {:?}, flter:{:?}", blur_level, flter);
                for id in &flter.0 {
                    if let Ok(mut svg) = query_svg.get_mut(*id) {
                        
                        svg.style.shadow.blur_level = blur_level.level;
                    }
                }
            }
        }
    }
}

pub fn flter_offset(
    query: Query<(Entity, &'static SvgFilterOffset), Changed<SvgFilterOffset>>,
    r: OrInitRes<IsRun>,
    query_parent: Query<&Up>,
    query_flter: Query<&SvgFilter>,
    mut query_svg: Query<&mut SvgInnerContent>,
) {
    if r.0 {
        return;
    }

    for (entity, offset) in query.iter() {
        log::debug!("========== flter_offset00000: {:?}", entity);
        if let Ok(flter_id) = query_parent.get(entity) {
            log::debug!("========== flter_offset1111: {:?}", flter_id);
            if let Ok(flter) = query_flter.get(flter_id.parent()) {
                log::debug!("========== flter_offset2222: {:?}", flter);
                for id in &flter.0 {
                    if let Ok(mut svg) = query_svg.get_mut(*id) {
                        log::debug!("========== flter_offset: {:?}", offset);
                        svg.style.shadow.offset_x = offset.offset_x;
                        svg.style.shadow.offset_y = offset.offset_y;
                        svg.style.shadow.color = if (offset.color - 0.0).abs() > 0.1{
                            // svg.style.fill_color
                            // todo
                            CgColor::new(0.1,0.1,0.1,1.0)
                        }else{
                            CgColor::new(0.1,0.1,0.1,1.0)
                        }
                    }
                }
            }
        }
    }
}
