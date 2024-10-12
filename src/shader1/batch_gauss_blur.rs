
use render_derive::{BindLayout, BindingType, BufferSize, Uniform};

/*******************************************************************************相机**************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(0), binding(0))]
#[min_size(8)]
#[uniformbuffer]
pub struct TargetSizeBind; // storagebuffer: TODO

/// 投影矩阵
#[derive(Uniform)]
#[uniform(offset(0), len(8), bind(TargetSizeBind))]
pub struct TargetTexSizeUniform<'a>(pub &'a [f32]);


/***************************************************************************材质*********************************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(48)]
#[uniformbuffer]
pub struct GussMeterialBind; // storagebuffer: TODO

impl GussMeterialBind {
	pub const SIZE: usize = 48;
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BatchGussMeterial {
	pub box_layout: [f32; 4],       // 0 布局位置（offset， scale）
	pub uv: [f32; 4],        // 16 uv (单位:像素)
	pub texture_size: [f32; 2],           // 40 blurRadius
	pub blur_radius: f32,           // 40 blurRadius
	pub direcition: f32,            // 44 方向 GaussDirecition
}

#[repr(C)]
#[derive(Debug)]
pub enum GaussDirecition {
	Horizontal = 0,
	Vertical = 1,
}

impl pi_render::rhi::shader::WriteBuffer for BatchGussMeterial {
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
impl pi_render::rhi::shader::Uniform for BatchGussMeterial {
	type Binding = GussMeterialBind;
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
            format: wgpu::VertexFormat::Float32x2,
            offset: 32,
            shader_location: 3,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32,
            offset: 40,
            shader_location: 4,
        },
		wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32,
            offset: 44,
            shader_location: 5,
        },
    ]
}