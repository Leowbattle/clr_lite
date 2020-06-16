#[derive(Debug)]
pub struct MemberRef {
	pub class: MemberRefParent,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

crate::def_table!(
	MemberRef,
	MemberRefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<MemberRef> {
		Ok(MemberRef {
			class: reader.read_member_ref_parent()?,
			name: reader.read_string_handle()?,
			signature: reader.read_blob_handle()?,
		})
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	#[test]
	fn test_member_ref() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/MemberRefTests/bin/Debug/netcoreapp3.1/MemberRefTests.dll"
		);

		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let type_refs = metadata.tables.type_ref;

		let member_refs = metadata
			.tables
			.member_ref
			.rows()
			.iter()
			.map(|m| {
				(
					strings
						.get(match m.class {
							MemberRefParent::TypeRefHandle(t) => type_refs[t].type_name,
							_ => unimplemented!(),
						})
						.unwrap(),
					strings.get(m.name).unwrap(),
				)
			})
			.collect::<Vec<(&str, &str)>>();

		assert!(member_refs.contains(&("Console", "WriteLine")));
		assert!(member_refs.contains(&("String", "Format")));
		assert!(member_refs.contains(&("String", "get_Length")));
	}
}
