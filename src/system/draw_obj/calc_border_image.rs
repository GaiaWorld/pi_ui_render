use pi_world::filter::Or;
use pi_world::prelude::{Changed, With, Query, Plugin, IntoSystemConfigs};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_flex_layout::prelude::{Rect, Size};
use pi_style::style::Aabb2;

use crate::components::calc::{BorderImageTexture, DrawList, LayoutResult, WorldMatrix};
use crate::components::draw_obj::{BorderImageMark, InstanceIndex};
use crate::components::user::{BorderImageClip, Point2, BorderImageRepeat, BorderImageSlice};
use crate::resource::draw_obj::InstanceContext;
use crate::resource::BorderImageRenderObjType;
use crate::prelude::UiStage;

use crate::shader1::meterial::{RenderFlagType, TyUniform, UvUniform, BorderImageInfoUniform};
use crate::system::draw_obj::calc_background_image::calc_step;
use crate::system::draw_obj::set_box;
use crate::system::node::transition::transition_2;
use crate::system::system_set::UiSystemSet;
use crate::components::user::BorderImage;

use super::calc_text::IsRun;
use super::{life_drawobj, image_texture_load};

pub struct BorderImagePlugin;

impl Plugin for BorderImagePlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
			.add_system(UiStage, image_texture_load::image_load::<BorderImage, BorderImageTexture>.in_set(UiSystemSet::NextSetting).after(transition_2))
			.add_system(UiStage, 
				life_drawobj::draw_object_life_new::<
					BorderImageTexture,
					BorderImageRenderObjType,
					(BorderImageMark, ),
					{ BORDER_IMAGE_ORDER },
				>
					.in_set(UiSystemSet::LifeDrawObject)
					.after(image_texture_load::image_load::<BorderImage, BorderImageTexture>),
			)
			.add_system(UiStage, 
				calc_border_image
					.after(super::super::node::world_matrix::cal_matrix)
					.in_set(UiSystemSet::PrepareDrawObj)
			);
    }
}

