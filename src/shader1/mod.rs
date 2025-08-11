use std::{mem::transmute, ops::Range};

use ordered_float::NotNan;
use parry2d::transformation::utils::transform;
use pi_null::Null;
use pi_render::rhi::shader::{GetBuffer, WriteBuffer};

use crate::shader1::batch_meterial::DepthUniformMut;

use self::batch_meterial::TyMeterialMut;

// pub mod text_sdf2;
pub mod batch_meterial;
pub mod gup_arraybuffer;
pub mod batch_gauss_blur;
pub mod batch_sdf_gray;
pub mod batch_sdf_glow;

pub type InstanceIndex = usize;

// const UNUSE_START: usize = (std::u32::MAX / 2) as usize;

// 渲染实例数据
#[derive(Clone, Debug)]
pub struct GpuBuffer {
	pub data: Vec<u8>,
	// pub unused: BlockAlloter, // 未使用的数据（display为none， 或不在树上的渲染数据， 拷贝到此处）
	pub alignment: usize,
	pub cur_index: usize,

	pub dirty_range: Range<usize>,
}

impl GpuBuffer {
	// const UNUSE_BLOCK_SIZE: usize = 64;
	pub fn get_render_ty(&self, index: u32) -> usize {
		let mut uniform_data = [0.0];
		let mut uniform = TyMeterialMut(uniform_data.as_mut_slice());
		uniform.get_data(index, &self.data);

		uniform_data[0] as usize
	}

	pub fn get_depth(&self, index: u32) -> f32 {
		let mut uniform_data = [unsafe { NotNan::new_unchecked(0.0 as f32) }];
		let mut uniform = DepthUniformMut(uniform_data.as_mut_slice());
		uniform.get_data(index, &self.data);

		*uniform_data[0]
	}

