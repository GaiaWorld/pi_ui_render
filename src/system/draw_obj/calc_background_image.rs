use pi_world::filter::Removed;
use pi_world::param_set::ParamSet;
use pi_world::prelude::{Changed, With, Query, Plugin, OrDefault, IntoSystemConfigs, Has};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_flex_layout::style::Dimension;
use pi_style::style::{Aabb2, ImageRepeatOption, StyleType};

use crate::components::calc::{BackgroundImageTexture, DrawList, LayoutResult, StyleMark, WorldMatrix};
use crate::components::draw_obj::{BackgroundImageMark, InstanceIndex};
use crate::components::user::{BackgroundImageClip, BackgroundImageMod, FitType, Point2, Size, Vector2};
use crate::resource::draw_obj::InstanceContext;
use crate::resource::BackgroundImageRenderObjType;
use crate::prelude::UiStage;

use crate::shader1::meterial::{RenderFlagType, TyUniform, ImageRepeatUniform, UvUniform};
use crate::system::draw_obj::set_box;
use crate::system::node::layout::calc_layout;
use crate::system::system_set::UiSystemSet;
use crate::utils::tools::eq_f32;
use crate::components::user::BackgroundImage;

use super::calc_text::IsRun;
use super::{life_drawobj, image_texture_load};

pub struct BackgroundImagePlugin;

impl Plugin for BackgroundImagePlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
			.add_system(UiStage, image_texture_load::image_load::<BackgroundImage, BackgroundImageTexture>.in_set(UiSystemSet::NextSetting))
			.add_system(UiStage, set_image_default_size.in_set(UiSystemSet::BaseCalc)
				.before(calc_layout)
			)
			.add_system(UiStage, 
				life_drawobj::draw_object_life_new::<
					BackgroundImageTexture,
					BackgroundImageRenderObjType,
					(BackgroundImageMark, ),
					{ BACKGROUND_IMAGE_ORDER },
				>
					.in_set(UiSystemSet::LifeDrawObject)
					.after(image_texture_load::image_load::<BackgroundImage, BackgroundImageTexture>),
			)
			.add_system(UiStage, 
				calc_background_image
					.after(super::super::node::world_matrix::cal_matrix)
					.in_set(UiSystemSet::PrepareDrawObj)
			);
    }
}

pub const BACKGROUND_IMAGE_ORDER: u8 = 5;

