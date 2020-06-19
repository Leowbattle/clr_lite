use crate::metadata::tables::*;

#[derive(Debug)]
pub struct MethodSemantics {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodSemanticsHandle(pub(crate) usize);

impl From<MethodSemanticsHandle> for usize {
	fn from(h: MethodSemanticsHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodSemanticsHandle {
	fn from(x: usize) -> MethodSemanticsHandle {
		MethodSemanticsHandle(x + 1)
	}
}

impl TableRow for MethodSemantics {
	type Handle = MethodSemanticsHandle;
	const TYPE: TableType = TableType::MethodSemantics;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodSemantics, TableReaderError> {
		unimplemented!()
	}
}
