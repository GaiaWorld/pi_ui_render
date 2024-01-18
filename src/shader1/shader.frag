
// 输入
layout(location = 0) in vec2 vVertexPosition; // 顶点到裁剪中心点的距离
layout(location = 1) in vec2 vUv; // uv
layout(location = 5) in vec4 vData0; // vec4 uv | vec4 gradient_position | vec4 bg_color | vec4 border_color;
layout(location = 6) in vec4 vData1; // vec2 scale; float alpha; float ty;
layout(location = 7) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
layout(location = 8) in vec4 vData3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | 
layout(location = 9) in vec4 vData4; // vec4 clip_bottom_radius
layout(location = 10) in vec4 vData5; // vec4 gradient_color0 | vec4 border_width | vec4 text_color | float texture_index
layout(location = 11) in vec4 vData6; // vec4 gradient_color1 | vec4 stroke_color
layout(location = 12) in vec4 vData7; // vec4 gradient_color2
layout(location = 13) in vec4 vData8; // vec4 gradient_color3

sampler{1D,2D}Array
image{1D,2D}Array

// 纹理
layout(set=3,binding=0)uniform sampler linear_samp;
layout(set=3,binding=1)uniform sampler pointer_samp;
layout(set=3,binding=2)uniform texture2D tex2d[];


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

float calc_border_radius_alpha() {
	vec2 pos = vVertexPosition - vData2.xy; // 到中心的距离
	vec4 top = vData3;
	vec4 bottom = vData4;
	// 左上角
	vec2 c1 = vec2(max(0.01, top.y), max(0.01, top.x));
	// 右上角
	vec2 c2 = vec2(-max(0.01, top.z), max(0.01, top.w));
	// 右下角
	vec2 c3 = vec2(-max(0.01, bottom.y), -max(0.01, bottom.x));
	// 左下角
	vec2 c4 = vec2(max(0.01, bottom.z), -max(0.01, bottom.w));
	
	vec2 extent = clipSdf[1].xy;
	float d = sdfRoundRect(pos, extent, c1, c2, c3, c4);

	float aaRange = computeAARange(pos);
	return distanceAA(aaRange, d);
}

// 计算
float calc_sector_alpha() 
{

	layout(location = 7) in vec4 vData2; // vec2 center; float clip_circel_radius | vec2 clip_ellipse_ab | vec2 clip_sector_rotate | vec2 clip_extent(矩形和圆角矩形都需要)
	layout(location = 8) in vec4 vData3; // vec4 clip_top_radius | (vec2 clip_sector_radian; float clip_sector_radius) | 
	
	// (旋转角度+扇形弧度/2)的sin， cos值
	vec2 axisSC = vData2.zw;
	// (扇形弧度/2)的sin， cos值
	vec2 sc =  vData3.xy;
	// 扇形的半径
	float r = pie1.z;

	vec2 pos = vVertexPosition - pie1.xy;

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


void main(void) {
	int ty1 = int(vData1.w); // clip_rect = 1; clip_rect_radius = 4; clip_circel= 8; clip_ellipse = 16; clip_secotor = 32; uv = 64; color = 128; canvas_text = 256;text_stroke = 512;is_not_visibility = 1024;pointer_samp = 2048;
	vec4 color = vec4(1.0, 1.0, 1.0, 1.0);

	if (ty1 & 64 != 0) { // 采样，TODO
		// layout(set=3,binding=0)uniform sampler linear_samp;
		// layout(set=3,binding=1)uniform sampler pointer_samp;
		// layout(set=3,binding=2)uniform texture2D tex2d[];
		if (ty1 & 2048 != 0) {  // 表示需要点采样
			color = texture(sampler2D(tex2d[int(vData5.x)], pointer_samp),vUv);
		} else { // 否则进行线性采样
			color = texture(sampler2D(tex2d[int(vData5.x)], linear_samp),vUv);
		}

		// alpha = alpha * calc_rect_alpha(vVertexPosition);
	} else if (ty1 & 128 != 0) { 
		color.xyz = vData0.xyz;
		color.w = color.w * vData0.w;
	} else if (ty1 & 256 != 0) { // canvas text

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
	}

	if (ty1 & 1 != 0) {
		color.w = color.w * calc_rect_alpha(vVertexPosition);
	} else if (ty1 & 4 != 0) { 
		color.w = color.w * calc_border_radius_alpha(vVertexPosition);
	} else if (ty1 & 8 != 0) {
		color.w = color.w * calc_circel_alpha(vVertexPosition);
	} else if (ty1 & 16 != 0) {
		color.w = color.w * calc_ellipse_alpha(vVertexPosition);
	} else if (ty1 & 32 != 0) {
		color.w = color.w * calc_sector_alpha(vVertexPosition);
	}

	color.w = color.w * vData1.z;

	gl_FragColor = color;

}