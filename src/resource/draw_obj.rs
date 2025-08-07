//! 与DrawObject相关的资源
use std::{collections::hash_map::Entry, hash::Hash, marker::PhantomData, num::NonZeroU32, borrow::Cow};

use naga::Range;
use pi_world::prelude::{FromWorld, World, Entity};
use ordered_float::NotNan;
use pi_assets::{asset::Handle, homogeneous::HomogeneousMgr, mgr::AssetMgr};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{NodeId, PiRenderDevice, PiRenderQueue};
use pi_hash::{XHashMap, XHashSet};
use pi_map::vecmap::VecMap;
use pi_render::{
    components::view::target_alloc::FboRes, renderer::{draw_obj::DrawBindGroup, texture::ImageTextureFrame, vertices::{EVerticesBufferUsage, RenderVertices}}, rhi::{
        asset::{AssetWithId, RenderRes, TextureRes},
        bind_group::BindGroup,
        bind_group_layout::BindGroupLayout,
        buffer::Buffer,
        device::RenderDevice,
        dyn_uniform_buffer::{BufferGroup, GroupAlloter},
        pipeline::RenderPipeline,
        shader::ShaderMeta,
        texture::PiRenderDefault, RenderQueue,
    }
};
use pi_render::rhi::shader::AsLayoutEntry;
use pi_render::rhi::shader::BindLayout;
use pi_share::Share;
use pi_slotmap::{DefaultKey, SlotMap};
use wgpu::{
    BindGroupEntry, BindingType, BlendState, BufferDescriptor, CompareFunction, DepthBiasState, DepthStencilState, Extent3d, FrontFace, Limits, MultisampleState, PipelineLayout, RenderPass, Sampler, SamplerBindingType, ShaderModule, ShaderStages, StencilState, TextureDescriptor, TextureFormat, TextureSampleType, TextureView, TextureViewDescriptor, TextureViewDimension
};
use pi_render::rhi::shader::Input;

use crate::{
    components::{calc::WorldMatrix, pass_2d::{CacheTarget, DrawElement, InstanceDrawState}}, resource::ShareFontSheet, shader1::{batch_gauss_blur::{self, GussMeterialBind}, batch_meterial::{vert_layout, CameraBind, MeterialBind, PositionVert, ProjectUniform, Sdf2TextureSizeUniform, ViewUniform}, batch_sdf_glow::{self, GlowMeterialBind}, batch_sdf_gray::{self, GrayMeterialBind}, GpuBuffer}, utils::tools::{calc_float_hash, calc_hash}
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

#[derive(Clone, Debug)]
pub enum BatchTextureItem {
    Texture(Handle<AssetWithId<TextureRes>>),
    Frame(Handle<ImageTextureFrame>),
    Fbo(Handle<FboRes>)
}

impl BatchTextureItem {
    // fn id(&self) -> &KeyData {
    //     match self {
    //         BatchTextureItem::Texture(droper) => &droper.id,
    //         BatchTextureItem::Fbo(droper) => &droper.id,
    //     }
    // }

    fn texture_view(&self) -> &TextureView {
        match self {
            BatchTextureItem::Texture(droper) => &droper.texture_view,
            BatchTextureItem::Fbo(droper) => &droper.texture_view,
            BatchTextureItem::Frame(droper) => &droper.view,
        }
    }

    fn is_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BatchTextureItem::Texture(droper), BatchTextureItem::Texture(droper1)) => Share::ptr_eq(droper, droper1),
            (BatchTextureItem::Fbo(droper), BatchTextureItem::Fbo(droper1)) => Share::ptr_eq(droper, droper1),
            (BatchTextureItem::Frame(droper), BatchTextureItem::Frame(droper1)) => Share::ptr_eq(&droper.tex, &droper1.tex),
            _ => false,
        }
    }
}
// 批处理纹理
pub struct BatchTexture {
	// max_bind: usize,
	
	// temp_texture_indexs: SecondaryMap<DefaultKey, u32>, // 纹理在该bindgroup中的binding, 以及本利本身
	pub temp_textures: Vec<(BatchTextureItem, Share<wgpu::Sampler>)>,
    // pub temp_texture: (BatchTextureItem, Share<wgpu::Sampler>),

	// group_layouts: Vec<wgpu::BindGroupLayout>,
	group_layout: wgpu::BindGroupLayout,
    group_layout_array: wgpu::BindGroupLayout,
	pub(crate) default_texture_view: wgpu::TextureView,
    pub(crate) default_texture_array_view: wgpu::TextureView,
	pub(crate) default_sampler: wgpu::Sampler,
	default_texture_group: Share<wgpu::BindGroup>,
    default_texture_array_group: Share<wgpu::BindGroup>,
}

