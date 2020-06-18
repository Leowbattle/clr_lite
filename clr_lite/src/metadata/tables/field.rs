use crate::metadata::*;

#[derive(Debug)]
pub struct Field {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldHandle(pub(crate) usize);

impl From<FieldHandle> for usize {
	fn from(h: FieldHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldHandle {
	fn from(x: usize) -> FieldHandle {
		FieldHandle(x)
	}
}

impl TableRow for Field {
	type Handle = FieldHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Field, TableReaderError> {
		unimplemented!()
	}
}
