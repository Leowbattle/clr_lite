///! ECMA-335 II.22.24
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct ManifestResource {
	pub offset: usize,
	pub public: bool,
	pub name: StringHandle,
	pub implements: ImplementationHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ManifestResourceHandle(pub(crate) usize);

impl From<ManifestResourceHandle> for usize {
	fn from(h: ManifestResourceHandle) -> usize {
		h.0
	}
}

impl From<usize> for ManifestResourceHandle {
	fn from(x: usize) -> ManifestResourceHandle {
		ManifestResourceHandle(x + 1)
	}
}

impl TableRow for ManifestResource {
	type Handle = ManifestResourceHandle;
	const TYPE: TableType = TableType::ManifestResource;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ManifestResource, TableReaderError> {
		Ok(ManifestResource {
			offset: reader._read::<u32>()? as usize,
			public: if reader._read::<u32>()? == 0x1 {
				true
			} else {
				false
			},
			name: reader.read_string_handle()?,
			implements: reader.read_implementation_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_manifest_resource() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/ManifestResourceTests/bin/Debug/netcoreapp3.1/ManifestResourceTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let embedded_resources = metadata
			.tables()
			.manifest_resource
			.rows()
			.iter()
			.map(|mr| (metadata.strings().get(mr.name).unwrap(), mr))
			.collect::<HashMap<_, _>>();

		let text_file_1 = embedded_resources["ManifestResourceTests.TextFile1.txt"];
		assert_eq!(
			std::str::from_utf8(metadata.get_resource_data(text_file_1.offset).unwrap()).unwrap(),
			include_str!("../../../../tests/metadata/tables/ManifestResourceTests/TextFile1.txt")
		);
	}
}
