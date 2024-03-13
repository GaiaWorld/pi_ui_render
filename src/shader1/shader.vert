#version 450

precision highp float;

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
	vec2 index_tex_size; // sdf2： 索引纹理尺寸
	vec2 data_tex_size; // sdf2： 数据纹理尺寸
};



// 输入
layout(location = 0) in vec2 position; // 输入位置
// layout(location = 1) in float depth_offset; // 深度值的偏移
// layout(location = 1) in vec4 matrix0; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 2) in vec4 matrix1; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 3) in vec4 matrix2; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
// layout(location = 4) in vec4 matrix3; // matrix[0] （这里因为顶点流最大仅支持vec4，因此将矩阵分为4个vec4）
layout(location = 1) in vec4 data0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 2) in vec4 data1; // vec4 offset + scale; 
layout(location = 3) in vec4 data2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要) |info (sdf信息，不知道什么意思，渲染时需要)
layout(location = 4) in vec4 data3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | vec4 index_info(索引纹理偏移（单位： 像素）, 索引纹理宽高（晶格个数)
layout(location = 5) in vec4 data4; // vec4 clip_bottom_radius | (vec2 data_offset(数据纹理偏移);vec2 slope(倾斜， x方向上的剪切值, 倾斜原点的y值))
layout(location = 6) in vec4 data5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_index | vec4 box_shadow(h, v, spread, blur) | vec4 text_outer_glow(文字外法光颜色rgb+ 外法光发散范围)
layout(location = 7) in vec4 data6; // vec4 gradient_color1
layout(location = 8) in vec4 data7; // vec4 gradient_color2
layout(location = 9) in vec4 data8; // vec4 gradient_color3
layout(location = 10) in vec4 data9; // vec4 gradient_end
layout(location = 11) in vec4 data10; // vec4 border_image_offset; | vec4 u_outline(描边颜色rgb + 描边宽度u_outline.w)
layout(location = 12) in vec4 data11; // float alpha; float ty;float depth; float weight(文字粗细); 
layout(location = 13) in vec4 quad1; // 四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
layout(location = 14) in vec4 quad2; // 四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
// layout(location = 18) in float depth; // 深度
// bgcolor = 1;bordercolor = 2;borderimage = 3;bgimage=4;borderimage_repeat = 5;bgimage_repeat=6;bitmaptext = 7;fbo_rect = 8;fbo_circel=9;fbo_ellipse=10;fbo_sector=11;fbo_radius=12;fbo_svg=13;


// 输出
layout(location = 0) out vec2 vVertexPosition; // 输出位置
layout(location = 1) out vec2 vUv; // 输出位置

layout(location = 2) out vec4 vData0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 3) out vec4 vData1; // vec4 offset + scale; 
layout(location = 4) out vec4 vData2; // (vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要) ) | info (sdf信息，不知道什么意思，渲染时需要)
layout(location = 5) out vec4 vData3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | vec4 index_info(索引纹理宽高（晶格个数）， 索引纹理偏移（单位： 像素）)
layout(location = 6) out vec4 vData4; // vec4 clip_bottom_radius | (vec2 data_offset;)
layout(location = 7) out vec4 vData5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_index | vec4 box_shadow(h, v, spread, blur)
layout(location = 8) out vec4 vData6; // vec4 gradient_color1
layout(location = 9) out vec4 vData7; // vec4 gradient_color2
layout(location = 10) out vec4 vData8; // vec4 gradient_color3
layout(location = 11) out vec4 vData9; // vec4 gradient_end
layout(location = 12) out vec4 vData10; // vec4 border_image_offset | vec4 u_outline(描边颜色rgb + 描边宽度u_outline.w)
layout(location = 13) out vec4 vData11; // float alpha; float ty;
// layout(location = 14) out vec2 index_1; // float alpha; float ty;


void main() {
	int ty1 = int(data11.y); // clip = 1;uv = 4;

	vData0 = data0;
	vData1 = data1;
	vData2 = data2;
	vData3 = data3;
	vData4 = data4;
	vData5 = data5;
	vData6 = data6;
	vData7 = data7;
	vData8 = data8;
	vData9 = data9;
	vData10 = data10;
	vData11 = data11;

	if ((ty1 & 16384) != 0) {  // box_shadow
		float extend = data5.z + data5.w;
		vData1.zw = vData1.zw + extend;
		vData1.xy = vData1.xy + vec2(data5.x, data5.y) - extend;

		// 存在圆角，需要对圆角中心进行偏移， 对圆角extend范围进行扩展
		if ((ty1 & 4) != 0) {
			vData2.xy = vData2.xy + vec2(data5.x, data5.y);
			vData2.zw = vData2.zw + extend;
		}
	}


	if ((ty1 & 64) != 0 || (ty1 & 256) != 0) {  // 需要计算uv
		vUv = data0.xy + position * (data0.zw - data0.xy);
	}

	// 边框图片，
	if ((ty1 & 65536) != 0){
		// 将uv计算成像素单位
		vData0 = data0 * vec4(data5.zw, data5.zw);
	}

	if ((ty1 & 131072) != 0) {// 圆弧方案的sdf
		vec2 a_glyph_vertex = position * data1.zw; 
		vec2 slope = data4.zw;
		float x = (slope.y - a_glyph_vertex.y) * slope.x;
		vVertexPosition = vec2(a_glyph_vertex.x + x, a_glyph_vertex.y) + data1.xy;

		// vVertexPosition = position * 20.0 + data1.xy;
		// if data1.x > 10.0 && data1.x < 11.0 {
		// 	vVertexPosition = position * 20.0 + vec2(50.0, 50.0 );
		// } else {
		// 	vVertexPosition = position * 20.0 + data1.xy;
		// }
		vUv = vec2(position.x, 1.0 - position.y);
	} else {
		vVertexPosition = position * vData1.zw + vData1.xy; // 位置（布局坐标系）
	}
	
	// gl_Position = project * view * mat4(matrix0, matrix1, matrix2, matrix3) * vec4(vVertexPosition, 1.0, 1.0);
	float p_index = position.x + position.x + position.y;// 得到索引位置0或1或1或3
	vec2 p = quad1.xy * step(p_index, 0.1) // 小于0.1， 只可能个是0.0
			+ quad1.zw * step(0.9, p_index) * step(p_index, 1.1) // 0.9 < p_index <= 1.1, 只可能是1.0
			+ quad2.zw * step(1.9, p_index) * step(p_index, 2.1) // 1.9 < p_index <= 2.1， 只可能是2.0
			+ quad2.zy * step(2.9, p_index); // > 2.9, 只肯可能是3.0
	gl_Position = project * view * vec4(p, 1.0, 1.0);
	// gl_Position = project * view * vec4(vVertexPosition, 1.0, 1.0);
	// gl_Position.z = depth_offset +  depth;
	gl_Position.z = data11.z;

	if ((ty1 & 1024) != 0) { // is_not_visibility(不可见时， 返回的顶点面积为0)
		gl_Position = vec4(0.0, 0.0, 0.0, 0.0);
	}
}