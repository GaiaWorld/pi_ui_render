use pi_render::rhi::shader::{GetBuffer, WriteBuffer};
use render_derive::{BindLayout, BindingType, BufferSize, Input, Uniform};

// clip_rect = 1; clip_rect_radius = 4; clip_circel= 8; clip_ellipse = 16; clip_secotor = 32; uv = 64; color = 128; canvas_text = 256;text_stroke = 512;is_not_visibility = 1024;pointer_samp = 2048;linear_gradient = 4096

/// 渲染标记， 用于设置着色器如何计算
pub enum RenderFlagType {
	ClipRect = 0, // 1
	ClipRectRadius = 2, // 4
	ClipCircel = 3, // 8
	ClipEllipse = 4, // 16
	ClipSector = 5, // 32
	Uv = 6, // 64
	Color = 7, // 128
	CanvasText = 8, // 256
	TextStroke = 9, // 512
	NotVisibility = 10, // 1024
	PointerSamp = 11, // 2048
	LinearGradient = 12, // 4096
	Border = 13, // 8192
	BoxShadow = 14, // 16384
	ImageRepeat = 15, // 32768 图片uv重复
	BorderImage = 16, // 65536 border图片uv重复
	Sdf2 = 17, // 131072 sdf2渲染
	Sdf2OutGlow = 18, // 262144 sdf2文字外发光
	SvgStrokeDasharray = 19, // 524288 需要占用外发光槽位
	Svg = 20, // 1048576 svg uv 不需要y轴颠倒
	Shadow = 21, // 2097152 阴影 需要占用外发光槽位
	Premulti = 22, // 4194304 预乘模式渲染（需要预乘模式渲染的物体用普通模式渲染， 则设置此标记， 在着色器中会将预乘结果还原）
}

#[derive(Input)]
#[location(0)]
pub struct PositionVert;

/*******************************************************************************相机**************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(0), binding(0))]
#[min_size(144)]
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
#[uniform(offset(128), len(16), bind(CameraBind))]
pub struct Sdf2TextureSizeUniform<'a>(pub &'a [f32]);



/***************************************************************************材质*********************************************************************************/
#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(240)]
#[uniformbuffer]
pub struct MeterialBind; // storagebuffer: TODO

impl MeterialBind {
	pub const SIZE: usize = 240;
}


// #[derive(Uniform)]
// #[uniform(offset(0), len(64), bind(MeterialBind))]
// pub struct WorldUniform<'a>(pub &'a [f32]);

/// 深度偏移（设置在视口矩阵上）（不是实例数据， 而是设置在uniform上）
#[derive(Uniform)]
#[uniform(offset(184), len(4), bind(MeterialBind))]
pub struct DepthUniform<'a>(pub &'a [f32]);

/************************************************************************* offset(偏移) + scale(缩放) *************************************************/
#[derive(Uniform)]
#[uniform(offset(16), len(16), bind(MeterialBind))]
pub struct BoxUniform<'a>(pub &'a [f32]);

// 世界坐标系坐标，左上角原点， 逆时针顺序
#[derive(Uniform)]
#[uniform(offset(192), len(32), bind(MeterialBind))]
pub struct QuadUniform<'a>(pub &'a [f32]);

/************************************************************************* blur *************************************************/
#[derive(Uniform)]
#[uniform(offset(176), len(4), bind(MeterialBind))]
pub struct AlphaUniform<'a>(pub &'a [f32]);

/************************************************************************* type(渲染类型) *************************************************/
#[derive(Uniform)]
#[uniform(offset(180), len(4), bind(MeterialBind))]
pub struct TyUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(180), len(4), bind(MeterialBind))]
pub struct TyUniformMut<'a>(pub &'a mut [f32]);


impl<'a> GetBuffer for TyUniformMut<'a> {
	fn get_data(&mut self, index: u32, buffer: &[u8]) {
		let len = self.0.len() * 4;
		unsafe {
			buffer.as_ptr().add(index as usize + self.offset() as usize).copy_to_nonoverlapping(self.0.as_mut_ptr() as usize as *mut u8, len);
		};
	}
}



/************************************************************************* 背景颜色 *************************************************/
#[derive(Uniform)]
#[uniform(offset(0), len(16), bind(MeterialBind))]
pub struct ColorUniform<'a>(pub &'a [f32]);

/************************************************************************** 线性渐变信息 *********************************************** */
#[derive(Uniform)]
#[uniform(offset(0), len(16), bind(MeterialBind))]
pub struct GradientPositionUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(80), len(64), bind(MeterialBind))]
pub struct GradientColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(144), len(16), bind(MeterialBind))]
pub struct GradientEndUniform<'a>(pub &'a [f32]);

/************************************************************************** box shadow *********************************************** */
#[derive(Uniform)]
#[uniform(offset(80), len(16), bind(MeterialBind))]
pub struct BoxShadowUniform<'a>(pub &'a [f32]); // h, v, spread, blur

/************************************************************************uv**************************************************************/

#[derive(Uniform)]
#[uniform(offset(0), len(16), bind(MeterialBind))]
pub struct UvUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(80), len(4), bind(MeterialBind))]
pub struct TextureIndexUniform<'a>(pub &'a [f32]);

// [uoffset, voffset, ustep, vstep, uspace, vspace]
#[derive(Uniform)]
#[uniform(offset(88), len(24), bind(MeterialBind))]
pub struct ImageRepeatUniform<'a>(pub &'a [f32]);

/**************************************************************************** 边框信息 **********************************************/

