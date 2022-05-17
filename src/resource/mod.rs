// use pi_ecs::component::Component;
// use pi_share::Share;
// use crate::font::{font_sheet::FontSheet as FontSheet1, font_tex::FontTexture};

// pub struct FontSheet<T: FontTexture + Component>(Share<FontSheet1<T>>);

pub mod draw_obj;

use pi_ecs::prelude::{World, FromWorld, Id};

use crate::components::user::{TextContent, ClassName, Aabb2, Point2, Node};

/// 用户指令

#[derive(Default)]
pub struct UserCommands {
	/// 节点指令
	pub node_commands: Vec<NodeCommand>,
	/// 样式指令
	pub style_commands: StyleCommands,
	/// 文本指令
	pub text_commands: Vec<(Id<Node>, Option<TextContent>)>,
	/// class指令
	pub class_commands: Vec<(Id<Node>, ClassName)>,
}

/// 节点指令
pub enum NodeCommand {
	/// 插入节点（节点，父节点）,
	AppendNode(Id<Node>, Id<Node>),
	/// 插入节点（节点，锚点）,
	InsertBefore(Id<Node>, Id<Node>),
	/// 删除节点,
	RemoveNode(Id<Node>),
	/// 销毁节点
	DestroyNode(Id<Node>),
}

#[derive(Default)]
pub struct StyleCommands {
	/// 样式列表
	// pub style_list: Vec<Attribute>,
	pub style_buffer: Vec<u8>,
	/// 设置样式（节点，开始索引，结束索引），其中开始索引和结束索引是指在style_list中的索引
	pub commands: Vec<(Id<Node>, usize, usize)>,
}

/// 渲染上下文标记分配器，每一种可以使节点成为渲染上下文的属性，都可以让全局单例RenderContextMarkAlloc分配一个id
#[derive(Debug, Default, Deref, DerefMut)]
pub struct RenderContextMarkAlloc(usize);

/// 渲染上下文类型，每一种可以使节点成为渲染上下文的属性，都对应一个RenderContextMarkType，类型值是在初始化时，找RenderContextMarkAlloc分配的。
#[derive(Debug, Deref, DerefMut, Default)]
pub struct RenderContextMarkType(usize);

impl FromWorld for RenderContextMarkType {
	fn from_world(world: &mut World) -> Self {
		let cur_mark_index = match world.get_resource_mut::<RenderContextMarkAlloc>() {
			Some(r) => r,
			None =>{ 
				world.insert_resource(RenderContextMarkAlloc::default());
				world.get_resource_mut::<RenderContextMarkAlloc>().unwrap()
			}
		};
		**cur_mark_index += 1;
		Self(**cur_mark_index)
	}
}

// /// 宏标记分配器
// #[derive(Debug, Default, Deref, DerefMut)]
// pub struct DefineAlloc(usize);

// /// 宏类型，每一get不同的宏，都对应一个数字，一个shader中的宏开关，就是这些数字的组合种可以使节点成为渲染上下文的属性，都对应一个DefineType，类型值是在初始化时，找DefineAlloc分配的。
// #[derive(Debug, Deref, DerefMut, Default)]
// pub struct DefineType(usize);

// impl FromWorld for DefineType {
// 	fn from_world(world: &mut World) -> Self {
// 		let cur_mark_index = world.get_resource_mut::<DefineAlloc>().unwrap();
// 		**cur_mark_index += 1;
// 		Self(**cur_mark_index)
// 	}
// }

#[derive(Clone, Serialize, Deserialize, Deref, DerefMut)]
pub struct Viewport(pub Aabb2);

impl Default for Viewport {
    fn default() -> Self {
        Self(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0)))
    }
}