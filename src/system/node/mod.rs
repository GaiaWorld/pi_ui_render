use bevy::app::Plugin;
use bevy::ecs::{query::Changed};
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig};
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::component::GraphId;

use crate::components::draw_obj::{BackgroundImageMark, TextMark, BorderImageMark, BorderColorMark, BackgroundColorMark, BoxShadowMark, CanvasMark};
use crate::components::user::{TextContent, BorderColor, BackgroundColor, BoxShadow, Canvas};
use crate::resource::{BackgroundImageRenderObjType, TextRenderObjType, BorderImageRenderObjType, BorderColorRenderObjType, BackgroundColorRenderObjType, BoxShadowRenderObjType, CanvasRenderObjType};
use crate::resource::draw_obj::{PosUv1VertexLayout, PosUvColorVertexLayout, PosUv2VertexLayout, PosVertexLayout};
use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, ContentBox, LayoutResult, NodeState, Quad, RenderContextMark},
        pass_2d::ParentPassId,
        user::{BackgroundImage, BorderImage, Transform},
    },
    resource::{animation_sheet::KeyFramesSheet, draw_obj::EmptyVertexBuffer, ClassSheet, QuadTree, ShareFontSheet, TimeInfo, UserCommands},
};

use self::background_image::calc_texture;
use self::world_matrix::OldQuad;

use super::draw_obj::world_marix::calc_matrix_group;
use super::system_set::UiSystemSet;
use super::{render_run, layout_run, matrix_run, AddEvent};

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
pub mod life_drawobj;

pub struct UiNodePlugin;

impl Plugin for UiNodePlugin {
    fn build(&self, app: &mut bevy::app::App) {
		app.configure_set(UiSystemSet::Load.run_if(layout_run));
		app.configure_set(UiSystemSet::Layout.run_if(layout_run));
		app.configure_set(UiSystemSet::Matrix.run_if(matrix_run));
		app.configure_set(UiSystemSet::BaseCalc.run_if(render_run));
		app.configure_set(UiSystemSet::LifeDrawObject.run_if(render_run));
		
        app
			.add_frame_event::<ComponentEvent<Changed<Layer>>>()
            .init_resource::<UserCommands>()
            .init_resource::<ClassSheet>()
            .init_resource::<TimeInfo>()
            .init_resource::<KeyFramesSheet>()
            .add_system(user_setting::user_setting.in_set(UiSystemSet::Setting))
            .add_system(animation::calc_animation.after(user_setting::user_setting).in_set(UiSystemSet::Setting).run_if(render_run))
            .add_system(user_setting::set_image_default_size.in_set(UiSystemSet::Layout))
            .add_system(z_index::calc_zindex.in_set(UiSystemSet::BaseCalc))
            .add_frame_event::<ComponentEvent<Changed<LayoutResult>>>()
            .add_system(layout::calc_layout.after(user_setting::set_image_default_size).in_set(UiSystemSet::Layout))
            .add_frame_event::<ComponentEvent<Changed<Transform>>>()
            .add_frame_event::<ComponentEvent<Changed<Quad>>>()
            .add_frame_event::<OldQuad>()
            .init_resource::<QuadTree>()
            .add_system(world_matrix::cal_matrix.after(layout::calc_layout).in_set(UiSystemSet::Matrix))
            .add_frame_event::<ComponentEvent<Changed<ContentBox>>>()
            .add_system(content_box::calc_content_box.after(world_matrix::cal_matrix).in_set(UiSystemSet::BaseCalc))
            .init_resource::<ShareFontSheet>()
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_system(text_split::text_split.before(layout::calc_layout).in_set(UiSystemSet::Layout))
            .add_system(
                text_glyph::text_glyph.after(world_matrix::cal_matrix).after(text_split::text_split).in_set(UiSystemSet::Matrix),
            )
            .add_frame_event::<ComponentEvent<Changed<RenderContextMark>>>()
            .add_frame_event::<ComponentEvent<Changed<ParentPassId>>>()
            .add_system(context::cal_context.in_set(UiSystemSet::BaseCalc))
            .add_system(context_opacity::opacity_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc))
            .add_system(context_overflow::overflow_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc))
            .add_system(context_hsi::hsi_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc))
            .add_system(context_blur::blur_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc))
            .add_system(
                context_transform_will_change::transform_willchange_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc),
            )
            .add_system(context_root::root_calc.before(context::cal_context).in_set(UiSystemSet::BaseCalc))
			.init_resource::<EmptyVertexBuffer>()
			
			// BackgroundImage功能
			.add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
			.add_system(image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>.in_set(UiSystemSet::Load),)
			.add_system(life_drawobj::draw_object_life::<BackgroundImageTexture, BackgroundImageRenderObjType, BackgroundImageMark, PosUv1VertexLayout, crate::shader::image::ProgramMeta, {background_image::BACKGROUND_IMAGE_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(calc_texture.in_set(UiSystemSet::BaseCalc).before(background_image::calc_background_image))
			.add_system(
				background_image::calc_background_image.after(layout::calc_layout)
				.in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group))
			
			// 文字功能
			.add_frame_event::<ComponentEvent<Changed<TextContent>>>()
			.add_system(life_drawobj::draw_object_life::<TextContent, TextRenderObjType, TextMark, PosUvColorVertexLayout, crate::shader::text::ProgramMeta, {text::TEXT_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(text::calc_text.in_set(UiSystemSet::BaseCalc).before(calc_matrix_group))
			
			// 背景颜色功能
			.add_frame_event::<ComponentEvent<Changed<BackgroundColor>>>()
			.add_system(life_drawobj::draw_object_life::<BackgroundColor, BackgroundColorRenderObjType, BackgroundColorMark, PosVertexLayout, crate::shader::color::ProgramMeta, {background_color::BACKGROUND_COLOR_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(background_color::calc_background
				.after(layout::calc_layout).in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group))
			
			// BorderColor功能
			.add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
			.add_system(life_drawobj::draw_object_life::<BorderColor, BorderColorRenderObjType, BorderColorMark, PosVertexLayout, crate::shader::color::ProgramMeta, {border_color::BORDER_COLOR_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
            .add_system(border_color::calc_border_color.after(layout::calc_layout)
				.in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group))

			// BorderImage功能
			.add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
            .add_system(image_texture_load::image_change::<BorderImage, BorderImageTexture>.in_set(UiSystemSet::Load))
			.add_system(life_drawobj::draw_object_life::<BorderImageTexture, BorderImageRenderObjType, BorderImageMark, PosUv2VertexLayout, crate::shader::image::ProgramMeta, {border_image::BORDER_IMAGE_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(border_image::calc_border_image.after(layout::calc_layout)
				.in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group)
			)

			// BoxShadow功能
			.add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
			.add_system(life_drawobj::draw_object_life::<BoxShadow, BoxShadowRenderObjType, BoxShadowMark, PosVertexLayout, crate::shader::color::ProgramMeta, {box_shadow::BOX_SHADOW_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
            .add_system(box_shadow::calc_box_shadow.after(layout::calc_layout)
				.in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group))
			
			// canvas功能
			.add_frame_event::<ComponentEvent<Changed<Canvas>>>()
			.add_system(life_drawobj::draw_object_life::<Canvas, CanvasRenderObjType, (CanvasMark, GraphId), PosUv1VertexLayout, crate::shader::image::ProgramMeta, {canvas::CANVAS_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(canvas::calc_canvas
				.in_set(UiSystemSet::BaseCalc)
				.before(calc_matrix_group))
			
            .add_system(show::calc_show.in_set(UiSystemSet::BaseCalc))
			;
    }
}
