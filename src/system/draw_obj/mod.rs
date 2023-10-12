use bevy_ecs::prelude::{Changed, IntoSystemSetConfig, IntoSystemConfigs};
use bevy_app::{Plugin, Update, Startup, App};

use crate::resource::draw_obj::{EmptyVertexBuffer, MaxViewSize};

use super::{render_run, system_set::UiSystemSet, AddEvent, pass::update_graph::update_graph};

use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::component::GraphId;

use crate::components::draw_obj::{BackgroundColorMark, BackgroundImageMark, BorderColorMark, BorderImageMark, BoxShadowMark, CanvasMark};
use crate::components::user::{BackgroundColor, BorderColor, BoxShadow, Canvas};
use crate::components::{
    calc::{BackgroundImageTexture, BorderImageTexture},
    user::{BackgroundImage, BorderImage},
};
use crate::resource::draw_obj::{PosUv1VertexLayout, PosUv2VertexLayout, PosVertexLayout};
use crate::resource::{
    BackgroundColorRenderObjType, BackgroundImageRenderObjType, BorderColorRenderObjType, BorderImageRenderObjType, BoxShadowRenderObjType,
    CanvasRenderObjType,
};

use self::{calc_background_image::calc_background_image_texture, calc_text::UiTextPlugin};
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

pub mod blend_mode;
pub mod clear_draw_obj;
pub mod pipeline;
pub mod root_clear_color;
pub mod root_view_port;
pub mod set_world_marix;

pub struct UiReadyDrawPlugin;

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, UiSystemSet::PrepareDrawObj.run_if(render_run));

        app.add_systems(Startup, clear_draw_obj::init)// PostStartup, 
            .init_resource::<MaxViewSize>()
            .add_systems(Update, root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
            .add_systems(Update, pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawObj))
            .add_systems(Update, 
                blend_mode::calc_drawobj_blendstate
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(pipeline::calc_node_pipeline)
                    .after(UiSystemSet::LifeDrawObject),
            )
            // 在世界矩阵之后运行
            .add_systems(Update, 
                set_world_marix::set_matrix_group
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .after(crate::system::node::world_matrix::cal_matrix),
            )
            .add_systems(Update, 
                root_clear_color::clear_change
                    .run_if(render_run)
                    .after(UiSystemSet::PassFlush)
                    .after(UiSystemSet::PassCalc),
            )
            // .add_systems(Update, root_view_port::view_port_change.in_set(UiSystemSet::PrepareDrawObj))
            .init_resource::<EmptyVertexBuffer>()
            // BackgroundImage功能
            .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
            .add_systems(Update, image_texture_load::image_change::<BackgroundImage, BackgroundImageTexture>.in_set(UiSystemSet::Load))
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_background_image_texture
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(calc_background_image::calc_background_image),
            )
            .add_systems(Update, 
                calc_background_image::calc_background_image
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(blend_mode::calc_drawobj_blendstate)
                    .before(calc_border_radius::calc_border_radius),
            )
            // 文字功能
            .add_plugins(UiTextPlugin)
            // 背景颜色功能
            .add_frame_event::<ComponentEvent<Changed<BackgroundColor>>>()
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_background_color::calc_background_color
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(blend_mode::calc_drawobj_blendstate)
                    .before(calc_border_radius::calc_border_radius),
            )
            // BorderColor功能
            .add_frame_event::<ComponentEvent<Changed<BorderColor>>>()
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_border_color::calc_border_color
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(blend_mode::calc_drawobj_blendstate)
                    .before(calc_border_radius::calc_border_radius),
            )
            // BorderImage功能
            .add_frame_event::<ComponentEvent<Changed<BorderImageTexture>>>()
            .add_systems(Update, image_texture_load::image_change::<BorderImage, BorderImageTexture>.in_set(UiSystemSet::Load))
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_border_image::calc_border_image
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(calc_border_radius::calc_border_radius),
            )
            // BoxShadow功能
            .add_frame_event::<ComponentEvent<Changed<BoxShadow>>>()
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_box_shadow::calc_box_shadow
                    .after(super::node::layout::calc_layout)
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(calc_border_radius::calc_border_radius),
            )
            // canvas功能
            .add_frame_event::<ComponentEvent<Changed<Canvas>>>()
            .add_systems(Update, 
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
            .add_systems(Update, 
                calc_canvas::calc_canvas
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .before(calc_border_radius::calc_border_radius)
					.before(update_graph),
            )
            .add_systems(Update, 
                calc_border_radius::calc_border_radius
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .after(UiSystemSet::LifeDrawObject),
            );
    }
}
