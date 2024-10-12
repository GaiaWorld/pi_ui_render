
#version 450

precision highp float;

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
	vec2 sdf_tex_size; // sdf2： 索引纹理尺寸
};

layout(location = 0) in vec2 position;

layout(location = 1) in vec4 box_layout;  // 0 布局位置（offset， scale）
layout(location = 2) in vec4 sdf_uv;      // 16 sdfuv (单位: 像素)
layout(location = 3) in vec2 sdf_info;    // 32 float distance_px_range, float fill_bound;


// 输出
layout(location = 0) out vec2 vSdfUv; // 当前点的uv坐标
layout(location = 1) out vec2 VSdfInfo; // 单个像素的uv偏移

// 着色器入口
void main(void) {

	// 计算sdfuv
	float p_index = position.x + position.x + position.y;// 得到索引位置0或1或2或3（left_top = 0, left_bottom = 1, right_top = 2, right_bottom = 3）
	vec4 select = vec4(
		step(p_index, 0.1),                      // 小于0.1， 只可能是0.0
		step(0.9, p_index) * step(p_index, 1.1), // 0.9 < p_index <= 1.1, 只可能是1.0, 
		step(1.9, p_index) * step(p_index, 2.1), // 1.9 < p_index <= 2.1， 只可能是2.0, 
		step(2.9, p_index)                       // > 2.9, 只肯可能是3.0
	);
	vSdfUv = sdf_uv.xy * select.x
		+ sdf_uv.xw * select.y
		+ sdf_uv.zy * select.z
		+ sdf_uv.zw * select.w;
	vSdfUv = vSdfUv/sdf_tex_size;

	VSdfInfo = sdf_info;
	
    gl_Position = vec4((position.xy * box_layout.zw + box_layout.xy) * vec2(2.0, -2.0) + vec2(-1.0, 1.0), 0.0, 1.0); // 变换到-1.0~1.0
	// gl_Position = vec4(position.xy, 0.0, 1.0); // 变换到-1.0~1.0
}