impl BatchTexture {
	const BINDING_COUNT: u32 = 1;
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
			label: Some("single texture layout"),
			entries: &entry[..],
		});
        let group_layout_array = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("batch texture layout"),
			entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                }
            ],
		});
		let default_array_texture = device.create_texture(&TextureDescriptor {
			label: Some("default texture"),
			size: Extent3d { width: 4, height: 4, depth_or_array_layers: 2 },
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});
       let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("default sampler"),
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});
        
		let texture_array_view = default_array_texture.create_view(&TextureViewDescriptor::default());
        let default_texture_array_group = Self::take_group1(device, &Vec::new(), &texture_array_view, &sampler, &group_layout_array, "batch texture bindgroup");

        let default_texture = device.create_texture(&TextureDescriptor {
			label: Some("default texture"),
			size: Extent3d { width: 4, height: 4, depth_or_array_layers: 1 },
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});
        
		let texture_view = default_texture.create_view(&TextureViewDescriptor::default());

		let default_texture_group = Self::take_group1(device, &Vec::new(), &texture_view, &sampler, &group_layout, "single texture bindgroup");
	
		let r = Self {
			// max_bind: Self::BINDING_COUNT,
			// temp_texture_indexs: SecondaryMap::new(),
			temp_textures: Vec::new(),

			group_layout,
            group_layout_array,
			default_texture_view: texture_view,
            default_texture_array_view: texture_array_view,
			default_sampler: sampler,
			default_texture_group: Share::new(default_texture_group),
            default_texture_array_group: Share::new(default_texture_array_group),
			// common_sampler: CommonSampler::new(device),
		};
		r
	}
	/// push一张纹理，返回纹理索引， 当纹理数量达到max_bind限制时， 会返回一个wgpu::BindGroup，并先清空当前所有的临时数据， 再添加数据
	/// 注意， 目前同一张纹理只能用同一种采样方式，使用不同的采样样式push纹理，将不会覆盖之前的（主要原因是目前gui并没有不同采样方式的需求）
	pub fn push(&mut self, texture: BatchTextureItem, sampler: &Share<Sampler>, device: &wgpu::Device) -> (usize, Option<wgpu::BindGroup>, &'static str) {
        let index = match &texture {
            BatchTextureItem::Texture(_droper) => todo!(),
            BatchTextureItem::Frame(droper) => droper.coord(),
            BatchTextureItem::Fbo(_droper) => 0,
        } as usize;
        if let Some(r) = self.temp_textures.get(0) {
            if r.0.is_eq(&texture) {
                return (index, None, "none");
            }

            let (group, name) = self.take_group(device);
            self.temp_textures.push((texture, sampler.clone()));
            return (index, group, name)
        }

		self.temp_textures.push((texture, sampler.clone()));

		(index, None, "none")
	}

	/// 将当前的临时数据立即创建一个bindgroup，并返回
	pub fn take_group(&mut self, device: &wgpu::Device) -> (Option<wgpu::BindGroup>, &'static str) {
		if self.temp_textures.len() == 0 {
			return (None, "none");
		}

        // log::debug!("take_group========{:?}", self.temp_textures.len());
        // let len = self.temp_textures.len();
        let (group_layout, name) = match &self.temp_textures[0].0 {
            BatchTextureItem::Texture(_droper) => todo!(),
            BatchTextureItem::Frame(_droper) => if _droper.frame().is_some() { 
                (&self.group_layout_array, "batch texture bindgroup") 
            } else { 
                (&self.group_layout,  "single texture bindgroup")
            },
            BatchTextureItem::Fbo(_droper) => {
                (&self.group_layout, "single texture bindgroup")
            },
        };
		let group = Some(Self::take_group1(device, &self.temp_textures, &self.default_texture_view, &self.default_sampler, &group_layout, name));
		// 清理临时数据
		// self.temp_texture_indexs.clear();
		self.temp_textures.clear();
		(group, name)
	}

	pub fn default_group(&self, is_single: bool) -> Share<wgpu::BindGroup> {
		// let r = Self::take_group1(device, &Vec::new(), &self.default_texture_view, &self.default_sampler, &self.group_layout_array, "batch texture bindgroup");
        // r
        if is_single {
            self.default_texture_group.clone()
        } else {
            self.default_texture_array_group.clone()
        }
	}

	/// 将当前的临时数据立即创建一个bindgroup，并返回
	fn take_group1(device: &wgpu::Device, temp_textures: &Vec<(BatchTextureItem, Share<wgpu::Sampler>)>, default_texture_view: &wgpu::TextureView, default_sampler: &wgpu::Sampler, group_layout: &wgpu::BindGroupLayout, name: &str) -> wgpu::BindGroup {    
        let mut entrys = Vec::with_capacity(Self::BINDING_COUNT as usize * 2);
		for (binding, (texture, sampler)) in temp_textures.iter().enumerate() {
			entrys.push(
				BindGroupEntry {
					binding: (binding * 2) as u32,
					resource: wgpu::BindingResource::TextureView(&texture.texture_view()) ,
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
			label: Some(name),
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

// 用于将根节点渲染到屏幕的图节点
#[derive(Default, Debug)]
pub struct LastGraphNode(pub NodeId);

// gui子图
#[derive(Default, Debug)]
pub struct GuiSubGraphNode(pub NodeId);

pub struct DefaultPipelines {
    pub common_pipeline: Share<wgpu::RenderPipeline>,
    pub common_opacity_pipeline: Share<wgpu::RenderPipeline>,
    pub copy_pipeline: Share<wgpu::RenderPipeline>,
	pub premultiply_pipeline: Share<wgpu::RenderPipeline>,

    pub common_fbo_pipeline: Share<wgpu::RenderPipeline>, 
    pub common_fbo_opacity_pipeline: Share<wgpu::RenderPipeline>,
    pub fbo_premultiply_pipeline: Share<wgpu::RenderPipeline>,

	pub clear_pipeline: Share<wgpu::RenderPipeline>,
    pub mask_image_pipeline: Share<wgpu::RenderPipeline>,

    pub text_gray_pipeline: Share<wgpu::RenderPipeline>,
    pub text_shadow_pipeline: Share<wgpu::RenderPipeline>,
    pub text_glow_pipeline: Share<wgpu::RenderPipeline>,
}
pub struct InstanceContext {
	pub vert: RenderVertices,

	vs: wgpu::ShaderModule,
	fs: wgpu::ShaderModule,

	fs_opacity: wgpu::ShaderModule,
    fs_fbo: wgpu::ShaderModule,
    fs_opacity_fbo: wgpu::ShaderModule,

	pipeline_cache: XHashMap<u64, Share<wgpu::RenderPipeline>>,
	pub common_blend_state_hash: u64,
	pub premultiply_blend_state_hash: u64,

    pub default_pipelines: DefaultPipelines,
	

	pub instance_data: GpuBuffer,
	pub instance_buffer: Option<(wgpu::Buffer, usize)>,
    pub batch_texture: BatchTexture,

    pub text_gray_instance_data: GpuBuffer,
	pub text_gray_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub text_shadow_h_instance_data: GpuBuffer,
	pub text_shadow_h_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub text_shadow_v_instance_data: GpuBuffer,
	pub text_shadow_v_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub text_glow_instance_data: GpuBuffer,
	pub text_glow_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub svg_gray_instance_data: GpuBuffer,
	pub svg_gray_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub svg_shadow_h_instance_data: GpuBuffer,
	pub svg_shadow_h_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub svg_shadow_v_instance_data: GpuBuffer,
	pub svg_shadow_v_instance_buffer: Option<(wgpu::Buffer, usize)>,

    pub svg_glow_instance_data: GpuBuffer,
	pub svg_glow_instance_buffer: Option<(wgpu::Buffer, usize)>,

	// // // 深度buffer
	// pub depth_data: GpuBuffer,
	// pub depth_buffer: Option<(wgpu::Buffer, usize)>,

	// sdf纹理(由于实例数据槽位有限， text的数据填充后没有空间放置纹理索引， 因此这里将文字纹理单独放在group中， 文字采样固定纹理)
	pub sdf2_texture_group: Option<Share<wgpu::BindGroup>>,
	pub sdf2_texture_layout: wgpu::BindGroupLayout,
	pub camera_alloter: ShareGroupAlloter<CameraGroup>,

	pub pipeline_layout: PipelineLayout,
    pub fbo_pipeline_layout: PipelineLayout,
    pub text_effect_pipeline_layout: PipelineLayout,

	pub default_camera: BufferGroup,

	pub draw_list: Vec<(DrawElement, Entity/*fbo passid*/)>, // 渲染元素
    // /// 批处理是否需要调整
    // /// 当draw_obj新增和删除、RenderCount发生改变、纹理发生改变（包含动态分配的fbo）时， 需要重新调整批处理
    pub rebatch: bool, 
    pub posts: Vec<Entity>, // 渲染元素



    pub pass_toop_list: Vec<Entity>, //该根下 从叶子开始的广度遍历排序
    pub next_node_with_depend: Vec<usize>,

    pub debug_info: VecMap<String>,

    pub draw_screen_range: std::ops::Range<usize>
}

impl InstanceContext {
    pub fn set_pipeline<'a>(&'a self, rp: &mut RenderPass<'a>, instance_draw: &'a InstanceDrawState, render_state: &mut RenderState) {
        let p = match &instance_draw.pipeline {
			Some(r) => r,
			None => &self.default_pipelines.common_pipeline,
		};
        if render_state.reset || !Share::ptr_eq(&p, &render_state.pipeline)  {
            rp.set_pipeline(p);
            render_state.pipeline = p.clone();
        }
    }
	pub fn draw<'a>(&'a self, rp: &mut RenderPass<'a>, instance_draw: &'a InstanceDrawState, render_state: &mut RenderState) {
        // log::debug!("draw====={:?}", (render_state.reset, &instance_draw.texture_bind_group, &render_state.texture));
        if render_state.reset  {
            if let Some(texture) = &self.sdf2_texture_group {
                // log::debug!("set_bind_group 1===================={:p}, {:?}", &**texture, (instance_draw.pipeline_type, instance_draw.texture_bind_group_type, &instance_draw.instance_data_range));
                rp.set_bind_group(1, &**texture, &[]);
            }
            if let Some(texture) = &instance_draw.texture_bind_group {
                rp.set_bind_group(2, &**texture, &[]);
                render_state.texture = texture.clone();
                // log::debug!("set_bind_group 2===================={:p}, {:?}", &**texture, (instance_draw.pipeline_type, instance_draw.texture_bind_group_type, &instance_draw.instance_data_range));
            }
            rp.set_vertex_buffer(0, self.vert.slice());
		    rp.set_vertex_buffer(1, self.instance_buffer.as_ref().unwrap().0.slice(..));
            render_state.reset = false;
        } else {   
            if let Some(texture) = &instance_draw.texture_bind_group {
                if !Share::ptr_eq(&texture, &render_state.texture) {
                    // log::warn!("set_bind_group 3===================={:p}, {:?}", &**texture, (instance_draw.pipeline_type, instance_draw.texture_bind_group_type,&instance_draw.instance_data_range));
                    rp.set_bind_group(2, &**texture, &[]);
                    render_state.texture = texture.clone();
                } else {
                    // log::debug!("4===================={:p}, {:?}", &**texture, (instance_draw.pipeline_type, instance_draw.texture_bind_group_type, &instance_draw.instance_data_range));
                }
            }
        }

        log::debug!("darw================={:?}", instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32 );
        // log::debug!("instance_data_range====={:?}", (&instance_draw.instance_data_range, instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32));
		#[cfg(debug_assertions)]
        {
            for i in instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32 {
                // let debug_info = self.debug_info.get(i as usize/MeterialBind::SIZE);
                // let index = i as usize * self.instance_data.alignment;
                // let render_flag = self.instance_data.get_render_ty(index as u32);
                // if render_flag == 0 {
                //     panic!("!!!!!!!!!!!!!!, {}", index);
                // }
                rp.draw(0..6, i..i+1);
            } 
        }
        #[cfg(not(debug_assertions))]
        rp.draw(0..6, instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32);

	}


    pub fn draw_effect<'a>(
        &'a self, 
        rp: &mut RenderPass<'a>, 
        instance_draw: &'a InstanceDrawState, 
        instance_buffer: &'a Option<(wgpu::Buffer, usize)>, 
        instance_data: &'a GpuBuffer, 
        render_state: &mut RenderState) {
        // log::debug!("draw_effect====={:?}", (render_state.reset, &instance_draw.instance_data_range, &instance_draw.texture_bind_group, &render_state.texture));
        if render_state.reset  {
            if let Some(texture) = &instance_draw.texture_bind_group {
                rp.set_bind_group(1, &**texture, &[]);
                render_state.texture = texture.clone();
            }
            rp.set_vertex_buffer(0, self.vert.slice());
		    rp.set_vertex_buffer(1, instance_buffer.as_ref().unwrap().0.slice(..));
            render_state.reset = false;
        } else {   
            if let Some(texture) = &instance_draw.texture_bind_group {
                if !Share::ptr_eq(&texture, &render_state.texture) {
                    rp.set_bind_group(1, &**texture, &[]);
                    render_state.texture = texture.clone();
                }
            };
        }

        // log::debug!("darw================={:?}", instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32 );
        // log::debug!("instance_data_range====={:?}", (&instance_draw.instance_data_range, instance_draw.instance_data_range.start as u32/self.instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/self.instance_data.alignment as u32));
		#[cfg(debug_assertions)]
        {
            for i in instance_draw.instance_data_range.start as u32/instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/instance_data.alignment as u32 {
                // let debug_info = self.debug_info.get(i as usize/MeterialBind::SIZE);
                // let index = i as usize * instance_data.alignment;
                // let render_flag = instance_data.get_render_ty(index as u32);
                // if render_flag == 0 {
                //     panic!("!!!!!!!!!!!!!!, {}", index);
                // }
                // log::debug!("instance_data_range effect====={:?}", (i, i as u32/instance_data.alignment as u32));
                rp.draw(0..6, i..i+1);
            } 
        }
        #[cfg(not(debug_assertions))]
        rp.draw(0..6, instance_draw.instance_data_range.start as u32/instance_data.alignment as u32..instance_draw.instance_data_range.end as u32/instance_data.alignment as u32);

	}
}

pub struct RenderState {
    pub reset: bool,
    pub pipeline: Share<wgpu::RenderPipeline>,
    pub texture: Share<wgpu::BindGroup>,
}
impl FromWorld for InstanceContext {
	
    fn from_world(world: &mut World) -> Self {
		world.init_single_res::<UnitQuadBuffer>();
        world.init_single_res::<GroupAlloterCenter>();
		let world1 = world.unsafe_world();
        let device = world1.get_single_res::<PiRenderDevice>().unwrap();
        let mut world2 = world.unsafe_world();
        let group_center = world2.get_single_res_mut::<GroupAlloterCenter>().unwrap();

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
		

		let text_texture_layout =(****device).create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: None,
			entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // wgpu::BindGroupLayoutEntry {
                // 	binding: 2,
                // 	visibility: ShaderStages::FRAGMENT,
                // 	ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false },
                // 	count: None,
                // },
                // wgpu::BindGroupLayoutEntry {
                // 	binding: 3,
                // 	visibility: ShaderStages::FRAGMENT,
                // 	ty: BindingType::Sampler(SamplerBindingType::Filtering),
                // 	count: None,
                // },
                // wgpu::BindGroupLayoutEntry {
                // 	binding: 4,
                // 	visibility: ShaderStages::FRAGMENT,
                // 	ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false },
                // 	count: None,
                // },
                // wgpu::BindGroupLayoutEntry {
                // 	binding: 5,
                // 	visibility: ShaderStages::FRAGMENT,
                // 	ty: BindingType::Sampler(SamplerBindingType::Filtering),
                // 	count: None,
                // }
            ],
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
                shader: Cow::Borrowed(include_str!("../shader1/batch_shader.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: &[],
            },
        });
        let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_shader.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let fs_opacity = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"opacity_ui_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_shader_opacity.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let fs_fbo = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_fbo_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_shader_fbo.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let fs_opacity_fbo = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"opacity_fbo_ui_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_shader_opacity_fbo.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_shader"),
            bind_group_layouts: &[&*uniform_layout, &text_texture_layout, &batch_texture.group_layout_array],
            push_constant_ranges: &[],
        });

        let fbo_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_fbo_shader"),
            bind_group_layouts: &[&*uniform_layout, &text_texture_layout, &batch_texture.group_layout],
            push_constant_ranges: &[],
        });

		let common_blend_state_hash = calc_hash(&(CommonBlendState::NORMAL, false, false), 0);
		let copy_pipeline = Share::new(create_render_pipeline("copy_pipeline ui", &device, &fbo_pipeline_layout, &vs, &fs_opacity_fbo, Some(CommonBlendState::NORMAL), CompareFunction::Always, false, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));
		let common_pipeline  = Share::new(create_render_pipeline("common_pipeline ui", &device, &pipeline_layout, &vs, &fs, Some(CommonBlendState::NORMAL), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, false, FrontFace::Ccw));
        
        let common_opacity_blend_state_hash = calc_hash(&(CommonBlendState::NORMAL, true, false), 0);
        let common_opacity_pipeline = Share::new(create_render_pipeline("common_opacity_pipeline ui", &device, &pipeline_layout, &vs, &fs_opacity, Some(CommonBlendState::NORMAL), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));

        let common_fbo_blend_state_hash = calc_hash(&(CommonBlendState::NORMAL, false, true), 0);
        let common_fbo_pipeline  = Share::new(create_render_pipeline("common_fbo_pipeline ui", &device, &fbo_pipeline_layout, &vs, &fs_fbo, Some(CommonBlendState::NORMAL), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, false, FrontFace::Ccw));
        
        let common_fbo_opacity_blend_state_hash = calc_hash(&(CommonBlendState::NORMAL, true, true), 0);
        let common_fbo_opacity_pipeline = Share::new(create_render_pipeline("common_fbo_opacity_pipeline ui", &device, &fbo_pipeline_layout, &vs, &fs_opacity_fbo, Some(CommonBlendState::NORMAL), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));

		let premultiply_blend_state_hash = calc_hash(&(CommonBlendState::PREMULTIPLY, false, false), 0);
		let premultiply_pipeline = Share::new(create_render_pipeline("premultiply_pipeline ui", &device, &pipeline_layout, &vs, &fs, Some(CommonBlendState::PREMULTIPLY), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));
        
        let fbo_premultiply_blend_state_hash = calc_hash(&(CommonBlendState::PREMULTIPLY, false, true), 0);
        let fbo_premultiply_pipeline = Share::new(create_render_pipeline("fbo_premultiply_pipeline ui", &device, &fbo_pipeline_layout, &vs, &fs_fbo, Some(CommonBlendState::PREMULTIPLY), CompareFunction::GreaterEqual, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));

		let clear_blend_state_hash = calc_hash(&CompareFunction::Always, calc_hash(&CommonBlendState::NORMAL, 0));
		let clear_pipeline = Share::new(create_render_pipeline("clear ui", &device, &pipeline_layout, &vs, &fs, Some(BlendState {
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
		}), CompareFunction::Always, true, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Cw));
        let mask_image_pipeline = Share::new(create_render_pipeline("mask image", &device, &pipeline_layout, &vs, &fs, Some(BlendState {
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
		}), CompareFunction::Always, false, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));


        let text_gray_vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_gray_vs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_sdf_gray.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: &[],
            },
        });
        let text_gray_fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_gray_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_sdf_gray.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let text_effect_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_text_gray_shader"),
            bind_group_layouts: &[&*uniform_layout, &text_texture_layout],
            push_constant_ranges: &[],
        });
        let text_gray_pipeline = Share::new(create_render_pipeline("batch text gray", &device, &text_effect_pipeline_layout, &text_gray_vs, &text_gray_fs, Some(BlendState {
			color: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::SrcAlpha,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
			alpha: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
		}), CompareFunction::Always, false, wgpu::TextureFormat::R8Unorm, batch_sdf_gray::vert_layout().as_slice(), GrayMeterialBind::SIZE, true, FrontFace::Ccw));
       
        let text_shadow_vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_shadow_vs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_gauss_blur.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: &[],
            },
        });
        let text_shadow_fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_shadow_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_gauss_blur.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });
        let text_shadow_pipeline = Share::new(create_render_pipeline("batch text shadow", &device, &text_effect_pipeline_layout, &text_shadow_vs, &text_shadow_fs, Some(BlendState {
			color: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::SrcAlpha,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
			alpha: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::Zero,
				operation: wgpu::BlendOperation::Add,
			},
		}), CompareFunction::Always, false, wgpu::TextureFormat::R8Unorm, batch_gauss_blur::vert_layout().as_slice(), GussMeterialBind::SIZE, true, FrontFace::Ccw));
        
        let text_glow_vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_glow_vs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_sdf_glow.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: &[],
            },
        });
        let text_glow_fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&"ui_text_glow_fs"),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(include_str!("../shader1/batch_sdf_glow.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[],
            },
        });
        let text_glow_pipeline = Share::new(create_render_pipeline("batch text glow", &device, &text_effect_pipeline_layout, &text_glow_vs, &text_glow_fs, Some(BlendState {
			color: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::One,
				operation: wgpu::BlendOperation::Max,
			},
			alpha: wgpu::BlendComponent {
				src_factor: wgpu::BlendFactor::One,
				dst_factor: wgpu::BlendFactor::One,
				operation: wgpu::BlendOperation::Max,
			},
		}), CompareFunction::Always, false, wgpu::TextureFormat::R8Unorm, batch_sdf_glow::vert_layout().as_slice(), GlowMeterialBind::SIZE, true, FrontFace::Ccw));

		let mut pipeline_cache = XHashMap::default();
        pipeline_cache.insert(clear_blend_state_hash, clear_pipeline.clone());

		pipeline_cache.insert(common_blend_state_hash, common_pipeline.clone());
		pipeline_cache.insert(premultiply_blend_state_hash, premultiply_pipeline.clone());
		
        pipeline_cache.insert(common_fbo_blend_state_hash, common_fbo_pipeline.clone());
		pipeline_cache.insert(fbo_premultiply_blend_state_hash, fbo_premultiply_pipeline.clone());

        pipeline_cache.insert(common_opacity_blend_state_hash, common_opacity_pipeline.clone());
         pipeline_cache.insert(common_fbo_opacity_blend_state_hash, common_fbo_opacity_pipeline.clone());

		let view_project = WorldMatrix::default().0;
		let mut default_camera = camera_alloter.alloc();
		let _ = default_camera.set_uniform(&ProjectUniform(view_project.as_slice()));
		let _ = default_camera.set_uniform(&ViewUniform(view_project.as_slice()));

        let font_sheet = world.get_single_res::<ShareFontSheet>().unwrap();
        let font_sheet = font_sheet.borrow();
        let data_texture_size = font_sheet.font_mgr().table.sdf2_table.data_packer_size();
		let _ = default_camera.set_uniform(&Sdf2TextureSizeUniform(&[data_texture_size.width as f32, data_texture_size.height as f32]));

		
		Self {
			vert: RenderVertices {
				slot: PositionVert::location(),
				buffer: EVerticesBufferUsage::Temp(Share::new(vertex_buf)),
				buffer_range: None,
				size_per_value: 8,
			},
			vs,
			fs,
            fs_opacity,
            fs_opacity_fbo,
            fs_fbo,
			pipeline_cache,
			common_blend_state_hash,
			premultiply_blend_state_hash,
            default_pipelines : DefaultPipelines {
                common_pipeline,
                common_opacity_pipeline,
                common_fbo_pipeline, 
                common_fbo_opacity_pipeline,
                fbo_premultiply_pipeline,
                copy_pipeline,
                premultiply_pipeline,
                clear_pipeline,
                mask_image_pipeline,

                text_gray_pipeline,
                text_shadow_pipeline,
                text_glow_pipeline,
            },

			instance_data: GpuBuffer::new(MeterialBind::SIZE, 1000 * MeterialBind::SIZE),
			instance_buffer: None,

            text_gray_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GrayMeterialBind::SIZE),
			text_gray_instance_buffer: None,

            text_shadow_h_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GussMeterialBind::SIZE),
			text_shadow_h_instance_buffer: None,

            text_shadow_v_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GussMeterialBind::SIZE),
			text_shadow_v_instance_buffer: None,

            text_glow_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GlowMeterialBind::SIZE),
			text_glow_instance_buffer: None,

            svg_gray_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GrayMeterialBind::SIZE),
			svg_gray_instance_buffer: None,

            svg_shadow_h_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GussMeterialBind::SIZE),
			svg_shadow_h_instance_buffer: None,

            svg_shadow_v_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GussMeterialBind::SIZE),
			svg_shadow_v_instance_buffer: None,

            svg_glow_instance_data: GpuBuffer::new(GrayMeterialBind::SIZE, 200 * GlowMeterialBind::SIZE),
			svg_glow_instance_buffer: None,

			batch_texture,

			// depth_data: GpuBuffer::new(4, 0),
			// depth_buffer: None,

			sdf2_texture_group: None,
			sdf2_texture_layout: text_texture_layout,
			camera_alloter,
			pipeline_layout,
            fbo_pipeline_layout,
            text_effect_pipeline_layout,
			default_camera,
            draw_list: Vec::new(),
            rebatch: false,
            posts: Vec::new(),

            pass_toop_list: Default::default(),
            next_node_with_depend: Default::default(),
            debug_info: VecMap::default(),
            draw_screen_range: 0..0,
		}
    }
	
	
}

