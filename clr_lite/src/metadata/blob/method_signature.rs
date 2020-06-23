///! ECMA-335 II.23.2.1
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodSignature {
	pub instance: bool,
	pub explicit_this: bool,
	pub vararg: bool,
	pub generic_param_count: usize,
	pub return_type: ElementType,
	pub params: Box<[ElementType]>,
}

impl BlobReader<'_> {
	pub fn read_method_signature(&mut self) -> Result<MethodSignature, BlobReaderError> {
		let next = self.read::<u8>()?;
		let instance = next & 0x20 == 0x20;
		let explicit_this = next & 0x40 == 0x40;
		let vararg = next & 0x5 == 0x5;
		let generic = next & 0x10 == 0x10;

		let generic_param_count = if generic {
			self.read_compressed_u32()? as usize
		} else {
			0
		};

		let param_count = self.read_compressed_u32()? as usize;

		let return_type = self.read_element_type()?;

		let params = {
			let mut params = Vec::with_capacity(param_count);
			for _ in 0..param_count {
				params.push(self.read_element_type()?);
			}
			params.into_boxed_slice()
		};

		Ok(MethodSignature {
			instance,
			explicit_this,
			vararg,
			generic_param_count,
			return_type,
			params,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashMap;

	#[test]
	fn test_method_def_signature() {
		let data = include_bytes!("../../../../tests/metadata/blob/MethodSignatureTests/bin/Debug/netcoreapp3.1/MethodSignatureTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let method_defs = metadata
			.tables()
			.method_def
			.rows()
			.iter()
			.map(|m| {
				(
					metadata.strings().get(m.name).unwrap(),
					metadata
						.blob()
						.new_reader(m.signature)
						.unwrap()
						.read_method_signature()
						.unwrap(),
				)
			})
			.collect::<HashMap<_, _>>();

		assert_eq!(
			method_defs["Basic"],
			MethodSignature {
				instance: true,
				explicit_this: false,
				vararg: false,
				generic_param_count: 0,
				return_type: ElementType::Void,
				params: Box::new([])
			}
		);

		assert_eq!(
			method_defs["Static"],
			MethodSignature {
				instance: false,
				explicit_this: false,
				vararg: false,
				generic_param_count: 0,
				return_type: ElementType::Void,
				params: Box::new([])
			}
		);

		assert_eq!(
			method_defs["Generic"],
			MethodSignature {
				instance: true,
				explicit_this: false,
				vararg: false,
				generic_param_count: 1,
				return_type: ElementType::Void,
				params: Box::new([ElementType::MethodGenericParam(0)])
			}
		);

		assert_eq!(
			method_defs["ReturnsInt"],
			MethodSignature {
				instance: true,
				explicit_this: false,
				vararg: false,
				generic_param_count: 0,
				return_type: ElementType::Int,
				params: Box::new([])
			}
		);

		assert_eq!(
			method_defs["Add"],
			MethodSignature {
				instance: true,
				explicit_this: false,
				vararg: false,
				generic_param_count: 0,
				return_type: ElementType::Int,
				params: Box::new([ElementType::Int, ElementType::Int])
			}
		);
	}
}
