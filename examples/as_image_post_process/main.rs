// 测试对AsImage节点进行自定义后处理的情况

#[path = "../framework.rs"]
mod framework;

use std::mem::swap;

use bevy_ecs::system::SystemState;
use bevy_app::prelude::{Update, App};
use bevy_ecs::prelude::{World, Query, ResMut, IntoSystemConfigs, Commands};
use framework::Example;
use pi_bevy_render_plugin::{PiRenderGraph, SimpleInOut, RenderContext};
use pi_bevy_render_plugin::component::GraphId;
use pi_bevy_render_plugin::node::{Node, ParamUsage};
/// 渲染四边形 demo
use pi_flex_layout::style::{Dimension, PositionType};
use pi_futures::BoxFuture;
use pi_share::ShareRefCell;
use pi_null::Null;
use pi_style::{
    style::{Aabb2, Point2},
    style_type::{
        AsImageType, BackgroundColorType, HeightType, MarginLeftType, MarginTopType, PositionLeftType, PositionTopType, PositionTypeType, RotateType,
        WidthType,
    },
};
use pi_ui_render::components::user::AsImage;
use pi_ui_render::resource::PostProcessCmd;
use pi_ui_render::system::node::user_setting;
use pi_ui_render::system::system_set::UiSystemSet;
use pi_ui_render::{
    components::{
        calc::EntityKey,
        user::{CgColor, ClearColor, Color, Viewport},
        NodeBundle,
    },
    resource::{NodeCmd, UserCommands},
};
use wgpu::CommandEncoder;

fn main() { framework::start(CustomPostExample::default()) }


#[derive(Default)]
pub struct CustomPostExample {
    cmd: UserCommands,
}

impl Example for CustomPostExample {
	fn setting(&mut self, app: &mut App) {
		app.add_systems(Update, create_post_graph.in_set(UiSystemSet::Setting).after(user_setting::user_setting));
	}

    fn init(&mut self, world: &mut World, size: (usize, usize)) {
        // 设置清屏颜色为绿色
        // gui.gui.world_mut().insert_resource(ClearColor(CgColor::new(0.0, 1.0, 1.0, 1.0)));

        // 添加根节点
        let root = world.spawn(NodeBundle::default()).id();
        self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
        self.cmd.push_cmd(NodeCmd(
            Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
            root,
        ));

        self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
        self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));

        self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
        self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
        self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
        self.cmd.append(root, <EntityKey as pi_null::Null>::null().0);

		let post_entity = world.spawn(GraphId::default()).id();

        // 添加一个红色div到根节点， 并设置AsImage的post_process
        let div1 = world.spawn(NodeBundle::default()).id();
        self.cmd.set_style(div1, WidthType(Dimension::Points(200.0)));
        self.cmd.set_style(div1, HeightType(Dimension::Points(200.0)));
        self.cmd.set_style(div1, AsImageType(pi_style::style::AsImage::Force));
        self.cmd
            .set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 0.0, 1.0))));
        self.cmd.set_style(div1, RotateType(30.0));
		self.cmd.push_cmd(PostProcessCmd(EntityKey(post_entity), div1));
        self.cmd.append(div1, root);
    }

    fn render(&mut self, cmd: &mut UserCommands, _cmd1: &mut Commands) {
        swap(&mut self.cmd, cmd);
    }
}


// 创建后处理节点
pub fn create_post_graph(
	query: Query<&AsImage>, 
	mut query1: Query<&mut GraphId>, 
	mut rg: ResMut<PiRenderGraph>,
) {
	for as_image in query.iter() {
		if let Ok(mut graph_id) = query1.get_mut(*as_image.post_process) {
			if graph_id.is_null() {
				let graph_node_id = match rg.add_node(format!("Test_CustomPost{:?}", *as_image.post_process), CustomPostNode, GraphNodeId::default()) {
					Ok(r) => r,
					Err(e) => {
						log::error!("node: {:?}, {:?}", format!("Test_CustomPost_{:?}", *as_image.post_process), e);
						return;
					}
				};
				log::warn!("add graph node: {:?}", graph_node_id);
	
				*graph_id = GraphId(graph_node_id);
			}
		}
	}
}

pub struct CustomPostNode;

// (, Handle<RenderRes<BindGroup>>)

use pi_bevy_render_plugin::node::NodeId as GraphNodeId;

impl Node for CustomPostNode {
    type Input = SimpleInOut;
    type Output = SimpleInOut;

    type RunParam = ();
	type BuildParam = ();

	fn build<'a>(
		&'a mut self,
		_world: &'a mut bevy_ecs::world::World,
		_param: &'a mut bevy_ecs::system::SystemState<Self::BuildParam>,
		_context: pi_bevy_render_plugin::RenderContext,
		input: &'a Self::Input,
		_usage: &'a pi_bevy_render_plugin::node::ParamUsage,
		_id: GraphNodeId,
		_from: &'a [GraphNodeId],
		_to: &'a [GraphNodeId],
	) -> Result<Self::Output, String> {
		Ok(input.clone())
	}

    fn run<'a>(
        &'a mut self,
        _world: &'a World,
        _query_param_state: &'a mut SystemState<Self::RunParam>,
        _context: RenderContext,
        _commands: ShareRefCell<CommandEncoder>,
        _input: &'a Self::Input,
        _usage: &'a ParamUsage,
        _id: GraphNodeId,
        _from: &'a [GraphNodeId],
        _to: &'a [GraphNodeId],
        // context: RenderContext,
        // mut commands: ShareRefCell<CommandEncoder>,
        // inputs: &'a [Self::Output],
    ) -> BoxFuture<'a, Result<(), String>> {
		Box::pin(async move {
			Ok(())
			// Err("".to_string())
		})
	}
}
