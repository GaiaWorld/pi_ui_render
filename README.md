# 1. 使用

## 1.2. 运行 Windows 平台

执行 `cargo run --example background_image` 命令运行 background_image example

## 1.2.1 调试工具
浏览器访问[调试工具](http://192.168.35.74:3000/index.html) (ip修改为自己的ip)

## 1.3. 运行 [Web 平台](https://rustwasm.github.io/docs/wasm-bindgen/contributing/testing.html)

+ 执行`wasm-pack test  --chrome --example background_image`命令，构建wasm以及测试环境，并开启服务器监听在`8000`端口 
+  在浏览器中访问`http://127.0.0.1:8000`地址，即可运行测试

## 1.4. 运行 Android 平台

+ 打开Linux Shell, 执行 `cargo apk run --example background_image --lib` 编译 example为apk
+ 链接手机 在 `target\debug\apk\examples` 中打开cmd， 并执行 `adb install background_image.apk` 来安装apk
+ 

## 运行quad demo： `cargo run --example quad`
+ 设置日志过滤， 如： $env:RUST_LOG="warn,pi_ui_render::system::node::user_setting=debug,pi_ui_render::components::user=debug,pi_ui_render=warn,pi_ui_render::system::pass_effect::opacity=trace,pi_ui_render::system::pass::pass_graph_node=trace,pi_ui_render::system::draw_obj::calc_background_color=debug,pi_ui_render::system::pass::pass_life=trace,pi_ui_render::system::draw_obj::life_drawobj=trace,pi_ui_render::system::pass::update_graph=trace"
+ tracing warn
$env:RUST_LOG="warn"
+ tracing info
+ $env:RUST_LOG="warn,pi_world=info,pi_bevy_ecs_extend=info"

	
## 测试gui性能 
+ `cargo run --example a_cmd_play --release --features tracing-chrome`
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
+ 重置gui大小
+ 处理设备丢失
+ 指令录制优化
+ 压缩纹理
+ 实现transition
+ style支持属性：cache（可以缓冲fbo）
+ mask-image
+ 高斯模糊
+ fbo缓冲和建议缓冲方案
  - 用户可设置某节点缓冲，该节点对应的子树将被渲染到一张fbo上，并缓冲，后续渲染如果发现子树未改变，则不需要再重新渲染该子树，直接将缓冲的fbo渲染到gui上
  - 用户可设置某节点建议缓冲（root组件上，打开的界面根节点都默认设置建议缓冲）
      * gui系统迭代动画动画节点， 和动画组件删除列表， 如果发现动画组件被添加或删除， 则更新其所在的fbo的AnimationCount组件的part_count（该组件记录子树的动画节点数量，此处仅包含该fbo下的普通节点，不包含其他fbo的递归子节点）
      * 迭代所有fbo，将AnimationCount组件的part_count累加到父的fbo的AnimationCount组件的all_count字段上
      * 记录脏频次在DirtyFrequency，每次发现fbo脏，将DirtyFrequency清0零，否则在原值基础上+1
      * 渲染fbo， 如果发现fbo不脏， 并且存在对应的缓冲fbo，则将缓冲fbo输出；如果发现fbo脏，则渲染fbo，如果fbo为建议缓冲，并且AnimationCount组件的all_count字段未0，则缓冲该fbo


### 无方案
+ 合并渲染
+ 支持伪类

### 误区
+ 层脏的mark使用bitvec？（不合理， mark中需要记录层）

## Android 编译
1. cargo.toml添加依赖 
```
[target.'cfg(target_os = "android")'.dev-dependencies]
ndk-glue = "0.7"

```

2. 在需要编译的用例的main函数上添加,如下

```
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
fn main(){

}
```
3. 在caogo.toml中添加如下代码
```
[[example]]
name = "a_html" # 用例名称
crate-type = ["cdylib"]
test = true
```
4. 在wsl2中使用cargo apk run --example 用例名称  将用例编译成apk; 

* 注意： wsl2搭建教程参照http://192.168.31.241:8181/docs/ops/ops-1c94vf4dk94n7; 安装镜像使用\\192.168.31.241\tech\software\wsl2\centos7.tar.gz

## gui