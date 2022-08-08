#version 450
// 输入uv
layout(location = 0) in vec2 vUv;
// 输出颜色
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ImageMaterial {
	mat4 world; // 世界矩阵
	float depth; // 深度
	float opacity; // 半透明度
	mat4 transform; // 变换（用于旋转裁剪的还原）
};

// 纹理
layout (set = 2, binding = 0) uniform sampler samp;
layout (set = 2, binding = 1) uniform texture2D tex2d;

void main() {
	o_Target = texture(sampler2D(tex2d, samp), vUv);
	#ifdef OPACITY
		o_Target = o_Target * opacity;
		// o_Target = vec4(o_Target.r * opacity, o_Target.g * opacity, o_Target.b * opacity, o_Target.a * opacity);
	#endif
}