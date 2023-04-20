## 运行quad demo： `cargo run --example quad`

## 检查重复库 
* 安装工具 `cargo install --locked cargo-deny`
* 查看重复库 `cargo deny check bans 2>a.txt`

## 测试性能（chrome://tracing）
例： `cargo run --example cmd_play --release --features trace`

## 

## 编译为wasm

1. `set RUST_LOG=info`
2. `set RUSTFLAGS=--cfg=web_sys_unstable_apis`
3. 根据需求编译
    + 构建： `cargo build --target wasm32-unknown-unknown`
    + 编译release版本： `wasm-pack build --release  --target web --out-dir pkg_release --out-name gui`
	+ 编译profiling版本： `wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name gui`
	+ 编译debug版本： `wasm-pack build --debug  --target web --out-dir pkg_pdebug --out-name gui`
4. 编译为pi可用的wasm：wasm_engine中执行编译脚本



## 测试gui性能 
+ 利用feature屏蔽掉info一下的日志（性能将大幅度提升， 以layout为例，可以从400ms降为10ms）
+ 谨慎使用bevy command的**insert_or_spawn_batch**方法， 可能进入巨大的性能陷阱
+ 某些场景smallvecmap代替vecmap（calc_background_image system从48ms降低至35ms）
+ 利用par_iter, 充分并行任务
+ background_image, text等系统尽量并行


## TODO
### 待做
+ fbo分配， 增加padding（已知项目有黑线问题）
+ sdf文字
+ 文字阴影
+ overflow优化： 如果一个设置了overflow的旋转节点，相对于父上下文未旋转，则该节点不需要成为一个renderPass
+ 旋转时的抗锯齿
+ 远程调试工具
+ 文字异步渲染
+ dyn_uniform_buffer， 未使用的buffer进入到资源管理器进行回收
+ uniform_buffer 动静分离 LRU
+ 跑通项目
+ 文档
+ psd 加快构建速度
+ css 解析，友好的错误提示
+ 重置gui大小
+ 处理设备丢失
+ 指令录制优化
+ 压缩纹理
+ 实现transition
+ style支持属性：cache（可以缓冲fbo）
+ mask-image
+ 高斯模糊


### 无方案
+ 合并渲染
+ 支持伪类

### 误区
+ 层脏的mark使用bitvec？（不合理， mark中需要记录层）


文档

依赖库去重
thread 'Default-Single-Worker' panicked at 'Error in Surface::configure: Both `Surface` width and height must be non-zero. Wait to recreate the `Surface` until the window has non-zero area.
transform数据结构修改
vue: 事件监听，可以在模板上阻止默认行为，阻止冒泡等


panicked at 'wgpu error: Validation Error

Caused by:
    In Device::create_render_pipeline
      note: label = `ColorEffect`
    Downlevel flags BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED are required but not supported on the device.
This is not an invalid use of WebGPU: the underlying API or device does not support enough features to be a fully compliant implementation. A subset of the features can still be used. If you are running this program on native and not in a browser and wish to work around this issue, call Adapter::downlevel_properties or Device::downlevel_properties to get a listing of the features the current platform supports


### 仙书

已处理：
1. log，JSON.stringify修改为formatJson
2. old gui, 用csspaser解析css、调整样式标记组件（使用bivec）， 样式设置延时，class样式设置不在使用match的方式设置，直接根绝样式类型索引设置（宏展开）；导出接口方面，与新的gui保持一致
3. 新的gui支持background-image-repeat、border-image表现调整为与浏览器一致（边缘处，slice与border-width不相等时，对原图边缘进行一定缩放；中心部分，fill时根据边缘的repeat属性调整fill的行为）
4. 老的gui支持background-image-repeat、border-image表现调整为与浏览器一致（边缘处，slice与border-width不相等时，对原图边缘进行一定缩放；中心部分，fill时根据边缘的repeat属性调整fill的行为）



未处理
1. 资源加载（imagemap 缩略图）
2. 构建系统，连续改文件，每个任务都会排队执行
3. 新老gui fbo合成问题（新的gui不需要合成器，老的gui需要） 
	+ 方案1： 直接在老的gui底层使用一个单独的fbo渲染gui， 并在底层负责直接将该离屏fbo渲染到canvas（不可行，gui可能做后处理效果，目前后处理是在js层完成，js层需要关心该fbo）
	+ 方案2： 
4. 优化： 旧的gui， 文字脏了，导致布局脏，但布局后未清除脏（文字字形system需要用到），由于calc_layout,calc_geo等方法的存在，可能导致多次布局

项目修改
1. set_default_style 参数改为字符串
2. 



3. 后处理， draw_final, src最好是所有权，否则会有生命周期问题
4. 后处理， calc colorstate 最好是传引用
5. 后处理， [Bgra8Unorm] != [Rgba8UnormSrgb]
6. 后处理，min_uniform_buffer_offset_alignment问题
7.  后处理，hsi问题
8.  PostProcessGeometryManager、PostProcessMaterialMgr实现default
9.  后处理，Attempted to use texture (5, 1, Vulkan) mips 0..1 layers 0..1 as a combination of COLOR_TARGET within a usage scope.
10. map_reduce必须事先指定任务数量的容量（但实际应用中，不容易事先确定任务数量）


11. start方法， isloop和ELoopMode::Not重复


# 动画TODO
iterator_count 支持浮点数和负数

