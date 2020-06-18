use crate::metadata::*;

#[derive(Debug)]
pub struct Constant {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstantHandle(pub(crate) usize);

impl From<ConstantHandle> for usize {
	fn from(h: ConstantHandle) -> usize {
		h.0
	}
}

impl From<usize> for ConstantHandle {
	fn from(x: usize) -> ConstantHandle {
		ConstantHandle(x)
	}
}

impl TableRow for Constant {
	type Handle = ConstantHandle;
	const TYPE: TableType = TableType::Constant;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Constant, TableReaderError> {
		unimplemented!()
	}
}
