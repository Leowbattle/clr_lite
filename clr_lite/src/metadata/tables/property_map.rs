use crate::metadata::*;

#[derive(Debug)]
pub struct PropertyMap {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PropertyMapHandle(pub(crate) usize);

impl From<PropertyMapHandle> for usize {
	fn from(h: PropertyMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for PropertyMapHandle {
	fn from(x: usize) -> PropertyMapHandle {
		PropertyMapHandle(x + 1)
	}
}

impl TableRow for PropertyMap {
	type Handle = PropertyMapHandle;
	const TYPE: TableType = TableType::PropertyMap;

	fn read_row(reader: &mut TableReader<'_>) -> Result<PropertyMap, TableReaderError> {
		unimplemented!()
	}
}
