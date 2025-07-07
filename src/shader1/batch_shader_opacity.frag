
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
layout(set=2,binding=0) uniform texture2D tex2d0;
layout(set=2,binding=2) uniform texture2D tex2d1;
layout(set=2,binding=4) uniform texture2D tex2d2;
layout(set=2,binding=6) uniform texture2D tex2d3;
layout(set=2,binding=8) uniform texture2D tex2d4;
layout(set=2,binding=10) uniform texture2D tex2d5;
layout(set=2,binding=12) uniform texture2D tex2d6;
layout(set=2,binding=14) uniform texture2D tex2d7;
layout(set=2,binding=16) uniform texture2D tex2d8;
layout(set=2,binding=18) uniform texture2D tex2d9;
layout(set=2,binding=20) uniform texture2D tex2d10;
layout(set=2,binding=22) uniform texture2D tex2d11;
layout(set=2,binding=24) uniform texture2D tex2d12;

// 采样器
layout(set=2,binding=1) uniform sampler samp0;
layout(set=2,binding=3) uniform sampler samp1;
layout(set=2,binding=5) uniform sampler samp2;
layout(set=2,binding=7) uniform sampler samp3;
layout(set=2,binding=9) uniform sampler samp4;
layout(set=2,binding=11) uniform sampler samp5;
layout(set=2,binding=13) uniform sampler samp6;
layout(set=2,binding=15) uniform sampler samp7;
layout(set=2,binding=17) uniform sampler samp8;
layout(set=2,binding=19) uniform sampler samp9;
layout(set=2,binding=21) uniform sampler samp10;
layout(set=2,binding=23) uniform sampler samp11;
layout(set=2,binding=25) uniform sampler samp12;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main(void) {
	float texture_index = vTextureInfo.z;
	
	if (texture_index < 0.1) {
		outColor = texture(sampler2D(tex2d0, samp0),vTextureInfo.xy);
	} else if (texture_index < 1.1) {
		outColor = texture(sampler2D(tex2d1, samp1),vTextureInfo.xy);
	} else if (texture_index < 2.1) {
		outColor = texture(sampler2D(tex2d2, samp2),vTextureInfo.xy);
	} else if (texture_index < 3.1) {
		outColor = texture(sampler2D(tex2d3, samp3),vTextureInfo.xy);
	} else if (texture_index < 4.1) {
		outColor = texture(sampler2D(tex2d4, samp4),vTextureInfo.xy);
	} else if (texture_index < 5.1) {
		outColor = texture(sampler2D(tex2d5, samp5),vTextureInfo.xy);
	} else if (texture_index < 6.1) {
		outColor = texture(sampler2D(tex2d6, samp6),vTextureInfo.xy);
	} else if (texture_index < 7.1) {
		outColor = texture(sampler2D(tex2d7, samp7),vTextureInfo.xy);
	} else if (texture_index < 8.1) {
		outColor = texture(sampler2D(tex2d8, samp8),vTextureInfo.xy);
	} else if (texture_index < 9.1) {
		outColor = texture(sampler2D(tex2d9, samp9),vTextureInfo.xy);
	} else if (texture_index < 10.1) {
		outColor = texture(sampler2D(tex2d10, samp10),vTextureInfo.xy);
	} else if (texture_index < 11.1) {
		outColor = texture(sampler2D(tex2d11, samp11),vTextureInfo.xy);
	} else if (texture_index < 12.1) {
		outColor = texture(sampler2D(tex2d12, samp12),vTextureInfo.xy);
	} else {
		outColor = vColor;
	}

	outColor.a = outColor.a * opacity;
}