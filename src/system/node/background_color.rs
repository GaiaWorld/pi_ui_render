use std::{collections::hash_map::Entry, borrow::Cow};
use std::io::Result;

use naga::{ShaderStage};
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{Query, Changed, EntityCommands, Commands, Write, ResMut, Res, res::WriteRes, Event, Id};
use pi_ecs_macros::{listen, setup};
use pi_hash::XHashMap;
use pi_map::vecmap::VecMap;
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::{device::RenderDevice, shader::{Shader, ShaderProcessor, ShaderId}};
use pi_share::Share;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_slotmap::DefaultKey;
use wgpu::{IndexFormat, DepthStencilState, TextureFormat, CompareFunction, StencilState, DepthBiasState};

use crate::components::user::CgColor;
use crate::resource::draw_obj::ShaderMap;
use crate::utils::tools::calc_hash;
use crate::{components::{user::{Node, BackgroundColor, Color}, calc::{NodeId, DrawList}, draw_obj::{IsUnitQuad, DrawObject, DrawState, VSDefines, FSDefines, ShaderKey, PipelineKey, VertexBufferLayoutKey}}, resource::draw_obj::{Shaders, PipelineState, StateMap, VertexBufferLayouts, VertexBufferLayout, VertexBufferLayoutMap, ShaderStatic, ShaderCatch, UnitQuadBuffer, ShareLayout, Program}, utils::shader_helper::{WORLD_MATRIX_GROUP, DEPTH_GROUP, PROJECT_GROUP, VIEW_GROUP}};
// use crate::utils::tools::calc_hash;

pub struct CalcBackGroundColor;

#[setup]
impl CalcBackGroundColor {
	#[init]
	pub fn init(
		mut shader_static_map: ResMut<Shaders>,
		mut state_map: ResMut<StateMap>,
		mut vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		device: Res<RenderDevice>,
		mut static_index: WriteRes<BackgroundStaticIndex>,
	) {
		let shader_static = create_shader_static(
			&share_layout, 
			&mut shader_catch, 
			&mut shader_map,  
			&device);

		shader_static_map.0.push(shader_static);
		let shader_index = shader_static_map.0.len() - 1;

		let pipeline_state = create_pipeline_state();
		let pipeline_state = state_map.insert(pipeline_state);

		let vertex_buffer = create_vertex_buffer_layout();
		let vertex_buffer_index = vertex_buffer_map.insert(vertex_buffer);

		// 插入背景颜色shader的索引
		static_index.write(BackgroundStaticIndex {
			shader: shader_index,
			pipeline_state,
			vertex_buffer_index,
		});
	}
	/// 创建RenderObject，用于渲染背景颜色
	#[system]
	pub async fn calc_background(
		mut query: Query<Node, (Id<Node>, &BackgroundColor, Write<BackgroundDrawId>, Write<DrawList>), Changed<BackgroundColor>>,
		query_draw: Query<DrawObject, Write<DrawState>>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut is_unit_quad_commands: Commands<DrawObject, IsUnitQuad>,
		mut shader_id_commands: Commands<DrawObject, ShaderKey>,
		mut pipeline_state_commands: Commands<DrawObject, PipelineKey>,
		mut vertex_buffer_layout_commands: Commands<DrawObject, VertexBufferLayoutKey>,
		
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'static, RenderDevice>,
		static_index: Res<'static, BackgroundStaticIndex>,
		shader_static: Res<'static, Shaders>,
		unit_quad_buffer: Res<'static, UnitQuadBuffer>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
	) -> Result<()> {
		// log::info!("calc_background=================");
		let color_group_layout = match shader_static.get(static_index.shader) {
			Some(r) => r.bind_group.get(COLOR_GROUP).unwrap(),
			None => return Ok(()),
		};

		for (node, background_color, mut draw_index, mut render_list ) in query.iter_mut() {
			match draw_index.get() {
				// background_color已经存在一个对应的DrawObj， 则修改color group
				Some(r) => {
					let mut draw_state_item = query_draw.get_unchecked(**r);
					let draw_state = draw_state_item.get_mut().unwrap();
					modify_color_group(&background_color, draw_state, &device, &color_group_layout, &buffer_assets, &bind_group_assets).await;
					draw_state_item.notify_modify();
				},
				// 否则，创建一个新的DrawObj，并设置color group; 
				// 修改以下组件：
				// * <Node, BackgroundDrawId>
				// * <Node, DrawList>
				// * <DrawObject, DrawState>
				// * <DrawObject, NodeId>
				// * <DrawObject, IsUnitQuad>
				None => {
					// log::info!("create_background=================");
					// 创建新的DrawObj
					let new_draw_obj = draw_obj_commands.spawn();
					// 设置DrawState（包含color group）
					let mut draw_state = DrawState::default();
					modify_color_group(&background_color, &mut draw_state, &device, color_group_layout, &buffer_assets, &bind_group_assets).await;
					draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
					draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));
					
					draw_state_commands.insert(new_draw_obj, draw_state);
					// 建立DrawObj对Node的索引
					node_id_commands.insert(new_draw_obj, NodeId(node));
					is_unit_quad_commands.insert(new_draw_obj, IsUnitQuad(true));
					shader_id_commands.insert(new_draw_obj, ShaderKey(static_index.shader));
					pipeline_state_commands.insert(new_draw_obj, PipelineKey(static_index.pipeline_state));
					vertex_buffer_layout_commands.insert(new_draw_obj, VertexBufferLayoutKey(static_index.vertex_buffer_index));

					// 建立Node对DrawObj的索引
					draw_index.write(BackgroundDrawId(new_draw_obj));

					match render_list.get_mut() {
						Some(r) => {
							r.push(new_draw_obj);
							render_list.notify_modify();
						},
						None => {
							let mut r = DrawList::default();
							r.push(new_draw_obj);
							render_list.write(r);
						},
					};
				}
			}
		}
		return Ok(())
	}
}



