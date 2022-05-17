use std::hash::{Hasher, Hash};


use pi_ecs::entity::Id;
use pi_hash::XHashSet;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{bind_group::BindGroup, buffer::Buffer, IndexFormat, pipeline::RenderPipeline};
use pi_share::Share;
use pi_slotmap::DefaultKey;
use wgpu::RenderPass;

use super::pass_2d::Camera;

pub type DrawKey = Id<DrawObject>;

pub struct DrawObject;

#[derive(Debug, Default)]
pub struct DrawState {
    // 一个 Pipeleine
    pub pipeline: Option<Share<RenderPipeline>>,

    // 一堆 UBO
    pub bind_groups: VecMap<Share<BindGroup>>,

    // 一堆 VB
    pub vbs: VecMap<(Share<Buffer>, u64)>,

    // IB 可有 可无
    pub ib: Option<(Share<Buffer>, u64, IndexFormat)>,
}

impl DrawState {
    pub fn draw<'w, 'a>(&'a self, rpass: &'w mut RenderPass<'a>, camera: &'a Camera) {
		// log::info!("draw======={}, {}", self.pipeline.is_some(), self.ib.is_some());
        if let (Some(p), Some(ib)) = (&self.pipeline, &self.ib) {
			rpass.set_pipeline(p);
			if let Some(r) = &camera.bind_group {
				rpass.set_bind_group(0, r, &[]);
			}
			let mut i = 0;
			for r in self.bind_groups.iter() {
				if let Some(group) = r {
					rpass.set_bind_group(i as u32, group, &[]);
				}
				i += 1;
			}
			i = 0;
			for r in self.vbs.iter() {
				if let Some(vertex_buf) = r {
					rpass.set_vertex_buffer(i as u32, (**vertex_buf.0).slice(..));
				}
				i += 1;
			}

			rpass.set_index_buffer((**ib.0).slice(..), ib.2);
			rpass.draw_indexed(0..ib.1 as u32, 0, 0..1);
		}
    }
}

/// 是否使用单位四边形渲染
#[derive(Default, Deref, DerefMut)]
pub struct IsUnitQuad(pub bool);

/// vs shader的宏开关
#[derive(Deref, DerefMut, Default)]
pub struct VSDefines(pub XHashSet<String>);

impl Hash for VSDefines {
	fn hash<H: Hasher>(&self, state: &mut H) {
		for i in self.0.iter() {
			i.hash(state);
		}
	}
}

/// fs shader的宏开关
#[derive(Deref, DerefMut, Default)]
pub struct FSDefines(pub XHashSet<String>);

impl Hash for FSDefines {
	fn hash<H: Hasher>(&self, state: &mut H) {
		for i in self.0.iter() {
			i.hash(state);
		}
	}
}

/// 渲染状态
#[derive(Debug, Deref, DerefMut, Hash)]
pub struct PipelineKey(pub DefaultKey);

#[derive(Debug, Deref, DerefMut, Hash)]
pub struct VertexBufferLayoutKey(pub DefaultKey);

#[derive(Debug, Deref, DerefMut, Hash)]
pub struct ShaderKey(pub usize);

