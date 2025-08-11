

use std::ops::Range;

use pi_flex_layout::prelude::Rect;
use pi_null::Null;
use pi_world::filter::Or;
use pi_world::param_set::ParamSet;
use pi_world::prelude::{Changed, With, Query, Plugin, OrDefault, IntoSystemConfigs, Has, ComponentRemoved};
use pi_bevy_ecs_extend::prelude::{Layer, OrInitSingleRes, OrInitSingleResMut};

use pi_flex_layout::style::Dimension;
use pi_style::style::{ImageRepeatOption, StyleType};
use pi_world::single_res::{SingleRes, SingleResMut};
use pi_world::world::Entity;

use crate::components::calc::{style_bit, BackgroundImageTexture, DrawList, IsRotate, LayoutResult, RectSdfSlice, SdfSlice, SdfUv, StyleBit, StyleMark, StyleMarkType, Texture, WorldMatrix};
use crate::components::draw_obj::{BackgroundImageMark, BoxType, InstanceIndex, RenderCount, TempGeo};
use crate::components::user::{BackgroundImageClip, BackgroundImageMod, FitType, Opacity, Point2, Size, Vector2};
use crate::resource::draw_obj::InstanceContext;
use crate::resource::{BackgroundImageRenderObjType, GlobalDirtyMark, OtherDirtyType};
use crate::prelude::UiStage;

use crate::system::base::draw_obj::life_drawobj::update_render_instance_data;
use crate::system::base::draw_obj::set_geo_uniform::set_matrix_uniform;
use crate::system::base::node::layout::calc_layout;
use crate::system::base::node::transition::transition_2;
use crate::system::draw_obj::geo_split::{set_grid_instance, DirectionDesc, RepeatInfo};
use crate::system::system_set::UiSystemSet;
use crate::utils::tools::{eq_f32, is_large_size};
use crate::components::user::BackgroundImage;

use crate::system::base::draw_obj::{image_texture_load, life_drawobj, set_box_type, set_box_type_count};
use crate::resource::IsRun;

use super::geo_split::GridBufer;

pub struct BackgroundImagePlugin;

impl Plugin for BackgroundImagePlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
			.add_system(UiStage, image_texture_load::image_load::<BackgroundImage, BackgroundImageTexture, {OtherDirtyType::BackgroundImageTexture}, BackgroundImageRenderObjType>
				.in_set(UiSystemSet::NextSetting)

				.after(transition_2))
			.add_system(UiStage, set_image_default_size.in_set(UiSystemSet::BaseCalc).run_if(background_image_life_change)
				.before(calc_layout)
			)
			.add_system(UiStage, 
				life_drawobj::draw_object_life_new::<
					BackgroundImageTexture,
					BackgroundImageRenderObjType,
					BackgroundImageMark,
					{ BACKGROUND_IMAGE_ORDER },
					{ BoxType::None },
				>
					.in_set(UiSystemSet::LifeDrawObject)
					.run_if(background_image_life_change)
					.after(image_texture_load::image_load::<BackgroundImage, BackgroundImageTexture, {OtherDirtyType::BackgroundImageTexture}, BackgroundImageRenderObjType>),
			)
			.add_system(UiStage, 
				calc_background_image
					.after(crate::system::base::node::world_matrix::cal_matrix)
					.before(set_matrix_uniform)
					.in_set(UiSystemSet::PrepareDrawObj)
					.run_if(background_texture_change)
			)
			.add_system(UiStage, 
				calc_background_image_instance_count
					.after(UiSystemSet::LifeDrawObjectFlush)
					.before(update_render_instance_data)
					.after(calc_layout)
					.run_if(background_texture_change)
					.in_set(UiSystemSet::IsRun)
			)
		;
    }
}

pub const BACKGROUND_IMAGE_ORDER: u8 = 5;

lazy_static! {
	// 不需要关心布局脏， 布局脏， 世界矩阵脏， 会转化为对应渲染类型的全局脏
	pub static ref BACKGROUND_TEXTURE_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BackgroundImageClip as usize)
		.set_bit(OtherDirtyType::BackgroundImageTexture as usize)
		.set_bit(OtherDirtyType::NodeTreeAdd as usize)
		.set_bit(StyleType::ObjectFit as usize); 
}

pub fn background_texture_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY)
}

pub fn background_image_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(StyleType::BackgroundImage as usize).map_or(false, |display| {*display == true})
}

pub fn background_image_life_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.get(OtherDirtyType::BackgroundImageTexture as usize).map_or(false, |display| {*display == true})
}

