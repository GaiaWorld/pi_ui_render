#version 450

layout(location=0)in vec2 position;
layout(location=0)out vec2 vVertexPosition;
layout(set=0,binding=0) uniform M_0_0{
mat4 project;
mat4 view;
};
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
	vVertexPosition = position;
	gl_Position = project * view * world * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;

}
