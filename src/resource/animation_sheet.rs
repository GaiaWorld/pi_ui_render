//！ 动画表资源
use std::{any::Any, collections::VecDeque, mem::replace};

use pi_world::{prelude::Entity, world::ComponentIndex};
use bitvec::array::BitArray;
use ordered_float::NotNan;
use pi_style::{style::{AnimationPlayState, GUI_STYLE_COUNT}, style_type::{Attr, STYLE_COUNT}};
use pi_animation::{
    amount::AnimationAmountCalc,
    animation::AnimationInfo,
    animation_group::AnimationGroupID,
    animation_group_manager::AnimationGroupManagerDefault,
    animation_listener::EAnimationEvent,
    animation_result_pool::TypeAnimationResultPool,
    base::EFillMode,
    loop_mode::ELoopMode,
    runtime_info::RuntimeInfoMap,
    type_animation_context::{AnimationContextAmount, AnimationContextMgr, TypeAnimationContext},
};
use pi_atom::Atom;
use pi_curves::{
    curve::{frame::FrameDataValue, frame_curve::FrameCurve, FramePerSecond},
    easing::EEasingMode,
};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_print_any::out_any;
use pi_share::Share;
use pi_slotmap::{DefaultKey, SecondaryMap};
use pi_style::style::{AnimationDirection, AnimationTimingFunction};
use pi_style::{style_parse::Attribute, style_type::*};
use smallvec::SmallVec;

use crate::{components::{calc::{StyleMark, StyleMarkType}, user::{serialize::{AttrSet, Setting, STYLE_ATTR, SVGTYPE_ATTR}, Animation}}, system::base::node::user_setting::StyleDirtyList};
use pi_style::style::Time;

use super::GlobalDirtyMark;

// use super::StyleCommands;

/// 曲线管理器
pub struct CurveMgr {
    list: Vec<AnimationType>,
}

impl AnimationContextMgr for CurveMgr {
    fn remove_curve(&mut self, info: &AnimationInfo) {
        let ctx = &mut self.list[info.ty];
        (ctx.remove_animation)(&mut ctx.context, info);
    }
}

/// 帧动画表，css帧动画配置被存储在动画表中
pub struct KeyFramesSheet {
    animation_attr_types: CurveMgr, // Vec<TypeAnimationContext<T>>,

    static_key_frames_map: XHashMap<(usize, Atom), KeyFrames>, // 永不释放的帧动画列表
	key_frames_attr_map: XHashMap<(usize, Atom),  XHashMap<NotNan<f32>, VecDeque<Attribute>>>, // 用于缓存帧动画的元数据（未变成曲线之前），方便高层能取到帧动画值
    key_frames_map: XHashMap<(usize, Atom), KeyFrameAttr>,       // 帧动画列表, key为（作用域hash，动画名称）
    // curve_infos: FrameCurveInfoManager,
    type_use_mark: StyleMarkType, // 标记被使用的TypeAnimationContext，加速run（只有被使用到的TypeAnimationContext才会被调用run方法）

    runtime_info_map: RuntimeInfoMap<ObjKey>,
    animation_context_amount: AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,

    animation_bind: SecondaryMap<ObjKey, SmallVec<[AnimationGroupID; 1]>>, // 描述节点上绑定了什么动画

	transition_bind: SecondaryMap<ObjKey, SmallVec<[(AnimationGroupID, usize/*property*/); 1]>>, // 描述节点上绑定了什么transition

    group_bind: SecondaryMap<AnimationGroupID, (ObjKey, GroupType)>,   // 描述group对应的节点， 以及group的名称

    temp_keyframes_ptr: VecMap<Share<dyn Any + Send + Sync>>, // 临时帧动画指针（添加帧动画时用到）
    temp_keyframes_mark: StyleMarkType,                              // 临时帧动画标记，表示哪些属性存在曲线（加帧动画时用到）

    // animation_events: Vec<(AnimationGroupID, EAnimationEvent, u32)>,
    // animation_events_callback: Option<Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>)>>, // 动画事件回调函数
    temp_keyframnames: XHashMap<ObjKey, Vec<(usize, Atom, KeyFrames)>>, // 记录某节点添加了哪些运行时动画
    temp_errs: Vec<KeyFrameError>,

    // run_count: usize,
}

#[derive(Clone)]
pub struct KeyFrameAttr {
	pub data: Vec<(Share<dyn Any + Send + Sync>, usize)>,
	pub property_mark: StyleMarkType,
}

#[derive(Debug, Clone)]
pub enum GroupType{
	Animation((usize, Atom)),
	Transition(usize),
}

unsafe impl Send for KeyFramesSheet {}
unsafe impl Sync for KeyFramesSheet {}

