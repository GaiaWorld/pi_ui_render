
// 输入
layout(location = 0) in vec2 vVertexPosition; // 顶点到裁剪中心点的距离
layout(location = 1) in vec2 vUv; // uv
layout(location = 2) in vec4 vData0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 3) in vec4 vData1; // vec4 offset + scale; 
layout(location = 4) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
layout(location = 5) in vec4 vData3; // vec4 clip_min_yop_max_xadius | (vec2 clip_sector_radian; float clip_sector_radius) | 
layout(location = 6) in vec4 vData4; // vec4 clip_max_yottom_radius
layout(location = 7) in vec4 vData5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_layer;float 1.0表示需要填充; vec2 uv_offset | vec4 box_shadow(h, v, spread, blur) 
layout(location = 8) in vec4 vData6; // vec4 gradient_color1 | vec4 stroke_color |  vec2 position_step; vec2 position_space
layout(location = 9) in vec4 vData7; // vec4 gradient_color2
layout(location = 10) in vec4 vData8; // vec4 gradient_color3
layout(location = 11) in vec4 vData9; // vec4 gradient_end
layout(location = 12) in vec4 vData10; // vec4 border_image_uv_offset | vec4 u_outline(描边颜色rgb + 描边宽度u_outline.w); 
layout(location = 13) in vec4 vData11; // float alpha; float ty;float depth;
layout(location = 14) in vec2 vData12; // float alpha; float ty;float depth;
layout(location = 15) in vec3 vData13; // 阴影偏移和模糊等级

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
	vec2 index_tex_size; // sdf2： 索引纹理尺寸
	vec2 data_tex_size; // sdf2： 数据纹理尺寸
};


// 纹理， 最多一次传16张纹理
layout(set=1,binding=0) uniform texture2D tex2d0;
layout(set=1,binding=2) uniform texture2D tex2d1;
layout(set=1,binding=4) uniform texture2D tex2d2;
layout(set=1,binding=6) uniform texture2D tex2d3;
layout(set=1,binding=8) uniform texture2D tex2d4;
layout(set=1,binding=10) uniform texture2D tex2d5;
layout(set=1,binding=12) uniform texture2D tex2d6;
layout(set=1,binding=14) uniform texture2D tex2d7;
layout(set=1,binding=16) uniform texture2D tex2d8;
layout(set=1,binding=18) uniform texture2D tex2d9;
layout(set=1,binding=20) uniform texture2D tex2d10;
layout(set=1,binding=22) uniform texture2D tex2d11;
layout(set=1,binding=24) uniform texture2D tex2d12;
// layout(set=1,binding=26) uniform texture2D tex2d13;
// layout(set=1,binding=28) uniform texture2D tex2d14;
// layout(set=1,binding=30) uniform texture2D tex2d15;

// 采样器
layout(set=1,binding=1) uniform sampler samp0;
layout(set=1,binding=3) uniform sampler samp1;
layout(set=1,binding=5) uniform sampler samp2;
layout(set=1,binding=7) uniform sampler samp3;
layout(set=1,binding=9) uniform sampler samp4;
layout(set=1,binding=11) uniform sampler samp5;
layout(set=1,binding=13) uniform sampler samp6;
layout(set=1,binding=15) uniform sampler samp7;
layout(set=1,binding=17) uniform sampler samp8;
layout(set=1,binding=19) uniform sampler samp9;
layout(set=1,binding=21) uniform sampler samp10;
layout(set=1,binding=23) uniform sampler samp11;
layout(set=1,binding=25) uniform sampler samp12;
// layout(set=1,binding=27) uniform sampler samp13;
// layout(set=1,binding=29) uniform sampler samp14;
// layout(set=1,binding=31) uniform sampler samp15;

layout(set=2,binding=0) uniform texture2D u_index_tex;
layout(set=2,binding=1) uniform texture2D u_data_tex;
layout(set=2,binding=2) uniform sampler tex_samp;
layout(set=2,binding=3) uniform texture2D u_shadow_tex;
layout(set=2,binding=4) uniform sampler shadow_samp;

// 输出颜色
layout(location=0) out vec4 o_Target;





// 从webrender-15版本中拷贝过来的shader
// An approximation of the error function, which is related to the integral of the Gaussian
// function:
//
//     "erf"(x) = 2/sqrt(pi) int_0^x e^(-t^2) dt
//              ~~ 1 - 1 / (1 + a_1 x + a_2 x^2 + a_3 x^3 + a_4 x^4)^4
//
// where:
//
//     a_1 = 0.278393, a_2 = 0.230389, a_3 = 0.000972, a_4 = 0.078108
//
// This approximation is accurate to '5 xx 10^-4', more than accurate enough for our purposes.
//
// See: https://en.wikipedia.org/wiki/Error_function#Approximation_with_elementary_functions
float erf(float x) {
	bool negative = x < 0.0;
	if (negative)
		x = -x;
	float x2 = x * x;
	float x3 = x2 * x;
	float x4 = x2 * x2;
	float denom = 1.0 + 0.278393 * x + 0.230389 * x2 + 0.000972 * x3 + 0.078108 * x4;
	float result = 1.0 - 1.0 / (denom * denom * denom * denom);
	return negative ? -result : result;
}

// A useful helper for calculating integrals of the Gaussian function via the error function:
//
//      "erf"_sigma(x) = 2 int 1/sqrt(2 pi sigma^2) e^(-x^2/(2 sigma^2)) dx
//                     = "erf"(x/(sigma sqrt(2)))
float erfSigma(float x, float sigma) {
	return erf(x / (sigma * 1.4142135623730951));
}

// Returns the blurred color value from the box itself (not counting any rounded corners). 'p_0' is
// the vector distance to the top left corner of the box; 'p_1' is the vector distance to its
// bottom right corner.
//
//      "colorFromRect"_sigma(p_0, p_1)
//          = int_{p_{0_y}}^{p_{1_y}} int_{p_{1_x}}^{p_{0_x}} G_sigma(y) G_sigma(x) dx dy
//          = 1/4 ("erf"_sigma(p_{1_x}) - "erf"_sigma(p_{0_x}))
//              ("erf"_sigma(p_{1_y}) - "erf"_sigma(p_{0_y}))
float colorFromRect(vec2 p0, vec2 p1, float sigma) {
	return (erfSigma(p1.x, sigma) - erfSigma(p0.x, sigma)) *
		(erfSigma(p1.y, sigma) - erfSigma(p0.y, sigma)) / 4.0;
}

// The blurred color value for the point at 'pos' with the top left corner of the box at
// 'p_{0_"rect"}' and the bottom right corner of the box at 'p_{1_"rect"}'.
float getShadowAlpha(vec2 pos, vec2 ptMin, vec2 ptMax, float sigma) {
	// Compute the vector distances 'p_0' and 'p_1'.
	vec2 dMin = pos - ptMin, dMax = pos - ptMax;

	// Compute the basic color '"colorFromRect"_sigma(p_0, p_1)'. This is all we have to do if
	// the box is unrounded.
	return colorFromRect(dMin, dMax, sigma);
}




