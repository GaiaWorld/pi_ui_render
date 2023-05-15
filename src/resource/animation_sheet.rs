use std::{any::Any, collections::VecDeque};

use bevy::ecs::prelude::{Entity, Resource};
use bitvec::vec::BitVec;
use log::debug;
use ordered_float::NotNan;
use pi_animation::{
    amount::AnimationAmountCalc,
    type_animation_context::{AnimationContextAmount, TypeAnimationContext},
    animation_group::AnimationGroupID,
    animation_group_manager::AnimationGroupManagerDefault,
    animation_listener::EAnimationEvent,
    animation_result_pool::TypeAnimationResultPool,
    loop_mode::ELoopMode,
    runtime_info::RuntimeInfoMap, animation::AnimationInfo,
};
use pi_atom::Atom;
use pi_curves::{
    curve::{frame::FrameDataValue, frame_curve::FrameCurve},
    easing::EEasingMode,
};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_null::Null;
use pi_print_any::out_any;
use pi_slotmap::{Key, SecondaryMap};
use pi_style::style::{AnimationDirection, AnimationTimingFunction};
use pi_style::{style_parse::Attribute, style_type::*};
use smallvec::SmallVec;

use crate::components::user::{serialize::StyleAttr, Animation};

use super::StyleCommands;

#[derive(Debug, Clone, Deref, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct ObjKey(pub Entity);

unsafe impl Key for ObjKey {
    fn data(&self) -> pi_slotmap::KeyData {
        // (u64::from(self.version.get()) << 32) | u64::from(self.idx)

        pi_slotmap::KeyData::from_ffi((u64::from(self.0.generation()) << 32) | u64::from(self.0.index()))
    }
}

impl From<pi_slotmap::KeyData> for ObjKey {
    fn from(value: pi_slotmap::KeyData) -> Self { Self(Entity::from_bits(value.as_ffi())) }
}

impl Default for ObjKey {
    fn default() -> Self { Self(Entity::from_bits(u64::null())) }
}

#[derive(Resource)]
pub struct KeyFramesSheet {
    animation_attr_types: Vec<AnimationType>, // Vec<TypeAnimationContext<T>>,

    key_frames_map: XHashMap<(usize, Atom), KeyFrames>, // 帧动画列表, key为（作用域hash，动画名称）
    // curve_infos: FrameCurveInfoManager,
    type_use_mark: BitVec, // 标记被使用的TypeAnimationContext，加速run（只有被使用到的TypeAnimationContext才会被嗲用run方法）

    runtime_info_map: RuntimeInfoMap<ObjKey>,
    animation_context_amount: AnimationContextAmount<ObjKey, AnimationGroupManagerDefault<ObjKey>>,

    animation_bind: SecondaryMap<ObjKey, SmallVec<[AnimationGroupID; 1]>>, // 描述节点上绑定了什么动画
    group_bind: SecondaryMap<AnimationGroupID, (ObjKey, Atom)>,            // 描述group对应的节点， 以及group的名称

    temp_keyframes_ptr: VecMap<usize>, // 临时帧动画指针（添加帧动画时用到）
    temp_keyframes_mark: BitVec,       // 临时帧动画标记，表示哪些属性存在曲线（加帧动画时用到）

	// animation_events: Vec<(AnimationGroupID, EAnimationEvent, u32)>,
    // animation_events_callback: Option<Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>)>>, // 动画事件回调函数
}

unsafe impl Send for KeyFramesSheet {}
unsafe impl Sync for KeyFramesSheet {}

