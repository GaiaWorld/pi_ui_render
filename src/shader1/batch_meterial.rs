use pi_render::rhi::shader::{GetBuffer, WriteBuffer};
use render_derive::{BindLayout, BindingType, BufferSize, Input, Uniform};

// clip_rect = 1; clip_rect_radius = 4; clip_circel= 8; clip_ellipse = 16; clip_secotor = 32; uv = 64; color = 128; canvas_text = 256;text_stroke = 512;is_not_visibility = 1024;pointer_samp = 2048;linear_gradient = 4096

/// 渲染标记， 用于设置着色器如何计算
pub enum RenderFlagType {
	// ClipRect = 0, // 1
	IgnoreCamera = 1, // 2（不需要乘视图矩阵、投影矩阵）
	ClipRectRadius = 2, // 4 (圆角)
	// ClipCircel = 3, // 8
	Premulti = 3, // 8 预乘模式渲染（需要预乘模式渲染的物体用普通模式渲染， 则设置此标记， 在着色器中会将预乘结果还原）
	R8 = 4, // 16 表示被采样纹理是一个R8纹理， 将r作为aplha值，应用在vColor上
	ClipSector = 5, // 32
	Uv = 6, // 64
	Color = 7, // 128
	// 存在描边时， 顶点着色器应该设置描边因子为0, 否则设置为1.0， 该因此用于乘以填充颜色
	// 不存在描边时， 要保证strokeColor设置为全0， 同时描边因子为1.0， 使得 strokeColor + strokefactor * vColor = vColor
	Stroke = 8, // 256  
	TextStroke = 9, // 512
	NotVisibility = 10, // 1024
	Invalid = 11, // 2048 无效渲染， 诸如像文字这类渲染， 可能会在实例buffer上保留一定数量的冗余实例， 这些实例被设置为无效渲染
	LinearGradient = 12, // 4096
	Opacity = 13, // 8192
	// BoxShadow = 14, // 16384
	// ImageRepeat = 15, // 32768 图片uv重复
	// BorderImage = 16, // 65536 border图片uv重复
	// Sdf2 = 17, // 131072 sdf2渲染
	// Sdf2OutGlow = 18, // 262144 sdf2文字外发光
	// SvgStrokeDasharray = 19, // 524288 需要占用外发光槽位
	// Svg = 20, // 1048576 svg uv 不需要y轴颠倒
	// Sdf2Shadow = 21, // 2097152 阴影 需要占用外发光槽位
}

#[derive(Input)]
#[location(0)]
pub struct PositionVert;

