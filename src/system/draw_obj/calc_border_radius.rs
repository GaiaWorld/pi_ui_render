
use std::hash::{Hash, Hasher};

use guillotiere::euclid::num::Ceil;
use nalgebra::ComplexField;
use pi_flex_layout::prelude::Rect;
use pi_hal::svg::{Path, PathVerb, SvgInfo};
// use pi_sdf::shape::SvgInfo;
// use pi_sdf::shape::PathVerb;
use pi_style::style::StyleType;
use pi_world::filter::Or;
/// 为圆角设置渲染数据

use pi_world::prelude::{Changed, Query, ComponentRemoved};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes};
use pi_world::schedule_config::IntoSystemConfigs;
use pi_world::single_res::{SingleRes, SingleResMut};
use pi_world::prelude::Plugin;
use crate::components::root::{RootScale, Viewport};
use crate::prelude::UiStage;

use crate::components::calc::{style_bit, LayoutResult, SdfSlice, SdfUv, StyleBit, StyleMarkType, WorldMatrix, LAYOUT_DIRTY};

use crate::components::user::BorderRadius;
use crate::resource::{GlobalDirtyMark, OtherDirtyType, ShareFontSheet};
use crate::system::base::node::world_matrix;
use crate::system::system_set::UiSystemSet;
use crate::utils::tools::{cal_border_radius, calc_hash, eq_f32, BorderRadiusPixel};

use crate::resource::IsRun;

pub struct BorderRadiusPlugin;

impl Plugin for BorderRadiusPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		 // BackgroundColor功能
		app
		// 圆角
        .add_system(UiStage, 
            calc_border_radius
                .after(world_matrix::cal_matrix)
                .before(UiSystemSet::LifeDrawObjectFlush),
        )
		;
    }
}

