## 重用方式
1. gui中的根节点，可作为asimage重用节点，渲染到其他上下文中， 初为根节点外的其他gui节点， 不可作为asimage重用节点
2. 其他系统（gui以外的系统），也可将节点作为asimage重用节点
3. 用户调用`to_asimage_url(id: Entity) -> Option<String>`接口，如果id是gui的根节点，则返回格式为`asimage:://${id.index}v${id.version}`的字符串
4. 用户在background-image、border-image、mask-image可设置图片路径的地方，使用`asimage:://${id.index}v${id.version}`字符串，即可将id对应的节点的渲染结果加载到对应组件中使用
5. asimage重用节点需包含以下组件：
   * pi_bevy_render_plugin::as_image_url::RenderTarget, 重用节点的渲染结果
   * pi_bevy_render_plugin::render_cross::GraphId, 重用节点使用哪个图节点来渲染，因为`asimage:://${id.index}v${id.version}`引用的是rendertarget， 需要添加图依赖关系，确定渲染顺序，（asimage需要先渲染， 引用方后渲染）

## 实现
1. 使用`asimage:://${id.index}v${id.version}`路径的节点， 下面称`引用节点`， `${id.index}v${id.version}`对应的节点， 下面称`被引用节点`
2. gui中提供接口`to_asimage_url(id: Entity) -> Option<String>`，返回引用url（需要检查是否为根节点， 非根节点返回None）
3. 定义组件`AsImageBindList`, 存放`被引用节点`的`GraphId` 与 `引用节点`所在渲染图节点的依赖关系（因为一个节点的`BackgrounImageTexture`或 `BorderImageTexture`或 `MaskTexture`可能都使用了`asimage`, 所以AsImageBindList存放的是数组）
    ```rust
        // asimage依赖关系
        #[derive(Debug, Component, PartialEq, Eq)]
        pub struct AsImageBind {
            pub before_entity: Entity, // 绑定实体
            pub before_graph_id: GraphId, /*新的graphId */
            pub old_before_graph_id: GraphId, /*旧的graphid*/
            pub obj_type: RenderObjType, // 渲染类型(背景图片， border图片， mask图片)
            pub after_graph: NodeId, // 后续绑定图节点
        }

        // 一个节点可能存在多个引用关系， 所以是数组
        // SmallVec<AsImageBind>中不存在重复的引用关系
        #[derive(Debug, Component, Default)]
        pub struct AsImageBindList(pub SmallVec<[AsImageBind; 1]>);
    ```

4. 定义单例`AsImageRefCount`，用于存放图依赖`边`的引用计数
    ```rust
        // 引用计数
        // 如节点1 使用`asimage:://节点3`作为图片路径
        // 如节点2也使用`asimage:://节点3`作为图片路径
        // 节点1和节点2在同一个`图节点1`中渲染
        // 节点3在            `图节点2`中渲染
        // 在此处将记录 (`图节点2`, `图节点1`) -> 2
        #[derive(Debug, Default)]
        pub struct AsImageRefCount(pub XHashMap<(NodeId, NodeId), usize>);
    ```
5. image_texture_load模块， 在加载图片时， 判断是否为`asimage:://`协议， 如果是， 尝试取到`${id.index}v${id.version}`实体的`RenderTarget`和`GraphId`组件，如果未娶到， 放入等待队列， 在下一帧继续尝试获取， 知道获取到为止， 将`RenderTarget`设置到`BackgrounImageTexture`或 `BorderImageTexture`或 `MaskTexture`中， 将`GraphId`存入`AsImageBind`中的`before_entity`和`before_entity`
6. 添加`add_as_image_graph_depend`系统， 处理变化的`AsImageBindList`组件，
   * 添加新增的`被引用节点图id`和`引用节点所在图id`的依赖关系，并新增（`被引用节点图id`,`引用节点所在图id`）的引用计数
   * 减少解除依赖关系的（`被引用节点图id`,`引用节点所在图id`）的引用计数，如果引用计数为0， 则移除图依赖关系
   * 比如： 节点1的背景色图由`asimage:://节点1`修改为`asimage:://节点2`，就会触发`节点2图id`,`引用节点所在图id`）的引用计数增加， （`节点1图id`,`引用节点所在图id`）的引用计数减少
7. 在`user_setting`system中处理节点销毁销毁时， 检查节点上是否存在`AsImageBindList`组件， 如果存在， 减少对应的引用计数
8. 在`update_graph`system中， 处理`RenderContextMark`组件变化时
    * 如果一个节点由`有图节点`变为`无图节点`， 需要增加`旧的图节点`的(`from`,`to`)的引用计数，减少(`from`，`旧的图节点`)的引用计数
    * 如果一个节点由`无图节点`变为`有图节点`， TODO
9. 目前， 当销毁一个`被引用节点`， 但依然存在某些节点引用它， 此时渲染会错乱， TODO
 