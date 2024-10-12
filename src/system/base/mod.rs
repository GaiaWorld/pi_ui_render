pub mod draw_obj;
pub mod node;
pub mod pass;

use node::world_matrix::cal_matrix;
/// sdf
use pi_world::prelude::Plugin;
use pi_world::prelude::WorldPluginExtent;
use pi_world::schedule_config::IntoSystemConfigs;
use crate::prelude::UiStage;

use super::system_set::UiSystemSet;


pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
            .add_plugins(draw_obj::sdf_gen::SdfPlugin)
            .add_plugins(node::UiNodePlugin)
            .add_plugins(pass::UiPassPlugin)
            .add_system(UiStage, draw_obj::set_geo_uniform::set_matrix_uniform
                .after(cal_matrix)
                .in_set(UiSystemSet::PrepareDrawObj))
        ;
        
    }
}