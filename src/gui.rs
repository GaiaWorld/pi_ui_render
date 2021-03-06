use std::{any::TypeId, mem::{replace, size_of}, sync::Arc};

use pi_assets::{mgr::AssetMgr, asset::{GarbageEmpty, Handle}, homogeneous::HomogeneousMgr};
use pi_ecs::{prelude::{World, ArchetypeId, StageBuilder, Id, Setup, FromWorld, QueryState, Join, OrDefault}, component::MultiCaseImpl};
use pi_ecs_utils::prelude::{NodeDown, NodeUp, Layer};
use pi_render::{
	rhi::{
		asset::{RenderRes, TextureRes},
		buffer::Buffer, 
		bind_group::BindGroup, 
		device::RenderDevice, RenderQueue
	}, 
	components::view::target_alloc::{SafeAtlasAllocator, DEPTH_TEXTURE, ShareTargetView, UnuseTexture},
	font::FontSheet,
};
use pi_share::{Share, ShareCell, cell::TrustCell};
use wgpu::TextureView;

use crate::{
	components::{
		user::{ClassName, Aabb2, Point2, BorderImage, BackgroundImage, Overflow}, 
		pass_2d::{RenderTarget, ScreenTarget, Pass2D, ParentPassId}, calc::{BorderImageTexture, BackgroundImageTexture, LayoutResult, Quad, WorldMatrix, ZRange, IsEnable, InPassId, NodeId}
	}, 
	resource::{UserCommands, NodeCommand, Viewport, draw_obj::CommonSampler, DefaultStyle}, 
	utils::{style::{style_sheet::{StyleAttr, Attr, ClassSheet}, style_parse::parse_class_map_from_string}, tools::calc_hash, cmd::Command}, 
	system::{
		node::{user_setting::CalcUserSetting, context::CalcContext, z_index::CalcZindex, layout::CalcLayout, quad::CalcQuad, world_matrix::CalcMatrix, content_box::CalcContentBox, background_color::CalcBackGroundColor, context_opacity::{CalcOpacity, CalcOpacityPostProcess}, context_transform_will_change::CalcTransformWillChange, context_overflow::CalcOverflow, context_root::CalcRoot, text::CalcText, text_split::CalcTextSplit, text_glphy::CalcTextGlyph, border_image::CalcBorderImage, border_color::CalcBorderColor, box_shadow::CalcBoxShadow, background_image::CalcBackgroundImage, image_texture_load::CalcImageLoad}, 
		draw_obj::{world_marix::CalcWorldMatrixGroup, pipeline::CalcPipeline}, 
		pass::{
			pass_render::CalcRender, 
			pass_dirty_rect::CalcDirtyRect, 
			pass_graph_node::{InitGraphData, PostBindGroupLayout}
		}, 
		shader_utils::{post_process::CalcPostProcessShader, with_vert_color::WithColorShader, text::TextShader, color::ColorShader, image::ImageShader, color_shadow::ColorShadowShader}
	}
};

use crate::components::user::Node;

pub struct Gui {
	pub world: World,

	user_commands: UserCommands,

	node_archetype_id: ArchetypeId,

	pub down_query: QueryState<Node, &'static NodeDown<Node>>,
	pub up_query: QueryState<Node, &'static NodeUp<Node>>,
	pub layer_query: QueryState<Node, &'static Layer>,
	pub enable_query: QueryState<Node, &'static IsEnable>,
	pub depth_query: QueryState<Node, &'static ZRange>,
	pub layout_query: QueryState<Node, &'static LayoutResult>,
	pub quad_query: QueryState<Node, &'static Quad>,
	pub matrix_query: QueryState<Node, &'static WorldMatrix>,
	pub overflow_query: QueryState<Pass2D, Join<NodeId, Node, (&'static Quad, OrDefault<Overflow>, Option<&'static ParentPassId>)>>,
	pub in_pass2d_query: QueryState<Node, &'static InPassId>,

	// node_archetype: ArchetypeId,
	pub quad_component_comtainer: Arc<TrustCell<MultiCaseImpl<Quad>>>,
}

