// Attributes
layout (location = 0) in vec2 position;

#ifdef VERTEX_COLOR
	layout (location = 1) in vec4 color;
#endif

// Uniforms
layout (set = 0, binding = 0) uniform mat4 viewMatrix;
layout (set = 0, binding = 1) uniform mat4 projectMatrix;

layout (set = 1, binding = 0) uniform mat4 worldMatrix;

layout (set = 2, binding = 0) uniform mat4 uColor;


void main(void) {
	vec4 p1 = viewMatrix * worldMatrix * vec4(position.x, position.y, 1.0, 1.0);
	vec4 p = projectMatrix * p1;

	gl_Position = vec4(p.x, p.y, depth, 1.0);

	#ifdef VERTEX_COLOR
		vColor = color;
	#endif
}

void main(void) {
	vec4 c = uColor;
	#ifdef VERTEX_COLOR
		c = c * vColor;
	#endif

	gl_FragColor = c;
}