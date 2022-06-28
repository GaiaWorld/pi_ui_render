#version 450
// 输入uv
layout(location = 0) in vec2 vUv;

// 纹理
layout (set = 4, binding = 0) uniform sampler samp;
layout (set = 4, binding = 1) uniform texture2D tex2d;

// 输出颜色
layout(location = 0) out vec4 o_Target;

void main() {
	o_Target = texture(sampler2D(tex2d, samp), vUv);
}