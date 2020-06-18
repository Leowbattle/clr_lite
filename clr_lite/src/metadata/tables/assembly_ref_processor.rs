use crate::metadata::*;

#[derive(Debug)]
pub struct AssemblyRefProcessor {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyRefProcessorHandle(pub(crate) usize);

impl From<AssemblyRefProcessorHandle> for usize {
	fn from(h: AssemblyRefProcessorHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyRefProcessorHandle {
	fn from(x: usize) -> AssemblyRefProcessorHandle {
		AssemblyRefProcessorHandle(x + 1)
	}
}

impl TableRow for AssemblyRefProcessor {
	type Handle = AssemblyRefProcessorHandle;
	const TYPE: TableType = TableType::AssemblyRefProcessor;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyRefProcessor, TableReaderError> {
		unimplemented!()
	}
}
