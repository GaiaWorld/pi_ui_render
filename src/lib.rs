#![feature(specialization)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(box_into_inner)]
#![feature(if_let_guard)]
#![feature(fmt_helpers_for_derive)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]
#![allow(invalid_reference_casting)]

// use pi_hash::XHashSet;


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
pub mod shader1;
pub mod system;
pub mod utils;
pub mod events;


pub mod prelude {
    use pi_world::{prelude::{App, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, WorldPluginExtent}, schedule::PostUpdate};

    use pi_hal::font::font::FontType;

    pub use crate::resource::UserCommands;
    use crate::system::{
        /*shader_utils::UiShaderPlugin, */ base::{node::UiNodePlugin, pass::UiPassPlugin, BasePlugin}, draw_obj::UiReadyDrawPlugin, pass_effect::UiEffectPlugin, shader_utils::UiShaderPlugin, system_set::{UiSchedule, UiSystemSet}, RunState
    };

    // #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
    // pub struct UiStage;

    pub use pi_world::prelude::PreUpdate as UiStage;

    #[derive(Default)]
    pub struct UiPlugin {
        #[cfg(feature = "debug")]
        pub cmd_trace: crate::system::base::node::cmd_play::TraceOption,
		pub font_type: FontType,
    }
    impl Plugin for UiPlugin {
        fn build(&self, app: &mut App) {
            // let mut ui_schedule = Schedule::new();
            // ui_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

            // let mut order = app.world.get_single_res_mut::<MainScheduleOrder>().unwrap();
            // app.world.get_single_res_mut::<MainScheduleOrder>().unwrap().ui_schedule = UiStage;
            // MainScheduleOrder

            app.world.init_single_res::<RunState>();
            // app.init_single_res::<RunState>();
            app
            
            .configure_set(UiStage, UiSystemSet::Setting.in_schedule(UiSchedule::Layout).in_schedule(UiSchedule::Calc).in_schedule(UiSchedule::Geo))
            .configure_set(PostUpdate, UiSystemSet::ClearSetting.in_schedule(UiSchedule::Layout).in_schedule(UiSchedule::Calc).in_schedule(UiSchedule::Geo))
            // NextSetting在Setting之后运行， Setting用于作用用户指令， NextSetting用于设置加载、动画等派发过程中产生的指令
            .configure_set(UiStage, UiSystemSet::NextSetting
                // 
                .after(UiSystemSet::Setting)
            )
            // 所有其他逻辑SystemSet应该在所有指令完成后运行
            // .configure_set(UiStage, UiSystemSet::Setting.run_if(setting_run))
            // .configure_set(UiStage, UiSystemSet::Layout.run_if(layout_run).after(UiSystemSet::NextSetting))
            // .configure_set(UiStage, UiSystemSet::Matrix.run_if(matrix_run).after(UiSystemSet::NextSetting))

            // .configure_set(UiStage, UiSystemSet::Setting.run_if(setting_run))
            // .configure_set(UiStage, UiSystemSet::Layout.run_if(layout_run).after(UiSystemSet::NextSetting))
            // .configure_set(UiStage, UiSystemSet::Matrix.run_if(matrix_run).after(UiSystemSet::NextSetting))
            
            .configure_set(UiStage, UiSystemSet::PrepareDrawObj
                // 
                .after(UiSystemSet::LifeDrawObjectFlush)
                .before(UiSystemSet::PassCalc)
            )
            .configure_set(UiStage, UiSystemSet::BaseCalc
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::BaseCalcFlush)
            )
            .configure_set(UiStage, UiSystemSet::LifeDrawObject
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::LifeDrawObjectFlush)
            )
            .configure_set(UiStage, UiSystemSet::PassMark
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::PassLife)
            )
            .configure_set(UiStage, UiSystemSet::PassFlush
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::PassSetting)
            )
            .configure_set(UiStage, UiSystemSet::PassSetting
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::PassCalc)
            )
            .configure_set(UiStage, UiSystemSet::PassLife
                // 
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::PassFlush)
            )
            .configure_set(UiStage, UiSystemSet::PassSettingWithParent
                // 
                .after(UiSystemSet::NextSetting)
            )
            .configure_set(UiStage, UiSystemSet::PassCalc
                // 
                .after(UiSystemSet::NextSetting)
            )
			
            .configure_set(UiStage, 
                UiSystemSet::Layout
                .after(UiSystemSet::NextSetting)
                .before(UiSystemSet::LayoutAfter))

            // .configure_sets(
			// 	UiStage, 
            //     (
            //         UiSystemSet::Layout,
            //         UiSystemSet::Matrix,
            //     )
            //         .chain(),
            // )
            // .configure_sets(UiStage, (UiSystemSet::BaseCalc, UiSystemSet::BaseCalcFlush).chain())

            // .configure_sets(UiStage, (
            //     UiSystemSet::PassMark, 
            //     UiSystemSet::PassLife, 
            //     UiSystemSet::PassFlush, 
            //     UiSystemSet::PassSetting, 
            //     UiSystemSet::PassCalc
            // ).chain())	

            // .configure_sets(
			// 	UiStage, 
            //     (
            //         UiSystemSet::LifeDrawObject,
            //         UiSystemSet::LifeDrawObjectFlush,
            //         UiSystemSet::PrepareDrawObj,
            //         UiSystemSet::PassCalc,
            //     )
            //         .chain(),
            // )

			// .add_frame_event::<EntityChange>()
			// .add_frame_event::<NodeZindexChange>()
			// .add_frame_event::<NodeDisplayChange>()
			.add_system(UiStage, crate::system::res_load::load_res.in_set(UiSystemSet::NextSetting))
            .add_plugins(UiShaderPlugin)
            .add_plugins(BasePlugin)
            .add_plugins(UiEffectPlugin)
            .add_plugins(UiReadyDrawPlugin {
				font_type: self.font_type
			})
            // .add_system(UiStage, apply_deferred.in_set(UiSystemSet::LifeDrawObjectFlush))

			// .add_system(Last, crate::clear_remove_component.after(bevy_window::FrameSet)); // 在每帧结束时清理删除组件的列表
            ;
            #[cfg(feature = "debug")]
            app.add_plugins(crate::system::base::node::cmd_play::UiCmdTracePlugin { option: self.cmd_trace });
        }
    }
}

