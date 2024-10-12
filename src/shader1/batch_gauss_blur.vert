
#version 450

precision highp float;

layout(set = 0, binding = 0) uniform Camera {
	mat4 project;
	mat4 view;
	vec2 sdf_tex_size; // sdf2： 索引纹理尺寸
};

layout(location = 0) in vec2 position;

layout(location = 1) in vec4 box_layout;  // 0 布局位置（offset， scale）
layout(location = 2) in vec4 uv;    // 16 uv 像素单位
layout(location = 3) in vec2 textureSize; // 32 纹理尺寸
layout(location = 4) in float blurRadius;  // 40 blurRadius
layout(location = 5) in float direcition; // 44 方向 当为0.0时表示水平方向， 为1.0时为垂直方向


// 输出
layout(location = 0) out vec2 vUv; // 当前点的uv坐标
layout(location = 1) out vec2 vOffsetScale; // 单个像素的uv偏移
layout(location = 2) out vec2 vGaussCoefficients; // 高斯模糊系数（由顶点着色器根据模糊半径算出）
layout(location = 3) out vec4 uvRect; // uv矩形范围(0~1)
layout(location = 4) out float support; // 采样像素个数（大致为模糊半径的两倍，因为需要左右对称）

// 根据半径计算高斯函数最大值（该最大值）
// 高斯模糊公式： f(x) = 1/(√2πσ²) * e^-x²/2σ²
// 公式中存在两个常量 A: 1/(√2πσ²), B: e^-/2σ²
// 当x = 0时， f(0) = A*B^0 = A,         令C0= B
// 当x = 1时， f(1) = A*B^1 = f(0) * C0, 令C1= C0 * B² = B^3
// 当x = 2时， f(2) = A*B^4 = f(1) * C1, 令C2= C1 * B² = B^5
// 当x = 3时， f(3) = A*B^9 = f(2) * C2, 令C3= C2 * B² = B^7
// 当x = 4时， f(4) = A*B*16= f(3) * C3  令C4= C3 * B² = B^9....
void calculate_gauss_coefficients(float sigma) {
	vGaussCoefficients = vec2(1.0 / (sqrt(2.0 * 3.14159265) * sigma),
                              exp(-0.5 / (sigma * sigma)));

	// x: A, y: B, z: B²
    vec3 gauss_coefficient = vec3(vGaussCoefficients,
                                  vGaussCoefficients.y * vGaussCoefficients.y);
	
	// 积分 对覆盖到的像素权重求和
    float gauss_coefficient_total = gauss_coefficient.x;

	// int support = int(ceil(support));
    for (float i = 1.0; i <= 300.0; i += 1.0) {
		if (i > support) {
			break;
		}
        gauss_coefficient.xy *= gauss_coefficient.yz;
		// 除x=0的像素，其他像素都是对称的，所以乘2
        gauss_coefficient_total += 2.0 * gauss_coefficient.x;
    }

	// 求x=0是的权重值（原权重/总权重，因为求值过程是乘的操作，因此用该值重复上面的计算，可得到其他像素的新权重）
    vGaussCoefficients.x = vGaussCoefficients.x/gauss_coefficient_total;
}

// 着色器入口
void main(void) {
	// support = blurRadius;
	support = ceil(blurRadius);
    if (support > 0.0) {
		// 3σ处，权重几乎为0, 而在高斯模糊中，采样半径之外的地方，权重为0
		// 因此， 近似的认为，3σ = (blurRadius + 0.5)， σ = (blurRadius + 0.5) / 3.0；
        calculate_gauss_coefficients((blurRadius + 0.5) / 3.0); 
    } else {
		// support不大于0，则默认为1.0
        vGaussCoefficients = vec2(1.0, 1.0);
    }


	// 计算一个像素在纹理上的uv偏移（0~1的数）
	vOffsetScale = vec2(0.0, 0.0);

	if (direcition < 0.5) {
		vOffsetScale = vec2(1.0 / textureSize.x, 0.0);
	} else {
		vOffsetScale = vec2(0.0, 1.0 / textureSize.y);
	}

	// 像素单位计算到uv单位，因为是采用线性采样，为防止溢出，这里会有一定偏移
	uvRect = uv;

	// 计算uv
	float p_index = position.x + position.x + position.y;// 得到索引位置0或1或2或3（left_top = 0, left_bottom = 1, right_top = 2, right_bottom = 3）
	vec4 select = vec4(
		step(p_index, 0.1),                      // 小于0.1， 只可能是0.0
		step(0.9, p_index) * step(p_index, 1.1), // 0.9 < p_index <= 1.1, 只可能是1.0, 
		step(1.9, p_index) * step(p_index, 2.1), // 1.9 < p_index <= 2.1， 只可能是2.0, 
		step(2.9, p_index)                       // > 2.9, 只肯可能是3.0
	);
	vUv = uv.xy * select.x
		+ uv.xw * select.y
		+ uv.zy * select.z
		+ uv.zw * select.w;
	

    gl_Position = vec4((position.xy * box_layout.zw + box_layout.xy) * vec2(2.0, -2.0) + vec2(-1.0, 1.0), 0.0, 1.0); // 变换到-1.0~1.0
}