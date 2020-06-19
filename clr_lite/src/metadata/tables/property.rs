use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Property {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PropertyHandle(pub(crate) usize);

impl From<PropertyHandle> for usize {
	fn from(h: PropertyHandle) -> usize {
		h.0
	}
}

impl From<usize> for PropertyHandle {
	fn from(x: usize) -> PropertyHandle {
		PropertyHandle(x + 1)
	}
}

impl TableRow for Property {
	type Handle = PropertyHandle;
	const TYPE: TableType = TableType::Property;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Property, TableReaderError> {
		unimplemented!()
	}
}
