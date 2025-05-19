

use std::ops::Range;

use pi_flex_layout::prelude::Rect;
use pi_null::Null;
use pi_polygon::{find_lg_endp, interp_mult_by_lg, split_by_lg, Attribute, LgCfg, PolygonIndices};
use pi_style::style::LinearGradientColor;

use crate::{components::draw_obj::{PolygonType, TempGeo, TempGeoBuffer, VColor}, resource::draw_obj::InstanceContext, shader::ui_meterial::ColorUniform, shader1::batch_meterial::{LayoutUniform, LinearGradientColorUniform, LinearGradientPointUniform, LinearGradientSdfUvUniform, RenderFlagType, SdfUniform, SdfUvUniform, StrokeColorUniform, TyMeterial, UvUniform}, utils::tools::eq_f32};

impl TempGeo {
   
    pub fn absolute_slice(slice: &Rect<f32>, target: &Rect<f32>) -> Rect<f32> {
        let split_target_width = target.right - target.left;
        let split_target_height = target.bottom - target.top;

        let r = Rect {
            top: target.top + slice.top * split_target_height,
            right: target.left + slice.right * split_target_width,
            bottom: target.top + slice.bottom * split_target_height,
            left: target.left + slice.left * split_target_width
        };
        log::debug!("absolute_slice: {:?}", (slice, target, &r));
        r
    }

    // 九宫格切分(将div切割为九个块)
    // 按照上左， 下左， 下右， 上右， 上中，右中, 下中， 左中， 中的顺序， 返回九个四边形，
    // 注意， 这里四边形用aabb来表示， 而非四个点
    pub fn grid_aabbs(target_slice: &Rect<f32>, target: &Rect<f32>, buffer: &mut Vec<f32>){
        buffer.extend_from_slice(&[
            // 四角
            target.left, target.top, target_slice.left, target_slice.top,
            target.left, target_slice.bottom, target_slice.left, target.bottom,
            target_slice.right, target_slice.bottom, target.right, target.bottom,
            target_slice.right, target.top, target.right, target_slice.top,

            // 四边
            target_slice.left, target.top, target_slice.right, target_slice.top,
            target_slice.right, target_slice.top, target.right, target_slice.bottom,
            target_slice.left, target_slice.bottom, target_slice.right, target.bottom,
            target.left, target_slice.top, target_slice.left, target_slice.bottom,

            // 中间
            target_slice.left, target_slice.top, target_slice.right, target_slice.bottom
        ]);
        log::debug!("grid_aabbs: {:?}", (target_slice, target, &buffer[buffer.len() - 9 * 4..buffer.len()]));
    }

    // 九宫格切分(将div切割为九个块) 
    // 按照上左， 下左， 下右， 上右， 上中，右中, 下中， 左中， 中的顺序， 返回九个四边形，
    // 注意， 与grid_aabbs不同，这里表示各顶点， 需要与索引搭配使用
    pub fn grid_point(target_slice: &Rect<f32>, target: &Rect<f32>, buffer: &mut Vec<f32>) {
        buffer.extend_from_slice(&[
            // 外层
            target.left, target.top,
            target.left, target.bottom,
            target.right, target.bottom,
            target.right, target.top,

            // 内层
            target_slice.left, target_slice.top,
            target_slice.left, target_slice.bottom,
            target_slice.right, target_slice.bottom,
            target_slice.right, target_slice.top,

            // 交点
            target_slice.right, target.top,
            target_slice.left,  target.top,
            target.left,        target_slice.top, 
            target.left,        target_slice.bottom, 
            target_slice.left,  target.bottom,
            target_slice.right, target.bottom,
            target.right,       target_slice.bottom, 
            target.right,       target_slice.top, 
        ]);
    }

