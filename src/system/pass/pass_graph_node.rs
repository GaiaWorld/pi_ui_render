use pi_render::{
    components::view::target::{RenderTarget, RenderTargets, TextureViews},
    graph::{
        node::{Node, NodeRunError, RealValue},
        node_slot::SlotInfo,
        RenderContext,
    },
    rhi::CommandEncoder,
};
use futures::{future::BoxFuture, FutureExt};
use pi_ecs::prelude::QueryState;
use pi_share::ShareRefCell;

use crate::{components::{draw_obj::{DrawObject, DrawState}, pass_2d::{Camera, Draw2DList, Pass2DKey, Pass2D}}, resource::draw_obj::RenderInfo};


/// Pass2D 渲染图节点
pub struct Pass2DNode{
	// // 输入描述
	// input: Vec<SlotInfo>,
	// // 输出描述
	// output: Vec<SlotInfo>,
	pub pass2d_id: Pass2DKey,
}

impl Node for Pass2DNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn output(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn prepare(
        &self,
        _context: RenderContext,
        _inputs: &[Option<RealValue>],
        _outputs: &[Option<RealValue>],
    ) -> Option<BoxFuture<'static, Result<(), NodeRunError>>> {
        None
    }

    fn run(
        &self,
        context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        _inputs: &[Option<RealValue>],
        _outputs: &[Option<RealValue>],
    ) -> BoxFuture<'static, Result<(), NodeRunError>> {
		// println!("pass_node==========================");
        let RenderContext { mut world, .. } = context;

        let pass_query = QueryState::<
            Pass2D,
            (
				&'static Camera,
                // &'static RenderTargetKey,
                &'static Draw2DList,
            ),
        >::new(&mut world);
		let rt_key = world.get_resource::<RenderInfo>().unwrap().rt_key.clone();

        let draw_query = QueryState::<DrawObject, &'static DrawState>::new(&mut world);
		
		let pass2d_id = self.pass2d_id;
        async move {
            let rts = world.get_resource::<RenderTargets>().unwrap();
            let views = world.get_resource::<TextureViews>().unwrap();

			if let Some((
				camera, 
				// rt_key, 
				list)) = pass_query.get(&world, pass2d_id) {

				let rt = rts.get(rt_key).unwrap();
				let RenderTarget { colors, .. } = rt;
				let color_attachments = colors
					.iter()
					.map(|view| {
						let view = views.get(*view).unwrap();
						wgpu::RenderPassColorAttachment {
							resolve_target: None,
							ops: wgpu::Operations {
								load: wgpu::LoadOp::Load,
								store: true,
							},
							view: view.as_ref().unwrap(),
						}
					})
					.collect::<Vec<wgpu::RenderPassColorAttachment>>();

				// TODO Detph-Stencil
				let depth_stencil_attachment = None;

				let mut rp = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: &color_attachments,
					depth_stencil_attachment,
				});

				rp.set_viewport(
					camera.view_port.mins.x,
					camera.view_port.mins.y,
					camera.view_port.maxs.x - camera.view_port.mins.x,
					camera.view_port.maxs.y - camera.view_port.mins.y,
					0.0,
					1.0
				);
				

				// println!("pass_node1==========================opaque: {}, transparent:{}", list.opaque.len(), list.transparent.len());
				// 渲染不透明
				for e in &list.opaque {
					if let Some(state) = draw_query.get(&world, *e) {
						state.draw(&mut rp, camera);
					}
				}

				// 渲染透明
				for e in &list.transparent {
					if let Some(state) = draw_query.get(&world, *e) {
						state.draw(&mut rp, camera);
					}
				}
			}

            Ok(())
        }
        .boxed()
    }
}