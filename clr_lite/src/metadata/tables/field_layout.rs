use crate::metadata::tables::*;

#[derive(Debug)]
pub struct FieldLayout {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldLayoutHandle(pub(crate) usize);

impl From<FieldLayoutHandle> for usize {
	fn from(h: FieldLayoutHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldLayoutHandle {
	fn from(x: usize) -> FieldLayoutHandle {
		FieldLayoutHandle(x + 1)
	}
}

impl TableRow for FieldLayout {
	type Handle = FieldLayoutHandle;
	const TYPE: TableType = TableType::FieldLayout;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldLayout, TableReaderError> {
		unimplemented!()
	}
}
