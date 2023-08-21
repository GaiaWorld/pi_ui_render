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
extern crate derive_deref_rs;
#[macro_use]
extern crate pi_enum_default_macro;

extern crate paste;
#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod resource;
pub mod shader;
pub mod system;
pub mod utils;

pub mod prelude {
    use bevy::{
        app::{App, Plugin},
        prelude::{IntoSystemSetConfigs, apply_deferred, Update, IntoSystemConfigs},
    };
    use pi_bevy_render_plugin::PiRenderSystemSet;

    pub use crate::resource::UserCommands;
    use crate::system::{
        /*shader_utils::UiShaderPlugin, */ draw_obj::UiReadyDrawPlugin, node::UiNodePlugin, pass::UiPassPlugin, pass_effect::UiEffectPlugin,
        shader_utils::UiShaderPlugin, system_set::UiSystemSet, RunState,
    };

    #[derive(Default)]
    pub struct UiPlugin {
        #[cfg(feature = "debug")]
        pub cmd_trace: crate::system::cmd_play::TraceOption,
    }
    impl Plugin for UiPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<RunState>();
            app.configure_sets(
				Update, 
                (
                    UiSystemSet::Setting,
                    UiSystemSet::Load,
                    UiSystemSet::Layout,
                    UiSystemSet::LifeDrawObjectFlush,
                    UiSystemSet::PassFlush,
                    UiSystemSet::Matrix,
                )
                    .chain(),
            )
            .configure_sets(
				Update, 
                (
                    UiSystemSet::Setting,
                    UiSystemSet::LifeDrawObject,
                    UiSystemSet::LifeDrawObjectFlush,
                    UiSystemSet::PrepareDrawObj,
                    UiSystemSet::PrepareDrawObjFlush,
                )
                    .chain(),
            )
            .configure_sets(Update, (UiSystemSet::Setting, UiSystemSet::BaseCalc, UiSystemSet::BaseCalcFlush).chain())
            .configure_sets(
				Update, 
                (
                    UiSystemSet::Setting,
                    UiSystemSet::PrepareDrawObjFlush,
                    UiSystemSet::BaseCalcFlush,
                    PiRenderSystemSet,
                )
                    .chain(),
            )
            .add_plugins(UiShaderPlugin)
            .add_plugins(UiNodePlugin)
            .add_plugins(UiEffectPlugin)
            .add_plugins(UiReadyDrawPlugin)
            .add_plugins(UiPassPlugin)
            // .add_systems(Update, apply_system_buffers.in_set(UiSystemSet::LoadFlush))
            .add_systems(Update, apply_deferred.in_set(UiSystemSet::LifeDrawObjectFlush))
            // .add_systems(Update, apply_system_buffers.in_set(UiSystemSet::BaseCalcFlush))
            .add_systems(Update, apply_deferred.in_set(UiSystemSet::PrepareDrawObjFlush));

            #[cfg(feature = "debug")]
            app.add_plugins(crate::system::cmd_play::UiCmdTracePlugin { option: self.cmd_trace });
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
    let mut parser = naga::front::glsl::Frontend::default();
    let modlue = parser.parse(&naga::front::glsl::Options::from(naga::ShaderStage::Vertex), r);
    println!("modle================={:?}, \nmodle================={:?}", modlue, parser);
}
