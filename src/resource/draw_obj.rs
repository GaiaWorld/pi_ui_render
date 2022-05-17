//! 与DrawObject相关的单例

use std::{collections::hash_map::Entry, hash::Hash, num::NonZeroU32};

use ordered_float::NotNan;
use pi_ecs::world::FromWorld;
use pi_hash::XHashMap;
use pi_map::{vecmap::VecMap};
use pi_render::{rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, shader::{ShaderId, Shader}, device::RenderDevice, pipeline::RenderPipeline, buffer::Buffer}, components::view::target::RenderTargetKey};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};
use wgpu::{PipelineLayout, ShaderModule};

use crate::{components::{draw_obj::{VSDefines, FSDefines}}, utils::{tools::calc_hash, shader_helper::{create_matrix_group_layout, create_depth_layout, create_camera_layout}}};

/// viewMatrix、projectMatrix 的BindGroupLayout
#[derive(Deref)]
pub struct CameraGroupLayout(pub Share<BindGroupLayout>);

/// worldMatrix 的BindGroupLayout
#[derive(Deref)]
pub struct WolrdMartixGroupLayout(pub Share<BindGroupLayout>);

/// depth 的BindGroupLayout
#[derive(Deref)]
pub struct DepthGroupLayout(pub Share<BindGroupLayout>);

/// depth的Group缓冲
#[derive(Deref)]
pub struct DepthGroup(pub Vec<Share<BindGroup>>);

/// color 的BindGroupLayout
#[derive(Deref)]
pub struct ColorGroupLayout(pub Share<BindGroupLayout>);

/// shader的静态属性
pub struct ShaderStatic {
	pub vs_shader_soruce: ShaderId,
	pub fs_shader_soruce: ShaderId,
	pub bind_group: VecMap<Share<BindGroupLayout>>,
	pub create_shader_info: fn (
		vs_shader_soruce: &ShaderId,
		fs_shader_soruce: &ShaderId,
		vs_defines: &VSDefines, 
		fs_defines: &FSDefines, 
		bind_group_layout: VecMap<Share<BindGroupLayout>>,
		device: &RenderDevice,
		shaders: &XHashMap<ShaderId, Shader>,
	) -> ShaderInfo,
}

pub fn list_share_as_ref<'a, T, I: Iterator<Item=&'a Option<Share<T>>>>(list: I) -> Vec<&'a T> {
	let mut v = Vec::new();
	for r in list {
		if let Some(r) = r {
			v.push(&**r)
		}
	}
	v
}

/// shader的静态属性缓冲
#[derive(Deref, DerefMut, Default)]
pub struct Shaders(pub Vec<ShaderStatic>);

#[derive(Deref, DerefMut, Default)]
pub struct ShaderCatch(pub XHashMap<ShaderId, Shader>);

/// 根据shader的原始代码、defines计算获得
pub struct ShaderInfo {
	pub pipeline_layout: Share<PipelineLayout>,
	pub vs_shader: Share<ShaderModule>,
	pub fs_shader: Share<ShaderModule>,
}

#[derive(Default)]
pub struct ShaderInfoMap(pub XHashMap<u64, Share<ShaderInfo>>);
pub type StateMap = ResMap<PipelineState>;

#[derive(Default)]
pub struct PipelineMap(pub XHashMap<u64, Share<RenderPipeline>>);

pub type VertexBufferLayoutMap = ResMap<VertexBufferLayouts>;

pub type VertexBufferLayouts = Vec<VertexBufferLayout>;

#[derive(Hash, Debug)]
pub struct VertexBufferLayout {
	pub array_stride: wgpu::BufferAddress,
	pub step_mode: wgpu::VertexStepMode,
	pub attributes: Vec<wgpu::VertexAttribute>
}

pub struct ResMap<T> {
	pub map: XHashMap<u64, DefaultKey>,
	pub slot: SlotMap<DefaultKey, T>,
}

impl<T> Default for ResMap<T> {
    fn default() -> Self {
        Self { map: Default::default(), slot: Default::default() }
    }
}

impl<T: Hash> ResMap<T> {
	pub fn get(&self, key: DefaultKey) -> Option<&T> {
		self.slot.get(key)
	}

	pub fn insert(&mut self, value: T) -> DefaultKey {
		match self.map.entry(calc_hash(&value)) {
			Entry::Occupied(r) => r.get().clone(),
			Entry::Vacant(r) => {
				let index = self.slot.insert(value);
				r.insert(index);
				index
			},
		}
	}
}

/// 渲染状态
#[derive(Clone, Debug)]
pub struct PipelineState {
	pub targets: Vec<wgpu::ColorTargetState>,
	pub primitive: wgpu::PrimitiveState,
	pub depth_stencil: Option<wgpu::DepthStencilState>,
	pub multisample: wgpu::MultisampleState,
	pub multiview: Option<NonZeroU32>,
}

impl Hash for PipelineState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.targets.hash(state);
        self.primitive.hash(state);
		match &self.depth_stencil {
			Some(r) => {
				r.format.hash(state);
				r.depth_write_enabled.hash(state);
				r.depth_compare.hash(state);
				r.stencil.hash(state);
				r.bias.constant.hash(state);
				unsafe{NotNan::unchecked_new(r.bias.slope_scale).hash(state)};
				unsafe{NotNan::unchecked_new(r.bias.clamp).hash(state)};
			},
			None => (),
		};
        self.multisample.hash(state);
        self.multiview.hash(state);
    }
}

/// 单位四边形对应的定点buffer和索引buffer
#[derive(Debug)]
pub struct UnitQuadBuffer {
	pub vertex: Share<Buffer>,
	pub index: Share<Buffer>,
}
impl FromWorld for UnitQuadBuffer {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
		let device = world.get_resource::<RenderDevice>().expect("create UnitQuadBuffer need RenderDevice");
        let vertex_data: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];
		let index_data: [u16; 6] = [0, 1, 2, 0, 2, 3];
		let vertex_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
			label: Some("Unit Quad Vertex Buffer"),
			contents: bytemuck::cast_slice(&vertex_data),
			usage: wgpu::BufferUsages::VERTEX,
		});
	
		let index_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
			label: Some("Unit Quad Index Buffer"),
			contents: bytemuck::cast_slice(&index_data),
			usage: wgpu::BufferUsages::INDEX,
		});

		UnitQuadBuffer {
			vertex: Share::new(vertex_buf),
			index: Share::new(index_buf),
		}
    }
}

#[derive(Debug)]
pub struct ShareLayout {
	pub depth: Share<BindGroupLayout>,
	pub matrix: Share<BindGroupLayout>,
	pub camera: Share<BindGroupLayout>,
}

impl FromWorld for ShareLayout {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
		let device = world.get_resource::<RenderDevice>().expect("create ShareLayout need RenderDevice");
		ShareLayout {
			camera: Share::new(create_camera_layout(device)),
			matrix: Share::new(create_matrix_group_layout(device)),
			depth: Share::new(create_depth_layout(device)),
		}
    }
}

pub struct RenderInfo {
	pub rt_key: RenderTargetKey,
}





