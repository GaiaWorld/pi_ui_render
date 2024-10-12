
use render_derive::{BindLayout, BindingType, BufferSize};


/***************************************************************************材质*********************************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(48)]
#[uniformbuffer]
pub struct GlowMeterialBind; // storagebuffer: TODO

impl GlowMeterialBind {
	pub const SIZE: usize = 48;
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BatchGlowMeterial {
	pub box_layout: [f32; 4],       // 0 布局位置（offset， scale）
	pub sdf_uv: [f32; 4],           // 16 uv (单位:像素)
	pub fill_bound: f32,            // 32 填充边界
}


impl pi_render::rhi::shader::WriteBuffer for BatchGlowMeterial {
	fn write_into(&self, index: u32, buffer: &mut [u8]) {
		
		unsafe { std::ptr::copy_nonoverlapping(
			self as *const Self as usize as *const u8,
			buffer.as_mut_ptr().add(index as usize),
			std::mem::size_of::<Self>(),
		) };
	}
	#[inline]
	fn byte_len(&self) -> u32 {
		std::mem::size_of::<Self>() as u32
	}

	#[inline]
	fn offset(&self) -> u32 {
		0
	}
}
impl pi_render::rhi::shader::Uniform for BatchGlowMeterial {
	type Binding = GlowMeterialBind;
}


pub fn vert_layout() -> Vec<wgpu::VertexAttribute> {
    vec![
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 0,
            shader_location: 1,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 16,
            shader_location: 2,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 32,
            shader_location: 3,
        },
    ]
}