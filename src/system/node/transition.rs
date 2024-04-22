//! 由于动画或transition设置属性后， 不能影响插值数据的end， 只能影响插值数据的start
//! 而用户设置的style， 既可以影响start， 也可以影响end，
//! transation分为两步处理， 在usersetting之后和动画插值前的处理阶段(阶段1)、和动画插值后的阶段（阶段2）
//! 阶段1： 
//! 	* 属性脏，需要记录属性为start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和wnd）
//! 	* transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
//! 阶段2：
//! 	* 属性脏，则记录属性在start上
//! 
//! 
//! 优化？（TODO）： 动画正在运行的节点，设置RuningForTransition组件， 在节点2中只遍历有RuningForTransition组件的节点

use std::mem::transmute;

use bevy_app::{Plugin, App};
use bevy_ecs::{
    prelude::{Entity, World},
    system::{Local, Query, ResMut, SystemState}, event::EventReader, change_detection::{DetectChanges, DetectChangesMut}, schedule::IntoSystemConfigs,
};
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_null::Null;
use pi_style::style::StyleType;
use smallvec::SmallVec;

use crate::{
    components::{calc::{style_bit, StyleBit, StyleMark, StyleMarkType}, user::{
        serialize::{Setting, StyleAttr, StyleQuery}, Transition,
    }},
    resource::animation_sheet::{KeyFramesSheet, ObjKey, TransitionData}, system::{draw_obj::calc_text::IsRun, system_set::UiSystemSet},
};

use crate::system::node::user_setting::StyleChange;
use crate::prelude::UiSchedule;

use super::animation::calc_animation;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_systems(UiSchedule, transition_1.in_set(UiSystemSet::NextSetting).before(calc_animation))
			.add_systems(UiSchedule, transition_2.in_set(UiSystemSet::NextSetting).after(calc_animation))
		;
	}
}

// // 一个标记
// // 节点存在Transition时，该节点如果有正在运行的动画
// pub struct RuningForTransition;

/// 处理transition(阶段1)
pub fn transition_1(

	world: &mut World,
    param: &mut SystemState<(EventReader<StyleChange>, Query<(&mut Transition, &StyleMark, Entity)>, ResMut<KeyFramesSheet>, OrInitRes<IsRun>)>,

    style_query: Local<StyleQuery>,
) {
	let (dirty_list, query, mut keyframes_sheet, is_run) = param.get_mut(world);

	if is_run.0 {
		return
	}
	// 如果当前帧什么也不脏， 则不需要处理
	if dirty_list.len() == 0 {
		return
	}
	log::debug!("transition_1======");

	// 取style的StyleQuery会和此查询冲突， 因此用非安全方法绕过借用问题
	// 但逻辑保证了安全性
	let keyframes_sheet: &'static mut KeyFramesSheet = unsafe{transmute(&mut *keyframes_sheet)};
	let mut query: Query<'static, 'static, (&mut Transition, &StyleMark, Entity)> = unsafe{transmute(query)};

	let mut setting = Setting { style: &style_query, world };
	
	for (mut transition, style_mark, entity) in query.iter_mut() {
		let transition_is_change = transition.is_changed();
		let transition = transition.bypass_change_detection();
		// transition如果改变， 则删除原有binding， 重新计算需绑定的属性
		if transition_is_change {
			keyframes_sheet.unbind_transition_all(ObjKey(entity));

			transition.mark.fill(false);
			transition.is_all = std::usize::MAX;
			
			let mut j = 0;
			for i in transition.property.iter() {
				if (*i).is_null() {
					transition.is_all = j;
					break;
				} else {
					transition.mark.set(*i, true);
				}
				j += 1;
			}

			// 拷贝旧的data
			if transition.is_all.is_null() {
				let mut i = transition.property.len();
				let mut data = std::mem::replace(&mut transition.data, SmallVec::with_capacity(i));
				for property in  transition.property.iter() {
					// 设置data的默认值
					transition.data.push(TransitionData {
						start: None,
						end: None,
						property: *property,
					});
				}
				while data.len() > 0 && i > 0 {
					i -= 1;
					let property = transition.property[i];

					let mut j = data.len();
					while j > 0 {
						j -= 1;
						if data[j].property == property {
							let data = data.swap_remove(j);
							transition.data[i] = data;
							break;
						}
					}
				}
			}
		}

		// 属性脏，需要记录属性为start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和wnd）
		// transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
		let dirty: StyleMarkType = style_mark.dirty_style & transition.mark;
		if transition_is_change || dirty.any() {
			if transition.is_all.is_null() {
				for i in 0..transition.property.len() {
					let property = transition.property[i];
					let data = &mut transition.data[i];
					let style_change = style_mark.dirty_style[property];
					if style_change {
						let attr = match StyleAttr::get(property as u8, &mut setting, entity) {
							Some(r) => r,
							None => {
								data.start = None;
								data.end = None;
								keyframes_sheet.unbind_transition_single(property, ObjKey(entity));
								continue;
							},
						};
						match &data.start {
							Some(_r) => data.end = Some(attr),
							None => data.start = Some(attr),
						}
					}

					if (transition_is_change || style_change) && data.start.is_some() && data.end.is_some() {
						// 由于修改了end，需要重新binding transition
						// transition将从当前start重新过度到end
						let _ = keyframes_sheet.bind_trasition(
							ObjKey(entity), 
							property, 
							Transition::get_attr(i, &transition.duration), 
							Transition::get_attr(i, &transition.delay), 
							&Transition::get_attr(i, &transition.timing_function),  
							data
						);
					}
				}
			} else {
			 	// property为all， 所有属性都参与transition
				let style = (style_mark.class_style | &style_mark.local_style) & &*INTERPOLABLE_PROPERTY;
				// 属性个数发生改变， 需要重新设置data
				if &style != &transition.mark {
					transition.mark = style;
					let mut datas = std::mem::take(&mut transition.data);
					for property in style.iter_ones() {
						let mut j = 0;
						while j < datas.len() {
							if datas[j].property == property {
								let data = datas.swap_remove(j);
								transition.data.push(data);
								break;
							}
							j += 1;
						}
	
						// 没有旧的data, 设置默认的
						if datas.len() == j {
							transition.data.push(TransitionData {
								start: None,
								end: None,
								property,
							});
						}
					}
				}

				let mut i = 0;
				for property in transition.mark.iter_ones() {
					let data = &mut transition.data[i];
					let style_change = style_mark.dirty_style[property];
					if style_change {
						let attr = match StyleAttr::get(property as u8, &mut setting, entity) {
							Some(r) => r,
							None => {
								data.start = None;
								data.end = None;
								keyframes_sheet.unbind_transition_single(property, ObjKey(entity));
								continue;
							},
						};
						match &data.start {
							Some(_r) => data.end = Some(attr),
							None => data.start = Some(attr),
						}
					}

					if (transition_is_change || style_change) && data.start.is_some() && data.end.is_some() {
						// 由于修改了end，需要重新binding transition
						// transition将从当前start重新过度到end
						let _ = keyframes_sheet.bind_trasition(
							ObjKey(entity), 
							property, 
							Transition::get_attr(transition.is_all, &transition.duration), 
							Transition::get_attr(transition.is_all, &transition.delay), 
							&Transition::get_attr(transition.is_all, &transition.timing_function), 
							data);
					}

					i += 1;
				}
				
			}
		}
	}
}

