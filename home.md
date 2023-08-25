pi_ui_render是一个以wgpu作为底层渲染接口、以rust作为编程语言、以ecs作为其编程范式的gui库；其实现了一个css的子集。

## Start

用gui绘制一个四边形：

```rust
use bevy::prelude::*;
use pi_ui_render::prelude::*;

fn main(){
	let cmd = UserCommands::default();
	let root = world.spawn(NodeBundle::default()).id();
	self.cmd.push_cmd(NodeCmd(ClearColor(CgColor::new(1.0, 1.0, 1.0, 1.0), true), root));
	self.cmd.push_cmd(NodeCmd(
		Viewport(Aabb2::new(Point2::new(0.0, 0.0), Point2::new(size.0 as f32, size.1 as f32))),
		root,
	));

    // 设置节点的宽高、位置、背景颜色
	self.cmd.set_style(root, WidthType(Dimension::Points(size.0 as f32)));
	self.cmd.set_style(root, HeightType(Dimension::Points(size.1 as f32)));
	self.cmd.set_style(root, PositionTypeType(PositionType::Absolute));
	self.cmd.set_style(root, PositionLeftType(Dimension::Points(0.0)));
	self.cmd.set_style(root, PositionTopType(Dimension::Points(0.0)));
	self.cmd.set_style(root, MarginLeftType(Dimension::Points(0.0)));
	self.cmd.set_style(root, MarginTopType(Dimension::Points(0.0)));
	self.cmd.set_style(div1, BackgroundColorType(Color::RGBA(CgColor::new(1.0, 0.0, 1.0, 1.0))));
    // 将加点添加到渲染树中（parent为Null表示该节点为根节点）
	self.cmd.append(root, EntityKey::null().0);

	let app = App::new();
	app
		.insert_resource(cmd);
        // 这里应该插入基础插件（此处省略）
		.add_plugin(UiRenderPlugin::default())
        .run();
}
```

## 样式