impl Default for KeyFramesSheet {
    fn default() -> Self {
        let mut b = RuntimeInfoMap::<ObjKey>::default();
        let animation_attr_types = vec![
            AnimationType::new::<PaddingTopType>(&mut b),       // 占位
            AnimationType::new::<BackgroundRepeatType>(&mut b), // 占位
            AnimationType::new::<FontStyleType>(&mut b),
            AnimationType::new::<FontWeightType>(&mut b),
            AnimationType::new::<FontSizeType>(&mut b),
            AnimationType::new::<FontFamilyType>(&mut b),
            AnimationType::new::<LetterSpacingType>(&mut b),
            AnimationType::new::<WordSpacingType>(&mut b),
            AnimationType::new::<LineHeightType>(&mut b),
            AnimationType::new::<TextIndentType>(&mut b),
            AnimationType::new::<WhiteSpaceType>(&mut b),
            AnimationType::new::<TextAlignType>(&mut b),
            AnimationType::new::<VerticalAlignType>(&mut b),
            AnimationType::new::<ColorType>(&mut b),
            AnimationType::new::<TextStrokeType>(&mut b),
            AnimationType::new::<TextShadowType>(&mut b),
            AnimationType::new::<BackgroundImageType>(&mut b),
            AnimationType::new::<BackgroundImageClipType>(&mut b),
            AnimationType::new::<ObjectFitType>(&mut b),
            AnimationType::new::<BackgroundColorType>(&mut b),
            AnimationType::new::<BoxShadowType>(&mut b),
            AnimationType::new::<BorderImageType>(&mut b),
            AnimationType::new::<BorderImageClipType>(&mut b),
            AnimationType::new::<BorderImageSliceType>(&mut b),
            AnimationType::new::<BorderImageRepeatType>(&mut b),
            AnimationType::new::<BorderColorType>(&mut b),
            AnimationType::new::<HsiType>(&mut b),
            AnimationType::new::<BlurType>(&mut b),
            AnimationType::new::<MaskImageType>(&mut b),
            AnimationType::new::<MaskImageClipType>(&mut b),
            AnimationType::new::<TransformType>(&mut b),
            AnimationType::new::<TransformOriginType>(&mut b),
            AnimationType::new::<TransformWillChangeType>(&mut b),
            AnimationType::new::<BorderRadiusType>(&mut b),
            AnimationType::new::<ZIndexType>(&mut b),
            AnimationType::new::<OverflowType>(&mut b),
            AnimationType::new::<BlendModeType>(&mut b),
            AnimationType::new::<DisplayType>(&mut b),
            AnimationType::new::<VisibilityType>(&mut b),
            AnimationType::new::<EnableType>(&mut b),
            AnimationType::new::<WidthType>(&mut b),
            AnimationType::new::<HeightType>(&mut b),
            AnimationType::new::<MarginTopType>(&mut b),
            AnimationType::new::<MarginRightType>(&mut b),
            AnimationType::new::<MarginBottomType>(&mut b),
            AnimationType::new::<MarginLeftType>(&mut b),
            AnimationType::new::<PaddingTopType>(&mut b),
            AnimationType::new::<PaddingRightType>(&mut b),
            AnimationType::new::<PaddingBottomType>(&mut b),
            AnimationType::new::<PaddingLeftType>(&mut b),
            AnimationType::new::<BorderTopType>(&mut b),
            AnimationType::new::<BorderRightType>(&mut b),
            AnimationType::new::<BorderBottomType>(&mut b),
            AnimationType::new::<BorderLeftType>(&mut b),
            AnimationType::new::<PositionTopType>(&mut b),
            AnimationType::new::<PositionRightType>(&mut b),
            AnimationType::new::<PositionBottomType>(&mut b),
            AnimationType::new::<PositionLeftType>(&mut b),
            AnimationType::new::<MinWidthType>(&mut b),
            AnimationType::new::<MinHeightType>(&mut b),
            AnimationType::new::<MaxHeightType>(&mut b),
            AnimationType::new::<MaxWidthType>(&mut b),
            AnimationType::new::<DirectionType>(&mut b),
            AnimationType::new::<FlexDirectionType>(&mut b),
            AnimationType::new::<FlexWrapType>(&mut b),
            AnimationType::new::<JustifyContentType>(&mut b),
            AnimationType::new::<AlignContentType>(&mut b),
            AnimationType::new::<AlignItemsType>(&mut b),
            AnimationType::new::<PositionTypeType>(&mut b),
            AnimationType::new::<AlignSelfType>(&mut b),
            AnimationType::new::<FlexShrinkType>(&mut b),
            AnimationType::new::<FlexGrowType>(&mut b),
            AnimationType::new::<AspectRatioType>(&mut b),
            AnimationType::new::<OrderType>(&mut b),
            AnimationType::new::<FlexBasisType>(&mut b),
            AnimationType::new::<OpacityType>(&mut b),
            AnimationType::new::<TextContentType>(&mut b),
            AnimationType::new::<VNodeType>(&mut b),
            AnimationType::new::<TransformFuncType>(&mut b),
        ];
        let mut temp_keyframes_mark = BitVec::with_capacity(animation_attr_types.len());
        unsafe { temp_keyframes_mark.set_len(animation_attr_types.len()) };
        temp_keyframes_mark.fill(false);
        Self {
            animation_attr_types,
            key_frames_map: Default::default(),
            animation_bind: SecondaryMap::with_capacity(0),
            group_bind: SecondaryMap::with_capacity(0),
            animation_context_amount: AnimationContextAmount::default(AnimationGroupManagerDefault::default()),
            // curve_infos: c,
            type_use_mark: temp_keyframes_mark.clone(),
            runtime_info_map: b,
            temp_keyframes_ptr: Default::default(),
            temp_keyframes_mark: temp_keyframes_mark,
            // animation_events:  Vec::new(),
        }
    }
}


