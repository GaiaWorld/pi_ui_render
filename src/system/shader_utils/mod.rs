//! 一些关于shader的静态信息与rust的对应，通常Shader一旦确定，这些对应关系就确定了
//! 其中包括： GroupLayout、SharderModule、pipline_state、vertLayout等
//! 在wgpu中，一些信息是可变的，如pipline_state、GroupLayout中的一些描述。
//! 目前上不知道这些信息在何种情况下、需要怎样变化
//! 将他们的值认为是确定的，目前对编程没有影响
//! TODO: 后续，可能将不可变因素通过shader静态编译出来（尚不确定哪些通常不变），当前通过手动编写代码的方式来确定
//! 

use std::collections::hash_map::Entry;

use naga::ShaderStage;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader}, bind_group_layout::BindGroupLayout, asset::RenderRes, buffer::Buffer, bind_group::BindGroup};
use pi_share::Share;
use pi_slotmap::DefaultKey;
use wgpu::{DepthStencilState, TextureFormat, CompareFunction, StencilState, DepthBiasState, MultisampleState};

use crate::{resource::draw_obj::{ShareLayout, ShaderCatch, ShaderMap, ShaderStatic, Program, PipelineState}, utils::{shader_helper::{WORLD_MATRIX_GROUP, DEPTH_GROUP, PROJECT_GROUP, VIEW_GROUP}, tools::{calc_float_hash, calc_hash}}, components::{draw_obj::{FSDefines, VSDefines}, user::Matrix4}};

use super::pass::pass_render::DEPTH;

pub mod post_process;
pub mod with_vert_color;
pub mod text;
pub mod color;
pub mod image;
pub mod color_shadow;

pub struct GlslShaderStatic {
	pub shader_vs: ShaderId,
	pub shader_fs: ShaderId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticIndex {
	pub shader: usize,
	pub pipeline_state: DefaultKey,
	pub vertex_buffer_index: DefaultKey,
	pub name: &'static str,
}

impl GlslShaderStatic {
    fn init(
		vs_name: &'static str,
		fs_name: &'static str,
		shader_catch: &mut ShaderCatch, 
		shader_map: &mut ShaderMap, 
		load_vs: impl Fn() -> &'static str, 
		load_fs: impl Fn() -> &'static str
	) -> Self {
		let (shader_vs, shader_fs) = {
			(
				match shader_map.entry(vs_name) {
					Entry::Vacant(r) => {
						let shader = Shader::from_glsl(
							load_vs(), 
							ShaderStage::Vertex);
						let r = r.insert(shader.id()).clone();
						shader_catch.insert(shader.id(), shader);
						r
					},
					Entry::Occupied(r) =>r.get().clone()
				},
				match shader_map.entry(fs_name) {
					Entry::Vacant(r) => {
						let shader = Shader::from_glsl(
							load_fs(), 
							ShaderStage::Fragment);
						let r = r.insert(shader.id()).clone();
						shader_catch.insert(shader.id(), shader);
						r
					},
					Entry::Occupied(r) => r.get().clone()
				}
			)
		};
		Self {
			shader_vs,
			shader_fs
		}
	}
}

pub fn create_shader_common_static(
	share_layout: &ShareLayout, 
	shader: GlslShaderStatic,
	create_shader_info: fn (
		vs_shader_soruce: &ShaderId,
		fs_shader_soruce: &ShaderId,
		vs_defines: &VSDefines, 
		fs_defines: &FSDefines, 
		bind_group_layout: VecMap<Share<BindGroupLayout>>,
		empty_group_layout: &Share<BindGroupLayout>,
		device: &RenderDevice,
		shaders: &XHashMap<ShaderId, Shader>,
	) -> Program
) -> ShaderStatic {
	let mut bind_group_layout = VecMap::new();
	// 通用Layout
	bind_group_layout.insert(WORLD_MATRIX_GROUP, share_layout.matrix.clone());
	bind_group_layout.insert(DEPTH_GROUP, share_layout.depth.clone());
	bind_group_layout.insert(PROJECT_GROUP, share_layout.project.clone());
	bind_group_layout.insert(VIEW_GROUP, share_layout.view.clone());

	ShaderStatic {
		vs_shader_soruce: shader.shader_vs,
		fs_shader_soruce: shader.shader_fs,
		bind_group: bind_group_layout,
		create_shader_info,
	}
}

pub fn create_camera_bind_group(
	view: &Matrix4,
	layout: &BindGroupLayout,
	device: &RenderDevice,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_float_hash(view.as_slice());

	match bind_group_assets.get(&key) {
		Some(r) => r,
		None => {
			let buf = match buffer_assets.get(&key) {
				Some(r) => r,
				None => {
					let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("camera buffer init"),
						contents: bytemuck::cast_slice(view.as_slice()),
						usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
					});
					buffer_assets.cache(key, RenderRes::new(buf, 5));
					buffer_assets.get(&key).unwrap()
				}
			};
			let group = device.create_bind_group(
				&wgpu::BindGroupDescriptor {
					layout: &layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: buf.as_entire_binding(),
						},
					],
					label: Some("camera create"),
				}
			);
			bind_group_assets.cache(key, RenderRes::new(group, 5));
			bind_group_assets.get(&key).unwrap()
		}
	}
	
}

pub fn create_depth_group(
	cur_depth: usize,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
	depth_cache: &mut Vec<Handle<RenderRes<BindGroup>>>,
	device: &RenderDevice,
	share_layout: &ShareLayout,
) -> Handle<RenderRes<BindGroup>> {
	match depth_cache.get(cur_depth) {
		Some(r) => r.clone(),
		None => {
			let value = cur_depth as f32 / 600000.0;
			let key = calc_hash(&(DEPTH.clone(), cur_depth)); // TODO
			let d = match bind_group_assets.get(&key) {
				Some(r) => r,
				None => {
					let uniform_buf = match buffer_assets.get(&key) {
						Some(r)=> r,
						None => {
							let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
								label: Some("depth buffer init"),
								contents: bytemuck::cast_slice(&[value]),
								usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
							});
							buffer_assets.cache(key, RenderRes::new(uniform_buf, 5));
							buffer_assets.get(&key).unwrap()
						}
					};
					let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
						layout: &share_layout.depth,
						entries: &[
							wgpu::BindGroupEntry {
								binding: 0,
								resource: uniform_buf.as_entire_binding(),
							},
						],
						label: Some("depth group create"),
					});
					bind_group_assets.cache(key, RenderRes::new(group, 5));
					bind_group_assets.get(&key).unwrap()
				},
			};
			depth_cache.push(d.clone());
			d
		}
	}
}

pub fn create_common_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![wgpu::ColorTargetState {
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
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
		}],
		primitive: wgpu::PrimitiveState {
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			..Default::default()
		},
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth32Float,
			depth_write_enabled: true,
			depth_compare: CompareFunction::Always,
			stencil: StencilState::default(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState::default(),
		multiview: None,
	}
}