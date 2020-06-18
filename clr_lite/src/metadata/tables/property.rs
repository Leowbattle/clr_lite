use crate::metadata::*;

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
		PropertyHandle(x)
	}
}

impl TableRow for Property {
	type Handle = PropertyHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Property, TableReaderError> {
		unimplemented!()
	}
}
