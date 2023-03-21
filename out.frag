#version 450

precision highp float;

float antialiase(float d) 

{

	float anti = fwidth(d);

	return 1.0 - smoothstep(-anti, anti, d);

}

float sdfEllipse(vec2 pt, vec2 center, vec2 ab)

{

	pt -= center;

	vec2 recAB = 1.0 / ab;

	vec2 scale = pt * recAB;

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

float antialiase(float d) 

{

	float anti = 1.0 * fwidth(d);

	return 1.0 - smoothstep(-anti, anti, d);

}

layout(location=0)in vec2 vVertexPosition;
layout(location=0)out vec4 o_Target;
layout(set=1,binding=0) uniform M_1_0{
mat4 world;
mat4 clipSdf;
vec4 color;
vec4 strokeColorOrURect;
vec2 textureSizeOrBottomLeftBorder;
float depth;
float blur;
};
void main(){
	vec4 c = color.rgba;

	o_Target = c;

}
