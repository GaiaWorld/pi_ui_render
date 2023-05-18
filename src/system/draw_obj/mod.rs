use bevy::{app::Plugin, prelude::{StartupSet, IntoSystemConfig, IntoSystemSetConfig, Changed}};

use crate::{resource::draw_obj::{MaxViewSize, EmptyVertexBuffer},};

use super::{system_set::UiSystemSet, render_run, AddEvent};

use pi_bevy_ecs_extend::{system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::component::GraphId;

use crate::components::draw_obj::{BackgroundImageMark, TextMark, BorderImageMark, BorderColorMark, BackgroundColorMark, BoxShadowMark, CanvasMark};
use crate::components::user::{TextContent, BorderColor, BackgroundColor, BoxShadow, Canvas};
use crate::resource::{BackgroundImageRenderObjType, TextRenderObjType, BorderImageRenderObjType, BorderColorRenderObjType, BackgroundColorRenderObjType, BoxShadowRenderObjType, CanvasRenderObjType};
use crate::resource::draw_obj::{PosUv1VertexLayout, PosUvColorVertexLayout, PosUv2VertexLayout, PosVertexLayout};
use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, NodeState},
        user::{BackgroundImage, BorderImage},
    },
    resource::ShareFontSheet,
};

use self::background_image::calc_texture;
use super::draw_obj::world_marix::calc_matrix_group;

pub mod text_glyph;
pub mod text_split;
pub mod background_color;
pub mod background_image;
pub mod border_color;
pub mod border_image;
pub mod box_shadow;
pub mod canvas;
pub mod image_texture_load;
pub mod text;
pub mod life_drawobj;

pub mod clear_draw_obj;
pub mod pipeline;
pub mod root_clear_color;
pub mod world_marix;
pub mod root_view_port;

pub struct UiReadyDrawPlugin;

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut bevy::app::App) {
		app.configure_set(UiSystemSet::PrepareDrawObj.run_if(render_run));
		
        app
			.add_startup_system(clear_draw_obj::init.in_base_set(StartupSet::PostStartup))
            .init_resource::<MaxViewSize>()
            .add_system(root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
            .add_system(pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawObj))
			// 在世界矩阵之后运行
            .add_system(world_marix::calc_matrix_group.in_set(UiSystemSet::BaseCalc)
				.after(crate::system::node::world_matrix::cal_matrix)
			)
			.add_system(root_clear_color::clear_change.in_set(UiSystemSet::PrepareDrawObj))
			.add_system(root_view_port::view_port_change.in_set(UiSystemSet::PrepareDrawObj))
			
			
			.init_resource::<EmptyVertexBuffer>()
			
			// BackgroundImage功能
			.add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
			.add_system(image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>.in_set(UiSystemSet::Load),)
			.add_system(life_drawobj::draw_object_life::<BackgroundImageTexture, BackgroundImageRenderObjType, BackgroundImageMark, PosUv1VertexLayout, crate::shader::image::ProgramMeta, {background_image::BACKGROUND_IMAGE_ORDER}>
				.in_set(UiSystemSet::LifeDrawObject)
				.after(image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>))
			.add_system(calc_texture.in_set(UiSystemSet::PrepareDrawObj).before(background_image::calc_background_image))
			.add_system(
				background_image::calc_background_image.after(super::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group))
			
			// 文字功能
			.init_resource::<ShareFontSheet>()
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_system(text_split::text_split.before(super::node::layout::calc_layout).in_set(UiSystemSet::Layout))
            .add_system(
                text_glyph::text_glyph.after(super::node::world_matrix::cal_matrix).before(text::calc_text).in_set(UiSystemSet::Matrix),
            )
			.add_frame_event::<ComponentEvent<Changed<TextContent>>>()
			.add_system(life_drawobj::draw_object_life::<TextContent, TextRenderObjType, TextMark, PosUvColorVertexLayout, crate::shader::text::ProgramMeta, {text::TEXT_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(text::calc_text.in_set(UiSystemSet::PrepareDrawObj).before(calc_matrix_group))
			
			// 背景颜色功能
			.add_frame_event::<ComponentEvent<Changed<BackgroundColor>>>()
			.add_system(life_drawobj::draw_object_life::<BackgroundColor, BackgroundColorRenderObjType, BackgroundColorMark, PosVertexLayout, crate::shader::color::ProgramMeta, {background_color::BACKGROUND_COLOR_ORDER}>.in_set(UiSystemSet::LifeDrawObject).before(background_color::calc_background))
			.add_system(background_color::calc_background
				.after(super::node::layout::calc_layout).in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group))
			
			// BorderColor功能
			.add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
			.add_system(life_drawobj::draw_object_life::<BorderColor, BorderColorRenderObjType, BorderColorMark, PosVertexLayout, crate::shader::color::ProgramMeta, {border_color::BORDER_COLOR_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
            .add_system(border_color::calc_border_color.after(super::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group))

			// BorderImage功能
			.add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
            .add_system(image_texture_load::image_change::<BorderImage, BorderImageTexture>.in_set(UiSystemSet::Load))
			.add_system(life_drawobj::draw_object_life::<BorderImageTexture, BorderImageRenderObjType, BorderImageMark, PosUv2VertexLayout, crate::shader::image::ProgramMeta, {border_image::BORDER_IMAGE_ORDER}>
				.in_set(UiSystemSet::LifeDrawObject)
				.after(image_texture_load::image_change::<BorderImage, BorderImageTexture>))
			.add_system(border_image::calc_border_image.after(super::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group)
			)

			// BoxShadow功能
			.add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
			.add_system(life_drawobj::draw_object_life::<BoxShadow, BoxShadowRenderObjType, BoxShadowMark, PosVertexLayout, crate::shader::color::ProgramMeta, {box_shadow::BOX_SHADOW_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
            .add_system(box_shadow::calc_box_shadow.after(super::node::layout::calc_layout)
				.in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group))
			
			// canvas功能
			.add_frame_event::<ComponentEvent<Changed<Canvas>>>()
			.add_system(life_drawobj::draw_object_life::<Canvas, CanvasRenderObjType, (CanvasMark, GraphId), PosUv1VertexLayout, crate::shader::image::ProgramMeta, {canvas::CANVAS_ORDER}>.in_set(UiSystemSet::LifeDrawObject))
			.add_system(canvas::calc_canvas
				.in_set(UiSystemSet::PrepareDrawObj)
				.before(calc_matrix_group))
			
			;
    }
}
