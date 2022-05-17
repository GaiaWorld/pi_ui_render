use std::{any::TypeId, sync::Arc, mem::replace};

use pi_async::rt::{AsyncRuntime, single_thread::SingleTaskPool};
use pi_ecs::prelude::{World, ArchetypeId, SingleDispatcher, StageBuilder, Id, Setup, Dispatcher};
use pi_render::RenderStage;

use crate::{components::user::ClassName, resource::{UserCommands, NodeCommand}, utils::style::style_sheet::{StyleAttr, Attr}, system::{node::{user_setting::CalcUserSetting, context::CalcContext, z_index::CalcZindex, layout::CalcLayout, quad::CalcQuad, world_matrix::CalcMatrix, content_box::CalcContentBox, context_root::CalcRoot, background_color::CalcBackGroundColor}, draw_obj::{world_marix::CalcWorldMatrixGroup, pipeline::CalcPipeline}, pass::{pass_render::CalcRender, pass_dirty_rect::CalcDirtyRect}}};

use crate::components::user::Node;

pub struct Gui {
	world: World,

	user_commands: UserCommands,

	node_archetype_id: ArchetypeId,

	dispatcher: SingleDispatcher<SingleTaskPool<()>>,
}

impl Gui {
	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn new(rt: AsyncRuntime<(), SingleTaskPool<()>>) -> Gui {
		let mut world = World::new();
		world.new_archetype::<Node>().create(); // 创建Node原型

		let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

		let dispatcher= SingleDispatcher::new(rt);

		Gui {
			world,
			node_archetype_id,
			user_commands: UserCommands::default(),
			dispatcher,
		}
	}

	// 初始化gui
	pub fn init(&mut self, render_stages: RenderStage) {
		init_dispatcher(&mut self.world, render_stages, &mut self.dispatcher);
	}

	// 创建节点
	pub fn create_node(&mut self) -> Id<Node> {
		let node_archetype_id = self.node_archetype_id;
		unsafe { Id::new(self.world.archetypes_mut()[node_archetype_id].reserve_entity()) }
	}

	/// 将节点作为子节点挂在父上
	pub fn append(&mut self, entity: Id<Node>, parent: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
	}

	/// 将节点插入到某个节点之前
	pub fn insert_before(&mut self, entity: Id<Node>, anchor: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
	}

	/// 从父节点上移除节点
	pub fn remove_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
	}

	/// 从父节点上移除节点，并销毁该节点及所有子节点
	pub fn destroy_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
	}

	/// 设置节点样式
	pub fn set_style<T: Attr>(&mut self, entity: Id<Node>, value: T){
		let start = self.user_commands.style_commands.style_buffer.len();
		unsafe {StyleAttr::write(
			value,
			&mut self.user_commands.style_commands.style_buffer,
		)};
		if let Some(r) = self.user_commands.style_commands.commands.last_mut() {
			if r.0 == entity {
				r.2 = self.user_commands.style_commands.style_buffer.len();
				return;
			}
		}
		self.user_commands.style_commands.commands.push((entity, start, self.user_commands.style_commands.style_buffer.len()));
	}

	/// 设置节点的class
	pub fn set_class(&mut self, entity: Id<Node>, value: ClassName){
		self.user_commands.class_commands.push((entity, value));
	}

	/// 推动gui运行
	pub fn run(&mut self) {
		let node_archetype_id = self.node_archetype_id;
		self.world.archetypes_mut()[node_archetype_id].flush();
		let commands = replace(&mut self.user_commands, UserCommands::default());
		self.world.insert_resource(commands);
		self.dispatcher.run();
	}
}

fn init_dispatcher(world: &mut World, render_stages: RenderStage, dispatcher: &mut SingleDispatcher<SingleTaskPool<()>>) {
	// let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
	let mut stages = Vec::new();

	// 节点属性计算阶段
	let mut node_stage = StageBuilder::new();
	CalcUserSetting::setup(world, &mut node_stage);
	CalcContext::setup(world, &mut node_stage);
	CalcZindex::setup(world, &mut node_stage);
	CalcLayout::setup(world, &mut node_stage);
	CalcQuad::setup(world, &mut node_stage);
	CalcMatrix::setup(world, &mut node_stage);
	CalcContentBox::setup(world, &mut node_stage);
	CalcRoot::setup(world, &mut node_stage);
	CalcBackGroundColor::setup(world, &mut node_stage);
	
	// 渲染对象计算
	let mut draw_stage = StageBuilder::new();
	CalcWorldMatrixGroup::setup(world, &mut draw_stage);
	CalcPipeline ::setup(world, &mut draw_stage);

	// Pass计算
	let mut pass_stage = StageBuilder::new();
	CalcRender::setup(world, &mut pass_stage);
	CalcDirtyRect::setup(world, &mut pass_stage);

	
	stages.push(Arc::new(node_stage.build(world)));
	stages.push(Arc::new(draw_stage.build(world)));
	stages.push(Arc::new(pass_stage.build(world)));
	stages.push(Arc::new(render_stages.extract_stage.build(world)));
	stages.push(Arc::new(render_stages.prepare_stage.build(world)));
	stages.push(Arc::new(render_stages.render_stage.build(world)));

	dispatcher.init(stages, world);
}