pub const BORDER_IMAGE_ORDER: u8 = 5;

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_border_image(
	mut instances: OrInitSingleResMut<InstanceContext>,
	query: Query<
		(
			&WorldMatrix,
			&LayoutResult,
			&DrawList,
			&BorderImageTexture,
			Option<&BorderImageClip>,
			Option<&BorderImageRepeat>,
			Option<&BorderImageSlice>,
			&BorderImage,
		),
		Or<(Changed<BorderImageTexture>, Changed<BorderImageClip>, Changed<BorderImageRepeat>, Changed<BorderImageSlice>,  Changed<WorldMatrix>)>,
	>,
    mut query_draw: Query<&InstanceIndex, With<BorderImageMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BorderImageRenderObjType>,
) {
	if r.0 {
		return;
	}
	let render_type = ***render_type;
	let image_clip = BorderImageClip::default();
	let image_mod = BorderImageRepeat::default();
	let image_slice = BorderImageSlice::default();

	for (world_matrix, layout, draw_list, border_image_texture_ref, border_image_clip, border_image_repeat, border_image_slice, background_image) in query.iter() {
		let border_image_texture = match &border_image_texture_ref.0 {
			Some(r) => {
				// 图片不一致， 返回
				if *r.key() != background_image.0.str_hash() as u64 {
					continue;
				}
				r
			},
			None => continue, 
		};

		let draw_id = match draw_list.get_one(render_type) {
			Some(r) => r.id,
			None => continue,
		};
		if let Ok(instance_index) = query_draw.get_mut(draw_id) {
			// 节点可能设置为dispaly none， 此时instance_index可能为Null
			if pi_null::Null::is_null(&instance_index.0.start) {
				continue;
			}
			
			let mut instance_data = instances.instance_data.instance_data_mut(instance_index.0.start);
			let mut render_flag = instance_data.get_render_ty();
			// let layout_is_changed = layout.is_changed();
			// if border_image_texture_ref.is_changed() || 
			// 	border_image_clip.as_ref().map(|r| {r.is_changed()}).unwrap_or(false) || 
			// 	border_image_repeat.as_ref().map(|r| {r.is_changed()}).unwrap_or(false) || 
			// 	layout_is_changed || world_matrix.is_changed(){

				render_flag |= 1 << RenderFlagType::Uv as usize;
				
				let border_clip = match &border_image_clip {
					Some(r) => &*r,
					None => &image_clip,
				};
				let border_slice = match &border_image_slice {
					Some(r) => &*r,
					None => &image_slice,
				};
				let border_image_mod = match &border_image_repeat {
					Some(r) => &*r,
					None => &image_mod,
				};

				let border_box = layout.border_box();

				let p1 = Point2::new(border_box[0], border_box[1]);
				let p2 = Point2::new(p1.x + border_box[2], p1.y + border_box[3]);
				let layout_width = border_box[2].max(0.003);
				let layout_height = border_box[3].max(0.003);
				
				let mut clip = Rect {
					left: *border_clip.left,
					right: *border_clip.right,
					top: *border_clip.top,
					bottom: *border_clip.bottom,
				};
				verify_sero_size(&mut clip, 0.001);
				let clip_size = Size{ width: clip.right - clip.left, height: clip.bottom - clip.top };

				let mut slice_uv = Rect {
					left:   (clip.left   + *border_slice.left   * clip_size.width) * border_image_texture.width as f32,
					right:  (clip.right  - *border_slice.right  * clip_size.width) * border_image_texture.width as f32,
					top:    (clip.top    + *border_slice.top    * clip_size.height) * border_image_texture.height as f32,
					bottom: (clip.bottom - *border_slice.bottom * clip_size.height) * border_image_texture.height as f32,
				};
				verify_sero_size(&mut slice_uv, 0.001);
				let slice_size = Size {
					width: (slice_uv.right - slice_uv.left),
					height: (slice_uv.bottom - slice_uv.top),
				};

				let mut border = Rect {
					left:   layout.border.left.max(0.001),
					right:  (layout_width - layout.border.right).max(0.002),
					top:    layout.border.top.max(0.001),
					bottom: (layout_height - layout.border.bottom).max(0.002),
				};
				verify_sero_size(&mut border, 0.001);

				render_flag |= 1 << RenderFlagType::BorderImage as usize;
				let w = p2.x - p1.x - layout.border.left - layout.border.right;
				let h = p2.y - p1.y - layout.border.top - layout.border.bottom;

				
				 // 上右下左，比率
				let factor = (
					border.top / slice_uv.top.max(0.001), 
					(layout_width - border.right).max(0.001) / (clip_size.width * border_image_texture.width as f32 - slice_uv.right).max(0.001), 
					(layout_height - border.bottom).max(0.001) / (clip_size.height * border_image_texture.height as f32 - slice_uv.bottom).max(0.001), 
					border.left / slice_uv.left.max(0.001)
				);
				
				// 上右下左
				let (offset_top, step_top, space_top) = calc_step(w, slice_size.width * factor.0, border_image_mod.x);
				let (offset_right, step_right, space_right ) = calc_step(h, slice_size.height * factor.1, border_image_mod.y);
				let (offset_bottom, step_bottom, space_bottom) = calc_step(w, slice_size.width * factor.2, border_image_mod.x);
				let (offset_left, step_left, space_left) = calc_step(h, slice_size.height * factor.3, border_image_mod.y);

				instance_data.set_data(&UvUniform(&[clip.left, clip.top, clip.left + clip_size.width, clip.top + clip_size.height])); // uv 0~1
				// instance_data.set_data(&BoxUniform(&[p1.x, p1.y, layout_width, layout_height]));
				set_box(&world_matrix, &Aabb2::new(p1, p2), &mut instance_data);
				instance_data.set_data(&BorderImageInfoUniform(&[
					if border_slice.fill {1.0} else { 0.0 },
					border_image_texture.width as f32, border_image_texture.height as f32,
					border.top, layout_width - border.right, layout_height - border.bottom, border.left, // 变宽宽度
					slice_uv.top, slice_uv.right, slice_uv.bottom, slice_uv.left,
					step_top, step_right, step_bottom, step_left,
					space_top, space_right, space_bottom, space_left,
					offset_top, offset_right, offset_bottom, offset_left,
				]));

				log::trace!("border image, fill={:?}, \ntexture_size={:?}, \nborder_width={:?}, \nslice_uv={:?}, \nstep={:?}, \nspace={:?}, \noffset={:?}", 
					if border_slice.fill {1.0} else { 0.0 },  // data5.y 表示中心部分是否需要填充 1.0000

					&[border_image_texture.width as f32, border_image_texture.height as f32], // data5.zw 边框纹理的宽高 684.0000	104.0000

					&[border.top, layout_width - border.right, layout_height - border.bottom, border.left], // data6 border宽度(这里表示上右下左的border的宽度, 单位： 像素)
					&[slice_uv.top, slice_uv.right, slice_uv.bottom, slice_uv.left], // data7 (纹理左上角为原点， 这里表示上右下左的切割线相对原点的位置， 单位： 像素) 0.0000	624.0132	0.0010	59.9868

					&[step_top, step_right, step_bottom, step_left],  // data8 (步长， 这里表示上右下左的中间部分，纹理重复的步长(布局单位)， 即每重复渲染一次纹理， 实际的布局空间占用多少) 564.0264	0.0010	564.0209	0.0010


					&[space_top, space_right, space_bottom, space_left],  // data9 (空白长度， 这里表示上右下左的中间部分，每次纹理重复， 需要间隔多少布局空间)564.0264	0.0010	564.0209	0.0010

					&[offset_top, offset_right, offset_bottom, offset_left,]); // data10 (偏移，空白长度， 这里表示上右下左的中间部分，开始的第一个纹理， 需要偏移多少布局空间) 0.0000	0.0000	0.0000	0.0000
				instance_data.set_data(&TyUniform(&[render_flag as f32]));
				
			// }

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			// let is_add =  border_image_texture_ref.is_changed(); // background_image_texture会提前创建
			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
		}
	}
}


pub fn verify_sero_size(value: &mut Rect<f32>, min_size: f32) {
	value.right = value.left + (value.right - value.left).max(min_size);
	value.bottom = value.top + (value.bottom - value.top).max(min_size);
}
