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
	vec4 c = color;
	
	#ifdef VERTEX_COLOR
        // c = pow(vColor.rgb, vec3(2.2));
		c = vColor;
    #endif

	float sdf = texture(sampler2D(tex2d, samp), vUv).r;
	vec4 line = clipSdfOrSdfline[0];
	// 当前到填充边界的像素距离
	float fillSdPx = line.x * (sdf - line.y); 
	float fillOpacity = clamp(fillSdPx + 0.5, 0.0, 1.0);

	float outlineOpacity = 0.0;

	#ifdef STROKE
		// 填充与边框混合
		c.rgb = mix(strokeColorOrURect.rgb, c.rgb, fillOpacity);

		// 当前到描边边界的像素距离
		float outlineSdPx = line.x * (sdf - line.z);
		outlineOpacity = clamp(outlineSdPx + 0.5, 0.0, 1.0);
	#endif

	float a = clamp(outlineOpacity + fillOpacity, 0.0, 1.0);
	o_Target = vec4(c.rgb, a * c.a);
}