    // 0|-----|9---------- 8|------|3
    //10|-----|4---------- 7|------|15
    //11|-----|5---------- 6|------|14
    // 1|-----|12----------13|-----|2
    pub fn grid_index(start: u16) -> PolygonType {
        PolygonType::NoRule(
            PolygonIndices {
                indices: vec![
                    0 + start, 10 + start, 4 + start, 9 + start,
                    9 + start, 4 + start, 7 + start, 8 + start,
                    8 + start, 7 + start, 15 + start, 3 + start,
                    10 + start, 11 + start, 5 + start, 4 + start,
                    4 + start, 5 + start, 6 + start, 7 + start,
                    7 + start, 6 + start, 14 + start, 15 + start,
                    11 + start, 1 + start, 12 + start, 5 + start,
                    5 + start, 12 + start, 13 + start, 6 + start,
                    6 + start, 13 + start, 2 + start, 14 + start,
                ],
                counts: vec![4;9],
            }

        )

    }

    // 线切分
    pub fn line_split(repeat_info: &RepeatInfo, split: f32, default_value: usize) -> usize {
        let (mut start, mut end) = (repeat_info.start, repeat_info.end);
        if split < start || eq_f32(split, start) || split > end || eq_f32(split, end) {
            return default_value;
        }

        let step = repeat_info.space + repeat_info.item_size;


        let mut split_index = 0;
        if repeat_info.bound_step > 0.0 {
            // 处理开头和结尾
            start = start + repeat_info.bound_step;  
            end = end - repeat_info.bound_step;
            if eq_f32(split, start) && eq_f32(split, end) {
                // 正好在线上， 不需要劈分
                return default_value;
            }

            if split < start  {
                return 0;
            }
 
            start += repeat_info.space;

            if split > end {
                let int_count = ((( end - start ) / step) + 0.001).floor().max(0.0) as usize; // 整数倍重复多少次
                return int_count + 2;
            }

            split_index += 1;
        }

        // 处理中间重复部分
        let remain = (split - start) % step;
        if remain > 0.0 && remain < repeat_info.item_size && !eq_f32(remain, 0.0) && !eq_f32(remain, repeat_info.item_size){
            // 求余部分小于item_size， 并且不为0，也不为item_size， 说明不在min和max线上， 返回对应的值
            return split_index + ((split - start - 0.001) / step).ceil() as usize;
        }
        
        // 不满足条件， split会与某条线重合
        return default_value;
    }