/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_background_image(
	mut instances: OrInitSingleResMut<InstanceContext>,
	query: Query<
		(
			&WorldMatrix,
			&LayoutResult,
			&DrawList,
			&BackgroundImageTexture,
			Option<&BackgroundImageClip>,
			
			Option<&BackgroundImageMod>,
			&BackgroundImage,
		),
		(Changed<BackgroundImageTexture>, Changed<BackgroundImageClip>,  Changed<WorldMatrix>),
	>,
    mut query_draw: Query<&InstanceIndex, With<BackgroundImageMark>>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundImageRenderObjType>,
) {
	if r.0 {
		return;
	}
	log::trace!("bg image========================");
	let render_type = ***render_type;
	let image_clip = BackgroundImageClip::default();
	let image_mod = BackgroundImageMod::default();

	for (world_matrix, layout, draw_list, background_image_texture_ref, background_image_clip, background_image_mod, background_image) in query.iter() {
		let background_image_texture = match &background_image_texture_ref.0 {
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
			// if background_image_texture_ref.is_changed() || 
			// 	background_image_clip.as_ref().map(|r| {r.is_changed()}).unwrap_or(false) || 
			// 	background_image_mod.as_ref().map(|r| {r.is_changed()}).unwrap_or(false) || 
			// 	layout_is_changed || world_matrix.is_changed()
			{

				render_flag |= 1 << RenderFlagType::Uv as usize;
				
				let clip = match &background_image_clip {
					Some(r) => &*r,
					None => &image_clip,
				};
				let background_image_mod = match &background_image_mod {
					Some(r) => &*r,
					None => &image_mod,
				};

				let padding_box = layout.padding_box();

				let mut p1 = Point2::new(padding_box[0], padding_box[1]);
				let mut p2 = Point2::new(p1.x + padding_box[2], p1.y + padding_box[3]);
				let layout_width = padding_box[2];
				let layout_height = padding_box[3];
				let texture_size = Vector2::new(
					background_image_texture.width as f32 * (clip.right - clip.left).abs(),
					background_image_texture.height as f32 * (clip.bottom - clip.top).abs(),
				);
				let mut uv1 = Point2::new(*clip.left, *clip.top);
				let mut uv2 = Point2::new(*clip.right, *clip.bottom);

				if background_image_mod.repeat.x == ImageRepeatOption::Stretch && background_image_mod.repeat.y == ImageRepeatOption::Stretch {
					// 在x、y方向上都为拉伸时， object_fit才生效
					match background_image_mod.object_fit { // 如果不是填充，总是居中显示。 如果在范围内，则修改点坐标。如果超出的部分，会进行剪切，剪切会修改uv坐标。
						FitType::None => {
							// 保持原有尺寸比例。同时保持内容原始尺寸大小。 超出部分会被剪切
							if texture_size.x <= layout_width {
								let x = (layout_width - texture_size.x) / 2.0;
								p1.x += x;
								p2.x -= x;
							} else {
								let x = (texture_size.x - layout_width) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
								uv1.x += x;
								uv2.x -= x;
							}
							if texture_size.y <= layout_height {
								let y = (layout_height - texture_size.y) / 2.0;
								p1.y += y;
								p2.y -= y;
							} else {
								let y = (texture_size.y - layout_height) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
								uv1.y += y;
								uv2.y -= y;
							}
						}
						FitType::Contain => {
							// 保持原有尺寸比例。保证内容尺寸一定可以在容器里面放得下。因此，此参数可能会在容器内留下空白。
							fill(&texture_size, &mut p1, &mut p2, layout_width, layout_height);
						}
						FitType::Cover => {
							// 保持原有尺寸比例。保证内容尺寸一定大于容器尺寸，宽度和高度至少有一个和容器一致。超出部分会被剪切
							if layout_width != 0.0 && layout_height != 0.0 {
								let rw = texture_size.x / layout_width;
								let rh = texture_size.y / layout_height;
				
								if rw > rh {
									let x = (texture_size.x - layout_width * rh) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
									uv1.x += x;
									uv2.x -= x;
								} else {
									let y = (texture_size.y - layout_height * rw) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
									uv1.y += y;
									uv2.y -= y;
								}
							}
						}
						FitType::ScaleDown => {
							// 如果内容尺寸小于容器尺寸，则直接显示None。否则就是Contain
							if texture_size.x <= layout_width && texture_size.y <= layout_height {
								let x = (layout_width - texture_size.x) / 2.0;
								let y = (layout_height - texture_size.y) / 2.0;
								p1.x += x;
								p1.y += y;
								p2.x -= x;
								p2.y -= y;
							} else {
								fill(&texture_size, &mut p1, &mut p2, layout_width, layout_height);
							}
						}
						FitType::Fill => (), // 填充。 内容拉伸填满整个容器，不保证保持原有的比例
					}

					render_flag &= !(1 << RenderFlagType::ImageRepeat as usize);

					// log::trace!("bg========================{:?}, {:?}");
					
				} else {
					render_flag |= 1 << RenderFlagType::ImageRepeat as usize;
					let w = p2.x - p1.x;
					let h = p2.y - p1.y;

					let (uoffset, ustep, uspace) = calc_step(w, texture_size.x, background_image_mod.repeat.x);
					let (voffset, vstep, vspace, ) = calc_step(h, texture_size.y, background_image_mod.repeat.y);

					instance_data.set_data(&ImageRepeatUniform(&[uoffset, voffset, ustep, vstep, uspace, vspace]));
					instance_data.set_data(&UvUniform(&[uv1.x, uv1.y, uv2.x, uv2.y]));
				}
				// instance_data.set_data(&BoxUniform(&[p1.x, p1.y, p2.x - p1.x, p2.y - p1.y]));
				// println!("bg========================{:?}, {:?}, {:?}, {:?}, {:?}", entity, draw_id, instance_index, background_image, layout);
				set_box(&world_matrix, &Aabb2::new(p1, p2), &mut instance_data);
				instance_data.set_data(&UvUniform(&[uv1.x, uv1.y, uv2.x, uv2.y]));
				instance_data.set_data(&TyUniform(&[render_flag as f32]));

			}

			// 这里世界矩阵和layout的设置，不单独抽取到一个system中， 有由当前设计的数据结构决定的
			// 当前的实例数据，将每个drawobj所有数据放在一个连续的内存中，当修改材质数据和修改世界矩阵、布局是连续的操作是，缓冲命中率高
			// 而像clip这类不是每个draw_obj都具有的属性，可以单独在一个system设置，不怎么会影响性能
			// let is_add =  background_image_texture_ref.is_changed(); // background_image_texture会提前创建
			// if is_add || world_matrix.is_changed() {
			// 	instance_data.set_data(&WorldUniform(world_matrix.as_slice()));
				
			// }
		}
	}

	log::trace!("bg image end========================");
}