#[derive(Debug, Clone)]
pub enum KeyFrameError {
    InvalidKeyFrameName(Atom),
}

impl KeyFramesSheet {
    // 推动动画
    pub fn run(&mut self, style_commands: &mut StyleCommands, delta_ms: u64) {
        self.runtime_info_map.reset();
        self.animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_info_map);
        for i in self.type_use_mark.iter_ones() {
            let ty = &self.animation_attr_types[i];
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

	pub fn get_animation_events(
		&self,
	) -> &Vec<(AnimationGroupID, EAnimationEvent, u32)> {
		&self.animation_context_amount.group_events
	}

	pub fn log(
		&self,
	) {
		// self.animation_context_amount.log_groups();
	}

	pub fn get_group_bind(
		&self,
	) -> &SecondaryMap<AnimationGroupID, (ObjKey, Atom)> {
		&self.group_bind
	}

    /// 设置事件监听回调
    // pub fn set_event_listener(
    //     &mut self,
    //     callback: Share<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (ObjKey, Atom)>)>,
    // ) {
    //     // self.animation_events_callback = Some(callback);
    // }

    // 将动画绑定到目标上
    pub fn bind_animation(&mut self, target: ObjKey, animation: &Animation) -> Result<(), KeyFrameError> {
        log::debug!("bind_animation====={:?}", animation);
        // 先解绑节点上的动画
        self.unbind_animation(target);

        let mut err = None;
        let mut groups = SmallVec::with_capacity(animation.name.value.len());
        // 然后重新将动画绑定上去
        for i in 0..animation.name.value.len() {
            let name = (animation.name.scope_hash, animation.name.value[i].clone());
            let curves = match self.key_frames_map.get(&name) {
                Some(r) => r,
                None => {
                    err = Some(KeyFrameError::InvalidKeyFrameName(name.1.clone()));
                    break;
                }
            };

            debug!("bind_animation, target: {:?}, animation: {:?}", target, animation);
            let group0 = self.animation_context_amount.create_animation_group();
            groups.push(group0);
            for (attr_animation, curve_id) in curves.0.iter() {
                // 向动画组添加 动画
                self.animation_context_amount
                    .add_target_animation(attr_animation.clone(), group0, target)
                    .unwrap();
                self.type_use_mark.set(*curve_id, true);
            }
            self.group_bind.insert(group0, (target, name.1.clone()));

            // 启动动画组
            debug!(
                "start anim, direction: {:?}, frame_per_second: {}, from: {}, to:  {}, duration: {}s",
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
            let duration = *Animation::get_attr(i, &animation.duration) as f32 / 1000.0;
            let timing_function = Animation::get_attr(i, &animation.timing_function);
			// let frame_per_second = (FRAME_COUNT / duration).round() as u16;
            // TODO
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
                )
                .unwrap();
        }

        self.animation_bind.insert(target, groups);
        if let Some(err) = err {
            self.unbind_animation(target);
            return Err(err);
        }

        Ok(())
    }

    // 解绑定动画
    pub fn unbind_animation(&mut self, target: ObjKey) {
        if let Some(r) = self.animation_bind.remove(target) {
            // 移除目标上绑定的所有动画
            for single_animation in r {
                debug!("unbind_animation, name: {:?}", self.group_bind.get(single_animation));
                self.animation_context_amount.del_animation_group(single_animation);
                self.group_bind.remove(single_animation);
            }
        }
    }

    // 添加一个帧动画
    pub fn add_keyframes(&mut self, scope_hash: usize, name: Atom, value: XHashMap<NotNan<f32>, VecDeque<Attribute>>) {
        debug!("add_keyframes, name: {:?}, scope_hash: {:?}", name, scope_hash);
        fn add_progress<T: Attr + FrameDataValue>(progress: u16, value: T, temp_keyframes_ptr: &mut VecMap<usize>, temp_keyframes_mark: &mut BitVec) {
            let index = T::get_style_index() as usize;
            let ptr = match temp_keyframes_ptr.get_mut(index) {
                Some(r) => r,
                None => {
                    temp_keyframes_ptr.insert(
                        index,
                        Box::into_raw(Box::new(FrameCurve::<T>::curve_frame_values(FRAME_COUNT as u16))) as usize,
                    );
                    temp_keyframes_mark.set(index, true);
                    &mut temp_keyframes_ptr[index]
                }
            };
            unsafe { &mut *(*ptr as *mut FrameCurve<T>) }.curve_frame_values_frame(progress, value);
        }

        for (progress, attrs) in value.into_iter() {
            let progress = (progress * FRAME_COUNT).round() as u16;
            for attr in attrs.into_iter() {
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
                    Attribute::TransformFunc(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::AnimationName(_) => (),
                    Attribute::AnimationDuration(_) => (),
                    Attribute::AnimationTimingFunction(_) => (),
                    Attribute::AnimationDelay(_) => (),
                    Attribute::AnimationIterationCount(_) => (),
                    Attribute::AnimationDirection(_) => (),
                    Attribute::AnimationFillMode(_) => (),
                    Attribute::AnimationPlayState(_) => (),
                    Attribute::ClipPath(_) => (),
                }
            }
        }

        let mut key_frame = KeyFrames(Vec::new());
        for i in self.temp_keyframes_mark.iter_ones() {
            let curve = self.temp_keyframes_ptr.remove(i).unwrap();
            let ctx = &mut self.animation_attr_types[i];
            // let curve_id = (ctx.create_animation)(&mut ctx.context, &mut self.curve_infos, curve);
            let attr_animation_id = (ctx.create_animation)(&mut ctx.context, curve);
            key_frame.0.push((attr_animation_id, i));
        }
        self.temp_keyframes_mark.fill(false);

        // 记录KeyFrames
        self.key_frames_map.insert((scope_hash, name.clone()), key_frame);
    }
}

pub struct KeyFrames(Vec<(AnimationInfo, usize)>); // (动画属性id， 曲线类型)

pub struct CurveId {
    pub ty: usize,
    pub id: usize,
}

pub struct TypeAnimationMgr<F: FrameDataValue> {
	context: TypeAnimationContext<F, FrameCurve<F>>,
}

impl AnimationType {
    fn new<F: AnimationTypeInterface + Attr + FrameDataValue>(
        runtime_info_map: &mut RuntimeInfoMap<ObjKey>,
    ) -> Self {
        Self {
            context: Box::new(TypeAnimationMgr{
				context: TypeAnimationContext::<F, FrameCurve<F>>::new(
					F::get_style_index() as usize,
					runtime_info_map,
				)
			}),
            run: F::run,
            create_animation: F::create_animation,
        }
    }
}

pub struct AnimationType {
    context: Box<dyn Any>, // TypeAnimationContext<T>
    run: fn(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut StyleCommands),
    create_animation: fn(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo,
}

trait AnimationTypeInterface {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut StyleCommands);
    // fn add_frame_curve(context: &mut Box<dyn Any>, curve_infos: &mut FrameCurveInfoManager, curve_ptr: usize) -> CurveId;
	fn create_animation(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo;
}

impl<T: Attr + FrameDataValue> AnimationTypeInterface for T {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<ObjKey>, style_commands: &mut StyleCommands) {
        if let Err(e) = context
            .downcast_ref::<TypeAnimationMgr<Self>>()
            .unwrap()
            .context.anime(runtime_infos, style_commands)
        {
            log::error!("{:?}", e);
        }
    }

    fn create_animation(context: &mut Box<dyn Any>, curve_ptr: usize) -> AnimationInfo {
        let curve = unsafe { std::ptr::read(curve_ptr as *const FrameCurve<Self>) };
        out_any!(debug, "add_frame_curve, curve: {:?}", &curve);
		let mgr = context
		.downcast_mut::<TypeAnimationMgr<Self>>()
		.unwrap();
		// mgr.curves.push(curve);
		mgr.context.create_animation(T::get_style_index(), curve)
        // CurveId {
        //     ty: T::get_style_index() as usize,
        //     id: curves.len() - 1,
        // }
    }
}

impl<F: Attr + FrameDataValue> TypeAnimationResultPool<F, ObjKey> for StyleCommands {
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
        let start = self.style_buffer.len();
        unsafe { StyleAttr::write(result.value, &mut self.style_buffer) };
        if let Some(r) = self.commands.last_mut() {
            if r.0 == entity.0 {
                r.2 = self.style_buffer.len();
                return Ok(());
            }
        }
        self.commands.push((entity.0, start, self.style_buffer.len()));
        Ok(())
    }
}


const FRAME_COUNT: f32 = 10000.0;
