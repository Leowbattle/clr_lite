use std::io::{self, Seek};
use std::mem;

#[derive(Copy, Clone)]
pub struct BinaryReader<'data> {
	data: &'data [u8],
	pos: usize,
}

impl<'data> BinaryReader<'data> {
	pub fn new(data: &'data [u8]) -> Self {
		Self { data, pos: 0 }
	}

	pub fn read<T: BinaryReadable>(&mut self) -> io::Result<T> {
		T::read(self)
	}

	pub fn peek<T: BinaryReadable>(&mut self) -> io::Result<T> {
		let result = T::read(self);
		self.seek(io::SeekFrom::Current(-(mem::size_of::<T>() as i64)))?;
		result
	}

	pub fn read_slice<T: BinaryReadable>(&mut self, buf: &mut [T]) -> io::Result<()> {
		let size = buf.len() * mem::size_of::<T>();
		if self.data.len() - self.pos < size {
			Err(io::Error::from(io::ErrorKind::UnexpectedEof))
		} else {
			for t in buf.iter_mut() {
				*t = self.read::<T>()?;
			}
			Ok(())
		}
	}

	pub fn read_array<T: BinaryReadable>(&mut self, count: usize) -> io::Result<Box<[T]>> {
		let mut arr = Vec::with_capacity(count);
		for _ in 0..count {
			arr.push(self.read::<T>()?);
		}
		Ok(arr.into_boxed_slice())
	}

	pub fn read_string(&mut self, length: usize) -> io::Result<String> {
		String::from_utf8(self.read_array::<u8>(length)?.into_vec())
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
	}

	pub fn read_null_terminated_string(&mut self) -> io::Result<String> {
		let null_index = self.data[self.pos..]
			.iter()
			.position(|&c| c == b'\0')
			.ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))?;
		Ok(String::from(
			std::str::from_utf8(&self.data[self.pos..self.pos + null_index])
				.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
		))
	}
}

impl Seek for BinaryReader<'_> {
	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		let pos = match pos {
			io::SeekFrom::Start(p) => Some(p as i64),
			io::SeekFrom::End(p) => (self.data.len() as i64).checked_sub(p),
			io::SeekFrom::Current(p) => (self.pos as i64).checked_add(p),
		};
		match pos {
			Some(p) if p > 0 && p < self.data.len() as i64 => {
				self.pos = p as usize;
				Ok(p as u64)
			}
			_ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
		}
	}
}

pub trait BinaryReadable: Sized {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Self>;
}

macro_rules! def_binary_readable {
	($type:ty) => {
		impl BinaryReadable for $type {
			fn read(reader: &mut BinaryReader<'_>) -> io::Result<Self> {
				const SIZE: usize = mem::size_of::<$type>();
				if reader.data.len() - reader.pos < SIZE {
					Err(io::Error::from(io::ErrorKind::UnexpectedEof))
				} else {
					let t = unsafe {
						*(reader.data[reader.pos..reader.pos + SIZE].as_ptr() as *const $type)
					};
					reader.pos += SIZE;
					Ok(t)
				}
			}
		}
	};
}

def_binary_readable!(i8);
def_binary_readable!(i16);
def_binary_readable!(i32);
def_binary_readable!(i64);
def_binary_readable!(i128);

def_binary_readable!(u8);
def_binary_readable!(u16);
def_binary_readable!(u32);
def_binary_readable!(u64);
def_binary_readable!(u128);

def_binary_readable!(isize);
def_binary_readable!(usize);

def_binary_readable!(f32);
def_binary_readable!(f64);

def_binary_readable!(char);
def_binary_readable!(bool);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_i8() {
		let mut br = BinaryReader::new(&[4, 5]);
		assert_eq!(br.read::<i8>().ok(), Some(4));
		assert_eq!(br.read::<i8>().ok(), Some(5));
		assert_eq!(br.read::<i8>().ok(), None);
	}

	#[test]
	fn test_f32() {
		let bytes = u32::to_ne_bytes(0x40490fd0);
		let mut br = BinaryReader::new(&bytes);
		assert_eq!(br.read::<f32>().ok(), Some(3.14159));
		assert_eq!(br.read::<f32>().ok(), None);
	}

	#[test]
	fn test_read_array() {
		let mut br = BinaryReader::new(&[1, 2, 3, 4]);
		let arr: &[u8] = &br.read_array::<u8>(4).unwrap();
		assert_eq!(arr, &[1, 2, 3, 4]);
	}
}
