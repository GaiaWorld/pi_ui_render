### 实例化渲染

#### 什么时机应该修改实例数据
1. 需要记录节点从可视范围和非可视范围之间移动， 如果移动了，需要重新组织实例数据（如果是滚动，需要每帧重新组织渲染数据？）
2. 节点创建、删除，需要重新组织实例数据（实际上也可以看做是非可视范围移动到了可视范围）
3. zIndex发生变化，需要重新组织实例数据（渲染顺序影响实例数据的顺序）

#### 实例数据
// 渲染颜色对象（size: 20 * 4）
struct InstanceData {
	// 世界矩阵（深度值放在世界矩阵中）
	worldMatrix: mat4, 
	// 圆角数据
	clipSdf: mat4, 

	// rgba颜色（背景或边框）
	color: vec4,
}

// 渲染普通图片（size: 20 * 4）
struct InstanceData {
	// 世界矩阵（深度值放在世界矩阵中）
	worldMatrix: mat4, 
	// 裁剪需要的数据（border_radius）
	clipSdf: mat4, 
	// uv范围
	uv: vec4, 
}

// clip
struct InstanceData {
	// 世界矩阵（深度值放在世界矩阵中）
	worldMatrix: mat4, 
	// 裁剪需要的数据（ellipse | circle | sector | rect）
	clipSdf: mat4, 

	// 颜色或uv
	color_uv: vec4,
}

// 渲染渐变颜色对象（size: 20 * 4）
// 如果是渐变颜色+圆角， 则为采样sdf方式渲染
struct InstanceData {
	// 世界矩阵（深度值放在世界矩阵中）
	worldMatrix: mat4, 
	// colors为支持4个渐变颜色
	colors: mat4, 

	// 渐变颜色的渐变位置（最多四个位置）
	gradientPosition: vec4,
}

// 渲染文字
struct InstanceData {
	// 世界矩阵（深度值放在世界矩阵中）
	worldMatrix: mat4, 

	// 为纯颜色时，颜色放在colors[0]中， 否则为渐变颜色
	colors: mat4

	// 渐变颜色的渐变位置（最多四个位置）
	gradientPosition: vec4,

	// 索引纹理偏移， 宽高（用于计算uv）
	index: vec4,
	// 数据纹理偏移
	data_offset: float,

	u_info： vec4, // 文字信息
}

#### 问题
节点增加或删除， 会导致实例数据重新组织，消耗较多的时间， 但某些场景确实这样（比如每帧刷新一组数字， 数字的个数不一样）





<!-- layout(set=2,binding=0)uniform UiMaterial{
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

layout (location = 5) in vec2 uv;
layout (location = 6) in vec2 lp;
layout (location = 7) in vec4 index_offset_and_size;
layout (location = 8) in vec2 u_data_offset;
layout (location = 9) in vec4 u_info; -->

### fbo discard



性能问题（创建fontface时间长， 原因未知，单独测试创建font-face并不费）
transition 功能实现（未完成）
glyph is not exist, GlyphId(DefaultKey(4294967295v0))（点击美食家这个角色，出现过， 可能是因为未处理不在树上的节点， 修改了代码，暂时未重现）
节点不销毁问题(场景中出现)
flex_layout parent不存在问题
pi_share, 多次借用问题
pi_ui_render, 插入一个节点时，已经存在一个parent
