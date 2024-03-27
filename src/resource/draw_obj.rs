//! 与DrawObject相关的资源
use std::{collections::hash_map::Entry, hash::Hash, marker::PhantomData, num::NonZeroU32, sync::atomic::{Ordering, AtomicUsize}, borrow::Cow};
use std::ops::Deref;

use bevy_ecs::{
    prelude::{FromWorld, World},
    system::Resource
};
use ordered_float::NotNan;
use pi_assets::{asset::{Handle, Asset}, mgr::AssetMgr};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_hash::{XHashMap, XHashSet};
use pi_key_alloter::KeyData;
use pi_map::vecmap::VecMap;
use pi_render::{
    renderer::{draw_obj::DrawBindGroup, vertices::{RenderVertices, EVerticesBufferUsage}},
    rhi::{
        asset::{RenderRes, TextureRes, AssetWithId},
        bind_group::BindGroup,
        bind_group_layout::BindGroupLayout,
        buffer::Buffer,
        device::RenderDevice,
        dyn_uniform_buffer::{GroupAlloter, BufferGroup},
        pipeline::RenderPipeline,
        shader::{ShaderMeta, ShaderProgram},
        texture::PiRenderDefault, RenderQueue,
    },
};
use pi_render::rhi::shader::AsLayoutEntry;
use pi_render::rhi::shader::BindLayout;
use pi_share::Share;
use pi_slotmap::{DefaultKey, SlotMap, SecondaryMap};
use wgpu::{
    BlendState, CompareFunction, DepthBiasState, DepthStencilState, Limits, MultisampleState, PipelineLayout, Sampler, ShaderModule, StencilState,
    TextureFormat, util::{BufferInitDescriptor, DeviceExt}, ShaderStages, BindingType, TextureSampleType, TextureViewDimension, SamplerBindingType, BindGroupEntry, TextureDescriptor, Extent3d, TextureViewDescriptor, RenderPass,
};
use pi_render::rhi::shader::Input;

use crate::{
    components::{draw_obj::{DrawState, PipelineMeta}, pass_2d::{CacheTarget, InstanceDrawState}, calc::WorldMatrix},
	shader1::meterial::{CameraBind, MeterialBind, ProjectUniform, ViewUniform},
    shader::{
        depth::{DepthBind, DepthUniform},
        ui_meterial::UiMaterialBind, color::PositionVert,
    },
    system::draw_obj::clear_draw_obj::create_clear_pipeline_state,
    utils::{
        shader_helper::{create_depth_layout, create_empty_layout, create_matrix_group_layout, create_project_layout, create_view_layout},
        tools::{calc_float_hash, calc_hash, calc_hash_slice},
    }, shader1::RenderInstances,
};

// /// 一组纹理的绑定， 用于实例化渲染
// #[derive(Debug, Default)]
// pub struct TexturesBindTemp {
// 	pub texture_indexs: SecondaryMap<DefaultKey, (u32, u32)>, // 纹理对应BindGroup在texture_bind_groups中的索引， 以及纹理在该bindgroup中的binding
// }

// impl TexturesBindItem {

// 	fn clear(&mut self) {
// 		self.texture_bind_groups.clear();
// 		self.texture_indexs.clear();
// 	}
// }

// pub struct TexturesBind {
// 	pub max_bind: usize, // 一个BindGroup最大的绑定数量
// 	pub texture_id_alloc: Share<pi_key_alloter::KeyAlloter>, // 每个纹理应该分配一个索引，方便后续进行纹理对比
// 	// pub cur_bindgroups: Vec<BindGroup>, // 当前渲染，根据该数据设置bindgroup
// 	// pub prepare_bindgroups: Vec<BindGroup>, // 空闲的BindGroup， 通常用于本次改变时， 准备当前数据（准备完成后， 需要跟cur进行交换， 使得渲染数据得以更新）
// }

// 纹理key分配器
#[derive(Debug, Default, Resource, Clone, Deref)]
pub struct TextureKeyAlloter(pub Share<pi_key_alloter::KeyAlloter>);

// 批处理纹理
pub struct BatchTexture {
	// max_bind: usize,
	
	temp_texture_indexs: SecondaryMap<DefaultKey, u32>, // 纹理在该bindgroup中的binding, 以及本利本身
	temp_textures: Vec<(Handle<AssetWithId<TextureRes>>, Share<wgpu::Sampler>)>,

	// group_layouts: Vec<wgpu::BindGroupLayout>,
	group_layout: wgpu::BindGroupLayout,
	pub(crate) default_texture_view: wgpu::TextureView,
	pub(crate) default_sampler: wgpu::Sampler,
	pub default_texture_group: wgpu::BindGroup,
}