	/// 创建
	pub fn new(alignment: usize, capacity: usize) -> Self {
		Self {
			data: Vec::with_capacity(capacity),
			// unused: BlockAlloter::new(alignment, Self::UNUSE_BLOCK_SIZE, Self::UNUSE_BLOCK_SIZE),
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

		// for i in 0..slice.len() / 240 {
		// 	let offset = i * 240 + 180;
		// 	let rrr = &slice[offset..offset + 4];
		// 	let f32rr: &[f32] = bytemuck::cast_slice(rrr);
		// 	// if f32rr[0] == 4195392.0 {
		// 		if i + self.cur_index/240 == 64 || start_index.start / 240 + i == 64 || i + self.cur_index/240 == 65 || start_index.start / 240 + i == 65{
		// 		pi_print_any::out_any!(log::error, "extend==={:?}, src: {:?}, dst: {:?}", f32rr, i + self.cur_index/240, start_index.start / 240 + i );
		// 	}
		// }
		

		// if self.cur_index / 240 <= 64 && slice.len() / 240 + self.cur_index/ 240 > 64{
		// 	let start = (64 - self.cur_index / 240) * 240 + 180;
		// 	let rrr = &slice[start..start + 4];
		// 	let f32rr: &[f32] = bytemuck::cast_slice(rrr);
		// 	pi_print_any::out_any!(log::error, "extend1==={:?}, src: {:?}, dst: 64", f32rr, (64 - self.cur_index / 240) + start_index.start/240);
		// }

		// if start_index.start/240 <= 64 && start_index.end/240 > 64  {
		// 	let start = (64 - start_index.start/240) * 240 + 180;
		// 	let rrr = &slice[start..start + 4];
		// 	let f32rr: &[f32] = bytemuck::cast_slice(rrr);
		// 	pi_print_any::out_any!(log::error, "extend2==={:?}, src: 64, dts: {:?}", f32rr, (64 - start_index.start/240) + self.cur_index / 240);
		// }
		

		self.cur_index += slice.len();
	}

	// 为该实例设置数据
	pub fn set_data(&mut self, index: usize, value: &[u8]) {
		let len = value.len();
		// if index >= UNUSE_START {
		// 	// 设置未使用的数据
		// 	index = index - UNUSE_START;
		// 	unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), self.unused.get_mut(index).as_mut_ptr().add(index), len) }
		// } else {
			// 在debug版本， 检查数据写入是否超出自身对齐范围
			debug_assert!((len as usize + index) > self.data.len());
			
			unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), self.data.as_mut_ptr().add(index), len) }
			// let d = self.data.as_mut_slice();
			// for i in 0..value.len() {
			// 	d[i] = value[i];
			// }
			// value.write_into(self.index as u32, &mut self.data.data);
			log::trace!("byte_len1========={:?}", value.byte_len());
			self.update_dirty_range(index..index + value.len());
		// }
	}

	// 连续设置相同的buffer到多个实例
	pub fn set_data_mult<T: WriteBuffer>(&mut self, mut index: usize, count: usize, value: &T) {
		let end = index + count * self.alignment;
		while index < end {
			self.instance_data_mut(index).set_data(value);
			index += self.alignment;
		}
	}

	// 连续设置相同的buffer到多个实例
	pub fn set_data_mult1<T: WriteBuffer>(&mut self, index: Range<usize>, value: &T) {
		let mut i = index.start;
		while i < index.end {
			self.instance_data_mut(i).set_data(value);
			i += self.alignment;
		}
	}

	// 连续设置相同的buffer到多个实例(设置之前先比较)
	pub fn set_data_mult2<T: WriteBuffer + GetBuffer + PartialEq>(&mut self, index: Range<usize>, value: &T, mut old: T) {
		let mut i = index.start;
		while i < index.end {
			old.get_data(i as u32, &self.data);
			if &old != value {
				self.instance_data_mut(i).set_data(value);
			}
			i += self.alignment;
		}
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
		self.data.get_render_ty(self.index as u32)
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

	// 	if self.index == 18446744073709551615 {
	// 	// println_any!("set data========={:?}, {:?}, {:?}", self.index, value.offset(),  value);
	// 	log::warn!("byte_len========={:?}, {:?}, {:?}, {:?}, {:?}, {:?}", self.index, value.offset(), value.byte_len(), self.data.data.len(), self.data.capacity(), self.data.alignment);
	// }
		// if value.offset() == 0 {
		// 	log::warn!("set_data========={:?}, {:?}, {:?}, {:?}, {:?}, {:?}", self.index, value.offset(), value.byte_len(), self.data.data.len(), self.data.capacity(), self.data.alignment);
		// }
		#[cfg(debug_assertions)]
		if (self.index + value.offset() as usize + value.byte_len() as usize) > self.data.data.len() {
			panic!("byte_len========={:?}, {:?}, {:?}, {:?}, {:?}, {:?}", self.index, value.offset(), value.byte_len(), self.data.data.len(), self.data.capacity(), self.data.alignment);
		}
		// pi_print_any::out_any!(log::error, "set_data==={:?}, {:?}", value, (value.offset(), value.offset() + value.byte_len(), self.index));
		// if (self.index/ 240 == 64 || self.index/ 240 == 65 || self.index/ 240 == 63) && value.offset() <= 180 && value.offset() + value.byte_len() >=184 {
		// 	pi_print_any::out_any!(log::error, "set_data==={:?}, {:?}", self.index/ 240, value);
		// }
		// 在debug版本， 检查数据写入是否超出自身对齐范围
		debug_assert_eq!((value.byte_len() as usize + self.index - 1) / self.data.alignment, self.index / self.data.alignment);
		debug_assert!((self.index + value.offset() as usize + value.byte_len() as usize) <= self.data.data.len());

		value.write_into(self.index as u32, &mut self.data.data);
		// log::debug!("byte_len0========={:?}", value.byte_len(), );
		self.data.update_dirty_range(self.index..self.index + self.data.alignment);
		

		// if value.offset() <= 180 && value.offset() + value.byte_len() >=184 {
			// let start = 180 + self.index;
			// let rrr = &self.data.data[start..start + 4];
			// let f32rr: &[f32] = bytemuck::cast_slice(rrr);
			// if f32rr[0] == 12582976.0 {
			// 	pi_print_any::out_any!(println, "set_data1==={:?}, {:?}, {:?}",  self.index/ 240, value, f32rr);
			// }
		// }

		// if self.index == 64 || self.index == 65 {
			// let start = self.index;
			// let rrr = &self.data.data[start..start + value.byte_len() as usize];
			// let f32rr: &[f32] = bytemuck::cast_slice(rrr);
			// pi_print_any::out_any!(log::error, "set_data1==={:?}, {:?}, {:?}",  self.index/ 240, value, f32rr);
		// }
	}

	pub fn get_data<T: WriteBuffer + GetBuffer>(&self, value: &mut T) {
		value.get_data(self.index as u32, &self.data.data);
	}

	// 取到当前实例的索引
	pub fn index(&self) -> InstanceIndex {
		self.index
	}
}

