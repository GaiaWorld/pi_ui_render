//! 与DrawObject相关的单例

use std::{collections::hash_map::Entry, hash::Hash, num::NonZeroU32};

use ordered_float::NotNan;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_dirty::LayerDirty;
use pi_ecs::{world::FromWorld, prelude::World, entity::Id};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, shader::{ShaderId, Shader}, device::RenderDevice, pipeline::RenderPipeline, buffer::Buffer, asset::RenderRes};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};
use wgpu::{PipelineLayout, ShaderModule, Sampler};

use crate::{components::{draw_obj::{VSDefines, FSDefines, DrawState}, pass_2d::Pass2D}, utils::{tools::{calc_hash, calc_float_hash}, shader_helper::{create_matrix_group_layout, create_depth_layout, create_view_layout, create_project_layout, create_empty_layout}}};

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

/// 每个shader对应一个ShaderId
#[derive(Default, Deref, DerefMut)]
pub struct ShaderMap(XHashMap<&'static str, ShaderId>);

/// 每个渲染对象，关于shader的静态属性
pub struct ShaderStatic {
	pub vs_shader_soruce: ShaderId, // 顶点shader的id
	pub fs_shader_soruce: ShaderId, // 片元shader的id
	pub bind_group: VecMap<Share<BindGroupLayout>>, // shader中全部的BindGroup
	pub create_shader_info: fn (
		vs_shader_soruce: &ShaderId,
		fs_shader_soruce: &ShaderId,
		vs_defines: &VSDefines, 
		fs_defines: &FSDefines, 
		bind_group_layout: VecMap<Share<BindGroupLayout>>,
		empty_group_layout: &Share<BindGroupLayout>,
		device: &RenderDevice,
		shaders: &XHashMap<ShaderId, Shader>,
	) -> Program,
}

/// shader的静态属性缓冲
#[derive(Deref, DerefMut, Default)]
pub struct Shaders(pub Vec<ShaderStatic>);

#[derive(Deref, DerefMut, Default)]
pub struct ShaderCatch(pub XHashMap<ShaderId, Shader>);

/// Program, 根据shader的原始代码、defines计算获得
pub struct Program {
	pub pipeline_layout: Share<PipelineLayout>,
	pub vs_shader: Share<ShaderModule>,
	pub fs_shader: Share<ShaderModule>,
}

#[derive(Default)]
pub struct ShaderInfoMap(pub XHashMap<u64, Share<Program>>);
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
				unsafe{NotNan::new_unchecked(r.bias.slope_scale).hash(state)};
				unsafe{NotNan::new_unchecked(r.bias.clamp).hash(state)};
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
	pub vertex: Handle<RenderRes<Buffer>>,
	pub uv: Handle<RenderRes<Buffer>>,
	pub index: Handle<RenderRes<Buffer>>,
}
impl FromWorld for UnitQuadBuffer {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
		let device = world.get_resource::<RenderDevice>().expect("create UnitQuadBuffer need RenderDevice");
		let buffer_asset_mgr = world.get_resource::<Share<AssetMgr<RenderRes<Buffer>>>>().expect("create UnitQuadBuffer need buffer AssetMgr");
        let vertex_data: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];
		let uv_data: [f32; 8] = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
		let index_data: [u16; 6] = [0, 1, 2, 0, 2, 3];
		let vertex_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
			label: Some("Unit Quad Vertex Buffer"),
			contents: bytemuck::cast_slice(&vertex_data),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let uv_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
			label: Some("Unit Quad UV Buffer"),
			contents: bytemuck::cast_slice(&uv_data),
			usage: wgpu::BufferUsages::VERTEX,
		});
	
		let index_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
			label: Some("Unit Quad Index Buffer"),
			contents: bytemuck::cast_slice(&index_data),
			usage: wgpu::BufferUsages::INDEX,
		});

		let ib_key = calc_hash(&index_data);
		let vb_key = calc_float_hash(&vertex_data);
		let uv_key = calc_float_hash(&uv_data);
		AssetMgr::cache(&buffer_asset_mgr, vb_key, RenderRes::new(vertex_buf, 32));
		AssetMgr::cache(&buffer_asset_mgr, uv_key, RenderRes::new(uv_buf, 32));
		AssetMgr::cache(&buffer_asset_mgr, ib_key, RenderRes::new(index_buf, 12));

		UnitQuadBuffer {
			vertex: AssetMgr::get(&buffer_asset_mgr, &vb_key).unwrap(),
			uv: AssetMgr::get(&buffer_asset_mgr, &uv_key).unwrap(),
			index: AssetMgr::get(&buffer_asset_mgr, &ib_key).unwrap(),
		}
    }
}

