#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

layout(set = 0, binding = 0) uniform ProjectMatrix {
	mat4 projectMatrix;
};

#ifdef VIEW
	layout(set = 1, binding = 0) uniform ViewMatrix {
		mat4 viewMatrix;
	};
#endif


layout(set = 2, binding = 0) uniform Matrix {
	mat4 worldMatrix;
};

layout(set = 3, binding = 0) uniform Depth {
	float depth;
};

layout(location = 0) out vec2 vUv;

void main() {
	gl_Position = worldMatrix * vec4(position.x, position.y, 1.0, 1.0);

	#ifdef VIEW
	gl_Position = viewMatrix * gl_Position;
	#endif

	gl_Position = projectMatrix * gl_Position;
	
	gl_Position.z = depth/60000.0;

	vUv = uv;
}