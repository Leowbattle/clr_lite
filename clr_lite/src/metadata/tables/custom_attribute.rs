use crate::metadata::*;

#[derive(Debug)]
pub struct CustomAttribute {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CustomAttributeHandle(pub(crate) usize);

impl From<CustomAttributeHandle> for usize {
	fn from(h: CustomAttributeHandle) -> usize {
		h.0
	}
}

impl From<usize> for CustomAttributeHandle {
	fn from(x: usize) -> CustomAttributeHandle {
		CustomAttributeHandle(x)
	}
}

impl TableRow for CustomAttribute {
	type Handle = CustomAttributeHandle;
	const TYPE: TableType = TableType::CustomAttribute;

	fn read_row(reader: &mut TableReader<'_>) -> Result<CustomAttribute, TableReaderError> {
		unimplemented!()
	}
}