impl Gui {
	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn new(world: &mut World) -> Gui {
		world.new_archetype::<Node>().create(); // ??????Node??????

		// ?????????????????????
		register_assets_mgr(world);

		let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

		// let dispatcher= SingleDispatcher::new(rt);

		let archetype_id = world.archetypes_mut().get_or_create_archetype::<Node>();
		let quad_id = world.get_or_register_component::<Quad>(archetype_id);
		let c = unsafe { world.archetypes().get(archetype_id).unwrap().get_component(quad_id)};
		let quad = match c.clone().downcast() {
			Ok(r) => {
				let r: Arc<TrustCell<MultiCaseImpl<Quad>>> = r;
				r
			},
			Err(_) => panic!("downcast fail")
		};
		// let archetype = 
		Gui {
			world: world.clone(),
			node_archetype_id,
			user_commands: UserCommands::default(),

			down_query: world.query(),
			up_query: world.query(),
			layer_query: world.query(),
			enable_query: world.query(),
			depth_query: world.query(),
			layout_query: world.query(),
			quad_query: world.query(),
			matrix_query: world.query(),
			in_pass2d_query: world.query(),
			overflow_query: world.query(),

			quad_component_comtainer: quad,
		}
	}

	/// ?????????gui
	/// ???????????????????????????DeviceRender?????????resource???
	pub fn init(
		&mut self, 
		x: u32, 
		y: u32, 
		width: u32, 
		height: u32,
	) -> Vec<StageBuilder> {
		// ??????????????????
		insert_resource(&mut self.world, x, y, width, height);

		init_stage(&mut self.world)
	}

	// ????????????
	pub fn create_node(&mut self) -> Id<Node> {
		let node_archetype_id = self.node_archetype_id;
		let r = unsafe { Id::new(self.world.archetypes_mut()[node_archetype_id].reserve_entity()) };
		r
	}

	/// ????????????????????????????????????
	pub fn append(&mut self, entity: Id<Node>, parent: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
		// println!("append====={:?}, {:?}", entity, parent);
	}

	/// ????????????????????????????????????
	pub fn insert_before(&mut self, entity: Id<Node>, anchor: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
		// println!("insert_before====={:?}, {:?}", entity, anchor);
	}

	/// ???????????????????????????
	pub fn remove_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
		// println!("insert_before====={:?}", entity.clone());
	}

	/// ??????????????????????????????????????????????????????????????????
	pub fn destroy_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
	}

	/// ??????????????????
	pub fn set_style<T: Attr>(&mut self, entity: Id<Node>, value: T){
		let start = self.user_commands.style_commands.style_buffer.len();
		unsafe {StyleAttr::write(
			value,
			&mut self.user_commands.style_commands.style_buffer,
		)};
		if let Some(r) = self.user_commands.style_commands.commands.last_mut() {
			if r.0 == entity {
				r.2 = self.user_commands.style_commands.style_buffer.len();
				return;
			}
		}
		self.user_commands.style_commands.commands.push((entity, start, self.user_commands.style_commands.style_buffer.len()));
	}

	/// ???????????????????????????????????????
	pub fn set_default_style_by_bin(&mut self, bin: &[u8]) {
		let class_sheet_new: ClassSheet = match bincode::deserialize(bin) {
			Ok(r) => r,
			Err(e) => {
				log::error!("deserialize ClassSheet error: {:?}", e);
				return;
			}
		};

		let class_sheet = self.world.get_resource_mut::<ClassSheet>();
		let class_sheet = match class_sheet {
			Some(r) => r,
			None => {
				self.world.insert_resource(ClassSheet::default());
				self.world.get_resource_mut::<ClassSheet>().unwrap()
			}
		};
		class_sheet.extend_from_class_sheet(class_sheet_new);

	}

	/// ?????????????????????????????????
	pub fn set_default_style_by_str(&mut self, class: &str) {
		let class_sheet = self.world.get_resource_mut::<ClassSheet>();
		let class_sheet = match class_sheet {
			Some(r) => r,
			None => {
				self.world.insert_resource(ClassSheet::default());
				self.world.get_resource_mut::<ClassSheet>().unwrap()
			}
		};
		match parse_class_map_from_string(class, class_sheet) {
			Ok(_r) => self.world.insert_resource(DefaultStyle), // ??????DefaultStyle??????
			Err(e) => {
				log::error!("set_default_style_by_str fail, parse style err: {:?}", e);
				return
			},
		};
	}

	/// ???????????????class
	pub fn set_class(&mut self, entity: Id<Node>, value: ClassName){
		self.user_commands.class_commands.push((entity, value));
	}

	/// ????????????
	pub fn push_cmd<T: Command>(&mut self, cmd: T) {
		self.user_commands.other_commands.push(cmd);
	}

	/// ??????gui??????
	pub fn run(&mut self) {
		let node_archetype_id = self.node_archetype_id;
		self.world.archetypes_mut()[node_archetype_id].flush();
		let commands = replace(&mut self.user_commands, UserCommands::default());
		self.world.insert_resource(commands);
	}
}

