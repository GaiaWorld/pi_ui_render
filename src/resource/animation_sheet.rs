use std::{any::Any, collections::VecDeque};

use bitvec::vec::BitVec;
use log::trace;
use ordered_float::NotNan;
use pi_animation::{
    amount::AnimationAmountCalc,
    animation::AnimationManagerDefault,
    animation_context::{AnimationContextAmount, TypeAnimationContext},
    animation_group::AnimationGroupID,
    animation_group_manager::AnimationGroupManagerDefault,
    animation_result_pool::TypeAnimationResultPool,
    frame_curve_manager::FrameCurveInfoManager,
    runtime_info::RuntimeInfoMap,
};
use pi_curves::{
    curve::{
        frame::{FrameDataValue, KeyFrameCurveValue},
        frame_curve::FrameCurve,
    },
    easing::EEasingMode,
};
use pi_ecs::{prelude::Id, storage::SecondaryMap};
use pi_hash::XHashMap;
use pi_map::{vecmap::VecMap, Map};
use pi_print_any::out_any;
use pi_style::{
    style::{Animation, TimingFunction},
    style_parse::Attribute,
    style_type::*,
};

use crate::components::user::Node;

use super::StyleCommands;

pub struct KeyFramesSheet {
    animation_attr_types: Vec<AnimationType>, // Vec<TypeAnimationContext<T>>,

    key_frames_map: XHashMap<usize, KeyFrames>, // 帧动画列表
    curve_infos: FrameCurveInfoManager,
    type_use_mark: BitVec, // 标记被使用的TypeAnimationContext，加速run（只有被使用到的TypeAnimationContext才会被嗲用run方法）

    runtime_info_map: RuntimeInfoMap<Id<Node>>,
    animation_context_amount: AnimationContextAmount<AnimationManagerDefault, Id<Node>, AnimationGroupManagerDefault<Id<Node>>>,

    animation_bind: SecondaryMap<Id<Node>, (AnimationGroupID, Animation)>, // 描述节点上绑定了什么动画

    temp_keyframes_ptr: VecMap<usize>, // 临时帧动画指针（添加帧动画时用到）
    temp_keyframes_mark: BitVec,       // 临时帧动画标记，表示哪些属性存在曲线（加帧动画时用到）
}

unsafe impl Send for KeyFramesSheet {}
unsafe impl Sync for KeyFramesSheet {}

