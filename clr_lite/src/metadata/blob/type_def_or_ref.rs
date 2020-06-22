/// ECMA-335 II.23.2.8
use super::*;

use crate::metadata::tables::*;

impl BlobReader<'_> {
	pub fn read_type_def_or_ref(&mut self) -> Result<TypeDefOrRefHandle, BlobReaderError> {
		let val = self.read_compressed_u32()? as usize;
		let tag = val & 0b11;
		let index = (val & !0b11) >> 2;
		Ok(match tag {
			0x0 => TypeDefOrRefHandle::TypeDefHandle(TypeDefHandle(index)),
			0x1 => TypeDefOrRefHandle::TypeRefHandle(TypeRefHandle(index)),
			0x2 => TypeDefOrRefHandle::TypeSpecHandle(TypeSpecHandle(index)),
			_ => {
				return Err(BlobReaderError::BadBlob(format!(
					"Invalid TypeDefOrRef tag {}",
					tag
				)))
			}
		})
	}
}
