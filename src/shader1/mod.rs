use std::ops::Range;

use pi_null::Null;
use pi_render::rhi::shader::{WriteBuffer, GetBuffer};

use self::meterial::TyUniformMut;

// pub mod text_sdf2;
pub mod meterial;
pub mod gup_arraybuffer;

pub type InstanceIndex = usize;

// 渲染实例数据
#[derive(Clone, Debug)]
pub struct GpuBuffer {
	pub data: Vec<u8>,
	pub alignment: usize,
	pub cur_index: usize,

	pub dirty_range: Range<usize>,
}

impl GpuBuffer {
	/// 创建
	pub fn new(alignment: usize, capacity: usize) -> Self {
		Self {
			data: Vec::with_capacity(capacity),
			alignment,
			cur_index: 0,
			dirty_range: std::usize::MAX..std::usize::MAX,
		}
	}

	pub fn clear(&mut self) {
		self.data.clear();
		self.cur_index = 0;
		self.dirty_range = std::usize::MAX..std::usize::MAX;
	}

	pub fn update_dirty_range(&mut self, range: Range<usize>) {
		if range.start < self.dirty_range.start {
			self.dirty_range.start = range.start;
		}

		if self.dirty_range.end.is_null() || range.end > self.dirty_range.end {
			self.dirty_range.end = range.end;
		}
	}

	/// 如果data的长度不足（小于cur_index,则对data进行扩容)
	pub fn reserve(&mut self) {
		if self.data.capacity() < self.cur_index {
			self.data.reserve(self.cur_index - self.data.len());
		}

		// 安全： 前一步保证了容量一定足够， 这里的操作必然是安全的
		unsafe {self.data.set_len(self.cur_index)};
	}

	/// 分配一个实例的索引
	pub fn alloc_instance_data(&mut self) -> InstanceIndex {
		let ret = self.cur_index;
		self.cur_index += self.alignment;
		self.update_dirty_range(ret..self.cur_index);
		self.reserve();
		// log::warn!("alloc_instance_data=============={:?}", ret);
		ret
	}

	/// 分配指定数量的实例数据
	pub fn alloc_instance_data_mult(&mut self, count: usize) -> Range<usize> {
		let ret = self.cur_index;
		self.cur_index += self.alignment * count;
		self.update_dirty_range(ret..self.cur_index);
		self.reserve();
		// log::warn!("alloc_instance_data_mult=============={:?}", ret..self.cur_index);
		ret..self.cur_index
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

	// 为该实例设置数据
	pub fn set_data(&mut self, index: usize, value: &[u8]) {
		// 在debug版本， 检查数据写入是否超出自身对齐范围
		debug_assert!((value.byte_len() as usize + index) > self.data.len());
		let d = self.data.as_mut_slice();
		for i in 0..value.len() {
			d[i] = value[i];
		}

		// value.write_into(self.index as u32, &mut self.data.data);
		log::trace!("byte_len1========={:?}", value.byte_len());
		self.update_dirty_range(index..index + value.len());
	}

	

	/// 在cur_index索引之后扩展片段
	#[inline]
	pub fn extend_count(&mut self, count: usize) {
		self.cur_index += count * self.alignment;
		self.reserve();
	}

	#[inline]
	pub fn extend_to(&mut self, index: usize) {
		if self.cur_index < index {
			self.cur_index = index;
			self.reserve();
		}
		
	}

	#[inline]
	pub fn slice(&self, range: Range<usize>) -> &[u8] {
		&self.data[range]
	}

	/// 当前索引
	pub fn cur_index(&self) -> usize {
		self.cur_index
	}

	/// 当前索引
	pub fn capacity(&self) -> usize {
		self.data.capacity()
	}

	/// 下一个索引
	pub fn next_index(&self, index: InstanceIndex) -> usize {
		index + self.alignment
	}

	pub fn data(&self) -> &[u8] {
		&self.data
	}
 }

pub struct InstanceData<'a> {
	pub index: InstanceIndex,
	data: &'a mut GpuBuffer,
}

impl<'a> InstanceData<'a> {
	pub fn get_render_ty(&self) -> usize {
		let mut uniform_data = [0.0];
		let mut uniform = TyUniformMut(uniform_data.as_mut_slice());
		self.get_data(&mut uniform);

		uniform_data[0] as usize
	}
}



impl<'a> std::fmt::Debug for InstanceData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstanceData").field("index", &self.index).finish()
    }
}

impl<'a> InstanceData<'a> {
	// 为该实例设置数据
	pub fn set_data<T: WriteBuffer>(&mut self, value: &T) {

		log::trace!("byte_len========={:?}, {:?}, {:?}, {:?}, {:?}, {:?}", self.index, value.offset(), value.byte_len(), self.data.data.len(), self.data.capacity(), self.data.alignment);

		if (self.index + value.offset() as usize + value.byte_len() as usize) > self.data.data.len() {
			panic!("byte_len========={:?}, {:?}, {:?}, {:?}, {:?}, {:?}", self.index, value.offset(), value.byte_len(), self.data.data.len(), self.data.capacity(), self.data.alignment);

		}
		// 在debug版本， 检查数据写入是否超出自身对齐范围
		debug_assert_eq!((value.byte_len() as usize + self.index) / self.data.alignment, self.index / self.data.alignment);
		debug_assert!((self.index + value.offset() as usize + value.byte_len() as usize) <= self.data.data.len());

		value.write_into(self.index as u32, &mut self.data.data);
		log::trace!("byte_len0========={:?}", value.byte_len());
		self.data.update_dirty_range(self.index..self.index + self.data.alignment);
	}

	pub fn get_data<T: WriteBuffer + GetBuffer>(&self, value: &mut T) {
		value.get_data(self.index as u32, &self.data.data);
	}

	// 取到当前实例的索引
	pub fn index(&self) -> InstanceIndex {
		self.index
	}
}