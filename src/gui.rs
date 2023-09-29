use std::{
    any::TypeId,
    mem::size_of,
    sync::Arc,
};

use bevy_app::App;
use pi_animation::{animation_group::AnimationGroupID, animation_listener::EAnimationEvent};
use pi_assets::{
    asset::{GarbageEmpty, Handle},
    homogeneous::HomogeneousMgr,
    mgr::AssetMgr,
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_ecs::{
    component::MultiCaseImpl,
    prelude::{ArchetypeId, FromWorld, Id, Join, OrDefault, QueryState, Setup, StageBuilder, World, SingleDispatcher, DispatcherMgr}, resource::ResourceId,
};
use pi_bevy_ecs_extend::prelude::{Layer, Down, Up};
use pi_print_any::out_any;
use pi_render::{
    components::view::target_alloc::{SafeAtlasAllocator, UnuseTexture, DEPTH_TEXTURE},
    font::FontSheet,
    rhi::{
        asset::{RenderRes, TextureRes},
        bind_group::BindGroup,
        buffer::Buffer,
        device::RenderDevice,
        pipeline::RenderPipeline,
        RenderQueue,
    },
};
use pi_share::{cell::TrustCell, Share, ShareCell};
use pi_slotmap::{DefaultKey, Key, SecondaryMap};
use wgpu::TextureView;

use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, InPassId, IsEnable, LayoutResult, NodeId, Quad, WorldMatrix, ZRange, Visibility, Pass2DId},
        pass_2d::{ParentPassId, Pass2D, ScreenTarget, GraphId},
        user::{Aabb2, BackgroundImage, BorderImage, ClassName, Overflow, Point2, serialize::StyleAttr, Viewport},
    },
    resource::{
        draw_obj::{CommonSampler, Program},
        DefaultStyle, NodeCommand, animation_sheet::KeyFramesSheet, UserCommandsCache,
    },
    system::{
        // draw_obj::{pipeline::CalcPipeline, world_marix::CalcWorldMatrixGroup},
        // node::{
        //     animation::CalcAnimation,
        //     background_color::CalcBackGroundColor,
        //     background_image::CalcBackgroundImage,
        //     border_color::CalcBorderColor,
        //     border_image::CalcBorderImage,
        //     box_shadow::CalcBoxShadow,
        //     content_box::CalcContentBox,
        //     context::CalcContext,
        //     context_blur::CalcBlur,
        //     context_hsi::CalcHsi,
        //     context_opacity::{CalcOpacity, CalcOpacityPostProcess},
        //     context_overflow::CalcOverflow,
        //     context_root::CalcRoot,
        //     context_transform_will_change::CalcTransformWillChange,
        //     image_texture_load::CalcImageLoad,
        //     layout::CalcLayout,
        //     quad::CalcQuad,
        //     text::CalcText,
        //     text_glphy::CalcTextGlyph,
        //     text_split::CalcTextSplit,
        //     user_setting::CalcUserSetting,
        //     world_matrix::CalcMatrix,
        //     z_index::CalcZindex, flush::{CalcFlush}, show::CalcShow, canvas::CalcCanvas,
        // },
        // pass::{
        //     pass_dirty_rect::CalcDirtyRect,
        //     pass_graph_node::{InitGraphData, PostBindGroupLayout},
        //     pass_render::CalcRender, pass_render_clear::CalcRenderClear,
        // },
        // shader_utils::{color::CalcColorShader, image::CalcImageShader, text::CalcTextShader},
    },
    utils::{cmd::Command, tools::calc_hash},
};

use pi_style::{
    style_parse::parse_class_map_from_string,
    style_type::{Attr, ClassSheet},
};

use crate::components::user::Node;

pub struct Gui {
    pub app: App,

    // user_commands: UserCommands,

    // node_archetype_id: ArchetypeId,
	// cmd_id: ResourceId,

    // pub down_query: QueryState<Node, &'static Down<Node>>,
    // pub up_query: QueryState<Node, &'static Up<Node>>,
    // pub layer_query: QueryState<Node, &'static Layer<Node>>,
    // pub enable_query: QueryState<Node, &'static IsEnable>,
	// pub visibility_query: QueryState<Node, &'static Visibility>,
    // pub depth_query: QueryState<Node, &'static ZRange>,
    // pub layout_query: QueryState<Node, &'static LayoutResult>,
    // pub quad_query: QueryState<Node, &'static Quad>,
    // pub matrix_query: QueryState<Node, &'static WorldMatrix>,
    // pub overflow_query: QueryState<Pass2D, (OrDefault<ParentPassId>, Join<NodeId, Node, (&'static Quad, OrDefault<Overflow>)>)>,
    // pub in_pass2d_query: QueryState<Node, &'static InPassId>,
	// pub graph_id: QueryState<Node, Join<Pass2DId, Pass2D, &'static GraphId>>,

