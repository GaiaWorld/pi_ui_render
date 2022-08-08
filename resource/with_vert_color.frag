#version 450

// 顶点颜色渲染

layout(location = 0) in vec4 vColor;

layout(location = 0) out vec4 o_Target;

void main() {
	o_Target = vColor;
}