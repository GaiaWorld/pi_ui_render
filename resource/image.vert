#version 450

#extension GL_OES_standard_derivatives : require
precision highp float;

// 输入
layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

// 输出
layout(location = 0) out vec2 vUv;
layout(location = 1) out vec2 vVertexPosition;

layout(set = 0, binding = 0) uniform CameraMatrix {
	mat4 project;
	mat4 view;
};

layout(set = 1, binding = 0) uniform UiMaterial {
	mat4 world; // 世界矩阵
	// 扇形 SDF 信息
    // [
    //    vec3 (布局中心.x, 布局中心.y, 布局缩放.x)
    //    vec3 (布局缩放.y, sin(对称轴-y轴), cos(对称轴-y轴))
    //    vec3 (sin(边缘-对称轴), cos(边缘-对称轴), r)
    // ]
	mat4 clipSdf; // border_radius | ellipse | circle | sector | rect
	float depth; // 深度
};


void main() {
	vVertexPosition = position;

	gl_Position = project * view * world * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;

	vUv = uv;
}