// 根据 d, 抗锯齿, 返回 alpha值
float antialiase(float d) 
{
	float anti = 1.0 * fwidth(d);
	
	// smoothstep(-a, a, d) 意思是 根据 d-值 将 [-a, a] 平滑到 [0, 1] 中
	// d < -a, 全内部, 得到0, 这时期望 alpha = 1.0
	// d > a, 全外部, 得到1, 这时期望 alpha = 0.0
	
	return 1.0 - smoothstep(-anti, anti, d);
}

// 可以看成 fs 中 计算 统一缩放系数 的 倒数
float computeAARange(vec2 position) {
	// position 变化率，放大2倍，w 0.5
	vec2 w = fwidth(position);
	
	// sqrt(2)/length(w) = inversesqrt(0.5 * dot(w, w))
	return inversesqrt(0.5 * dot(w, w));
}

// The aa_range is already stored as a reciprocal with uniform scale
// so just multiply it, then use that for AA.
float distanceAA(float recip_scale, float signed_distance) {
	
	float d = recip_scale * signed_distance;
	
	// webrender 原始 公式，太严格，导致 抗锯齿 不大 成功？
	// d 在 [-0.5, 0.5] 之间，0.5 - d 在 [0, 1]
	// return clamp(0.5 - d, 0.0, 1.0);
	
	// d 在 [-1.0, 1.0] 之间，0.5 * (1.0 + d) 在 [0, 1]
	return clamp(0.5 * (1.0 - d), 0.0, 1.0);
}

// https://iquilezles.org/articles/ellipsoids/
float sdfEllipseSimple(vec2 p, vec2 center, vec2 ab)
{
	p -= center;

	float k1 = length(p / ab);
	float k2 = length(p/(ab * ab));
	return (k1 - 1.0) * k1 / k2;
}

