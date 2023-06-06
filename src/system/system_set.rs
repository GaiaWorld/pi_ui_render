use bevy::prelude::SystemSet;


#[derive(Debug, Clone, Hash, SystemSet, PartialEq, Eq)]
pub enum UiSystemSet {
    Setting, // 设置
    Load,    // 加载
    // LoadFlush,
    Layout,       // 布局
    Matrix,       // 世界矩阵
    
    BaseCalc,     // 基础计算
    BaseCalcFlush,
    LifeDrawObject,      // 创建或删除DrawObject
    LifeDrawObjectFlush, // 删除的flush
    PrepareDrawObj,      // 准备渲染数据
    PrepareDrawObjFlush,
	PassMark,  // 上下文标记
    PassFlush, // 上下文刷新
    PassSetting,  // 上下文计算
	PassCalc, // 计算Pass数据
}