    // // node_archetype: ArchetypeId,
    // pub quad_component_comtainer: Arc<TrustCell<MultiCaseImpl<Quad>>>,

	// pub layout_dispacher: DefaultKey,
	// pub geo_dispacher: DefaultKey,
	// pub calc_dispacher: DefaultKey,
}

pub struct GuiStages {
	pub node_stage: StageBuilder,
	pub post_stage: StageBuilder,
	pub draw_obj_stage: StageBuilder,
	pub pass_2d_stage: StageBuilder,
	pub clear_stage: StageBuilder,
}

impl Gui {
    pub fn world_mut(&mut self) -> &mut World { &mut self.world }

    pub fn new(world: &mut World) -> Gui {
        world.new_archetype::<Node>().create(); // 创建Node原型
		world.get_or_insert_resource::<UserCommandsCache>();

        // 注册资源管理器
        register_assets_mgr(world);

        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

        // let dispatcher= SingleDispatcher::new(rt);

        let archetype_id = world.archetypes_mut().get_or_create_archetype::<Node>();
		let cmd_id = world.get_resource_id::<UserCommandsCache>().unwrap().clone();
        let quad_id = world.get_or_register_component::<Quad>(archetype_id);
        let c = unsafe { world.archetypes().get(archetype_id).unwrap().get_component(quad_id) };
        let quad = match c.clone().downcast() {
            Ok(r) => {
                let r: Arc<TrustCell<MultiCaseImpl<Quad>>> = r;
                r
            }
            Err(_) => panic!("downcast fail"),
        };
        // let archetype =
        Gui {
            world: world.clone(),
            node_archetype_id,
			cmd_id,
            // user_commands: UserCommands::default(),

            down_query: world.query(),
            up_query: world.query(),
            layer_query: world.query(),
            enable_query: world.query(),
			visibility_query: world.query(),
            depth_query: world.query(),
            layout_query: world.query(),
            quad_query: world.query(),
            matrix_query: world.query(),
            in_pass2d_query: world.query(),
            overflow_query: world.query(),
			graph_id: world.query(),

            quad_component_comtainer: quad,
            layout_dispacher: DefaultKey::null(),
            geo_dispacher: DefaultKey::null(),
            calc_dispacher: DefaultKey::null(),
        }
    }

    /// 初始化gui
    /// 调用此方法必须保证DeviceRender已经在resource上
    pub fn init<RT: AsyncRuntime>(&mut self, x: u32, y: u32, width: u32, height: u32, runtime: RT, dispatcher_mgr: &mut DispatcherMgr) -> GuiStages {
        // 添加必要资源
        insert_resource(&mut self.world, x, y, width, height);

        self.init_stage(runtime, dispatcher_mgr)
    }

    // 创建节点
    pub fn create_node(&mut self) -> Id<Node> {
        // println!("create_node =====");
        let node_archetype_id = self.node_archetype_id;
        let r = unsafe { Id::new(self.world.archetypes_mut()[node_archetype_id].reserve_entity()) };
        r
    }

    /// 将节点作为子节点挂在父上
    pub fn append(&mut self, entity: Id<Node>, parent: Id<Node>) {
        // println!("append node ====={:?}, {:?}", entity, parent);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
    }

    /// 将节点插入到某个节点之前
    pub fn insert_before(&mut self, entity: Id<Node>, anchor: Id<Node>) {
        // println!("insert_before node ====={:?}, {:?}", entity, anchor);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
    }

    /// 从父节点上移除节点
    pub fn remove_node(&mut self, entity: Id<Node>) {
        // println!("remove_node====={:?}", entity.clone());
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
    }

    /// 从父节点上移除节点，并销毁该节点及所有子节点
    pub fn destroy_node(&mut self, entity: Id<Node>) {
        // println_any!("destroy_node===={:?}", &entity);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
    }

	/// 取到图节点id（只有根节点的图节点id能取到）
	pub fn get_graph_node_id(&mut self, entity: Id<Node>) -> Option<GraphNodeId> {
		self.graph_id.get(&self.world, entity).map(|r| {r.0.clone()})
	}

