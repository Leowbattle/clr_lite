use crate::metadata::*;

#[derive(Debug)]
pub struct MethodDef {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodDefHandle(pub(crate) usize);

impl From<MethodDefHandle> for usize {
	fn from(h: MethodDefHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodDefHandle {
	fn from(x: usize) -> MethodDefHandle {
		MethodDefHandle(x)
	}
}

impl TableRow for MethodDef {
	type Handle = MethodDefHandle;
	const TYPE: TableType = TableType::MethodDef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodDef, TableReaderError> {
		unimplemented!()
	}
}
