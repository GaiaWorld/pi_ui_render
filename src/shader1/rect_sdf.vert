#version 450

precision highp float;
// 输入
layout(location = 0) in vec2 position; // 输入位置
// 输出
layout(location = 0) out vec2 vVertexPosition; // 输出位置

layout(set = 0, binding = 0) uniform RectSdfMeterial {
	vec4 geo; // offset + scale (0~1范围)
	vec2 center; // 中心点(0~1范围)
	vec2 extent; // 半宽半高(0~1范围)
};

void main() {
	vVertexPosition = position * geo.zw + geo.xy; // 位置（0~1范围）
	gl_Position = vec4(vVertexPosition * 2.0 - 1.0, 0.0, 1.0); // 位置（-1~1范围）
}