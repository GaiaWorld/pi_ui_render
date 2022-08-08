#version 450

// 顶点颜色渲染

layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;

layout(set = 0, binding = 0) uniform ProjectMatrix {
	mat4 projectMatrix;
};
layout(set = 1, binding = 0) uniform ViewMatrix {
	mat4 viewMatrix;
};

layout(set = 2, binding = 0) uniform Matrix {
	mat4 worldMatrix;
};

layout(set = 3, binding = 0) uniform Depth {
	float depth;
};

layout(location = 0) out vec4 vColor;

void main() {
	gl_Position = projectMatrix * viewMatrix * worldMatrix * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;

	vColor = color;
}