#[derive(Default)]
pub struct BackgroundImageTemp (pub GridBufer, pub Vec<(Entity, (Range<usize>, Range<usize>), bool)>);

pub fn calc_background_image_instance_count(
	// dirty_list: Event<StyleChange>,
	query1: Query<
		(
			(
				pi_world::world::Entity,
				&LayoutResult,
				&BackgroundImageTexture,
				OrDefault<BackgroundImageClip>,
				OrDefault<BackgroundImageMod>,
				&BackgroundImage,
				Option<&SdfSlice>,
				OrDefault<SdfUv>,
				OrDefault<Opacity>,
				&IsRotate,
			),
			&DrawList,
			&Layer,
		),
		Or<(Changed<BackgroundImageTexture>, Changed<BackgroundImageClip>, Changed<WorldMatrix>, Changed<SdfSlice>, Changed<BackgroundImageMod>, Changed<Opacity>, Changed<Layer>)>,
	>,
    mut query_draw: Query<(&mut BoxType, &mut RenderCount)>,
	r: OrInitSingleRes<IsRun>,
	render_type: OrInitSingleRes<BackgroundImageRenderObjType>,
	mut grid_buffer: OrInitSingleResMut<BackgroundImageTemp>,
	mut mark: SingleResMut<GlobalDirtyMark>,
	rect_sdf_slice: OrInitSingleRes<RectSdfSlice>,
) {
	if r.0 {
		return;
	}
	log::trace!("bg image========================{:?}", mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY));
	let render_type = ***render_type;

	let mut layout_slice = Rect {
		left: 0.0,
		right: 1.0,
		top: 0.0,
		bottom: 1.0,
	};
	let layout_slice1 = Rect {
		left: 0.0,
		right: 1.0,
		top: 0.0,
		bottom: 1.0,
	};
	if mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY) {
		for (r, draw_list, layer) in query1.iter() {
			if layer.layer().is_null() {
				continue;
			}
			let draw_id = match draw_list.get_one(render_type) {
				Some(r) => r.id,
				None => continue,
			};
			log::debug!("calc_background_image_instance_count===={:?}", &r.0);
			if let Some(ret) = calc_background_image_inner(r, &mut grid_buffer.0, &mut layout_slice, &rect_sdf_slice, &layout_slice1) {
				grid_buffer.1.push((draw_id, ret.0,  ret.1.opacity > 0));
				
				set_box_type_count(draw_id, BoxType::None, ret.1, &mut query_draw, &mut mark);
			} else {
				set_box_type(draw_id, BoxType::None2, &mut query_draw);
			};

		}
	}
}


/// 设置背景颜色的顶点，和颜色Uniform
pub fn calc_background_image(
	mut grid_buffer: OrInitSingleResMut<BackgroundImageTemp>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    query_draw: Query<&InstanceIndex, With<BackgroundImageMark>>,
	r: OrInitSingleRes<IsRun>,
	mark: SingleRes<GlobalDirtyMark>,
) {
	if r.0 {
		return;
	}

	let grid_buffer = &mut **grid_buffer;
	log::trace!("bg image========================{:?}", mark.mark.has_any(&*BACKGROUND_TEXTURE_DIRTY));
	for (draw_id, (x_range, y_range), is_opacity) in grid_buffer.1.drain(..) {

		
		if let Ok(instanceindex) = query_draw.get(draw_id) {
			set_grid_instance(
				&grid_buffer.0,
				x_range,
				y_range,
				instanceindex.index(is_opacity).start,
				&mut instances,
			);
		}
	}

	grid_buffer.0.positions.clear();
	grid_buffer.0.uvs.clear();
	grid_buffer.0.sdf_uvs.clear();
	
	// println!("bg image end========================{:?}", pi_time::Instant::now() - t1);
	log::trace!("bg image end========================");
}