#[derive(Deref, Default)]
pub struct BackgroundDrawId(Id<DrawObject>);

// 背景颜色 ShaderInfo的索引
pub struct BackgroundStaticIndex{
	pub shader: usize,
	pub pipeline_state: DefaultKey,
	pub vertex_buffer_index: DefaultKey,
}

pub const COLOR_GROUP: usize = 4;

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BackgroundColor, Delete), component=(Node, Node, Delete))]
pub fn background_color_delete(
	e: Event,
	query: Query<Node, &BackgroundDrawId>,
	mut draw_obj: EntityCommands<DrawObject>,
) {
	if let Some(index) = query.get_by_entity(e.id) {
		draw_obj.despawn(**index);
	}
}

async fn modify_color_group(
	color: &Color, 
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	color_group_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) {
	match color {
		Color::RGBA(color) => {
			let color_bind_group = create_reba_bind_group(color, device, color_group_layout, buffer_assets, bind_group_assets);
			// 插入到drawstate中
			draw_state.bind_groups.insert(COLOR_GROUP, color_bind_group);
		},
		_ => panic!("color is error..."),
	}
	
}

pub fn create_reba_bind_group(
	color: &CgColor,
	device: &RenderDevice, 
	color_group_layout: &BindGroupLayout,
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	bind_group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
) -> Handle<RenderRes<BindGroup>> {
	let key = calc_hash(color);
	match bind_group_assets.get(&key) {
		Some(r) => r,
		None => {
			let uniform_buf = match buffer_assets.get(&key) {
				Some(r) => r,
				None => {
					let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
						label: Some("color buffer init"),
						contents: bytemuck::cast_slice(&[color.x, color.y, color.z, color.w]),
						usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
					});
					buffer_assets.cache(key, RenderRes::new(uniform_buf, 5));
					buffer_assets.get(&key).unwrap()
				}
			};
			let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: color_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: uniform_buf.as_entire_binding(),
					},
				],
				label: Some("color group create"),
			});
			bind_group_assets.cache(key, RenderRes::new(group, 5));
			bind_group_assets.get(&key).unwrap()
		}
	}
}

/// 创建background相关的shader信息（静态信息，在运行时不变）
pub fn create_shader_static(
	share_layout: &ShareLayout, 
	shader_catch: &mut ShaderCatch, 
	shader_map: &mut ShaderMap,
	device: &RenderDevice,
) -> ShaderStatic {
	let color_static = ColorStatic::init(device).0;

	let mut bind_group_layout = VecMap::new();
	bind_group_layout.insert(COLOR_GROUP, color_static);
	// 通用Layout
	bind_group_layout.insert(WORLD_MATRIX_GROUP, share_layout.matrix.clone());
	bind_group_layout.insert(DEPTH_GROUP, share_layout.depth.clone());
	bind_group_layout.insert(PROJECT_GROUP, share_layout.project.clone());
	bind_group_layout.insert(VIEW_GROUP, share_layout.view.clone());

	let bg_shader = GlslShaderStatic::init(shader_catch, shader_map, ||{include_str!("../../source/shader/common.vert")}, ||{include_str!("../../source/shader/color.frag")});
	
	ShaderStatic {
		vs_shader_soruce: bg_shader.shader_vs,
		fs_shader_soruce: bg_shader.shader_fs,
		bind_group: bind_group_layout,
		create_shader_info,
	}
}