impl Default for KeyFramesSheet {
    fn default() -> Self {
        let mut b = RuntimeInfoMap::<Id<Node>>::default();
        let mut c = FrameCurveInfoManager::default();
        let animation_attr_types = vec![
            AnimationType::new::<PaddingTopType>(&mut b, &mut c), // 占位
            AnimationType::new::<PaddingTopType>(&mut b, &mut c), // 占位
            AnimationType::new::<FontStyleType>(&mut b, &mut c),
            AnimationType::new::<FontWeightType>(&mut b, &mut c),
            AnimationType::new::<FontSizeType>(&mut b, &mut c),
            AnimationType::new::<FontFamilyType>(&mut b, &mut c),
            AnimationType::new::<LetterSpacingType>(&mut b, &mut c),
            AnimationType::new::<WordSpacingType>(&mut b, &mut c),
            AnimationType::new::<LineHeightType>(&mut b, &mut c),
            AnimationType::new::<TextIndentType>(&mut b, &mut c),
            AnimationType::new::<WhiteSpaceType>(&mut b, &mut c),
            AnimationType::new::<TextAlignType>(&mut b, &mut c),
            AnimationType::new::<VerticalAlignType>(&mut b, &mut c),
            AnimationType::new::<ColorType>(&mut b, &mut c),
            AnimationType::new::<TextStrokeType>(&mut b, &mut c),
            AnimationType::new::<TextShadowType>(&mut b, &mut c),
            AnimationType::new::<BackgroundImageType>(&mut b, &mut c),
            AnimationType::new::<BackgroundImageClipType>(&mut b, &mut c),
            AnimationType::new::<ObjectFitType>(&mut b, &mut c),
            AnimationType::new::<BackgroundColorType>(&mut b, &mut c),
            AnimationType::new::<BoxShadowType>(&mut b, &mut c),
            AnimationType::new::<BorderImageType>(&mut b, &mut c),
            AnimationType::new::<BorderImageClipType>(&mut b, &mut c),
            AnimationType::new::<BorderImageSliceType>(&mut b, &mut c),
            AnimationType::new::<BorderImageRepeatType>(&mut b, &mut c),
            AnimationType::new::<BorderColorType>(&mut b, &mut c),
            AnimationType::new::<HsiType>(&mut b, &mut c),
            AnimationType::new::<BlurType>(&mut b, &mut c),
            AnimationType::new::<MaskImageType>(&mut b, &mut c),
            AnimationType::new::<MaskImageClipType>(&mut b, &mut c),
            AnimationType::new::<TransformType>(&mut b, &mut c),
            AnimationType::new::<TransformOriginType>(&mut b, &mut c),
            AnimationType::new::<TransformWillChangeType>(&mut b, &mut c),
            AnimationType::new::<BorderRadiusType>(&mut b, &mut c),
            AnimationType::new::<ZIndexType>(&mut b, &mut c),
            AnimationType::new::<OverflowType>(&mut b, &mut c),
            AnimationType::new::<BlendModeType>(&mut b, &mut c),
            AnimationType::new::<DisplayType>(&mut b, &mut c),
            AnimationType::new::<VisibilityType>(&mut b, &mut c),
            AnimationType::new::<EnableType>(&mut b, &mut c),
            AnimationType::new::<WidthType>(&mut b, &mut c),
            AnimationType::new::<HeightType>(&mut b, &mut c),
            AnimationType::new::<MarginTopType>(&mut b, &mut c),
            AnimationType::new::<MarginRightType>(&mut b, &mut c),
            AnimationType::new::<MarginBottomType>(&mut b, &mut c),
            AnimationType::new::<MarginLeftType>(&mut b, &mut c),
            AnimationType::new::<PaddingTopType>(&mut b, &mut c),
            AnimationType::new::<PaddingRightType>(&mut b, &mut c),
            AnimationType::new::<PaddingBottomType>(&mut b, &mut c),
            AnimationType::new::<PaddingLeftType>(&mut b, &mut c),
            AnimationType::new::<BorderTopType>(&mut b, &mut c),
            AnimationType::new::<BorderRightType>(&mut b, &mut c),
            AnimationType::new::<BorderBottomType>(&mut b, &mut c),
            AnimationType::new::<BorderLeftType>(&mut b, &mut c),
            AnimationType::new::<PositionTopType>(&mut b, &mut c),
            AnimationType::new::<PositionRightType>(&mut b, &mut c),
            AnimationType::new::<PositionBottomType>(&mut b, &mut c),
            AnimationType::new::<PositionLeftType>(&mut b, &mut c),
            AnimationType::new::<MinWidthType>(&mut b, &mut c),
            AnimationType::new::<MinHeightType>(&mut b, &mut c),
            AnimationType::new::<MaxHeightType>(&mut b, &mut c),
            AnimationType::new::<MaxWidthType>(&mut b, &mut c),
            AnimationType::new::<DirectionType>(&mut b, &mut c),
            AnimationType::new::<FlexDirectionType>(&mut b, &mut c),
            AnimationType::new::<FlexWrapType>(&mut b, &mut c),
            AnimationType::new::<JustifyContentType>(&mut b, &mut c),
            AnimationType::new::<AlignContentType>(&mut b, &mut c),
            AnimationType::new::<AlignItemsType>(&mut b, &mut c),
            AnimationType::new::<PositionTypeType>(&mut b, &mut c),
            AnimationType::new::<AlignSelfType>(&mut b, &mut c),
            AnimationType::new::<FlexShrinkType>(&mut b, &mut c),
            AnimationType::new::<FlexGrowType>(&mut b, &mut c),
            AnimationType::new::<AspectRatioType>(&mut b, &mut c),
            AnimationType::new::<OrderType>(&mut b, &mut c),
            AnimationType::new::<FlexBasisType>(&mut b, &mut c),
            AnimationType::new::<PositionType>(&mut b, &mut c),
            AnimationType::new::<BorderType>(&mut b, &mut c),
            AnimationType::new::<MarginType>(&mut b, &mut c),
            AnimationType::new::<PaddingType>(&mut b, &mut c),
            AnimationType::new::<OpacityType>(&mut b, &mut c),
            AnimationType::new::<TextContentType>(&mut b, &mut c),
            AnimationType::new::<VNodeType>(&mut b, &mut c),
            AnimationType::new::<TransformFuncType>(&mut b, &mut c),
        ];
        let mut temp_keyframes_mark = BitVec::with_capacity(animation_attr_types.len());
        unsafe { temp_keyframes_mark.set_len(animation_attr_types.len()) };
        temp_keyframes_mark.fill(false);
        Self {
            animation_attr_types,
            key_frames_map: Default::default(),
            animation_bind: SecondaryMap::with_capacity(0),
            animation_context_amount: AnimationContextAmount::default(AnimationManagerDefault::default(), AnimationGroupManagerDefault::default()),
            curve_infos: c,
            type_use_mark: temp_keyframes_mark.clone(),
            runtime_info_map: b,
            temp_keyframes_ptr: Default::default(),
            temp_keyframes_mark: temp_keyframes_mark,
        }
    }
}


