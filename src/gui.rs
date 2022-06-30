use std::{any::TypeId, mem::replace};

use pi_assets::{mgr::AssetMgr, asset::{GarbageEmpty, Handle}};
use pi_ecs::prelude::{World, ArchetypeId, StageBuilder, Id, Setup, FromWorld};
use pi_render::{
	rhi::{
		asset::{RenderRes, TextureRes},
		buffer::Buffer, 
		bind_group::BindGroup, 
		device::RenderDevice, RenderQueue
	}, 
	components::view::target_alloc::{SafeAtlasAllocator, DEPTH_TEXTURE, ShareTargetView},
	font::FontSheet,
};
use pi_share::{Share, ShareCell};
use wgpu::TextureView;

use crate::{
	components::{
		user::{ClassName, Aabb2, Point2}, 
		pass_2d::{RenderTarget, ScreenTarget}
	}, 
	resource::{UserCommands, NodeCommand, Viewport, draw_obj::CommonSampler}, 
	utils::{style::style_sheet::{StyleAttr, Attr}, tools::calc_hash}, 
	system::{
		node::{user_setting::CalcUserSetting, context::CalcContext, z_index::CalcZindex, layout::CalcLayout, quad::CalcQuad, world_matrix::CalcMatrix, content_box::CalcContentBox, background_color::CalcBackGroundColor, context_opacity::{CalcOpacity, CalcOpacityPostProcess}, context_transform_will_change::CalcTransformWillChange, context_overflow::CalcOverflow, context_root::CalcRoot, text::CalcText, text_split::CalcTextSplit, text_glphy::CalcTextGlyph, border_image::{CalcBorderImage, CalcBorderImageLoad}, border_color::CalcBorderColor, box_shadow::CalcBoxShadow}, 
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
	world: World,

	user_commands: UserCommands,

	node_archetype_id: ArchetypeId,
}

impl Gui {
	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn new(world: &mut World) -> Gui {
		world.new_archetype::<Node>().create(); // 创建Node原型

		// 注册资源管理器
		register_assets_mgr(world);

		let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

		// let dispatcher= SingleDispatcher::new(rt);

		Gui {
			world: world.clone(),
			node_archetype_id,
			user_commands: UserCommands::default(),
			// dispatcher,
		}
	}

	/// 初始化gui
	/// 调用此方法必须保证DeviceRender已经在resource上
	pub fn init(
		&mut self, 
		x: u32, 
		y: u32, 
		width: u32, 
		height: u32,
	) -> Vec<StageBuilder> {
		// 添加必要资源
		insert_resource(&mut self.world, x, y, width, height);

		init_stage(&mut self.world)
	}

	// 创建节点
	pub fn create_node(&mut self) -> Id<Node> {
		let node_archetype_id = self.node_archetype_id;
		unsafe { Id::new(self.world.archetypes_mut()[node_archetype_id].reserve_entity()) }
	}

	/// 将节点作为子节点挂在父上
	pub fn append(&mut self, entity: Id<Node>, parent: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
	}

	/// 将节点插入到某个节点之前
	pub fn insert_before(&mut self, entity: Id<Node>, anchor: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
	}

	/// 从父节点上移除节点
	pub fn remove_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
	}

	/// 从父节点上移除节点，并销毁该节点及所有子节点
	pub fn destroy_node(&mut self, entity: Id<Node>) {
		self.user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
	}

	/// 设置节点样式
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

	/// 设置节点的class
	pub fn set_class(&mut self, entity: Id<Node>, value: ClassName){
		self.user_commands.class_commands.push((entity, value));
	}

	/// 推动gui运行
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
}

// 插入必须的资源
fn insert_resource(
	world: &mut World,
	x: u32, 
	y: u32, 
	width: u32, 
	height: u32,
) {
	let texture_res_mgr = world.get_resource::<Share<AssetMgr<RenderRes<TextureView>>>>().unwrap().clone();
	let device = world.get_resource::<RenderDevice>().unwrap().clone();
	let queue = world.get_resource::<RenderQueue>().unwrap().clone();

	let view_port = Viewport(Aabb2::new(Point2::new(x as f32, y as f32), Point2::new((x + width) as f32, (y + height) as f32)));
	
	// 设置gui默认渲染到屏幕
	let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, width, height);
	world.insert_resource(ScreenTarget {
		aabb: view_port.0.clone(),
		depth: Some(depth_buffer), // 深度缓冲区
		// depth: None,
	});

	let allocator = SafeAtlasAllocator::new(device.clone(), texture_res_mgr.clone());
	let dyn_target_type = InitGraphData::create_dyn_target_type(&allocator, &view_port);

	// 需要单独的一个target类型
	let last_target = allocator.allocate::<&ShareTargetView, _>(
		(view_port.maxs.x - view_port.mins.x).ceil() as u32,
		(view_port.maxs.y - view_port.mins.y).ceil() as u32,
		dyn_target_type.has_depth,
		[].iter()
	);
	// 添加最终渲染目标
	world.insert_resource(RenderTarget::OffScreen(last_target));
	
	// 添加纹理分配器
	world.insert_resource(allocator);

	// 动态纹理类型
	world.insert_resource(dyn_target_type);

	// 插入视口
	world.insert_resource(view_port);

	// 插入PostBindGroupLayout
	let post_layout = PostBindGroupLayout::from_world(world);
	world.insert_resource(post_layout);

	// 插入CommonSampler
	let common_sampler = CommonSampler::from_world(world);
	world.insert_resource(common_sampler);
	
	// 插入FontSheet
	world.insert_resource(Share::new(ShareCell::new(FontSheet::new(&device,&texture_res_mgr, &queue))));
	
}

// 创建深度缓冲区
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

	let hash = calc_hash(&(DEPTH_TEXTURE.get_hash(), width, height));
	texture_res_mgr.cache(hash, RenderRes::new(texture_view, (width * height * 3) as usize));
	texture_res_mgr.get(&hash).unwrap()
}

// fn calc_texture

fn init_stage(world: &mut World) -> Vec<StageBuilder> {
	// let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
	let mut stages = Vec::new();

	// 节点属性计算阶段
	let mut node_stage = StageBuilder::new();

	// 初始化数据
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
	CalcBorderImageLoad::setup(world, &mut node_stage);
	CalcBorderColor::setup(world, &mut node_stage);
	CalcBoxShadow::setup(world, &mut node_stage);
	
	let mut post_stage = StageBuilder::new();
	CalcOpacityPostProcess::setup(world, &mut post_stage);
	CalcTransformWillChange::setup(world, &mut post_stage);
	CalcOverflow::setup(world, &mut post_stage);

	// 渲染对象计算
	let mut draw_stage = StageBuilder::new();
	CalcWorldMatrixGroup::setup(world, &mut draw_stage);
	CalcPipeline::setup(world, &mut draw_stage);


	// Pass计算
	let mut pass_stage = StageBuilder::new();
	CalcRender::setup(world, &mut pass_stage);
	CalcDirtyRect::setup(world, &mut pass_stage);

	stages.push(node_stage);
	stages.push(post_stage);
	stages.push(draw_stage);
	stages.push(pass_stage);
	
	stages
}