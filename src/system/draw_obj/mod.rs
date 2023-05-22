use bevy::{
    app::Plugin,
    prelude::{Changed, IntoSystemConfig, IntoSystemSetConfig, StartupSet},
};

use crate::resource::draw_obj::{EmptyVertexBuffer, MaxViewSize};

use super::{render_run, system_set::UiSystemSet, AddEvent};

use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::component::GraphId;

use crate::components::draw_obj::{BackgroundColorMark, BackgroundImageMark, BorderColorMark, BorderImageMark, BoxShadowMark, CanvasMark, TextMark};
use crate::components::user::{BackgroundColor, BorderColor, BoxShadow, Canvas, TextContent};
use crate::resource::draw_obj::{PosUv1VertexLayout, PosUv2VertexLayout, PosUvColorVertexLayout, PosVertexLayout};
use crate::resource::{
    BackgroundColorRenderObjType, BackgroundImageRenderObjType, BorderColorRenderObjType, BorderImageRenderObjType, BoxShadowRenderObjType,
    CanvasRenderObjType, TextRenderObjType,
};
use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, NodeState},
        user::{BackgroundImage, BorderImage},
    },
    resource::ShareFontSheet,
};

use self::calc_background_image::calc_background_image_texture;
use super::draw_obj::set_world_marix::set_matrix_group;

pub mod calc_background_color;
pub mod calc_background_image;
pub mod calc_border_color;
pub mod calc_border_image;
pub mod calc_border_radius;
pub mod calc_box_shadow;
pub mod calc_canvas;
pub mod calc_text;
pub mod image_texture_load;
pub mod life_drawobj;

pub mod clear_draw_obj;
pub mod pipeline;
pub mod root_clear_color;
pub mod root_view_port;
pub mod set_world_marix;

pub struct UiReadyDrawPlugin;

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_set(UiSystemSet::PrepareDrawObj.run_if(render_run));

        app.add_startup_system(clear_draw_obj::init.in_base_set(StartupSet::PostStartup))
            .init_resource::<MaxViewSize>()
            .add_system(root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
            .add_system(pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawObj))
            // 在世界矩阵之后运行
            .add_system(
                set_world_marix::set_matrix_group
                    .in_set(UiSystemSet::BaseCalc)
                    .after(crate::system::node::world_matrix::cal_matrix),
            )
            .add_system(root_clear_color::clear_change.in_set(UiSystemSet::PrepareDrawObj))
            .add_system(root_view_port::view_port_change.in_set(UiSystemSet::PrepareDrawObj))
            .init_resource::<EmptyVertexBuffer>()
            // BackgroundImage功能
            .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
            .add_system(image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>.in_set(UiSystemSet::Load))
            .add_system(
                life_drawobj::draw_object_life::<
                    BackgroundImageTexture,
                    BackgroundImageRenderObjType,
                    BackgroundImageMark,
                    PosUv1VertexLayout,
                    crate::shader::image::ProgramMeta,
                    { calc_background_image::BACKGROUND_IMAGE_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject)
                    .after(image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>),
            )
            .add_system(
                calc_background_image_texture
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(calc_background_image::calc_background_image),
            )
            .add_system(
                calc_background_image::calc_background_image
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group),
            )
            // 文字功能
            .init_resource::<ShareFontSheet>()
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_system(calc_text::text_split.before(super::node::layout::calc_layout).in_set(UiSystemSet::Layout))
            .add_system(
                calc_text::text_glyph
                    .after(super::node::world_matrix::cal_matrix)
                    .before(calc_text::calc_text)
                    .in_set(UiSystemSet::Matrix),
            )
            .add_frame_event::<ComponentEvent<Changed<TextContent>>>()
            .add_system(
                life_drawobj::draw_object_life::<
                    TextContent,
                    TextRenderObjType,
                    TextMark,
                    PosUvColorVertexLayout,
                    crate::shader::text::ProgramMeta,
                    { calc_text::TEXT_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject),
            )
            .add_system(calc_text::calc_text.in_set(UiSystemSet::PrepareDrawObj).before(set_matrix_group))
            // 背景颜色功能
            .add_frame_event::<ComponentEvent<Changed<BackgroundColor>>>()
            .add_system(
                life_drawobj::draw_object_life::<
                    BackgroundColor,
                    BackgroundColorRenderObjType,
                    BackgroundColorMark,
                    PosVertexLayout,
                    crate::shader::color::ProgramMeta,
                    { calc_background_color::BACKGROUND_COLOR_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject)
                    .before(calc_background_color::calc_background_color),
            )
            .add_system(
                calc_background_color::calc_background_color
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group),
            )
            // BorderColor功能
            .add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
            .add_system(
                life_drawobj::draw_object_life::<
                    BorderColor,
                    BorderColorRenderObjType,
                    BorderColorMark,
                    PosVertexLayout,
                    crate::shader::color::ProgramMeta,
                    { calc_border_color::BORDER_COLOR_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject),
            )
            .add_system(
                calc_border_color::calc_border_color
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group),
            )
            // BorderImage功能
            .add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
            .add_system(image_texture_load::image_change::<BorderImage, BorderImageTexture>.in_set(UiSystemSet::Load))
            .add_system(
                life_drawobj::draw_object_life::<
                    BorderImageTexture,
                    BorderImageRenderObjType,
                    BorderImageMark,
                    PosUv2VertexLayout,
                    crate::shader::image::ProgramMeta,
                    { calc_border_image::BORDER_IMAGE_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject)
                    .after(image_texture_load::image_change::<BorderImage, BorderImageTexture>),
            )
            .add_system(
                calc_border_image::calc_border_image
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group),
            )
            // BoxShadow功能
            .add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
            .add_system(
                life_drawobj::draw_object_life::<
                    BoxShadow,
                    BoxShadowRenderObjType,
                    BoxShadowMark,
                    PosVertexLayout,
                    crate::shader::color::ProgramMeta,
                    { calc_box_shadow::BOX_SHADOW_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject),
            )
            .add_system(
                calc_box_shadow::calc_box_shadow
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group),
            )
            // canvas功能
            .add_frame_event::<ComponentEvent<Changed<Canvas>>>()
            .add_system(
                life_drawobj::draw_object_life::<
                    Canvas,
                    CanvasRenderObjType,
                    (CanvasMark, GraphId),
                    PosUv1VertexLayout,
                    crate::shader::image::ProgramMeta,
                    { calc_canvas::CANVAS_ORDER },
                >
                    .in_set(UiSystemSet::LifeDrawObject),
            )
            .add_system(calc_canvas::calc_canvas.in_set(UiSystemSet::PrepareDrawObj).before(set_matrix_group))
            .add_system(
                calc_border_radius::calc_border_radius
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .after(UiSystemSet::LifeDrawObject),
            );
    }
}
