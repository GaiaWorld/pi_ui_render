use crate::components::user::{SvgFilterBlurLevel, SvgGradient, SvgInnerContent, SvgStop, Vector2};
use bevy_ecs::{prelude::Entity, query::Changed, system::Query};
use pi_bevy_ecs_extend::system_param::{res::OrInitRes, tree::Up};


use pi_style::style::{Color, ColorAndPosition, LinearGradientColor};

use crate::system::draw_obj::calc_text::IsRun;
// use super::IsRun;

pub fn gradient_offset(
    query: Query<(Entity, &'static SvgGradient), Changed<SvgGradient>>,
    r: OrInitRes<IsRun>,
    mut query_svg: Query<&mut SvgInnerContent>,
) {
    if r.0 {
        return;
    }

    for (entity, gradient) in query.iter() {
        let dir = (Vector2::new(gradient.x2, gradient.y2) - Vector2::new(gradient.x1, gradient.y1)).normalize();
        let cos_theta = dir.x;
        let theta = cos_theta.acos();
        let degrees = theta.to_degrees();
        log::debug!(
            "============ gradient_offset: entity: {:?}, {}, gradient: {:?}, gradient.id: {:?}",
            entity,
            degrees,
            gradient,
            gradient.id
        );

        for id in &gradient.id {
            if let Ok(mut svg) = query_svg.get_mut(*id) {
                if let Color::LinearGradient(linear_lradient) = &mut svg.style.fill_color {
                    linear_lradient.direction = degrees;
                } else {
                    let mut linear_lradient = LinearGradientColor::default();
                    linear_lradient.direction = degrees;
                    svg.style.fill_color = Color::LinearGradient(linear_lradient);
                }
            }
        }
    }
}

/// 文字字形计算
pub fn gradient_stop(
    query: Query<(Entity, &'static SvgStop), Changed<SvgStop>>,
    r: OrInitRes<IsRun>,
    query_parent: Query<&Up>,
    query_gradient: Query<&SvgGradient>,
    mut query_svg: Query<&mut SvgInnerContent>,
) {
    if r.0 {
        return;
    }

    for (entity, stop) in query.iter() {
        log::debug!("============0000 gradient_stop: {:?}", stop);
        if let Ok(gradient_id) = query_parent.get(entity) {
            if let Ok(gradient) = query_gradient.get(gradient_id.parent()) {
                log::debug!("============1111 gradient_stop: {:?}", gradient);
                for id in &gradient.id {
                    if let Ok(mut svg) = query_svg.get_mut(*id) {
                        log::debug!("============ gradient_stop: {:?}", stop);
                        let v = ColorAndPosition {
                            position: stop.offset / 100.0,
                            rgba: stop.color.clone(),
                        };
                        if let Color::LinearGradient(linear_lradient) = &mut svg.style.fill_color {
                            linear_lradient.list.push(v);
                        } else {
                            let mut linear_lradient = LinearGradientColor::default();
                            linear_lradient.list.push(v);
                            svg.style.fill_color = Color::LinearGradient(linear_lradient);
                        }
                    }
                }
            }
        }
    }
}
