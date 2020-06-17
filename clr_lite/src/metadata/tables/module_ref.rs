use crate::metadata::*;

#[derive(Debug)]
pub struct ModuleRef {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModuleRefHandle(pub(crate) usize);

impl From<ModuleRefHandle> for usize {
	fn from(h: ModuleRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for ModuleRefHandle {
	fn from(x: usize) -> ModuleRefHandle {
		ModuleRefHandle(x)
	}
}

impl TableRow for ModuleRef {
	type Handle = ModuleRefHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ModuleRef, TableReaderError> {
		unimplemented!()
	}
}