    // 
    pub fn grid_split(
        repeat_info: &RepeatInfo,
        buffer: &mut GridBufer,
        desc: &DirectionDesc,
        uv: Range<f32>,
    ) -> Range<usize> {
        let min_position = Self::line_split(repeat_info, desc.split.start, 0);
        let max_position = Self::line_split(repeat_info, desc.split.end, <usize as Null>::null());
        let uv_size = uv.end - uv.start;
        let sdf_uv_size = desc.sdf_uv.end - desc.sdf_uv.start;
        let splt_end_size = desc.layout_range.end - desc.split.end;

        let position_start = buffer.positions.len();
        
        let mut start = repeat_info.start;
        let mut next = (repeat_info.start + if repeat_info.bound_step > 0.0 {repeat_info.bound_step} else {repeat_info.item_size}).min(repeat_info.end);
        let mut start_uv = if repeat_info.bound_step > 0.0 {uv.start + (1.0 - repeat_info.bound_step / repeat_info.item_size) * uv_size } else {uv.start};

        let mut i = 0;
        while start < repeat_info.end {
            buffer.positions.push(start);
            buffer.uvs.push(start_uv);
            
            // sdf_uv进行插值
            if start < desc.split.start && !eq_f32(start, desc.split.start) {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (start / desc.split.start) * desc.sdf_slice.start * sdf_uv_size);
            } else if start > desc.split.end && !eq_f32(start, desc.split.end) {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (desc.sdf_slice.end + (start - desc.split.end) / splt_end_size * (1.0 - desc.sdf_slice.end)) * sdf_uv_size);
            } else {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (desc.sdf_slice.start + (start / (desc.split.end - desc.split.start)) * (desc.sdf_slice.end - desc.sdf_slice.start)) * sdf_uv_size);
            }

            let next_i = i + 1;
            if next_i ==  min_position {
                buffer.positions.push(desc.split.start);
                buffer.positions.push(desc.split.start);
                let uv = start_uv + ((desc.split.start - start) / repeat_info.item_size * uv_size);
                buffer.uvs.push(uv);
                buffer.uvs.push(uv);

                let sdf_uv = desc.sdf_uv.start + desc.sdf_slice.start * sdf_uv_size;
                buffer.sdf_uvs.push(sdf_uv);
                buffer.sdf_uvs.push(sdf_uv);
            } 
            if next_i ==  max_position {
                buffer.positions.push(desc.split.end);
                buffer.positions.push(desc.split.end);
                let uv = start_uv + ((desc.split.end - start) / repeat_info.item_size * uv_size);
                buffer.uvs.push(uv);
                buffer.uvs.push(uv);

                let sdf_uv = desc.sdf_uv.start + desc.sdf_slice.end * sdf_uv_size;
                buffer.sdf_uvs.push(sdf_uv);
                buffer.sdf_uvs.push(sdf_uv);
            }

            
            buffer.positions.push(next);
            buffer.uvs.push(start_uv + ((next - start) / repeat_info.item_size * uv_size));
            // sdf_uv进行插值
            if next < desc.split.start && !eq_f32(next, desc.split.start) {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (next / desc.split.start) * desc.sdf_slice.start * sdf_uv_size);
            } else if next > desc.split.end && !eq_f32(next, desc.split.end) {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (desc.sdf_slice.end + (next - desc.split.end) / splt_end_size * (1.0 - desc.sdf_slice.end)) * sdf_uv_size);
            } else {
                buffer.sdf_uvs.push(desc.sdf_uv.start + (desc.sdf_slice.start + (next / (desc.split.end - desc.split.start)) * (desc.sdf_slice.end - desc.sdf_slice.start)) * sdf_uv_size);
            }

            start = next + repeat_info.space;
            next = repeat_info.end.min(start + repeat_info.item_size);
            start_uv = uv.start;

            i += 1;
        }

        let len = buffer.positions.len();
        // log::warn!("grid_split======={:?}", (
        //     &repeat_info,
        //     &positions, 
        //     &uvs, 
        //     & sdf_uvs, 
        //     &uv, 
        //     &sdf_uv,
        //     &sdf_slice,
        //     &layout_range,
    
        //     &desc.split.start, 
        //     &desc.split.end,
        //     &positions[position_start..len],
        //     position_start..len
        // ));

        // 不满足条件， split会与某条线重合
        return position_start..len;
    }

    pub fn rect_to_quad(rect: &Rect<f32>, buffer: &mut Vec<f32>) {
        buffer.extend_from_slice(&[
            rect.left,
            rect.top,
            rect.left,
            rect.bottom,
            rect.right,
            rect.bottom,
            rect.right,
            rect.top,
        ]);
    }

    pub fn rect_to_rect_geo(rect: &Rect<f32>) -> Vec<f32> {
        vec![
            rect.left,
            rect.top,
            rect.right,
            rect.bottom,
        ]
    }

    pub fn rect_position(target: &Rect<f32>) -> Vec<f32> {
        vec![
            target.left, target.top,
            target.left, target.top,
        ]
    }

     
    // 生成渐变颜色实例数据
    pub fn linear_gradient_split(&mut self, color: &LinearGradientColor, lg_rect: &Rect<f32>, buffer: &mut TempGeoBuffer) {
        let geo = self;
        let endp = find_lg_endp(
            &[
                lg_rect.left,
                lg_rect.top,
                lg_rect.left,
                lg_rect.bottom,
                lg_rect.right,
                lg_rect.bottom,
                lg_rect.right,
                lg_rect.top,//渐变端点
            ],
            color.direction,
        );
        

        let mut lg_pos = Vec::with_capacity(color.list.len());
        let mut lg_color = Vec::with_capacity(color.list.len() * 4);
        for v in color.list.iter() {
            lg_pos.push(v.position);
            lg_color.extend_from_slice(&[v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w]);
        }
        let lg_color = vec![LgCfg {
            unit: 4,
            data: lg_color,
        }];

        let mut data = LinearData {
            endp, lg_pos, lg_color, polygons: Default::default(), attrs: Vec::default(), positions: &mut buffer.positions,
        };
        data.attrs.push(Attribute {
            unit: 2,
            value: &mut buffer.sdf_uvs,
        });
        if buffer.uvs.len() > 0 {
            data.attrs.push(Attribute {
                unit: 2,
                value: &mut buffer.uvs,
            });
        }

        match &geo.polygons {
            PolygonType::NoRule(polygons) => {
                let mut index_start = 0;
                for j in polygons.counts.iter() {
                    linear_gradient_split_inner(&mut data, &mut buffer.colors, &polygons.indices[index_start..index_start + *j]);
                    index_start += *j;
                }
            },
            PolygonType::Rule(r, range) => {
                let end = range.end / 2;
                let mut i = range.start / 2;

                let mut templ_indices = Vec::with_capacity(*r);
                unsafe {templ_indices.set_len(*r)};
                while (i as usize) < end {
                    // 设置多边形索引
                    for j in 0..*r {
                        templ_indices[j] = (i + j) as u16;
                    }
                    linear_gradient_split_inner(&mut data, &mut buffer.colors,&mut templ_indices);
                    i = i + *r;
                }
            },
            PolygonType::Rect(range) => {
                let mut i = range.start;

                let mut templ_indices = Vec::with_capacity(4);
                while (i as usize) < range.end {
                    let s = [data.positions[i], data.positions[i + 3], data.positions[i + 2], data.positions[i + 1]];
                    let sdf = [data.attrs[0].value[i], data.attrs[0].value[i + 3], data.attrs[0].value[i + 2], data.attrs[0].value[i + 1]];
                    data.positions.extend_from_slice(s.as_slice());
                    data.attrs[0].value.extend_from_slice(sdf.as_slice());
                    // 设置多边形索引
                    let ii = i / 2;
                    let len = data.positions.len() / 2;
                    templ_indices.extend_from_slice(&[
                        ii as u16, (len - 2) as u16,  (ii + 1) as u16, (len - 1) as u16,
                    ]);
                    log::debug!("data.positions========={:?}", data.positions.len());
                    linear_gradient_split_inner(&mut data, &mut buffer.colors,&mut templ_indices);
                    templ_indices.clear();
                    i = i + 4;
                }
            }
            _ => todo!(), // 不对已经处理成三角形的索引进行处理, 也不对矩形进行处理(不支持)
        }

       
        geo.colors = VColor::Linear;
        geo.polygons = PolygonType::NoRule(data.polygons);
        log::debug!("linner======{:?}", (&geo.polygons, &buffer.positions, &geo.colors, lg_rect, color));
    }

    pub fn set_instance_data(&self, mut instance_start: usize, instances: &mut InstanceContext, other_info: Option<&OtherInfo>, buffer: &TempGeoBuffer) -> usize {
		match &self.polygons {
            PolygonType::Rect(range) => {
                let mut i = range.start;
                let end = range.end;
                
                while i < end {
                    let pindex0 = i;
                    let pindex1 = i + 2;
                    let mut instance_data = instances.instance_data.instance_data_mut(instance_start);
                    let mut render_flag = instance_data.get_render_ty();
                    render_flag &= !(1 << RenderFlagType::LinearGradient as usize);
                    instance_data.set_data(&TyMeterial(&[render_flag as f32]));
        
                    let p = [
                        buffer.positions[pindex0], buffer.positions[pindex0 + 1],
                        buffer.positions[pindex1] - buffer.positions[pindex0], buffer.positions[pindex1 + 1] - buffer.positions[pindex0 + 1],
                    ];
                    let sdf_uv = [
                        buffer.sdf_uvs[pindex0], buffer.sdf_uvs[pindex0 + 1],
                        buffer.sdf_uvs[pindex1], buffer.sdf_uvs[pindex1 + 1],
                    ];
                    let colors = match &self.colors {
                        VColor::CgColor(c) => c, 
                        VColor::Linear => todo!(), // 四边形的情况， 一定是纯色
                    };
                    // log::warn!("geo_split================{:?}", (instance_start, p));
                    instance_data.set_data(&LinearGradientPointUniform(&p));
                    instance_data.set_data(&ColorUniform(&[colors.x, colors.y, colors.z, colors.w]));
                    instance_data.set_data(&SdfUvUniform(&sdf_uv));
                   
                    if let Some(other_info) = other_info {
                        instance_data.set_data(&SdfUniform(&other_info.sdf_info));
                        instance_data.set_data(&StrokeColorUniform(&other_info.stroke_color));
                        instance_data.set_data(&TyMeterial(&[other_info.ty]));
                        log::debug!("set_instance_data rect other============{:?}", other_info );
                    }

                    

                    if buffer.uvs.len() > 0 {
                        let uvs = [
                            buffer.uvs[pindex0], buffer.uvs[pindex0 + 1],
                            buffer.uvs[pindex1], buffer.uvs[pindex1 + 1],
                        ];
                        instance_data.set_data(&UvUniform(&uvs));
                    } 
                    
                    log::debug!("geo color======={:?}", (instance_start, p, colors));
                    i += 4;
                    instance_start += instances.instance_data.alignment;
                }
            },
            
            PolygonType::Triangle(indices) => {
                let mut i = 0;
                while i < indices.len() {
                    let pindex0 = (indices[i] * 2) as usize;
                    let cindex0 = (indices[i] * 4) as usize;
                    let pindex1 = (indices[i + 1] * 2) as usize;
                    let cindex1 = (indices[i + 1] * 4) as usize;
                    let pindex2 = (indices[i + 2] * 2) as usize;
                    let cindex2 = (indices[i + 2] * 4) as usize;
                    let mut instance_data = instances.instance_data.instance_data_mut(instance_start);
                    let mut render_flag = instance_data.get_render_ty();
                    render_flag |= 1 << RenderFlagType::LinearGradient as usize;
                    render_flag |= 1 << RenderFlagType::Stroke as usize;
                    instance_data.set_data(&TyMeterial(&[render_flag as f32]));
        
                    let p = [
                        buffer.positions[pindex0], buffer.positions[pindex0 + 1],
                        buffer.positions[pindex1], buffer.positions[pindex1 + 1],
                        buffer.positions[pindex2], buffer.positions[pindex2 + 1],
                    ];
                    let sdf_uv = [
                        buffer.sdf_uvs[pindex0], buffer.sdf_uvs[pindex0 + 1],
                        buffer.sdf_uvs[pindex1], buffer.sdf_uvs[pindex1 + 1],
                        buffer.sdf_uvs[pindex2], buffer.sdf_uvs[pindex2 + 1],
                    ];

                    let c = [
                        buffer.colors[cindex0], buffer.colors[cindex0 + 1], buffer.colors[cindex0 + 2], buffer.colors[cindex0 + 3],
                        buffer.colors[cindex1], buffer.colors[cindex1 + 1], buffer.colors[cindex1 + 2], buffer.colors[cindex1 + 3],
                        buffer.colors[cindex2], buffer.colors[cindex2 + 1], buffer.colors[cindex2 + 2], buffer.colors[cindex2 + 3],
                    ];
                    instance_data.set_data(&LinearGradientPointUniform(&p));
                    instance_data.set_data(&LinearGradientColorUniform(&c));
                    instance_data.set_data(&LinearGradientSdfUvUniform(&sdf_uv));
                    if let Some(other_info) = other_info {
                        instance_data.set_data(&SdfUniform(&other_info.sdf_info));
                        instance_data.set_data(&StrokeColorUniform(&other_info.stroke_color));
                    }

                    // 当为渐变颜色时， 不会存在uv
                    // if self.uvs.len() > 0 {
                    //     let uvs = [
                    //         self.uvs[pindex0], self.uvs[pindex0 + 1],
                    //         self.uvs[pindex1], self.uvs[pindex1 + 1],
                    //         self.uvs[pindex2], self.uvs[pindex2 + 1],
                    //     ];
                    //     instance_data.set_data(&UvUniform(&uvs));
                    // }
                    
                    log::debug!("geo linear color======={:?}", (instance_start, p, c));
                    i += 3;
                    instance_start += instances.instance_data.alignment;
                }
            },
            _ => todo!(),
        }	

        instance_start		
        
    }
}

