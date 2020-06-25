///! ECMA-335 II.23.2.5
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PropertySignature {
	pub instance: bool,
	pub return_type: ElementType,
	pub params: Box<[ElementType]>,
}

impl BlobReader<'_> {
	pub fn read_property_signature(&mut self) -> Result<PropertySignature, BlobReaderError> {
		let next = self.read::<u8>()?;
		if next & 0x8 != 0x8 {
			return Err(BlobReaderError::BadBlob(
				"Invalid property signature".to_string(),
			));
		}
		let instance = next & 0x20 == 0x20;

		let param_count = self.read_compressed_u32()? as usize;

		let return_type = self.read_element_type()?;

		let params = {
			let mut params = Vec::with_capacity(param_count);
			for _ in 0..param_count {
				params.push(self.read_element_type()?);
			}
			params.into_boxed_slice()
		};

		Ok(PropertySignature {
			instance,
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
	fn test_property_signature() {
		let data = include_bytes!("../../../../tests/metadata/blob/PropertySignatureTests/bin/Debug/netcoreapp3.1/PropertySignatureTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let properties = metadata
			.tables()
			.property
			.rows()
			.iter()
			.map(|p| {
				(
					metadata.strings().get(p.name).unwrap(),
					metadata
						.blob()
						.new_reader(p.signature)
						.unwrap()
						.read_property_signature()
						.unwrap(),
				)
			})
			.collect::<HashMap<_, _>>();

		assert_eq!(
			properties["Prop"],
			PropertySignature {
				instance: true,
				return_type: ElementType::Int,
				params: Box::new([])
			}
		);

		assert_eq!(
			properties["Prop2"],
			PropertySignature {
				instance: true,
				return_type: ElementType::String,
				params: Box::new([])
			}
		);

		// Internally, indexers are called "Item".
		assert_eq!(
			properties["Item"],
			PropertySignature {
				instance: true,
				return_type: ElementType::Int,
				params: Box::new([ElementType::Int, ElementType::Int])
			}
		);
	}
}
