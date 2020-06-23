///! ECMA-335 II.23.2.14
use super::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TypeSpec {
	pub r#type: ElementType,
}

impl BlobReader<'_> {
	pub fn read_type_spec(&mut self) -> Result<TypeSpec, BlobReaderError> {
		Ok(TypeSpec {
			r#type: self.read_element_type()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashSet;

	#[test]
	fn test_type_spec() {
		let data = include_bytes!("../../../../tests/metadata/blob/GenericInstanceTests/bin/Debug/netcoreapp3.1/GenericInstanceTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let type_spec = metadata
			.tables()
			.type_spec
			.rows()
			.iter()
			.map(|p| {
				stringify_element_type(
					&metadata,
					&metadata
						.blob()
						.new_reader(p.signature)
						.unwrap()
						.read_type_spec()
						.unwrap()
						.r#type,
				)
			})
			.collect::<HashSet<_>>();

		assert!(type_spec.contains("List`1<Int>"));
		assert!(type_spec.contains("Dictionary`2<String, List`1<String>>"));
	}

	fn stringify_element_type<'a>(metadata: &'a Metadata<'a>, e: &ElementType) -> String {
		use std::fmt::Write;
		match e {
			ElementType::ValueType(t) | ElementType::Class(t) => metadata
				.strings()
				.get(match t {
					TypeDefOrRefHandle::TypeDefHandle(t) => metadata.tables().type_def[*t].name,
					TypeDefOrRefHandle::TypeRefHandle(t) => metadata.tables().type_ref[*t].name,
					_ => unreachable!(),
				})
				.unwrap()
				.to_string(),
			ElementType::Generic { r#type, args } => {
				let mut s = String::new();
				write!(&mut s, "{}<", stringify_element_type(metadata, r#type)).unwrap();
				for a in args.iter() {
					write!(&mut s, "{}, ", stringify_element_type(metadata, a)).unwrap();
				}
				s.truncate(s.len() - 2);
				write!(&mut s, ">").unwrap();
				s
			}
			ElementType::SzArray(a) => format!("{}[]", stringify_element_type(metadata, a)),
			_ => format!("{:?}", e),
		}
	}
}