#[derive(Debug)]
pub struct OtherInfo {
    pub sdf_info: [f32; 3],
    pub stroke_color: [f32; 4],
    pub ty: f32,
}

struct LinearData<'a> {
    endp: ((f32, f32), (f32, f32)),
    lg_pos: Vec<f32>,
    lg_color: Vec<LgCfg>,
    polygons: PolygonIndices,
    attrs: Vec<Attribute<'a>>,
    positions: &'a mut Vec<f32>,
}

fn linear_gradient_split_inner(data: &mut LinearData, colors: &mut Vec<f32>, templ_indices: &[u16]) {
    let polygons = split_by_lg(
        &mut data.polygons,
        &mut data.positions,
        &mut data.attrs,
        &templ_indices,
        data.lg_pos.as_slice(),
        data.endp.0.clone(),
        data.endp.1.clone(),
    );

    interp_mult_by_lg(
        data.positions,
        &data.polygons,
        polygons,
        &mut [colors],
        data.lg_color.clone(),
        data.lg_pos.as_slice(),
        data.endp.0.clone(),
        data.endp.1.clone(),
    );
}

#[test]
pub fn grid_split() {
    let info = RepeatInfo {
        start: 0.0,
        end: 60.0,
        bound_step: 5.0,
        space: 10.0,
        item_size: 10.0,
    };
    let mut split = 0.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 4.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 11.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 21.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 31.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 41.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 51.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 56.0;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

    split = 60.1;
    println!("{:?}, {:?}", (&info, split), TempGeo::line_split(&info, split, <usize as Null>::null()));

}

