
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
layout(set=2,binding=0) uniform texture2D tex2d;

// 采样器
layout(set=2,binding=1) uniform sampler samp;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main(void) {
	outColor = texture(sampler2D(tex2d, samp),vTextureInfo.xy);
	outColor = outColor + vColor;

	// rgb还原预乘模式（只有采样纹理才可能还原预乘法）
	outColor.rgb = outColor.rgb / clamp(outColor.a, vSdf.w, 1.0);

	float sdf = texture(sampler2D(tex2dSdf, sampSdf), vSdfUv).r;
	// fillSdPx为当前到填充边界的像素距离
	float fillSdPx = vSdf.x * (sdf - vSdf.y); 
	float fillOpacity = clamp(fillSdPx + 0.5, 0.0, 1.0);
	
	if (vTextureInfo.w < 0.1) {
		// 如果需要描边
		// 填充与边框混合（在填充和描边交界处需要两种颜色过度）
		outColor = mix(vStrokeColor, outColor, fillOpacity);

		// outlineSdPx为当前到描边边界的像素距离
		float outlineSdPx = vSdf.x * (sdf - vSdf.z);
		// outlineOpacity，表示描边与背景的混合因子
		float outlineOpacity = clamp(outlineSdPx + 0.5, 0.0, 1.0);
		// outColor = vec4(sdf, 0.0, 0.0, 1.0);
		outColor.a = outColor.a * clamp(max(outlineOpacity, fillOpacity), 0.0, 1.0);
	} else if (vTextureInfo.w < 1.1) {
		outColor.a =  outColor.a * fillOpacity * opacity;
	} else {
		// 阴影或外发光， 把outColor的r值表示灰度
		outColor.rgba = vec4(vColor.rgb, outColor.r);
	}

	// outColor = vec4(sdf, 0.0, 0.0, 1.0);
}