use crate::metadata::*;

#[derive(Debug)]
pub struct MemberRef {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemberRefHandle(pub(crate) usize);

impl From<MemberRefHandle> for usize {
	fn from(h: MemberRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for MemberRefHandle {
	fn from(x: usize) -> MemberRefHandle {
		MemberRefHandle(x + 1)
	}
}

impl TableRow for MemberRef {
	type Handle = MemberRefHandle;
	const TYPE: TableType = TableType::MemberRef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MemberRef, TableReaderError> {
		unimplemented!()
	}
}
