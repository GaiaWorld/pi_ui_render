//! 与DrawObject相关的单例

use std::{collections::hash_map::Entry, hash::Hash, num::NonZeroU32, borrow::Cow, marker::PhantomData};

use ordered_float::NotNan;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_dirty::LayerDirty;
use pi_ecs::{world::FromWorld, prelude::World, entity::Id};
use pi_hash::XHashMap;
use pi_map::{vecmap::VecMap};
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, shader::{ShaderId, Shader, ShaderProcessor}, device::RenderDevice, pipeline::RenderPipeline, buffer::Buffer, asset::RenderRes, dyn_uniform_buffer::Group, texture::PiRenderDefault};
use pi_share::Share;
use pi_slotmap::{SlotMap, DefaultKey};
use wgpu::{PipelineLayout, ShaderModule, Sampler, DepthStencilState, TextureFormat, CompareFunction, StencilState, DepthBiasState, MultisampleState};

use crate::{components::{draw_obj::{VSDefines, FSDefines, DrawState, DrawGroup}, pass_2d::Pass2D}, utils::{tools::{calc_hash, calc_float_hash}, shader_helper::{create_matrix_group_layout, create_depth_layout, create_view_layout, create_project_layout, create_empty_layout}}};
use pi_render::rhi::dyn_uniform_buffer::BufferGroup;

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
pub struct ColorGroupLayout(pub BindGroupLayout);

/// 每个shader对应一个ShaderId
#[derive(Default, Deref, DerefMut)]
pub struct ShaderMap(XHashMap<&'static str, ShaderId>);

/// 每个渲染对象，关于shader的静态属性
pub struct ShaderStatic {
	pub vs_shader_soruce: ShaderId, // 顶点shader的id
	pub fs_shader_soruce: ShaderId, // 片元shader的id
	pub bind_group_layout: VecMap<BindGroupLayout>, // shader中全部的BindGroup
	// pub bind_group: Groups, // shader中全部的BindGroup
	// pub create_shader_info: fn (
	// 	vs_shader_soruce: &ShaderId,
	// 	fs_shader_soruce: &ShaderId,
	// 	vs_defines: &VSDefines, 
	// 	fs_defines: &FSDefines, 
	// 	bind_group_layout: VecMap<Share<BindGroupLayout>>,
	// 	empty_group_layout: &Share<BindGroupLayout>,
	// 	device: &RenderDevice,
	// 	shaders: &XHashMap<ShaderId, Shader>,
	// ) -> Program,
}


impl ShaderStatic {
	pub fn create_shader_info(
		&self,
		vs_defines: &VSDefines, 
		fs_defines: &FSDefines, 
		device: &RenderDevice,
		shaders: &XHashMap<ShaderId, Shader>,
	) -> Program {
		let processor = ShaderProcessor::default();
		let imports = XHashMap::default();
	
		let vs = processor
				.process(&self.vs_shader_soruce, vs_defines, shaders, &imports)
				.unwrap();
		let vs = vs.get_glsl_source().unwrap();
	
		// // 优化 TODO
		// let mut vs_defines1 = naga::FastHashMap::default();
		// for f in vs_defines.iter() {
		// 	vs_defines1.insert(f.clone(), f.clone());
		// }
	
		// // 优化 TODO
		// let mut fs_defines1 = naga::FastHashMap::default();
		// for  f in fs_defines.iter() {
		// 	fs_defines1.insert(f.clone(), f.clone());
		// }
	
		let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("post_process_vs_shader_module"),
			source: wgpu::ShaderSource::Glsl {
				shader: Cow::Borrowed(vs),
				stage: naga::ShaderStage::Vertex,
				defines: naga::FastHashMap::default() ,
			},
		});
	
		let fs = processor
				.process(&self.fs_shader_soruce, fs_defines, shaders, &imports)
				.unwrap();
		let fs = fs.get_glsl_source().unwrap();
		
		let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("post_process_fs_shader_module"),
			source: wgpu::ShaderSource::Glsl {
				shader: Cow::Borrowed(fs),
				stage: naga::ShaderStage::Fragment,
				defines: naga::FastHashMap::default(),
			},
		});

		let mut layouts: Vec<&wgpu::BindGroupLayout> = Vec::new();
		for i in self.bind_group_layout.iter() {
			if let Some(r) = i {
				layouts.push(r)
			}
		}
		
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("cerate post process pipeline_layout"),
			bind_group_layouts: layouts.as_slice(),
			push_constant_ranges: &[],
		});
		
		Program {
			pipeline_layout: Share::new(pipeline_layout),
			vs_shader: Share::new(vs),
			fs_shader: Share::new(fs),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticIndex {
	pub shader: usize,
	pub pipeline_state: DefaultKey,
	pub vertex_buffer_index: DefaultKey,
	pub name: &'static str,
}

#[derive(Deref)]
pub struct DynBindGroupLayout<T>(pub BindGroupLayout, PhantomData<T>);

impl<T: Group> FromWorld for DynBindGroupLayout<T> {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().expect("RenderDevice is not exist");
		DynBindGroupLayout(T::create_layout(device, true), PhantomData)
    }
}

#[derive(Deref)]
pub struct ColorStaticIndex(pub StaticIndex);

#[derive(Deref)]
pub struct GradientColorStaticIndex(pub StaticIndex);

#[derive(Deref)]
pub struct ImageStaticIndex(pub StaticIndex);

#[derive(Deref)]
pub struct TextStaticIndex(pub StaticIndex);