impl Default for KeyFramesSheet {
    fn default() -> Self {
        let mut b = RuntimeInfoMap::<ObjKey>::default();
        let animation_attr_types = vec![
            AnimationType::new::<BackgroundRepeatType>(&mut b),    // 0
            AnimationType::new::<FontStyleType>(&mut b),           // 1
            AnimationType::new::<FontWeightType>(&mut b),          // 2
            AnimationType::new::<FontSizeType>(&mut b),            // 3
            AnimationType::new::<FontFamilyType>(&mut b),          // 4
            AnimationType::new::<LetterSpacingType>(&mut b),       // 5
            AnimationType::new::<WordSpacingType>(&mut b),         // 6
            AnimationType::new::<LineHeightType>(&mut b),          // 7
            AnimationType::new::<TextIndentType>(&mut b),          // 8
            AnimationType::new::<WhiteSpaceType>(&mut b),          // 9
            AnimationType::new::<TextAlignType>(&mut b),           // 11
            AnimationType::new::<VerticalAlignType>(&mut b),       // 12
            AnimationType::new::<ColorType>(&mut b),               // 13
            AnimationType::new::<TextStrokeType>(&mut b),          // 14
            AnimationType::new::<TextShadowType>(&mut b),          // 15
            AnimationType::new::<BackgroundImageType>(&mut b),     // 16
            AnimationType::new::<BackgroundImageClipType>(&mut b), // 17
            AnimationType::new::<ObjectFitType>(&mut b),           // 18
            AnimationType::new::<BackgroundColorType>(&mut b),     // 19
            AnimationType::new::<BoxShadowType>(&mut b),           // 20
            AnimationType::new::<BorderImageType>(&mut b),         // 21
            AnimationType::new::<BorderImageClipType>(&mut b),     // 22
            AnimationType::new::<BorderImageSliceType>(&mut b),    // 23
            AnimationType::new::<BorderImageRepeatType>(&mut b),   // 24
            AnimationType::new::<BorderColorType>(&mut b),         // 23
            AnimationType::new::<HsiType>(&mut b),                 // 24
            AnimationType::new::<BlurType>(&mut b),                // 26
            AnimationType::new::<MaskImageType>(&mut b),           // 27
            AnimationType::new::<MaskImageClipType>(&mut b),       // 28
            AnimationType::new::<TransformType>(&mut b),           // 29
            AnimationType::new::<TransformOriginType>(&mut b),     // 30
            AnimationType::new::<TransformWillChangeType>(&mut b), // 31
            AnimationType::new::<BorderRadiusType>(&mut b),        // 32
            AnimationType::new::<ZIndexType>(&mut b),              // 33
            AnimationType::new::<OverflowType>(&mut b),            // 34
            AnimationType::new::<BlendModeType>(&mut b),           // 35
            AnimationType::new::<DisplayType>(&mut b),             // 36
            AnimationType::new::<VisibilityType>(&mut b),          // 37
            AnimationType::new::<EnableType>(&mut b),              // 38
            AnimationType::new::<WidthType>(&mut b),               // 39
            AnimationType::new::<HeightType>(&mut b),              // 40
            AnimationType::new::<MarginTopType>(&mut b),           // 41
            AnimationType::new::<MarginRightType>(&mut b),         // 42
            AnimationType::new::<MarginBottomType>(&mut b),        // 43
            AnimationType::new::<MarginLeftType>(&mut b),          // 44
            AnimationType::new::<PaddingTopType>(&mut b),          // 45
            AnimationType::new::<PaddingRightType>(&mut b),        // 46
            AnimationType::new::<PaddingBottomType>(&mut b),       // 47
            AnimationType::new::<PaddingLeftType>(&mut b),         // 48
            AnimationType::new::<BorderTopType>(&mut b),           // 49
            AnimationType::new::<BorderRightType>(&mut b),         // 50
            AnimationType::new::<BorderBottomType>(&mut b),        // 51
            AnimationType::new::<BorderLeftType>(&mut b),          // 52
            AnimationType::new::<PositionTopType>(&mut b),         // 53
            AnimationType::new::<PositionRightType>(&mut b),       // 54
            AnimationType::new::<PositionBottomType>(&mut b),      // 55
            AnimationType::new::<PositionLeftType>(&mut b),        // 56
            AnimationType::new::<MinWidthType>(&mut b),            // 57
            AnimationType::new::<MinHeightType>(&mut b),           // 58
            AnimationType::new::<MaxHeightType>(&mut b),           // 59
            AnimationType::new::<MaxWidthType>(&mut b),            // 60
            AnimationType::new::<DirectionType>(&mut b),           // 61
            AnimationType::new::<FlexDirectionType>(&mut b),       // 62
            AnimationType::new::<FlexWrapType>(&mut b),            // 63
            AnimationType::new::<JustifyContentType>(&mut b),      // 64
            AnimationType::new::<AlignContentType>(&mut b),        // 65
            AnimationType::new::<AlignItemsType>(&mut b),          // 66
            AnimationType::new::<PositionTypeType>(&mut b),        // 67
            AnimationType::new::<AlignSelfType>(&mut b),           // 68
            AnimationType::new::<FlexShrinkType>(&mut b),          // 69
            AnimationType::new::<FlexGrowType>(&mut b),            // 70
            AnimationType::new::<AspectRatioType>(&mut b),         // 71
            AnimationType::new::<OrderType>(&mut b),               // 72
            AnimationType::new::<FlexBasisType>(&mut b),           // 73
            AnimationType::new::<OpacityType>(&mut b),             // 74
            AnimationType::new::<TextContentType>(&mut b),         // 75
            AnimationType::new::<VNodeType>(&mut b),               // 76
            AnimationType::new::<EmptyType>(&mut b),       // 占位77
            AnimationType::new::<EmptyType>(&mut b),       // 占位78
            AnimationType::new::<EmptyType>(&mut b),       //  占位79
            AnimationType::new::<EmptyType>(&mut b),       // 占位80
            AnimationType::new::<EmptyType>(&mut b),       // 占位81
            AnimationType::new::<EmptyType>(&mut b),       //  占位82
            AnimationType::new::<EmptyType>(&mut b),       // 占位83
            AnimationType::new::<EmptyType>(&mut b),       //  占位 84
            AnimationType::new::<ClipPathType>(&mut b),            // 85
            AnimationType::new::<TranslateType>(&mut b),           // 86
            AnimationType::new::<ScaleType>(&mut b),               // 87
            AnimationType::new::<RotateType>(&mut b),              // 88

			AnimationType::new::<EmptyType>(&mut b),       // 占位89
            AnimationType::new::<EmptyType>(&mut b),       // 占位80
            AnimationType::new::<EmptyType>(&mut b),       //  占位91
            AnimationType::new::<EmptyType>(&mut b),       // 占位92
            AnimationType::new::<EmptyType>(&mut b),       //  占位 93
			AnimationType::new::<EmptyType>(&mut b),       //  占位 94
			AnimationType::new::<EmptyType>(&mut b),       //  占位 95
        ];
        Self {
            animation_attr_types: CurveMgr { list: animation_attr_types },
            static_key_frames_map: Default::default(),
            key_frames_map: Default::default(),
			key_frames_attr_map: Default::default(),
            animation_bind: SecondaryMap::with_capacity(0),
			transition_bind: SecondaryMap::with_capacity(0),
            group_bind: SecondaryMap::with_capacity(0),
            animation_context_amount: AnimationContextAmount::default(AnimationGroupManagerDefault::default()),
            // curve_infos: c,
            type_use_mark: BitArray::default(),
            runtime_info_map: b,
            temp_keyframes_ptr: Default::default(),
            temp_keyframes_mark: BitArray::default(),
            temp_keyframnames: XHashMap::default(),
            // run_count: 0,
            temp_errs: Vec::default(),
            // animation_events:  Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransitionData {
	pub start: Option<Attribute>,
	pub end: Option<Attribute>,
	pub property: usize,
}


#[derive(Debug, Clone)]
pub enum KeyFrameError {
    NotExistFrameData(ObjKey, Animation),
}

impl KeyFramesSheet {
    // 推动动画
    pub fn run(&mut self, style_commands: &mut AnimationStyle, delta_ms: u64) {
        // self.run_count += 1;
        self.runtime_info_map.reset();
        self.animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_info_map);
        for i in self.type_use_mark.iter_ones() {
            let ty = &self.animation_attr_types.list[i];
            (ty.run)(&ty.context, &self.runtime_info_map, style_commands);
        }

        // // 通知动画监听器
        // if self.animation_context_amount.group_events.len() > 0 {
        // 	r(&self.animation_context_amount.group_events, &self.group_bind);
        // }
        // if let Some(r) = &self.animation_events_callback {
        //     if self.animation_context_amount.group_events.len() > 0 {
        //         r(&self.animation_context_amount.group_events, &self.group_bind);
        //     }
        // }
    }

    pub fn get_animation_events(&self) -> &Vec<(AnimationGroupID, EAnimationEvent, u32)> {
		// log::debug!("get_animation_events======================");
        // if self.run_count > 1 {
        // 	log::warn!("get_animation_events fail====={:?}", self.run_count);
        // }
        // unsafe {&mut *((self) as *const Self as usize as *mut Self)}.run_count = 0;
        // log::warn!("get_animation_events=====");
        &self.animation_context_amount.group_events
    }

    pub fn log(&self) {
        // self.animation_context_amount.log_groups();
    }

    pub fn get_group_bind(&self) -> &SecondaryMap<AnimationGroupID, (ObjKey, GroupType)> { &self.group_bind }

    /// 设置事件监听回调
    // pub fn set_event_listener(
    //     &mut self,
    //     callback: Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>)>,
    // ) {
    //     // self.animation_events_callback = Some(callback);
    // }
	pub fn get_keyframes(&self, name: Atom, scope_hash: usize) -> Option<&XHashMap<NotNan<f32>, VecDeque<Attribute>>> {
		let key = (scope_hash, name.clone());
		let key_frame = match self.key_frames_attr_map.get(&key) {
			Some(r) => r,
			None => {
				let key = (0, name);
				// 取全局动画
				match self.key_frames_attr_map.get(&key) {
					Some(r) => r,
					None => {
						return None;
					}
				}
			}
		};
		Some(key_frame)
	} 

	/// 绑定transition
	pub fn bind_trasition(
		&mut self, 
		target: ObjKey, 
		property: usize, 
		duration: Time, 
		delay: Time, 
		timing_function: &AnimationTimingFunction,
		data: &TransitionData
	) -> Result<(), Vec<KeyFrameError>> {
        log::debug!("bind_trasition=====target={:?}, property={:?}, duration={:?}, delay={:?}, timing_function={:?}, data={:?}", target, property, duration, delay, timing_function, data);

		// 如果当前运行动画正在支配该属性， 则不需要添加对该属性的transition
		if let Some(animation_bind) = self.animation_bind.get(target) {
			for i in animation_bind.iter() {
				if let GroupType::Animation(name) = &self.group_bind[*i].1 {
					if let Some(animation) = self.key_frames_map.get(name) {
						if animation.property_mark.get(property).as_deref() == Some(&true) && 
							self.animation_context_amount.group_infos[*i].is_playing
						 {
							return Ok(())
						}
					}
				}
			}
		}

		// 移除当前属性的旧的transition
		if let Some(r) = self.transition_bind.get_mut(target) {
			for i in 0..r.len() {
				if r[i].1 == property {
					r.swap_remove(i);
					break;
				}
			}
		} else {
			self.transition_bind.insert(target, SmallVec::default());
		}

        // 创建新的属性曲线
        if let (Some(start), Some(end)) = (&data.start, &data.end) {
			self.add_progress(0.0, start);
			self.add_progress(1.0, end);
		} else {
			return Ok(());
		}
        
		let curve = self.temp_keyframes_ptr.remove(property).unwrap();
        self.temp_keyframes_mark.fill(false);

		let group0 = self.animation_context_amount.create_animation_group();
		let ctx = &mut self.animation_attr_types.list[property];
			// 向动画组添加 动画
			(ctx.add_target_animation)(
				&mut self.animation_context_amount,
				&mut ctx.context,
				curve.clone(),
				group0,
				target,
			)
			.unwrap();
		self.type_use_mark.set(property, true);

		self.group_bind.insert(group0, (target, GroupType::Transition(property)));

		let duration = *duration as f32 / 1000.0;
		let delay = *delay as f32;
		let _ = self.animation_context_amount
			.force_group_total_frames(group0, Some(FRAME_COUNT), FRAME_COUNT as FramePerSecond);
		self.animation_context_amount
			.start_complete(
				group0,
				duration,
				ELoopMode::Positive(Some(1)),
				120,
				match timing_function {
					AnimationTimingFunction::Linear => AnimationAmountCalc::from_easing(EEasingMode::None),
					AnimationTimingFunction::Ease(r) => AnimationAmountCalc::from_easing(*r),
					AnimationTimingFunction::Step(step, mode) => AnimationAmountCalc::from_steps(*step as u16, *mode),
					AnimationTimingFunction::CubicBezier(x1, y1, x2, y2) => AnimationAmountCalc::from_cubic_bezier(*x1, *y1, *x2, *y2),
				},
				delay,
				EFillMode::FORWARDS,
			)
			.unwrap();
		
        self.transition_bind[target].push((group0, property));

        if self.temp_errs.len() > 0 {
            return Err(replace(&mut self.temp_errs, Vec::default()));
        }
		Ok(())
    }

    // 将动画绑定到目标上（目标即节点的实体id）
    pub fn bind_static_animation(&mut self, target: ObjKey, animation: &Animation) -> Result<(), Vec<KeyFrameError>> {
        log::debug!("bind_static_animation====={:?}, {:p}, {:?}", target, animation, animation);
        // 先解绑节点上的动画
        self.unbind_animation_all(target);
        // 再绑定新的动画
        self.bind_animation(target, animation)
    }

	

    /// 绑定运行时动画
    /// 运行时动画不会放入static_key_frames_map中，当其不在被引用时， 会被销毁
    pub fn add_runtime_keyframes(
        &mut self,
        target: ObjKey,
        animation: &Animation,
        mut value: XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>,
    ) {
        // 移除旧的运行时帧动画
        self.remove_runtime_keyframs(target);

        let mut names = Vec::with_capacity(value.len());
        log::debug!(
            "bind_runtime_animation====={:?}， {:p}, animaton： {:?}, keyframes: {:?}",
            target,
            animation,
            animation,
            value
        );
        for name in animation.name.value.iter() {
            if let Some(m) = value.remove(name) {
                self.add_keyframes(animation.name.scope_hash, name.clone(), &m);
                names.push((
                    animation.name.scope_hash,
                    name.clone(),
                    self.key_frames_map.get(&(animation.name.scope_hash, name.clone())).unwrap().data.clone(),
                ));
            }
        }

        if names.len() > 0 {
            self.temp_keyframnames.insert(target, names);
        }
    }

    // // animation: &Animation,

    // pub fn bind_runtime_animation(&mut self, target: ObjKey, animation: &Animation, value: XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>){
    // 	// 先解绑节点上的动画
    // 	self.unbind_animation_all(target);
    //     log::warn!("bind_runtime_animation====={:?}， {:?}", target, animation);
    // 	for (name, map) in value.into_iter() {
    // 		self.add_keyframes( animation.name.scope_hash, name.clone(), map);
    // 		self.temp_keyframnames.push(name);
    // 	}

    // 	// 如果binding出错，移除刚创建的keyframes
    //     if let Err(_) = self.bind_animation(target, animation) {
    // 		for i in self.temp_keyframnames.drain(..) {
    // 			self.key_frames_map.remove(&(animation.name.scope_hash, i));
    // 		}
    // 	}
    // }

    // 解绑定动画
    pub fn unbind_animation_all(&mut self, target: ObjKey) {
        log::debug!("unbind_animation_all====={:?}", target);
        if let Some(r) = self.animation_bind.remove(target) {
            // 移除目标上绑定的所有动画
            for single_animation in r {
                Self::remove_animation(
                    &mut self.animation_context_amount,
                    &mut self.group_bind,
                    &mut self.animation_attr_types,
                    single_animation,
                );
            }
        }
    }

	// 解绑定过度动画
    pub fn unbind_transition_all(&mut self, target: ObjKey) {
        if let Some(r) = self.transition_bind.remove(target) {
			log::debug!("unbind_transition, target:{:?}", target);
			for i in r.into_iter() {
				self.animation_context_amount.remove_animation_group(i.0, &mut self.animation_attr_types);
			}
        }
    }

	// 解绑定过度动画中的单个属性
    pub fn unbind_transition_single(&mut self, property: usize, target: ObjKey) {
		// 移除当前属性的旧的transition
		if let Some(r) = self.transition_bind.get_mut(target) {
			for i in 0..r.len() {
				if r[i].1 == property {
					log::debug!("unbind_transition_single, target:{:?}, property: {:?}", target, property);
					let i = r.swap_remove(i);
					self.animation_context_amount.remove_animation_group(i.0, &mut self.animation_attr_types);
					break;
				}
			}
		}
    }

	

    // 移除运行时帧数据
    pub fn remove_runtime_keyframs(&mut self, target: ObjKey) {
        // 移除运行时动画帧数据
        if let Some(runtime_frames) = self.temp_keyframnames.remove(&target) {
            log::debug!("remove_runtime_keyframs, target:{:?}", target);
            for key in runtime_frames.into_iter() {
                {
                    let _i = key.2;
                }; // 在此处销毁KeyFrames
                if let Some(key_frame) = self.key_frames_map.get(&(key.0, key.1.clone())) {
                    if key_frame.data.len() == 0 {
                        return;
                    }
                    if Share::strong_count(&key_frame.data[0].0) == 1 {
                        self.key_frames_map.remove(&(key.0, key.1));
                    }
                }
            }
        }
    }

    // 解绑定动画
    pub fn unbind_animation_single(&mut self, target: ObjKey, scope_hash: usize, name: Atom) {
        if let Some(r) = self.animation_bind.get_mut(target) {
            // 移除目标上绑定的所有动画
            let mut i = 0;
            while i < r.len() {
                let single_animation = r[i];
                if let Some(group_bind) = self.group_bind.get(single_animation) {
					if let GroupType::Animation(group_bind) = &group_bind.1 {
						if group_bind.0 == scope_hash && group_bind.1 == name {
							Self::remove_animation(
								&mut self.animation_context_amount,
								&mut self.group_bind,
								&mut self.animation_attr_types,
								single_animation,
							);
							r.swap_remove(i);
							continue;
						}
					}
                }
                i += 1;
            }
        }
    }

    // 添加一个静态的帧动画
    // 该动画无法移除
    pub fn add_static_keyframes(&mut self, scope_hash: usize, name: Atom, value: XHashMap<NotNan<f32>, VecDeque<Attribute>>) {
        self.add_keyframes(scope_hash, name.clone(), &value);
		self.key_frames_attr_map.insert((scope_hash, name.clone()), value);
        self.static_key_frames_map
            .insert((scope_hash, name.clone()), self.key_frames_map.get(&(scope_hash, name)).unwrap().data.clone());
    }
	fn add_progress(
		&mut self,
		progress: f32,
		attr: &Attribute,
	) {
		let progress = (progress * FRAME_COUNT).round() as u16;
		fn add_progress<T: Attr + FrameDataValue>(
            progress: u16,
            value: &T,
            temp_keyframes_ptr: &mut VecMap<Share<dyn Any + Send + Sync>>,
            temp_keyframes_mark: &mut StyleMarkType,
        ) {
            let index = T::get_style_index() as usize;
            let ptr = match temp_keyframes_ptr.get_mut(index) {
                Some(r) => r,
                None => {
                    temp_keyframes_ptr.insert(index, Share::new(FrameCurve::<T>::curve_frame_values(FRAME_COUNT as u16)));
                    temp_keyframes_mark.set(index, true);
                    &mut temp_keyframes_ptr[index]
                }
            };
            let f = Share::downcast::<FrameCurve<T>>(ptr.clone()).unwrap();
            unsafe { &mut *(Share::as_ptr(&f) as usize as *mut FrameCurve<T>) }.curve_frame_values_frame(progress, value.clone());
        }

		match attr {
			Attribute::BackgroundRepeat(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FontStyle(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FontWeight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FontSize(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FontFamily(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::LetterSpacing(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::WordSpacing(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::LineHeight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TextIndent(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::WhiteSpace(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TextAlign(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::VerticalAlign(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Color(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TextStroke(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TextShadow(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BackgroundImage(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BackgroundImageClip(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::ObjectFit(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BackgroundColor(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BoxShadow(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderImage(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderImageClip(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderImageSlice(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderImageRepeat(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderColor(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Hsi(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Blur(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MaskImage(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MaskImageClip(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Transform(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TransformOrigin(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TransformWillChange(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderRadius(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::ZIndex(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Overflow(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BlendMode(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Display(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Visibility(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Enable(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Width(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Height(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MarginTop(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MarginRight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MarginBottom(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MarginLeft(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PaddingTop(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PaddingRight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PaddingBottom(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PaddingLeft(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderTop(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderRight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderBottom(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::BorderLeft(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PositionTop(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PositionRight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PositionBottom(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PositionLeft(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MinWidth(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MinHeight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MaxHeight(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::MaxWidth(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Direction(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FlexDirection(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FlexWrap(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::JustifyContent(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::AlignContent(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::AlignItems(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::PositionType(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::AlignSelf(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FlexShrink(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FlexGrow(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::AspectRatio(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Order(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::FlexBasis(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Opacity(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::TextContent(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::VNode(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Translate(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Scale(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::Rotate(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
			Attribute::ClipPath(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),

			Attribute::AnimationName(_) => (),
			Attribute::AnimationDuration(_) => (),
			Attribute::AnimationTimingFunction(_) => (),
			Attribute::AnimationDelay(_) => (),
			Attribute::AnimationIterationCount(_) => (),
			Attribute::AnimationDirection(_) => (),
			Attribute::AnimationFillMode(_) => (),
			Attribute::AnimationPlayState(_) => (),
			Attribute::AsImage(_) => (),
			Attribute::TextOverflow(_) => (),
			Attribute::OverflowWrap(_) => (),
			Attribute::TransitionProperty(_) => (),
			Attribute::TransitionDuration(_) => (),
			Attribute::TransitionTimingFunction(_) => (),
			Attribute::TransitionDelay(_) => (),
            Attribute::TextOuterGlow(_) => (),
            // Attribute::RowGap(_) => (),
            // Attribute::ColumnGap(_) => (),
            // Attribute::AutoReduce(_) => (),
		}

	}

    // 添加一个帧动画
    pub fn add_keyframes(&mut self, scope_hash: usize, name: Atom, value: &XHashMap<NotNan<f32>, VecDeque<Attribute>>) {
        log::debug!("add_keyframes, name: {:?}, scope_hash: {:?}", name, scope_hash);
        for (progress, attrs) in value.iter() {
            for attr in attrs.into_iter() {
                self.add_progress(**progress, attr);
            }
        }

        let mut key_frame = Vec::new();
        for i in self.temp_keyframes_mark.iter_ones() {
            let curve = self.temp_keyframes_ptr.remove(i).unwrap();
            // let curve_id = (ctx.create_animation)(&mut ctx.context, &mut self.curve_infos, curve);
            // let attr_animation_id = (ctx.create_animation)(&mut ctx.context, curve);
            key_frame.push((curve, i));
        }
        let mark = std::mem::take(&mut self.temp_keyframes_mark);

        // 记录KeyFrames
        self.key_frames_map.insert((scope_hash, name.clone()), KeyFrameAttr { data: key_frame, property_mark: mark });
    }

    pub fn set_play_state(&mut self, target: ObjKey, state: AnimationPlayState) {
        let groups = match self.animation_bind.get(target) {
            Some(r) => r,
            None => return,
        };

        for group in groups.iter() {
            let _ = match state {
                AnimationPlayState::Paused => self.animation_context_amount.pause(group.clone()),
                AnimationPlayState::Running => self.animation_context_amount.restart(group.clone()),
            };
        }
    }

    // 将动画绑定到目标上（目标即节点的实体id）
    fn bind_animation(&mut self, target: ObjKey, animation: &Animation) -> Result<(), Vec<KeyFrameError>> {
        log::debug!("bind_animation====={:?}, {:?}", target, animation);

        let mut groups = SmallVec::with_capacity(animation.name.value.len());
        // 然后重新将动画绑定上去
        for i in 0..animation.name.value.len() {
            let name = (animation.name.scope_hash, animation.name.value[i].clone());
            let curves = match self.key_frames_map.get(&name) {
                Some(r) => r,
                None => {
					let name = (0, animation.name.value[i].clone());
                    // 取全局动画
					match self.key_frames_map.get(&name) {
						Some(r) => r,
						None => {
							self.temp_errs.push(KeyFrameError::NotExistFrameData(target, animation.clone()));
                            // log::warn!("key_frames_map===:{:?}", self.key_frames_map.keys().collect::<Vec<_>>());
							continue;
						}
					}
                }
            };

            log::debug!("bind_animation, target: {:?}, animation: {:?}", target, animation);
            let group0 = self.animation_context_amount.create_animation_group();
            groups.push(group0);
            for (attr_animation, curve_id) in curves.data.iter() {
                let ctx = &mut self.animation_attr_types.list[*curve_id];
                // 向动画组添加 动画
                (ctx.add_target_animation)(
                    &mut self.animation_context_amount,
                    &mut ctx.context,
                    attr_animation.clone(),
                    group0,
                    target,
                )
                .unwrap();
                self.type_use_mark.set(*curve_id, true);
            }
            self.group_bind.insert(group0, (target, GroupType::Animation(name.clone())));

            // 启动动画组
            log::debug!(
                "start anim, target: {:?}, direction: {:?}, frame_per_second: {}, from: {}, to:  {}, duration: {}s",
				target,
                animation.direction,
                (FRAME_COUNT / (*Animation::get_attr(i, &animation.duration) as f32 / 1000.0)).round() as u16,
                0.0,
                FRAME_COUNT,
                *Animation::get_attr(i, &animation.duration) as f32 / 1000.0
            );
            let iter_count = *Animation::get_attr(i, &animation.iteration_count);
            let iter_count = if f32::is_infinite(iter_count) { None } else { Some(iter_count as u32) };
            let direction = Animation::get_attr(i, &animation.direction);
            let direction = match direction {
                AnimationDirection::Normal => ELoopMode::Positive(iter_count),
                AnimationDirection::Reverse => ELoopMode::OppositePly(iter_count),
                AnimationDirection::Alternate => ELoopMode::Opposite(iter_count),
                AnimationDirection::AlternateReverse => ELoopMode::OppositePly(iter_count),
            };

            let duration = *Animation::get_attr(i, &animation.duration) as f32 / 1000.0; // 单位s
            let delay = *Animation::get_attr(i, &animation.delay) as f32; // 单位ms
            let file_mode = Animation::get_attr(i, &animation.fill_mode);
            let timing_function = Animation::get_attr(i, &animation.timing_function);
            // let frame_per_second = (FRAME_COUNT / duration).round() as u16;
            // TODO
            // log::warn!("start_complete==========={:?}, {:?},{:?}, {:?}, {:?}, {:?},  ", animation.name, duration, direction, timing_function, file_mode, delay);
            let _ = self.animation_context_amount
                .force_group_total_frames(group0, Some(FRAME_COUNT), FRAME_COUNT as FramePerSecond);
            self.animation_context_amount
                .start_complete(
                    group0,
                    duration,
                    direction,
                    120,
                    match timing_function {
                        AnimationTimingFunction::Linear => AnimationAmountCalc::from_easing(EEasingMode::None),
                        AnimationTimingFunction::Ease(r) => AnimationAmountCalc::from_easing(r),
                        AnimationTimingFunction::Step(step, mode) => AnimationAmountCalc::from_steps(step as u16, mode),
                        AnimationTimingFunction::CubicBezier(x1, y1, x2, y2) => AnimationAmountCalc::from_cubic_bezier(x1, y1, x2, y2),
                    },
                    delay,
                    match file_mode {
                        pi_style::style::AnimationFillMode::None => EFillMode::NONE,
                        pi_style::style::AnimationFillMode::Forwards => EFillMode::FORWARDS,
                        pi_style::style::AnimationFillMode::Backwards => EFillMode::BACKWARDS,
                        pi_style::style::AnimationFillMode::Both => EFillMode::BOTH,
                    },
                )
                .unwrap();
        }

        self.animation_bind.insert(target, groups);

        if self.temp_errs.len() > 0 {
            return Err(replace(&mut self.temp_errs, Vec::default()));
        }

        Ok(())
    }

    // 移除动画
    fn remove_animation(
        animation_context_amount: &mut AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,
        group_bind: &mut SecondaryMap<AnimationGroupID, (ObjKey, GroupType)>,
        animation_attr_types: &mut CurveMgr,
        single_animation: DefaultKey,
    ) {
        log::debug!("remove_animation, name: {:?}", group_bind.get(single_animation));
        animation_context_amount.remove_animation_group(single_animation, animation_attr_types);
        group_bind.remove(single_animation);
    }
}

// #[derive(Debug, Clone, Deref, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub type ObjKey = Entity;

// impl Key for ObjKey {
//     fn data(&self) -> pi_slotmap::KeyData {
//         // (u64::from(self.version.get()) << 32) | u64::from(self.idx)

//         pi_slotmap::KeyData::from_ffi((u64::from(self.0.generation()) << 32) | u64::from(self.0.index()))
//     }

// 	fn index(&self) -> usize {
// 		self.0.index() as usize
// 	}

//     fn with(idx: usize) -> Self { Self(Entity::from_raw(idx as u32)) }
// }

// impl Null for ObjKey {
// 	fn null() -> Self { Self(Entity::from_bits(u64::null())) }

//     fn is_null(&self) -> bool { self.0.to_bits().is_null() }
// }

// impl From<pi_slotmap::KeyData> for ObjKey {
//     fn from(value: pi_slotmap::KeyData) -> Self { Self(Entity::from_bits(value.as_ffi())) }
// }

// impl Default for ObjKey {
//     fn default() -> Self { Self(Entity::from_bits(u64::null())) }
// }

type KeyFrames = Vec<(Share<dyn Any + Send + Sync>, usize)>; // Vec<(动画曲线， 曲线类型)>


pub struct CurveId {
    pub ty: usize,
    pub id: usize,
}

pub struct TypeAnimationMgr<F: FrameDataValue> {
    context: TypeAnimationContext<F, Share<FrameCurve<F>>>,
}

impl AnimationType {
    fn new<F: AnimationTypeInterface + Attr + FrameDataValue>(runtime_info_map: &mut RuntimeInfoMap<ObjKey>) -> Self {
        Self {
            context: Box::new(TypeAnimationMgr {
                context: TypeAnimationContext::<F, Share<FrameCurve<F>>>::new(F::get_style_index() as usize, runtime_info_map),
            }),
            run: F::run,
            // create_animation: F::create_animation,
            remove_animation: F::remove_animation,
            add_target_animation: F::add_target_animation,
        }
    }
}

pub struct AnimationType {
    context: Box<dyn Any>, // TypeAnimationContext<T>
    run: fn(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut AnimationStyle),
    // create_animation: fn(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo,
    remove_animation: fn(context: &mut Box<dyn Any>, info: &AnimationInfo),
    add_target_animation: fn(
        s: &mut AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,
        context: &mut Box<dyn Any>,
        // type_ctx: &mut TypeAnimationContext<F, D>,
        curve: Share<dyn Any + Send + Sync>,
        group_id: AnimationGroupID,
        target: ObjKey,
    ) -> Result<(), pi_animation::error::EAnimationError>,
}

trait AnimationTypeInterface {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut AnimationStyle);
    // fn add_frame_curve(context: &mut Box<dyn Any>, curve_infos: &mut FrameCurveInfoManager, curve_ptr: usize) -> CurveId;
    // fn create_animation(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo;
    fn remove_animation(context: &mut Box<dyn Any>, info: &AnimationInfo);
    fn add_target_animation(
        s: &mut AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,
        context: &mut Box<dyn Any>,
        // type_ctx: &mut TypeAnimationContext<F, D>,
        curve: Share<dyn Any + Send + Sync>,
        group_id: AnimationGroupID,
        target: ObjKey,
    ) -> Result<(), pi_animation::error::EAnimationError>;
}

impl<T: AttrSet + FrameDataValue> AnimationTypeInterface for T {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut AnimationStyle) {
        if let Err(e) = context
            .downcast_ref::<TypeAnimationMgr<Self>>()
            .unwrap()
            .context
            .anime(runtime_infos, style_commands)
        {
            log::error!("{:?}", e);
        }
    }

    // fn create_animation(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo {
    //     let curve = unsafe { std::ptr::read(curve_ptr as *const FrameCurve<Self>) };
    //     out_any!(debug, "add_frame_curve, curve: {:?}", &curve);
    //     let mgr = context.downcast_mut::<TypeAnimationMgr<Self>>().unwrap();
    //     // mgr.curves.push(curve);
    //     // mgr.context.create_animation(T::get_style_index(), curve)
    //     // CurveId {
    //     //     ty: T::get_style_index() as usize,
    //     //     id: curves.len() - 1,
    //     // }
    // }

    fn remove_animation(context: &mut Box<dyn Any>, info: &AnimationInfo) {
        //     out_any!(debug, "remove_frame_curve, curve: {:?}", &animation_info);
        let mgr = context.downcast_mut::<TypeAnimationMgr<Self>>().unwrap();
        //     // mgr.curves.push(curve);
        mgr.context.remove_one(info)
    }

    fn add_target_animation(
        s: &mut AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,
        context: &mut Box<dyn Any>,
        // type_ctx: &mut TypeAnimationContext<F, D>,
        curve: Share<dyn Any + Send + Sync>,
        group_id: AnimationGroupID,
        target: ObjKey,
    ) -> Result<(), pi_animation::error::EAnimationError> {
        let curve = Share::downcast::<FrameCurve<Self>>(curve).unwrap();
        let mgr = context.downcast_mut::<TypeAnimationMgr<Self>>().unwrap();
        s.add_target_animation(&mut mgr.context, curve, group_id, target)
        // todo!()
    }
}

// impl<F: AttrSet + FrameDataValue> TypeAnimationResultPool<F, ObjKey> for StyleCommands {
//     fn record_target(&mut self, _id_target: ObjKey) {
//         // todo!()
//     }

//     fn record_result(
//         &mut self,
//         entity: ObjKey,
//         _id_attr: pi_animation::target_modifier::IDAnimatableAttr,
//         result: pi_animation::animation_result_pool::AnimeResult<F>,
//     ) -> Result<(), pi_animation::error::EAnimationError> {
//         out_any!(log::trace, "record animation result===={:?}, {:?}", &result.value, entity);
//         self.set_style(entity, result.value);
//         Ok(())
//     }
// }

pub struct AnimationStyle<'a, 'w, 's> {
    pub style_query: &'a mut Setting<'w>,
    pub dirty_list: &'a mut StyleDirtyList<'w, 's>,
    pub global_mark: &'a mut GlobalDirtyMark,
    pub components_ids: Vec<(ComponentIndex, bool)>,
}

impl<'a, 'w, 's, F: AttrSet + FrameDataValue> TypeAnimationResultPool<F, ObjKey> for AnimationStyle<'a, 'w, 's> {
    fn record_target(&mut self, _id_target: ObjKey) {
        // todo!()
    }

    fn record_result(
        &mut self,
        entity: ObjKey,
        _id_attr: pi_animation::target_modifier::IDAnimatableAttr,
        result: pi_animation::animation_result_pool::AnimeResult<F>,
    ) -> Result<(), pi_animation::error::EAnimationError> {
        out_any!(log::trace, "record animation result===={:?}, {:?}", &result.value, entity);
        let style_index = F::get_style_index();
        self.set_style(entity, style_index, &result.value as *const F as usize as *const u8);
        Ok(())
    }
}

impl<'a, 'w, 's> AnimationStyle<'a, 'w, 's> {
    pub fn set_style(&mut self, entity: Entity, style_index: u16, ptr: *const u8) {
        let mut style_mark = if let Ok(style_mark) = self.style_query.world.get_component_mut_by_index::<StyleMark>(entity, self.style_query.style.style_mark) {
            style_mark
        } else {
            log::debug!("node is not exist: {:?}", entity);
            // 不存在实体，不处理
            return;
        };
        log::trace!("set animation style==========={:?}", entity);
        
        if style_index < STYLE_COUNT * 2 {
            if style_index >= GUI_STYLE_COUNT {
                log::debug!("style_index: {}", style_index);
                let index = (style_index - 97) as usize;
                (SVGTYPE_ATTR[index].push_component_ops)(&self.style_query.style, &mut self.components_ids);
                if self.components_ids.len() > 0 {
                    let _ = self.style_query.world.make_entity_editor().alter_components_by_index(entity, &self.components_ids);
                    self.components_ids.clear();
                }
                (SVGTYPE_ATTR[index].set)(ptr, &mut self.style_query, entity, true);
            } else {
                (STYLE_ATTR[(style_index) as usize].push_component_ops)(&self.style_query.style, &mut self.components_ids);
                if self.components_ids.len() > 0 {
                    let _ = self.style_query.world.make_entity_editor().alter_components_by_index(entity, &self.components_ids);
                    self.components_ids.clear();
                }
                (STYLE_ATTR[style_index as usize].set)(ptr, &mut self.style_query, entity, true);
            }
            style_mark.local_style.set(style_index as usize, true);
            style_mark.dirty_style.set(style_index as usize, true);
            self.global_mark.mark.set(style_index as usize, true);
        } 

        self.dirty_list.mark_dirty(entity);
    }
}



const FRAME_COUNT: f32 = 10000.0;

// #[test]
// fn test() {
// 	fn aa<A: AsRef<usize>>(a: A) {

// 	}
// 	aa(std::sync::Arc::new(0))
// }
