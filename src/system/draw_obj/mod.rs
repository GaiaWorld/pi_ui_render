use blend_mode::blend_mod_change;
use pi_world::prelude::IntoSystemConfigs;
use pi_world::prelude::{App, Plugin, WorldPluginExtent};
use pi_bevy_render_plugin::{GraphBuild, GraphRun};
use pi_hal::font::font::FontType;
use pi_style::style::Aabb2;
use pi_world::schedule::PostUpdate;

use crate::components::calc::WorldMatrix;
use crate::resource::draw_obj::MaxViewSize;
use crate::shader1::InstanceData;
use crate::shader1::batch_meterial::{LayoutUniform, WorldMatrixMeterial};

use super::debug::DebugPlugin;
use crate::system::base::{
	node::{show, z_index},
	pass::{last_update_wgpu::last_update_wgpu, pass_life, update_graph::update_graph},
};
use super::system_set::UiSystemSet;

use crate::prelude::UiStage;

use self::calc_border_radius::BorderRadiusPlugin;
use self::calc_background_color::BackgroundColorPlugin;
use self::calc_background_image::BackgroundImagePlugin;
use self::calc_border_color::BorderColorPlugin;
use self::calc_border_image::BorderImagePlugin;
use self::calc_box_shadow::BoxShadowPlugin;
use self::calc_canvas::CanvasPlugin;
use self::calc_text::UiTextPlugin;
use crate::system::base::draw_obj::life_drawobj::{batch_instance_data, update_render_instance_data};

pub mod calc_background_color;
pub mod calc_background_image;
pub mod calc_border_color;
pub mod calc_border_image;
pub mod calc_box_shadow;
pub mod calc_canvas;
pub mod calc_text;
// pub mod calc_svg;
pub mod calc_border_radius;

pub mod blend_mode;
// pub mod clear_draw_obj;
pub mod pipeline;
// pub mod root_clear_color;
pub mod root_view_port;
pub mod geo_split;

pub struct UiReadyDrawPlugin {
	pub font_type: FontType,
}

impl Plugin for UiReadyDrawPlugin {
    fn build(&self, app: &mut App) {
		app.world.init_single_res::<MaxViewSize>();

        app
			// .add_system(Startup, clear_draw_obj::init)// PostStartup, 
            // .init_single_res::<MaxViewSize>()
			.add_system(UiStage, update_render_instance_data
				.after(UiSystemSet::LifeDrawObjectFlush)
				.after( UiSystemSet::PassFlush)
				.after(z_index::calc_zindex)
            	.after(show::calc_show)
				.after(update_graph)
				.after(pass_life::calc_pass_toop_sort)
				.before(UiSystemSet::PrepareDrawObj)
				)
			.add_system(PostUpdate, batch_instance_data
				.before(last_update_wgpu)
				.after(GraphBuild)
				.before(GraphRun)
				)
            .add_system(UiStage, root_view_port::calc_dyn_target_type.in_set(UiSystemSet::BaseCalc))
            // .add_system(UiStage, pipeline::calc_node_pipeline.in_set(UiSystemSet::PrepareDrawObj))
            // 混合模式
			.add_system(UiStage, 
                blend_mode::calc_drawobj_blendstate
					.run_if(blend_mod_change)
                    
                    .before(UiSystemSet::LifeDrawObjectFlush)
                    .after(UiSystemSet::LifeDrawObject),
            )

			.add_plugins(BorderRadiusPlugin)
			// 背景图片功能
			.add_plugins(BackgroundImagePlugin)
			// 背景颜色功能
			.add_plugins(BackgroundColorPlugin)
			// 九宫格功能
			.add_plugins(BorderImagePlugin)
			// 边框颜色功能
            .add_plugins(BorderColorPlugin)
		    // box-shadow功能
		    .add_plugins(BoxShadowPlugin)
		    // canvas功能
		    .add_plugins(CanvasPlugin)
			// 文字功能
			.add_plugins(UiTextPlugin {font_type: self.font_type})
            // svg功能
		    // .add_plugins(SvgPlugin)
			.add_plugins(DebugPlugin)
			;
    }
}

pub fn set_box(_world_matrix: &WorldMatrix, _layou_rect: &Aabb2, _instance_data: &mut InstanceData) {
	// let left_top = world_matrix * Vector4::new(layou_rect.mins.x, layou_rect.mins.y, 0.0, 1.0);
	// let left_bottom = world_matrix * Vector4::new(layou_rect.mins.x, layou_rect.maxs.y, 0.0, 1.0);
	// let right_bottom = world_matrix * Vector4::new(layou_rect.maxs.x, layou_rect.maxs.y, 0.0, 1.0);
	// let right_top = world_matrix * Vector4::new(layou_rect.maxs.x, layou_rect.mins.y, 0.0, 1.0);

	// instance_data.set_data(&BoxUniform(&[layou_rect.mins.x, layou_rect.mins.y, layou_rect.maxs.x - layou_rect.mins.x, layou_rect.maxs.y - layou_rect.mins.y]));
	// instance_data.set_data(&QuadUniform(&[
	// 	left_top.x, left_top.y,
	// 	left_bottom.x, left_bottom.y,
	// 	right_bottom.x, right_bottom.y,
	// 	right_top.x, right_top.y,
	// ]));
}


pub fn set_matrix(world_matrix: &WorldMatrix, layou_rect: &Aabb2, instance_data: &mut InstanceData) {
	instance_data.set_data(&LayoutUniform(&[layou_rect.mins.x, layou_rect.mins.y, layou_rect.maxs.x - layou_rect.mins.x, layou_rect.maxs.y - layou_rect.mins.y]));
	instance_data.set_data(&WorldMatrixMeterial(world_matrix.as_slice()));
}