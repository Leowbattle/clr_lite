use crate::metadata::tables::*;

#[derive(Debug)]
pub struct FieldRva {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldRvaHandle(pub(crate) usize);

impl From<FieldRvaHandle> for usize {
	fn from(h: FieldRvaHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldRvaHandle {
	fn from(x: usize) -> FieldRvaHandle {
		FieldRvaHandle(x + 1)
	}
}

impl TableRow for FieldRva {
	type Handle = FieldRvaHandle;
	const TYPE: TableType = TableType::FieldRva;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldRva, TableReaderError> {
		unimplemented!()
	}
}
