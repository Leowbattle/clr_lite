/// ECMA-335 II.23.2.1
use std::io;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDefSig {
	pub instance: bool,
	pub explicit_this: bool,
	pub vararg: bool,
	pub num_generic_args: u32,
	pub return_type: ElementType,
	pub params: Box<[ElementType]>,
}

pub trait ReadMethodDefSig {
	fn read_method_def_sig(&mut self) -> io::Result<MethodDefSig>;
}

impl ReadMethodDefSig for BlobReader<'_> {
	fn read_method_def_sig(&mut self) -> io::Result<MethodDefSig> {
		let _length = self.read_compressed_u32()?;

		let next = self.reader.read::<u8>()?;
		let instance = next & 0x20 == 0x20;
		let explicit_this = next & 0x40 == 0x40;
		let vararg = next & 0x5 == 0x5;
		let generic = next & 0x10 == 0x10;

		let num_generic_args = if generic {
			self.read_compressed_u32()?
		} else {
			0
		};

		let param_count = self.read_compressed_u32()? as usize;

		let return_type = self.read_element_type()?;

		let mut params = Vec::with_capacity(param_count);
		for _ in 0..param_count {
			params.push(self.read_element_type()?);
		}

		Ok(MethodDefSig {
			instance,
			explicit_this,
			vararg,
			num_generic_args,
			return_type,
			params: params.into_boxed_slice(),
		})
	}
}
