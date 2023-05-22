//! 与DrawObject相关的资源

use std::{borrow::Cow, collections::hash_map::Entry, hash::Hash, marker::PhantomData, num::NonZeroU32};

use bevy::ecs::{
    prelude::{FromWorld, World},
    system::Resource,
};
use ordered_float::NotNan;
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_hash::{XHashMap, XHashSet};
use pi_map::vecmap::VecMap;
use pi_render::{
    renderer::draw_obj::DrawBindGroup,
    rhi::{
        asset::RenderRes,
        bind_group::BindGroup,
        bind_group_layout::BindGroupLayout,
        buffer::Buffer,
        device::RenderDevice,
        dyn_uniform_buffer::GroupAlloter,
        pipeline::RenderPipeline,
        shader::{AsLayoutEntry, BindLayout, BufferSize, ShaderMeta, ShaderProgram},
        texture::PiRenderDefault,
        BufferInitDescriptor,
    },
};
use pi_share::Share;
use pi_slotmap::{DefaultKey, SlotMap};
use wgpu::{
    BindingType, BufferUsages, CompareFunction, DepthBiasState, DepthStencilState, Limits, MultisampleState, PipelineLayout, Sampler, ShaderModule,
    StencilState, TextureFormat,
};

use crate::{
    components::draw_obj::{DrawState, PipelineMeta},
    shader::{camera::CameraBind, depth::DepthBind, ui_meterial::UiMaterialBind},
    system::draw_obj::clear_draw_obj::create_clear_pipeline_state,
    utils::{
        shader_helper::{create_depth_layout, create_empty_layout, create_matrix_group_layout, create_project_layout, create_view_layout},
        tools::{calc_float_hash, calc_hash, calc_hash_slice},
    },
};

/// depth 的BindGroupLayout
#[derive(Deref, Resource)]
pub struct DepthGroupLayout(pub Share<BindGroupLayout>);

/// depth的Group缓冲
#[derive(Deref, Resource)]
pub struct DepthGroup(pub Vec<Share<BindGroup>>);

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
    pub premultiply: Share<PipelineStateWithHash>,
    pub clear: Share<PipelineStateWithHash>,

    pub vert_layout: XHashMap<u64, Share<VertexBufferLayoutWithHash>>,
}

impl Default for ShaderInfoCache {
    fn default() -> Self {
        let clear = create_clear_pipeline_state();
        let common = create_common_pipeline_state();
        let premultiply = create_premultiply_pipeline_state();

        let clear_hash = calc_hash(&clear, 0);
        let common_hash = calc_hash(&common, 0);
        let premultiply_hash = calc_hash(&premultiply, 0);

        let clear = Share::new(PipelineStateWithHash {
            hash: clear_hash,
            state: clear,
        });
        let common = Share::new(PipelineStateWithHash {
            hash: common_hash,
            state: common,
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

        let mut meta = T::create_meta();
        // depth不使用动态偏移
        if let Some(depth_entry) = meta.bindings.bind_group_entrys.get_mut(DepthBind::set() as usize) {
            if depth_entry.len() == 1 {
                if let BindingType::Buffer { has_dynamic_offset, .. } = &mut depth_entry[0].ty {
                    *has_dynamic_offset = false;
                }
            }
        }
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
        let vs_code = self.shader_meta.to_code(defines, wgpu::ShaderStages::VERTEX);
        let fs_code = self.shader_meta.to_code(defines, wgpu::ShaderStages::FRAGMENT);

        // log::warn!("vs_code================{}", vs_code);
        // log::warn!("fs_code================{}", fs_code);
        // std::fs::write("out.vert", &vs_code);
        // std::fs::write("out.frag", &fs_code);

        let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&self.shader_meta.name),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(vs_code.as_str()),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });

        // let fs = processor
        // 		.process(&self.fs_shader_soruce, fs_defines, shaders, &imports)
        // 		.unwrap();
        // let fs = fs.get_glsl_source().unwrap();
        let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&self.shader_meta.name),
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Borrowed(fs_code.as_str()),
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

// #[derive(Deref, DerefMut, Default, Resource)]
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
pub struct DynFboClearColorBindGroup(pub (DrawBindGroup, DrawBindGroup));

