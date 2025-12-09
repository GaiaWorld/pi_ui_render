
#version 450

precision highp float;

// 输入
layout(location = 0) in vec4 vColor; // color
layout(location = 1) in vec4 vStrokeColor; // strokeColor
layout(location = 2) in vec4 vSdf; // sdf distancePixelRange, fillSdf, strokeSdf, not_premultiply(非预乘因子， 预乘为0.001， 非预乘为1.0)
layout(location = 3) in vec4 vTextureInfo; // uv + texture_index + strokeFactor(该值为0.0时，表示描边， 为1.0时表示不描边， 为2.0是， 表示采样纹理为阴影纹理或外发光纹理)
layout(location = 4) in vec2 vSdfUv; // uv + texture_index
layout(location = 5) in float opacity; // 半透明度

// sdf
layout(set=1,binding=0) uniform texture2D tex2dSdf;
layout(set=1,binding=1) uniform sampler sampSdf;

// 纹理， 最多一次传13张纹理
layout(set=2,binding=0) uniform texture2DArray tex2d;

// 采样器
layout(set=2,binding=1) uniform sampler samp;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main(void) {
	outColor = texture(sampler2DArray(tex2d, samp),vTextureInfo.xyz);
	// android 压缩纹理数组采样通道可能小于0？
	outColor.a = clamp(outColor.a, 0.0, 1.0);
	outColor.r = clamp(outColor.r, 0.0, 1.0);
	outColor.g = clamp(outColor.g, 0.0, 1.0);
	outColor.b = clamp(outColor.b, 0.0, 1.0);
	outColor = outColor + vColor;

	outColor.a = outColor.a * opacity;
}