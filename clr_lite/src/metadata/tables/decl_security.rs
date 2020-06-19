///! ECMA-335 II.22.11
/// ECMA-335 Says this is deprecated, however it has to be parsed anyway due to a
/// (in my opinion) flaw in the .NET metadata format: The table stream header does not
/// contain a list of offsets of each table, which means that if you want to parse a specific table
/// you have to parse all of the tables before it, no skipping.
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct DeclSecurity {
	pub action: u16,
	pub parent: HasDeclSecurityHandle,
	pub permission_set: BlobHandle,
}

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
		Ok(DeclSecurity {
			action: reader._read::<u16>()?,
			parent: reader.read_has_decl_security_handle()?,
			permission_set: reader.read_blob_handle()?,
		})
	}
}
