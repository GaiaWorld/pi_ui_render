use std::{intrinsics::transmute, any::TypeId};

use pi_ecs::{prelude::{World, Entities, ArchetypeId}, entity::Entity};

use crate::{components::{user::ClassName, calc::StyleType}, resource::{UserCommands, NodeCommand}, utils::style::style_sheet::StyleAttr};

use crate::components::user::Node;

pub struct Gui {
	world: World,

	node_entities: &'static mut Entities,

	user_commands: &'static mut UserCommands,

	node_archetype_id: ArchetypeId,
}

impl Gui {
	pub fn init() -> Gui {
		let mut world = World::new();
		world.new_archetype::<Node>().create();

		let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();
		let n =  unsafe{transmute(world.entities_mut(node_archetype_id.clone()))};

		let user_commands = UserCommands::default();

		world.insert_resource(user_commands);

		let user_commands = unsafe{transmute(world.get_resource_mut::<UserCommands>().unwrap())};
		Gui {
			world,
			node_entities: n,
			node_archetype_id,
			user_commands,
		}
	}
	// 创建节点
	pub fn create_node(&mut self) -> Entity {
		let local = self.node_entities.reserve_entity();
		Entity::new(self.node_archetype_id, local)
	}

	/// 将节点作为子节点挂在父上
	pub fn append(&mut self, entity: Entity, parent: Entity) {
		self.user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
	}

	/// 将节点插入到某个节点之前
	pub fn insert_before(&mut self, entity: Entity, anchor: Entity) {
		self.user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
	}

	/// 从父节点上移除节点
	pub fn remove_node(&mut self, entity: Entity) {
		self.user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
	}

	/// 从父节点上移除节点，并销毁该节点及所有子节点
	pub fn destroy_node(&mut self, entity: Entity) {
		self.user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
	}

	/// 设置节点样式
	pub fn set_style<T>(&mut self, entity: Entity, ty: StyleType, value: T){
		let start = self.user_commands.style_commands.style_buffer.len();
		unsafe {StyleAttr::write(
			ty,
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
	pub fn set_class(&mut self, entity: Entity, value: ClassName){
		self.user_commands.class_commands.push((entity, value));
	}

	/// 推动gui运行
	pub fn run(&mut self) {
		self.node_entities.flush();
	}
}



// struct StyleList{

// }

// #[allow(non_snake_case)]
// fn to_Dimension(t: u8, v: f32) -> Dimension {
// 	match t {
// 		1 => Dimension::Auto,
// 		2 => Dimension::Points(v),
// 		3 => Dimension::Percent(v),
// 		_ => Dimension::Undefined,
// 	}
// }

// #[allow(non_snake_case)]
// fn to_FontSize(t: u8, v: f32) -> FontSize {
// 	match t {
// 		1 => FontSize::Length(v),
// 		2 => FontSize::Percent(v),
// 		_ => FontSize::None,
// 	}
// }

// #[allow(non_snake_case)]
// fn to_LineHeight(t: u8, v: f32) -> LineHeight {
// 	match t {
// 		1 => LineHeight::Length(v),
// 		2 => LineHeight::Number(v),
// 		3 => LineHeight::Percent(v),
// 		_ => LineHeight::Normal,
// 	}
// }

// #[allow(non_snake_case)]
// fn to_Blur(v: f32) -> Blur {
// 	Blur(v)
// }

// #[allow(non_snake_case)]
// fn to_Opacity(v: f32) -> Opacity {
// 	Opacity(v)
// }

// #[allow(non_snake_case)]
// fn to_BorderImageRepeat(x: u8, y: u8) -> BorderImageRepeat {
// 	BorderImageRepeat (unsafe { transmute(x)}, unsafe{transmute(y)})
// }

// #[allow(non_snake_case)]
// fn to_BackgroundColor(x: u8) -> BackgroundColor {
// 	unsafe { transmute(x)}
// }

// set_mult_attr!(min_width, MaskImage, MaskImage, u8, f32);

// set_mult_attr!(max_width, BackgroundColor, BackgroundColor, u8, f32);
// set_mult_attr!(max_height, BorderColor, BorderColor, u8, f32);
// set_mult_attr!(flex_basis, BoxShadow, BoxShadow, u8, f32);

// set_mult_attr!(min_width, ImageClip, ImageClip, u8, f32);
// set_mult_attr!(min_height, BorderImageClip, BorderImageClip, u8, f32);
// set_mult_attr!(max_width, BorderImageSlice, BorderImageSlice, u8, f32);
// set_mult_attr!(max_height, TextShadow, TextShadow, u8, f32);
// set_mult_attr!(flex_basis, Stroke, Stroke, u8, f32);

// set_mult_attr!(min_width, BorderRadius, BorderRadius, u8, f32);
// set_mult_attr!(min_height, TransformFunc, TransformFunc, u8, f32);
// set_mult_attr!(max_width, TransformOrigin, TransformOrigin, u8, f32);
// set_mult_attr!(max_height, Filter, Filter, u8, f32);
// set_mult_attr!(flex_basis, MaskImageClip, ImageClip, u8, f32);