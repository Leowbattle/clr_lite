use crate::metadata::*;

#[derive(Debug)]
pub struct ManifestResource {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ManifestResourceHandle(pub(crate) usize);

impl From<ManifestResourceHandle> for usize {
	fn from(h: ManifestResourceHandle) -> usize {
		h.0
	}
}

impl From<usize> for ManifestResourceHandle {
	fn from(x: usize) -> ManifestResourceHandle {
		ManifestResourceHandle(x+1)
	}
}

impl TableRow for ManifestResource {
	type Handle = ManifestResourceHandle;
	const TYPE: TableType = TableType::ManifestResource;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ManifestResource, TableReaderError> {
		unimplemented!()
	}
}
