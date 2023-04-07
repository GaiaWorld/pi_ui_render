use std::{
    hash::{Hash, Hasher},
};


use crate::{
    resource::draw_obj::{PipelineStateWithHash, ProgramMetaInner, VertexBufferLayoutWithHash},
};
use bevy::ecs::{prelude::Component};
use pi_atom::Atom;
use pi_hash::{XHashSet};
use pi_render::renderer::draw_obj::DrawObj as DrawState1;
use pi_share::Share;

pub use super::root::{ClearColorBindGroup, CopyFboToScreen, DynTargetType};

pub struct DrawObject;

#[derive(Debug, Default, Component, Deref, DerefMut)]
pub struct DrawState(DrawState1);

/// 是否使用单位四边形渲染
#[derive(EnumDefault, PartialEq, Eq, Component)]
pub enum BoxType {
    /// 渲染为content区，世界矩阵不变换
    ContentNone,
    /// 渲染为border区，世界矩阵不变换
    BorderNone,
    /// 渲染为content区，世界矩阵需要变换
    ContentRect,
    /// 渲染为border区，世界矩阵需要变换
    BorderRect,
    /// 渲染为边框部分
    Border,
}

// /// vs shader的宏开关
// #[derive(Deref, DerefMut, Default, Debug, Clone, Component)]
// pub struct VSDefines(pub XHashSet<Atom>);

// impl Hash for VSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }

// /// fs shader的宏开关
// #[derive(Deref, DerefMut, Default, Debug, Clone, Component)]
// pub struct FSDefines(pub XHashSet<Atom>);

// impl Hash for FSDefines {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		for i in self.0.iter() {
// 			i.hash(state);
// 		}
// 	}
// }


// #[derive(Clone, Debug, PartialEq, Eq, Component, Default)]
// pub struct StaticIndex {
// 	pub shader: usize,
// 	pub pipeline_state: DefaultKey,
// 	pub vertex_buffer_index: DefaultKey,
// 	pub name: &'static str,
// }

// #[derive(Clone, Component, Deref, Hash)]
// pub struct PipelineMeta(pub Share<ProgramMetaInner>);

#[derive(Clone, Component)]
pub struct PipelineMeta {
    pub program: Share<ProgramMetaInner>,
    pub state: Share<PipelineStateWithHash>,
    pub vert_layout: Share<VertexBufferLayoutWithHash>,
    pub defines: XHashSet<Atom>,
}

impl Hash for PipelineMeta {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.program.hash(state);
        self.state.hash(state);
        self.vert_layout.hash(state);
        for i in self.defines.iter() {
            i.hash(state);
        }
    }
}
