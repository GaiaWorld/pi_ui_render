#version 450

layout (set = 4, binding = 0) uniform Color {
	vec4 color;
};

layout(location = 0) out vec4 o_Target;

void main() {
	o_Target = color;
	// o_Target = vec4(1.0, 0.0, 0.0, 1.0);
}