impl BatchTexture {
	const BINDING_COUNT: u32 = 14;
	pub fn new(device: &wgpu::Device) -> Self {
		let mut entry = Vec::with_capacity(Self::BINDING_COUNT as usize);
		for i in 0..Self::BINDING_COUNT {
			entry.push(wgpu::BindGroupLayoutEntry {
				binding: i * 2,
				visibility: ShaderStages::FRAGMENT,
				ty: BindingType::Texture {
					sample_type: TextureSampleType::Float { filterable: true },
					view_dimension: TextureViewDimension::D2,
					multisampled: false,
				},
				count: None,
			});
			entry.push(wgpu::BindGroupLayoutEntry {
				binding: i * 2 + 1,
				visibility: ShaderStages::FRAGMENT,
				ty: BindingType::Sampler(SamplerBindingType::Filtering),
				count: None,
			});
		}
		// let mut layouts: Vec<wgpu::BindGroupLayout> = Vec::new();
		// for i in 1..Self::BINDING_COUNT + 1 {
		// 	layouts.push(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		// 		label: Some("batch texture layout"),
		// 		entries: &entry[0..i as usize * 2],
		// 	}));
		// }
		let group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("batch texture layout"),
			entries: &entry[..],
		});
		let default_texture = device.create_texture(&TextureDescriptor {
			label: Some("default texture"),
			size: Extent3d { width: 4, height: 4, depth_or_array_layers: 1 },
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::R8Unorm,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});
		let texture_view = default_texture.create_view(&TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("default sampler"),
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Linear,
			..Default::default()
		});

		let default_texture_group = Self::take_group1(device, &Vec::new(), &texture_view, &sampler, &group_layout);
	
		let r = Self {
			// max_bind: Self::BINDING_COUNT,
			temp_texture_indexs: SecondaryMap::new(),
			temp_textures: Vec::new(),

			group_layout,
			default_texture_view: texture_view,
			default_sampler: sampler,
			default_texture_group,
			// common_sampler: CommonSampler::new(device),
		};
		r
	}
	/// push一张纹理，返回纹理索引， 当纹理数量达到max_bind限制时， 会返回一个wgpu::BindGroup，并先清空当前所有的临时数据， 再添加数据
	/// 注意， 目前同一张纹理只能用同一种采样方式，使用不同的采样样式push纹理，将不会覆盖之前的（主要原因是目前gui并没有不同采样方式的需求）
	pub fn push(&mut self, texture: &Handle<AssetWithId<TextureRes>>, sampler: &Share<Sampler>, device: &wgpu::Device) -> (usize, Option<wgpu::BindGroup>) {
		let mut index = self.temp_textures.len();

		if let Some(r) = self.temp_texture_indexs.get(DefaultKey::from(texture.id)) {
			return (*r as usize, None); //
		}

		let group;
		if index == Self::BINDING_COUNT as usize {
			group = self.take_group(device);
			index = 0;
		} else {
			group = None;
		}

		self.temp_texture_indexs.insert(DefaultKey::from(texture.id), index as u32);
		self.temp_textures.push((texture.clone(), sampler.clone()));

		(index, group)
	}

	/// 将当前的临时数据立即创建一个bindgroup，并返回
	pub fn take_group(&mut self, device: &wgpu::Device) -> Option<wgpu::BindGroup> {
		let group = Some(Self::take_group1(device, &self.temp_textures, &self.default_texture_view, &self.default_sampler, &self.group_layout));
		// 清理临时数据
		self.temp_texture_indexs.clear();
		self.temp_textures.clear();
		group
	}

	/// 将当前的临时数据立即创建一个bindgroup，并返回
	fn take_group1(device: &wgpu::Device, temp_textures: &Vec<(Handle<AssetWithId<TextureRes>>, Share<wgpu::Sampler>)>, default_texture_view: &wgpu::TextureView, default_sampler: &wgpu::Sampler, group_layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
		let mut entrys = Vec::with_capacity(Self::BINDING_COUNT as usize * 2);
		for (binding, (texture, sampler)) in temp_textures.iter().enumerate() {
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2) as u32,
					resource: wgpu::BindingResource::TextureView(&texture.texture_view) ,
				}
			);
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2 + 1) as u32,
					resource: wgpu::BindingResource::Sampler(&**sampler) ,
				}
			);
		}

		
		for binding in temp_textures.len()..Self::BINDING_COUNT as usize {
			// log::warn!("len===================={:?}", (self.temp_textures.len(), binding));
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2) as u32,
					resource: wgpu::BindingResource::TextureView(default_texture_view) ,
				}
			);
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2 + 1) as u32,
					resource: wgpu::BindingResource::Sampler(default_sampler) ,
				}
			);
		}
		
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("batch texture bindgroup"),
			layout: group_layout, //&self.group_layouts[self.temp_textures.len() - 1],
			entries: entrys.as_slice(),
		})
	}

	/// 将当前的临时数据立即创建一个bindgroup，并返回
	pub fn create_group(&self, device: &wgpu::Device, texture: &wgpu::TextureView, sampler: &wgpu::Sampler) -> wgpu::BindGroup {
		let mut entrys = Vec::with_capacity(Self::BINDING_COUNT as usize * 2);
		entrys.push(
			BindGroupEntry {
				binding: (0 * 2) as u32,
				resource: wgpu::BindingResource::TextureView(texture) ,
			}
		);
		entrys.push(
			BindGroupEntry {
				binding: (0 * 2 + 1) as u32,
				resource: wgpu::BindingResource::Sampler(&sampler) ,
			}
		);
		for binding in 1..Self::BINDING_COUNT as usize {
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2) as u32,
					resource: wgpu::BindingResource::TextureView(&self.default_texture_view) ,
				}
			);
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2 + 1) as u32,
					resource: wgpu::BindingResource::Sampler(&self.default_sampler) ,
				}
			);
		}
		
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("batch texture bindgroup"),
			layout: &self.group_layout, //&self.group_layouts[self.temp_textures.len() - 1],
			entries: entrys.as_slice(),
		})
	}
}

#[derive(Resource)]
pub struct InstanceContext {
	pub vert: RenderVertices,

	vs: wgpu::ShaderModule,
	fs: wgpu::ShaderModule,

	pipeline_cache: XHashMap<u64, Share<wgpu::RenderPipeline>>,
	pub common_blend_state_hash: u64,
	pub premultiply_blend_state_hash: u64,

	pub common_pipeline: Share<wgpu::RenderPipeline>,
	pub premultiply_pipeline: Share<wgpu::RenderPipeline>,
	pub clear_pipeline: Share<wgpu::RenderPipeline>,

	pub instance_data: RenderInstances,
	pub instance_buffer: Option<(wgpu::Buffer, usize)>,

	// // 深度buffer
	pub depth_data: RenderInstances,
	pub depth_buffer: Option<(wgpu::Buffer, usize)>,

	pub batch_texture: BatchTexture,

	// sdf纹理(由于实例数据槽位有限， text的数据填充后没有空间放置纹理索引， 因此这里将文字纹理单独放在group中， 文字采样固定纹理)
	pub sdf2_texture_group: Option<Share<wgpu::BindGroup>>,
	pub sdf2_texture_layout: wgpu::BindGroupLayout,
	pub camera_alloter: ShareGroupAlloter<CameraGroup>,

	pub pipeline_layout: PipelineLayout,

	pub default_camera: BufferGroup,
}

impl InstanceContext {
	pub fn draw<'a>(&'a self, rp: &mut RenderPass<'a>, instance_draw: &'a InstanceDrawState) {
		rp.set_pipeline(match &instance_draw.pipeline {
			Some(r) => &**r,
			None => &*self.common_pipeline,
		});
		if let Some(texture) = &self.sdf2_texture_group {
			rp.set_bind_group(2, texture, &[]);
		};
		if let Some(texture) = &instance_draw.texture_bind_group {
			rp.set_bind_group(1, texture, &[]);
		};
		
		rp.set_vertex_buffer(0, self.vert.slice());
		// rp.set_vertex_buffer(1, depth_vert.buffer().slice(depth_vert.range()));
		rp.set_vertex_buffer(1, self.instance_buffer.as_ref().unwrap().0.slice(0..instance_draw.instance_data_range.end as u64));
		// rp.set_vertex_buffer(18, param.instance_draw.instance_buffer.as_ref().unwrap().0.slice(..));

		rp.draw(0..6, instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32)

		

	}
}

impl FromWorld for InstanceContext {
	