#[derive(Debug)]
pub struct RepeatInfo {
    pub start: f32, 
    pub end: f32, 
    pub bound_step: f32, 
    pub space: f32, 
    pub item_size: f32,
}

#[derive(Debug, Default)]
pub struct GridBufer {
    pub uvs: Vec<f32>,
    pub positions: Vec<f32>,
    pub sdf_uvs: Vec<f32>,
    // pub objs: Vec<(Entity, BufferRange)>,
}

#[derive(Debug)]
pub enum BufferRange {
    Mult( Vec<(Range<usize>, Range<usize>)> ),
    Single( Range<usize>, Range<usize> )
}

pub fn set_grid_instance(buffer: &GridBufer, x_range: Range<usize>, y_range: Range<usize>, mut instance_start: usize, instances: &mut InstanceContext) -> usize {
    
    // log::warn!("set_grid_instance1==={:?}", (&x_range, &y_range, ((x_range.end - x_range.start) / 2) * ((y_range.end - y_range.start) / 2)));
    
    let mut j = y_range.start;
    while j < y_range.end {
        let mut i = x_range.start;
        while i < x_range.end {
            let mut instance_data = instances.instance_data.instance_data_mut(instance_start);
            let positions = [
                buffer.positions[i], 
                buffer.positions[j],
                buffer.positions[i + 1] - buffer.positions[i],
                buffer.positions[j + 1] - buffer.positions[j],
            ];
            let sdf_uvs = [
                buffer.sdf_uvs[i], 
                buffer.sdf_uvs[j],
                buffer.sdf_uvs[i + 1],
                buffer.sdf_uvs[j + 1],
            ];
            instance_data.set_data(&LayoutUniform(&positions));
            if buffer.uvs.len() > 0 {
                let uvs = [
                    buffer.uvs[i], 
                    buffer.uvs[j],
                    buffer.uvs[i + 1],
                    buffer.uvs[j + 1],
                ];
                instance_data.set_data(&UvUniform(&uvs));
            }    
           
            instance_data.set_data(&SdfUvUniform(&sdf_uvs));

            log::debug!("set_grid_instance==={:?}", (instance_start, &positions, sdf_uvs));
            
            instance_start += instances.instance_data.alignment;
            i += 2;
        }
        j += 2;
    }

    instance_start
}