impl InstanceContext {
	pub fn get_or_create_pipeline(&mut self, device: &RenderDevice, blend_state: wgpu::BlendState, has_depth: bool, is_fbo: bool, is_opacity: bool) -> Share<wgpu::RenderPipeline> {
		let hash = calc_hash(&(blend_state, has_depth, is_fbo, is_opacity), 0);

		match self.pipeline_cache.entry(hash) {
			Entry::Occupied(r) => r.get().clone(),
			Entry::Vacant(r) => {
                let (pipeline_layout, fs, name) = if is_fbo {
                    if is_opacity {
                        (&self.fbo_pipeline_layout, &self.fs_opacity_fbo, "single opacity ui pipeline")
                    } else {
                        (&self.fbo_pipeline_layout, &self.fs_fbo, "single ui pipeline")
                    }
                } else {
                    if is_opacity {
                        (&self.pipeline_layout, &self.fs_opacity, "batch opacity ui pipeline")
                    } else {
                        (&self.pipeline_layout, &self.fs, "batch ui pipeline")
                    }
                };
				let pipeline = Share::new(create_render_pipeline(
                    name, &device, pipeline_layout, &self.vs, fs, Some(blend_state), CompareFunction::GreaterEqual, has_depth, wgpu::TextureFormat::pi_render_default(), vert_layout().as_slice(), MeterialBind::SIZE, true, FrontFace::Ccw));
				r.insert(pipeline.clone());
				pipeline
			},
		}
	}

