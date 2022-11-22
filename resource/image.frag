#version 450

// 输入uv
layout(location = 0) in vec2 vUv;
// 输入position
layout(location = 1) in vec2 vVertexPosition;

// 输出颜色
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform UiMaterial {
	mat4 world; // 世界矩阵
	// 扇形 SDF 信息
    // [
    //    vec3 (布局中心.x, 布局中心.y, 布局缩放.x)
    //    vec3 (布局缩放.y, sin(对称轴-y轴), cos(对称轴-y轴))
    //    vec3 (sin(边缘-对称轴), cos(边缘-对称轴), r)
    // ]
	mat4 clipSdf; // border_radius | ellipse | circle | sector | rect
	float depth; // 深度
};

// 纹理
layout (set = 2, binding = 0) uniform sampler samp;
layout (set = 2, binding = 1) uniform texture2D tex2d;

// 根据 d, 抗锯齿, 返回 alpha值
float antialiase(float d) 
{
	float anti = fwidth(d);
	
	// smoothstep(-a, a, d) 意思是 根据 d-值 将 [-a, a] 平滑到 [0, 1] 中
	// d < -a, 全内部, 得到0, 这时期望 alpha = 1.0
	// d > a, 全外部, 得到1, 这时期望 alpha = 0.0
	
	return 1.0 - smoothstep(-anti, anti, d);
}

#ifdef BORDER_RADIUS
	
	float sdfEllipse(vec2 pt, vec2 center, vec2 ab)
	{
		pt -= center;
		
		// 求 (1/a, 1/b)
		vec2 recAB = 1.0 / ab;
		// 求 (x/a, y/b) = (x, y) * (1/a, 1/b)
		vec2 scale = pt * recAB;
		
		// 椭圆值 f = (x/a)^2 + (y/b)^2 - 1
		return dot(scale, scale) - 1.0;
	}

	float sdfRect(vec2 pt, vec2 wh)
	{
		vec2 d = abs(pt) - wh;
		return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
	}

	float cross_pt(vec2 v1, vec2 v2) {
		return -(v1.x * v2.y - v1.y * v2.x);
	}
	// p0, p1, p2 是否 逆时针
	bool is_ccw(vec2 p0, vec2 p1, vec2 p2) {
		vec2 v1 = p1 - p0;
		vec2 v2 = p2 - p0;
		float r = cross_pt(v1, v2);
		return r > 0.0;
	}
	bool is_left_top(vec2 pt, vec2 wh, vec2 center) {
		vec2 pt0 = vec2(-wh.x, center.y);
		vec2 pt1 = vec2(center.x, -wh.y);
		return is_ccw(pt, pt0, pt1);
	}
	bool is_top_right(vec2 pt, vec2 wh, vec2 center) {
		vec2 pt0 = vec2(center.x, -wh.y);
		vec2 pt1 = vec2(wh.x, center.y);
		return is_ccw(pt, pt0, pt1);
	}
	bool is_right_bottom(vec2 pt, vec2 wh, vec2 center) {
		vec2 pt0 = vec2(wh.x, center.y);
		vec2 pt1 = vec2(center.x, wh.y);
		return is_ccw(pt, pt0, pt1);
	}
	bool is_bottom_left(vec2 pt, vec2 wh, vec2 center) {
		vec2 pt0 = vec2(center.x, wh.y);
		vec2 pt1 = vec2(-wh.x, center.y);
		return is_ccw(pt, pt0, pt1);
	}
	float antialiase_round_rect(vec2 pt, vec2 extent, vec2 offset1, vec2 offset2, vec2 offset3, vec2 offset4) {
		
		float d_rect = sdfRect(pt, extent);
		float a_rect = antialiase(d_rect);
		vec2 center = vec2(-extent.x + offset1.x, -extent.y + offset1.y); 
		if (is_left_top(pt, extent, center)) {
			float d = sdfEllipse(pt, center, abs(offset1));
			float a = antialiase(d);
			return min(a_rect, a);
		}
		center = vec2(extent.x + offset2.x, -extent.y + offset2.y); 
		if (is_top_right(pt, extent, center)) {
			float d = sdfEllipse(pt, center, abs(offset2));
			float a = antialiase(d);
			return min(a_rect, a);
		}
		center = vec2(extent.x + offset3.x, extent.y + offset3.y); 
		if (is_right_bottom(pt, extent, center)) {
			float d = sdfEllipse(pt, center, abs(offset3));
			float a = antialiase(d);
			return min(a_rect, a);
		}
		
		center = vec2(-extent.x + offset4.x, extent.y + offset4.y); 
		if (is_bottom_left(pt, extent, center)) {
			float d = sdfEllipse(pt, center, abs(offset4));
			float a = antialiase(d);
			return min(a_rect, a);
		}
		return a_rect;
	}

	float calc_alpha(vec2 vVertexPosition) {
		vec4 scale = clipSdf[0];
		vec2 pos = scale.zw * vVertexPosition - scale.xy;
		vec4 top = clipSdf[2];
		vec4 bottom = clipSdf[3];
		// 左上角
		vec2 c1 = vec2(max(0.01, top.y), max(0.01, top.x));
		// 右上角
		vec2 c2 = vec2(-max(0.01, top.z), max(0.01, top.w));
		// 右下角
		vec2 c3 = vec2(-max(0.01, bottom.y), -max(0.01, bottom.x));
		// 左下角
		vec2 c4 = vec2(max(0.01, bottom.z), -max(0.01, bottom.w));
		
		vec4 extent = clipSdf[1];
		return antialiase_round_rect(pos, extent.xy, c1, c2, c3, c4);
	}
