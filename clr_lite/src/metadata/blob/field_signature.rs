///! ECMA-335 II.23.2.4
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldSignature {
	pub r#type: ElementType,
}

impl BlobReader<'_> {
	pub fn read_field_signature(&mut self) -> Result<FieldSignature, BlobReaderError> {
		let b = self.read::<u8>()?;
		if b != 0x6 {
			return Err(BlobReaderError::BadBlob(format!(
				"Expected 0x6 at the start of field signature, found {:#x}",
				b
			)));
		}

		let b = self.peek::<u8>()?;
		if b == 0x1f || b == 0x20 {
			unimplemented!("ELEMENT_TYPE_CMOD_REQD/ELEMENT_TYPE_CMOD_OPT not implemented");
		}

		Ok(FieldSignature {
			r#type: self.read_element_type()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashMap;

	#[test]
	fn test_field_signature() {
		let data = include_bytes!("../../../../tests/metadata/blob/FieldSignatureTests/bin/Debug/netcoreapp3.1/FieldSignatureTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let fields = metadata
			.tables()
			.field
			.rows()
			.iter()
			.map(|f| {
				(
					metadata.strings().get(f.name).unwrap(),
					metadata
						.blob()
						.new_reader(f.signature)
						.unwrap()
						.read_field_signature()
						.unwrap(),
				)
			})
			.collect::<HashMap<_, _>>();

		assert_eq!(fields["Int"].r#type, ElementType::Int);

		assert_eq!(fields["Str"].r#type, ElementType::String);

		assert_eq!(
			fields["Ptr"].r#type,
			ElementType::Pointer(Box::new(ElementType::Int))
		);

		assert_eq!(
			metadata
				.strings()
				.get(match fields["ValueType"].r#type {
					ElementType::ValueType(t) => match t {
						TypeDefOrRefHandle::TypeRefHandle(t) => metadata.tables().type_ref[t].name,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				})
				.unwrap(),
			"DateTime"
		);

		assert_eq!(
			metadata
				.strings()
				.get(match &fields["ValuePtr"].r#type {
					ElementType::Pointer(t) => match t.as_ref() {
						&ElementType::ValueType(t) => match t {
							TypeDefOrRefHandle::TypeRefHandle(t) =>
								metadata.tables().type_ref[t].name,
							_ => unreachable!(),
						},
						_ => unreachable!(),
					},
					_ => unreachable!(),
				})
				.unwrap(),
			"TimeSpan"
		);

		assert_eq!(
			metadata
				.strings()
				.get(match fields["Class"].r#type {
					ElementType::Class(t) => match t {
						TypeDefOrRefHandle::TypeRefHandle(t) => metadata.tables().type_ref[t].name,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				})
				.unwrap(),
			"Exception"
		);

		assert_eq!(
			fields["TypeGenericParam"].r#type,
			ElementType::TypeGenericParam(0)
		);

		assert_eq!(
			fields["Arr"].r#type,
			ElementType::Array {
				r#type: Box::new(ElementType::Int),
				shape: ArrayShape {
					rank: 2,
					sizes: Box::new([]),
					lower_bounds: Box::new([0, 0])
				}
			}
		);

		assert_eq!(
			fields["GenericInstantiation"].r#type,
			ElementType::Generic {
				r#type: Box::new(ElementType::Class(TypeDefOrRefHandle::TypeRefHandle(
					TypeRefHandle::from(
						metadata
							.tables()
							.type_ref
							.rows()
							.iter()
							.position(|f| metadata.strings().get(f.name).unwrap() == "List`1")
							.unwrap(),
					)
				))),
				args: Box::new([ElementType::Int])
			}
		);

		assert_eq!(fields["Obj"].r#type, ElementType::Object);

		assert_eq!(
			fields["Arr2"].r#type,
			ElementType::SzArray(Box::new(ElementType::ValueType(
				TypeDefOrRefHandle::TypeRefHandle(TypeRefHandle::from(
					metadata
						.tables()
						.type_ref
						.rows()
						.iter()
						.position(|f| metadata.strings().get(f.name).unwrap() == "Guid")
						.unwrap(),
				))
			)))
		);
	}
}