pub fn create_pipeline_state() -> PipelineState {
	PipelineState {
		targets: vec![wgpu::ColorTargetState {
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			blend: Some(wgpu::BlendState {
				color: wgpu::BlendComponent {
					operation: wgpu::BlendOperation::Add,
					src_factor: wgpu::BlendFactor::SrcAlpha,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
				},
				alpha: wgpu::BlendComponent::REPLACE,
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
		multisample: wgpu::MultisampleState::default(),
		multiview: None,
	}
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayouts {
	vec![VertexBufferLayout {
		array_stride: 8 as wgpu::BufferAddress,
		step_mode: wgpu::VertexStepMode::Vertex,
		attributes: vec![
			wgpu::VertexAttribute {
				format: wgpu::VertexFormat::Float32x2,
				offset: 0,
				shader_location: 0,
			},
		],
	}]
}

fn create_shader_info(
	vs_shader_id: &ShaderId,
	fs_shader_id: &ShaderId,
	vs_defines: &VSDefines, 
	fs_defines: &FSDefines, 
	bind_group_layout: VecMap<Share<BindGroupLayout>>,
	device: &RenderDevice,
	shaders: &XHashMap<ShaderId, Shader>,
) -> Program {
	let processor = ShaderProcessor::default();
	let imports = XHashMap::default();

	let vs = processor
            .process(&vs_shader_id, vs_defines, shaders, &imports)
            .unwrap();
	let vs = vs.get_glsl_source().unwrap();
	let vs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("bg_color_vs_shader_module"),
		source: wgpu::ShaderSource::Glsl {
			shader: Cow::Borrowed(vs),
			stage: naga::ShaderStage::Vertex,
			defines: naga::FastHashMap::default(),
		},
	});

	let fs = processor
            .process(&fs_shader_id, fs_defines, shaders, &imports)
            .unwrap();
	let fs = fs.get_glsl_source().unwrap();
	let fs = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("bg_color_fs_shader_module"),
		source: wgpu::ShaderSource::Glsl {
			shader: Cow::Borrowed(fs),
			stage: naga::ShaderStage::Fragment,
			defines: naga::FastHashMap::default(),
		},
	});
	

	// 根据defines， 删除layout(TODO)
	
	let mut v = Vec::new();
	for r in bind_group_layout.iter() {
		if let Some(r) = r {
			v.push(&***r);
		}
	}
	//list_share_as_ref(bind_group_layout.iter());
	let slice = v.as_slice();
	println!("len===={}, {}", slice.len(), v.len());
	let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("cerate bg_color pipeline_layout"),
		bind_group_layouts: slice,
		push_constant_ranges: &[],
	});
	
	Program {
		pipeline_layout: Share::new(pipeline_layout),
		vs_shader: Share::new(vs),
		fs_shader: Share::new(fs),
	}
}

pub struct GlslShaderStatic {
	pub shader_vs: ShaderId,
	pub shader_fs: ShaderId,
}

impl Clone for GlslShaderStatic {
    fn clone(&self) -> Self {
        Self { shader_vs: self.shader_vs.clone(), shader_fs: self.shader_fs.clone() }
    }
}

impl GlslShaderStatic {
    fn init(shader_catch: &mut ShaderCatch, shader_map: &mut ShaderMap, load_vs: impl Fn() -> &'static str, load_fs: impl Fn() -> &'static str) -> Self {
		let (shader_vs, shader_fs) = {
			(
				match shader_map.entry(COLOR_SHADER_VS) {
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
				match shader_map.entry(COLOR_SHADER_FS) {
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

#[derive(Deref)]
pub struct ColorStatic(Share<BindGroupLayout>);

impl ColorStatic {
	fn init(device: &RenderDevice) -> Self {
		ColorStatic(Share::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("color_layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: wgpu::BufferSize::new(16), // rgba四个通道，每个通道为一个f32, 大小为 4 * 4（每个通道一个u8， todo）
					},
					count: None,
				},
			],
		})))
	}
}

const COLOR_SHADER_VS: &'static str = "color_shader_vs";
const COLOR_SHADER_FS: &'static str = "color_shader_fs";