    fn from_world(world: &mut World) -> Self {
		world.init_resource::<UnitQuadBuffer>();
		let world = world.cell();
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let mut group_center = world.get_resource_mut::<GroupAlloterCenter>().unwrap();

        let vertex_data: [f32; 12] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0];
        let vertex_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
            label: Some("Unit Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

		let batch_texture = BatchTexture::new(&**device);


        let limits = group_center.limits();
        let min_alignment = limits.min_uniform_buffer_offset_alignment;
        let max_binding_size = limits.max_uniform_buffer_binding_size;

        let camera_entry = CameraBind::as_layout_entry(wgpu::ShaderStages::VERTEX_FRAGMENT);
		let uniform_layout = Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: None,
			entries: &[camera_entry],
		}));
        let alloter = Share::new(
            GroupAlloter::new(
                Some("uniform group".to_string()),
                min_alignment,
                max_binding_size,
                None,
                vec![CameraBind::as_layout_entry(wgpu::ShaderStages::VERTEX_FRAGMENT)],
                uniform_layout.clone(),
            )
            .unwrap(),
        );
        group_center.add_alloter(alloter.clone());
		let camera_alloter:  ShareGroupAlloter<CameraGroup> = ShareGroupAlloter {
            alloter: alloter,
            group_index: CameraBind::set(),
            mark: PhantomData,
        };
		

		let text_texture_layout =(***device).create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: None,
			entries: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: ShaderStages::FRAGMENT,
				ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false },
				count: None,
			},
			wgpu::BindGroupLayoutEntry {
				binding: 1,
				visibility: ShaderStages::FRAGMENT,
				ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false },
				count: None,
			},
			wgpu::BindGroupLayoutEntry {
				binding: 2,
				visibility: ShaderStages::FRAGMENT,
				ty: BindingType::Sampler(SamplerBindingType::Filtering),
				count: None,
			}],
		});
		// let default_texture_group = Share::new((***device).create_bind_group(&wgpu::BindGroupDescriptor {
		// 	label: Some("default text texture bindgroup"),
		// 	layout: &text_texture_layout,
		// 	entries: &[
		// 		BindGroupEntry {
		// 			binding: 0,
		// 			resource: wgpu::BindingResource::TextureView(&batch_texture.default_texture_view) ,
		// 		},
		// 		BindGroupEntry {
		// 			binding: 1,
		// 			resource: wgpu::BindingResource::TextureView(&batch_texture.default_texture_view) ,
		// 		},
		// 		BindGroupEntry {
		// 			binding: 2,
		// 			resource: wgpu::BindingResource::Sampler(&batch_texture.default_sampler) ,
		// 		}
		// 	],
		// }));



		// pipeline
		let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_vs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/shader.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });
        let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/shader.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: naga::FastHashMap::default(),
            },
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_shader"),
            bind_group_layouts: &[&*uniform_layout, &batch_texture.group_layout, &text_texture_layout],
            push_constant_ranges: &[],
        });

		let common_blend_state_hash = calc_hash(&CommonBlendState::NORMAL, 0);
		let common_pipeline = Share::new(create_render_pipeline(&device, &pipeline_layout, &vs, &fs, Some(CommonBlendState::NORMAL), CompareFunction::GreaterEqual));

		let premultiply_blend_state_hash = calc_hash(&CommonBlendState::PREMULTIPLY, 0);
		let premultiply_pipeline = Share::new(create_render_pipeline(&device, &pipeline_layout, &vs, &fs, Some(CommonBlendState::PREMULTIPLY), CompareFunction::GreaterEqual));

		let clear_blend_state_hash = calc_hash(&CompareFunction::Always, calc_hash(&CommonBlendState::NORMAL, 0));
		let clear_pipeline = Share::new(create_render_pipeline(&device, &pipeline_layout, &vs, &fs, Some(BlendState {
			color: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
			alpha: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
		}), CompareFunction::Always));

		let mut pipeline_cache = XHashMap::default();
		pipeline_cache.insert(common_blend_state_hash, common_pipeline.clone());
		pipeline_cache.insert(premultiply_blend_state_hash, premultiply_pipeline.clone());
		pipeline_cache.insert(clear_blend_state_hash, clear_pipeline.clone());

		let view_project = WorldMatrix::default().0;
		let mut default_camera = camera_alloter.alloc();
		let _ = default_camera.set_uniform(&ProjectUniform(view_project.as_slice()));
		let _ = default_camera.set_uniform(&ViewUniform(view_project.as_slice()));

		
		Self {
			vert: RenderVertices {
				slot: PositionVert::location(),
				buffer: EVerticesBufferUsage::Temp(Share::new(vertex_buf)),
				buffer_range: None,
				size_per_value: 8,
			},
			vs,
			fs,
			pipeline_cache,
			common_blend_state_hash,
			premultiply_blend_state_hash,
			common_pipeline,
			premultiply_pipeline,
			clear_pipeline,
			instance_data: RenderInstances::new(MeterialBind::SIZE, 200 * MeterialBind::SIZE),
			instance_buffer: None,

			batch_texture,

			depth_data: RenderInstances::new(4, 0),
			depth_buffer: None,

			sdf2_texture_group: None,
			sdf2_texture_layout: text_texture_layout,
			camera_alloter,
			pipeline_layout,
			default_camera,
		}
    }
	
	
}

impl InstanceContext {
	pub fn get_or_create_pipeline(&mut self, device: &RenderDevice, blend_state: wgpu::BlendState) -> Share<wgpu::RenderPipeline> {
		let blend_state_hash = calc_hash(&blend_state, 0);
		match self.pipeline_cache.entry(blend_state_hash) {
			Entry::Occupied(r) => r.get().clone(),
			Entry::Vacant(r) => {
				let pipeline = Share::new(create_render_pipeline(&device, &self.pipeline_layout, &self.vs, &self.fs, Some(blend_state), CompareFunction::GreaterEqual));
				r.insert(pipeline.clone());
				pipeline
			},
		}
	}

	pub fn update(&mut self, device: &RenderDevice, queue: &RenderQueue) {
		log::trace!("update instance_buffer={:?}", &self.instance_data.dirty_range);
		Self::update1(device, queue, &mut self.instance_data, &mut self.instance_buffer);
		// Self::update1(device, queue, &mut self.depth_data, &mut self.depth_buffer);
		
	}

	pub fn update1(device: &RenderDevice, queue: &RenderQueue, instance_data: &mut RenderInstances, instance_buffer: &mut Option<(wgpu::Buffer, usize)>) {
		log::trace!("update instance_buffer={:?}", &instance_data.dirty_range);
		if instance_data.dirty_range.len() != 0 {
			if let Some((buffer, size)) = &instance_buffer {
				if *size >= instance_data.dirty_range.end {
					queue.write_buffer(
						buffer,
						instance_data.dirty_range.start as u64,
						&instance_data.data()[instance_data.dirty_range.clone()],
					);
					return;
				}

			}

			*instance_buffer = Some(((***device).create_buffer_init(&BufferInitDescriptor {
				label: Some("instance_buffer"),
				usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
				contents: instance_data.data(),
			}), instance_data.data().len()));
			
			// log::trace!("create instance_buffer={:?}", instance_data.data());
			instance_data.dirty_range = std::usize::MAX..std::usize::MAX;
		}
	}
}

use super::RenderObjType;

// /// depth 的BindGroupLayout
// #[derive(Deref, Resource)]
// pub struct DepthGroupLayout(pub Share<BindGroupLayout>);

/// depth的Group缓冲
#[derive(Resource)]
pub struct DepthGroup;

/// pos 和uv在同一个buffer中
#[derive(Deref, Resource)]
pub struct PosUv2VertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for PosUv2VertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout_p_v2()))
    }
}

/// pos和uv在不同buffer中
#[derive(Deref, Resource)]
pub struct PosUv1VertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for PosUv1VertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout_p_v1()))
    }
}

#[derive(Deref, Resource)]
pub struct PosVertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for PosVertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout()))
    }
}

#[derive(Deref, Resource)]
pub struct PosUvColorVertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for PosUvColorVertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout_p_v_c()))
    }
}


#[derive(Deref, Resource)]
pub struct PosColorVertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for PosColorVertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout_p_c()))
    }
}

#[derive(Deref, Resource)]
pub struct Sdf2VertexLayout(pub Share<VertexBufferLayoutWithHash>);
impl FromWorld for Sdf2VertexLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let mut catch = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        Self(catch.vert_layout(create_vertex_buffer_layout_sdf2()))
    }
}