// pub fn clear_remove_component(world: &mut pi_world::prelude::World) {
// 	world.removed_components_update();
// }



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


// #[test]
// fn aa() {
// 	use pi_async_rt::prelude::AsyncRuntime;
// 	use std::sync::Arc;
// 	use std::sync::atomic::AtomicU32;
// 	let aa = pi_async_rt::rt::startup_global_time_loop(10);

// 	let a = Arc::new(AtomicU32::new(0));
// 	for i in 0..100 {
// 		// let mut t = Vec::new();
// 		// for i in 0..100 {
// 		// 	let a1 = a.clone();
// 		// 	let task = async move {
// 		// 		// a1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
// 		// 	};
// 		// 	t.push(task);
// 		// }
// 		let time = std::time::Instant::now();
// 		for i in 0..100000 {
// 			// let a1 = a.clone();
// 			let task = async move {
// 				// a1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
// 			};
// 			pi_hal::runtime::MULTI_MEDIA_RUNTIME.spawn(task);
// 		}
// 		let time1 = std::time::Instant::now();
// 		println!("time====={:?}, {:?}", a, time1 - time);
// 	}
	
// }

// #[test]
// fn test1() {
// 	let meta = <crate::shader::image::ProgramMeta as pi_render::rhi::shader::ShaderProgram>::create_meta();
// 	// println!("shader====={}", meta.to_code(&pi_hash::XHashSet::default(), wgpu::ShaderStages::VERTEX));
// 	println!("shader====={}", meta.to_code(&pi_hash::XHashSet::default(), wgpu::ShaderStages::FRAGMENT));

// 	let bind_group_layout = meta.bind_group_layout(&pi_hash::XHashSet::default(), wgpu::ShaderStages::FRAGMENT);
// 	println!("shader_code================bind_group_layout={bind_group_layout:?}");
// 	// println!("shader====={}", meta.to_code(&pi_hash::XHashSet::default(), wgpu::ShaderStages::FRAGMENT));
// 	// bind_group_layout
// }