	pub fn update(&mut self, device: &RenderDevice, queue: &RenderQueue) {
        self.instance_data.merge_ranges();
        log::trace!(
            "update instance_buffer={:?}",
            (
                &self.instance_data.dirty_range,
                self.instance_data.dirty_range.len(),
                &self.instance_data.merge_ranges,
                self.instance_data.size,
                &self.instance_data.merge_ranges.len()
            )
        );

		Self::update1(device, queue, &mut self.instance_data, &mut self.instance_buffer);
        Self::update1(device, queue, &mut self.text_shadow_h_instance_data, &mut self.text_shadow_h_instance_buffer);
        Self::update1(device, queue, &mut self.text_shadow_v_instance_data, &mut self.text_shadow_v_instance_buffer);
        Self::update1(device, queue, &mut self.text_gray_instance_data, &mut self.text_gray_instance_buffer);
        Self::update1(device, queue, &mut self.text_glow_instance_data, &mut self.text_glow_instance_buffer);

        Self::update1(device, queue, &mut self.svg_shadow_h_instance_data, &mut self.svg_shadow_h_instance_buffer);
        Self::update1(device, queue, &mut self.svg_shadow_v_instance_data, &mut self.svg_shadow_v_instance_buffer);
        Self::update1(device, queue, &mut self.svg_gray_instance_data, &mut self.svg_gray_instance_buffer);
        Self::update1(device, queue, &mut self.svg_glow_instance_data, &mut self.svg_glow_instance_buffer);
		// Self::update1(device, queue, &mut self.depth_data, &mut self.depth_buffer);
		
	}