#[derive(Debug)]
pub struct VertexBufferLayoutWithHash {
    pub value: VertexBufferLayouts,
    pub hash: u64,
}

impl std::ops::Deref for VertexBufferLayoutWithHash {
    type Target = VertexBufferLayouts;

    fn deref(&self) -> &Self::Target { &self.value }
}

impl Hash for VertexBufferLayoutWithHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.hash.hash(state); }
}

#[derive(Debug, Deref, Resource, Default)]
pub struct DrawObjDefaults(pub VecMap<DrawObjDefault>);

#[derive(Debug)]
pub struct DrawObjDefault {
    pub blend_state: BlendState,
}

impl DrawObjDefault {
    pub fn add(world: &mut World, ty: RenderObjType, state: DrawObjDefault) {
        let mut drawobj_defaults = match world.get_resource_mut::<DrawObjDefaults>() {
            Some(r) => r,
            None => {
                world.insert_resource(DrawObjDefaults::default());
                world.get_resource_mut::<DrawObjDefaults>().unwrap()
            }
        };
        drawobj_defaults.insert(*ty, state);
    }
}


#[derive(Debug)]
pub struct PipelineStateWithHash {
    pub state: PipelineState,
    pub hash: u64,
}

impl std::ops::Deref for PipelineStateWithHash {
    type Target = PipelineState;

    fn deref(&self) -> &Self::Target { &self.state }
}

impl Hash for PipelineStateWithHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.hash.hash(state); }
}


#[derive(Resource)]
pub struct ShaderInfoCache {
    // 缓冲BindGroupLayout
    pub bind_group_layout: XHashMap<u64, Share<BindGroupLayout>>,

    pub pipeline_state: XHashMap<u64, Share<PipelineStateWithHash>>,
    pub common: Share<PipelineStateWithHash>,
    pub common_no_depth: Share<PipelineStateWithHash>,
    pub premultiply: Share<PipelineStateWithHash>,
    pub clear: Share<PipelineStateWithHash>,

    pub vert_layout: XHashMap<u64, Share<VertexBufferLayoutWithHash>>,
}

impl Default for ShaderInfoCache {
    fn default() -> Self {
        let clear = create_clear_pipeline_state();
        let common = create_common_pipeline_state();
        let premultiply = create_premultiply_pipeline_state();
        let mut common_no_depeth = common.clone();
        common_no_depeth.depth_stencil = None;

        let clear_hash = calc_hash(&clear, 0);
        let common_hash = calc_hash(&common, 0);
        let common_no_depeth_hash = calc_hash(&common_no_depeth, 0);
        let premultiply_hash = calc_hash(&premultiply, 0);

        let clear = Share::new(PipelineStateWithHash {
            hash: clear_hash,
            state: clear,
        });
        let common = Share::new(PipelineStateWithHash {
            hash: common_hash,
            state: common,
        });
        let common_no_depeth = Share::new(PipelineStateWithHash {
            hash: common_no_depeth_hash,
            state: common_no_depeth,
        });
        let premultiply = Share::new(PipelineStateWithHash {
            hash: premultiply_hash,
            state: premultiply,
        });

        let mut pipeline_state = XHashMap::default();
        pipeline_state.insert(clear_hash, clear.clone());
        pipeline_state.insert(common_hash, common.clone());
        pipeline_state.insert(premultiply_hash, premultiply.clone());
        Self {
            bind_group_layout: Default::default(),
            pipeline_state,
            common,
            common_no_depth: common_no_depeth,
            premultiply,
            clear,
            vert_layout: Default::default(),
        }
    }
}

impl ShaderInfoCache {
    pub fn bind_group_layout(&mut self, entrys: &[wgpu::BindGroupLayoutEntry], device: &PiRenderDevice) -> Share<BindGroupLayout> {
        let hash = calc_hash_slice(entrys, 0);
        match self.bind_group_layout.entry(hash) {
            Entry::Occupied(r) => r.get().clone(),
            Entry::Vacant(r) => r
                .insert(Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: entrys,
                })))
                .clone(),
        }
    }

    pub fn pipeline_state(&mut self, state: PipelineState) -> Share<PipelineStateWithHash> {
        let hash = calc_hash(&state, 0);
        match self.pipeline_state.entry(hash) {
            Entry::Occupied(r) => r.get().clone(),
            Entry::Vacant(r) => r.insert(Share::new(PipelineStateWithHash { state, hash })).clone(),
        }
    }

    pub fn vert_layout(&mut self, value: VertexBufferLayouts) -> Share<VertexBufferLayoutWithHash> {
        let hash = calc_hash_slice(&value, 0);
        match self.vert_layout.entry(hash) {
            Entry::Occupied(r) => r.get().clone(),
            Entry::Vacant(r) => r.insert(Share::new(VertexBufferLayoutWithHash { value, hash })).clone(),
        }
    }
}

// pub program: Share<ProgramMetaInner>,
//     pub state: Share<PipelineStateWithHash>,
//     pub vert_layout: Share<VertexBufferLayout>,
//     pub defines: XHashSet<Atom>,


/// 每个渲染对象，关于shader的静态属性
#[derive(Resource)]
pub struct ProgramMetaRes<T: ShaderProgram>(Share<ProgramMetaInner>, PhantomData<T>);

impl<T: ShaderProgram> std::ops::Deref for ProgramMetaRes<T> {
    type Target = Share<ProgramMetaInner>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: ShaderProgram> FromWorld for ProgramMetaRes<T> {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();

        let world = world.cell();

        let mut shader_info = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        let device = world.get_resource::<PiRenderDevice>().unwrap();

        let meta = T::create_meta();
        // // depth不使用动态偏移
        // if let Some(depth_entry) = meta.bindings.bind_group_entrys.get_mut(DepthBind::set() as usize) {
        //     if depth_entry.len() == 1 {
        //         if let BindingType::Buffer { has_dynamic_offset, .. } = &mut depth_entry[0].ty {
        //             *has_dynamic_offset = false;
        //         }
        //     }
        // }
        let mut vert_layouts = Vec::with_capacity(meta.ins.0.len());
        for i in meta.ins.0.iter() {
            let (format, size) = match i.format.as_str() {
                "vec4" => (wgpu::VertexFormat::Float32x4, 16),
                "vec3" => (wgpu::VertexFormat::Float32x3, 12),
                "vec2" => (wgpu::VertexFormat::Float32x2, 8),
                "float" => (wgpu::VertexFormat::Float32, 4),
                "ivec4" => (wgpu::VertexFormat::Sint32x4, 16),
                "ivec3" => (wgpu::VertexFormat::Sint32x3, 12),
                "ivec2" => (wgpu::VertexFormat::Sint32x2, 8),
                "int" => (wgpu::VertexFormat::Sint32, 4),
                "uvec4" => (wgpu::VertexFormat::Uint32x4, 16),
                "uvec3" => (wgpu::VertexFormat::Uint32x3, 12),
                "uvec2" => (wgpu::VertexFormat::Uint32x2, 8),
                "uint" => (wgpu::VertexFormat::Uint32, 4),
                r => panic!("vert format invalid, {:?}", r),
            };
            vert_layouts.push(VertexBufferLayout {
                array_stride: size as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: vec![wgpu::VertexAttribute {
                    format,
                    offset: 0,
                    shader_location: i.location,
                }],
            });
        }

        let mut bind_group_layout = VecMap::new();
        for (index, item) in meta.bindings.bind_group_entrys.iter().enumerate() {
            if let Some(r) = item {
                bind_group_layout.insert(index, shader_info.bind_group_layout(r.as_slice(), &device));
            }
        }

        let hash = calc_hash(&meta, 0);
        Self(
            Share::new(ProgramMetaInner {
                bind_group_layout: bind_group_layout,
                shader_meta: meta,
                vert_layout: vert_layouts,
                hash,
            }),
            PhantomData,
        )
        // log::warn!("shader_static_map.0=================={:?}, {:p}", shader_static_map.0.len(), &shader_static_map.0);

        // // 插入背景颜色shader的索引
        // let shader_index = shader_static_map.0.len() - 1;
        // command.insert_resource(ColorStaticIndex(StaticIndex {
        // 	shader: shader_index,
        // 	pipeline_state: common_state.common,
        // 	vertex_buffer_index,
        // 	name: COLOR_PIPELINE,
        // }));

        // command.insert_resource(GradientColorStaticIndex(StaticIndex {
        // 	shader: shader_index,
        // 	pipeline_state: common_state.common,
        // 	vertex_buffer_index: vertex_buffer_index1,
        // 	name: COLOR_PIPELINE,
        // }));
    }
}

