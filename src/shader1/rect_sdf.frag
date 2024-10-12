
#version 450

precision highp float;

// 输入
layout(location = 0) in vec2 vVertexPosition; // 顶点到裁剪中心点的距离

layout(set = 0, binding = 0) uniform RectSdfMeterial {
	vec4 geo; // offset + scale (0~1范围)
	vec2 center; // 中心点(0~1范围)
	vec2 extent; // 半宽半高(0~1范围)
};

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

// 可以看成 fs 中 计算 统一缩放系数 的 倒数
float computeAARange(vec2 position) {
	// position 变化率，放大2倍，w 0.5
	vec2 w = fwidth(position);
	
	// sqrt(2)/length(w) = inversesqrt(0.5 * dot(w, w))
	return inversesqrt(0.5 * dot(w, w));
}

// 返回 coord 到 矩形 最短距离, 负值表示 在里面, 正值表示在外面
float sdfRect(vec2 xy, vec2 wh)
{
	vec2 d = abs(xy) - wh;
	return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

// 计算alpha
float calc_rect_alpha() {
	vec2 pos = vVertexPosition - geo.xy;
	float d = sdfRect(pos, extent);

	float aaRange = computeAARange(pos);
	return distanceAA(aaRange, d);
}

void main(void) {
	o_Target = vec4(calc_rect_alpha(), 1.0, 1.0, 1.0);
}