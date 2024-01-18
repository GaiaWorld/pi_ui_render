use std::ops::Range;

use bevy_ecs::system::Resource;
use pi_render::rhi::shader::WriteBuffer;

pub mod text_sdf2;
pub mod meterial;

pub type InstanceIndex = usize;

// 渲染实例数据
#[derive(Clone, Debug, Resource)]
pub struct RenderInstances {
	data: Vec<u8>,
	alignment: usize,
	cur_index: usize,
}

impl RenderInstances {
	/// 创建
	pub fn new(alignment: usize, capacity: usize) -> Self {
		Self {
			data: Vec::with_capacity(capacity),
			alignment,
			cur_index: 0,
		}
	}

	/// 如果data的长度不足（小于cur_index,则对data进行扩容)
	pub fn reserve(&mut self) {
		if self.data.capacity() < self.cur_index {
			self.data.reserve(self.cur_index - self.data.capacity());
		}

		// 安全： 前一步保证了容量一定足够， 这里的操作必然是安全的
		unsafe {self.data.set_len(self.cur_index)};
	}

	/// 分配一个实例数据
	pub fn alloc_instance_data(&mut self) -> InstanceIndex {
		let ret = self.cur_index;
		self.cur_index += self.alignment;
		ret
	}

	/// 引用一个实例数据
	#[inline]
	pub fn instance_data_mut(&mut self, index: InstanceIndex) -> InstanceData {
		InstanceData {
			index,
			data: self
		}
	}

	/// 在cur_index索引之后扩展片段
	#[inline]
	pub fn extend(&mut self, slice: &[u8]) {
		debug_assert_eq!(slice.len() % self.alignment, 0);
		self.reserve();
		self.data.extend_from_slice(slice);

		self.cur_index += slice.len();
	}

	/// 在cur_index索引之后扩展片段
	#[inline]
	pub fn extend_count(&mut self, count: usize) {
		self.cur_index += count * self.alignment;
		self.reserve();
	}

	#[inline]
	pub fn slice(&self, range: Range<usize>) -> &[u8] {
		&self.data[range]
	}

	/// 当前索引
	pub fn cur_index(&self) -> usize {
		self.cur_index
	}

	/// 下一个索引
	pub fn next_index(&self, index: InstanceIndex) -> usize {
		index + self.alignment
	}
 }

pub struct InstanceData<'a> {
	index: InstanceIndex,
	data: &'a mut RenderInstances,
}

impl<'a> InstanceData<'a> {
	// 为该实例设置数据
	pub fn set_data<T: WriteBuffer>(&mut self, value: &T) {
		// 在debug版本， 检查数据写入是否超出自身对齐范围
		debug_assert_eq!(value.byte_len() as usize + self.index / self.data.alignment, self.index / self.data.alignment);

		value.write_into(self.index as u32, &mut self.data.data);
	}

	// 取到当前实例的索引
	pub fn index(&self) -> InstanceIndex {
		self.index
	}
}