
#version 450

precision highp float;

// 输入
// 输出
layout(location = 0) in vec2 vSdfUv; // 当前点的uv坐标
layout(location = 1) in vec2 VSdfInfo; // 单个像素的uv偏移


// sdf
layout(set=1,binding=0) uniform texture2D tex2dSdf;
layout(set=1,binding=1) uniform sampler sampSdf;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main() {
	float sdf = texture(sampler2D(tex2dSdf, sampSdf), vSdfUv).r;
	// fillSdPx为当前到填充边界的像素距离
	float fillSdPx = VSdfInfo.x * (sdf - VSdfInfo.y); 
	outColor = vec4(clamp(fillSdPx + 0.5, 0.0, 1.0), 1.0, 1.0, 1.0);
}