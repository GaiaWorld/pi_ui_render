// // 背景颜色
// layout(set=2,binding=0) uniform BackgroundColor {
// 	mat4 world; // 世界矩阵

// 	// 几何信息
// 	vec4 rect; // 矩形范围
// 	vec4 top_radius; // top_left_x, top_left_y, top_right_y, top_right_x
// 	vec4 bottom_radius; // bottom_right_x, bottom_right_y, bottom_left_y, bottom_left_x

// 	vec4 background_color; // 背景颜色
// 	float ty; // 渲染类型
// };

// // 边框颜色
// layout(set=2,binding=0) uniform BorderColor {
// 	mat4 world; // 世界矩阵

// 	// 几何信息
// 	vec4 border_rect; // 矩形范围
// 	vec4 border_top_radius; // top_left_x, top_left_y, top_right_y, top_right_x
// 	vec4 border_bottom_radius; // bottom_right_x, bottom_right_y, bottom_left_y, bottom_left_x
// 	vec4 border_width; // 边框宽度（上右下左）

// 	vec4 border_color; // 边框颜色
// 	float ty; // 渲染类型
// };

// // 渐变颜色
// layout(set=2,binding=2) uniform GradientColor {
// 	mat4 world; // 世界矩阵

// 	// 几何信息
// 	vec4 rect; // 矩形范围
// 	vec4 top_radius; // top_left_x, top_left_y, top_right_y, top_right_x
// 	vec4 bottom_radius; // bottom_right_x, bottom_right_y, bottom_left_y, bottom_left_x

// 	mat4 gradient_colors; // 渐变颜色
// 	vec4 gradient_position; // 显示颜色
// 	float ty; // 渲染类型
// };

// 背景图片
layout(set=2,binding=3) uniform BackgroundImage {
	mat4 world; // 世界矩阵

	// 几何信息
	vec4 rect; // 矩形范围
	vec4 top_radius; // top_left_x, top_left_y, top_right_y, top_right_x
	vec4 bottom_radius; // bottom_right_x, bottom_right_y, bottom_left_y, bottom_left_x

	vec4 background_uv; // uv
	float ty; // 渲染类型
};

// // 位图文字
// layout(set=2,binding=4) uniform BitmapText {
// 	mat4 world; // 世界矩阵

// 	vec4 stroke_color; // 边框颜色
// 	vec4 color; // 颜色
// 	vec4 text_uv; // uv
// 	vec2 text_texture_size; // 图片纹理大小

// 	float ty; // 渲染类型
// };

// // 用圆形裁剪fbo
// layout(set=2,binding=5) uniform FboCircel {
// 	mat4 world; // 世界矩阵
// 	// vec4 fbo_uv; // uv
// 	// float fbo_opacity; // 半透明

// 	// 几何信息
// 	vec2 clip_circel_center; // 矩形范围
// 	float clip_circel_radius; // 半径

// 	float ty; // 渲染类型
	
// };

// // 用椭圆形裁剪fbo
// layout(set=2,binding=6) uniform FboEllipse {
// 	mat4 world; // 世界矩阵
// 	// vec4 fbo_uv; // uv
// 	// float fbo_opacity; // 半透明

// 	// 几何信息
// 	vec2 clip_ellipse_center; // 矩形范围
// 	vec2 clip_ellipse_ab; // 长轴和短轴

// 	float ty; // 渲染类型
	
// };

// // 用扇形形裁剪fbo
// layout(set=2,binding=7) uniform FboSector {
// 	mat4 world; // 世界矩阵

// 	// vec4 fbo_uv; // uv
// 	// float fbo_opacity; // 半透明

// 	// 几何信息
// 	vec2 clip_sector_center; // 扇形中心点
// 	vec2 clip_sector_rotate; // (旋转角度+扇形弧度/2)的sin， cos值
// 	vec2 clip_sector_radian; // (扇形弧度/2)的sin， cos值
// 	float clip_sector_radius; // 扇形半径

// 	float ty; // 渲染类型
	
// };

// // 用矩形形裁剪fbo
// layout(set=2,binding=8) uniform FboRect {
// 	mat4 world; // 世界矩阵

// 	// vec4 fbo_uv; // uv
// 	// float fbo_opacity; // 半透明

// 	// 几何信息
// 	vec2 clip_rect_center; // 矩形形中心点
// 	vec2 clip_rect_extent; // 矩形的半宽半高

// 	float ty; // 渲染类型
	
// };

// // 用圆角形裁剪fbo
// layout(set=2,binding=8) uniform FboRect {
// 	mat4 world; // 世界矩阵

// 	// vec4 fbo_uv; // uv
// 	// float fbo_opacity; // 半透明

// 	// 几何信息
// 	vec4 clip_rect; // 矩形范围
// 	vec4 clip_top_radius; // top_left_x, top_left_y, top_right_y, top_right_x
// 	vec4 clip_bottom_radius; // bottom_right_x, bottom_right_y, bottom_left_y, bottom_left_x

// 	float ty; // 渲染类型
	
// };
