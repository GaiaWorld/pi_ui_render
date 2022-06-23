#version 450

// uv position
layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

// 顶点色
#ifdef VERTEX_COLOR
	layout(location = 2) in vec2 color;
#endif

// 矩阵
layout(set = 0, binding = 0) uniform ProjectMatrix {
	mat4 projectMatrix;
};
layout(set = 1, binding = 0) uniform ViewMatrix {
	mat4 viewMatrix;
};
layout(set = 2, binding = 0) uniform Matrix {
	mat4 worldMatrix;
};

// 深度
layout(set = 3, binding = 0) uniform Depth {
	float depth;
};

// 纹理尺寸
// 由于纹理纹理的尺寸会发生改变，一旦改变，每个文字的uv会随之而变
// 如果纹理尺寸作为uniform传入着色器，文字uv采用绝对像素的方式描述，由着色器算出最终的uv
// 当纹理尺寸发生改变时，每个文字渲染只需要修改TextureSize即可（TextureSize）是所用文字共用的，而无须再次为每个文字创建不同的uv buffer
layout(set = 6, binding = 0) uniform TextureSize {
	vec2 texture_size;
};

layout(location = 0) out vec2 vUv;

#ifdef VERTEX_COLOR
	layout(location = 1) out vec4 vColor;
#endif

void main() {
	gl_Position = projectMatrix * viewMatrix * worldMatrix * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth;

	vUv = uv/texture_size;
#ifdef VERTEX_COLOR
	vColor = color;
#endif
}