///! ECMA-335 II.23.2.15
use super::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MethodSpec {
	pub args: Box<[ElementType]>,
}

impl BlobReader<'_> {
	pub fn read_method_spec(&mut self) -> Result<MethodSpec, BlobReaderError> {
		let b = self.read::<u8>()?;
		if b != 0xa {
			return Err(BlobReaderError::BadBlob(format!(
				"Expected 0xa at the start of MethodSpec, found {:#x}",
				b
			)));
		}

		let count = self.read_compressed_u32()? as usize;
		let args = {
			let mut params = Vec::with_capacity(count);
			for _ in 0..count {
				params.push(self.read_element_type()?);
			}
			params.into_boxed_slice()
		};

		Ok(MethodSpec { args })
	}
}

#[cfg(test)]
mod tests {
	use crate::metadata::blob::ElementType;
	use crate::metadata::tables::*;
	use crate::metadata::*;

	use std::collections::HashSet;

	#[test]
	fn test_method_spec() {
		let data = include_bytes!("../../../../tests/metadata/blob/GenericInstanceTests/bin/Debug/netcoreapp3.1/GenericInstanceTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let method_spec = metadata
			.tables()
			.method_spec
			.rows()
			.iter()
			.map(|m| stringify_method_spec(&metadata, &m))
			.collect::<HashSet<_>>();

		assert!(method_spec.contains("Where<Int>"));
		assert!(method_spec.contains("Generic<String, Object>"));
	}

	fn stringify_method_spec<'a>(metadata: &'a Metadata<'a>, m: &MethodSpec) -> String {
		use std::fmt::Write;
		let mut s = String::new();
		write!(
			&mut s,
			"{}<",
			metadata
				.strings()
				.get(match m.method {
					MethodDefOrRefHandle::MethodDefHandle(m) =>
						metadata.tables().method_def[m].name,
					MethodDefOrRefHandle::MemberRefHandle(m) =>
						metadata.tables().member_ref[m].name,
				})
				.unwrap()
		)
		.unwrap();
		let args = metadata
			.blob()
			.new_reader(m.instantiation)
			.unwrap()
			.read_method_spec()
			.unwrap()
			.args;
		for arg in args.iter() {
			write!(&mut s, "{}, ", stringify_element_type(metadata, arg)).unwrap();
		}
		s.truncate(s.len() - 2);
		write!(&mut s, ">").unwrap();
		s
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
