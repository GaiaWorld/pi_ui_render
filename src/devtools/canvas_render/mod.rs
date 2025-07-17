
use std::ops::Deref;

use pi_bevy_render_plugin::{node::Node, PiScreenTexture, SimpleInOut};
use pi_render::{components::view::target_alloc::ShareTargetView, renderer::sampler::SamplerRes, rhi::{bind_group::BindGroup, buffer::Buffer, device::RenderDevice, sampler::SamplerDesc, BufferInitDescriptor}};
use pi_share::Share;
use pi_world::{query::Query, single_res::SingleRes, world::Entity};

#[derive(Default)]
pub struct CanvasRenderer {
    bindgroup: Option<BindGroup>,
    texture: Option<ShareTargetView>,
    uv: Option<Buffer>,
}


// TODO Send问题， 临时解决
unsafe impl Send for CanvasRenderer {}
unsafe impl Sync for CanvasRenderer {}


impl CanvasRenderer {
    pub fn try_change(&mut self, device: &RenderDevice, texture: &ShareTargetView, bindgroup_layout: &wgpu::BindGroupLayout) {
        let mut is_changed = false;
        match &self.texture {
            Some(t) => {
                if !Share::ptr_eq(&t.target().colors[0].0 , &texture.target().colors[0].0) {
                    is_changed = true;
                } else {
                    let rect1 = t.uv_box();
                    let rect2 = texture.uv_box();
                    if rect1 != rect2 {
                        is_changed = true;
                    }
                }
            },
            None => is_changed = true
        };
        
        if is_changed {
            let sampler = SamplerRes::new(device, &SamplerDesc::default());
            let bindgroup = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: None,
                    layout: bindgroup_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture.target().colors[0].0)},
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler.0)  },
                    ],
                }
            );
            self.bindgroup = Some(bindgroup);

            let uv: [f32; 4] = texture.uv_box();
            let uvs: [f32; 12] = [uv[0], uv[3], uv[2], uv[3], uv[2], uv[1], uv[0], uv[3], uv[2], uv[1], uv[0], uv[1]];
            // log::warn!("uvs: {:?}", (uvs, uv));
            //let points =       [-0.5, -0.5,   0.5,   -0.5,   0.5,   0.5,   -0.5, -0.5,  0.5,   0.5,   -0.5,  0.5];
            let uvs = device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("FinalRenderUv"),
                    contents: bytemuck::cast_slice(&uvs),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );
            self.uv = Some(uvs);
        }
    }
}

pub struct CanvasRendererNode {
    bindgroup_layout: wgpu::BindGroupLayout,
    vertex: Buffer,
    pipeline: wgpu::RenderPipeline,
    device: RenderDevice,

    objs: Vec<CanvasRenderer>,
}
impl CanvasRendererNode {
    pub fn new(device: &RenderDevice, _screen: &PiScreenTexture, surface_format: wgpu::TextureFormat) -> Self {
        let device = device.clone();
        let device1 = &**device;
        let points: [f32; 12] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5];
        let vertex = device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("FinalRender"),
                contents: bytemuck::cast_slice(&points),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let vs = device1.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Final-VS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./pass.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: &[]
            },
        });

        let fs = device1.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Final-FS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./pass.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: &[]
            },
        });


        let bindgroup_layout = device1.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None  },
                    wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering), count: None }
                ] 
            }
        );
        let pipeline_layout = device1.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device1.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            cache: None,
            label: Some("Final"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState  {
                compilation_options: Default::default(),
                module: &vs,
                entry_point: Some("main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 2 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 0, shader_location: 0 }
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: 2 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 0, shader_location: 1 }
                        ],
                    }
                ],
            },
            primitive: wgpu::PrimitiveState {
                polygon_mode: wgpu::PolygonMode::Fill,
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false  },
            fragment: Some(
                wgpu::FragmentState {compilation_options: Default::default(), module: &fs, entry_point: Some("main"), targets: &[Some(wgpu::ColorTargetState { format: surface_format, blend: None, write_mask: wgpu::ColorWrites::ALL })]  }
            ),
            multiview: None
        });

        Self {
            pipeline,
            vertex,
            bindgroup_layout,
            objs: Vec::default(),
            device,
        }
    }
}
impl Node for CanvasRendererNode {
    type BuildParam = Query<'static, &'static mut SimpleInOut>;
    type ResetParam = ();
	type RunParam = SingleRes<'static, PiScreenTexture>;

	fn build<'a>(
		&'a mut self,
		// _world: &'a  World,
		param: &'a mut Self::BuildParam,
		_context: pi_bevy_render_plugin::RenderContext,
		_id: Entity,
		from: &'a [Entity],
		_to: &'a [Entity],
	) -> Result<(), String> {
        let mut i = 0;
        if from.len() > 0 {
            for entity in from.iter() {
                let input = match param.get(*entity) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                let target = input.target.as_ref().unwrap();
                if self.objs.get(i).is_none() {
                    self.objs.push(CanvasRenderer::default());
                }
                let obj = &mut self.objs[i];
                obj.try_change(&self.device, target, &self.bindgroup_layout);
                i += 1;
            }
        }
		Ok(())
	}

    fn run<'a>(
        &'a mut self,
        // world: &'a World,
        param: &'a Self::RunParam,
        // param: &'a mut bevy_ecs::system::SystemState<Self::RunParam>,
        _context: pi_bevy_render_plugin::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
		_id: Entity,
		from: &'a [Entity],
		_to: &'a [Entity],
    ) -> pi_futures::BoxFuture<'a, Result<(), String>> {
        if from.len() > 0 {
            let mut rpass = commands.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some(""),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: param.as_ref().unwrap().view().as_ref().unwrap(),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );
            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, self.vertex.slice(..).deref().clone());
            let mut i = 0;
             for _entity in from.iter() {
                let obj = &self.objs[i];
                let b = obj.bindgroup.as_ref().unwrap();
                rpass.set_bind_group(0, &**b, &[]);
                rpass.set_vertex_buffer(1, obj.uv.as_ref().unwrap().slice(..).deref().clone());
                rpass.draw(0..6, 0..1);
                i += 1;
            }
        }

        Box::pin(async move {
            Ok(())
        })
    }
    
    fn reset<'a>(
        &'a mut self,
        _param: &'a mut Self::ResetParam,
        _context: pi_bevy_render_plugin::RenderContext,
        _id: pi_world::world::Entity,
    ) {
    }

    
}

// fn sys_changesize(
//     window: SingleRes<PiRenderWindow>,
//     device: SingleRes<PiRenderDevice>,


//     mut final_render: SingleResMut<CanvasRenderer>,
// ) {
//     if window.0.width > 0 && window.0.height > 0 {
//         let surface_size = wgpu::Extent3d { width: window.0.width, height: window.0.height, depth_or_array_layers: 1 };
//         final_render.change(wgpu::TextureFormat::Rgba8Unorm, surface_size, &device);
//     }
// }