/*******************************************************************************相机**************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(0), binding(0))]
#[min_size(160)]
#[uniformbuffer]
pub struct CameraBind; // storagebuffer: TODO

/// 投影矩阵
#[derive(Uniform)]
#[uniform(offset(0), len(64), bind(CameraBind))]
pub struct ProjectUniform<'a>(pub &'a [f32]);

/// 视图矩阵
#[derive(Uniform)]
#[uniform(offset(64), len(64), bind(CameraBind))]
pub struct ViewUniform<'a>(pub &'a [f32]);

/// sdf纹理尺寸
/// [index_tex_w, index_tex_h, data_tex_w, data_tex_w]
#[derive(Uniform)]
#[uniform(offset(128), len(8), bind(CameraBind))]
pub struct Sdf2TextureSizeUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(136), len(8), bind(CameraBind))]
pub struct TargetTextureSizeUniform<'a>(pub &'a [f32]);

// /// 用于调试，投影矩阵对应的Aabb
// #[derive(Uniform)]
// #[uniform(offset(144), len(16), bind(CameraBind))]
// pub struct ProjectAabbUniform<'a>(pub &'a [f32]);



/***************************************************************************材质*********************************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(224)]
#[uniformbuffer]
pub struct MeterialBind; // storagebuffer: TODO

impl MeterialBind {
	pub const SIZE: usize = 224;
}

#[derive(Uniform, Debug)]
#[uniform(offset(0), len(64), bind(MeterialBind))]
pub struct WorldMatrixMeterial<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct LayoutUniform<'a>(pub &'a [f32]);

// 三个顶点的位置
#[derive(Uniform, Debug)]
#[uniform(offset(64), len(24), bind(MeterialBind))]
pub struct LinearGradientPointUniform<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(88), len(4), bind(MeterialBind))]
pub struct TetxureIndexMeterial<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(92), len(4), bind(MeterialBind))]
pub struct TyMeterial<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(92), len(4), bind(MeterialBind))]
pub struct TyMeterialMut<'a>(pub &'a mut [f32]);

impl<'a> GetBuffer for TyMeterialMut<'a> {
	fn get_data(&mut self, index: u32, buffer: &[u8]) {
		let len = self.0.len() * 4;
		unsafe {
			buffer.as_ptr().add(index as usize + self.offset() as usize).copy_to_nonoverlapping(self.0.as_mut_ptr() as usize as *mut u8, len);
		};
	}
}

// 倾斜
#[derive(Uniform, Debug)]
#[uniform(offset(96), len(16), bind(MeterialBind))]
pub struct SlopeUniform<'a>(pub &'a [f32]);

// 三个顶点的颜色
#[derive(Uniform, Debug)]
#[uniform(offset(112), len(48), bind(MeterialBind))]
pub struct LinearGradientColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(128), len(16), bind(MeterialBind))]
pub struct ColorUniform<'a>(pub &'a [f32]);

/// 0~1范围
#[derive(Uniform, Debug)]
#[uniform(offset(144), len(16), bind(MeterialBind))]
pub struct UvUniform<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(160), len(16), bind(MeterialBind))]
pub struct StrokeColorUniform<'a>(pub &'a [f32]);

/// 半透明, 半透明只在渲染图片时存在， 不可能存在描边， 因此与描边的偏移保持一致， 不会冲突
#[derive(Uniform, Debug)]
#[uniform(offset(160), len(4), bind(MeterialBind))]
pub struct OpacityUniform<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(176), len(12), bind(MeterialBind))]
pub struct SdfUniform<'a>(pub &'a [f32]);

#[derive(Uniform, Debug)]
#[uniform(offset(188), len(4), bind(MeterialBind))]
pub struct DepthUniform<'a>(pub &'a [f32]);

/// 单位： 像素
#[derive(Uniform, Debug)]
#[uniform(offset(192), len(16), bind(MeterialBind))]
pub struct SdfUvUniform<'a>(pub &'a [f32]);

/// 单位： 像素
#[derive(Uniform, Debug)]
#[uniform(offset(192), len(24), bind(MeterialBind))]
pub struct LinearGradientSdfUvUniform<'a>(pub &'a [f32]);

#[repr(C)]
#[derive(Debug)]
pub struct BatchMeterial {
	pub other: [f32; 4],
	pub slope: [f32; 4],
	pub color0: [f32; 4],
	pub color: [f32; 4],
	pub uv: [f32; 4], // min, max  (min是左上角)
	pub stroke_color: [f32; 4],
	pub sdf: [f32; 4], // distance_px_range, fill_bound, stroke_bound, depth
	pub sdf_uv: [f32; 4],
}

impl pi_render::rhi::shader::WriteBuffer for BatchMeterial {
	fn write_into(&self, index: u32, buffer: &mut [u8]) {
		
		unsafe { std::ptr::copy_nonoverlapping(
			self as *const Self as usize as *const u8,
			buffer.as_mut_ptr().add(index as usize + 80),
			128,
		) };
	}
	#[inline]
	fn byte_len(&self) -> u32 {
		128
	}

	#[inline]
	fn offset(&self) -> u32 {
		80
	}
}
impl pi_render::rhi::shader::Uniform for BatchMeterial {
	type Binding = MeterialBind;
}

impl Default for BatchMeterial {
	fn default() -> Self {
		Self {
			other: [0.0, 0.0, 50.0, 0.0],
			slope: Default::default(),
			color: Default::default(), 
			stroke_color: Default::default(), 
			sdf: [5000.0/*默认设置很大， 使其很锐*/, 0.498, 0.498, 0.0], 
			uv: Default::default(),
			sdf_uv: Default::default(),
    		color0: Default::default(),
		}
	}
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
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 48,
            shader_location: 4,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 64,
            shader_location: 5,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 80,
            shader_location: 6,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 96,
            shader_location: 7,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 112,
            shader_location: 8,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 128,
            shader_location: 9,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 144,
            shader_location: 10,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 160,
            shader_location: 11,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 176,
            shader_location: 12,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 192,
            shader_location: 13,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: 208,
            shader_location: 14,
        },
        // wgpu::VertexAttribute {
        // 	format: wgpu::VertexFormat::Float32x4,
        // 	offset: 224,
        // 	shader_location: 15,
        // },
        // wgpu::VertexAttribute {
        // 	format: wgpu::VertexFormat::Float32x4,
        // 	offset: 224,
        // 	shader_location: 15,
        // },
        // wgpu::VertexAttribute {
        // 	format: wgpu::VertexFormat::Float32x4,
        // 	offset: 240,
        // 	shader_location: 16,
        // },
    ]
}


/******************************************************************************调试字段***************************************************************************/

#[derive(Uniform, Debug)]
#[uniform(offset(220), len(4), bind(MeterialBind))]
pub struct DebugInfo<'a>(pub &'a [f32]);

