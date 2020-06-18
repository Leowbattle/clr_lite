use crate::metadata::*;

#[derive(Debug)]
pub struct AssemblyOs {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyOsHandle(pub(crate) usize);

impl From<AssemblyOsHandle> for usize {
	fn from(h: AssemblyOsHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyOsHandle {
	fn from(x: usize) -> AssemblyOsHandle {
		AssemblyOsHandle(x + 1)
	}
}

impl TableRow for AssemblyOs {
	type Handle = AssemblyOsHandle;
	const TYPE: TableType = TableType::AssemblyOs;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyOs, TableReaderError> {
		unimplemented!()
	}
}
