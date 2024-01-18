// // 渲染颜色对象（size: 20 * 4）
// pub struct ColorData {
// 	// 世界矩阵（深度值放在世界矩阵中）
// 	world_matrix: WorldMatrix,
// 	// 圆角数据
// 	clip_sdf: [f32; 16], 

// 	// rgba颜色（背景或边框）
// 	color: CgColor,
// }

// // // 渲染普通图片（size: 20 * 4）
// // struct InstanceData {
// // 	// 世界矩阵（深度值放在世界矩阵中）
// // 	worldMatrix: mat4, 
// // 	// 裁剪需要的数据（border_radius）
// // 	clipSdf: mat4, 
// // 	// uv范围
// // 	uv: vec4, 
// // }

// // // clip
// // struct InstanceData {
// // 	// 世界矩阵（深度值放在世界矩阵中）
// // 	worldMatrix: mat4, 
// // 	// 裁剪需要的数据（ellipse | circle | sector | rect）
// // 	clipSdf: mat4, 

// // 	// 颜色或uv
// // 	color_uv: vec4,
// // }

// // // 渲染渐变颜色对象（size: 20 * 4）
// // // 如果是渐变颜色+圆角， 则为采样sdf方式渲染
// // struct InstanceData {
// // 	// 世界矩阵（深度值放在世界矩阵中）
// // 	worldMatrix: mat4, 
// // 	// colors为支持4个渐变颜色
// // 	colors: mat4, 

// // 	// 渐变颜色的渐变位置（最多四个位置）
// // 	gradientPosition: vec4,
// // }

// // // 渲染文字
// // struct InstanceData {
// // 	// 世界矩阵（深度值放在世界矩阵中）
// // 	worldMatrix: mat4, 

// // 	// 为纯颜色时，颜色放在colors[0]中， 否则为渐变颜色
// // 	colors: mat4

// // 	// 渐变颜色的渐变位置（最多四个位置）
// // 	gradientPosition: vec4,

// // 	// 索引纹理偏移， 宽高（用于计算uv）
// // 	index: vec4,
// // 	// 数据纹理偏移
// // 	data_offset: float,

// // 	u_info: vec4, // 文字信息

// // 	tranlation: vec2, // 偏移
// // 	tranlation: vec4, // 缩放
// // }