fn register_assets_mgr(world: &mut World) {
	world.insert_resource(AssetMgr::<RenderRes<Buffer>>::new(
		GarbageEmpty(),
		false,
		20 * 1024 * 1024,
		3 * 60 * 1000));
	world.insert_resource(AssetMgr::<RenderRes<BindGroup>>::new(
		GarbageEmpty(), 
		false,
		5 * 1024, 
		3 * 60 * 1000));
	world.insert_resource(AssetMgr::<RenderRes<TextureView>>::new(
		GarbageEmpty(), 
		false,
		60 * 1024 * 1024, 
		3 * 60 * 1000));
	world.insert_resource(AssetMgr::<TextureRes>::new(
		GarbageEmpty(), 
		false,
		60 * 1024 * 1024, 
		3 * 60 * 1000));
	world.insert_resource(HomogeneousMgr::<RenderRes<UnuseTexture>>::new(
		pi_assets::homogeneous::GarbageEmpty(), 
		10 * size_of::<UnuseTexture>(),
		size_of::<UnuseTexture>(),
		3 * 60 * 1000));
}

// ?????????????????????
fn insert_resource(
	world: &mut World,
	x: u32, 
	y: u32, 
	width: u32, 
	height: u32,
) {
	let texture_res_mgr = world.get_resource::<Share<AssetMgr<RenderRes<TextureView>>>>().unwrap().clone();
	let texture_res_mgr1 = world.get_resource::<Share<AssetMgr<TextureRes>>>().unwrap().clone();
	let unuse_texture_res_mgr = world.get_resource::<Share<HomogeneousMgr<RenderRes<UnuseTexture>>>>().unwrap().clone();
	let device = world.get_resource::<RenderDevice>().unwrap().clone();
	let queue = world.get_resource::<RenderQueue>().unwrap().clone();

	let view_port = Viewport(Aabb2::new(Point2::new(x as f32, y as f32), Point2::new((x + width) as f32, (y + height) as f32)));
	
	// ??????gui?????????????????????
	let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, width, height);
	world.insert_resource(ScreenTarget {
		aabb: view_port.0.clone(),
		depth: Some(depth_buffer), // ???????????????
		// depth: None,
	});

	let allocator = SafeAtlasAllocator::new(device.clone(), texture_res_mgr.clone(), unuse_texture_res_mgr);
	let dyn_target_type = InitGraphData::create_dyn_target_type(&allocator, &view_port);

	// ?????????????????????target??????
	let last_target = allocator.allocate::<&ShareTargetView, _>(
		(view_port.maxs.x - view_port.mins.x).ceil() as u32,
		(view_port.maxs.y - view_port.mins.y).ceil() as u32,
		dyn_target_type.has_depth,
		[].iter()
	);
	// ????????????????????????
	world.insert_resource(RenderTarget::OffScreen(last_target));
	
	// ?????????????????????
	world.insert_resource(allocator);

	// ??????????????????
	world.insert_resource(dyn_target_type);

	// ????????????
	world.insert_resource(view_port);

	// ??????PostBindGroupLayout
	let post_layout = PostBindGroupLayout::from_world(world);
	world.insert_resource(post_layout);

	// ??????CommonSampler
	let common_sampler = CommonSampler::from_world(world);
	world.insert_resource(common_sampler);
	
	// ??????FontSheet
	world.insert_resource(Share::new(ShareCell::new(FontSheet::new(&device,&texture_res_mgr1, &queue))));
	
}

