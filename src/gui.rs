use std::{
    any::TypeId,
    mem::{replace, size_of},
    sync::Arc,
};

use log::trace;
use pi_assets::{
    asset::{GarbageEmpty, Handle},
    homogeneous::HomogeneousMgr,
    mgr::AssetMgr,
};
use pi_ecs::{
    component::MultiCaseImpl,
    prelude::{ArchetypeId, FromWorld, Id, Join, OrDefault, QueryState, Setup, StageBuilder, World},
};
use pi_ecs_utils::prelude::{Layer, NodeDown, NodeUp};
use pi_print_any::{out_any, println_any};
use pi_render::{
    components::view::target_alloc::{SafeAtlasAllocator, ShareTargetView, UnuseTexture, DEPTH_TEXTURE},
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
use wgpu::TextureView;

use crate::{
    components::{
        calc::{BackgroundImageTexture, BorderImageTexture, InPassId, IsEnable, LayoutResult, NodeId, Quad, WorldMatrix, ZRange},
        pass_2d::{ParentPassId, Pass2D, RenderTarget, ScreenTarget},
        user::{Aabb2, BackgroundImage, BorderImage, ClassName, Overflow, Point2},
    },
    resource::{
        draw_obj::{CommonSampler, Program},
        DefaultStyle, NodeCommand, UserCommands, Viewport,
    },
    system::{
        draw_obj::{pipeline::CalcPipeline, world_marix::CalcWorldMatrixGroup},
        node::{
            animation::CalcAnimation,
            background_color::CalcBackGroundColor,
            background_image::CalcBackgroundImage,
            border_color::CalcBorderColor,
            border_image::CalcBorderImage,
            box_shadow::CalcBoxShadow,
            content_box::CalcContentBox,
            context::CalcContext,
            context_blur::CalcBlur,
            context_hsi::CalcHsi,
            context_opacity::{CalcOpacity, CalcOpacityPostProcess},
            context_overflow::CalcOverflow,
            context_root::CalcRoot,
            context_transform_will_change::CalcTransformWillChange,
            image_texture_load::CalcImageLoad,
            layout::CalcLayout,
            quad::CalcQuad,
            text::CalcText,
            text_glphy::CalcTextGlyph,
            text_split::CalcTextSplit,
            user_setting::CalcUserSetting,
            world_matrix::CalcMatrix,
            z_index::CalcZindex,
        },
        pass::{
            pass_dirty_rect::CalcDirtyRect,
            pass_graph_node::{InitGraphData, PostBindGroupLayout},
            pass_render::CalcRender,
        },
        shader_utils::{color::CalcColorShader, image::CalcImageShader, text::CalcTextShader},
    },
    utils::{cmd::Command, tools::calc_hash},
};

use pi_style::{
    style_parse::parse_class_map_from_string,
    style_type::{Attr, ClassSheet, StyleAttr},
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
    pub fn world_mut(&mut self) -> &mut World { &mut self.world }

    pub fn new(world: &mut World) -> Gui {
        world.new_archetype::<Node>().create(); // 创建Node原型

        // 注册资源管理器
        register_assets_mgr(world);

        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<Node>()).unwrap().clone();

        // let dispatcher= SingleDispatcher::new(rt);

        let archetype_id = world.archetypes_mut().get_or_create_archetype::<Node>();
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

    /// 初始化gui
    /// 调用此方法必须保证DeviceRender已经在resource上
    pub fn init(&mut self, x: u32, y: u32, width: u32, height: u32) -> Vec<StageBuilder> {
        // 添加必要资源
        insert_resource(&mut self.world, x, y, width, height);

        init_stage(&mut self.world)
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
        self.user_commands.node_commands.push(NodeCommand::AppendNode(entity, parent));
    }

    /// 将节点插入到某个节点之前
    pub fn insert_before(&mut self, entity: Id<Node>, anchor: Id<Node>) {
        // println!("insert_before node ====={:?}, {:?}", entity, anchor);
        self.user_commands.node_commands.push(NodeCommand::InsertBefore(entity, anchor));
    }

    /// 从父节点上移除节点
    pub fn remove_node(&mut self, entity: Id<Node>) {
        // println!("remove_node====={:?}", entity.clone());
        self.user_commands.node_commands.push(NodeCommand::RemoveNode(entity));
    }

    /// 从父节点上移除节点，并销毁该节点及所有子节点
    pub fn destroy_node(&mut self, entity: Id<Node>) {
        // println_any!("destroy_node===={:?}", &entity);
        self.user_commands.node_commands.push(NodeCommand::DestroyNode(entity));
    }

    /// 设置节点样式
    pub fn set_style<T: Attr>(&mut self, entity: Id<Node>, value: T) {
        out_any!(trace, "set_style, entity: {:?}, value: {:?}", entity, &value);
        let start = self.user_commands.style_commands.style_buffer.len();
        unsafe { StyleAttr::write(value, &mut self.user_commands.style_commands.style_buffer) };
        if let Some(r) = self.user_commands.style_commands.commands.last_mut() {
            if r.0 == entity {
                r.2 = self.user_commands.style_commands.style_buffer.len();
                return;
            }
        }
        self.user_commands
            .style_commands
            .commands
            .push((entity, start, self.user_commands.style_commands.style_buffer.len()));
    }

    /// 设置默认样式（二进制样式）
    pub fn set_default_style_by_bin(&mut self, bin: &[u8]) {
        // println_any!("set_default_style_by_bin===={:?}", 1);
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

    /// 设置默认样式（字符串）
    pub fn set_default_style_by_str(&mut self, class: &str) {
        // println_any!("set_default_style_by_str===={:?}", class);
        let class_sheet = self.world.get_resource_mut::<ClassSheet>();
        let class_sheet = match class_sheet {
            Some(r) => r,
            None => {
                self.world.insert_resource(ClassSheet::default());
                self.world.get_resource_mut::<ClassSheet>().unwrap()
            }
        };
        match parse_class_map_from_string(class) {
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
        self.user_commands.class_commands.push((entity, value));
    }

    /// 添加指令
    pub fn push_cmd<T: Command>(&mut self, cmd: T) {
        // println_any!("push_cmd===={:?}", 1);
        self.user_commands.other_commands.push(cmd);
    }

    /// add css
    pub fn extend_css(&mut self, cmd: ClassSheet) {
        // println_any!("push_cmd===={:?}", 1);
        self.user_commands.css_commands.push(cmd);
    }

    /// 推动gui运行
    pub fn run(&mut self) {
        // println_any!("run===={:?}", 1);
        let node_archetype_id = self.node_archetype_id;
        self.world.archetypes_mut()[node_archetype_id].flush();
        let commands = replace(&mut self.user_commands, UserCommands::default());
        self.world.insert_resource(commands);
        // println_any!("run===={:?}", 2);
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
    let dyn_target_type = InitGraphData::create_dyn_target_type(&allocator, &view_port);

    // 需要单独的一个target类型
    let last_target = allocator.allocate::<&ShareTargetView, _>(
        (view_port.maxs.x - view_port.mins.x).ceil() as u32,
        (view_port.maxs.y - view_port.mins.y).ceil() as u32,
        dyn_target_type.has_depth,
        [].iter(),
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

// fn calc_texture

fn init_stage(world: &mut World) -> Vec<StageBuilder> {
    // let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
    let mut stages = Vec::new();

    // 节点属性计算阶段
    let mut node_stage = StageBuilder::new();

    // 初始化数据
    InitGraphData::setup(world, &mut node_stage);
    CalcImageShader::setup(world, &mut node_stage);
    // WithColorShader::setup(world, &mut node_stage);
    CalcTextShader::setup(world, &mut node_stage);
    CalcColorShader::setup(world, &mut node_stage);
    // ImageShader::setup(world, &mut node_stage);
    // ColorShadowShader::setup(world, &mut node_stage);


    let user_shetting_id = CalcUserSetting::setup(world, &mut node_stage).unwrap();
    let animation_id = CalcAnimation::setup(world, &mut node_stage).unwrap();
    node_stage = node_stage.order(user_shetting_id, animation_id);

    CalcOpacity::setup(world, &mut node_stage);
    CalcContext::setup(world, &mut node_stage);
    CalcZindex::setup(world, &mut node_stage);
    let split_id = CalcTextSplit::setup(world, &mut node_stage).unwrap();
    let glyphid = CalcTextGlyph::setup(world, &mut node_stage).unwrap();
    let layout_id = CalcLayout::setup(world, &mut node_stage).unwrap();
    node_stage = node_stage.order(split_id, layout_id).order(layout_id, glyphid);
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

    stages.push(node_stage);
    stages.push(post_stage);
    stages.push(draw_stage);
    stages.push(pass_stage);

    stages
}
