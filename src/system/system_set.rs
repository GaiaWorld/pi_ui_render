use bevy_ecs::prelude::SystemSet;


#[derive(Debug, Clone, Hash, SystemSet, PartialEq, Eq)]
pub enum UiSystemSet {
    Setting, // 用户指令设置设置
    NextSetting, // 动画， 加载等内部设置
    // Load,    // 加载
    // LoadFlush,
    Layout, // 布局
    Matrix, // 世界矩阵

    BaseCalc, // 基础计算
    BaseCalcFlush,
    LifeDrawObject,      // 创建或删除DrawObject
    LifeDrawObjectFlush, // 删除的flush
    PrepareDrawObj,      // 准备渲染数据
    PassMark,    // 上下文标记
	PassLife,    // 上下生命周期相关（创建、删除上下文）
    PassFlush,   // 上下文刷新
    PassSetting, // 上下文计算(此时设置Pass， 与Pass的父子关系无关)
	PassSettingWithParent, // 上下文计算(此时设置Pass，依赖于Pass的父子关系)
    PassCalc,    // 计算Pass数据
}
