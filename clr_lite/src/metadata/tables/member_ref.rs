///! ECMA-335 II.22.25
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct MemberRef {
	pub class: MemberRefParentHandle,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemberRefHandle(pub(crate) usize);

impl From<MemberRefHandle> for usize {
	fn from(h: MemberRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for MemberRefHandle {
	fn from(x: usize) -> MemberRefHandle {
		MemberRefHandle(x + 1)
	}
}

impl TableRow for MemberRef {
	type Handle = MemberRefHandle;
	const TYPE: TableType = TableType::MemberRef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MemberRef, TableReaderError> {
		Ok(MemberRef {
			class: reader.read_member_ref_parent_handle()?,
			name: reader.read_string_handle()?,
			signature: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_member_ref() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/MemberRefTests/bin/Debug/netcoreapp3.1/MemberRefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let member_refs = metadata
			.tables()
			.member_ref
			.rows()
			.iter()
			.map(|m| {
				(
					match m.class {
						MemberRefParentHandle::TypeRefHandle(t) => metadata
							.strings()
							.get(metadata.tables().type_ref[t].name)
							.unwrap(),
						_ => unimplemented!(),
					},
					metadata.strings().get(m.name).unwrap(),
				)
			})
			.collect::<Box<[(&str, &str)]>>();

		assert!(member_refs.contains(&("Console", "WriteLine")));
		assert!(member_refs.contains(&("String", "Format")));
	}
}