pub struct ProgramMetaInner {
    pub bind_group_layout: VecMap<Share<BindGroupLayout>>, // shader中全部的BindGroup
    pub shader_meta: ShaderMeta,
    pub vert_layout: Vec<VertexBufferLayout>,
    pub hash: u64,
}

impl std::hash::Hash for ProgramMetaInner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.hash.hash(state); }
}

impl ProgramMetaInner {
    pub fn create_program(&self, defines: &XHashSet<Atom>, device: &RenderDevice) -> Program {
        // let processor = ShaderProcessor::default();
        // let imports = XHashMap::default();

        // let vs = processor
        // 		.process(&self.vs_shader_soruce, vs_defines, shaders, &imports)
        // 		.unwrap();
        // let vs = vs.get_glsl_source().unwrap();

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
        let vs = self.shader_meta.create_shader_module(device, defines, naga::ShaderStage::Vertex);
        let fs = self.shader_meta.create_shader_module(device, defines, naga::ShaderStage::Fragment);
        // std::fs::write("out.vert", &vs_code);
        // std::fs::write("out.frag", &fs_code);

        // let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: Some(&self.shader_meta.name),
        //     source: wgpu::ShaderSource::Glsl {
        //         shader: Cow::Borrowed(vs_code.as_str()),
        //         stage: naga::ShaderStage::Vertex,
        //         defines: naga::FastHashMap::default(),
        //     },
        // });

        // let fs = processor
        // 		.process(&self.fs_shader_soruce, fs_defines, shaders, &imports)
        // 		.unwrap();
        // let fs = fs.get_glsl_source().unwrap();
        // let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: Some(&self.shader_meta.name),
        //     source: wgpu::ShaderSource::Glsl {
        //         shader: Cow::Borrowed(fs_code.as_str()),
        //         stage: naga::ShaderStage::Fragment,
        //         defines: naga::FastHashMap::default(),
        //     },
        // });

        let mut layouts: Vec<&wgpu::BindGroupLayout> = Vec::new();
        for i in self.bind_group_layout.iter() {
            if let Some(r) = i {
                layouts.push(r)
            }
        }

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&self.shader_meta.name),
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

// pub fn init(
// 	mut shader_static_map: ResMut<Shaders>,
// 	color_layout: Res<DynBindGroupLayout<ColorMaterialGroup>>,
// 	camera_layout: Res<DynBindGroupLayout<CameraMatrixGroup>>,
// 	mut shader_catch: ResMut<ShaderCatch>,
// 	mut shader_map: ResMut<ShaderMap>,
// 	common_state: Res<CommonPipelineState>,
// 	mut command: Commands,
// 	// mut static_index: WriteRes<ColorStaticIndex>,
// 	// mut gradient_static_index: WriteRes<GradientColorStaticIndex>,
// ) {
// 	let shader = GlslShaderStatic::init(
// 		COLOR_SHADER_VS,
// 		COLOR_SHADER_FS,
// 		&mut shader_catch,
// 		&mut shader_map,
// 		|| include_str!("../../../resource/color.vert"),
// 		|| include_str!("../../../resource/color.frag"),
// 	);

// 	let vertex_buffer = create_vertex_buffer_layout();
// 	let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

// 	let vertex_buffer1 = create_vertex_buffer_layout_with_color();
// 	let vertex_buffer_index1 = vertex_buffer_map.insert(vertex_buffer1);

// 	let mut bind_group_layout = VecMap::new();
// 	bind_group_layout.insert(CameraMatrixGroup::id() as usize, (*camera_layout).clone());
// 	bind_group_layout.insert(ColorMaterialGroup::id() as usize, (*color_layout).clone());

// 	shader_static_map.0.push(ShaderStatic {
// 		vs_shader_soruce: shader.shader_vs,
// 		fs_shader_soruce: shader.shader_fs,
// 		bind_group_layout,
// 	});
// 	log::warn!("shader_static_map.0=================={:?}, {:p}", shader_static_map.0.len(), &shader_static_map.0);

// 	// 插入背景颜色shader的索引
// 	let shader_index = shader_static_map.0.len() - 1;
// 	command.insert_resource(ColorStaticIndex(StaticIndex {
// 		shader: shader_index,
// 		pipeline_state: common_state.common,
// 		vertex_buffer_index,
// 		name: COLOR_PIPELINE,
// 	}));

// 	command.insert_resource(GradientColorStaticIndex(StaticIndex {
// 		shader: shader_index,
// 		pipeline_state: common_state.common,
// 		vertex_buffer_index: vertex_buffer_index1,
// 		name: COLOR_PIPELINE,
// 	}));
// }


#[derive(Deref, Resource)]
pub struct PostBindGroupLayout(pub Share<BindGroupLayout>);

impl FromWorld for PostBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let world = world.cell();
        let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        // bind_group_layout
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let layout = cache.bind_group_layout(
            &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::default(),
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            &device,
        );
        Self(layout)
    }
}

// #[derive(Deref, Default, Resource)]
// pub struct ShaderCatch(pub XHashMap<ShaderId, Shader>);

/// Program, 根据shader的原始代码、defines计算获得
pub struct Program {
    pub pipeline_layout: Share<PipelineLayout>,
    pub vs_shader: Share<ShaderModule>,
    pub fs_shader: Share<ShaderModule>,
}

// #[derive(Default)]
// pub struct ShaderInfoMap(pub XHashMap<u64, Share<Program>>);
// pub type StateMap = ResMap<PipelineState>;

#[derive(Default)]
pub struct PipelineMap(pub XHashMap<u64, Share<RenderPipeline>>);

// pub type VertexBufferLayoutMap = ResMap<VertexBufferLayouts>;

pub type VertexBufferLayouts = Vec<VertexBufferLayout>;

