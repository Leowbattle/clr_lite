/// ECMA-335 II.23.2
use std::io;

use binary_reader::*;

pub mod array_shape;
pub use array_shape::*;

pub mod element_type;
pub use element_type::*;

pub mod field_sig;
pub use field_sig::*;

use crate::metadata;

pub struct BlobReader<'data> {
	reader: BinaryReader<'data>,
}

impl<'data> BlobReader<'data> {
	pub fn new(data: &'data [u8]) -> Self {
		Self {
			reader: BinaryReader::new(data),
		}
	}

	fn read_compressed_u32(&mut self) -> io::Result<u32> {
		let x = self.reader.peek_u8()?;
		if x & 0xc0 == 0xc0 {
			Ok(u32::from_be(self.reader.read::<u32>()?) - 0xc000_0000)
		} else if x & 0x80 == 0x80 {
			Ok(u16::from_be(self.reader.read::<u16>()?) as u32 - 0x8000)
		} else {
			Ok(self.reader.read::<u8>()? as u32)
		}
	}

	fn read_metadata_token(&mut self) -> io::Result<metadata::Token> {
		Ok(metadata::Token(self.read_compressed_u32()?))
	}
}
