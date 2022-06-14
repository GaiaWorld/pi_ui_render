use std::collections::hash_map::Entry;

use naga::ShaderStage;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::{device::RenderDevice, shader::{ShaderId, Shader}, bind_group_layout::BindGroupLayout, asset::RenderRes, buffer::Buffer, bind_group::BindGroup};
use pi_share::Share;
use pi_slotmap::DefaultKey;

use crate::{resource::draw_obj::{ShareLayout, ShaderCatch, ShaderMap, ShaderStatic, Program}, utils::{shader_helper::{WORLD_MATRIX_GROUP, DEPTH_GROUP, PROJECT_GROUP, VIEW_GROUP}, tools::calc_float_hash}, components::{draw_obj::{FSDefines, VSDefines}, user::Matrix4}};

pub mod post_process;
pub mod image;
pub struct GlslShaderStatic {
	pub shader_vs: ShaderId,
	pub shader_fs: ShaderId,
}

pub struct StaticIndex {
	pub shader: usize,
	pub pipeline_state: DefaultKey,
	pub vertex_buffer_index: DefaultKey,
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