pub fn calc_background_image_inner(
	data: (
		pi_world::world::Entity,
		&LayoutResult,
		&BackgroundImageTexture,
		&BackgroundImageClip,
		&BackgroundImageMod,
		&BackgroundImage,
		Option<&SdfSlice>,
		&SdfUv,
		&Opacity,
		&IsRotate,
	),
	// default_sdf_uv: &Rect<f32>, // 默认sdfuv
	grid_buffer: &mut GridBufer,
	layout_slice: &mut Rect<f32>, // 
	rect_sdf_slice: &RectSdfSlice,
	rect_sdf_slice1: &Rect<f32>,

	// query_up: Query<(&pi_bevy_ecs_extend::prelude::Up, &LayoutResult, &WorldMatrix)>,
    // query_draw: &mut Query<(&InstanceIndex, &mut BoxType, &RenderCount), With<BackgroundImageMark>>,
	// render_type: RenderObjType,
) -> Option<((Range<usize>, Range<usize>), RenderCount/*rendercount*/)> {
	let (entity, layout, background_image_texture_ref, background_image_clip, background_image_mod, background_image, sdf_slice, sdf_uv, opacity, is_rotate) = data;
	let sdf_uv = &sdf_uv.0;
	let background_image_texture = match &background_image_texture_ref.0 {
		Some(r) => {
			// 图片不一致， 返回
			// if let Texture::All(r) = r {
			// 	if *r.key() != background_image.0.str_hash() as u64 {
			// 		log::debug!("calc_background_image1, entity={:?}, {:?}", entity, (r.key(), background_image.0.str_hash()));
			// 		return None;
			// 	}
			// } else 
			if let Texture::Frame(_r, key) = r {
				if key.str_hash() != background_image.0.str_hash() as u64 {
					log::debug!("calc_background_image1, entity={:?}, {:?}", entity, (key, background_image.0.str_hash()));
					return None;
				}
			}
			r
		},
		None => {
			log::debug!("calc_background_image2, entity={:?}", entity);
			return None
		}, 
	};

	let padding_aabb = layout.padding_aabb();
	let padding_width = padding_aabb.maxs.x - padding_aabb.mins.x;
	let padding_height = padding_aabb.maxs.y - padding_aabb.mins.y;
	let large_size = is_large_size( padding_width, padding_height);
	let is_opacity = background_image_texture.is_opacity() && opacity.0 > 0.99;
	let mut pstart = padding_aabb.mins.clone();
	let mut pend = padding_aabb.maxs.clone();
	// let (has_slice, sdf_slice) = match sdf_slice {
	// 	Some(r) => (true, r.clone()),
	// 	None => (false, SdfSlice {
	// 		sdf_slice: rect.clone(),
	// 		layout_slice: rect,
	// 	}),
	// };
	

	// if background_image.0.as_str().contains("yxxq_lv1") {
	// 	let (layout_width, layout_height) = layout.size();
	// 	log::debug!("=============!!!!!!!!!!!=============={:?}", (entity, background_image.0.as_str(), layout_width, layout_height, &sdf_uv, &sdf_slice));
	// }
	{
		let s = background_image_texture.size();
		let texture_size = Vector2::new(
			s.width as f32 * (background_image_clip.right - background_image_clip.left).abs(),
			s.height as f32 * (background_image_clip.bottom - background_image_clip.top).abs(),
		);
		let (mut uv1, mut uv2) = background_image_texture.to_uv(&background_image_clip);
		let is_stretch = background_image_mod.repeat.x == ImageRepeatOption::Stretch && background_image_mod.repeat.y == ImageRepeatOption::Stretch;
		if is_stretch {
			// 在x、y方向上都为拉伸时， object_fit才生效
			match background_image_mod.object_fit { // 如果不是填充，总是居中显示。 如果在范围内，则修改点坐标。如果超出的部分，会进行剪切，剪切会修改uv坐标。
				FitType::None => {
					// 保持原有尺寸比例。同时保持内容原始尺寸大小。 超出部分会被剪切
					if texture_size.x <= padding_width {
						let x = (padding_width - texture_size.x) / 2.0;
						pstart.x += x;
						pend.x -= x;
					} else {
						let x = (texture_size.x - padding_width) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
						uv1.x += x;
						uv2.x -= x;
					}
					if texture_size.y <= padding_height {
						let y = (padding_height - texture_size.y) / 2.0;
						pstart.y += y;
						pend.y -= y;
					} else {
						let y = (texture_size.y - padding_height) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
						uv1.y += y;
						uv2.y -= y;
					}
				}
				FitType::Contain => {
					// 保持原有尺寸比例。保证内容尺寸一定可以在容器里面放得下。因此，此参数可能会在容器内留下空白。
					fill(&texture_size, &mut pstart, &mut pend, padding_width, padding_height);
				}
				FitType::Cover => {
					// 保持原有尺寸比例。保证内容尺寸一定大于容器尺寸，宽度和高度至少有一个和容器一致。超出部分会被剪切
					if padding_width != 0.0 && padding_height != 0.0 {
						let rw = texture_size.x / padding_width;
						let rh = texture_size.y / padding_height;
		
						if rw > rh {
							let x = (texture_size.x - padding_width * rh) * (uv2.x - uv1.x) * 0.5 / texture_size.x;
							uv1.x += x;
							uv2.x -= x;
						} else {
							let y = (texture_size.y - padding_height * rw) * (uv2.y - uv1.y) * 0.5 / texture_size.y;
							uv1.y += y;
							uv2.y -= y;
						}
					}
				}
				FitType::ScaleDown => {
					// 如果内容尺寸小于容器尺寸，则直接显示None。否则就是Contain
					if texture_size.x <= padding_width && texture_size.y <= padding_height {
						let x = (padding_width - texture_size.x) / 2.0;
						let y = (padding_height - texture_size.y) / 2.0;
						pstart.x += x;
						pstart.y += y;
						pend.x -= x;
						pend.y -= y;
					} else {
						fill(&texture_size, &mut pstart, &mut pend, padding_width, padding_height);
					}
				}
				FitType::Fill => (), // 填充。 内容拉伸填满整个容器，不保证保持原有的比例
			}

			
		}

	
		let width = pend.x - pstart.x;
		let height = pend.y - pstart.y;
		let sdf_slice = match sdf_slice {
			Some(slice) => Some((&slice.sdf_slice, &slice.layout_slice)),
			_ => if is_opacity && large_size && is_rotate.0 {
				*layout_slice = Rect {
					left: 2.0 / width,
					right: (width - 2.0) / width,
					top: 2.0 / height,
					bottom: (height - 2.0) / height,
				};
				Some((&rect_sdf_slice.0, &*layout_slice))
			} else if is_stretch {
				// 半透明不需要切九宫格， 整体渲染
				None
			} else {
				Some((rect_sdf_slice1, rect_sdf_slice1))
			}
		};

		let (sdf_slice, layout_slice) = match sdf_slice {
			None => {
				grid_buffer.positions.extend_from_slice(&[pstart.x, pend.x, pstart.y, pend.y]);
				grid_buffer.uvs.extend_from_slice(&[uv1.x, uv2.x, uv1.y, uv2.y]);
				grid_buffer.sdf_uvs.extend_from_slice(&[sdf_uv.left, sdf_uv.right, sdf_uv.top, sdf_uv.bottom]);
				let len = grid_buffer.positions.len();
				
				let render_count = if is_opacity {
					RenderCount {
						opacity: 1,
						transparent: 0,
					}
				} else {
					RenderCount {
						opacity: 0,
						transparent: 1,
					}
				};
				log::debug!("bg image stretch========================{:?}", (entity , background_image.0.as_str(), [pstart.x, pend.x, pend.y, pend.y], [uv1.x, uv2.x, uv1.y, uv2.y], &padding_aabb));
				return Some(((len-4..len-2, len-2..len), render_count));
			},
			Some(r) => r,
		};

		// 优化有圆角或旋转， 但非repeat的情况的不透明渲染切分， TODO

		let (layout_width, layout_height) = layout.size();
		let layout_slice_absolute = Rect {
			left: layout_slice.left * layout_width,
			right: layout_slice.right * layout_width,
			top: layout_slice.top * layout_height,
			bottom: layout_slice.bottom * layout_height,
		};
		// use pi_key_alloter::Key;
		// if background_image.0.as_str().contains("shouZhi") {
		// 	log::debug!("bg image1========================{:?}", (entity , background_image.0.as_str(), [pstart.x, pend.x, pend.y, pend.y], [uv1.x, uv2.x, uv1.y, uv2.y], &padding_aabb));
		// }

		let w = pend.x - pstart.x;
		let h = pend.y - pstart.y;
		let (uspace, ulayout, ucount) = calc_step(w, texture_size.x, background_image_mod.repeat.x);
		let (vspace, vlayout, vcount) = calc_step(h, texture_size.y, background_image_mod.repeat.y);

		log::debug!("bg image split========================{:?}, \n{:?}, {:?}, \n{:?}", 
			background_image.0.as_str(),
			(pstart.x, uspace, ulayout, ucount), (pstart.y, vspace, vlayout, vcount),
			(w, h, texture_size, &background_image_mod.repeat)
		);

		let x_range = TempGeo::grid_split(&RepeatInfo {
			start: 0.0,
			end: pend.x,
			bound_step: 0.0,
			space: uspace,
			item_size: ulayout,
		}, 
		grid_buffer,
		&DirectionDesc { 
			sdf_uv: sdf_uv.left..sdf_uv.right, 
			sdf_slice: sdf_slice.left..sdf_slice.right, 
			layout_range: pstart.x..pend.x, 
			split: layout_slice_absolute.left..layout_slice_absolute.right
		},
		uv1.x..uv2.x);

		let y_range = TempGeo::grid_split(&RepeatInfo {
			start: 0.0,
			end: pend.y,
			bound_step: 0.0,
			space: vspace,
			item_size: vlayout,
		}, 
		grid_buffer,
		&DirectionDesc {
			sdf_uv: sdf_uv.top..sdf_uv.bottom, 
			sdf_slice: sdf_slice.top..sdf_slice.bottom, 
			layout_range: pstart.y..pend.y, 
			split: layout_slice_absolute.top.. layout_slice_absolute.bottom,
			
		}, uv1.y..uv2.y);
		let instance_count = (y_range.len() / 2) * (x_range.len() / 2);
		log::debug!("BackgroundImageEvent==============entity: {:?}, instance_count: {:?}, \nsdf_uv: {:?}, \nsdf_slice: {:?}, \nlayout_slice: {:?}", entity, instance_count,  sdf_uv, sdf_slice, layout_slice);
		return Some(((x_range, y_range), RenderCount {
			opacity: 0,
			transparent: instance_count as u32, // 被切分后, 全部当做半透明处理， TODO
		}));
	}
}


