//! 参考1：https://developer.mozilla.org/zh-CN/docs/Web/CSS/transition
//! 参考2：https://drafts.csswg.org/css-transitions/#transition-shorthand-property
//! 由于动画或transition设置属性后， 不能影响插值数据的end， 只能影响插值数据的start（设置transition的那一刻，将当前样式当做start，重新进行过度）
//! transation分为两步处理， 在usersetting之后和动画插值前的处理阶段(阶段1)、和动画插值后的阶段（阶段2）
//! 而用户设置的style， 既可以影响start， 也可以影响end（style设置一个初始值是， 影响start， 后续设置style， 影响end），
//! 阶段1： 
//! 	* 属性脏，需要将属性记录为start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和end）
//! 	* transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
//! 阶段2， 在动画推动之后执行， 动画推动之后， 又反向修改了style属性， 需要记录该属性为transition的start：
//! 	* 属性脏，则将属性记录在start上
//! 
//! 
//! 优化？（TODO）： 动画正在运行的节点，设置RuningForTransition组件， 在阶段2中只遍历有RuningForTransition组件的节点
//! TODO: 仅实现了一个简单版本的transition， 一些复杂的机制暂未实现

use pi_bevy_ecs_extend::{prelude::Layer, system_param::res::OrInitSingleResMut};
use pi_world::{filter::{Changed, Or}, prelude::{App, Entity, IntoSystemConfigs, Plugin, Query, SingleResMut, Ticker}, single_res::SingleRes, system_params::Local, world::World};

use pi_null::Null;
use pi_style::{style::StyleType, style_parse::Attribute};
use smallvec::SmallVec;

use crate::{
    components::{calc::{style_bit, StyleBit, StyleMark, StyleMarkType}, user::{
        serialize::StyleAttr, Transition,
    }, SettingComponentIds},
    resource::{animation_sheet::{KeyFramesSheet, TransitionData}, GlobalDirtyMark, OtherDirtyType}, system::{base::node::animation::calc_animation_2, system_set::UiSystemSet},
};

use crate::prelude::UiStage;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_system(UiStage, transition_1_1.in_set(UiSystemSet::NextSetting).run_if(transition_change)
				// .before(calc_animation)
			)
			.add_system(UiStage, transition_1_2.in_set(UiSystemSet::NextSetting)
				.after(transition_1_1)
			)
			.add_system(UiStage, transition_1_3.in_set(UiSystemSet::NextSetting)
				.after(transition_1_2)
				.before(calc_animation_2) // 在推动动画之前执行
			)
			.add_system(UiStage, transition_2.in_set(UiSystemSet::NextSetting).after(transition_1_3)
			)
			
		;
	}
}

lazy_static! {
    // 布局脏
    pub static ref TRANSITION_DIRTY: StyleMarkType = style_bit()
        .set_bit(StyleType::TransitionProperty as usize)
		.set_bit(StyleType::TransitionDuration as usize)
		.set_bit(StyleType::TransitionTimingFunction as usize)
		.set_bit(StyleType::TransitionDelay as usize)
		.set_bit(OtherDirtyType::NodeTreeAdd as usize)
		.set_bit(OtherDirtyType::NodeTreeDel as usize)
		.set_bit(OtherDirtyType::NodeTreeRemove as usize);
}


pub fn transition_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*TRANSITION_DIRTY)
}


