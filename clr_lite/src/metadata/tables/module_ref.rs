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
		ModuleRefHandle(x + 1)
	}
}

impl TableRow for ModuleRef {
	type Handle = ModuleRefHandle;
	const TYPE: TableType = TableType::ModuleRef;
	fn read_row(reader: &mut TableReader<'_>) -> Result<ModuleRef, TableReaderError> {
		unimplemented!()
	}
}
