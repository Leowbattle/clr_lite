use crate::metadata::*;

#[derive(Debug)]
pub struct InterfaceImpl {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceImplHandle(pub(crate) usize);

impl From<InterfaceImplHandle> for usize {
	fn from(h: InterfaceImplHandle) -> usize {
		h.0
	}
}

impl From<usize> for InterfaceImplHandle {
	fn from(x: usize) -> InterfaceImplHandle {
		InterfaceImplHandle(x)
	}
}

impl TableRow for InterfaceImpl {
	type Handle = InterfaceImplHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<InterfaceImpl, TableReaderError> {
		unimplemented!()
	}
}
