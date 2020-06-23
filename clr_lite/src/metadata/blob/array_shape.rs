///! ECMA-335 II.23.2.13
use super::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArrayShape {
	pub rank: usize,
	pub sizes: Box<[usize]>,
	pub lower_bounds: Box<[usize]>,
}

impl BlobReader<'_> {
	pub fn read_array_shape(&mut self) -> Result<ArrayShape, BlobReaderError> {
		Ok(ArrayShape {
			rank: self.read_compressed_u32()? as usize,
			sizes: {
				let count = self.read_compressed_u32()? as usize;
				let mut sizes = Vec::with_capacity(count);
				for _ in 0..count {
					sizes.push(self.read_compressed_u32()? as usize);
				}
				sizes.into_boxed_slice()
			},
			lower_bounds: {
				let count = self.read_compressed_u32()? as usize;
				let mut lower_bounds = Vec::with_capacity(count);
				for _ in 0..count {
					lower_bounds.push(self.read_compressed_u32()? as usize);
				}
				lower_bounds.into_boxed_slice()
			},
		})
	}
}
