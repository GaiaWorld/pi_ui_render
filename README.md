
TODO
样式默认值
文字阴影
文字异步渲染
渲染管线异步
圆角锯齿
调试工具
跑通项目
ecs支持foreach
文档
新老版本都支持 background-image-repeat、border-image,fill修改为repeat行为
接入动画（动画运行system、动画css解析，js层兼容，构建系统兼容）



1. 后处理， draw_final, src最好是所有权，否则会有生命周期问题
2. 后处理， calc colorstate 最好是传引用
3. 后处理， [Bgra8Unorm] != [Rgba8UnormSrgb]
4. 后处理，min_uniform_buffer_offset_alignment问题
5. 后处理，hsi问题
6. PostProcessGeometryManager、PostProcessMaterialMgr实现default
7. 后处理，Attempted to use texture (5, 1, Vulkan) mips 0..1 layers 0..1 as a combination of COLOR_TARGET within a usage scope.
8. map_reduce必须事先指定任务数量的容量（但实际应用中，不容易事先确定任务数量）


1. start方法， isloop和ELoopMode::Not重复