// 返回 coord 到 矩形 最短距离, 负值表示 在里面, 正值表示在外面
float sdfRect(vec2 xy, vec2 wh)
{
	vec2 d = abs(xy) - wh;
	return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float crossPt(vec2 v1, vec2 v2) {
	return -(v1.x * v2.y - v1.y * v2.x);
}

// p0, p1, p2 是否 逆时针
bool isCcw(vec2 p0, vec2 p1, vec2 p2) {
	vec2 v1 = p1 - p0;
	vec2 v2 = p2 - p0;
	float r = crossPt(v1, v2);
	
	return r > 0.0;
}

bool isLeftTop(vec2 pt, vec2 wh, vec2 center) {

	vec2 pt0 = vec2(-wh.x, center.y);
	vec2 pt1 = vec2(center.x, -wh.y);

	return isCcw(pt, pt0, pt1);
}

bool isTopRight(vec2 pt, vec2 wh, vec2 center) {
	vec2 pt0 = vec2(center.x, -wh.y);
	vec2 pt1 = vec2(wh.x, center.y);

	return isCcw(pt, pt0, pt1);
}

bool isRightBottom(vec2 pt, vec2 wh, vec2 center) {
	vec2 pt0 = vec2(wh.x, center.y);
	vec2 pt1 = vec2(center.x, wh.y);

	return isCcw(pt, pt0, pt1);
}

bool isBottomLeft(vec2 pt, vec2 wh, vec2 center) {
	vec2 pt0 = vec2(center.x, wh.y);
	vec2 pt1 = vec2(-wh.x, center.y);

	return isCcw(pt, pt0, pt1);
}

// 扇形 sdf，负数在里面，正数在外面
// pt 待求点
// sc 扇形 边缘处 距离 y轴的 夹角 sin, cos
// r 半径
// 参考 https://zhuanlan.zhihu.com/p/427587359
float sdfPie(vec2 p, vec2 sc, float r)
{
	p.x = abs(p.x);

	float d_circle = length(p) - r;
	
	// pie 为 0 或者 180 需要额外处理
	if (sc.x < 0.0001) {
		return abs(sc.y + 1.0) < 0.0001 ? d_circle : sc.y;
	}

	float d_border = length(p - sc * clamp(dot(p, sc), 0.0, r)) * sign(sc.y * p.x - sc.x * p.y);
	return max(d_circle, d_border);
}

// 返回 coord 到 圆的 最短距离, 负值表示 在里面, 正值表示在外面
float sdfCircle(vec2 xy, float r) {
	return length(xy) - r;
}

float sdfRoundRect(vec2 pt, vec2 extent, vec2 offset1, vec2 offset2, vec2 offset3, vec2 offset4) {
	float d_rect = sdfRect(pt, extent);
	
	vec2 center = vec2(-extent.x + offset1.x, -extent.y + offset1.y); 
	if (isLeftTop(pt, extent, center)) {
		float d_lt = sdfEllipseSimple(pt, center, abs(offset1));
		return max(d_rect, d_lt);
	}
	
	center = vec2(extent.x + offset2.x, -extent.y + offset2.y); 
	if (isTopRight(pt, extent, center)) {
		float d_tr = sdfEllipseSimple(pt, center, abs(offset2));
		return max(d_rect, d_tr);
	}
	
	center = vec2(extent.x + offset3.x, extent.y + offset3.y); 
	if (isRightBottom(pt, extent, center)) {
		float d_rb = sdfEllipseSimple(pt, center, abs(offset3));
		return max(d_rect, d_rb);
	}

	center = vec2(-extent.x + offset4.x, extent.y + offset4.y); 
	if (isBottomLeft(pt, extent, center)) {
		float d_bl = sdfEllipseSimple(pt, center, abs(offset4));
		return max(d_rect, d_bl);
	}

	return d_rect;
}

float radius_d(vec2 pos, vec2 extent) {
	// 圆角半径
	vec4 top = vData3;
	vec4 bottom = vData4;
	// vec4 top = vec4(10.0, 10.0, 10.0, 10.0);
	// vec4 bottom = vec4(10.0, 10.0, 10.0, 10.0);
	// 左上角
	vec2 c1 = vec2(max(0.01, top.y), max(0.01, top.x));
	// 右上角
	vec2 c2 = vec2(-max(0.01, top.z), max(0.01, top.w));
	// 右下角
	vec2 c3 = vec2(-max(0.01, bottom.y), -max(0.01, bottom.x));
	// 左下角
	vec2 c4 = vec2(max(0.01, bottom.z), -max(0.01, bottom.w));
	
	return sdfRoundRect(pos, extent, c1, c2, c3, c4);
}

float calc_radius_alpha() {
	// 到中心的距离
	vec2 pos = vVertexPosition - vData2.xy;
	float d = radius_d(pos, vData2.zw);
	float aaRange = computeAARange(vVertexPosition);
	return distanceAA(aaRange, d);
}



// 计算边框
float calc_border_alpha() {
	vec2 ab = vData1.zw/2.0; // center/extend
	// 到中心的距离
	vec2 pos = vVertexPosition - ab; // 边框取geo中心点为原点
	float d_big = radius_d(pos, ab);
	
	// ============ 内 圆角矩形
	// vec4 top = vData3;
	// vec4 bottom = vData4;
	// vec4 border_width = vData5;// 上-右-下-左
	// vec2 extent = vData2.zw;
	vec2 lt_small = vec2(max(0.01, vData3.y - vData5.w), max(0.01, vData3.x - vData5.x));
	vec2 rt_small = vec2(-max(0.01, vData3.z - vData5.y), max(0.01, vData3.w - vData5.x));
	vec2 rb_small = vec2(-max(0.01, vData4.y - vData5.y), -max(0.01, vData4.x - vData5.z));
	vec2 lb_small = vec2(max(0.01, vData4.z - vData5.w), -max(0.01, vData4.w - vData5.z));

	vec2 pos_small = pos - 0.5 * vec2(vData5.w - vData5.y, vData5.x - vData5.z);
	vec2 extent_small = ab - 0.5 * vec2(vData5.w + vData5.y, vData5.x + vData5.z);
	float d_small = sdfRoundRect(pos_small, extent_small, lt_small, rt_small, rb_small, lb_small);

	// ========== 外 - 内
	float d = max(d_big, -d_small);
	float aaRange = computeAARange(vVertexPosition);

	return distanceAA(aaRange, d);
}

// 计算
float calc_sector_alpha() 
{

	// layout(location = 7) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
	// layout(location = 8) in vec4 vData3; // vec4 clip_min_yop_max_xadius | (vec2 clip_sector_radian; float clip_sector_radius) | 
	
	// (旋转角度+扇形弧度/2)的sin， cos值
	vec2 axisSC = vData2.zw;
	// (扇形弧度/2)的sin， cos值
	vec2 sc =  vData3.xy;
	// 扇形的半径
	float r = vData3.z;

	vec2 pos = vVertexPosition - vData2.xy;

	// 逆过来乘，将 扇形 乘回 到 对称轴 为 y轴 处
	// 调整到 PI / 2 = 1.570796325
	// cos(a- pi/2) = sin(a), sin(a - pi/2) = -cos(a)
	// 要乘以 旋转矩阵 的 逆
	pos = vec2(axisSC.x * pos.x - axisSC.y * pos.y, axisSC.y * pos.x + axisSC.x * pos.y);
	float d = sdfPie(pos, sc, r);
	
	float aaRange = computeAARange(pos);
	return distanceAA(aaRange, d);
}

// 计算alpha
float calc_rect_alpha() {
	// 半宽半高
	vec2 uExtent = vData2.zw; 
	vec2 pos = vVertexPosition - vData2.xy;
	float d = sdfRect(pos, uExtent);

	float aaRange = computeAARange(pos);
	return distanceAA(aaRange, d);
}

// 计算alpha
float calc_ellipse_alpha() {
	float d = sdfEllipseSimple(vVertexPosition, vData2.xy, vData2.zw);
	
	// return antialiase(d);
	float aaRange = computeAARange(vVertexPosition);
	return distanceAA(aaRange, d);
}

// 计算alpha
float calc_circel_alpha() {
	vec2 pos = vVertexPosition - vData2.xy;
	float d = sdfCircle(pos, vData2.z);
	
	float aaRange = computeAARange(vVertexPosition);
	return distanceAA(aaRange, d);
}


/******************************************************sdf2 **********************************************************/

#define GLYPHY_INFINITY 1e6
#define GLYPHY_EPSILON  1e-4
#define GLYPHY_MAX_D 0.5
#define GLYPHY_MAX_NUM_ENDPOINTS 20

// 索引信息  
struct glyphy_index_t {
	
	// 编码信息
	int encode;

	// 端点的数量 
	// 0 代表 一直读取到 像素为 (0, 0, 0, 0) 的 数据为止
	int num_endpoints;

	// 在数据纹理的偏移，单位：像素
	int offset;

	// 晶格中心点的sdf
	float sdf;
};

// 从 p0 到 p1 的 圆弧
// 2 * d 为 tan(弧心角)
// d = 0 代表 这是 一条线段 
struct glyphy_arc_t {
	vec2  p0;
	vec2  p1;
	float d;
};

// 圆弧 端点 
struct glyphy_arc_endpoint_t {
	// 圆弧 第二个 端点 
	vec2  p;
	
	/** 
	 * d = 0 表示 这是一个 line 
	 * d = Infinity 表示 该点是 move_to 语义，通过 glyphy_isinf() 判断 
	 */
	float d;
};

struct line_t {
	float distance;

	float angle;
};

// 修复glsl bug 的 取余
// 某些显卡, 当b为uniform, 且 a % b 为 0 时候，会返回 b
// 128 , 256
vec2 div_mod(float a, float b) {
	float d = floor(a / b);
	float m = mod(a, b);
	if (m == b) {
		return vec2(d + 1.0, 0.0);
	}
	return vec2(d, m);
}

// 超过 最大值的 一半，就是 无穷 
bool glyphy_isinf(const float v)
{
	return abs (v) >= GLYPHY_INFINITY * 0.5;
}

// 小于 最小值 的 两倍 就是 0 
bool glyphy_iszero(const float v)
{
	return abs (v) <= GLYPHY_EPSILON * 2.0;
}

// v 的 垂直向量 
vec2 glyphy_ortho(const vec2 v)
{
	return vec2 (-v.y, v.x);
}

// [0, 1] 浮点 --> byte 
int glyphy_float_to_byte(const float v)
{
	return int (v * (256.0 - GLYPHY_EPSILON));
}

// [0, 1] 浮点 --> byte 
ivec4 glyphy_vec4_to_bytes(const vec4 v)
{
	return ivec4 (v * (256.0 - GLYPHY_EPSILON));
}

// 浮点编码，变成两个 整数 
ivec2 glyphy_float_to_two_nimbles(const float v)
{
	int f = glyphy_float_to_byte (v);


	vec2 r = div_mod(float(f), 16.0);

	return ivec2 (f / 16, int(r.y));
}

// returns tan (2 * atan (d))
float glyphy_tan2atan(const float d)
{
	return 2.0 * d / (1.0 - d * d);
}

// 取 arc 的 圆心 
vec2 glyphy_arc_center(const glyphy_arc_t a)
{
	return mix (a.p0, a.p1, 0.5) +
		glyphy_ortho(a.p1 - a.p0) / (2.0 * glyphy_tan2atan(a.d));
}

float glyphy_arc_wedge_signed_dist_shallow(const glyphy_arc_t a, const vec2 p)
{
	vec2 v = normalize (a.p1 - a.p0);
	float line_d = dot (p - a.p0, glyphy_ortho (v));
	if (a.d == 0.0) {
		return line_d;
	}
	
	float d0 = dot ((p - a.p0), v);
	if (d0 < 0.0) {
		return sign (line_d) * distance (p, a.p0);
	}

	float d1 = dot ((a.p1 - p), v);
	if (d1 < 0.0) {
		return sign (line_d) * distance (p, a.p1);
	}
	
	float r = 2.0 * a.d * (d0 * d1) / (d0 + d1);
	if (r * line_d > 0.0) {
		return sign (line_d) * min (abs (line_d + r), min (distance (p, a.p0), distance (p, a.p1)));
	}

	return line_d + r;
}

float glyphy_arc_wedge_signed_dist(const glyphy_arc_t a, const vec2 p)
{
	if (abs (a.d) <= 0.03) {
		return glyphy_arc_wedge_signed_dist_shallow(a, p);
	}
	
	vec2 c = glyphy_arc_center (a);
	return sign (a.d) * (distance (a.p0, c) - distance (p, c));
}

// 解码 arc 端点 
glyphy_arc_endpoint_t glyphy_arc_endpoint_decode(const vec4 v, const vec2 nominal_size)
{
	vec2 p = (vec2 (glyphy_float_to_two_nimbles (v.a)) + v.gb) / 16.0;
	float d = v.r;
	if (d == 0.0) {
		d = GLYPHY_INFINITY;
	} else {
		d = float(glyphy_float_to_byte(d) - 128) * GLYPHY_MAX_D / 127.0;
	}

	p *= nominal_size;
	return glyphy_arc_endpoint_t (p, d);
}

// 判断是否 尖角内 
bool glyphy_arc_wedge_contains(const glyphy_arc_t a, const vec2 p)
{
	float d2 = glyphy_tan2atan (a.d);

	return dot (p - a.p0, (a.p1 - a.p0) * mat2(1,  d2, -d2, 1)) >= 0.0 &&
		dot (p - a.p1, (a.p1 - a.p0) * mat2(1, -d2,  d2, 1)) <= 0.0;
}

// 点 到 圆弧 的 距离
float glyphy_arc_extended_dist(const glyphy_arc_t a, const vec2 p)
{
	// Note: this doesn't handle points inside the wedge.
	vec2 m = mix(a.p0, a.p1, 0.5);

	float d2 = glyphy_tan2atan(a.d);

	if (dot(p - m, a.p1 - m) < 0.0) {
		return dot(p - a.p0, normalize((a.p1 - a.p0) * mat2(+d2, -1, +1, +d2)));
	} else {
		return dot(p - a.p1, normalize((a.p1 - a.p0) * mat2(-d2, -1, +1, -d2)));
	}
}

line_t decode_line(const vec4 v, const vec2 nominal_size) {
	ivec4 iv = glyphy_vec4_to_bytes(v);

	line_t l;

	int ua = iv.b * 256 + iv.a;
	int ia = ua - 0x8000;
	l.angle = -float(ia) / float(0x7FFF) * 3.14159265358979;

	int ud = (iv.r - 128) * 256 + iv.g;
	int id = ud - 0x4000;
	float d = float(id) / float(0x1FFF);
	
	float scale = max(nominal_size.x, nominal_size.y);
	
	l.distance = d * scale;
	return l;
}

// 解码 索引纹理 
glyphy_index_t decode_glyphy_index(vec4 v, const vec2 nominal_size)
{	
	vec4 u_info = vData2;

	ivec4 c = glyphy_vec4_to_bytes(v);

	int value = c.r + 256 * c.g;

	int v2 = value;

	// 注：移动端，int 范围有可能是 [-2^15, 2^15)
	if (value < 0) {
		v2 += 32766;
		v2 += 2;
	}

	int num_endpoints = v2 / 16384;
	int sdf_and_offset_index = v2 - 16384 * num_endpoints;
	if (value < 0) {
		num_endpoints += 2; // 因为 32768 / 16384 = 2
	}

	// Amd 显卡 Bug：整除时，余数不为0
	if (sdf_and_offset_index == 16384) {
		sdf_and_offset_index = 0;
		num_endpoints += 1;
	}

	int sdf_index = sdf_and_offset_index / int(u_info.x);
	int offset = sdf_and_offset_index - sdf_index * int(u_info.x);
	
	// Amd 显卡 Bug：整除时，余数不为0；
	if (offset == int(u_info.x)) {
		offset = 0;
		sdf_index += 1;
	}
	
	float sdf = 0.0;

	if (sdf_index == 0) {
		// 用 0 表示 完全 在内 的 晶格！
		sdf = -GLYPHY_INFINITY;
	} else if (sdf_index == 1) {
		// 用 1 表示 完全 在外 的 晶格！
		sdf = GLYPHY_INFINITY;
	} else {
		// 比实际的 sdf 范围多出 2
		sdf_index -= 2;
		sdf = float(sdf_index) * u_info.z + u_info.y;
	}

	glyphy_index_t index;

	index.sdf = sdf;
	index.encode = v2;
	index.offset = offset;
	index.num_endpoints = num_endpoints;
	
	return index;
}

// 取 索引 uv
vec2 get_index_uv(vec2 p)
{
	vec2 offset = vData3.xy; // 索引纹理偏移（单位： 像素）
	return (p + offset) / index_tex_size;
}



glyphy_index_t get_glyphy_index(vec2 p, vec2 nominal_size) {
	
	vec2 index_uv = get_index_uv(p);
	
	vec4 c = texture(sampler2D(u_index_tex, tex_samp), index_uv).rgba;
	// vec4 c = vec4(0.0, 0.0, 0.0, 0.0);
	return decode_glyphy_index(c, nominal_size);
}


// 重点 计算 sdf 
float glyphy_sdf(vec2 p, vec2 nominal_size) {
	vec2 u_data_offset = vData4.xy;

	glyphy_index_t index_info = get_glyphy_index(p, nominal_size);
		
	// if (index_info.sdf >= GLYPHY_INFINITY - GLYPHY_EPSILON) {
	// 	// 全外面
	// 	return GLYPHY_INFINITY;
	// } else if (index_info.sdf <= -GLYPHY_INFINITY + GLYPHY_EPSILON) {
	// 	// 全里面
	// 	return -GLYPHY_INFINITY;
	// }

	// 处理相交的晶格

	float side = index_info.sdf < 0.0 ? -1.0 : 1.0;
	float min_dist = GLYPHY_INFINITY;
	
	// 注：N卡，和 高通 的 显卡，纹理 需要 加 0.5像素
	float offset = 0.5 + float(index_info.offset);
	// float a = offset / u_info.x;
	float x = mod(offset, 8.0) + u_data_offset.x;
	float y = floor(offset / 8.0) + u_data_offset.y;


	vec4 rgba = texture(sampler2D(u_data_tex, tex_samp), vec2(x, y) / data_tex_size);
	// vec4 rgba =vec4(0.0, 0.0, 0.0, 0.0);
	

	glyphy_arc_t closest_arc;
	glyphy_arc_endpoint_t endpoint = glyphy_arc_endpoint_decode(rgba, nominal_size);

	
	vec2 pp = endpoint.p;
	// 1个像素 最多 32次 采样 
	for(int i = 1; i < GLYPHY_MAX_NUM_ENDPOINTS; i++) {
		// vec4 rgba = vec4(0.0);
		float offset = 0.5 + float(index_info.offset + i);
		float x = mod(offset, 8.0) + u_data_offset.x;
		float y = floor(offset / 8.0) + u_data_offset.y;
		
		vec4 rgba = texture(sampler2D(u_data_tex, tex_samp), vec2(x, y) / data_tex_size);
		// vec4 rgba =vec4(0.0, 0.0, 0.0, 0.0);

		if(index_info.num_endpoints == 0) {
			if (rgba == vec4(0.0)) {
				break;
			}
		} else if (i < index_info.num_endpoints) {
		} else {
			break;
		}
		
		endpoint = glyphy_arc_endpoint_decode(rgba, nominal_size);
		
		glyphy_arc_t a = glyphy_arc_t(pp, endpoint.p, endpoint.d);

		// 无穷的 d 代表 Move 语义 
		if(glyphy_isinf(a.d)) {
			pp = endpoint.p;
			continue;
		}

		if(glyphy_arc_wedge_contains(a, p)) { // 处理 尖角 
			float sdist = glyphy_arc_wedge_signed_dist(a, p);
			float udist = abs(sdist) * (1.0 - GLYPHY_EPSILON);

			if(udist <= min_dist) {
				min_dist = udist;
				side = sdist <= 0.0 ? -1.0 : +1.0;
			}
		} else {
			float udist = min(distance(p, a.p0), distance(p, a.p1));

			if(udist < min_dist - GLYPHY_EPSILON) {
				side = 0.0;
				min_dist = udist;
				closest_arc = a;
			} else if(side == 0.0 && udist - min_dist <= GLYPHY_EPSILON) {
				float old_ext_dist = glyphy_arc_extended_dist(closest_arc, p);
				float new_ext_dist = glyphy_arc_extended_dist(a, p);

				float ext_dist = abs(new_ext_dist) <= abs(old_ext_dist) ? old_ext_dist : new_ext_dist;

				side = sign(ext_dist);
			}
		}
		pp = endpoint.p;
	}
	
	if(side == 0.) {
		float ext_dist = glyphy_arc_extended_dist(closest_arc, p);
		side = sign(ext_dist);
	}

	// 线段 特殊处理
	if(index_info.num_endpoints == 1) {
		line_t line = decode_line(rgba, nominal_size);
		
		vec2 n = vec2(cos(line.angle), sin(line.angle));
		
		side = 1.0;
		
		// min_dist = float(index_info.num_endpoints) / 6.0;
		min_dist = dot(p - 0.5 * vec2(nominal_size), n) - line.distance;
	}

		// side = 1.0;
		// min_dist = float(index_info.num_endpoints) / 6.0;
	// }
 
	return min_dist * side;
}

// 1.0 / sqrt(2.0)
#define SQRT2_2 0.70710678118654757 

// sqrt(2.0)
#define SQRT2   1.4142135623730951

struct glyph_info_t {
	// 网格 宽度，高度 的 格子数量 
	vec2 nominal_size;

	// 索引纹理坐标
	vec2 atlas_pos;

	float sdf;
};

// 解码 
// v.x (有效位 低15位) --> (高7位:纹理偏移.x, 中6位:网格宽高.x, 低2位: 00) 
// v.y (有效位 低15位) --> (高7位:纹理偏移.y, 中6位:网格宽高.y, 低2位: 00) 
glyph_info_t glyph_info_decode(vec2 v) {
	glyph_info_t gi;

	// mod 256 取低8位
	// 除4 取低8位中的 高6位
	
	vec2 rx = div_mod(v.x, 256.0);
	vec2 ry = div_mod(v.y, 256.0);

	vec2 r = vec2(rx.y, ry.y);
	
	// TODO +2 不了解什么意思 
	ivec2 size = (ivec2(r) + 2) / 4;
	gi.nominal_size = vec2(size);

	// 去掉 低8位的 信息 
	ivec2 pos = ivec2(v) / 256;
	gi.atlas_pos = vec2(pos);
	
	return gi;
}

// 抗锯齿 1像素 
// d 在 [a, b] 返回 [0.0, 1.0] 
float antialias_sdf2(float d) {
	// TODO 这个值，文字越少，就应该越少。否则 就会出现 模糊；
	float b = 0.3;
	float a = -b;

	float r = (-d - a) / (b - a);

	return clamp(r, 0.0, 1.0);
}

vec4 outer_glow(float dist_f_, vec4 color_v4_, vec4 input_color_v4_, float radius_f_) {
    // dist_f_ > radius_f_ 结果为 0
    // dist_f_ < 0 结果为 1
    // dist_f_ > 0 && dist_f_ < radius_f_ 则 dist_f_ 越大 a_f 越小，范围 0 ~ 1
    float a_f = abs(clamp(dist_f_ / radius_f_, 0.0, 1.0) - 1.0);
    // pow：平滑 a_f
    // max and min：防止在物体内部渲染
    float b_f = min(max(0.0, dist_f_), pow(a_f, 5.0));
    return color_v4_ + input_color_v4_ * b_f;
}

// 虚线处理
vec4 stroke_dasharray(vec4 input_color,vec4 start_and_step){
	vec2 u_pos = vData12;
	vec2 start = start_and_step.xy;
	float u_step = start_and_step.z + start_and_step.w;
	float a1 = mod(length(u_pos - start), u_step);
	a1 = smoothstep(0.0, 1.0, -(a1 - start_and_step.z + 0.2));

	float a2 = mod(length(u_pos - start) + start_and_step.w, u_step);
	a2 = smoothstep(0.0, 1.0, (a2 - start_and_step.w + 0.2));

	float a = a1 * a2;
	return vec4(input_color.xyz, input_color.w * a);
}

vec4 get_blur_modulus(float blur_level){
	float level = round(blur_level) % 7.0;
	if (abs(level - 3.0) < 0.1) {
		return vec4(1.0, 0.0, 0.0, 0.0);
	} else if (abs(level - 4.0) < 0.1) {
		return vec4(0.5, 0.5, 0.0, 0.0);
	} else if (abs(level - 5.0) < 0.1) {
		return vec4(0.5, 0.3, 0.2, 0.0);
	} else if (abs(level - 6.0) < 0.1) {
		return vec4(0.33, 0.33, 0.33, 0.0);
	}

	return vec4(1.0, 0.0, 0.0, 0.0);
}

// 阴影模糊
// 1 => 
vec4 shadow_blur(vec4 input_color, vec4 shadow_color, vec2 offset, float blur_level){
	vec2 nominal_size = vec2(vData3.zw);
	float x;
	float y;
	if (offset.x > 0.0){
		x = clamp((vUv.x - offset.x) / (1.0 - offset.x), 0.01, 0.99);
	} else {
		x = clamp(vUv.x / (1.0 - abs(offset.x)), 0.01, 0.99);
	}

	if (offset.y > 0.0){
		y = clamp((vUv.y - offset.y) / (1.0 - offset.y), 0.01, 0.99);
	} else {
		y = clamp(vUv.y / (1.0 - abs(offset.y)), 0.01, 0.99);
	}

	vec2 uv1 = vec2(x, y);
	vec2 p1 = uv1 * nominal_size;
	float outlineWidth = vData10.w;
	float a1 =  textureLod(sampler2D(u_shadow_tex, shadow_samp), get_index_uv(p1), 0).r;
	float a2 =  textureLod(sampler2D(u_shadow_tex, shadow_samp), get_index_uv(p1), 1).r;
	float a3 =  textureLod(sampler2D(u_shadow_tex, shadow_samp), get_index_uv(p1), 2).r;
	// float a4 =  textureLod(sampler2D(u_sdf_tex, sdf_tex_samp), get_index_uv(p1), 3).r;
	vec4 modulus = get_blur_modulus(blur_level);
	float a = shadow_color.w * (a1 * modulus.x + a2  * modulus.y + a3 * modulus.z) ; // + a4 * modulus.w);

	return mix(input_color, vec4(shadow_color.rgb, smoothstep(0.1, 0.99, a) * shadow_color.w), 1.0 - input_color.w);
	// return vec4(x, y, 0.0, 1.);
}

void calc_sdf_color(int ty1) {
	vec2 nominal_size = vData3.zw;
	vec2 uv = vUv; 
	if ((ty1 & 2097152) != 0){
		float x;
		float y;
		if (vData13.x > 0.0){
			x = clamp(vUv.x / (1.0 - vData13.x), 0.0, 0.99);
		} else {
			float uvx = abs(vData13.x);
			x = clamp((vUv.x - uvx) / (1.0 - uvx), 0.0, 0.99);
		}

		if (vData13.y > 0.0){
			y = clamp(vUv.y / (1.0 - vData13.y), 0.0, 0.99);
		} else {
			float uvy = abs(vData13.y);
			y = clamp((vUv.y - uvy) / (1.0 - uvy), 0.0, 0.99);
		}

		uv = vec2(x, y); // vUv / (vec2(1.0, 1.0) - vData13.xy);
	}
	vec2 p = uv * nominal_size;
	// 重点：计算 SDF 
	float gsdist = glyphy_sdf(p, nominal_size);

	// o_Target = vec4(gsdist, 0.0, 0.0, 1.0);
	
	// 均匀缩放 
	float scale = SQRT2 / length(fwidth(p));

	float sdist = gsdist * scale;

	// 每渲染像素对应Distance
	// 1024. 是数据生成时用的计算范围
	float distancePerPixel = 1.;

	float weight = vData11.w;
	sdist = sdist - weight * distancePerPixel;

	float alpha = antialias_sdf2(sdist);
	if (vData10.w > 0.0){
		alpha = step(0., -sdist);
	}

	vec4 faceColor = vec4(vData0.rgb, vData0.a * alpha);

	if ((ty1 & 4096) != 0) { // 线性渐变颜色

		// 因为实例数据槽位限制， 传入数据优先， 因此渐变色仅支持3通道
		// 如果必须要支持4通道， 可将将外发光单独绘制为一个实例， 就有空间防止4通道了
		vec3 gColor1     = vData6.xyz;
		vec3 gColor2     = vec3(vData6.w, vData7.xy);
		vec3 gColor3     = vec3(vData7.zw, vData8.x);
		vec3 gColor4     = vData8.yzw;

		vec4 gPosition   = vData0;

		vec2 gradientDir        = vData9.zw - vData9.xy; // vData9为渐变端点 (逻辑控制 两者不相等)
		vec2 gradientDirNor     = normalize(gradientDir);
		float gradientLength    = length(gradientDir);

		vec2 gradientV          = vVertexPosition - vData9.xy;
		float gradient          = dot(gradientV, gradientDirNor) / gradientLength;

		vec3 color      = gColor1 * step(gradient, gPosition.x)
								// 这里加上0.00001 避免除以0
								+ mix(gColor1, gColor2, (gradient - gPosition.x) / (gPosition.y - gPosition.x + 0.00001)) * step(gPosition.x, gradient) * step(gradient, gPosition.y) 
								+ mix(gColor2, gColor3, (gradient - gPosition.y) / (gPosition.z - gPosition.y + 0.00001)) * step(gPosition.y, gradient) * step(gradient, gPosition.z)
								+ mix(gColor3, gColor4, (gradient - gPosition.z) / (gPosition.w - gPosition.z + 0.00001)) * step(gPosition.z, gradient) * step(gradient, gPosition.w) 
								+ gColor4 * step(gPosition.w, gradient);
		faceColor = vec4(color, alpha);
	}
							
    // faceColor.rgb   		= mix(faceColor.rgb, gradientColor, step(0.05, gradientLength));
	// faceColor.rgb *= 0.0;

	vec4 u_outline = vData10;
	float outlineSofeness 	= 0.8;
	float outlineWidth 		= u_outline.w * distancePerPixel;
	vec4 outlineColor 		= vec4(u_outline.xyz, 1.0);
	// outlineColor.rgb *=0.0;
	float outline 			= (1.0 - smoothstep(0., outlineWidth, abs(sdist)));// * step(-0.1, sdist);
	float alphaOutline 		= outline; // min(outline, 1.0 - alpha) * step(0.001, outline);
	float outlineFactor 	= smoothstep(0.0, outlineSofeness, alphaOutline);
	// outlineColor.a 			= outlineFactor;
	vec4 finalColor 		= mix(faceColor, outlineColor, outlineFactor);

	if ((ty1 & 262144) != 0) {// 外发光
		vec4 outer_glow_color_and_dist = vData5;
		o_Target = outer_glow(sdist, finalColor, vec4(outer_glow_color_and_dist.xyz, 1.0) , outer_glow_color_and_dist.w);
	} 
	else if ((ty1 & 524288) != 0) {// 虚线
		vec4 start_and_step = vData5;
		o_Target = stroke_dasharray(finalColor, start_and_step);
	} 
	else if ((ty1 & 2097152) != 0) {// 阴影
		vec4 shadow_color = vData5;
		vec3 shadow_offset_and_blur_level = vData13;
		o_Target = shadow_blur(finalColor, shadow_color, shadow_offset_and_blur_level.xy, shadow_offset_and_blur_level.z);

		// o_Target = finalColor;
	} 
	else {
		o_Target = finalColor;
	}
	// vec2 u_pos = vData12;
	// o_Target = vec4(u_pos.y / 420.0, 0.0, 0.0, 1.0);

	// o_Target = vec4(1.0, 0.0, 0.0, 1.0);
}
/********************************************************************************************************************************/


void main(void) {
	int ty1 = int(vData11.y); // clip_max_xect = 1; clip_max_xect_radius = 4; clip_circel= 8; clip_ellipse = 16; clip_secotor = 32; uv = 64; color = 128; canvas_text = 256;text_stroke = 512;is_not_visibility = 1024;pointer_samp = 2048;

	if ((ty1 & 131072) != 0) {// 圆弧方案的sdf
		calc_sdf_color(ty1);
		return;
	}

	vec4 color = vec4(0.0, 1.0, 0.0, 1.0);

	// color = texture(sampler2D(tex2d0, samp0),vUv);
	float texture_layer = vData5.x;
	bool is_uv = (ty1 & 64) != 0; 
	if (is_uv) {
		vec2 uv = vUv;
		if ((ty1 & 32768) != 0) { // BackgroundImageRepeat
			// vData5: float texture_layer; float empty; vec2 offset
			// vData6: vec2 position_step; vec2 position_space
			uv = (vVertexPosition + vData5.zw) % vData6.xy / vData6.zw;
		} else if ((ty1 & 65536) != 0){ // BorderImage 要求上下、左右不相等， 且分割范围在布局范围内（逻辑保证）
			// vData0: vec4 uvb (min max)(在整张图的uv，而不是局部的imageclip, 单位： 像素)
			// vData5: float texture_layer; float 1.0表示需要填充;vec2 texture_size;
			// vData6: vec4 border_width(上右下左的边框宽度)
			// vData7: vec4 border_slice(上右下左)(在整张图的uv，而不是局部的imageclip， 单位： 像素)

			// vData8: vec4 step
			// vData9: vec4 space
			// vData10: vec4 offset
			vec4 uv_max_box = vData0;
			vec4 border_width = vData6;
			vec4 border_slice = vData7;
			vec4 steps = vData8;
			vec4 space = vData9;
			vec4 offset = vData10;
			// vec4 offset = vec4(0.0, 0.0, 0.0, 0.0);
			vec2 pos_size = vData1.zw; // 布局尺寸

			vec2 border_right_bottom = pos_size - border_width.yz; // border， 右下切割线相对布局原点的位置
			// 计算命中区域（用top_right_bottom_left和fill代表， 他们都是0.0，或1.0， 当为1.0是，表示在该区域）
			// vec4 lefttop_max_rightbottom = step(vec4(vVertexPosition, border_right_bottom), vec4(border_width.wx, vVertexPosition));
			vec4 top_right_bottom_left = step(vec4(vVertexPosition.y, border_right_bottom.xy, vVertexPosition.x), vec4(border_width.x, vVertexPosition, border_width.w));
			vec2 fill = 1.0 - top_right_bottom_left.wx - top_right_bottom_left.yz;


			vec2 middle_xy = vec2(1.0 - step(fill.x - fill.y, 0.0), 1.0 - step(fill.y - fill.x, 0.0)); // 在y、x方向上，是否处于中间部分
			vec4 side_factor = vec4(middle_xy, middle_xy) * top_right_bottom_left;// 是否在四条边上（上右下左）
			float is_side = max(max(max(top_right_bottom_left.w, top_right_bottom_left.x), top_right_bottom_left.y), top_right_bottom_left.z);
			float is_center = max(fill.x, fill.y);
			// 是否处于四个角上或处于中心位置（用一个浮点表示即可， 每个角和中心位置的处理方式是相同的）
			// float rangle_fill_factor = max(max(max(top_right_bottom_left.w, top_right_bottom_left.x), top_right_bottom_left.y), top_right_bottom_left.z) - max(max(max(max(side_factor.x, side_factor.y), side_factor.y), side_factor.z), side_factor.w) + max(fill.x, fill.y) * vData5.z;
			float rangle_fill_factor = max(max((is_side - is_center), (1.0 - is_side) * vData5.y), 0.0);

			if (rangle_fill_factor + is_side <= 0) { // 空白处， 透明处理
				discard;
			}
	
			float uv_min_x = top_right_bottom_left.w * uv_max_box.x   + fill.x * border_slice.w + top_right_bottom_left.y * border_slice.y; // min_x  (min左  + min中 + min右)
			float uv_min_y = top_right_bottom_left.x * uv_max_box.y   + fill.y * border_slice.x + top_right_bottom_left.z * border_slice.z; // min_y (min上 + min中 + min下)
			float uv_max_x = top_right_bottom_left.w * border_slice.w + fill.x * border_slice.y + top_right_bottom_left.y * uv_max_box.z; // max_x (max左 + max中 + max右)
			float uv_max_y = top_right_bottom_left.x * border_slice.x + fill.y * border_slice.z + top_right_bottom_left.z * uv_max_box.w; // max_y (max上 + max中 + max下)

			// uv(当前命中区域的uv盒子)（命中区域分别可能是四角、四边、中间部分）
			// vec4 uv_max_box = vec4(uv_min_y, uv_max_x, uv_max_y, uv_min_x);
			// 顶点（命中区域的顶点起始点）
			float p_min_x =                                              fill.x * border_width.w +        top_right_bottom_left.y * border_right_bottom.x; // min_x  (min左（为0）  + min中 + min右)
			float p_min_y = 	                                         fill.y * border_width.x +        top_right_bottom_left.z * border_right_bottom.y; // min_y (min上（为0） + min中 + min下)
			float p_max_x = top_right_bottom_left.w * border_width.w + fill.x * border_right_bottom.x + top_right_bottom_left.y * pos_size.x; // max_x (max左 + max中 + max右)
			float p_max_y = top_right_bottom_left.x * border_width.x + fill.y * border_right_bottom.y + top_right_bottom_left.z * pos_size.y; // max_y (max上 + max中 + max下)


			vec2 size = vec2(uv_max_x, uv_max_y) - vec2(uv_min_x, uv_min_y);
			vec2 p_size = vec2(p_max_x, p_max_y) - vec2(p_min_x, p_min_y);

			vec2 scale = p_size / size;
			
			vec4 steps_x = side_factor * vec4(steps.x, p_size.x, steps.z, p_size.x); // 边（四周中间部分可能需要重复，其他部分使用填充 ）
			vec4 space_x = side_factor * vec4(space.x, p_size.x, space.z, p_size.x); // 边（四周中间部分可能需要重复，其他部分使用填充 ）
			vec4 offset_x = side_factor * vec4(offset.x, 0.0, offset.z, 0.0); // 边（四周中间部分可能需要重复，其他部分使用填充 ）

			vec4 steps_y = side_factor * vec4(p_size.y, steps.y, p_size.y, steps.w); // 边（四周中间部分可能需要重复，其他部分使用填充 ）
			vec4 space_y = side_factor * vec4(p_size.y, space.y, p_size.y, space.w); // 边（四周中间部分可能需要重复，其他部分使用填充 ）
			vec4 offset_y = side_factor * vec4(0.0, offset.y, 0.0, offset.w); //（四周中间部分可能需要重复，其他部分使用填充 ）
			
			vec3 sso_x = vec3(steps_x.x,                    space_x.x,                     offset_x.x) +
						vec3(steps_x.y,                     space_x.y,                     offset_x.y) +
						vec3(steps_x.z,                     space_x.z,                     offset_x.z) +
						vec3(steps_x.w,                     space_x.w,                     offset_x.w) +
						vec3(rangle_fill_factor * p_size.x, rangle_fill_factor * p_size.x, 0.0); // 角 + fill情况

			vec3 sso_y = vec3(steps_y.x,                    space_y.x,                     offset_y.x) +
						vec3(steps_y.y,                     space_y.y,                     offset_y.y) +
						vec3(steps_y.z,                     space_y.z,                     offset_y.z) +
						vec3(steps_y.w,                     space_y.w,                     offset_y.w) +
						vec3(rangle_fill_factor * p_size.y, rangle_fill_factor * p_size.y, 0.0); // 角 + fill情况

			// sso_x = vec3(rangle_fill_factor * p_size.x, rangle_fill_factor * p_size.x, 0.0); // 角 + fill情况

			// sso_y = vec3(rangle_fill_factor * p_size.y, rangle_fill_factor * p_size.y, 0.0); // 角 + fill情况


			uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) % vec2(sso_x.x, sso_y.x) / vec2(sso_x.y, sso_y.y);
			if (max(uv.x, uv.y) > 1.0) { // 空白处， 透明处理
				discard;
			}
			uv = (uv * size + vec2(uv_min_x, uv_min_y))/ vData5.zw;
			// uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) % vec2(sso_x.x, sso_y.x) / vec2(sso_x.y, sso_y.y);
			// uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) / vData1.zw;
			// uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) / vData1.zw;
			// uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) % vec2(sso_x.x, sso_y.x) / vec2(sso_x.y, sso_y.y);
			// uv = (vVertexPosition - vec2(p_min_x, p_min_y) + vec2(sso_x.z, sso_y.z)) % vec2(sso_x.x, sso_y.x) / vec2(sso_x.y, sso_y.y);
			// uv = steps_x.xy / space_x.xy;
			// uv = vec2(p_min_x, p_min_y) / p_size;
			// uv = vec2(rangle_fill_factor, rangle_fill_factor);
			// vec4 side_factor = vec4(middle_yx, middle_yx) * lefttop_max_rightbottom;// 是否在四条边上（上右下左）
			// // 是否处于四个角上或处于中心位置（用一个浮点表示即可， 每个角和中心位置的处理方式是相同的）
			// float rangle_fill_factor

				
			// float uv_min_x = top_right_bottom_left.w * uv_max_box.x   + fill.x * border_slice.w + top_right_bottom_left.y * border_slice.y; // min_x  (min左  + min中 + min右)
			// float uv_min_y = top_right_bottom_left.x * uv_max_box.y   + fill.y * border_slice.x + top_right_bottom_left.z * border_slice.z; // min_y (min上 + min中 + min下)
			// float uv_max_x = top_right_bottom_left.w * border_slice.w + fill.x * border_slice.y + top_right_bottom_left.y * uv_max_box.z; // max_x (max左 + max中 + max右)
			// float uv_max_y = top_right_bottom_left.x * border_slice.x + fill.y * border_slice.z + top_right_bottom_left.z * uv_max_box.w; // max_y (max上 + max中 + max下)
			// o_Target = vec4(vec2(uv_min_x, uv_min_y) / vData5.zw, 0.0, 1.0);
			// return;
		}

		if (uv.x > vData0.z || uv.y > vData1.w) { // 空白处， 透明处理
			discard;
		}else if (texture_layer == 0.0) {
			color = texture(sampler2D(tex2d0, samp0),uv);
		} else if (texture_layer == 1.0) {
			color = texture(sampler2D(tex2d1, samp1),uv);
		} else if (texture_layer == 2.0) {
			color = texture(sampler2D(tex2d2, samp2),uv);
		} else if (texture_layer == 3.0) {
			color = texture(sampler2D(tex2d3, samp3),uv);
		} else if (texture_layer == 4.0) {
			color = texture(sampler2D(tex2d4, samp4),uv);
		} else if (texture_layer == 5.0) {
			color = texture(sampler2D(tex2d5, samp5),uv);
		} else if (texture_layer == 6.0) {
			color = texture(sampler2D(tex2d6, samp6),uv);
		} else if (texture_layer == 7.0) {
			color = texture(sampler2D(tex2d7, samp7),uv);
		} else if (texture_layer == 8.0) {
			color = texture(sampler2D(tex2d8, samp8),uv);
		} else if (texture_layer == 9.0) {
			color = texture(sampler2D(tex2d9, samp9),uv);
		} else if (texture_layer == 10.0) {
			color = texture(sampler2D(tex2d10, samp10),uv);
		} else if (texture_layer == 11.0) {
			color = texture(sampler2D(tex2d11, samp11),uv);
		} else if (texture_layer == 12.0) {
			color = texture(sampler2D(tex2d12, samp12),uv);
		}
		//  else if (texture_layer == 13.0) {
		// 	color = texture(sampler2D(tex2d13, samp13),uv);
		// } 
		else {
			color = vec4(1.0, 0.0, 1.0, 1.0);
		}
		
		//  else if (texture_layer == 14.0) {
		// 	color = texture(sampler2D(tex2d14, samp14),uv);
		// } else if (texture_layer == 15.0) {
		// 	color = texture(sampler2D(tex2d15, samp15),vUv);
		// }
	} else if ((ty1 & 128) != 0) { // color
		color.xyz = vData0.xyz;
		color.w = color.w * vData0.w;
	} else if ((ty1 & 256) != 0) { // canvas text

		// color.w = vData5.w * color.w;
		// color.xyz = vData5.rgb;

		// // 描边
		// vec3 outlineColor = color;
		// if (ty1 & 512 != 0) { 
		// 	outlineColor = vData6;
		// }

		// vec4 samp = texture(sampler2D(tex2d, samp), vUv);

		
		// // 应该 c.a = 1.0 - samp.b, 由于纹理坐标误差， 导致采样到纹理的空白处（rgba都为0）， 会看到一条黑线
		// // b表示背景分量
		// color.w = color.w * clamp(samp.a - samp.b, 0.0, 1.0);
		
		// color.rgb = ((color * samp.g)  + (samp.r * outlineColor)); // rgb: (0, 255, 0); samp.g = 95; samp.r = 160; ret = (0, 182, 0 )
	} else if ((ty1 & 4096) != 0) { // 线性渐变颜色
	// layout(location = 5) in vec4 vData0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
	// layout(location = 6) in vec4 vData1; // vec4 offset + scale; 
	// layout(location = 7) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
	// layout(location = 8) in vec4 vData3; // vec4 clip_min_yop_max_xadius | (vec2 clip_sector_radian; float clip_sector_radius) | 
	// layout(location = 9) in vec4 vData4; // vec4 clip_max_yottom_radius
	// layout(location = 10) in vec4 vData5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_index
	// layout(location = 11) in vec4 vData6; // vec4 gradient_color1 | vec4 stroke_color
	// layout(location = 12) in vec4 vData7; // vec4 gradient_color2
	// layout(location = 13) in vec4 vData8; // vec4 gradient_color3

		vec4 gColor1     = vData5;
		vec4 gColor2     = vData6;
		vec4 gColor3     = vData7;
		vec4 gColor4     = vData8;

		vec4 gPosition   = vData0;

		// vec2 gradientStart      = vData9.xy;
		// vec2 gradientEnd        = vData9.zw;
		vec2 gradientDir        = vData9.zw - vData9.xy; // vData9为渐变端点 (逻辑控制 两者不相等)
		vec2 gradientDirNor     = normalize(gradientDir);
		float gradientLength    = length(gradientDir);

		vec2 gradientV          = vVertexPosition - vData9.xy;
		float gradient          = dot(gradientV, gradientDirNor) / gradientLength;

		color      = gColor1 * step(gradient, gPosition.x)
								// 这里加上0.00001 避免除以0
								+ mix(gColor1, gColor2, (gradient - gPosition.x) / (gPosition.y - gPosition.x + 0.00001)) * step(gPosition.x, gradient) * step(gradient, gPosition.y) 
								+ mix(gColor2, gColor3, (gradient - gPosition.y) / (gPosition.z - gPosition.y + 0.00001)) * step(gPosition.y, gradient) * step(gradient, gPosition.z)
								+ mix(gColor3, gColor4, (gradient - gPosition.z) / (gPosition.w - gPosition.z + 0.00001)) * step(gPosition.z, gradient) * step(gradient, gPosition.w) 
								+ gColor4 * step(gPosition.w, gradient);
	} else if ((ty1 & 16384) != 0) {  // box_shadow
		color = vData0;
		color.a = color.a * getShadowAlpha(vVertexPosition, vData1.xy + vData5.w, vData1.xy + vData1.zw - vData5.w, vData5.w / 2.0);
	}

	if ((ty1 & 8) != 0) {
		color.w = color.w * calc_circel_alpha();
	} else if ((ty1 & 16) != 0) {
		color.w = color.w * calc_ellipse_alpha();
	} else if ((ty1 & 32) != 0) {
		color.w = color.w * calc_sector_alpha();
	} else if ((ty1 & 8192) != 0) {
		color.w = color.w * calc_border_alpha();
		// color = vec4(0.0, 0.0, calc_border_alpha(), 1.0);
	} else if ((ty1 & 4) != 0) { 
		color.w = color.w * calc_radius_alpha();
		// color = vec4(0.0, 0.0, calc_radius_alpha(), 1.0);
	} else if ((ty1 & 1) != 0) {
		color.w = color.w * calc_rect_alpha();
	}

	// color = texture(sampler2D(tex2d0, samp0),vUv);

	// color.w = color.w * (1.0 - vData11.x);
	o_Target = color;

}