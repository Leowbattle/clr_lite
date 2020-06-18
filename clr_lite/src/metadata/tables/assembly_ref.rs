use crate::metadata::*;

#[derive(Debug)]
pub struct AssemblyRef {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyRefHandle(pub(crate) usize);

impl From<AssemblyRefHandle> for usize {
	fn from(h: AssemblyRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyRefHandle {
	fn from(x: usize) -> AssemblyRefHandle {
		AssemblyRefHandle(x + 1)
	}
}

impl TableRow for AssemblyRef {
	type Handle = AssemblyRefHandle;
	const TYPE: TableType = TableType::AssemblyRef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyRef, TableReaderError> {
		unimplemented!()
	}
}