/// 处理transition(阶段1的步骤1, 在usersetting之后运行， 在animation之前运行)（阶段1分两个步骤是因为读写引用冲突的问题）
/// 根据Transition.property，生成根据Transition.data
pub fn transition_1_1(
	mut query: Query<(&mut Transition, Entity, &Layer), Or<(Changed<Transition>, Changed<Layer>)>>,
	mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
) {
	// transition如果改变， 则删除原有binding， 重新计算需绑定的属性
	for (mut transition, entity, layer) in query.iter_mut() {
		let transition = transition.bypass_change_detection();

		keyframes_sheet.unbind_transition_all(entity);

		// 不在树上不处理
		if layer.layer().is_null() {
			continue;
		}

		transition.mark.fill(false);
		transition.is_all = std::usize::MAX;
		
		let mut j = 0;
		for i in transition.property.iter() {
			if (*i).is_null() {
				// i为null， 意味着transition.property长度为1， 并且表明所有可插值属性都需要作用在transition上
				// i为null，transition.property长度为非1，以及其他异常情况不处理
				transition.is_all = j;
				break;
			} else {
				// 标记对应属性需要应用transition
				transition.mark.set(*i, true);
			}
			j += 1;
		}

		// 拷贝旧的data
		if transition.is_all.is_null() {
			let mut i = transition.property.len();
			let mut data = std::mem::replace(&mut transition.data, SmallVec::with_capacity(i));
			for property in  transition.property.iter() {
				// 设置data的默认值（按照property的顺序组织的数组）
				transition.data.push(TransitionData {
					start: None,
					end: None,
					property: *property,
				});
			}
			// 拷贝旧值（双重循环， 当通常没有性能问题，property的长度通常都是一两个 ）
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
}

#[derive(Default)]
pub struct TransitionTempAttr {
	list: Vec<(Entity, TransitionCmd)>
}

enum TransitionCmd {
	Change1(TransitionAttrChange),
	Change2(StyleMarkType),
	Change3(usize, Attribute),
}

struct TransitionAttrChange {
	index: usize,
	property: u8,
	attr: Option<Attribute>,
	style_change: bool,
	unbind: bool,
}

// 每帧迭代所有存在Transition组件的节点
// 如果transition脏、layer脏、或者任何transition.property描述的样式脏
// 都需要重新生成TransitionAttrChange指令， 由transition_1_3来处理指令
pub fn transition_1_2(
	query: Query<(Ticker<&Transition>, &StyleMark, Entity, Ticker<&Layer>)>,
	world: &World,
	setting_components: Local<SettingComponentIds>,
) {
	let world1: &mut World = unsafe { &mut *(world as *const World as usize as *mut World)};
	let id = world1.init_single_res::<TransitionTempAttr>();
	let cmds = world1.index_single_res_mut::<TransitionTempAttr>(id).unwrap();

	for (transition, style_mark, entity, layer) in query.iter() {
		// 不在树上不处理
		if layer.layer().is_null() {
			continue;
		}
		// 属性脏，需要记录style属性到start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和wnd）
		// transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
		let transition_is_change = transition.is_changed() || layer.is_changed();
		let dirty: StyleMarkType = style_mark.dirty_style & transition.mark;
		if transition_is_change || dirty.any() {
			if transition.is_all.is_null() {
				for i in 0..transition.property.len() {
					let property = transition.property[i];
					let style_change = style_mark.dirty_style[property];
					if !(transition_is_change || style_change) {
						continue;
					}

					let mut cmd = TransitionAttrChange {
						index: i,
						property: property as u8,
						attr: None,
						style_change,
						unbind: false,
					};
					if style_change {
						cmd.attr = StyleAttr::get(property as u16, world, &setting_components, entity);
						if cmd.attr.is_none() {
							cmd.unbind = true; // 不存在属性， 则需要删除transition绑定
						}
					}
					cmds.list.push((entity, TransitionCmd::Change1(cmd)));
				}
			} else {
			 	// property为all， 所有属性都参与transition
				let style = (style_mark.class_style | &style_mark.local_style) & &*INTERPOLABLE_PROPERTY;
				// 属性个数发生改变， 需要重新设置data
				if &style != &transition.mark {
					cmds.list.push((entity, TransitionCmd::Change2(style)));
				}

				let mut i = 0;
				for property in transition.mark.iter_ones() {
					let style_change = style_mark.dirty_style[property];
					
					if !(style_change || transition_is_change) {
						continue;
					}
					
					let mut cmd = TransitionAttrChange {
						index: i,
						property: property as u8,
						attr: None,
						style_change,
						unbind: false,
					};

					if style_change {
						cmd.attr = StyleAttr::get(property as u16, world, &setting_components, entity);
						if cmd.attr.is_none() {
							cmd.unbind = true; // 不存在属性， 则需要删除transition绑定
						}
					}
					
					cmds.list.push((entity, TransitionCmd::Change1(cmd)));
					i += 1;
				}
				
			}
		}
	}
}

// 处理TransitionAttrChange指令，将指令
pub fn transition_1_3(
	mut query: Query<&mut Transition>,
	mut keyframes_sheet: SingleResMut<KeyFramesSheet>,
	mut cmds: OrInitSingleResMut<TransitionTempAttr>,
) {
	if cmds.list.len() == 0 {
		return;
	}
	for (entity, cmd) in cmds.list.drain(..) {
		let mut transition = query.get_mut(entity).unwrap();
		let transition = transition.bypass_change_detection();

		match cmd {
			TransitionCmd::Change1(r) => {
				let data: &mut TransitionData = &mut transition.data[r.index];
				if r.style_change {
					match r.attr {
						Some(attr) => match &data.start {
							Some(_r) => data.end = Some(attr),
							// 原有data.start为none， 表示第一次设置该属性，因此将start设置为attr， 这样将不会触发过度，只有后续修改会触发
							None => data.start = Some(attr), 
						},
						None => {
							// 属性被删除， 尝试解绑transition产生的动画
							data.start = None;
							data.end = None;
							keyframes_sheet.unbind_transition_single(r.property as usize, entity);
						},
					}	
				}

				if data.start.is_some() && data.end.is_some() {
					// transition修改， 或end修改，需要重新binding transition
					// transition将从当前start重新过度到end
					let _ = keyframes_sheet.bind_trasition(
						entity, 
						r.property as usize, 
						Transition::get_attr(r.index, &transition.duration), 
						Transition::get_attr(r.index, &transition.delay), 
						&Transition::get_attr(r.index, &transition.timing_function),  
						data
					);
				}

			},
			TransitionCmd::Change2(style) => {// 
				// change2表示插值属性个数发生变化，重新整理过度数组
				transition.mark = style;
				let mut datas = std::mem::take(&mut transition.data);
				for property in transition.mark.iter_ones() {
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
			},
			TransitionCmd::Change3(index, attr) => {
				let data = &mut transition.data[index];
				data.start = Some(attr);
			},
		}
	}
}

/// 处理transition(阶段2)
/// 属性脏，则将属性记录在start上
/// 
pub fn transition_2(
	query: Query<(&Transition, &StyleMark, Entity, &Layer)>,
	world: &World,
	setting_components: Local<SettingComponentIds>,
	mut cmds: SingleResMut<TransitionTempAttr>,
	// world: &mut World,
    // param: &mut SystemState<(EventReader<StyleChange>, Query<(&mut Transition, &StyleMark, Entity)>, OrInitSingleRes<IsRun>)>,

    // style_query: Local<StyleQuery>,
) {
	// let (dirty_list, query, is_run) = param.get_mut(world);

	// if is_run.0 {
	// 	return
	// }
	// 如果当前帧什么也不脏， 则不需要处理（TODO）
	// if dirty_list.len() == 0 {
	// 	return
	// }

	log::trace!("transition_2======");
	
	let cmds = &mut *cmds;
	for (transition, style_mark, entity, layer) in query.iter() {
		if layer.layer().is_null() {
			continue;
		}
		
		// 属性脏，需要记录属性为start或end（如果属性是被删除了， 则需要删除对应的插值曲线， 并重置start和wnd）
		// transition_is_change脏， 或属性脏, 如果记录后，既存在start， 也存在end， 则需要重新绑定插值曲线
		let dirty: StyleMarkType = style_mark.dirty_style & transition.mark;
		if dirty.any() {
			let mut set_data = |i: usize, property: usize| {
				let style_change = style_mark.dirty_style[property];
				if style_change {
					match StyleAttr::get(property as u16, world, &setting_components, entity) {
						Some(r) => cmds.list.push((entity, TransitionCmd::Change3(i, r))), // 该指令在下一帧的transition_1_3中处理
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

	// 定义可插值属性（只有这些属性才可以被transition过度）
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
