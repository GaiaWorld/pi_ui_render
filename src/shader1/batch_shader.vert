// 可渲染背景色、图片、渐变颜色
#version 450

precision highp float;

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
	vec2 sdf_tex_size; // sdf2： 索引纹理尺寸
	// vec2 data_tex_size; // sdf2： 数据纹理尺寸
	// vec4 project_aabb; // project_aabb 调试信息
};

layout(location = 0) in vec2 position;

// 输入
layout(location = 1) in vec4 matrix0;     // 0  四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
layout(location = 2) in vec4 matrix1;     // 16 四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
layout(location = 3) in vec4 matrix2;     // 32 四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
layout(location = 4) in vec4 matrix3;     // 48 四边形 （分别为四个顶点（0， 0）、（0， 1）、（1， 1），（1， 0）在世界坐标系上的位置， 由于槽位有限， 没有传入世界矩阵， 而是算出了绝对值）
layout(location = 5) in vec4 box_layout;  // 64 布局位置（offset， scale）                                                    当为渐变颜色时，表示point0、point1
layout(location = 6) in vec4 other;       // 80 vec2 point2,float texture_index, float ty;                                   当为渐变颜色时，xy表示point2
layout(location = 7) in vec4 slopePoint;  // 96 (倾斜， x方向上的剪切值, 倾斜原点的y值, zw是参考原点)
layout(location = 8) in vec4 color0;      //112                                                                              当为渐变颜色时， 表示color0
layout(location = 9) in vec4 color;       //128 vec4 color;                                                                  当为渐变颜色时， 表示color1
layout(location = 10) in vec4 uv;         //144 vec4 uv;                                                                     当为渐变颜色时， 表示color2 
layout(location = 11) in vec4 strokeColor; //160 vec4 strokeColor;                                                       
layout(location = 12) in vec4 sdf_depth;  //176 float distance_px_range, float fill_bound, float stroke_bound, float zdepth;
layout(location = 13) in vec4 sdfUv;      //192 sdfUv;																	     当为渐变颜色时， 表示sdfUv0、sdfUv1
layout(location = 14) in vec2 sdfUv2;     //208 ;                                                                            当为渐变颜色时， 表示sdfUv2, 否则为debu信息


// 输出
layout(location = 0) out vec4 vColor; // color
layout(location = 1) out vec4 vStrokeColor; // strokeColor
layout(location = 2) out vec4 vSdf; // sdf_depth
layout(location = 3) out vec4 vTextureInfo; // uv + texture_index + strokeFactor
layout(location = 4) out vec2 vSdfUv; // sdfUv

void main() {
	int ty1 = int(other.w + 0.1); 

	if ((ty1 & 3072) != 0) { // 1024 + 2048 is_not_visibility(不可见时， 返回的顶点面积为0)
		gl_Position = vec4(0.0, 0.0, 0.0, 0.0);
		return;
	}

	vec2 p = position * box_layout.zw + box_layout.xy;

	// 计算uv
	float p_index = position.x + position.x + position.y;// 得到索引位置0或1或2或3（left_top = 0, left_bottom = 1, right_top = 2, right_bottom = 3）
	vec4 select = vec4(
		step(p_index, 0.1),                      // 小于0.1， 只可能是0.0
		step(0.9, p_index) * step(p_index, 1.1), // 0.9 < p_index <= 1.1, 只可能是1.0, 
		step(1.9, p_index) * step(p_index, 2.1), // 1.9 < p_index <= 2.1， 只可能是2.0, 
		step(2.9, p_index)                       // > 2.9, 只肯可能是3.0
	);
	vec4 uv_ = vec4(uv.xy, sdfUv.xy) * select.x
			 + vec4(uv.xw, sdfUv.xw) * select.y
			 + vec4(uv.zy, sdfUv.zy) * select.z
			 + vec4(uv.zw, sdfUv.zw) * select.w;
	float textureIndex = other.z;

	// 处理预乘因子
	float not_premultiply = 1.0; // 1.0表示非预乘模式， 0.001表示预乘模式
	if ((ty1 & 8) != 0) { // 预乘模式
		not_premultiply = 0.001;
	}

	// 处理渐变
	if ((ty1 & 4096) != 0) {
		vColor = color0 * select.x                  // 此时color0              表示渐变color0
			   + color    * select.y                // 此时color               表示渐变color1
			   + uv * select.z                      // 渐变只能渲染三角形，第三个点不使用， 颜色随意
			   + uv * select.w;                     // 此时uv                  表示渐变color2
		// 为渐变颜色时， 只能渲染三角形， 使用如下方式计算顶点（而非默认）
		p  = box_layout.xy * select.x               // 此时box_layout.xy       表示渐变point0
		   + box_layout.zw    * select.y            // 此时box_layout.zw       表示渐变point1
		   + other.xy * select.z                    // 渐变只能渲染三角形，第三个点不使用， 让其等于第四个点， 使得围成的三角形面积为0
		   + other.xy * select.w;                   // 此时othert.xy           表示渐变point3
		vSdfUv =  sdfUv.xy  * select.x              // 此时sdfUv.xy            表示渐变sdfUv0
				+ sdfUv.zw  * select.y              // 此时sdfUv.zw            表示渐变sdfUv1
				+ sdfUv2.xy * select.z              // 渐变只能渲染三角形，第三个点不使用， sdfuv随意
				+ sdfUv2.xy * select.w;             // 此时sdfUv2.xy           表示渐变sdfUv1
		textureIndex = 50.0; // 超出12， 不会采样纹理
	} else {
		// 输出
		vColor = color;
		vSdfUv = uv_.zw;
	}

	float strokeFactor = 1.0;
	if ((ty1 & 256) != 0) {
		// 存在描边时， 描边因子为0.0
		strokeFactor = 0.0;
	}

	if ((ty1 & 16) != 0) { // R8纹理
		strokeFactor = 2.0;
	}
	

	vSdfUv = vSdfUv/sdf_tex_size;
	vStrokeColor = strokeColor;
	vSdf = vec4(sdf_depth.xyz, not_premultiply);
	vTextureInfo = vec4(uv_.xy, textureIndex, strokeFactor);

	// 计算顶点	
	mat4 m = mat4(matrix0, matrix1, matrix2, matrix3);
	if ((ty1 & 2) == 0) { // 忽略相机
		vSdf.x = vSdf.x * length(matrix0.xyz); // distance_px_range乘以x轴上的缩放
		gl_Position = project * view * m * vec4(p, 1.0, 1.0);
	} else {
		gl_Position = m * vec4(p, 1.0, 1.0);
	}
	// gl_Position = vec4(p, 1.0, 1.0);
	gl_Position.z = sdf_depth.w; // sdf_depth.w 表示深度
}