#[derive(Debug, Clone)]
pub enum KeyFrameError {
    InvalidKeyFrameName,
}

impl KeyFramesSheet {
    // 推动动画
    pub fn run(&mut self, style_commands: &mut StyleCommands, delta_ms: u64) {
        self.runtime_info_map.reset();
        self.animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_info_map);
        for i in self.type_use_mark.iter_ones() {
            log::debug!("KeyFramesSheet run: {:?}", i);
            let ty = &self.animation_attr_types[i];
            (ty.run)(&ty.context, &self.runtime_info_map, style_commands);
        }
    }

    // 将动画绑定到目标上
    pub fn bind_animation(&mut self, target: Id<Node>, animation: &Animation) -> Result<(), KeyFrameError> {
        let curves = match self.key_frames_map.get(&animation.name) {
            Some(r) => r,
            None => return Err(KeyFrameError::InvalidKeyFrameName),
        };


        if let Some(r) = self.animation_bind.get(&target) {
            if r.1.name == animation.name {
                trace!("update_animation, target: {:?}, animation: {:?}", target, animation);
                // 更新， TODO
                return Ok(());
            }
        }

        trace!("bind_animation, target: {:?}, animation: {:?}", target, animation);
        let group0 = self.animation_context_amount.create_animation_group();
        for (attr_animation_id, curve_id) in curves.0.iter() {
            // 向动画组添加 动画
            self.animation_context_amount
                .add_target_animation(*attr_animation_id, group0, target)
                .unwrap();
            self.type_use_mark.set(*curve_id, true);
        }
        // 启动动画组
        let is_loop = animation.iteration_count > 1;
        trace!(
            "start anim, direction: {:?}, frame_per_second: {}, from: {}, to:  {}, duration: {}s",
            animation.direction,
            (FRAME_COUNT / (animation.duration as f32 / 1000.0)).round() as u16,
            0.0,
            FRAME_COUNT,
            animation.duration as f32 / 1000.0
        );
        // TODO
        self.animation_context_amount
            .start(
                group0,
                is_loop,
                1.0,
                animation.direction,
                0.0,
                FRAME_COUNT,
                (FRAME_COUNT / (animation.duration as f32 / 1000.0)).round() as u16,
                match animation.timing_function {
                    TimingFunction::Linear => AnimationAmountCalc::from_easing(EEasingMode::None),
                    TimingFunction::Ease(r) => AnimationAmountCalc::from_easing(r),
                    TimingFunction::Step(step, mode) => AnimationAmountCalc::from_steps(step as u16, mode),
                    TimingFunction::CubicBezier(x1, y1, x2, y2) => AnimationAmountCalc::from_cubic_bezier(x1, y1, x2, y2),
                },
            )
            .unwrap();

        Ok(())
    }

    // 解绑定动画
    pub fn unbind_animation(&mut self, target: Id<Node>) {
        if let Some(r) = self.animation_bind.remove(&target) {
            self.animation_context_amount.del_animation_group(r.0);
        }
    }

    // 添加一个帧动画
    pub fn add_keyframes(&mut self, name: usize, value: XHashMap<NotNan<f32>, VecDeque<Attribute>>) {
        trace!("add_keyframes, name: {:?}", name);
        fn add_progress<T: Attr + FrameDataValue>(progress: u16, value: T, temp_keyframes_ptr: &mut VecMap<usize>, temp_keyframes_mark: &mut BitVec) {
            let index = T::get_style_index() as usize;
            let ptr = match temp_keyframes_ptr.get_mut(index) {
                Some(r) => r,
                None => {
                    temp_keyframes_ptr.insert(
                        index,
                        Box::into_raw(Box::new(FrameCurve::<T>::curve_frame_values(FRAME_COUNT as u16))) as usize,
                    );
                    temp_keyframes_mark.insert(index, true);
                    &mut temp_keyframes_ptr[index]
                }
            };
            unsafe { &mut *(*ptr as *mut FrameCurve<T>) }.curve_frame_values_frame(progress, value);
        }

        for (progress, attrs) in value.into_iter() {
            let progress = (progress * FRAME_COUNT).round() as u16;
            for attr in attrs.into_iter() {
                match attr {
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
                    Attribute::Position(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::Border(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::Margin(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::Padding(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::Opacity(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::TextContent(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::VNode(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                    Attribute::TransformFunc(r) => add_progress(progress, r, &mut self.temp_keyframes_ptr, &mut self.temp_keyframes_mark),
                }
            }
        }

        let mut key_frame = KeyFrames(Vec::new());
        for i in self.temp_keyframes_mark.iter_ones() {
            let curve = self.temp_keyframes_ptr.remove(i).unwrap();
            let ctx = &mut self.animation_attr_types[i];
            let curve_id = (ctx.add_frame_curve)(&mut ctx.context, &mut self.curve_infos, curve);
            let attr_animation_id = self
                .animation_context_amount
                .add_animation(&mut self.curve_infos, curve_id.id, 0, curve_id.ty)
                .unwrap();
            key_frame.0.push((attr_animation_id, i));
        }
        self.temp_keyframes_mark.fill(false);

        // 记录KeyFrames
        self.key_frames_map.insert(name, key_frame);
    }
}

pub struct KeyFrames(Vec<(usize, usize)>); // (动画属性id， 曲线类型)

pub struct CurveId {
    pub ty: usize,
    pub id: usize,
}

impl AnimationType {
    fn new<T: AnimationTypeInterface + Attr + FrameDataValue>(
        runtime_info_map: &mut RuntimeInfoMap<Id<Node>>,
        curve_infos: &mut FrameCurveInfoManager,
    ) -> Self {
        Self {
            context: Box::new(TypeAnimationContext::<T>::new(
                T::get_style_index() as usize,
                runtime_info_map,
                curve_infos,
            )),
            run: T::run,
            add_frame_curve: T::add_frame_curve,
        }
    }
}

pub struct AnimationType {
    context: Box<dyn Any>, // TypeAnimationContext<T>
    run: fn(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<Id<Node>>, style_commands: &mut StyleCommands),
    add_frame_curve: fn(&mut Box<dyn Any>, curve_infos: &mut FrameCurveInfoManager, curve_ptr: usize) -> CurveId,
}

trait AnimationTypeInterface {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<Id<Node>>, style_commands: &mut StyleCommands);
    fn add_frame_curve(context: &mut Box<dyn Any>, curve_infos: &mut FrameCurveInfoManager, curve_ptr: usize) -> CurveId;
}

impl<T: Attr + FrameDataValue> AnimationTypeInterface for T {
    fn run(context: &Box<dyn Any>, runtime_infos: &RuntimeInfoMap<Id<Node>>, style_commands: &mut StyleCommands) {
        trace!("anime run");
        if let Err(e) = context
            .downcast_ref::<TypeAnimationContext<Self>>()
            .unwrap()
            .anime(runtime_infos, style_commands)
        {
            log::error!("{:?}", e);
        }
    }

    fn add_frame_curve(context: &mut Box<dyn Any>, curve_infos: &mut FrameCurveInfoManager, curve_ptr: usize) -> CurveId {
        let curve = unsafe { std::ptr::read(curve_ptr as *const FrameCurve<Self>) };
        out_any!(trace, "add_frame_curve, curve: {:?}", &curve);
        CurveId {
            ty: T::get_style_index() as usize,
            id: context
                .downcast_mut::<TypeAnimationContext<Self>>()
                .unwrap()
                .add_frame_curve(curve_infos, curve),
        }
    }
}

impl<F: Attr + FrameDataValue> TypeAnimationResultPool<F, Id<Node>> for StyleCommands {
    fn record_target(&mut self, _id_target: Id<Node>) {
        // todo!()
    }

    fn record_result(
        &mut self,
        entity: Id<Node>,
        _id_attr: pi_animation::target_modifier::IDAnimatableAttr,
        result: pi_animation::animation_context::AnimeResult<F>,
    ) -> Result<(), pi_animation::error::EAnimationError> {
        out_any!(log::debug, "record animation result===={:?}, {:?}", &result.value, &entity);
        let start = self.style_buffer.len();
        unsafe { StyleAttr::write(result.value, &mut self.style_buffer) };
        if let Some(r) = self.commands.last_mut() {
            if r.0 == entity {
                r.2 = self.style_buffer.len();
                return Ok(());
            }
        }
        self.commands.push((entity, start, self.style_buffer.len()));
        Ok(())
    }
}


const FRAME_COUNT: f32 = 10000.0;
