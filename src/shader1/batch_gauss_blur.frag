
#version 450

precision highp float;

// 输入
layout(location = 0) in vec2 vUv; // 当前点的uv坐标
layout(location = 1) in vec2 vOffsetScale; // 单个像素的uv偏移
layout(location = 2) in vec2 vGaussCoefficients; // 高斯模糊系数（由顶点着色器根据模糊半径算出）
layout(location = 3) in vec4 uvRect; // uv矩形范围(0~1)
layout(location = 4) in float support; // 采样像素个数（大致为模糊半径的两倍，因为需要左右对称）


// texture
layout(set=1,binding=0) uniform texture2D tex2d;
layout(set=1,binding=1) uniform sampler samp;

// 输出颜色
layout(location=0) out vec4 outColor;

/********************************************************************************************************************************/

void main() {
	float original_color = texture(sampler2D(tex2d, samp), vUv).r;

	// Incremental Gaussian Coefficent Calculation (See GPU Gems 3 pp. 877 - 889)
	vec3 gauss_coefficient = vec3(vGaussCoefficients,
								vGaussCoefficients.y * vGaussCoefficients.y);
	// 当前采样点的权重
	float avg_color = original_color * gauss_coefficient.x;

	float a = support;
	
	// 其他权重的点的采样（左右需要对称）
	for (float i = 1.0; i <= 300.0; i += 1.0) {
		if (i > support) {
			break;
		}
		gauss_coefficient.xy *= gauss_coefficient.yz;
		vec2 offset = vOffsetScale * i;
		
		// 计算负方向和正方向上偏移的像素的像素值
		vec2 st0 = vUv - offset;
		vec2 st1 = vUv + offset;

		st0 = vec2(max(st0.x, uvRect.x), max(st0.y, uvRect.y));
		st1 = vec2(min(st1.x, uvRect.z), min(st1.y, uvRect.w));
		avg_color += (texture(sampler2D(tex2d, samp), st0).r + texture(sampler2D(tex2d, samp), st1).r) *
					gauss_coefficient.x;	
	}

	// 输出颜色值
	outColor = vec4(avg_color, 1.0, 1.0, 1.0);
}