use std::io;

use super::*;

#[derive(Debug)]
/// ECMA-335 II.22.2.13
pub struct ArrayShape {
	rank: usize,
	sizes: Box<[u32]>,
	lower_bounds: Box<[u32]>,
}

pub trait ReadArrayShape {
	fn read_array_shape(&mut self) -> io::Result<ArrayShape>;
}

impl ReadArrayShape for BlobReader<'_> {
	fn read_array_shape(&mut self) -> io::Result<ArrayShape> {
		let rank = self.read_compressed_u32()? as usize;

		let num_sizes = self.read_compressed_u32()? as usize;
		let mut sizes = Vec::with_capacity(num_sizes);
		for _ in 0..num_sizes {
			sizes.push(self.read_compressed_u32()?);
		}

		let num_lower_bounds = self.read_compressed_u32()? as usize;
		let mut lower_bounds = Vec::with_capacity(num_lower_bounds);
		for _ in 0..num_lower_bounds {
			lower_bounds.push(self.read_compressed_u32()?);
		}

		Ok(ArrayShape {
			rank,
			sizes: sizes.into_boxed_slice(),
			lower_bounds: lower_bounds.into_boxed_slice(),
		})
	}
}
