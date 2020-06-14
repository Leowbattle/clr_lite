/// ECMA-335 II.22.38
#[derive(Debug)]
pub struct TypeRef {
	pub resolution_scope: ResolutionScope,
	pub type_name: StringHandle,
	pub type_namespace: StringHandle,
}

crate::def_table!(
	TypeRef,
	TypeRefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<TypeRef> {
		Ok(TypeRef {
			resolution_scope: reader.read_resolution_scope()?,
			type_name: reader.read_string_handle()?,
			type_namespace: reader.read_string_handle()?,
		})
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	#[test]
	fn test_type_ref() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/TypeRefTests/bin/Debug/netcoreapp3.1/TypeRefTests.dll"
		);

		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		assert!(metadata
			.tables
			.type_ref
			.as_ref()
			.unwrap()
			.rows()
			.iter()
			.any(|t| {
				match metadata.strings_heap.get(t.type_name) {
					Some(name) => name == "Console",
					None => false,
				}
			}));
	}
}
