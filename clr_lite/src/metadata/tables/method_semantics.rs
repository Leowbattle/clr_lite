use crate::metadata::*;

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
		MethodSemanticsHandle(x)
	}
}

impl TableRow for MethodSemantics {
	type Handle = MethodSemanticsHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodSemantics, TableReaderError> {
		unimplemented!()
	}
}
