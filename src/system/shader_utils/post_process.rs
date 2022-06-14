use pi_ecs::prelude::{ResMut, Res, res::WriteRes};
use pi_ecs_macros::setup;
use pi_render::rhi::device::RenderDevice;

use crate::resource::draw_obj::{StateMap, Shaders, VertexBufferLayoutMap, ShareLayout, ShaderCatch, ShaderMap};

use super::{GlslShaderStatic, StaticIndex, image::init_static};

const POST_PROCESS_SHADER_VS: &'static str = "post_process_shader_vs";
const POST_PROCESS_SHADER_FS: &'static str = "post_process_shader_fs";

pub struct CalcPostProcessShader;

#[setup]
impl CalcPostProcessShader {
	#[init]
	pub fn init(
		shader_static_map: ResMut<Shaders>,
		state_map: ResMut<StateMap>,
		vertex_buffer_map: ResMut<VertexBufferLayoutMap>,
		share_layout: Res<ShareLayout>,
		mut shader_catch: ResMut<ShaderCatch>,
		mut shader_map: ResMut<ShaderMap>,
		device: Res<RenderDevice>,
		mut static_index: WriteRes<PostProcessStaticIndex>,
	) {
		let shader = GlslShaderStatic::init(
			POST_PROCESS_SHADER_VS,
			POST_PROCESS_SHADER_FS,
			&mut shader_catch, 
			&mut shader_map, 
			||{include_str!("../../source/shader/post_process.vert")}, 
			||{include_str!("../../source/shader/image.frag")});
		
		let r = init_static(
			shader_static_map,
			state_map,
			vertex_buffer_map,
			share_layout,
			device,
			shader,
		);

		// 插入背景颜色shader的索引
		static_index.write(PostProcessStaticIndex(r));
	}
}

#[derive(Deref)]
pub struct PostProcessStaticIndex(pub StaticIndex);

pub const POST_TEXTURE_GROUP: usize = 4;
pub const OPACITY_GROUP: usize = 5;
pub const POST_UV_LOCATION: usize = 1;