#[derive(Hash, Debug)]
pub struct VertexBufferLayout {
    pub array_stride: wgpu::BufferAddress,
    pub step_mode: wgpu::VertexStepMode,
    pub attributes: Vec<wgpu::VertexAttribute>,
}

#[derive(Resource)]
pub struct ResMap<T> {
    pub map: XHashMap<u64, DefaultKey>,
    pub slot: SlotMap<DefaultKey, T>,
}

impl<T> Default for ResMap<T> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            slot: Default::default(),
        }
    }
}

impl<T: Hash> ResMap<T> {
    pub fn get(&self, key: DefaultKey) -> Option<&T> { self.slot.get(key) }

    pub fn insert(&mut self, value: T) -> DefaultKey {
        match self.map.entry(calc_hash(&value, 0)) {
            Entry::Occupied(r) => r.get().clone(),
            Entry::Vacant(r) => {
                let index = self.slot.insert(value);
                r.insert(index);
                index
            }
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
                unsafe { NotNan::new_unchecked(r.bias.slope_scale).hash(state) };
                unsafe { NotNan::new_unchecked(r.bias.clamp).hash(state) };
            }
            None => (),
        };
        self.multisample.hash(state);
        self.multiview.hash(state);
    }
}

/// 单位四边形对应的定点buffer和索引buffer
#[derive(Debug, Resource)]
pub struct UnitQuadBuffer {
    pub vertex: Handle<RenderRes<Buffer>>,
    pub uv: Handle<RenderRes<Buffer>>,
    pub index: Handle<RenderRes<Buffer>>,
}
impl FromWorld for UnitQuadBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().expect("create UnitQuadBuffer need RenderDevice");
        let buffer_asset_mgr = world
            .get_resource::<ShareAssetMgr<RenderRes<Buffer>>>()
            .expect("create UnitQuadBuffer need buffer AssetMgr");
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

#[derive(Debug, Resource)]
pub struct ShareLayout {
    pub depth: BindGroupLayout,
    pub matrix: BindGroupLayout,
    pub view: BindGroupLayout,
    pub project: BindGroupLayout,
    pub empty: BindGroupLayout,
}

impl FromWorld for ShareLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().expect("create ShareLayout need RenderDevice");
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
#[derive(Resource)]
pub struct DynFboClearColorBindGroup(pub DrawBindGroup);

pub fn list_share_as_ref<'a, T, I: Iterator<Item = &'a Option<Share<T>>>>(list: I) -> Vec<&'a T> {
    let mut v = Vec::new();
    for r in list {
        if let Some(r) = r {
            v.push(&**r)
        }
    }
    v
}

// #[derive(Deref, Default, Resource)]
// pub struct DynBindGroups(Vec<(Option<BindGroup>, BindGroupLayout, fn(&RenderDevice, &BindGroupLayout, &Buffer) -> BindGroup)>);

// // 在DynBindGroups中的索引
// #[derive(Resource)]
// pub struct DynBindGroupIndex<T>(usize, PhantomData<T>);
// impl<T: BufferGroup + Group> FromWorld for DynBindGroupIndex<T> {
// 	fn from_world(world: &mut World) -> Self {
// 		let device = world.get_resource::<PiRenderDevice>().unwrap();
// 		let layout = T::create_layout(device, true);

// 		let mut groups = world.get_resource_mut::<DynBindGroups>().unwrap();
// 		groups.push((None, layout, T::create_bind_group ));
// 		let index= groups.len() - 1;
//         Self(index, PhantomData)
// 	}
// }

// impl<T> std::ops::Deref for DynBindGroupIndex<T> {
//     type Target = usize;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// #[derive(Deref, Resource)]
// pub struct DynUniformBuffer (pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer);

// impl FromWorld for DynUniformBuffer {
//     fn from_world(world: &mut World) -> Self {
// 		let limits = world.get_resource::<PiRenderDevice>().unwrap().limits();
//         DynUniformBuffer(
// 			pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer::new(
// 				Some("DynUniformBuffer".to_string()),
// 				limits.min_uniform_buffer_offset_alignment.max(192)))
// 	}
// }

#[derive(Resource)]
pub struct GroupAlloterCenter<>(Vec<Share<GroupAlloter>>, wgpu::Limits);

impl FromWorld for GroupAlloterCenter {
    fn from_world(world: &mut World) -> Self {
        let limits = world.get_resource::<PiRenderDevice>().unwrap().limits();
        GroupAlloterCenter(Vec::new(), limits)
    }
}

impl GroupAlloterCenter {
    fn limits(&self) -> &Limits { &self.1 }

    fn add_alloter(&mut self, alloter: Share<GroupAlloter>) -> usize {
        self.0.push(alloter);
        return self.0.len() - 1;
    }

    pub fn write_buffer(&self, device: &PiRenderDevice, queue: &PiRenderQueue) {
        for i in self.0.iter() {
            i.write_buffer(device, queue);
        }
    }
}

/// 相机binding组
pub struct CameraGroup;
/// ui材质绑定组
pub struct UiMaterialGroup;

/// 动态标记
#[derive(Debug, Default)]
pub struct DynMark;

// #[derive(Resource, Debug, Deref)]
// pub struct DirtyList {
// 	#[deref]
// 	list: SecondaryMap<EntityKey, ()>,
// 	pre_clear_tick: Tick, // 上次清理list的World节拍
// }

// impl Default for DirtyList {
//     fn default() -> Self {
//         Self { 
// 			list: Default::default(), 
// 			pre_clear_tick: Tick::new(0),
// 		}
//     }
// }

// impl DirtyList {
// 	#[inline]
// 	pub fn push(&mut self, entity: Entity) {
// 		self.list.insert(EntityKey(entity), ());
// 	}

// 	#[inline]
// 	pub fn clear(&mut self, tick: Tick) {
// 		self.list.clear();
// 		self.pre_clear_tick = tick;

// 	}
// }

/// buffer累的的binding组的分配器
#[derive(Resource)]
pub struct ShareGroupAlloter<T, M = ()> {
    pub group_index: u32,
    alloter: Share<GroupAlloter>,
    mark: PhantomData<(T, M)>,
}

impl<T, M> std::ops::Deref for ShareGroupAlloter<T, M> {
    type Target = Share<GroupAlloter>;

    fn deref(&self) -> &Self::Target { &self.alloter }
}

impl<M> FromWorld for ShareGroupAlloter<CameraGroup, M> {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let world = world.cell();
        let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let mut group_center = world.get_resource_mut::<GroupAlloterCenter>().unwrap();

        let limits = group_center.limits();
        let min_alignment = limits.min_uniform_buffer_offset_alignment;
        let max_binding_size = limits.max_uniform_buffer_binding_size;

        let entry = CameraBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
        let layout = cache.bind_group_layout(&[entry], &device);
        let alloter = Share::new(
            GroupAlloter::new(
                Some("camera group".to_string()),
                min_alignment,
                max_binding_size,
                None,
                vec![CameraBind::as_layout_entry(wgpu::ShaderStages::VERTEX)],
                layout,
            )
            .unwrap(),
        );
        group_center.add_alloter(alloter.clone());
        Self {
            alloter: alloter,
            group_index: CameraBind::set(),
            mark: PhantomData,
        }
    }
}

