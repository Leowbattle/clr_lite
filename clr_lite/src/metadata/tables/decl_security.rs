use crate::metadata::*;

#[derive(Debug)]
pub struct DeclSecurity {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DeclSecurityHandle(pub(crate) usize);

impl From<DeclSecurityHandle> for usize {
	fn from(h: DeclSecurityHandle) -> usize {
		h.0
	}
}

impl From<usize> for DeclSecurityHandle {
	fn from(x: usize) -> DeclSecurityHandle {
		DeclSecurityHandle(x + 1)
	}
}

impl TableRow for DeclSecurity {
	type Handle = DeclSecurityHandle;
	const TYPE: TableType = TableType::DeclSecurity;

	fn read_row(reader: &mut TableReader<'_>) -> Result<DeclSecurity, TableReaderError> {
		unimplemented!()
	}
}
