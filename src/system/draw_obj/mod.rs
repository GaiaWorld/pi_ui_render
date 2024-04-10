use bevy_ecs::prelude::{IntoSystemSetConfig, IntoSystemConfigs};
use bevy_app::{App, Plugin, PostUpdate, Update};
use pi_bevy_render_plugin::{FrameDataPrepare, GraphBuild, GraphRun};
use pi_hal::font::font::FontType;
use pi_style::style::Aabb2;

use crate::components::calc::WorldMatrix;
use crate::resource::draw_obj::{EmptyVertexBuffer, MaxViewSize};
use crate::shader1::InstanceData;
use crate::shader1::meterial::{BoxUniform, QuadUniform};

use super::node::{z_index, show};
use super::pass::last_update_wgpu::last_update_wgpu;
use super::pass::pass_life;
use super::{system_set::UiSystemSet, pass::update_graph::update_graph};

use crate::components::user::Vector4;

use self::calc_background_color::BackgroundColorPlugin;
use self::calc_background_image::BackgroundImagePlugin;
use self::calc_border_color::BorderColorPlugin;
use self::calc_border_image::BorderImagePlugin;
use self::calc_box_shadow::BoxShadowPlugin;
use self::calc_canvas::CanvasPlugin;
use self::calc_svg::SvgPlugin;
use self::calc_text::UiTextPlugin;
use self::life_drawobj::{batch_instance_data, update_render_instance_data};
// use self::calc_text::UiTextPlugin;

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
pub mod calc_svg;

pub mod blend_mode;
// pub mod clear_draw_obj;
pub mod pipeline;
// pub mod root_clear_color;
pub mod root_view_port;
// pub mod set_world_marix;

pub struct UiReadyDrawPlugin {
	pub font_type: FontType,
}

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, UiSystemSet::PrepareDrawObj.in_set(FrameDataPrepare));

        app
			// .add_systems(Startup, clear_draw_obj::init)// PostStartup, 
            .init_resource::<MaxViewSize>()
			.add_systems(Update, update_render_instance_data
				.after(UiSystemSet::LifeDrawObjectFlush)
				.after( UiSystemSet::PassFlush)
				.after(z_index::calc_zindex)
            	.after(show::calc_show)
				.after(update_graph)
				.after(pass_life::calc_pass_toop_sort)
				.before(UiSystemSet::PrepareDrawObj)
				.in_set(FrameDataPrepare))
			.add_systems(PostUpdate, batch_instance_data
				.before(last_update_wgpu)
				.after(GraphBuild)
				.before(GraphRun)
				.in_set(FrameDataPrepare))
            .add_systems(Update, root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
            .add_systems(Update, pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawObj))
            // 混合模式
			.add_systems(Update, 
                blend_mode::calc_drawobj_blendstate
                    .in_set(FrameDataPrepare)
                    .before(UiSystemSet::LifeDrawObjectFlush)
                    .after(UiSystemSet::LifeDrawObject),
            )
            // .add_systems(Update, 
            //     root_clear_color::clear_change
            //         .in_set(FrameDataPrepare)
            //         .after(UiSystemSet::PassFlush)
            //         .after(UiSystemSet::PassCalc),
            // )

			// 圆角
			.add_systems(Update, 
                calc_border_radius::calc_border_radius
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .after(UiSystemSet::LifeDrawObject),
            )
            // .add_systems(Update, root_view_port::view_port_change.in_set(UiSystemSet::PrepareDrawObj))
            .init_resource::<EmptyVertexBuffer>()
			// 背景图片功能
			.add_plugins(BackgroundImagePlugin)
			// 背景颜色功能
			.add_plugins(BackgroundColorPlugin)
			// 九宫格功能
			.add_plugins(BorderImagePlugin)
            // 文字功能
            .add_plugins(UiTextPlugin {font_type: self.font_type})
			// 边框颜色功能
            .add_plugins(BorderColorPlugin)
		    // box-shadow功能
		    .add_plugins(BoxShadowPlugin)
		    // canvas功能
		    .add_plugins(CanvasPlugin)
            // svg功能
		    .add_plugins(SvgPlugin);
    }
}

pub fn set_box(world_matrix: &WorldMatrix, layou_rect: &Aabb2, instance_data: &mut InstanceData) {
	let left_top = world_matrix * Vector4::new(layou_rect.mins.x, layou_rect.mins.y, 0.0, 1.0);
	let left_bottom = world_matrix * Vector4::new(layou_rect.mins.x, layou_rect.maxs.y, 0.0, 1.0);
	let right_bottom = world_matrix * Vector4::new(layou_rect.maxs.x, layou_rect.maxs.y, 0.0, 1.0);
	let right_top = world_matrix * Vector4::new(layou_rect.maxs.x, layou_rect.mins.y, 0.0, 1.0);

	instance_data.set_data(&BoxUniform(&[layou_rect.mins.x, layou_rect.mins.y, layou_rect.maxs.x - layou_rect.mins.x, layou_rect.maxs.y - layou_rect.mins.y]));
	instance_data.set_data(&QuadUniform(&[
		left_top.x, left_top.y,
		left_bottom.x, left_bottom.y,
		right_bottom.x, right_bottom.y,
		right_top.x, right_top.y,
	]));
}