#[derive(Uniform)]
#[uniform(offset(0), len(16), bind(MeterialBind))]
pub struct BorderColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(32), len(16), bind(MeterialBind))]
pub struct BorderRectUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(80), len(16), bind(MeterialBind))]
pub struct BorderWidthUniform<'a>(pub &'a [f32]);

/****************************************************************BorderImage*******************************************************************/
#[derive(Uniform)]
#[uniform(offset(84), len(92), bind(MeterialBind))]
pub struct BorderImageInfoUniform<'a>(pub &'a [f32]);

/****************************************************************************bitmap 文字信息**********************************************************************/
#[derive(Uniform)]
#[uniform(offset(0), len(16), bind(MeterialBind))]
pub struct TextUvUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(80), len(16), bind(MeterialBind))]
pub struct TextColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(96), len(16), bind(MeterialBind))]
pub struct StrokeColorUniform<'a>(pub &'a [f32]);

// #[derive(Uniform)]
// #[uniform(offset(176), len(8), bind(MeterialBind))]
// pub struct TextTextureSizeUniform<'a>(pub &'a [f32]);


/*******************************************************************************矩形裁剪信息 **************************************************************************/
// 矩形、圆角矩形、椭圆都需要传该信息， 表示渲染包围盒的范围
// 用中心点和半宽半高表示
#[derive(Uniform)]
#[uniform(offset(32), len(16), bind(MeterialBind))]
pub struct ClipRectRoundUniform<'a>(pub &'a [f32]);

/******************************************************************************* 圆角裁剪信息 *********************************************************************************/


#[derive(Uniform)]
#[uniform(offset(48), len(32), bind(MeterialBind))]
pub struct ClipRadiusUniform<'a>(pub &'a [f32]);

/****************************************************************************圆裁剪信息**********************************************************************/

// 中心点和半径
#[derive(Uniform)]
#[uniform(offset(32), len(12), bind(MeterialBind))]
pub struct ClipCircelUniform<'a>(pub &'a [f32]);


/****************************************************************************椭圆裁剪信息*********************************************************************/
#[derive(Uniform)]
#[uniform(offset(32), len(8), bind(MeterialBind))]
pub struct ClipEllipseCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(40), len(8), bind(MeterialBind))]
pub struct ClipEllipseAbUniform<'a>(pub &'a [f32]);

/**************************************************************************** 扇形裁剪信息*************************************************************************/
#[derive(Uniform)]
#[uniform(offset(32), len(8), bind(MeterialBind))]
pub struct ClipSectorCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(40), len(8), bind(MeterialBind))]
pub struct ClipSectorRotateUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(48), len(8), bind(MeterialBind))]
pub struct ClipSectorRadianUniform<'a>(pub &'a [f32]);





/********************************************************************************文字信息*******************************************************************************/
// 纯色文字，直接用ColorUnifom


// 文字外法光颜色rgb + 外法光发散范围
#[derive(Uniform)]
#[uniform(offset(80), len(16), bind(MeterialBind))]
pub struct TextOuterGlowUniform<'a>(pub &'a [f32]);

// 文字粗细
#[derive(Uniform)]
#[uniform(offset(188), len(4), bind(MeterialBind))]
pub struct TextWeightUniform<'a>(pub &'a [f32]);

// 文字渐变色（和普通渐变色的区别是， 文字渐变仅支持rgb， 而不支持alpha通道）
// [vec3 color1, vec3 color2, vec3 color3, vec3 color4]
#[derive(Uniform)]
#[uniform(offset(96), len(48), bind(MeterialBind))]
pub struct TextGradientColorUniform<'a>(pub &'a [f32]);

// 文字描边颜色rgb + 描边宽度u_outline.w

#[derive(Uniform)]
#[uniform(offset(160), len(16), bind(MeterialBind))]
pub struct TextOutlineUniform<'a>(pub &'a [f32]);

/// sdf信息[max_offset, min_sdf, sdf_step, check, index_offset_x, index_offset_y, index_w, index_h, data_offset_x, data_offset_y, scope_factor, scope_y]
#[derive(Uniform)]
#[uniform(offset(32), len(48), bind(MeterialBind))]
pub struct Sdf2InfoUniform<'a>(pub &'a [f32]);

/// 阴影偏移和模糊等级[offest_x, offest_y, blur_level]
#[derive(Uniform)]
#[uniform(offset(224), len(12), bind(MeterialBind))]
pub struct ShadowUniform<'a>(pub &'a [f32]);

// layout(location = 1) in vec4 matrix0; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 2) in vec4 matrix1; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 3) in vec4 matrix2; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 4) in vec4 matrix3; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 5) in vec4 data0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color; 64
// layout(location = 6) in vec4 data1; // vec4 offset + scale; 
// layout(location = 7) in vec4 data2; // 192 vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要) |info (sdf信息，不知道什么意思，渲染时需要)
// layout(location = 8) in vec4 data3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | vec4 index_info(索引纹理宽高（晶格个数）， 索引纹理偏移（单位： 像素）)
// layout(location = 9) in vec4 data4; // vec4 clip_bottom_radius | (vec2 data_offset;float scope; )
// layout(location = 10) in vec4 data5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_index | vec4 box_shadow(h, v, spread, blur)
// layout(location = 11) in vec4 data6; // vec4 gradient_color1
// layout(location = 12) in vec4 data7; // vec4 gradient_color2
// layout(location = 13) in vec4 data8; // vec4 gradient_color3
// layout(location = 14) in vec4 data9; // vec4 gradient_end
// layout(location = 15) in vec4 ; // vec4 border_image_offset; | vec4 u_outline(描边颜色rgb + 描边宽度u_outline.w) 224
// layout(location = 16) in vec4 data11; // float alpha; float ty;float depth; 

