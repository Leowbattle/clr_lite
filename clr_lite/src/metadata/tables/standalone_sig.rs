use crate::metadata::*;

#[derive(Debug)]
pub struct StandaloneSig {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StandaloneSigHandle(pub(crate) usize);

impl From<StandaloneSigHandle> for usize {
	fn from(h: StandaloneSigHandle) -> usize {
		h.0
	}
}

impl From<usize> for StandaloneSigHandle {
	fn from(x: usize) -> StandaloneSigHandle {
		StandaloneSigHandle(x + 1)
	}
}

impl TableRow for StandaloneSig {
	type Handle = StandaloneSigHandle;
	const TYPE: TableType = TableType::StandaloneSig;

	fn read_row(reader: &mut TableReader<'_>) -> Result<StandaloneSig, TableReaderError> {
		unimplemented!()
	}
}
