use std::mem::size_of;
use std::slice;

pub struct BinaryReader<'data> {
	data: &'data [u8],
	pos: usize,
}

impl<'data> BinaryReader<'data> {
	pub fn new(data: &'data [u8]) -> BinaryReader<'data> {
		BinaryReader { data, pos: 0 }
	}

	pub fn pos(&self) -> usize {
		self.pos
	}

	pub fn goto(&mut self, x: usize) -> bool {
		self.pos = x;
		self.pos <= self.data.len()
	}

	pub fn advance(&mut self, x: usize) -> bool {
		self.goto(self.pos + x)
	}

	pub fn peek<T: CopyFromBytes>(&self) -> Option<T> {
		if self.pos + size_of::<T>() >= self.data.len() {
			None
		} else {
			Some(unsafe { *(self.data[self.pos..].as_ptr() as *const T) })
		}
	}

	pub fn read<T: CopyFromBytes>(&mut self) -> Option<T> {
		let data = self.peek::<T>()?;
		self.advance(size_of::<T>());
		Some(data)
	}

	pub fn peek_array<T: CopyFromBytes>(&self, count: usize) -> Option<&'data [T]> {
		if self.pos + count * size_of::<T>() >= self.data.len() {
			None
		} else {
			Some(unsafe {
				slice::from_raw_parts(self.data[self.pos..].as_ptr() as *const T, count)
			})
		}
	}

	pub fn read_array<T: CopyFromBytes>(&mut self, count: usize) -> Option<&'data [T]> {
		if self.pos + count * size_of::<T>() >= self.data.len() {
			None
		} else {
			let data =
				unsafe { slice::from_raw_parts(self.data[self.pos..].as_ptr() as *const T, count) };
			self.advance(count * size_of::<T>());
			Some(data)
		}
	}

	pub fn read_str(&mut self, length: usize) -> Option<&'data str> {
		let mut data = self.read_array::<u8>(length)?;
		data = &data[0..data.iter().position(|&c| c == b'\0').unwrap_or(data.len())];
		std::str::from_utf8(data).ok()
	}

	pub fn read_c_str(&mut self) -> Option<&'data str> {
		let length = self.data[self.pos..].iter().position(|&c| c == b'\0')?;
		self.read_str(length)
	}
}

pub unsafe trait CopyFromBytes: Copy {}

unsafe impl CopyFromBytes for i8 {}
unsafe impl CopyFromBytes for i16 {}
unsafe impl CopyFromBytes for i32 {}
unsafe impl CopyFromBytes for i64 {}
unsafe impl CopyFromBytes for i128 {}
unsafe impl CopyFromBytes for isize {}

unsafe impl CopyFromBytes for u8 {}
unsafe impl CopyFromBytes for u16 {}
unsafe impl CopyFromBytes for u32 {}
unsafe impl CopyFromBytes for u64 {}
unsafe impl CopyFromBytes for u128 {}
unsafe impl CopyFromBytes for usize {}

unsafe impl CopyFromBytes for f32 {}
unsafe impl CopyFromBytes for f64 {}

unsafe impl CopyFromBytes for bool {}
unsafe impl CopyFromBytes for char {}
