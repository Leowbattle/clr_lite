use crate::metadata::*;

#[derive(Debug)]
pub struct AssemblyRefOs {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyRefOsHandle(pub(crate) usize);

impl From<AssemblyRefOsHandle> for usize {
	fn from(h: AssemblyRefOsHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyRefOsHandle {
	fn from(x: usize) -> AssemblyRefOsHandle {
		AssemblyRefOsHandle(x)
	}
}

impl TableRow for AssemblyRefOs {
	type Handle = AssemblyRefOsHandle;
	const TYPE: TableType = TableType::AssemblyRefOs;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyRefOs, TableReaderError> {
		unimplemented!()
	}
}
