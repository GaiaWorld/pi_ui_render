#![feature(specialization)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(type_name_of_val)]
#![feature(box_into_inner)]
#![feature(if_let_guard)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(print_internals)]



#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate pi_enum_default_macro;

extern crate paste;
#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod resource;
pub mod system;
pub mod utils;
// pub mod gui;
// pub mod export;
pub mod shader;


pub mod prelude {
	use pi_bevy_render_plugin::PiRenderSystemSet;
    use bevy::{app::{App, Plugin}, prelude::{IntoSystemSetConfigs, apply_system_buffers, IntoSystemConfig}};

    pub use crate::resource::UserCommands;
    use crate::system::{
        /*shader_utils::UiShaderPlugin, */ draw_obj::UiReadyDrawPlugin, node::UiNodePlugin, pass::UiPassPlugin, shader_utils::UiShaderPlugin, RunState, system_set::UiSystemSet,
    };

    #[derive(Default)]
    pub struct UiPlugin;
    impl Plugin for UiPlugin {
        fn build(&self, app: &mut App) {
			app.init_resource::<RunState>();
            app
				.configure_sets(
					(
						UiSystemSet::Setting,
						UiSystemSet::Load,
						UiSystemSet::Layout,
						UiSystemSet::Matrix,
						UiSystemSet::BaseCalc,
						UiSystemSet::PrepareDrawOb,
						UiSystemSet::PreparePass,
						PiRenderSystemSet,
					)
						.chain())
				.add_plugin(UiShaderPlugin)
                .add_plugin(UiNodePlugin)
                .add_plugin(UiReadyDrawPlugin)
                .add_plugin(UiPassPlugin)
				.add_system(apply_system_buffers.in_set(UiSystemSet::BaseCalc))
				.add_system(apply_system_buffers.in_set(UiSystemSet::PrepareDrawOb))
				.add_system(apply_system_buffers.in_set(UiSystemSet::PreparePass));
				
        }
    }
}

#[test]
fn test() {
    let r = r#"#version 450
	layout(set=1,binding=0) uniform M_1_0{
	mat4 world;
	mat4 clipSdf;
	vec4 color;
	vec4 strokeColorOrURect;
	vec2 textureSizeOrBottomLeftBorder;
	float depth;
	float blur;
	};
	void main(){
			
	
	}"#;
    let mut parser = naga::front::glsl::Parser::default();
    let modlue = parser.parse(&naga::front::glsl::Options::from(naga::ShaderStage::Vertex), r);
    println!("modle================={:?}, \nmodle================={:?}", modlue, parser);
}
