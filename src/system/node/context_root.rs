//! 处理root节点，将root节点标记为渲染上下文（设置RenderContextMark中的位标记）

use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{res::WriteRes, Res, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{asset::RenderRes, bind_group::BindGroup, device::RenderDevice, dyn_uniform_buffer::Group, pipeline::RenderPipeline};
use pi_share::Share;
use smallvec::smallvec;
use wgpu::IndexFormat;

use crate::{
    components::{
        calc::WorldMatrix,
        draw_obj::{DrawGroup, DrawState, DynDrawGroup, FSDefines, VSDefines},
        pass_2d::RenderTarget,
        user::Matrix4,
    },
    resource::draw_obj::{
        CommonPipelineState, CommonSampler, CopyFboToScreen, DynBindGroupIndex, DynUniformBuffer, ImageStaticIndex, Program, ShaderCatch, Shaders,
        StateMap, UnitQuadBuffer, VertexBufferLayoutMap,
    },
    shaders::{
        color::CameraMatrixGroup,
        image::{CameraMatrixBind, DepthUniform, ImageMaterialBind, ImageMaterialGroup, ProjectUniform, SampTex2DGroup, ViewUniform, WorldUniform},
    },
    system::{draw_obj::pipeline::CalcPipeline, pass::pass_graph_node::PostBindGroupLayout},
    utils::tools::calc_hash,
};


pub struct CalcRoot;

#[setup]
impl CalcRoot {
    #[system]
    pub async fn render_change<'a>(
        pipeline_map: Res<'static, Share<AssetMgr<RenderRes<RenderPipeline>>>>,
        shader_map: Res<'static, Share<AssetMgr<RenderRes<Program>>>>,

        unit_quad_buffer: Res<'a, UnitQuadBuffer>,
        image_static_index: Res<'a, ImageStaticIndex>,
        shader_statics: Res<'a, Shaders>,
        device: Res<'a, RenderDevice>,
        shader_catch: Res<'a, ShaderCatch>,
        vertex_buffer_layout_map: Res<'a, VertexBufferLayoutMap>,
        state_map: Res<'a, StateMap>,
        bind_group_assets: Res<'a, Share<AssetMgr<RenderRes<BindGroup>>>>,

        mut copy_draw_obj: WriteRes<'a, CopyFboToScreen>,
        post_bind_group_layout: Res<'a, PostBindGroupLayout>,
        common_sampler: Res<'a, CommonSampler>,

        render_target: Res<'a, RenderTarget>,
        camera_bind_group: Res<'a, DynBindGroupIndex<CameraMatrixGroup>>,
        post_bind_group: Res<'a, DynBindGroupIndex<ImageMaterialGroup>>,
        common_state: Res<'a, CommonPipelineState>,

        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
    ) -> std::io::Result<()> {
        if !render_target.is_changed() {
            return Ok(());
        }

        let target = if let RenderTarget::OffScreen(target) = &*render_target {
            target
        } else {
            return Ok(());
        };

        // 如果渲染目标不是一个离屏Target，则需要创建一个离屏fbo， 将gui渲染到离屏fbo上，再将fbo渲染到最终目标上
        // 原因是，gui的渲染机制为局部脏更机制，需要保留上一帧的画面，如果不用离屏fbo，在多缓冲模式下，不能保留原有画面
        // 此逻辑创建一个drawobj，用于将离屏的fbo渲染到最终目标上
        let mut draw_state = DrawState::default();
        draw_state.vbs.insert(0, (unit_quad_buffer.vertex.clone(), 0));
        draw_state.vbs.insert(1, (unit_quad_buffer.uv.clone(), 0));
        draw_state.ib = Some((unit_quad_buffer.index.clone(), 6, IndexFormat::Uint16));

        let mut image_static_index = image_static_index.clone();
        image_static_index.pipeline_state = common_state.premultiply;

        let pipeline = match CalcPipeline::calc_pipeline(
            &VSDefines::default(),
            &FSDefines::default(),
            &image_static_index,
            &shader_statics,
            &device,
            &vertex_buffer_layout_map,
            &state_map,
            &shader_catch,
            &pipeline_map,
            &shader_map,
        )
        .await
        {
            Ok(r) => r,
            Err(e) => panic!("create CopyFboToScreen pipeline fail, {:?}", e),
        };
        draw_state.pipeline = Some(pipeline);

        let camera_dyn_offset = dyn_uniform_buffer.alloc_binding::<CameraMatrixBind>();
        let camera_matrix = WorldMatrix::default();
        dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ProjectUniform(camera_matrix.as_slice()));
        dyn_uniform_buffer.set_uniform(&camera_dyn_offset, &ViewUniform(camera_matrix.as_slice()));
        draw_state.bind_groups.insert_group(
            CameraMatrixGroup::id(),
            DrawGroup::Dyn(DynDrawGroup::new(**camera_bind_group, smallvec![camera_dyn_offset])),
        );

        // 世界矩阵
        let world_matrix = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let post_dyn_offset = dyn_uniform_buffer.alloc_binding::<ImageMaterialBind>();
        dyn_uniform_buffer.set_uniform(&post_dyn_offset, &WorldUniform(world_matrix.as_slice()));
        dyn_uniform_buffer.set_uniform(&post_dyn_offset, &DepthUniform(&[0.0]));
        draw_state.bind_groups.insert_group(
            ImageMaterialGroup::id(),
            DrawGroup::Dyn(DynDrawGroup::new(**post_bind_group, smallvec![post_dyn_offset])),
        );

        let group_key = calc_hash(&("bind", target.target().colors[0].0.key()), 0);
        let texture_bind = match bind_group_assets.get(&group_key) {
            Some(r) => r,
            None => {
                let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &post_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&target.target().colors[0].0),
                        },
                    ],
                    label: Some("post process texture bind group create"),
                });
                bind_group_assets.insert(group_key, RenderRes::new(group, 5)).unwrap()
            }
        };
        draw_state.bind_groups.insert_group(SampTex2DGroup::id(), DrawGroup::Static(texture_bind));

        copy_draw_obj.write(CopyFboToScreen(draw_state));

        Ok(())
    }

    // #[listen(component=(Node, Root, (Create, Delete)))]
    // pub fn root_change(
    // 	e: Event,
    // 	root: Query<Node, &Root>,
    // 	render_mark: Query<Node, Write<RenderContextMark>>,
    // 	local: Local<RenderContextMarkType>,
    // ) {
    // 	let root_item = root.get_by_entity(e.id);

    // 	let mut render_mark_item = render_mark.get_unchecked_by_entity(e.id);
    // 	let mut render_mark_value = render_mark_item.get_or_default().clone();

    // 	match root_item {
    // 		Some(_) => {
    // 			render_mark_value.set(**local, true);
    // 		},
    // 		_ => {
    // 			render_mark_value.set(**local, false);
    // 			// 如果所有的位标记都被清除，则调用remove方法
    // 			if render_mark_value.not_any() {
    // 				render_mark_item.remove();
    // 				return;
    // 			}
    // 		},
    // 	};

    // 	render_mark_item.write(render_mark_value);

    // }
}