#endif

#ifdef SECTOR
	// 扇形 sdf，负数在里面，正数在外面
    // pt 待求点
    // c 扇形 边缘处 距离 y轴的 夹角 sin, cos
    // r 半径
    // 参考 https://zhuanlan.zhihu.com/p/427587359
    float sdfPie(vec2 p, vec2 sc, float r)
    {
        p.x = abs(p.x);
        float d1 = length(p) - r;
        
        float m = length(p - sc * clamp(dot(p, sc), 0.0, r) );
        float d2 = m * sign(sc.y * p.x - sc.x * p.y);
        return max(d1, d2);
    }

	// 计算
	float calc_alpha(float d) 
    {
        vec4 scale = clipSdf[0];
        vec4 pie2 = clipSdf[1];
        vec4 pie3 = clipSdf[2];
        
        vec2 axisSC = pie2.xy;
        vec2 sc = pie2.zw;
        float r = pie3.x;
        vec2 pos = scale.zw * vVertexPosition - scale.xy;
        
        // 逆过来乘，将 扇形 乘回 到 对称轴 为 y轴 处
        pos = vec2(axisSC.y * pos.x - axisSC.x * pos.y, axisSC.x * pos.x + axisSC.y * pos.y);
        float d = sdfPie(pos, sc, r);
        
        return antialiase(d);
    }
#endif

#ifdef RECT
    // 返回 coord 到 矩形 最短距离, 负值表示 在里面, 正值表示在外面
    // 
    float sdfRect(vec2 xy, vec2 wh)
    {
        vec2 d = abs(xy) - wh;
        return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
    }

	// 计算alpha
	float calc_alpha(float d) {
		vec4 scale = clipSdf[0];
		vec4 uExtent = clipSdf[1];
		vec2 pos = scale.zw * vVertexPosition - scale.xy;
		float d = sdfRect(pos, uExtent.xy);
	}
#endif

#ifdef ELLIPSE
	// 
    // TODO 当 a，b 的 值 相差 过大时，会 不准确
    // 
    float sdfEllipseSimple(vec2 xy, vec2 ab)
    {
        // 求 (1/a, 1/b)
        vec2 recAB = 1.0 / ab;
        // 求 (x/a, y/b) = (x, y) * (1/a, 1/b)
        vec2 scale = xy * recAB;
        
        // 椭圆值 f = (x/a)^2 + (y/b)^2 - 1
        return dot(scale, scale) - 1.0;
    }

	// 计算alpha
	float calc_alpha(float d) {
		vec4 scale = clipSdf[0];
		vec4 uEllipseAB = clipSdf[1];

		vec2 pos = scale.zw * vVertexPosition - scale.xy;
        float d = sdfEllipseSimple(pos, uEllipseAB.xy);
        
        return antialiase(d);
	}
#endif

#ifdef CIRCLE
	// 返回 coord 到 圆的 最短距离, 负值表示 在里面, 正值表示在外面
    float sdfCircle(vec2 xy, float r) {
        return length(xy) - r;
    }

	// 计算alpha
	float calc_alpha(float d) {
		vec4 scale = clipSdf[0];
		vec4 radius = clipSdf[1];
		vec2 pos = scale.zw * vVertexPosition - scale.xy;
        float d = sdfCircle(pos, radius.x);
        
        return antialiase(d);
	}

#endif

void main() {
	vec4 color = texture(sampler2D(tex2d, samp), vUv);

	#ifdef BORDER_RADIUS
		color.a = color.a * calc_alpha(vVertexPosition);
	#endif

	#ifdef SECTOR
		color.a = color.a * calc_alpha(vVertexPosition);
	#endif

	#ifdef RECT
		color.a = color.a * calc_alpha(vVertexPosition);
	#endif

	#ifdef ELLIPSE
		color.a = color.a * calc_alpha(vVertexPosition);
	#endif

	#ifdef CIRCLE
		color.a = color.a * calc_alpha(vVertexPosition);
	#endif

	// 預乘模式
	o_Target = vec4(color.rgb * color.a, color.a);
}