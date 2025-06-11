#version 450

#define SHADER_NAME fragment:Final

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_uv;

layout(location = 0) out vec2 vUV;

void main() {
    gl_Position = vec4(a_position * 2., 0., 1.);
    vUV = a_uv;
}