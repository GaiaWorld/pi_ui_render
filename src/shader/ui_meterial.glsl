

layout(set=2,binding=0) uniform UiMaterial {
	mat4 world;
	// 扇形 SDF 信息
	// [
		//    vec3 (布局中心.x, 布局中心.y, 布局缩放.x)
		//    vec3 (布局缩放.y, sin(对称轴-y轴), cos(对称轴-y轴))
		//    vec3 (sin(边缘-对称轴), cos(边缘-对称轴), r)
	// ]
	mat4 clipSdfOrSdfline;// border_radius | ellipse | circle | sector | rect | border | [0](x: 0~1的sdf代表的像素值 * 缩放值， y: 填充边界的sdf(0~1), z: 描边边界的sdf(0~1), w: 模糊半径) 还用于描述u_gradient（渐变颜色）
	// 模糊半径（阴影使用）
	float blur;
	// 如果是渲染文字，表示文字纹理大小， 如果渲染边框，表示边框下左两个值（clipSdf中不够放）
	// 纹理尺寸
	// 由于纹理纹理的尺寸会发生改变，一旦改变，每个文字的uv会随之而变
	// 如果纹理尺寸作为uniform传入着色器，文字uv采用绝对像素的方式描述，由着色器算出最终的uv
	// 当纹理尺寸发生改变时，每个文字渲染只需要修改TextureSize即可（TextureSize）是所用文字共用的，而无须再次为每个文字创建不同的uv buffer
	vec2 textureSizeOrBottomLeftBorder;
	// 如果渲染文字，表示文字颜色，如果渲染纯色矩形，表示矩形颜色
	vec4 color;
	// 如果渲染阴影，表示阴影渲染矩形。 xy是矩形最小点的坐标，zw是矩阵最大点的坐标；注：矩形必须排除阴影半径。
	// 如果渲染文字，该字段为文字的描边颜色
	// 如果渲染文字阴影，该字段的xy为阴影的h、v
	vec4 strokeColorOrURect;

	// sdf2文字额外需要的字段
	vec4 u_weightAndOffset;
	vec4 u_gradientStarteEnd;
	vec2 data_tex_size;
	vec2 slope;
	vec2 scale;
};
