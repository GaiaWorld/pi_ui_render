#version 450

layout(location = 0) in vec2 position; // 输入位置

#ifdef VERT_COLOR
	layout(location = 1) in vec4 vertColor;
#endif

layout(location = 0) out vec2 vVertexPosition; // 输出位置

#ifdef VERT_COLOR
	layout(location = 1) out vec4 vColor; // 输出颜色
#endif

layout(set = 0, binding = 0) uniform CameraMatrix {
	mat4 project;
	mat4 view;
};

layout(set = 1, binding = 0) uniform ColorMaterial {
	mat4 world;
	mat4 clipSdf; // border_radius | ellipse | circle | sector | rect | border
	float depth;
	float blur;
	vec2 bottomLeftBorder; // 如果是渲染边框，则需要这两个值，（clipSdf中不够放）
	vec4 color;
	vec4 uRect; // xy是矩形最小点的坐标，zw是矩阵最大点的坐标；注：矩形必须排除阴影半径。
};

void main() {
	vVertexPosition = position;

	gl_Position = project * view * world * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;

	#ifdef VERT_COLOR
		vColor = vertColor;
	#endif
}