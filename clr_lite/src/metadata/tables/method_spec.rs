use crate::metadata::*;

#[derive(Debug)]
pub struct MethodSpec {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodSpecHandle(pub(crate) usize);

impl From<MethodSpecHandle> for usize {
	fn from(h: MethodSpecHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodSpecHandle {
	fn from(x: usize) -> MethodSpecHandle {
		MethodSpecHandle(x + 1)
	}
}

impl TableRow for MethodSpec {
	type Handle = MethodSpecHandle;
	const TYPE: TableType = TableType::MethodSpec;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodSpec, TableReaderError> {
		unimplemented!()
	}
}
