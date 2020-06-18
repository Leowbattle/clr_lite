use crate::metadata::*;

#[derive(Debug)]
pub struct ExportedType {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportedTypeHandle(pub(crate) usize);

impl From<ExportedTypeHandle> for usize {
	fn from(h: ExportedTypeHandle) -> usize {
		h.0
	}
}

impl From<usize> for ExportedTypeHandle {
	fn from(x: usize) -> ExportedTypeHandle {
		ExportedTypeHandle(x)
	}
}

impl TableRow for ExportedType {
	type Handle = ExportedTypeHandle;
	const TYPE: TableType = TableType::ExportedType;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ExportedType, TableReaderError> {
		unimplemented!()
	}
}