impl<M> FromWorld for ShareGroupAlloter<UiMaterialGroup, M> {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<GroupAlloterCenter>();
        world.init_resource::<ShaderInfoCache>();
        let world = world.cell();
        let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        let device = world.get_resource::<PiRenderDevice>().unwrap();


        let entry = UiMaterialBind::as_layout_entry(wgpu::ShaderStages::VERTEX_FRAGMENT);
        let layout = cache.bind_group_layout(&[entry.clone()], &device);

        let mut group_center = world.get_resource_mut::<GroupAlloterCenter>().unwrap();
        let limits = group_center.limits();
        let min_alignment = limits.min_uniform_buffer_offset_alignment;
        let max_binding_size = limits.max_uniform_buffer_binding_size;
        let alloter = Share::new(
            GroupAlloter::new(
                Some("ui metarial group".to_string()),
                min_alignment,
                max_binding_size,
                None,
                vec![entry],
                layout,
            )
            .unwrap(),
        );
        group_center.add_alloter(alloter.clone());

        // println!("ui============{:?}", &layout);

        Self {
            alloter: alloter,
            group_index: UiMaterialBind::set(),
            mark: PhantomData,
        }
    }
}

impl<M> FromWorld for ShareGroupAlloter<DepthGroup, M> {
    fn from_world(world: &mut World) -> Self {
        world.init_resource::<GroupAlloterCenter>();
        world.init_resource::<ShaderInfoCache>();
        let world = world.cell();
        let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        let device = world.get_resource::<PiRenderDevice>().unwrap();

        let entry = DepthBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
        let layout = cache.bind_group_layout(&[entry.clone()], &device);

        let mut group_center = world.get_resource_mut::<GroupAlloterCenter>().unwrap();
        let limits = group_center.limits();
        let min_alignment = limits.min_uniform_buffer_offset_alignment;
        let max_binding_size = limits.max_uniform_buffer_binding_size;
        let alloter = Share::new(
            GroupAlloter::new(
                Some("depth group".to_string()),
                min_alignment,
                max_binding_size,
                None,
                vec![entry],
                layout,
            )
            .unwrap(),
        );
        group_center.add_alloter(alloter.clone());

        // println!("ui============{:?}", &layout);

        Self {
            alloter: alloter,
            group_index: DepthBind::set(),
            mark: PhantomData,
        }
    }
}


#[derive(Resource)]
pub struct CommonSampler {
    pub default: Share<Sampler>,
    pub pointer: Share<Sampler>,
}

impl CommonSampler {
	pub fn new(device: &wgpu::Device) -> Self {
		Self {
            default: Share::new(device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("linear sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            })),
            pointer: Share::new(device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("pointer sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })),
        }
	}
}

impl FromWorld for CommonSampler {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        Self::new(device)
    }
}

// /// 将pass2d组织为层的结构
// #[derive(Deref, Default, DerefMut)]
// pub struct LayerPass2D (LayerDirty<Entity>);

// 如果是sdf2方案，会有第二张纹理
#[derive(Resource, Default)]
pub struct TextTextureGroup(pub Option<Handle<RenderRes<BindGroup>>>, pub Option<Handle<RenderRes<BindGroup>>>);


#[derive(Deref, Resource)]
pub struct EmptyVertexBuffer(pub Handle<RenderRes<Buffer>>);

impl FromWorld for EmptyVertexBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let buffer_assets = world.get_resource::<ShareAssetMgr<RenderRes<Buffer>>>().unwrap();

        let gradient_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Empty VERTEX Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let key = calc_hash(&"Empty VERTEX Buffer", 0);
        let gradient_buf = buffer_assets.insert(key, RenderRes::new(gradient_buf, 4)).unwrap();

        EmptyVertexBuffer(gradient_buf)
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
            // depth_compare: CompareFunction::Always,
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
            blend: Some(CommonBlendState::PREMULTIPLY),
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
#[derive(Resource)]
pub struct ClearDrawObj(pub DrawState, pub PipelineMeta);

// 最大视口尺寸（gui中，各渲染共用同一个深度缓冲区， 统计各视口的最大尺寸，用该尺寸作为深度缓冲区的大小）
#[derive(Debug, Default, Clone, Resource)]
pub struct MaxViewSize {
    pub width: u32,
    pub height: u32,
}


pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
    vec![VertexBufferLayout {
        array_stride: 8 as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: vec![wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }],
    }]
}

pub fn create_vertex_buffer_layout_with_color() -> VertexBufferLayouts {
    vec![
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        VertexBufferLayout {
            array_stride: 16 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 1,
            }],
        },
    ]
}

pub fn create_vertex_buffer_layout_p_v1() -> VertexBufferLayouts {
    vec![
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 1,
            }],
        },
    ]
}

// position 和uv放在同一个buffer中（一些情况，position和uv严格相关，没必要将buffer分开）
pub fn create_vertex_buffer_layout_p_v2() -> VertexBufferLayouts {
    vec![VertexBufferLayout {
        array_stride: 16 as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: vec![
            // position
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            // uv
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 8,
                shader_location: 1,
            },
        ],
    }]
}

pub fn create_vertex_buffer_layout_p_c() -> VertexBufferLayouts {
    vec![
        // position
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        // color
        VertexBufferLayout {
            array_stride: 16 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 1,
            }],
        },
    ]
}

pub fn create_vertex_buffer_layout_p_v_c() -> VertexBufferLayouts {
    vec![
        // position
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        },
        // uv
        VertexBufferLayout {
            array_stride: 8 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 1,
            }],
        },
        // color
        VertexBufferLayout {
            array_stride: 16 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 2,
            }],
        },
    ]
}

pub fn create_vertex_buffer_layout_sdf2() -> VertexBufferLayouts {
    vec![
		VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: vec![wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x4,
				offset: 0,
				shader_location: 0,
			}],
		},
		VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: vec![wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x4,
				offset: 0,
				shader_location: 1,
			}],
		},
		VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: vec![wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x2,
				offset: 0,
				shader_location: 2,
			}],
		},
		VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: vec![wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x2,
				offset: 0,
				shader_location: 3,
			}],
		},
		VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: vec![wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x4,
				offset: 0,
				shader_location: 4,
			}],
		},
    ]
}


/// depth BindGroup缓存
#[derive(Resource, Default)]
pub struct DepthCache {
    pub list: Vec<DrawBindGroup>,
    // pub layout: Share<BindGroupLayout>,
}

// impl FromWorld for DepthCache {
//     fn from_world(world: &mut bevy_ecs::world::World) -> Self {
//         world.init_resource::<ShaderInfoCache>();
//         let world = world.cell();
//         // let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
//         // bind_group_layout
//         // let device = world.get_resource::<PiRenderDevice>().unwrap();
//         // let mut entry = DepthBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
//         // if let BindingType::Buffer { has_dynamic_offset, .. } = &mut entry.ty {
//         //     *has_dynamic_offset = false;
//         // }
//         // let layout = cache.bind_group_layout(&[entry], &device);
//         Self { list: Vec::new(), /*layout*/ }
//     }
// }