#[derive(Deref)]
pub struct PosUvVertexLayout(pub DefaultKey);

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
		match self.map.entry(calc_hash(&value, 0)) {
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
	pub targets: Vec<Option<wgpu::ColorTargetState>>,
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

		let ib_key = calc_hash(&index_data, calc_hash(&"index", 0));
		let vb_key = calc_float_hash(&vertex_data, calc_hash(&"vert", 0));
		let uv_key = calc_float_hash(&uv_data, calc_hash(&"vert", 0));
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
	pub depth: BindGroupLayout,
	pub matrix: BindGroupLayout,
	pub view: BindGroupLayout,
	pub project: BindGroupLayout,
	pub empty: BindGroupLayout,
}

impl FromWorld for ShareLayout {
    fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
		let device = world.get_resource::<RenderDevice>().expect("create ShareLayout need RenderDevice");
		ShareLayout {
			project: create_project_layout(device),
			view: create_view_layout(device),
			matrix: create_matrix_group_layout(device),
			depth: create_depth_layout(device),
			empty: create_empty_layout(device),
		}
    }
}

// #[derive(Debug, Clone)]
// pub struct EmptyBind(pub Handle<RenderRes<BindGroup>>);

/// 动态分配的纹理，清屏颜色的bindgroup（透明色）
pub struct DynFboClearColorBindGroup(pub DrawGroup);

pub fn list_share_as_ref<'a, T, I: Iterator<Item=&'a Option<Share<T>>>>(list: I) -> Vec<&'a T> {
	let mut v = Vec::new();
	for r in list {
		if let Some(r) = r {
			v.push(&**r)
		}
	}
	v
}

#[derive(Deref, DerefMut, Default)]
pub struct DynBindGroups(Vec<(Option<BindGroup>, BindGroupLayout, fn(&RenderDevice, &BindGroupLayout, &Buffer) -> BindGroup)>);

// 在DynBindGroups中的索引
pub struct DynBindGroupIndex<T>(usize, PhantomData<T>);
impl<T: BufferGroup + Group> FromWorld for DynBindGroupIndex<T> {
	fn from_world(world: &mut World) -> Self {
		let device = world.get_resource::<RenderDevice>().unwrap();
		let layout = T::create_layout(device, true);

		let groups = world.get_or_insert_resource_mut::<DynBindGroups>();
		groups.push((None, layout, T::create_bind_group ));
		let index= groups.len() - 1;
        Self(index, PhantomData)
	}
}

impl<T> std::ops::Deref for DynBindGroupIndex<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deref, DerefMut)]
pub struct DynUniformBuffer (pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer);

impl FromWorld for DynUniformBuffer {
    fn from_world(world: &mut World) -> Self {
		let limits = world.get_resource::<wgpu::Limits>().unwrap();
        DynUniformBuffer(
			pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer::new(
				Some("DynUniformBuffer".to_string()), 
				limits.min_uniform_buffer_offset_alignment.max(192)))
	}
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

#[derive(Deref, DerefMut)]
pub struct TextTextureGroup(pub Handle<RenderRes<BindGroup>>);

#[derive(Deref, DerefMut)]
pub struct EmptyVertexBuffer (pub Handle<RenderRes<Buffer>>);

impl FromWorld for EmptyVertexBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
		let buffer_assets = world.get_resource::<Share<AssetMgr<RenderRes<Buffer>>>>().unwrap();

		let gradient_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Empty VERTEX Buffer"),
			size: 0,
			usage: wgpu::BufferUsages::VERTEX,
			mapped_at_creation: false,
		});
		
		let key = calc_hash(&"Empty VERTEX Buffer", 0);
		let gradient_buf = buffer_assets.insert(key, RenderRes::new(gradient_buf, 0)).unwrap();

		EmptyVertexBuffer(gradient_buf)
    }
}

/// 常用渲染管线状态
pub struct CommonPipelineState {
	pub common: DefaultKey,
	pub premultiply: DefaultKey,
}


impl FromWorld for CommonPipelineState {
    fn from_world(world: &mut World) -> Self {
        let state_map = world.get_or_insert_resource_mut::<StateMap>();

		Self {
			common: state_map.insert(create_common_pipeline_state()),
			premultiply: state_map.insert(create_premultiply_pipeline_state())
		}
    }
}


pub fn create_common_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![Some(wgpu::ColorTargetState {
			format: wgpu::TextureFormat::pi_render_default(),
			blend: Some(wgpu::BlendState {
				color: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::SrcAlpha,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
			}),
			write_mask: wgpu::ColorWrites::ALL,
		})],
		primitive: wgpu::PrimitiveState {
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			..Default::default()
		},
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth32Float,
			depth_write_enabled: true,
			depth_compare: CompareFunction::GreaterEqual,
			stencil: StencilState::default(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState::default(),
		multiview: None,
	}
}

pub fn create_premultiply_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![Some(wgpu::ColorTargetState {
			format: wgpu::TextureFormat::pi_render_default(),
			blend: Some(wgpu::BlendState {
				color: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
			}),
			write_mask: wgpu::ColorWrites::ALL,
		})],
		primitive: wgpu::PrimitiveState {
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			..Default::default()
		},
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth32Float,
			depth_write_enabled: true,
			depth_compare: CompareFunction::GreaterEqual,
			stencil: StencilState::default(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState::default(),
		multiview: None,
	}
}

// 清屏的DrawObj（wgpu不支持清屏，因此用画矩形的方式模拟清屏）
pub struct ClearDrawObj(pub DrawState, pub StaticIndex);

// 最大视口尺寸（gui中，各渲染共用同一个深度缓冲区， 统计各视口的最大尺寸，用该尺寸作为深度缓冲区的大小）
#[derive(Debug, Default, Clone)]
pub struct MaxViewSize {
	pub width: u32,
	pub height: u32,
}





