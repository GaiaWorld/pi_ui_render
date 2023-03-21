use bevy::app::{CoreStage, Plugin};
use bevy::ecs::{query::Changed, schedule::IntoSystemDescriptor};
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};

use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, ContentBox, LayoutResult, NodeState, Quad, RenderContextMark},
        pass_2d::ParentPassId,
        user::{BackgroundImage, BorderImage, Transform},
    },
    resource::{animation_sheet::KeyFramesSheet, draw_obj::EmptyVertexBuffer, ClassSheet, QuadTree, ShareFontSheet, TimeInfo, UserCommands},
};

use self::{image_texture_load::ImageAwait, world_matrix::OldQuad};

use super::{setting_run, render_run, layout_run, matrix_run, AddEvent};

// pub mod flush;
pub mod layout;
pub mod text_glyph;
pub mod text_split;
pub mod world_matrix;
// pub mod quad;
pub mod content_box;
pub mod context;
pub mod context_blur;
pub mod context_hsi;
pub mod context_opacity;
pub mod context_root;
pub mod context_transform_will_change;
pub mod show;
pub mod user_setting;
pub mod z_index;
// pub mod context_mask_texture;
pub mod animation;
pub mod background_color;
pub mod background_image;
pub mod border_color;
pub mod border_image;
pub mod box_shadow;
pub mod canvas;
pub mod context_overflow;
pub mod image_texture_load;
pub mod text;

pub struct UiNodePlugin;

impl Plugin for UiNodePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>()
            .init_resource::<UserCommands>()
            .init_resource::<ClassSheet>()
            .init_resource::<TimeInfo>()
            .init_resource::<KeyFramesSheet>()
            .add_system_to_stage(CoreStage::First, user_setting::user_setting.at_start().with_run_criteria(setting_run))
            .add_system_to_stage(CoreStage::First, animation::calc_animation.at_start().after(user_setting::user_setting).with_run_criteria(render_run))
            // 加载图片
            .init_resource::<ImageAwait<BackgroundImage>>()
            .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
            .add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
            .add_system_to_stage(
                CoreStage::First,
                image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>.with_run_criteria(render_run),
            )
            .init_resource::<ImageAwait<BorderImage>>()
            .add_system_to_stage(CoreStage::First, image_texture_load::image_change::<BorderImage, BorderImageTexture>.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, user_setting::set_image_default_size.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, z_index::calc_zindex.with_run_criteria(render_run))
            .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_system_to_stage(CoreStage::PreUpdate, layout::calc_layout.after(user_setting::set_image_default_size).with_run_criteria(layout_run))
            .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            .add_frame_event::<OldQuad>()
            .init_resource::<QuadTree>()
            .add_system_to_stage(CoreStage::PreUpdate, world_matrix::cal_matrix.after(layout::calc_layout).with_run_criteria(matrix_run))
            .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            .add_system_to_stage(CoreStage::PreUpdate, content_box::calc_content_box.after(world_matrix::cal_matrix).with_run_criteria(matrix_run))
            .init_resource::<ShareFontSheet>()
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_system_to_stage(CoreStage::PreUpdate, text_split::text_split.with_run_criteria(layout_run))
            .add_system_to_stage(
                CoreStage::PreUpdate,
                text_glyph::text_glyph.after(world_matrix::cal_matrix).after(text_split::text_split).with_run_criteria(matrix_run),
            )
            .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_system_to_stage(CoreStage::PreUpdate, context::cal_context.with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, context_opacity::opacity_calc.before(context::cal_context).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, context_overflow::overflow_calc.before(context::cal_context).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, context_hsi::hsi_calc.before(context::cal_context).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, context_blur::blur_calc.before(context::cal_context).with_run_criteria(render_run))
            .add_system_to_stage(
                CoreStage::PreUpdate,
                context_transform_will_change::transform_willchange_calc.before(context::cal_context).with_run_criteria(render_run),
            )
            .add_system_to_stage(CoreStage::PreUpdate, context_root::root_calc.before(context::cal_context).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, background_color::calc_background.after(layout::calc_layout).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, border_color::calc_border_color.after(layout::calc_layout).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, box_shadow::calc_box_shadow.after(layout::calc_layout).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, background_image::calc_background_image.after(layout::calc_layout).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, border_image::calc_border_image.after(layout::calc_layout).with_run_criteria(render_run))
            .init_resource::<EmptyVertexBuffer>()
            .add_system_to_stage(CoreStage::PreUpdate, text::calc_text.after(text_glyph::text_glyph).with_run_criteria(render_run))
            .add_system_to_stage(CoreStage::PreUpdate, show::calc_show.with_run_criteria(render_run));
    }
}