#[derive(Debug)]
pub struct ShareLayout {
	pub depth: Share<BindGroupLayout>,
	pub matrix: Share<BindGroupLayout>,
	pub view: Share<BindGroupLayout>,
	pub project: Share<BindGroupLayout>,
	pub empty: Share<BindGroupLayout>,
}

impl FromWorld for ShareLayout {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
		let device = world.get_resource::<RenderDevice>().expect("create ShareLayout need RenderDevice");
		ShareLayout {
			project: Share::new(create_project_layout(device)),
			view: Share::new(create_view_layout(device)),
			matrix: Share::new(create_matrix_group_layout(device)),
			depth: Share::new(create_depth_layout(device)),
			empty: Share::new(create_empty_layout(device)),
		}
    }
}

#[derive(Debug, Clone)]
pub struct EmptyBind(pub Handle<RenderRes<BindGroup>>);

/// 清屏颜色的bindgroup（用户设置）
pub struct ClearColorBindGroup(pub Option<Handle<RenderRes<BindGroup>>>);

impl FromWorld for ClearColorBindGroup {
    fn from_world(_world: &mut pi_ecs::prelude::World) -> Self {
        ClearColorBindGroup(None)
    }
}

/// 动态分配的纹理，清屏颜色的bindgroup（透明色）
pub struct DynFboClearColorBindGroup(pub Handle<RenderRes<BindGroup>>);

pub fn list_share_as_ref<'a, T, I: Iterator<Item=&'a Option<Share<T>>>>(list: I) -> Vec<&'a T> {
	let mut v = Vec::new();
	for r in list {
		if let Some(r) = r {
			v.push(&**r)
		}
	}
	v
}

pub struct CommonSampler {
	pub default: Sampler,
	pub pointer: Sampler,
}

impl FromWorld for CommonSampler {
    fn from_world(world: &mut World) -> Self {
		let device = world.get_resource::<RenderDevice>().unwrap();
        Self {
			default: (**device).create_sampler(&wgpu::SamplerDescriptor {
				label: Some("default sampler"),
				address_mode_u: wgpu::AddressMode::ClampToEdge,
				address_mode_v: wgpu::AddressMode::ClampToEdge,
				address_mode_w: wgpu::AddressMode::ClampToEdge,
				mag_filter: wgpu::FilterMode::Linear,
				min_filter: wgpu::FilterMode::Linear,
				mipmap_filter: wgpu::FilterMode::Linear,
				..Default::default()
			}),
			pointer: (**device).create_sampler(&wgpu::SamplerDescriptor {
				label: Some("default sampler"),
				address_mode_u: wgpu::AddressMode::ClampToEdge,
				address_mode_v: wgpu::AddressMode::ClampToEdge,
				address_mode_w: wgpu::AddressMode::ClampToEdge,
				mag_filter: wgpu::FilterMode::Nearest,
				min_filter: wgpu::FilterMode::Nearest,
				mipmap_filter: wgpu::FilterMode::Nearest,
				..Default::default()
			}),
		}
    }
}

/// 将pass2d组织为层的结构
#[derive(Deref, Default, DerefMut)]
pub struct LayerPass2D (LayerDirty<Id<Pass2D>>);

pub struct CopyFboToScreen(pub DrawState);



