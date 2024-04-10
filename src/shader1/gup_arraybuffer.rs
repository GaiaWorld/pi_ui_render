// use super::GpuBuffer;

// pub struct GpuBufferArray {
//     buffers: Vec<GpuBuffer>, // 16
//     alignment: usize, 
//     limit: usize,
// }

// impl GpuBufferArray {
//     /// 创建
// 	pub fn new(alignment: usize, capacity: usize, limit: usize) -> Self {
//         let mut buffers = Vec::new();
//         for i in 0..capacity/limit {
//             buffers.push(GpuBuffer::new(alignment, limit));
//         }
//         let r = capacity%limit;
//         if r > 0 {
//             buffers.push(GpuBuffer::new(alignment, r));
//         }
// 		GpuBufferArray {
//             buffers: Vec::default(),
//             alignment,
//             limit,
// 		}
// 	}

// 	pub fn clear(&mut self) {
//         for i in self.buffers.iter_mut() {
//             i.clear();
//         }
// 	}

// 	pub fn update_dirty_range(&mut self, range: Range<usize>) {
// 		if range.start < self.dirty_range.start {
// 			self.dirty_range.start = range.start;
// 		}

// 		if self.dirty_range.end.is_null() || range.end > self.dirty_range.end {
// 			self.dirty_range.end = range.end;
// 		}
// 	}

// 	/// 如果data的长度不足（小于cur_index,则对data进行扩容)
// 	pub fn reserve(&mut self) {
//         for i in 0..self.buffers.len() {
//             self.buffers[i].reserve();
//         }
// 	}

// 	/// 分配一个实例的索引
// 	pub fn alloc_instance_data(&mut self) -> InstanceIndex {
// 		let ret = self.cur_index;
// 		self.cur_index += self.alignment;
// 		self.update_dirty_range(ret..self.cur_index);
// 		self.reserve();
// 		// log::warn!("alloc_instance_data=============={:?}", ret);
// 		ret
// 	}

// 	/// 分配指定数量的实例数据
// 	pub fn alloc_instance_data_mult(&mut self, count: usize) -> Range<usize> {
// 		let ret = self.cur_index;
// 		self.cur_index += self.alignment * count;
// 		self.update_dirty_range(ret..self.cur_index);
// 		self.reserve();
// 		// log::warn!("alloc_instance_data_mult=============={:?}", ret..self.cur_index);
// 		ret..self.cur_index
// 	}

// 	/// 引用一个实例数据
// 	#[inline]
// 	pub fn instance_data_mut(&mut self, index: InstanceIndex) -> InstanceData {
// 		InstanceData {
// 			index,
// 			data: self
// 		}
// 	}

// 	/// 在cur_index索引之后扩展片段
// 	#[inline]
// 	pub fn extend(&mut self, slice: &[u8]) {
// 		debug_assert_eq!(slice.len() % self.alignment, 0);
// 		self.reserve();
// 		self.data.extend_from_slice(slice);

// 		self.cur_index += slice.len();
// 	}

// 	// 为该实例设置数据
// 	pub fn set_data(&mut self, index: usize, value: &[u8]) {
// 		// 在debug版本， 检查数据写入是否超出自身对齐范围
// 		debug_assert!((value.byte_len() as usize + index) > self.data.len());
// 		let d = self.data.as_mut_slice();
// 		for i in 0..value.len() {
// 			d[i] = value[i];
// 		}

// 		// value.write_into(self.index as u32, &mut self.data.data);
// 		log::trace!("byte_len1========={:?}", value.byte_len());
// 		self.update_dirty_range(index..index + value.len());
// 	}

// 	/// 在cur_index索引之后扩展片段
// 	#[inline]
// 	pub fn extend_count(&mut self, count: usize) {
// 		self.cur_index += count * self.alignment;
// 		self.reserve();
// 	}

// 	#[inline]
// 	pub fn extend_to(&mut self, index: usize) {
// 		if self.cur_index < index {
// 			self.cur_index = index;
// 			self.reserve();
// 		}
		
// 	}

// 	#[inline]
// 	pub fn slice(&self, range: Range<usize>) -> &[u8] {
// 		&self.data[range]
// 	}

// 	/// 当前索引
// 	pub fn cur_index(&self) -> usize {
// 		self.cur_index
// 	}

// 	/// 当前索引
// 	pub fn capacity(&self) -> usize {
// 		self.data.capacity()
// 	}

// 	/// 下一个索引
// 	pub fn next_index(&self, index: InstanceIndex) -> usize {
// 		index + self.alignment
// 	}

// 	pub fn data(&self) -> &[u8] {
// 		&self.data
// 	}
// }
