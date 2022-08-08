#version 450

// 输入
layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

// 输出
layout(location = 0) out vec2 vUv;

layout(set = 0, binding = 0) uniform CameraMatrix {
	mat4 project;
	mat4 view;
};

layout(set = 1, binding = 0) uniform ImageMaterial {
	mat4 world; // 世界矩阵
	float depth; // 深度
	float opacity; // 半透明度
	mat4 transform; // 变换（用于旋转裁剪的还原）
};

void main() {
	gl_Position = view * world * vec4(position.x, position.y, 1.0, 1.0);

	#ifdef TRANSFORM
		gl_Position = transform * gl_Position;
	#endif

	gl_Position = project * gl_Position;
	
	gl_Position.z = depth/60000.0;

	vUv = uv;
}