pub fn list_share_as_ref<'a, T, I: Iterator<Item = &'a Option<Share<T>>>>(list: I) -> Vec<&'a T> {
    let mut v = Vec::new();
    for r in list {
        if let Some(r) = r {
            v.push(&**r)
        }
    }
    v
}

// #[derive(Deref, DerefMut, Default, Resource)]
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

// #[derive(Deref, DerefMut, Resource)]
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
pub struct GroupAlloterCenter(Vec<Share<GroupAlloter>>, wgpu::Limits);

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

/// buffer累的的binding组的分配器
#[derive(Resource)]
pub struct ShareGroupAlloter<T> {
    pub group_index: u32,
    alloter: Share<GroupAlloter>,
    mark: PhantomData<T>,
}

impl<T> std::ops::Deref for ShareGroupAlloter<T> {
    type Target = Share<GroupAlloter>;

    fn deref(&self) -> &Self::Target { &self.alloter }
}

impl FromWorld for ShareGroupAlloter<CameraGroup> {
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

impl FromWorld for ShareGroupAlloter<UiMaterialGroup> {
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


#[derive(Resource)]
pub struct CommonSampler {
    pub default: Sampler,
    pub pointer: Sampler,
}

impl FromWorld for CommonSampler {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        Self {
            default: (***device).create_sampler(&wgpu::SamplerDescriptor {
                label: Some("default sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            pointer: (***device).create_sampler(&wgpu::SamplerDescriptor {
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

// /// 将pass2d组织为层的结构
// #[derive(Deref, Default, DerefMut)]
// pub struct LayerPass2D (LayerDirty<Entity>);

#[derive(Deref, DerefMut, Resource)]
pub struct TextTextureGroup(pub Handle<RenderRes<BindGroup>>);

#[derive(Deref, DerefMut, Resource)]
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

// /// 常用渲染管线状态
// #[derive(Resource)]
// pub struct CommonPipelineState {
// 	pub common: DefaultKey,
// 	pub premultiply: DefaultKey,
// }


// impl FromWorld for CommonPipelineState {
//     fn from_world(world: &mut World) -> Self {
//         let mut state_map = world.get_resource_mut::<StateMap>().unwrap();

// 		Self {
// 			common: state_map.insert(create_common_pipeline_state()),
// 			premultiply: state_map.insert(create_premultiply_pipeline_state())
// 		}
//     }
// }


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


/// depth BindGroup缓存
#[derive(Resource)]
pub struct DepthCache {
    pub list: Vec<Handle<RenderRes<BindGroup>>>,
    pub layout: Share<BindGroupLayout>,
}

impl FromWorld for DepthCache {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        world.init_resource::<ShaderInfoCache>();
        let world = world.cell();
        let mut cache = world.get_resource_mut::<ShaderInfoCache>().unwrap();
        // bind_group_layout
        let device = world.get_resource::<PiRenderDevice>().unwrap();
        let mut entry = DepthBind::as_layout_entry(wgpu::ShaderStages::VERTEX);
        if let BindingType::Buffer { has_dynamic_offset, .. } = &mut entry.ty {
            *has_dynamic_offset = false;
        }
        let layout = cache.bind_group_layout(&[entry], &device);
        Self { list: Vec::new(), layout }
    }
}

impl DepthCache {
    pub fn or_create_depth<'a>(&mut self, cur_depth: usize, device: &'a RenderDevice, bind_group_assets: &'a Share<AssetMgr<RenderRes<BindGroup>>>) {
        if self.list.len() <= cur_depth {
            let buffer = device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("depth buffer"),
                usage: BufferUsages::UNIFORM,
                contents: bytemuck::cast_slice::<_, u8>(&[0.0, 0.0, 0.0, cur_depth as f32]),
            });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: DepthBind::binding(),
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: std::num::NonZeroU64::new(DepthBind::min_size() as u64),
                    }),
                }],
                label: Some("depth bindgroup"),
            });
            let hash = calc_hash(&("depth", cur_depth), 0);
            let r = bind_group_assets.insert(hash, RenderRes::new(bind_group, DepthBind::min_size()));
            self.list.push(r.unwrap());
        }
    }
}
