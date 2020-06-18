use crate::metadata::*;

#[derive(Debug)]
pub struct Assembly {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyHandle(pub(crate) usize);

impl From<AssemblyHandle> for usize {
	fn from(h: AssemblyHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyHandle {
	fn from(x: usize) -> AssemblyHandle {
		AssemblyHandle(x)
	}
}

impl TableRow for Assembly {
	type Handle = AssemblyHandle;
	const TYPE: TableType = TableType::Assembly;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Assembly, TableReaderError> {
		unimplemented!()
	}
}
