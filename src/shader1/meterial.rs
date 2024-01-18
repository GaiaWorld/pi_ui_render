use render_derive::{BindLayout, BindingType, BufferSize, Uniform};

#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(208)]
#[uniformbuffer]
pub struct MeterialBind; // storagebuffer: TODO


#[derive(Uniform)]
#[uniform(offset(0), len(64), bind(MeterialBind))]
pub struct WorldUniform<'a>(pub &'a [f32]);

/************************************************************************* scale(缩放) *************************************************/
#[derive(Uniform)]
#[uniform(offset(80), len(8), bind(MeterialBind))]
pub struct ScaleUniform<'a>(pub &'a [f32]);

/************************************************************************* alpha(半透明) *************************************************/
#[derive(Uniform)]
#[uniform(offset(88), len(4), bind(MeterialBind))]
pub struct AlphaUniform<'a>(pub &'a [f32]);

/************************************************************************* type(渲染类型) *************************************************/
#[derive(Uniform)]
#[uniform(offset(92), len(4), bind(MeterialBind))]
pub struct TyUniform<'a>(pub &'a [f32]);


/************************************************************************* 背景颜色 *************************************************/
#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct BackgroundColorUniform<'a>(pub &'a [f32]);

/************************************************************************** 线性渐变信息 *********************************************** */
#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct GradientPositionUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(144), len(64), bind(MeterialBind))]
pub struct GradientColorUniform<'a>(pub &'a [f32]);

/************************************************************************uv**************************************************************/

#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct UvUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(144), len(4), bind(MeterialBind))]
pub struct TextureIndexUniform<'a>(pub &'a [f32]);

/**************************************************************************** 边框信息 **********************************************/

#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct BorderColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(96), len(16), bind(MeterialBind))]
pub struct BorderRectUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(144), len(16), bind(MeterialBind))]
pub struct BorderWidthUniform<'a>(pub &'a [f32]);


/****************************************************************************bitmap 文字信息**********************************************************************/
#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(MeterialBind))]
pub struct TextUvUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(144), len(16), bind(MeterialBind))]
pub struct ColorUniform<'a>(pub &'a [f32]);

#[derive(Uniform)]
#[uniform(offset(160), len(16), bind(MeterialBind))]
pub struct StrokeColorUniform<'a>(pub &'a [f32]);

// #[derive(Uniform)]
// #[uniform(offset(176), len(8), bind(MeterialBind))]
// pub struct TextTextureSizeUniform<'a>(pub &'a [f32]);

/****************************************************************************圆裁剪信息**********************************************************************/

#[derive(Uniform)]
#[uniform(offset(96), len(8), bind(MeterialBind))]
pub struct ClipCircelCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(104), len(4), bind(MeterialBind))]
pub struct ClipCircelRadiusUniform<'a>(pub &'a [f32]);

/****************************************************************************椭圆裁剪信息*********************************************************************/
#[derive(Uniform)]
#[uniform(offset(96), len(8), bind(MeterialBind))]
pub struct ClipEllipseCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(104), len(8), bind(MeterialBind))]
pub struct ClipEllipseAbUniform<'a>(pub &'a [f32]);

/**************************************************************************** 扇形裁剪信息*************************************************************************/
#[derive(Uniform)]
#[uniform(offset(96), len(8), bind(MeterialBind))]
pub struct ClipSectorCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(104), len(8), bind(MeterialBind))]
pub struct ClipSectorRotateUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(112), len(8), bind(MeterialBind))]
pub struct ClipSectorRadianUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(120), len(4), bind(MeterialBind))]
pub struct ClipSectorRadiusUniform<'a>(pub &'a [f32]);

/*******************************************************************************矩形裁剪信息 **************************************************************************/

#[derive(Uniform)]
#[uniform(offset(96), len(8), bind(MeterialBind))]
pub struct ClipCenterUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(104), len(8), bind(MeterialBind))]
pub struct ClipExtentUniform<'a>(pub &'a [f32]);

/******************************************************************************* 圆角裁剪信息 *********************************************************************************/


#[derive(Uniform)]
#[uniform(offset(112), len(16), bind(MeterialBind))]
pub struct ClipTopRadiusUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(128), len(16), bind(MeterialBind))]
pub struct ClipBottomRadiuUniform<'a>(pub &'a [f32]);