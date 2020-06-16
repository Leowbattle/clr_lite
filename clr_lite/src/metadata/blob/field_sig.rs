/// ECMA-335 II.23.4.2
use std::io;

use super::*;

#[derive(Debug)]
pub struct FieldSig {
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

		let r#type = self.read_element_type()?;

		Ok(FieldSig { r#type })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::tables::*;
	use crate::metadata::*;
	use crate::pe::*;

	use std::collections::HashMap;

	#[test]
	fn test_field_sig() {
		let data = include_bytes!(
			"../../../../tests/metadata/blob/FieldSigTests/bin/Debug/netcoreapp3.1/FieldSigTests.dll"
		);

		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let blob = metadata.blob_heap;
		let types = &metadata.tables.type_def;
		let type_refs = &metadata.tables.type_ref;
		let fields = &metadata.tables.field;

		let fields = fields
			.rows()
			.iter()
			.map(|f| {
				(
					strings.get(f.name.into()).unwrap(),
					blob.get_field_sig(f.signature).unwrap(),
				)
			})
			.collect::<HashMap<&str, FieldSig>>();

		let find_type_def = |name| {
			TypeDefHandle(
				types
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name).unwrap() == name)
					.unwrap() + 1,
			)
		};

		let find_type_ref = |name| {
			TypeRefHandle(
				type_refs
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name).unwrap() == name)
					.unwrap() + 1,
			)
		};

		assert_eq!(fields.get("Bool").unwrap().r#type, ElementType::Bool);
		assert_eq!(fields.get("Str").unwrap().r#type, ElementType::String);
		assert_eq!(
			fields.get("Ptr").unwrap().r#type,
			ElementType::Pointer(Box::new(ElementType::Int))
		);
		assert_eq!(
			fields.get("Struct").unwrap().r#type,
			ElementType::ValueType(TypeDefOrRef::TypeDefHandle(find_type_def("MyStruct")))
		);
		assert_eq!(
			fields.get("Class").unwrap().r#type,
			ElementType::Class(TypeDefOrRef::TypeRefHandle(find_type_ref("Exception")))
		);
		assert_eq!(
			fields.get("GenericInstance").unwrap().r#type,
			ElementType::GenericInstantiation {
				r#type: Box::new(ElementType::Class(TypeDefOrRef::TypeDefHandle(
					find_type_def("Class1`1")
				))),
				generic_args: Box::new([ElementType::Int])
			}
		);
		assert_eq!(
			fields.get("t").unwrap().r#type,
			ElementType::FieldGenericParameter(0)
		);
		assert_eq!(fields.get("o").unwrap().r#type, ElementType::Object);
		assert_eq!(
			fields.get("arr").unwrap().r#type,
			ElementType::SzArray(Box::new(ElementType::Float))
		);
		assert_eq!(
			fields.get("arr2").unwrap().r#type,
			ElementType::SzArray(Box::new(ElementType::SzArray(Box::new(ElementType::Int))))
		);
		assert_eq!(
			fields.get("arr3").unwrap().r#type,
			ElementType::Array {
				element_type: Box::new(ElementType::Int),
				shape: ArrayShape {
					rank: 2,
					sizes: Box::new([]),
					lower_bounds: Box::new([0, 0])
				}
			}
		);
		assert_eq!(
			fields.get("finalBoss").unwrap().r#type,
			ElementType::SzArray(Box::new(ElementType::GenericInstantiation {
				r#type: Box::new(ElementType::Class(TypeDefOrRef::TypeDefHandle(
					find_type_def("Class1`1")
				))),
				generic_args: Box::new([ElementType::Array {
					element_type: Box::new(ElementType::ValueType(TypeDefOrRef::TypeDefHandle(
						find_type_def("MyStruct")
					))),
					shape: ArrayShape {
						rank: 4,
						sizes: Box::new([]),
						lower_bounds: Box::new([0, 0, 0, 0])
					}
				}])
			}))
		);
	}
}
