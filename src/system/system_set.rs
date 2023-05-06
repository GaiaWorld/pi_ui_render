use bevy::prelude::SystemSet;


#[derive(Debug, Clone, Hash, SystemSet, PartialEq, Eq)]
pub enum UiSystemSet {
	Setting, // 设置
	Load, // 加载
	LoadFlush,
	Layout, // 布局
	Matrix, // 世界矩阵
	BaseCalc, // 基础计算
	BaseCalcFlush,
	PrepareDrawOb, // 准备渲染数据
	PrepareDrawObFlush,
	PreparePass, // 准备渲染过程
	PreparePassFlush,
}