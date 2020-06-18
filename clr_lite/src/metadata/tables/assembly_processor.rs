use crate::metadata::*;

#[derive(Debug)]
pub struct AssemblyProcessor {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyProcessorHandle(pub(crate) usize);

impl From<AssemblyProcessorHandle> for usize {
	fn from(h: AssemblyProcessorHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyProcessorHandle {
	fn from(x: usize) -> AssemblyProcessorHandle {
		AssemblyProcessorHandle(x)
	}
}

impl TableRow for AssemblyProcessor {
	type Handle = AssemblyProcessorHandle;
	const TYPE: TableType = TableType::AssemblyProcessor;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyProcessor, TableReaderError> {
		unimplemented!()
	}
}
