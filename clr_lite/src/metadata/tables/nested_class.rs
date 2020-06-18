use crate::metadata::*;

#[derive(Debug)]
pub struct NestedClass {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NestedClassHandle(pub(crate) usize);

impl From<NestedClassHandle> for usize {
	fn from(h: NestedClassHandle) -> usize {
		h.0
	}
}

impl From<usize> for NestedClassHandle {
	fn from(x: usize) -> NestedClassHandle {
		NestedClassHandle(x)
	}
}

impl TableRow for NestedClass {
	type Handle = NestedClassHandle;
	const TYPE: TableType = TableType::NestedClass;

	fn read_row(reader: &mut TableReader<'_>) -> Result<NestedClass, TableReaderError> {
		unimplemented!()
	}
}
