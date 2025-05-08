
#version 450

precision highp float;

// 输入
layout(location = 0) in vec4 vColor; // color
layout(location = 1) in vec4 vStrokeColor; // strokeColor
layout(location = 2) in vec4 vSdf; // sdf distancePixelRange, fillSdf, strokeSdf, not_premultiply(非预乘因子， 预乘为0.001， 非预乘为1.0)
layout(location = 3) in vec4 vTextureInfo; // uv + texture_index + strokeFactor(该值为0.0时，表示描边， 为1.0时表示不描边， 为2.0是， 表示采样纹理为阴影纹理或外发光纹理)
layout(location = 4) in vec2 vSdfUv; // uv + texture_index

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

	// rgb还原预乘模式（只有采样纹理才可能还原预乘法）
	outColor.rgb = outColor.rgb / clamp(outColor.a, vSdf.w, 1.0);

	float sdf = texture(sampler2D(tex2dSdf, sampSdf), vSdfUv).r;
	// fillSdPx为当前到填充边界的像素距离
	float fillSdPx = vSdf.x * (sdf - vSdf.y); 
	float fillOpacity = clamp(fillSdPx + 0.5, 0.0, 1.0);
	
	if (vTextureInfo.w < 0.1) {
		// 填充与边框混合（在填充和描边交界处需要两种颜色过度）
		outColor = mix(vStrokeColor, outColor, fillOpacity);

		// outlineSdPx为当前到描边边界的像素距离，描边(计算alpha值，表示描边与背景的混合因子)
		float outlineSdPx = vSdf.x * (sdf - vSdf.z);
		float outlineOpacity = clamp(outlineSdPx + 0.5, 0.0, 1.0);
		// outColor = vec4(sdf, 0.0, 0.0, 1.0);
		outColor.a = outColor.a * clamp(max(outlineOpacity, fillOpacity), 0.0, 1.0);
	} else if (vTextureInfo.w < 1.1) {
		outColor.a = outColor.a * fillOpacity;
	} else {
		// 阴影或外发光， 把outColor的r值表示灰度
		outColor.rgba = vec4(vColor.rgb, outColor.r);
	}

	// outColor = vec4(sdf, 0.0, 0.0, 1.0);
}