/// ECMA-335 II.23.2.8
use super::*;

use crate::metadata::{tables::*, TypeDefOrRef};

pub trait ReadTypeDefOrRef {
	fn read_type_def_or_ref(&mut self) -> io::Result<TypeDefOrRef>;
}

impl ReadTypeDefOrRef for BlobReader<'_> {
	fn read_type_def_or_ref(&mut self) -> io::Result<TypeDefOrRef> {
		let val = self.read_compressed_u32()? as usize;
		let tag = val & 0b11;
		let index = (val & !0b11) >> 2;
		match tag {
			0x0 => Ok(TypeDefOrRef::TypeDefHandle(TypeDefHandle(index))),
			0x1 => Ok(TypeDefOrRef::TypeRefHandle(TypeRefHandle(index))),
			0x2 => Ok(TypeDefOrRef::TypeSpecHandle(TypeSpecHandle(index))),
			_ => unreachable!(),
		}
	}
}