/// 处理transition(阶段2)
pub fn transition_2(

	world: &mut World,
    param: &mut SystemState<(EventReader<StyleChange>, Query<(&mut Transition, &StyleMark, Entity)>, OrInitRes<IsRun>)>,

    style_query: Local<StyleQuery>,
) {
	let (dirty_list, query, is_run) = param.get_mut(world);

	if is_run.0 {
		return
	}
	// 如果当前帧什么也不脏， 则不需要处理
	if dirty_list.len() == 0 {
		return
	}

	log::debug!("transition_2======");
	// 取style的StyleQuery会和此查询冲突， 因此用非安全方法绕过借用问题
	// 但逻辑保证了安全性
	let mut query: Query<'static, 'static, (&mut Transition, &StyleMark, Entity)> = unsafe{transmute(query)};

	let mut setting = Setting { style: &style_query, world };
	
	for (mut transition, style_mark, entity) in query.iter_mut() {
		let transition = transition.bypass_change_detection();
		
		// 属性脏，需要记录属性为start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和wnd）
		// transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
		let dirty: StyleMarkType = style_mark.dirty_style & transition.mark;
		if dirty.any() {
			let mut set_data = |i: usize, property: usize| {
				let data = &mut transition.data[i];
				let style_change = style_mark.dirty_style[property];
				if style_change {
					match StyleAttr::get(property as u8, &mut setting, entity) {
						Some(r) => data.start = Some(r),
						None => return,
					};
				}
			};
			if !transition.is_all.is_null() { // property为all
				let change_style = transition.mark & style_mark.dirty_style;
				let mut i = 0;
				for property in change_style.iter_ones() {
					set_data(i, property);
					i += 1;
				}
			} else { // property不是all
				for i in 0..transition.property.len() {
					let property = transition.property[i];
					set_data(i, property);
				}
			}
		}
	}
}

lazy_static! {

	pub static ref INTERPOLABLE_PROPERTY: StyleMarkType = style_bit().set_bit(StyleType::BackgroundRepeat as usize)
	.set_bit(StyleType::Color as usize)
	.set_bit(StyleType::BackgroundImageClip as usize)
	.set_bit(StyleType::BackgroundColor as usize)
	.set_bit(StyleType::BorderColor as usize)
	.set_bit(StyleType::Hsi as usize)
	.set_bit(StyleType::Blur as usize)
	.set_bit(StyleType::Transform as usize)
	.set_bit(StyleType::BorderRadius as usize)

    .set_bit(StyleType::Width as usize)
    .set_bit(StyleType::Height as usize)

    .set_bit(StyleType::MarginTop as usize)
    .set_bit(StyleType::MarginRight as usize)
    .set_bit(StyleType::MarginBottom as usize)
    .set_bit(StyleType::MarginLeft as usize)

    .set_bit(StyleType::PaddingTop as usize)
    .set_bit(StyleType::PaddingRight as usize)
    .set_bit(StyleType::PaddingBottom as usize)
    .set_bit(StyleType::PaddingLeft as usize)

    .set_bit(StyleType::BorderTop as usize)
    .set_bit(StyleType::BorderRight as usize)
    .set_bit(StyleType::BorderBottom as usize)
    .set_bit(StyleType::BorderLeft as usize)

    .set_bit(StyleType::PositionTop as usize)
    .set_bit(StyleType::PositionRight as usize)
    .set_bit(StyleType::PositionBottom as usize)
    .set_bit(StyleType::PositionLeft as usize)

    .set_bit(StyleType::MinWidth as usize)
    .set_bit(StyleType::MinHeight as usize)
    .set_bit(StyleType::MaxHeight as usize)
    .set_bit(StyleType::MaxWidth as usize)

    .set_bit(StyleType::Opacity as usize)

	.set_bit(StyleType::Translate as usize)
	.set_bit(StyleType::Scale as usize)
	.set_bit(StyleType::Rotate as usize);
}
