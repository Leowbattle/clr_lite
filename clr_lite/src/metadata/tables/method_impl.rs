use crate::metadata::*;

#[derive(Debug)]
pub struct MethodImpl {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodImplHandle(pub(crate) usize);

impl From<MethodImplHandle> for usize {
	fn from(h: MethodImplHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodImplHandle {
	fn from(x: usize) -> MethodImplHandle {
		MethodImplHandle(x)
	}
}

impl TableRow for MethodImpl {
	type Handle = MethodImplHandle;
	const TYPE: TableType = TableType::MethodImpl;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodImpl, TableReaderError> {
		unimplemented!()
	}
}