/// 处理图片纹理加载成功，为没设置Size的节点设置默认的Size组件（与图片宽高相同）
/// 处理图片纹理删除， 如果实体依然存在，并且用户未设置Size组件， 则设置实体的Size为Undefined
pub fn set_image_default_size(
	removed: ComponentRemoved<BackgroundImageTexture>,
	mut param: ParamSet<(
		Query<(&mut Size, Has<BackgroundImageTexture>, &StyleMark)>,
		Query<(&mut Size, &BackgroundImageTexture, OrDefault<BackgroundImageClip>, &StyleMark), Changed<BackgroundImageTexture>>,
		 
	)>,
) {
    // 处理删除的图片纹理
	let p0 = param.p0();
	for removed_id in removed.iter() {
		if let Ok((mut size, has_bg, style_mark)) = p0.get_mut(*removed_id) {
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
	}
    
    // 处理增加的图片问题
    for (mut size, texture, clip, style_mark) in param.p1().iter_mut() {
		if let Some(texture) = &texture.0 {
			let s = texture.size();
			// 本地样式和class样式都未设置宽度，设置默认图片宽度
			if style_mark.local_style[StyleType::Width as usize] == false && style_mark.class_style[StyleType::Width as usize] == false {
				size.width = Dimension::Points(s.width as f32 * *(clip.right - clip.left));
			}

			// 本地样式和class样式都未设置高度，设置默认图片高度
			if style_mark.local_style[StyleType::Height as usize] == false && style_mark.class_style[StyleType::Height as usize] == false {
				size.height = Dimension::Points(s.height as f32 * *(clip.bottom - clip.top));
			}
		}
	}
}

pub fn calc_step(show_size: f32, img_size: f32, rtype: ImageRepeatOption) -> (f32/*每个item间隔）*/, f32/*每个item占用的布局宽度*/, usize/*重复次数*/) {
	if img_size == 0.0  {
        return (0.001, 0.0, 1); // 避免为0， 因为其将作为除数
    }

    let repeat_count = show_size / img_size; // 区域内可重复的次数
	log::debug!("repeat_count_round: {:?}", (repeat_count, repeat_count.round()));
    let repeat_count_round: f32 = repeat_count.round(); // 对重复次数四舍五入
    if eq_f32(repeat_count_round, repeat_count) {
        // 整数倍的情况（这里消除了浮点误差，大致为整数倍，都认为是整数倍）
        return (0.0, img_size, repeat_count_round as usize);
    }

	
    match rtype {
        ImageRepeatOption::Repeat => { // repeat
			let floor_count = repeat_count.ceil(); // 向上取整
			log::debug!("repeat_count: {:?}", (0.0, 0.0, 0.0, img_size, floor_count as usize));
			(0.0, img_size, floor_count as usize)
		},
        ImageRepeatOption::Round => { // 总是显示整数个图片， 四舍五入
			// 如果能完整显示奇数个（可以缩小， repeat_count.ceil()）， 则偏移为0， 否则需要偏移一半（为了显示对称）
			let count = repeat_count.round().max(1.0);
			let layout_size = show_size/count;
			return (0.0, layout_size, count as usize);
		}
        ImageRepeatOption::Space => {
            let space = show_size % img_size; // 空白尺寸
			let f = repeat_count.floor();
			let count = f.max(1.0) as usize;
			if repeat_count >= 2.0 {
				return (space / (f - 1.0), img_size, count)
			} else {
				return (0.0, img_size, count)
			}
        }
        ImageRepeatOption::Stretch => (0.0, show_size, 1),
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