/// 计算圆角
pub fn calc_border_radius( 
    removed: ComponentRemoved<BorderRadius>,
    // query_delete: Query<(Has<BorderRadius>, &'static DrawList)>,
    mut query: Query<
        (&'static BorderRadius, &'static LayoutResult, &mut SdfUv, &mut SdfSlice, &WorldMatrix),
        Or<(Changed<BorderRadius>, Changed<WorldMatrix>)>,
    >,

    query_root: Query<&RootScale>,

    global_mark: SingleRes<GlobalDirtyMark>,
    font_sheet: SingleResMut<ShareFontSheet>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
    // let mut map = await_list.1.lock().unwrap();
    // if map.get(&hash).is_none() {
    //     map.insert(hash, 0);
    //     await_set_gylph.push(entity);
    //     log::debug!("add_shape!! hash: {}", hash);
    //     match node_state.shape.clone() {
    //         Shape::Rect { x, y, width, height } => sdf2_table.add_shape(hash, pi_sdf::shape::Rect::new(x, y, width, height).get_svg_info()),

	// let instances = instances.bypass_change_detection();
    if removed.len() > 0 {
        todo!()
        // for i in removed.iter() {
        //     if let Ok((has_border_radius, render_list)) = query_delete.get(*i) {
        //         // border_radius不存在时，删除对应DrawObject的uniform
        //         if has_border_radius {
        //             continue;
        //         };
        
        //         for i in render_list.iter() {
        //             if let Ok(instance_index) = query_draw.get_mut(i.id) {
        //                 let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
        //                 let mut render_flag = instance_data.get_render_ty();
        //                 render_flag &= !(1 << RenderFlagType::ClipRectRadius as usize);
        //                 instance_data.set_data(&TyMeterial(&[render_flag as f32]));
        //             }
        //         }
        //     }
        // }
    }
    
    

	if global_mark.mark.has_any(&*BORDER_RADIUS_DIRTY) {
        let mut font_sheet = font_sheet.borrow_mut();
        let sdf2_table = &mut font_sheet.font_mgr_mut().table.sdf2_table;

        for (border_radius, layout, mut sdf_uv, mut sdf_slice, world_matrix) in query.iter_mut() {
            let scale = world_matrix.column(0).x.min(world_matrix.column(1).y);
            let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top; 
            let rd = cal_border_radius(border_radius, &layout.rect);
            let sdf_info = boder_sdf_info(&rd, scale);
            let hash = calc_hash(&sdf_info, 0);

            let sdf_glyph = match sdf2_table.shapes_tex_info.get(&hash) {
                Some(r) => r,
                None => {
                    let (mut verb, mut points) = (vec![], vec![]);
                    gen_sdf(&sdf_info, &mut points, &mut verb);
                    let svg_info = Path::new1(verb, points).get_svg_info();
                    // let point = [
                    //     100.0, 100.0, f32::INFINITY,
                    //     102.0, 100.0, 0.0,
                    //     117.0, 85.0, -std::f32::consts::FRAC_PI_8.tan(),
                    //     117.0, 83.0, 0.0,
                    //     102.0, 68.0, -std::f32::consts::FRAC_PI_8.tan(),
                    //     100.0, 68.0, 0.0,
                    //     85.0,  83.0, -std::f32::consts::FRAC_PI_8.tan(),
                    //     85.,   85., 0.0,
                    //     100., 100., -std::f32::consts::FRAC_PI_8.tan(),
                    // ];
                    // let binding_box = [85.0, 68.0, 117.0, 100.0];
                    // let svg_info = SvgInfo::new(&binding_box, point.to_vec(), true, None);

                    log::debug!("radius sdf_info====={:?}", sdf_info);
                    sdf2_table.add_shape(hash, svg_info, sdf_info.width as usize, 1,  2);
                    sdf2_table.shapes_tex_info.get(&hash).unwrap()
                },
            };
            
            *sdf_uv = SdfUv (Rect {
                left:   sdf_glyph.x as f32,
                right:  sdf_glyph.x as f32 + sdf_glyph.width as f32,
                top:    sdf_glyph.y as f32,
                bottom: sdf_glyph.y as f32 + sdf_glyph.height as f32,
            }, 2.0 / sdf_info.scale / (scale + 0.0001));

            log::debug!("radius sdf_uv====={:?}, {:?}", &*sdf_uv, sdf_glyph);

            *sdf_slice = SdfSlice {
                sdf_slice: Rect {
                    left: sdf_info.sdf_radius.x[0].max(sdf_info.sdf_radius.x[3]) / sdf_info.width,
                    right: (sdf_info.width - sdf_info.sdf_radius.x[1].max(sdf_info.sdf_radius.x[2])) / sdf_info.width,
                    top: sdf_info.sdf_radius.y[0].max(sdf_info.sdf_radius.y[1]) / sdf_info.height,
                    bottom: (sdf_info.height - sdf_info.sdf_radius.y[2].max(sdf_info.sdf_radius.y[3])) / sdf_info.height,
                },
                layout_slice: Rect {
                    left: rd.x[0].max(rd.x[3]) / width,
                    right: (width - rd.x[1].max(rd.x[2])) / width,
                    top: rd.y[0].max(rd.y[1]) / height,
                    bottom: (height - rd.y[2].max(rd.y[3])) / height,
                },
            };
            log::debug!("sdf_slice============{:?}",(&rd, sdf_info.scale, world_matrix.column(0).x, world_matrix.column(1).y, width, height, &sdf_info,  &*sdf_slice, &*sdf_uv, sdf_glyph));
        }
    }
    
}


#[derive(Debug)]
pub struct BorderSdfInfo {
    pub width: f32,
    pub height: f32,
    pub sdf_radius: BorderRadiusPixel,
    pub scale: f32,
}

impl Hash for BorderSdfInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.width as usize).hash(state);
        (self.height as usize).hash(state);
        ((self.sdf_radius.x[0] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.x[1] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.x[2] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.x[3] * 1000.0) as usize).hash(state); 
        ((self.sdf_radius.y[0] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.y[1] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.y[2] * 1000.0) as usize).hash(state);
        ((self.sdf_radius.y[3] * 1000.0) as usize).hash(state);
    }
}