	pub fn update1(device: &RenderDevice, queue: &RenderQueue, instance_data: &mut GpuBuffer, instance_buffer: &mut Option<(wgpu::Buffer, usize)>) {
        
		if instance_data.dirty_range.len() != 0 { 
            // log::debug!("update instance_buffer==============={:?}, {:?}", &instance_data.dirty_range, bytemuck::cast_slice::<u8, f32>(&instance_data.data[instance_data.dirty_range.clone()]));
            
			if let Some((buffer, size)) = &instance_buffer {
				if *size >= instance_data.dirty_range.end {
                    if !instance_data.merge_ranges.is_empty() {
                        for range in &instance_data.merge_ranges {
                            queue.write_buffer(
                                &buffer,
                                range.start as u64,
                                &instance_data.data()[range.clone()],
                            );
                        }
                    } else {
                        queue.write_buffer(
                            buffer,
                            instance_data.dirty_range.start as u64,
                            &instance_data.data()[instance_data.dirty_range.clone()],
                        );
                    // }
                    instance_data.reset_count_state(); 
					return;
				}

			}
            let len = instance_data.data.capacity();

            let buffer: wgpu::Buffer = (***device).create_buffer(&BufferDescriptor {
                label: Some("instance_buffer"),
                size: len as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            if !instance_data.merge_ranges.is_empty() {
                for range in &instance_data.merge_ranges {
                    queue.write_buffer(
                        &buffer,
                        range.start as u64,
                        &instance_data.data()[range.clone()],
                    );
                } 
            } else {
                queue.write_buffer(
                    &buffer,
                    instance_data.dirty_range.start as u64,
                    &instance_data.data()[instance_data.dirty_range.clone()],
                );
            }

            *instance_buffer = Some((buffer, len));
			
			// log::trace!("create instance_buffer={:?}", instance_data.data());
			instance_data.reset_count_state(); 
		}
	}
}

use super::RenderObjType;

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

#[derive(Debug, Deref, Default)]
pub struct DrawObjDefaults(pub VecMap<DrawObjDefault>);

#[derive(Debug)]
pub struct DrawObjDefault {
    pub blend_state: BlendState,
}

impl DrawObjDefault {
    pub fn add(world: &mut World, ty: RenderObjType, state: DrawObjDefault) {
        let drawobj_defaults = match world.get_single_res_mut::<DrawObjDefaults>() {
            Some(r) => r,
            None => {
                world.insert_single_res(DrawObjDefaults::default());
                world.get_single_res_mut::<DrawObjDefaults>().unwrap()
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
        //         defines: &[],
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
        //         defines: &[],
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
#[derive(Debug)]
pub struct UnitQuadBuffer {
    pub vertex: Handle<RenderRes<Buffer>>,
    pub uv: Handle<RenderRes<Buffer>>,
    pub index: Handle<RenderRes<Buffer>>,
}
impl FromWorld for UnitQuadBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_single_res::<PiRenderDevice>().expect("create UnitQuadBuffer need RenderDevice");
        let buffer_asset_mgr = world
            .get_single_res::<ShareAssetMgr<RenderRes<Buffer>>>()
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

/// 动态分配的纹理，清屏颜色的bindgroup（透明色）
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

pub struct GroupAlloterCenter<>(Vec<Share<GroupAlloter>>, wgpu::Limits);

impl FromWorld for GroupAlloterCenter {
    fn from_world(world: &mut World) -> Self {
        let limits = world.get_single_res::<PiRenderDevice>().unwrap().limits();
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

/// buffer累的的binding组的分配器
pub struct ShareGroupAlloter<T, M = ()> {
    pub group_index: u32,
    alloter: Share<GroupAlloter>,
    mark: PhantomData<(T, M)>,
}

impl<T, M> std::ops::Deref for ShareGroupAlloter<T, M> {
    type Target = Share<GroupAlloter>;

    fn deref(&self) -> &Self::Target { &self.alloter }
}

pub fn bind_group_layout(entrys: &[wgpu::BindGroupLayoutEntry], device: &PiRenderDevice) -> Share<BindGroupLayout> {
    Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: entrys,
    }))
}

impl<M> FromWorld for ShareGroupAlloter<CameraGroup, M> {
    fn from_world(world: &mut World) -> Self {
        // world.init_single_res::<ShaderInfoCache>();
        world.init_single_res::<GroupAlloterCenter>();
        let world1 = world.unsafe_world();
        // let mut cache = world.get_single_res_mut::<ShaderInfoCache>().unwrap();
        let device = world1.get_single_res::<PiRenderDevice>().unwrap();
        let mut world2 = world.unsafe_world();
        let group_center = world2.get_single_res_mut::<GroupAlloterCenter>().unwrap();

        let limits = group_center.limits();
        let min_alignment = limits.min_uniform_buffer_offset_alignment;
        let max_binding_size = limits.max_uniform_buffer_binding_size;

        let entry = CameraBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
        let layout = Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[entry],
        }));
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
                mipmap_filter: wgpu::FilterMode::Nearest,
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
        let device = world.get_single_res::<PiRenderDevice>().unwrap();
        Self::new(device)
    }
}

// /// 将pass2d组织为层的结构
// #[derive(Deref, Default, DerefMut)]
// pub struct LayerPass2D (LayerDirty<Entity>);

// 如果是sdf2方案，会有第二张纹理
#[derive(Default)]
pub struct TextTextureGroup(pub Option<Handle<RenderRes<BindGroup>>>, pub Option<Handle<RenderRes<BindGroup>>>);

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
            cull_mode: Some(wgpu::Face::Back),
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
            cull_mode: Some(wgpu::Face::Back),
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

// 最大视口尺寸（gui中，各渲染共用同一个深度缓冲区， 统计各视口的最大尺寸，用该尺寸作为深度缓冲区的大小）
#[derive(Debug, Default, Clone)]
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


// /// depth BindGroup缓存
// #[derive(Default)]
// pub struct DepthCache {
//     pub list: Vec<DrawBindGroup>,
//     // pub layout: Share<BindGroupLayout>,
// }

// impl FromWorld for DepthCache {
//     fn from_world(world: &mut pi_world::world::World) -> Self {
//         world.init_single_res::<ShaderInfoCache>();
//         let world = world.cell();
//         // let mut cache = world.get_single_res_mut::<ShaderInfoCache>().unwrap();
//         // bind_group_layout
//         // let device = world.get_single_res::<PiRenderDevice>().unwrap();
//         // let mut entry = DepthBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
//         // if let BindingType::Buffer { has_dynamic_offset, .. } = &mut entry.ty {
//         //     *has_dynamic_offset = false;
//         // }
//         // let layout = cache.bind_group_layout(&[entry], &device);
//         Self { list: Vec::new(), /*layout*/ }
//     }
// }

// impl DepthCache {
//     pub fn or_create_depth<'a>(
// 		&mut self, cur_depth: usize, 
// 		depth_alloter: &'a ShareGroupAlloter<DepthGroup>
// 	) {
//         let mut depth = self.list.len();
//         while depth <= cur_depth {
//             let mut group = depth_alloter.alloc();
//             let _ = group.set_uniform(&DepthUniform(&[depth as f32]));
//             // 添加深度group、永不释放
//             self.list.push(DrawBindGroup::Offset(group));
//             depth += 1;
//         }
//     }
// }

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
pub struct TargetCacheMgr(pub Share<HomogeneousMgr<CacheTarget>>);


pub fn create_render_pipeline(
    labal: &str,
	device: &wgpu::Device, 
	pipeline_layout: &PipelineLayout, 
	vs: &wgpu::ShaderModule, 
	fs: &wgpu::ShaderModule,
	blend: Option<BlendState>,
	depth_compare: CompareFunction,
    has_depth: bool,
    format: wgpu::TextureFormat,
    vert_layout: &[wgpu::VertexAttribute],
    size: usize,
    depth_write_enabled: bool,
    front_face: wgpu::FrontFace,
) -> wgpu::RenderPipeline {
	let state = PipelineState {
        targets: vec![Some(wgpu::ColorTargetState {
            format: format,
            blend,
            write_mask: wgpu::ColorWrites::ALL,
        })],
        primitive: wgpu::PrimitiveState {
            front_face,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        depth_stencil: match has_depth {
            true => Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled,
                depth_compare,
                // depth_compare: CompareFunction::Always,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            false => None,
        },
        multisample: MultisampleState::default(),
        multiview: None,
    };

	let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        cache: None,
		label: Some(labal),
		layout: Some(&pipeline_layout),
		vertex: wgpu::VertexState {
            compilation_options: Default::default(),
			module: vs,
			entry_point: Some("main"),
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
					array_stride: size as u64,
					step_mode: wgpu::VertexStepMode::Instance,
					attributes: vert_layout,
				},
			],
		},
		fragment: Some(wgpu::FragmentState {
            compilation_options: Default::default(),
			module: fs,
			entry_point: Some("main"),
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
// 	buffer: GpuBuffer,
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
// #[derive(Clone)]
// pub struct GpuArrayBufferIndex<T: > {
//     /// The index to use in a shader into the array.
//     pub index: NonMaxU32,
//     /// The dynamic offset to use when setting the bind group in a pass.
//     /// Only used on platforms that don't support storage buffers.
//     pub dynamic_offset: Option<NonMaxU32>,
//     pub element_type: PhantomData<T>,
// }



