#version 450

#import super::camera
#import super::depth
#import super::ui_meterial

// 输入
layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;
#ifdef VERTEX_COLOR
	layout(location = 2) in vec4 color; // 顶点色
#endif

// 输出
layout(location = 0) out vec2 vVertexPosition;
layout(location = 1) out vec2 vUv;
#ifdef VERTEX_COLOR
	layout(location = 2) out vec4 vColor;
#endif

void main() {
	vVertexPosition = position;
	// 阴影偏移
	#ifdef SHADOW
		vVertexPosition.xy = vVertexPosition.xy + strokeColorOrURect.xy;
	#endif
	
	vec4 p = view * world * vec4(vVertexPosition.x, vVertexPosition.y, 1.0, 1.0);
	gl_Position = project * vec4(floor(p.x + 0.5 ), floor(p.y + 0.5), 1.0, 1.0);
	gl_Position.z = depth/60000.0;

	vUv = uv/textureSizeOrBottomLeftBorder;
#ifdef VERTEX_COLOR
	vColor = color;
#endif


}