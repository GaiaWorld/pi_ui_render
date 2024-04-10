#version 450

#import super::ui_meterial
#import super::sdf

layout(location = 0) in vec2 vVertexPosition;

// 输入uv
layout(location = 1) in vec2 vUv;
#ifdef VERTEX_COLOR
	layout(location = 2) in vec4 vColor;
#endif

// 输出颜色
layout(location = 0) out vec4 o_Target;

// set 3
layout (set = 3, binding = 0) uniform sampler samp;
layout (set = 3, binding = 1) uniform texture2D tex2d;

void main() {
	// vec3 c = pow(color.rgb, vec3(2.2)) ; // 还原到线性空间(因为用户设置的颜色值应该是用户感知到的颜色值，为srgb空间的颜色值)
	vec3 c = color.rgb;
	float a = color.a;
	
	#ifdef VERTEX_COLOR
        // c = pow(vColor.rgb, vec3(2.2));
		a = vColor.a;
		c = vColor.rgb;
    #endif
	vec3 outlineColor = c;

	vec4 samp = texture(sampler2D(tex2d, samp), vUv);
    // samp.b < 1时，sample.b参与颜色计算，否则边缘过度部分可能出现锯齿
    // c.rgb = c.a * alpha * (samp.r * strokeColorOrURect.rgb + samp.g * c.rgb);
	// g表示填充分量
	// c.rgb = samp.g * c.rgb;

	// r表示描边分量
	#ifdef STROKE
		outlineColor = strokeColorOrURect.rgb;
	#endif
	// float ratio = 1.0/(samp.g + samp.b + 0.0001);

	
	// 应该 c.a = 1.0 - samp.b, 由于纹理坐标误差， 导致采样到纹理的空白处（rgba都为0）， 会看到一条黑线
	// b表示背景分量
	a = a * clamp(samp.a - samp.b, 0.0, 1.0);
	
	c = ((c * samp.g)  + (samp.r * outlineColor)); // rgb: (0, 255, 0); samp.g = 95; samp.r = 160; ret = (0, 182, 0 )
	// vec3 aa = color.rgb * samp.g;  //(0, 95, 0 )
	// c.rgb = samp.r * outlineColor.rgb;  (0, 160, 0 )
	// o_Target = c;
	// o_Target = vec4(0.1137, 0.0863, 0.0863, 1.0);
	o_Target = vec4(c, a); // vec4(c.r, c.g, c.b, 1.0);

	// o_Target = vec4(1.0, 0.0,0.0,1,0);
}