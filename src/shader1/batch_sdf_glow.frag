
#version 450

precision highp float;

// 输入
// 输出
layout(location = 0) in vec2 vSdfUv; // 当前点的uv坐标
layout(location = 1) in float fill_bound; 


// sdf
layout(set=1,binding=0) uniform texture2D tex2dSdf;
layout(set=1,binding=1) uniform sampler sampSdf;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main() {
	float sdf = texture(sampler2D(tex2dSdf, sampSdf), vSdfUv).r;

	outColor = vec4(pow(clamp(1.0 - (fill_bound - sdf) / fill_bound, 0.0, 1.0), 3.0), 0.0, 0.0, 1.0);
}