impl DepthCache {
    pub fn or_create_depth<'a>(
		&mut self, cur_depth: usize, 
		depth_alloter: &'a ShareGroupAlloter<DepthGroup>
	) {
        let mut depth = self.list.len();
        while depth <= cur_depth {
            let mut group = depth_alloter.alloc();
            let _ = group.set_uniform(&DepthUniform(&[depth as f32]));
            // 添加深度group、永不释放
            self.list.push(DrawBindGroup::Offset(group));
            depth += 1;
        }
    }
}

// 常用的默认
pub struct CommonBlendState;

impl CommonBlendState {
    // 正常状态
    pub const NORMAL: wgpu::BlendState = wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    };

    // 预乘
    pub const PREMULTIPLY: wgpu::BlendState = wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    };
}

// 渲染目标管理
#[derive(Resource)]
pub struct TargetCacheMgr {
	pub key: AtomicUsize,
	pub assets: ShareAssetMgr<CacheTarget>,
}

impl TargetCacheMgr {
	pub fn push(&self, value: CacheTarget) -> Handle<CacheTarget> {
		let key = self.key.fetch_add(1, Ordering::Relaxed);
		self.assets.insert(key, value).unwrap()
	}
}

pub fn create_render_pipeline(
	device: &wgpu::Device, 
	pipeline_layout: &PipelineLayout, 
	vs: &wgpu::ShaderModule, 
	fs: &wgpu::ShaderModule,
	blend: Option<BlendState>,
	depth_compare: CompareFunction,
) -> wgpu::RenderPipeline {
	let state = PipelineState {
        targets: vec![Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::pi_render_default(),
            blend,
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
            depth_compare,
            // depth_compare: CompareFunction::Always,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        multiview: None,
    };

	let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
		label: Some(&"ui_pipeline"),
		layout: Some(&pipeline_layout),
		vertex: wgpu::VertexState {
			module: vs,
			entry_point: "main",
			buffers: &[
				wgpu::VertexBufferLayout {
					array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
					step_mode: wgpu::VertexStepMode::Vertex,
					attributes: &[wgpu::VertexAttribute {
						format: wgpu::VertexFormat::Float32x2,
						offset: 0,
						shader_location: 0,
					}],
				},

				wgpu::VertexBufferLayout {
					array_stride: MeterialBind::SIZE as u64,
					step_mode: wgpu::VertexStepMode::Instance,
					attributes: &[
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 0,
							shader_location: 1,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 16,
							shader_location: 2,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 32,
							shader_location: 3,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 48,
							shader_location: 4,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 64,
							shader_location: 5,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 80,
							shader_location: 6,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 96,
							shader_location: 7,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 112,
							shader_location: 8,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 128,
							shader_location: 9,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 144,
							shader_location: 10,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 160,
							shader_location: 11,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 176,
							shader_location: 12,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 192,
							shader_location: 13,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x4,
							offset: 208,
							shader_location: 14,
						},
						// wgpu::VertexAttribute {
						// 	format: wgpu::VertexFormat::Float32x4,
						// 	offset: 224,
						// 	shader_location: 15,
						// },
						// wgpu::VertexAttribute {
						// 	format: wgpu::VertexFormat::Float32x4,
						// 	offset: 240,
						// 	shader_location: 16,
						// },
					],
				},
			],
		},
		fragment: Some(wgpu::FragmentState {
			module: fs,
			entry_point: "main",
			targets: state.targets.as_slice(),
		}),
		primitive: state.primitive.clone(),
		depth_stencil: state.depth_stencil.clone(),
		multisample: state.multisample.clone(),
		multiview: state.multiview.clone(),
	});

	pipeline
}





// pub struct GpuArrayBuffer {
// 	buffer: RenderInstances,
//     // Uniform(BatchedUniformBuffer<T>),
//     // Storage((StorageBuffer<Vec<T>>, Vec<T>)),
// }

// impl GpuArrayBuffer {
//     pub fn update_dirty_range(&mut self, range: Range<usize>) {
// 		log::trace!("update_dirty_range= {:?}", range);
// 		if self.dirty_range.start == std::usize::MAX {
// 			self.dirty_range.start = range.start;
// 		}

// 		self.dirty_range.end = range.end;
// 	}

// 	/// 如果data的长度不足（小于cur_index,则对data进行扩容)
// 	pub fn reserve(&mut self) {
// 		if self.data.capacity() < self.cur_index {
// 			self.data.reserve(self.cur_index - self.data.capacity());
// 		}

// 		// 安全： 前一步保证了容量一定足够， 这里的操作必然是安全的
// 		unsafe {self.data.set_len(self.cur_index)};
// 	}

// 	/// 分配一个实例数据
// 	pub fn alloc_instance_data(&mut self) -> InstanceIndex {
// 		let ret = self.cur_index;
// 		self.cur_index += self.alignment;
// 		self.update_dirty_range(ret..self.cur_index);
// 		ret
// 	}

// 	/// 引用一个实例数据
// 	#[inline]
// 	pub fn instance_data_mut(&mut self, index: InstanceIndex) -> InstanceData {
// 		InstanceData {
// 			index,
// 			data: self
// 		}
// 	}

// 	/// 在cur_index索引之后扩展片段
// 	#[inline]
// 	pub fn extend(&mut self, slice: &[u8]) {
// 		debug_assert_eq!(slice.len() % self.alignment, 0);
// 		self.reserve();
// 		self.data.extend_from_slice(slice);

// 		self.cur_index += slice.len();
// 	}

// 	// 为该实例设置数据
// 	pub fn set_data(&mut self, index: usize, value: &[u8]) {
// 		// 在debug版本， 检查数据写入是否超出自身对齐范围
// 		debug_assert!((value.byte_len() as usize + index) > self.data.len());
// 		let d = self.data.as_mut_slice();
// 		for i in 0..value.len() {
// 			d[i] = value[i];
// 		}

// 		// value.write_into(self.index as u32, &mut self.data.data);
// 		log::trace!("byte_len1========={:?}", value.byte_len());
// 		self.update_dirty_range(index..index + value.len());
// 	}

	

// 	/// 在cur_index索引之后扩展片段
// 	#[inline]
// 	pub fn extend_count(&mut self, count: usize) {
// 		self.cur_index += count * self.alignment;
// 		self.reserve();
// 	}

// 	#[inline]
// 	pub fn extend_to(&mut self, index: usize) {
// 		if self.cur_index < index {
// 			self.cur_index = index;
// 			self.reserve();
// 		}
		
// 	}

// 	#[inline]
// 	pub fn slice(&self, range: Range<usize>) -> &[u8] {
// 		&self.data[range]
// 	}

// 	/// 当前索引
// 	pub fn cur_index(&self) -> usize {
// 		self.cur_index
// 	}

// 	/// 下一个索引
// 	pub fn next_index(&self, index: InstanceIndex) -> usize {
// 		index + self.alignment
// 	}

// 	pub fn data(&self) -> &[u8] {
// 		&self.data
// 	}
// }

// /// An index into a [`GpuArrayBuffer`] for a given element.
// #[derive(Component, Clone)]
// pub struct GpuArrayBufferIndex<T: > {
//     /// The index to use in a shader into the array.
//     pub index: NonMaxU32,
//     /// The dynamic offset to use when setting the bind group in a pass.
//     /// Only used on platforms that don't support storage buffers.
//     pub dynamic_offset: Option<NonMaxU32>,
//     pub element_type: PhantomData<T>,
// }