// 某一方向的信息
#[derive(Debug)]
pub struct DirectionDesc { 
    pub sdf_uv: Range<f32>,
    pub sdf_slice: Range<f32>,
    pub layout_range: Range<f32>,
    pub split: Range<f32>,
}

// grid简单切分
pub fn grid_split_simple(grid_buffer: &mut GridBufer, rect: &Rect<f32>, sdf_uv0: &Rect<f32>, sdf_slice: &Rect<f32>, layout_slice: &Rect<f32>)-> (usize, [(Range<usize>, Range<usize>); 9]) {
    let start = grid_buffer.positions.len();
    grid_buffer.positions.extend_from_slice(&[
        rect.left, layout_slice.left, layout_slice.right, rect.right, // 纬线
        rect.top, layout_slice.top, layout_slice.bottom, rect.bottom, // 经线
    ]);
    grid_buffer.sdf_uvs.extend_from_slice(&[
        sdf_uv0.left, sdf_slice.left, sdf_slice.right, sdf_uv0.right, // 纬线
        sdf_uv0.top, sdf_slice.top, sdf_slice.bottom, sdf_uv0.bottom, // 经线
    ]);
    let fill_x_size = layout_slice.right - layout_slice.left;
    let fill_y_size = layout_slice.bottom - layout_slice.top;

    let (left_size, right_size, top_size, bottom_size) = (
        layout_slice.left - rect.left,
        rect.right - layout_slice.right,
        layout_slice.top - rect.top,
        rect.bottom - layout_slice.bottom,
    ); 
    let left = if left_size > 0.0 && !eq_f32(left_size, 0.0) {
        start + 0..start +2
    } else {
        0..0
    };

    let fill_latitude = if fill_x_size > 0.0 && !eq_f32(fill_x_size, 0.0) {
        start + 1..start + 3
    } else {
        0..0
    };

    let right = if right_size > 0.0 && !eq_f32(right_size, 0.0) {
        start + 2..start + 4
    } else {
        0..0
    };
    
    let top = if top_size > 0.0 && !eq_f32(top_size, 0.0) {
        start + 4..start + 6
    } else {
        0..0
    };

    let fill_meridian = if fill_y_size > 0.0 && !eq_f32(fill_y_size, 0.0) {
        start + 5..start + 7
    } else {
        0..0
    };

    let bottom = if bottom_size > 0.0 && !eq_f32(bottom_size, 0.0) {
        start + 6..start + 8
    } else {
        0..0
    };


    let mut count = left.len() * (top.len() + bottom.len() + fill_meridian.len());
    count += right.len() * (top.len() + bottom.len() + fill_meridian.len());
    count += fill_latitude.len() * (top.len() + bottom.len() + fill_meridian.len());
    count = count / 4;
    log::debug!("border-color==============={:?}", count);

    (count,  [
        (left.clone(), top.clone()),
        (right.clone(), top.clone()),
        (right.clone(), bottom.clone()),
        (left.clone(), bottom.clone()),
        (left, fill_meridian.clone()),
        (fill_latitude.clone(), top),
        (right, fill_meridian.clone()),
        (fill_latitude.clone(), bottom),
        (fill_latitude, fill_meridian),
    ])

}