pub fn gen_sdf(sdf_info: &BorderSdfInfo, points: &mut Vec<f32>, verb: &mut Vec<PathVerb>) {
    let width = sdf_info.width;
    let height = sdf_info.height;
    let rd = &sdf_info.sdf_radius;
    let mut x;
    let mut y = height - rd.y[3] ;
    verb.push(PathVerb::MoveTo);
    points.extend_from_slice(&[0.0, y]);

    // // 左边直线
    // if !eq_f32(y, rd.y[0]) {
    //     verb.push(PathVerb::LineTo);
    //     points.extend_from_slice(&[0.0, rd.y[0]]);
    // }
    
    // 左上圆角
    if rd.x[0] != 0.0 && rd.y[0] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[0], rd.y[0], 0.0, 0.0, rd.x[0], height]);
    }

    // 上边直线
    x = width - rd.x[1];
    if !eq_f32(rd.x[0], x) {
        verb.push(PathVerb::LineTo);
        points.extend_from_slice(&[x, height]);
    }
    
    // 右上圆角
    if rd.x[1] != 0.0 && rd.y[1] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[1], rd.y[1], 0.0, 0.0, width, height - rd.y[1]]);
    }

    // 右边直线
    if !eq_f32(y, rd.y[2]) {
        verb.push(PathVerb::LineTo);
        points.extend_from_slice(&[width, rd.y[2]]);
    }

    // 右下圆角
    x = width - rd.x[2];
    if rd.x[2] != 0.0 && rd.y[2] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[2], rd.y[2], 0.0, 0.0, x, 0.0]);
    } 
   
    // 下边直线
    if !eq_f32(x, rd.x[3]) {
        verb.push(PathVerb::LineTo);
        points.extend_from_slice(&[rd.x[3], 0.0]);
    }

    // 左下圆角
    y = rd.y[3];
    if rd.x[3] != 0.0 && rd.y[3] != 0.0 {
        // 椭圆弧
        verb.push(PathVerb::EllipticalArcTo);
        points.extend_from_slice(&[rd.x[3], rd.y[3], 0.0, 0.0, 0.0, y]);
    }

	verb.push(PathVerb::Close);                
}

// 由于当前像素范围设置为了最高精度，因为， 在显示时，
fn boder_sdf_info(rd: &BorderRadiusPixel, scale: f32) -> BorderSdfInfo {
    // let min_radius = rd.x[0].min(rd.x[1]).min(rd.y[0]).min(rd.y[1]).min(rd.x[2]).min(rd.y[2]).min(rd.x[3]).min(rd.y[3]) * scale;
    // let max_radius = rd.x[0].max(rd.x[1]).max(rd.y[0]).max(rd.y[1]).max(rd.x[2]).max(rd.y[2]).max(rd.x[3]).max(rd.y[3]) * scale;
    
    // 最小边
    let max_size = (rd.x[0] + rd.x[1]).max(rd.x[2] + rd.x[3]).max(rd.y[0] + rd.y[3]).max(rd.y[1] + rd.y[2]) * scale;

    let size = radius_edge_size(max_size);
    // let size = 32.0 * level;

    // let radius_size = 30.0 * level;

    let radius_scale = size / max_size; // 缩放比例
    let scale1 = radius_scale * scale;
    // println!("boder_sdf_info: {:?}", (max_size, size));

    BorderSdfInfo {
        width: size + 2.0,
        height: size + 2.0,
        sdf_radius: BorderRadiusPixel {
            x: [rd.x[0] * scale1, rd.x[1] * scale1, rd.x[2] * scale1, rd.x[3] * scale1],
            y: [rd.y[0] * scale1, rd.y[1] * scale1, rd.y[2] * scale1, rd.y[3] * scale1],
        },
        scale: radius_scale,
    }
}

pub fn radius_edge_size(max_size: f32) -> f32 {
    let max_size = max_size.ceil() as usize;
    if max_size < 32 && max_size != 0 {
        // println!("max_size: {:?}", (max_size, std::mem::size_of::<usize>(), max_size.leading_zeros()));
        let zero_count = (std::mem::size_of::<usize>() * 8) - max_size.leading_zeros() as usize;
        let odd_count = zero_count >> 1 << 1;
        (1 << odd_count ).max(2) as f32
    } else {
        (max_size as f32 / 32.0).ceil() * 32.0
    }
    // let max_size = if !eq_f32(max_radius, 0.0) {
    //     let max_scale = 2.0 / min_radius; // 最大缩放比例， 2 是一个经验值， 使缩放后， 圆角的最小半径不低于2px

    //     max_size * max_scale
    // } else {
    //     max_size
    // };
    
    
    // 30 是一个经验值， 使得在32边长尺寸下， 每边圆角半径和小于30， 圆角间隔（中间直线）为2， 总和为32
    // 在大于32的情况下， 圆角和直线间隔都按比例放大
    // (max_size / 30.0).ceil()
}



lazy_static! {
	pub static ref BORDER_RADIUS_DIRTY: StyleMarkType = style_bit() | &*LAYOUT_DIRTY
		.set_bit(StyleType::BorderRadius as usize)
        .set_bit(OtherDirtyType::NodeTreeAdd as usize)
        .set_bit(OtherDirtyType::NodeTreeDel as usize)
		.set_bit(OtherDirtyType::DrawObjCreate as usize);
}


