use pi_world::system::{Commands, SingleRes};
use pi_bevy_ecs_extend::system_param::res::{OrInitSingleRes, OrInitSingleResMut};
use pi_hash::XHashSet;
use pi_render::{
    renderer::{
        draw_obj::DrawBindGroup,
        vertices::{EVerticesBufferUsage, RenderIndices, RenderVertices},
    },
    rhi::texture::PiRenderDefault,
};
use wgpu::IndexFormat;

use crate::{
    components::{
        calc::WorldMatrix,
        draw_obj::{DrawState, PipelineMeta},
        user::Matrix4,
    },
    resource::{
        draw_obj::{
            CameraGroup, ClearDrawObj, DepthCache, DepthGroup, DynFboClearColorBindGroup, PipelineState, PosVertexLayout, ProgramMetaRes,
            ShaderInfoCache, ShareGroupAlloter, UiMaterialGroup, UnitQuadBuffer,
        },
        BackgroundColorRenderObjType,
    },
    shader::{
        camera::{ProjectUniform, ViewUniform},
        color::ProgramMeta,
        ui_meterial::{ColorUniform, WorldUniform},
    },
};

/// 需要为清屏颜色创建DrawObj
/// 创建清屏的drawobj
#[allow(unused_must_use)]
pub fn init(
    mut depth_cache: OrInitSingleResMut<DepthCache>,
    unit_quad_buffer: SingleRes<UnitQuadBuffer>,
    shader_static: OrInitSingleRes<ProgramMetaRes<ProgramMeta>>,
    vert_layout: OrInitSingleRes<PosVertexLayout>,
    // mut state_map: SingleResMut<StateMap>,
    shader_info_cache: OrInitSingleRes<ShaderInfoCache>,

    ui_group_alloter: OrInitSingleRes<ShareGroupAlloter<UiMaterialGroup>>,
    color_render_type: OrInitSingleRes<BackgroundColorRenderObjType>,
    camera_group_alloter: OrInitSingleRes<ShareGroupAlloter<CameraGroup>>,
    depth_alloter: OrInitSingleRes<ShareGroupAlloter<DepthGroup>>,
    mut commands: Commands,
) {
    // let pipeline_state = state_map.insert(pipeline_state);
    // 清屏使用的渲染状态不同
    let pipeline_meta = PipelineMeta {
        type_mark: ***color_render_type,
        program: shader_static.clone(),
        state: shader_info_cache.clear.clone(),
        vert_layout: vert_layout.clone(),
        defines: XHashSet::default(),
    };

    // 设置清屏颜色的vb、ib
    let mut draw_state = DrawState::default();
    draw_state.vertex = 0..4;
    draw_state.insert_vertices(RenderVertices {
        slot: 0,
        buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.vertex.clone()),
        buffer_range: None,
        size_per_value: 8,
    });
    draw_state.indices = Some(RenderIndices {
        buffer: EVerticesBufferUsage::GUI(unit_quad_buffer.index.clone()),
        buffer_range: None,
        format: IndexFormat::Uint16,
    });
    // 暂时在pipeline system中创建pipeline， 考虑ecs新增只运行一次的system，将该逻辑放入这类system中（创建pipeline为异步操作， 当前方法为同步方法，而pipeline system每帧都会运行， 此pipeline最适合放入到一个只运行一次的system中）
    // // 设置清屏颜色的pipeline
    // let (vs_defines, fs_defines) = (VSDefines::default(), FSDefines::default());
    // let pipeline = CalcPipeline::calc_pipeline(
    // 	&vs_defines,
    // 	&fs_defines,
    // 	&static_index,

    // 	&shader_statics,
    // 	&device,
    // 	&vertex_buffer_layout_map,
    // 	&state_map,
    // 	&shader_catch,

    // 	&mut pipeline_map,
    // 	&mut shader_map,
    // 	&share_layout,
    // );
    // // 设置pipeline
    // draw_state.pipeline = Some(pipeline);

    // 设置清屏颜色的世界矩阵

    // 设置清屏颜色的世界矩阵、投影矩阵、视图矩阵
    // 视图矩阵和投影矩阵都设置为单位阵
    let view_project = WorldMatrix::default().0;
    let mut camera_group = camera_group_alloter.alloc();
    camera_group.set_uniform(&ProjectUniform(view_project.as_slice()));
    camera_group.set_uniform(&ViewUniform(view_project.as_slice()));

    draw_state
        .bindgroups
        .insert_group(camera_group_alloter.group_index, DrawBindGroup::Offset(camera_group));

    let mut ui_meterial_group = ui_group_alloter.alloc();
    // 世界矩阵
    let world = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    ui_meterial_group.set_uniform(&WorldUniform(world.as_slice()));
    ui_meterial_group.set_uniform(&ColorUniform(&[0.0, 0.0, 0.0, 0.0]));

    let group = DrawBindGroup::Offset(ui_meterial_group);
    // let state_hash = calc_hash(&pipeline_state, 0);

    // 深度设置为-1(最远)
    depth_cache.or_create_depth(0, &depth_alloter); // 深度为0
    commands.insert_single_res(DynFboClearColorBindGroup(group));
    commands.insert_single_res(ClearDrawObj(draw_state, pipeline_meta.clone()));
}


pub fn create_clear_pipeline_state() -> PipelineState {
    PipelineState {
        targets: vec![Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::pi_render_default(),
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                },
                alpha: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    }
}
