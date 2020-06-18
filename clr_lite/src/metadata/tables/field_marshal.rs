use crate::metadata::*;

#[derive(Debug)]
pub struct FieldMarshal {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldMarshalHandle(pub(crate) usize);

impl From<FieldMarshalHandle> for usize {
	fn from(h: FieldMarshalHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldMarshalHandle {
	fn from(x: usize) -> FieldMarshalHandle {
		FieldMarshalHandle(x)
	}
}

impl TableRow for FieldMarshal {
	type Handle = FieldMarshalHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldMarshal, TableReaderError> {
		unimplemented!()
	}
}
