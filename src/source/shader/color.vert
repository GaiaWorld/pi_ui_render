#version 450

layout(location = 0) in vec2 position;

layout(set = 0, binding = 0) uniform Camera {
	mat4 projectMatrix;
	mat4 viewMatrix;
};

layout(set = 1, binding = 0) uniform Matrix {
	mat4 worldMatrix;
};

layout(set = 2, binding = 0) uniform Depth {
	float depth;
};

void main() {
	gl_Position = projectMatrix * viewMatrix * worldMatrix * vec4(position.x, position.y, depth, 1.0);
}