/// 处理图片纹理加载成功，为没设置Size的节点设置默认的Size组件（与图片宽高相同）
/// 处理图片纹理删除， 如果实体依然存在，并且用户未设置Size组件， 则设置实体的Size为Undefined
pub fn set_image_default_size(
	mut param: ParamSet<(
		Query<(&mut Size, Has<BackgroundImageTexture>, &StyleMark), Removed<BackgroundImageTexture>>,
		Query<(&mut Size, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark), Changed<BackgroundImageTexture>>,
		 
	)>,
) {
    // 处理删除的图片纹理
    for (mut size, has_bg, style_mark) in param.p0().iter_mut() {
		if has_bg {
			continue;
		}
		// 本地样式和class样式都未设置宽度，设置默认图片宽度
		if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
			size.width = Dimension::Undefined;
		}

		// 本地样式和class样式都未设置高度，设置默认图片高度
		if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
			size.height = Dimension::Undefined;
		}
	}

    // 处理增加的图片问题
    for (mut size, texture, clip, style_mark) in param.p1().iter_mut() {
		if let Some(texture) = &texture.0 {
			// 本地样式和class样式都未设置宽度，设置默认图片宽度
			if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
				size.width = Dimension::Points(texture.width as f32 * *(clip.right - clip.left));
			}

			// 本地样式和class样式都未设置高度，设置默认图片高度
			if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
				size.height = Dimension::Points(texture.height as f32 * *(clip.bottom - clip.top));
			}
		}
	}
}

pub fn calc_step(show_size: f32, img_size: f32, rtype: ImageRepeatOption) -> (f32/*第一个item的偏移（不是整数倍可能需要居中）*/ , f32/*每个item占用的布局宽度*/, f32/*每个item在布局空间的实际渲染宽度（存在间隔）*/) {
	if img_size == 0.0  {
        return (0.0, 0.001, 0.001); // 避免为0， 因为其将作为除数
    }

    let repeat_count = show_size / img_size; // 区域内可重复的次数
    let repeat_count_round: f32 = repeat_count.round(); // 对重复次数四舍五入
    if eq_f32(repeat_count_round, repeat_count) {
        // 整数倍的情况（这里消除了浮点误差，大致为整数倍，都认为是整数倍）
        return (0.0, img_size, img_size);
    }
	
    match rtype {
        ImageRepeatOption::Repeat => (img_size - ((repeat_count - 1.0) % 2.0 * img_size / 2.0), img_size, img_size),
        ImageRepeatOption::Round => {
			// 如果能完整显示奇数个（可以放大， repeat_count.ceil()）， 则偏移为0， 否则需要偏移一半（为了显示对称）
			let mut offset = 0.0;
			if repeat_count.ceil() % 2.0 == 0.0 { // 偶数个
				offset = 1.0;
			}
			let step = show_size/repeat_count.ceil();
			return (offset * step, step, step);
		}
        ImageRepeatOption::Space => {
            let space = show_size % img_size; // 空白尺寸
			let f = repeat_count.floor();
			if repeat_count >= 2.0 {
				return (0.0, img_size + space / (f - 1.0), img_size)
			} else if repeat_count >= 1.0 {
				return (img_size, img_size + space / 2.0, img_size)
			} else {
				return ((img_size - space) / 2.0, img_size, img_size)
			}
        }
        _ => (0.0, show_size, show_size),
    }
}

// 按比例缩放到容器大小，居中显示
fn fill(size: &Vector2, p1: &mut Point2, p2: &mut Point2, w: f32, h: f32) {
    let rw = size.x / w;
    let rh = size.y / h;
    if rw > rh {
        let y = (h - size.y / rw) / 2.0;
        p1.y += y;
        p2.y -= y;
    } else {
        let x = (w - size.x / rh) / 2.0;
        p1.x += x;
        p2.x -= x;
    }
}
