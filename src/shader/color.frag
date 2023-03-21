#version 450

precision highp float;

#import super::ui_meterial
#import super::sdf

layout(location = 0) in vec2 vVertexPosition;

#ifdef VERT_COLOR
	layout(location = 1) in vec4 vColor;
#endif

layout(location = 0) out vec4 o_Target;

void main() {
	vec4 c = color.rgba;

	// 顶点色
	#ifdef VERT_COLOR
		c = vColor;
	#endif

	#ifdef SHADOW
		c.a = c.a * getShadowAlpha(vVertexPosition, strokeColorOrURect.xy, strokeColorOrURect.zw, blur / 2.0);
	#endif

	#ifdef BORDER_RADIUS
		c.a = c.a * calc_alpha(vVertexPosition, clipSdf);
	#endif

	#ifdef BORDER
		c.a = c.a * calc_alpha(vVertexPosition, clipSdf);
	#endif

	o_Target = c;
	// o_Target = vec4( getShadowAlpha(vVertexPosition, uRect.xy, uRect.zw, blur / 2.0), 0.0, 0.0, 1.0);
	// o_Target = vec4(color.rgb, 0.5);
	// o_Target = vec4( getShadowAlpha(vVertexPosition, uRect.xy, uRect.zw, blur / 2.0), 0.0, 0.0, 1.0);
	// o_Target = vec4(vVertexPosition.x, vVertexPosition.y, 0.0, 1.0);
	// o_Target = vec4(uRect.x, uRect.y, uRect.z, uRect.w);
}