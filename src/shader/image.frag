#version 450

#import super::ui_meterial
#import super::sdf

// 输入uv
layout(location=0)in vec2 vUv;
// 输入position
layout(location=1)in vec2 vVertexPosition;

// 输出颜色
layout(location=0)out vec4 o_Target;

// 纹理
layout(set=3,binding=0)uniform sampler samp;
layout(set=3,binding=1)uniform texture2D tex2d;

void main(){
	vec4 color=texture(sampler2D(tex2d,samp),vUv);
	
	#ifdef BORDER_RADIUS
	color.a=color.a*calc_alpha(vVertexPosition, clipSdfOrSdfline);
	#endif
	
	#ifdef SECTOR
	color.a=color.a*calc_alpha(vVertexPosition, clipSdfOrSdfline);
	#endif
	
	#ifdef RECT
	color.a=color.a*calc_alpha(vVertexPosition, clipSdfOrSdfline);
	#endif
	
	#ifdef ELLIPSE
	color.a=color.a*calc_alpha(vVertexPosition, clipSdfOrSdfline);
	#endif
	
	#ifdef CIRCLE
	color.a=color.a*calc_alpha(vVertexPosition, clipSdfOrSdfline);
	#endif
	
	// 預乘模式
	o_Target=vec4(color.rgb,color.a);
}