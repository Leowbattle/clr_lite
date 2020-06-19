use crate::metadata::tables::*;

#[derive(Debug)]
pub struct ClassLayout {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClassLayoutHandle(pub(crate) usize);

impl From<ClassLayoutHandle> for usize {
	fn from(h: ClassLayoutHandle) -> usize {
		h.0
	}
}

impl From<usize> for ClassLayoutHandle {
	fn from(x: usize) -> ClassLayoutHandle {
		ClassLayoutHandle(x + 1)
	}
}

impl TableRow for ClassLayout {
	type Handle = ClassLayoutHandle;
	const TYPE: TableType = TableType::ClassLayout;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ClassLayout, TableReaderError> {
		unimplemented!()
	}
}
