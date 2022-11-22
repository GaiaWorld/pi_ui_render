#version 450

layout(location = 0) in vec2 position;

layout(set = 0, binding = 0) uniform CameraMatrix {
	mat4 project;
	mat4 view;
};

layout(set = 1, binding = 0) uniform MeshBase {
	mat4 world;
	float depth;
};

layout(location = 0) out vec2 vPosition;

void main(void) {
	gl_Position = project * view * world * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;

	vPosition = vec2(gl_Position.x, gl_Position.y);
}