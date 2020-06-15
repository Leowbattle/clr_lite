/// ECMA-335 II.23.4.2
use std::io;

use super::*;

#[derive(Debug)]
pub struct FieldSig {
	pub custom_mod: Box<[ElementType]>,
	pub r#type: ElementType,
}

pub trait ReadFieldSig {
	fn read_field_sig(&mut self) -> io::Result<FieldSig>;
}

impl ReadFieldSig for BlobReader<'_> {
	fn read_field_sig(&mut self) -> io::Result<FieldSig> {
		let _length = self.read_compressed_u32()?;
		let kind = self.reader.read::<u8>()?;
		if kind != 0x6 {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"Expected field signature",
			));
		}

		let custom_mod = Vec::new();

		let r#type = self.read_element_type()?;

		Ok(FieldSig {
			custom_mod: custom_mod.into_boxed_slice(),
			r#type,
		})
	}
}