// ?????????????????????
fn create_depth_buffer(
	texture_res_mgr: &Share<AssetMgr<RenderRes<TextureView>>>,
	device: &RenderDevice,
	width: u32,
	height: u32,
) -> Handle<RenderRes<TextureView>> {
	let texture = (**device).create_texture(&wgpu::TextureDescriptor {
		label: Some("first depth buffer"),
		size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Depth32Float,
		usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
	});
	let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

	let hash = calc_hash(&(DEPTH_TEXTURE.get_hash(), width, height), calc_hash(&"depth texture", 0));
	texture_res_mgr.insert(hash, RenderRes::new(texture_view, (width * height * 3) as usize)).unwrap()
}

// fn calc_texture

fn init_stage(world: &mut World) -> Vec<StageBuilder> {
	// let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
	let mut stages = Vec::new();

	// ????????????????????????
	let mut node_stage = StageBuilder::new();

	// ???????????????
	InitGraphData::setup(world, &mut node_stage);
	CalcPostProcessShader::setup(world, &mut node_stage);
	WithColorShader::setup(world, &mut node_stage);
	TextShader::setup(world, &mut node_stage);
	ColorShader::setup(world, &mut node_stage);
	ImageShader::setup(world, &mut node_stage);
	ColorShadowShader::setup(world, &mut node_stage);

	
	CalcUserSetting::setup(world, &mut node_stage);
	CalcOpacity::setup(world, &mut node_stage);
	CalcContext::setup(world, &mut node_stage);
	CalcZindex::setup(world, &mut node_stage);
	let split_id = CalcTextSplit::setup(world, &mut node_stage).unwrap();
	let glyphid = CalcTextGlyph::setup(world, &mut node_stage).unwrap();
	let layout_id = CalcLayout::setup(world, &mut node_stage).unwrap();
	node_stage = node_stage
		.order(split_id, layout_id)
		.order(layout_id, glyphid);
	CalcQuad::setup(world, &mut node_stage);
	CalcMatrix::setup(world, &mut node_stage);
	CalcContentBox::setup(world, &mut node_stage);
	CalcRoot::setup(world, &mut node_stage);
	CalcBackGroundColor::setup(world, &mut node_stage);
	CalcText::setup(world, &mut node_stage);
	CalcBorderImage::setup(world, &mut node_stage);
	CalcImageLoad::<BorderImage, BorderImageTexture>::setup(world, &mut node_stage);
	CalcBorderColor::setup(world, &mut node_stage);
	CalcBoxShadow::setup(world, &mut node_stage);
	
	CalcBackgroundImage::setup(world, &mut node_stage);
	CalcImageLoad::<BackgroundImage, BackgroundImageTexture>::setup(world, &mut node_stage);

	let mut post_stage = StageBuilder::new();
	CalcOpacityPostProcess::setup(world, &mut post_stage);
	CalcTransformWillChange::setup(world, &mut post_stage);
	CalcOverflow::setup(world, &mut post_stage);

	// ??????????????????
	let mut draw_stage = StageBuilder::new();
	CalcWorldMatrixGroup::setup(world, &mut draw_stage);
	CalcPipeline::setup(world, &mut draw_stage);


	// Pass??????
	let mut pass_stage = StageBuilder::new();
	CalcRender::setup(world, &mut pass_stage);
	CalcDirtyRect::setup(world, &mut pass_stage);

	stages.push(node_stage);
	stages.push(post_stage);
	stages.push(draw_stage);
	stages.push(pass_stage);
	
	stages
}