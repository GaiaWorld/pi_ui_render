// use pi_ecs::component::Component;
// use pi_share::Share;
// use crate::font::{font_sheet::FontSheet as FontSheet1, font_tex::FontTexture};

// pub struct FontSheet<T: FontTexture + Component>(Share<FontSheet1<T>>);

use pi_ecs::{entity::Entity, world::FromWorld, prelude::World};

use crate::{components::user::{TextContent, ClassName}};

/// 用户指令

#[derive(Default)]
pub struct UserCommands {
	/// 节点指令
	pub node_commands: Vec<NodeCommand>,
	/// 样式指令
	pub style_commands: StyleCommands,
	/// 文本指令
	pub text_commands: Vec<(Entity, Option<TextContent>)>,
	/// class指令
	pub class_commands: Vec<(Entity, ClassName)>,
}

/// 节点指令
pub enum NodeCommand {
	/// 插入节点（节点，父节点）,
	AppendNode(Entity, Entity),
	/// 插入节点（节点，锚点）,
	InsertBefore(Entity, Entity),
	/// 删除节点,
	RemoveNode(Entity),
	/// 销毁节点
	DestroyNode(Entity),
}

#[derive(Default)]
pub struct StyleCommands {
	/// 样式列表
	// pub style_list: Vec<Attribute>,
	pub style_buffer: Vec<u8>,
	/// 设置样式（节点，开始索引，结束索引），其中开始索引和结束索引是指在style_list中的索引
	pub commands: Vec<(Entity, usize, usize)>,
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct CurRenderContextMark(usize);


#[derive(Debug, Deref, DerefMut, Default)]
pub struct RenderContextMarkType(usize);

impl FromWorld for RenderContextMarkType {
	fn from_world(world: &mut World) -> Self {
		let cur_mark_index = world.get_resource_mut::<CurRenderContextMark>().unwrap();
		**cur_mark_index += 1;
		Self(**cur_mark_index)
	}
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Viewport {
	pub x: usize,
	pub y: usize,
	pub width: usize,
	pub height: usize,
}