在上述示例中，通过`self.cmd.set_style`为节点设置样式， 除了示例中用到的样式，还支持其他很多样式，这些样式都是参考w3c的css标准实现的，详情参考[css](https://developer.mozilla.org/zh-TW/docs/Web/CSS)、[当前支持的样式]()

## 组件
pi_ui_render将设置的样式以组件的形式存储在ecs系统中，例如， `WidthType`、`HeightType`会被存储在`Size`组件中。

### 普通组件
像Zindex、Transform、BackgroundImage这类组件，通过cpu计算，将计算结果一一设置在该节点对应的其他组件上，可视为普通节点

例如`z_index系统`，读取`Zindex`组件，对全局的所有节点分配一个唯一的Zdepth，将Zdepth写入对应组件

例如`draw_object_life系统`，读取`BackgroundImage`或`Text`等组件， 为节点创建对应的`DrawObj`

### Pass组件
Pass组件是指，因为设置了该组件，该节点及其递归子节点的渲染肯可能不得不成为一个单独的渲染过程，而不是与其他节点一起渲染到同一个渲染目标上。

例如，当节点设置了`Opacity`, 需要单独分配一个fbo，将该节点及其递归子节点渲染在该fbo上，然后再将该fbo以半透明的方式渲染在屏幕上， 因为**Opacity描述的是将节点及其递归子节点的整体做半透明效果**

类似的属性还包含`Hsi`、`Blur`、`ClipPath`、`MaskImage`

还有一些属性，他们不描述一些效果，但也是对**该节点及其递归子节点**做某种操作，包含`Overflow`、`TransformWillChange`, 它们在很多时候也会被独立成为一个渲染过程，后文会详细介绍

另外还有一个组件，`AsImage`，它不用于描述将某个节点做怎样的效果，而是意图将**该节点及其递归子节点**的渲染结果缓存下来，不言而喻，设置了该组件的节点也可能成为一个单独的渲染过程

#### Pass的整体设计
要做到在最后渲染时，能够将需要单独渲染到一个单独fbo上的**节点及其递归子节点**渲染到一个单独fbo上，pi_ui_render做了以下几步：
1. 标记为Pass， 并组织好Pass的父子关系
2. 为每个Pass创建一个渲染图节点（有些些Pass可能没有，后续会解释）
3. 为每个Pass计算脏区域，计算全局脏区域
4. 根据Pass的脏区域及全局脏区域，计算每个Pass的相机、并收集该Pass下，需要渲染的DrawObj
5. 对每个Pass下的DrawObj进行排序，按顺序分配深度BindGroup
6. 运行渲染图，按照依赖关系，执行每个渲染图节点（渲染图节点为每个Pass渲染DrawObj）

## TransformWillChange
TransformWillChange设置为true的节点， 在后续重新设置**Transform**组件时，不会标记**Transform脏节拍**，**world_matrix系统**不会检测到**Transform的变化**，因此不会递归计算每个节点的**WorldMatrix**

该属性通常用于优化某个**子节点较多的子树**，其**Transform频繁变化**的情况，可节省递归计算**WorldMatrix**的时间。


```
    令：

    W   = 设置TransformWillChange前，节点的世界矩阵
    W逆 = W的逆矩阵
    PW  = 节点父节点的世界矩阵
    T   = 节点当前Tranform对应的矩阵变化
    R_W = 节点的真实世界矩阵

    因为 R_W = PW * T
    有   R_W = PW * T * W逆 * W

    令 View = PW * T * W逆
    有 R_W = View * W
```

因此， 值需要计算出节点的视图矩阵**View**，并将其设置为节点及其子节点的视图矩阵，就能在不改变节点世界矩阵的情况下让着色器计算出正确的节点位置

需要考虑的是， 节点的递归父节点路径上，也存在设置了TransformWillChange为true的节点

假设**该路径上还有一个节点也设置了TransformWillChange为true**
```
    令：

    Parent   = 该节点的设置了TransformWillChange的某个父节点
    PPW = 设置TransformWillChange前，Parent的世界矩阵
    PPW逆 = PPW的逆矩阵
    PPPW     = Parent的父节点的世界矩阵
    PPT      = Parent当前的Tranform对应的矩阵变化
    R_PPW       = Parent的真实世界矩阵
    PView    = Parent的视图矩阵
    
    有：

    PView = PPPW * PPT * PPW逆
    R_PPW = PView * PPW；

    令：
    R_PW = 父节点的真实世界矩阵

    有：
    R_PW = PView * PW；

    此时：
    R_W = R_PW * T
        = PView * PW * T
        = PView * PW * T * W逆 * W
    则：
    View = PView * PW * T * W逆
```

### transform_will_change 系统
该system递归计算TransformWillChangeMatrix（就是上面的View）。

不仅仅是设置了TransformWillChange的节点， 所有被标记为Pass的节点，都计算了TransformWillChangeMatrix（如果没有设置TransformWillChangeMatrix， 实际上是继承了父的TransformWillChangeMatrix），这么做，是因为Pass节点在计算脏区域时， 是以以包围盒计算而得， 但包围盒是根据**有可能不正确的世界矩阵**计算得到，因此每个Pass都需要知道TransformWillChangeMatrix，才能计算出真实的、在世界坐标上的脏区域。


## Overflow

Overflow用于对节点及其递归子节点的整体做矩形裁剪，但实际的裁剪区域可能不为矩形

因为存在父子Overflow的嵌套，导致最终的裁剪区域可能为一个不规则多边形。

如果对每个DrawObj，在像素着色器中做裁剪计算，可能会对Gpu造成性能压力。

pi_ui_render将Overflow分为两种情况处理。

### 情况1：Overflow及其递归父节点都没有设置旋转变换

这种情况下，裁剪区域一定为一个矩形，并且该矩形相对于世界坐标无旋转。
这种情况只需要在渲染**节点及其递归子节点**时，设置裁剪区域为渲染视口即可（利用视口剔除；当然，该区域还应该与全局视口求交）

#### 处理过程
1. 递归计算Overflow的相交区域（OverflowView），每个Pass的OverflowView都是自身Overflow区域与父OverflowView的相交区域，如果自身没有设置Overflow，则继承付的OverflowView
2. 根据OverflowView，计算相机的视口区域： 有全局脏区域与OverflowView中的
3. 


### 情况2：情况1不成立，则视为情况为

这种情况下， 正常渲染**节点及其递归子节点**，但，需要将他们渲染在一个单独的fbo上，然后在将fbo渲染到父Pass上时，单独设置视口即可。

这里需要注意的是，由于节点很大可能是旋转的， 为了减少fbo的分配尺寸，需要将节点的内容区域旋转回来，使得节点的内容区域相对世界坐标系无旋转。






