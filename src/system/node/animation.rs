//! 1. 处理animation组件，为节点绑定动画或解绑动画
//! 2. 推动动画运行

use std::mem::transmute;

use pi_ecs::{
    prelude::{Id, ParamSet, Query, Res, ResMut, Write},
    query::{Changed, Deleted, Or},
};
use pi_ecs_macros::setup;

use crate::{
    components::{
        calc::{NodeState, StyleMark},
        user::{
            BackgroundColor, BackgroundImage, BackgroundImageClip, BlendMode, Blur, Border, BorderColor, BorderImage, BorderImageClip,
            BorderImageSlice, BorderRadius, BoxShadow, FlexContainer, FlexNormal, Hsi, Margin, MaskImage, MaskImageClip, MinMax, Node, Opacity,
            Overflow, Padding, Position, Show, Size, TextContent, TextStyle, Transform, TransformWillChange, ZIndex,
        },
    },
    resource::{animation_sheet::KeyFramesSheet, TimeInfo, UserCommands},
};
use pi_style::{
    style::{Animation, BackgroundImageMod, BorderImageRepeat},
    style_type::StyleQuery,
};

use super::user_setting::set_style;

pub struct CalcAnimation;

#[setup]
impl CalcAnimation {
    #[system]
    pub fn calc_animation(
        size: Query<'static, 'static, Node, Write<Size>>,
        margin: Query<'static, 'static, Node, Write<Margin>>,
        padding: Query<'static, 'static, Node, Write<Padding>>,
        border: Query<'static, 'static, Node, Write<Border>>,
        position: Query<'static, 'static, Node, Write<Position>>,
        min_max: Query<'static, 'static, Node, Write<MinMax>>,
        flex_container: Query<'static, 'static, Node, Write<FlexContainer>>,
        flex_normal: Query<'static, 'static, Node, Write<FlexNormal>>,
        z_index: Query<'static, 'static, Node, Write<ZIndex>>,
        overflow: Query<'static, 'static, Node, Write<Overflow>>,
        opacity: Query<'static, 'static, Node, Write<Opacity>>,
        blend_mode: Query<'static, 'static, Node, Write<BlendMode>>,
        show: Query<'static, 'static, Node, Write<Show>>,
        transform: Query<'static, 'static, Node, Write<Transform>>,
        background_color: Query<'static, 'static, Node, Write<BackgroundColor>>,
        border_color: Query<'static, 'static, Node, Write<BorderColor>>,
        background_image: Query<'static, 'static, Node, Write<BackgroundImage>>,
        background_image_clip: Query<'static, 'static, Node, Write<BackgroundImageClip>>,
        mask_image: Query<'static, 'static, Node, Write<MaskImage>>,
        mask_image_clip: Query<'static, 'static, Node, Write<MaskImageClip>>,
        hsi: Query<'static, 'static, Node, Write<Hsi>>,
        blur: Query<'static, 'static, Node, Write<Blur>>,
        background_image_mod: Query<'static, 'static, Node, Write<BackgroundImageMod>>,
        border_image: Query<'static, 'static, Node, Write<BorderImage>>,
        border_image_clip: Query<'static, 'static, Node, Write<BorderImageClip>>,
        border_image_slice: Query<'static, 'static, Node, Write<BorderImageSlice>>,
        border_image_repeat: Query<'static, 'static, Node, Write<BorderImageRepeat>>,
        border_radius: Query<'static, 'static, Node, Write<BorderRadius>>,
        box_shadow: Query<'static, 'static, Node, Write<BoxShadow>>,
        text_style: Query<'static, 'static, Node, Write<TextStyle>>,
        transform_will_change: Query<'static, 'static, Node, Write<TransformWillChange>>,
        node_state: Query<'static, 'static, Node, Write<NodeState>>,
        text_content: Query<'static, 'static, Node, Write<TextContent>>,

        mut animation: ParamSet<(
            Query<'static, 'static, Node, Write<Animation>>,
            Query<'static, 'static, Node, (Id<Node>, Option<&'static Animation>), Or<(Changed<Animation>, Deleted<Animation>)>>,
        )>,

        entitys: Query<'static, 'static, Node, Id<Node>>,
        mut style_mark: Query<'static, 'static, Node, &mut StyleMark>, // TODO OrDefaultMut

        mut user_commands: ResMut<UserCommands>,
        mut keyframes_sheet: ResMut<'static, KeyFramesSheet>,
        cur_time: Res<'static, TimeInfo>,
    ) {
        let animation1 = unsafe { transmute(animation.p0_mut()) };
        let mut style_query = StyleQuery {
            size,
            margin,
            padding,
            border,
            position,
            min_max,
            flex_container,
            flex_normal,
            z_index,
            overflow,
            opacity,
            blend_mode,
            show,
            transform,
            background_color,
            border_color,
            background_image,
            background_image_clip,
            mask_image,
            mask_image_clip,
            hsi,
            blur,
            background_image_mod,
            border_image,
            border_image_clip,
            border_image_slice,
            border_image_repeat,
            border_radius,
            box_shadow,
            text_style,
            transform_will_change,
            text_content,
            node_state,
            animation: animation1,
        };

        for (node, animation) in animation.p1().iter() {
            if let Some(r) = animation {
                if let Err(e) = keyframes_sheet.bind_animation(node, r) {
                    log::error!("{:?}", e);
                }
            } else {
                keyframes_sheet.unbind_animation(node);
            }
        }

        log::trace!("cur time: {:?}", &*cur_time);

        // 推动动画执行
        keyframes_sheet.run(&mut user_commands.style_commands, cur_time.delta);

        // 设置style只要节点存在,样式一定能设置成功
        set_style(&mut user_commands.style_commands, &mut style_query, &entitys, &mut style_mark);
    }
}