    /// 设置节点样式
    pub fn set_style<T: Attr>(&mut self, entity: Id<Node>, value: T) {
		// out_any!(log::info, "set_style, entity: {:?}, value: {:?}", entity, &value);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_mut::<UserCommandsCache>(cmd_id)}.unwrap().0;
		// let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        // out_any!(trace, "set_style, entity: {:?}, value: {:?}", entity, &value);
        let start = user_commands.style_commands.style_buffer.len();
        unsafe { StyleAttr::write(value, &mut user_commands.style_commands.style_buffer) };
        if let Some(r) = user_commands.style_commands.commands.last_mut() {
            if r.0 == entity {
                r.2 = user_commands.style_commands.style_buffer.len();
                return;
            }
        }
        user_commands
            .style_commands
            .commands
            .push((entity, start, user_commands.style_commands.style_buffer.len()));
    }

    /// 设置默认样式（二进制样式）
    pub fn set_default_style_by_bin(&mut self, bin: &[u8]) {
        // println_any!("set_default_style_by_bin===={:?}", 1);
        let class_sheet_new: ClassSheet = match postcard::from_bytes(bin) {
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

    /// 设置默认样式（字符串）TODO
    pub fn set_default_style_by_str(&mut self, class: &str, scope_hash: usize) {
        // println_any!("set_default_style_by_str===={:?}", class);
        let class_sheet = self.world.get_resource_mut::<ClassSheet>();
        let class_sheet = match class_sheet {
            Some(r) => r,
            None => {
                self.world.insert_resource(ClassSheet::default());
                self.world.get_resource_mut::<ClassSheet>().unwrap()
            }
        };

		let mut c = class;
		let class_temp;
		if !class.starts_with(".c0") {
			class_temp = ".c0{".to_string() + class + "}";
			c = class_temp.as_str();
		}
        match parse_class_map_from_string(c, scope_hash) {
            Ok(r) => {
                r.to_class_sheet(class_sheet);
                self.world.insert_resource(DefaultStyle);
            } // 触发DefaultStyle修改
            Err(e) => {
                log::error!("set_default_style_by_str fail, parse style err: {:?}", e);
                return;
            }
        };
    }

    /// 设置节点的class
    pub fn set_class(&mut self, entity: Id<Node>, value: ClassName) {
        // println_any!("set_class===={:?}", &value);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.class_commands.push((entity, value));
    }

    /// 添加指令
    pub fn push_cmd<T: Command>(&mut self, cmd: T) {
        // println_any!("push_cmd===={:?}", 1);
		let cmd_id = self.cmd_id;
		let user_commands = &mut unsafe { self.world.archetypes_mut().get_resource_unchecked_mut::<UserCommandsCache>(cmd_id)}.0;
        user_commands.other_commands.push(cmd);
    }

    /// 推动gui运行
    pub fn run(&mut self) {
        // log::info!("run user_commands ===={:?}", self.user_commands.style_commands.commands.len());
        // let node_archetype_id = self.node_archetype_id;
        // self.world.archetypes_mut()[node_archetype_id].flush();
        // let commands = replace(&mut self.user_commands, UserCommands::default());
        // self.world.insert_resource(commands);
        // println_any!("run===={:?}", 2);
    }

	/// 计算布局
	pub async fn calc_layout(&mut self, dispatcher_mgr: &DispatcherMgr, is_wait: bool) {
		// self.run();
		dispatcher_mgr.run(self.layout_dispacher,is_wait).await
	}

	/// 计算布局+世界矩阵+包围盒
	pub async fn calc_geo(&mut self, dispatcher_mgr: &DispatcherMgr, is_wait: bool) {
		// self.run();
		dispatcher_mgr.run(self.geo_dispacher, is_wait).await
	}

	/// 计算布局+世界矩阵+包围盒+文字
	pub async fn calc(&mut self, dispatcher_mgr: &DispatcherMgr, is_wait: bool) {
		// self.run();
		dispatcher_mgr.run(self.geo_dispacher, is_wait).await
	}

	/// 设置事件监听器
	pub fn set_event_listener(&mut self, callback: Box<dyn Fn(&Vec<(AnimationGroupID, EAnimationEvent, u32)>, &SecondaryMap<AnimationGroupID, (Id<Node>, Atom)>)>) {
		self.world.get_or_insert_resource_mut::<KeyFramesSheet>().set_event_listener(callback);
	}

	pub fn init_stage<RT: AsyncRuntime>(&mut self, runtime: RT, dispatcher_mgr: &mut DispatcherMgr) -> GuiStages {
		let world = &mut self.world;
		// 布局stage，仅用于布局
		let mut layout_stage = StageBuilder::new();
		// geo stage，用于布局+矩阵+包围盒
		let mut geo_stage = StageBuilder::new();
		// calc stage，用于布局+矩阵+包围盒+文字（文字第一帧问题）
		let mut calc_stage = StageBuilder::new();
	
		// let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
		// let mut stages = Vec::new();
	
		// 节点属性计算阶段
		let mut node_stage = StageBuilder::new();

		return GuiStages {
			node_stage,
			draw_obj_stage: StageBuilder::new(),
			pass_2d_stage: StageBuilder::new(),
			post_stage:StageBuilder::new(),
			clear_stage: StageBuilder::new(),

		};
	
		// 初始化数据
		InitGraphData::setup(world, &mut node_stage);
		CalcImageShader::setup(world, &mut node_stage);
		// WithColorShader::setup(world, &mut node_stage);
		CalcTextShader::setup(world, &mut node_stage);
		CalcColorShader::setup(world, &mut node_stage);
		// ImageShader::setup(world, &mut node_stage);
		// ColorShadowShader::setup(world, &mut node_stage);
		
		CalcFlush::setup(world, &mut node_stage).unwrap();
		let flush_system = node_stage.get_last_node().unwrap();
		layout_stage.add_node(flush_system.clone());
		geo_stage.add_node(flush_system.clone());
		calc_stage.add_node(flush_system.clone());
	
		let user_shetting_id = CalcUserSetting::setup(world, &mut node_stage).unwrap();
		let user_setting_system = node_stage.get_last_node().unwrap();
		layout_stage.add_node(user_setting_system.clone());
		geo_stage.add_node(user_setting_system.clone());
		calc_stage.add_node(user_setting_system.clone());
	
		let animation_id = CalcAnimation::setup(world, &mut node_stage).unwrap();
		node_stage = node_stage.order(user_shetting_id, animation_id);
	
		
		CalcShow::setup(world, &mut node_stage).unwrap();
		CalcOpacity::setup(world, &mut node_stage);
		CalcContext::setup(world, &mut node_stage);
		CalcZindex::setup(world, &mut node_stage);
		let split_id = CalcTextSplit::setup(world, &mut node_stage).unwrap();
		let split_system =  node_stage.get_last_node().unwrap();
		layout_stage.add_node(split_system.clone());
		geo_stage.add_node(split_system.clone());
		calc_stage.add_node(split_system.clone());
	
		let layout_id = CalcLayout::setup(world, &mut node_stage).unwrap();
		let layout_system = node_stage.get_last_node().unwrap();
		layout_stage.add_node(layout_system.clone());
		geo_stage.add_node(layout_system.clone());
		calc_stage.add_node(layout_system.clone());
	
		let glyphid = CalcTextGlyph::setup(world, &mut node_stage).unwrap();
		let glyph_system = node_stage.get_last_node().unwrap();
		calc_stage.add_node(glyph_system.clone());
	
		node_stage = node_stage.order(split_id, layout_id).order(layout_id, glyphid);
		layout_stage = layout_stage.order(split_id, layout_id);
		geo_stage = geo_stage.order(split_id, layout_id);
		calc_stage = calc_stage.order(split_id, layout_id).order(layout_id, glyphid);
	
		CalcMatrix::setup(world, &mut node_stage);
		let matrix_system = node_stage.get_last_node().unwrap();
		geo_stage.add_node(matrix_system.clone());
		calc_stage.add_node(matrix_system.clone());
	
		CalcQuad::setup(world, &mut node_stage);
		let quad_system = node_stage.get_last_node().unwrap();
		geo_stage.add_node(quad_system.clone());
		calc_stage.add_node(quad_system.clone());
	
		CalcContentBox::setup(world, &mut node_stage);
		CalcRoot::setup(world, &mut node_stage);
		CalcBackGroundColor::setup(world, &mut node_stage);
		CalcCanvas::setup(world, &mut node_stage);
	
		CalcText::setup(world, &mut node_stage);
		let text_system = node_stage.get_last_node().unwrap();
		calc_stage.add_node(text_system.clone());
	
		CalcBorderImage::setup(world, &mut node_stage);
		CalcImageLoad::<BorderImage, BorderImageTexture>::setup(world, &mut node_stage);
		CalcBorderColor::setup(world, &mut node_stage);
		CalcBoxShadow::setup(world, &mut node_stage);
	
		CalcBackgroundImage::setup(world, &mut node_stage);
		CalcImageLoad::<BackgroundImage, BackgroundImageTexture>::setup(world, &mut node_stage);
	
		let mut post_stage = StageBuilder::new();
		CalcOpacityPostProcess::setup(world, &mut post_stage);
		CalcHsi::setup(world, &mut post_stage);
		CalcBlur::setup(world, &mut post_stage);
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

		// Pass计算
		let mut clear_stage = StageBuilder::new();
		// CalcRenderClear::setup(world, &mut clear_stage);

		let mut layout_dispacher = SingleDispatcher::new(runtime.clone());
		let mut geo_dispacher = SingleDispatcher::new(runtime.clone());
		let mut calc_dispacher = SingleDispatcher::new(runtime.clone());
		layout_dispacher.init(vec![Share::new(layout_stage.build(world))], world);
		geo_dispacher.init(vec![Share::new(geo_stage.build(world))], world);
		calc_dispacher.init(vec![Share::new(calc_stage.build(world))], world);

		self.layout_dispacher = dispatcher_mgr.insert(Box::new(layout_dispacher));
		self.geo_dispacher = dispatcher_mgr.insert(Box::new(geo_dispacher));
		self.calc_dispacher = dispatcher_mgr.insert(Box::new(calc_dispacher));
	
		GuiStages {
			node_stage,
			draw_obj_stage: draw_stage,
			pass_2d_stage: pass_stage,
			post_stage,
			clear_stage,

		}
	}
}

fn register_assets_mgr(world: &mut World) {
    world.insert_resource(AssetMgr::<RenderRes<Buffer>>::new(GarbageEmpty(), false, 20 * 1024 * 1024, 3 * 60 * 1000));
    world.insert_resource(AssetMgr::<RenderRes<BindGroup>>::new(GarbageEmpty(), false, 5 * 1024, 3 * 60 * 1000));
    world.insert_resource(AssetMgr::<RenderRes<TextureView>>::new(
        GarbageEmpty(),
        false,
        60 * 1024 * 1024,
        3 * 60 * 1000,
    ));
    world.insert_resource(AssetMgr::<TextureRes>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 3 * 60 * 1000));
    world.insert_resource(AssetMgr::<RenderRes<RenderPipeline>>::new(
        GarbageEmpty(),
        false,
        60 * 1024 * 1024,
        3 * 60 * 1000,
    ));
    world.insert_resource(AssetMgr::<RenderRes<Program>>::new(
        GarbageEmpty(),
        false,
        60 * 1024 * 1024,
        3 * 60 * 1000,
    ));
    world.insert_resource(HomogeneousMgr::<RenderRes<UnuseTexture>>::new(
        pi_assets::homogeneous::GarbageEmpty(),
        10 * size_of::<UnuseTexture>(),
        size_of::<UnuseTexture>(),
        3 * 60 * 1000,
    ));
}

// 插入必须的资源
fn insert_resource(world: &mut World, x: u32, y: u32, width: u32, height: u32) {
    let texture_res_mgr = world.get_resource::<Share<AssetMgr<RenderRes<TextureView>>>>().unwrap().clone();
    let texture_res_mgr1 = world.get_resource::<Share<AssetMgr<TextureRes>>>().unwrap().clone();
    let unuse_texture_res_mgr = world.get_resource::<Share<HomogeneousMgr<RenderRes<UnuseTexture>>>>().unwrap().clone();
    let device = world.get_resource::<RenderDevice>().unwrap().clone();
    let queue = world.get_resource::<RenderQueue>().unwrap().clone();
    let limit = device.limits();
    world.insert_resource(limit);

    let view_port = Viewport(Aabb2::new(
        Point2::new(x as f32, y as f32),
        Point2::new((x + width) as f32, (y + height) as f32),
    ));

    // 设置gui默认渲染到屏幕
    let depth_buffer = create_depth_buffer(&texture_res_mgr, &device, width, height);
    world.insert_resource(ScreenTarget {
        aabb: view_port.0.clone(),
        depth: Some(depth_buffer), // 深度缓冲区
                                   // depth: None,
    });

    let allocator = SafeAtlasAllocator::new(device.clone(), texture_res_mgr.clone(), unuse_texture_res_mgr);

    // 添加纹理分配器
    world.insert_resource(allocator);

    // // 插入视口
    // world.insert_resource(view_port);

    // 插入PostBindGroupLayout
    let post_layout = PostBindGroupLayout::from_world(world);
    world.insert_resource(post_layout);

    // 插入CommonSampler
    let common_sampler = CommonSampler::from_world(world);
    world.insert_resource(common_sampler);

    // 插入FontSheet
    world.insert_resource(Share::new(ShareCell::new(FontSheet::new(&device, &texture_res_mgr1, &queue))));
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

    let hash = calc_hash(&(DEPTH_TEXTURE.get_hash(), width, height), calc_hash(&"depth texture", 0));
    texture_res_mgr
        .insert(hash, RenderRes::new(texture_view, (width * height * 3) as usize))
        .unwrap()
}