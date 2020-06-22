///! ECMA-335 II.22.4
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct AssemblyProcessor {
	processor: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyProcessorHandle(pub(crate) usize);

impl From<AssemblyProcessorHandle> for usize {
	fn from(h: AssemblyProcessorHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyProcessorHandle {
	fn from(x: usize) -> AssemblyProcessorHandle {
		AssemblyProcessorHandle(x + 1)
	}
}

impl TableRow for AssemblyProcessor {
	type Handle = AssemblyProcessorHandle;
	const TYPE: TableType = TableType::AssemblyProcessor;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyProcessor, TableReaderError> {
		Ok(AssemblyProcessor {
			processor: reader._read::<u32>()?,
		})
	}
}
