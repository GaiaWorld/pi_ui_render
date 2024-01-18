#version 450

precision highp float;

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
};

// 输入
layout(location = 0) in vec2 position; // 输入位置
layout(location = 1) in vec4 matrix0; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
layout(location = 2) in vec4 matrix1; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
layout(location = 3) in vec4 matrix2; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
layout(location = 4) in vec4 matrix3; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
layout(location = 5) in vec4 data0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 6) in vec4 data1; // vec2 scale; float alpha; float ty;
layout(location = 7) in vec4 data2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
layout(location = 8) in vec4 data3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | 
layout(location = 9) in vec4 data4; // vec4 clip_bottom_radius
layout(location = 10) in vec4 data5; // vec4 gradient_color0
layout(location = 11) in vec4 data6; // vec4 gradient_color1
layout(location = 12) in vec4 data7; // vec4 gradient_color2
layout(location = 13) in vec4 data7; // vec4 gradient_color3
// bgcolor = 1;bordercolor = 2;borderimage = 3;bgimage=4;borderimage_repeat = 5;bgimage_repeat=6;bitmaptext = 7;fbo_rect = 8;fbo_circel=9;fbo_ellipse=10;fbo_sector=11;fbo_radius=12;fbo_svg=13;


// 输出
layout(location = 0) out vec2 vVertexPosition; // 输出位置
layout(location = 1) out vec2 vUv; // 输出位置

layout(location = 5) in vec4 vData0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 6) in vec4 vData1; // vec2 scale; float alpha; float ty;
layout(location = 7) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
layout(location = 8) in vec4 vData3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | 
layout(location = 9) in vec4 vData4; // vec4 clip_bottom_radius
layout(location = 10) in vec4 vData5; // vec4 gradient_color0
layout(location = 11) in vec4 vData6; // vec4 gradient_color1
layout(location = 12) in vec4 vData7; // vec4 gradient_color2
layout(location = 13) in vec4 vData8; // vec4 gradient_color3


void main() {
	int ty1 = int(data1.z); // clip = 1;uv = 4;

	vVertexPosition = position * data1.xy; // 位置（布局坐标系）

	if (ty1 & 64 != 0 || ty1 & 256 != 0) {  // 需要计算uv
		vUv = data3.xy + position * (data3.zw - data3.xy);
	}

	vData0 = data0;
	vData1 = data1;
	vData2 = data2;
	vData3 = data3;
	vData4 = data4;
	vData5 = data5;
	vData6 = data6;
	vData7 = data7;
	vData8 = data8;


	gl_Position = project * view * mat4(matrix0, matrix1, matrix2, matrix3) * vec4(position.xy, 1.0, 1.0);
	gl_Position.z = matrix3[3]/60000.0;

	if (ty1 & 1024 != 0) { // is_not_visibility(不可见时， 返回的顶点面积为0)
		gl_Position = vec4(0.0, 0.0, 0.0, 0.0);
	}
}