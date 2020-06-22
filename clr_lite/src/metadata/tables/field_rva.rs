///! ECMA-335 II.22.18
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct FieldRva {
	pub rva: Rva,
	pub field: FieldHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldRvaHandle(pub(crate) usize);

impl From<FieldRvaHandle> for usize {
	fn from(h: FieldRvaHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldRvaHandle {
	fn from(x: usize) -> FieldRvaHandle {
		FieldRvaHandle(x + 1)
	}
}

impl TableRow for FieldRva {
	type Handle = FieldRvaHandle;
	const TYPE: TableType = TableType::FieldRva;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldRva, TableReaderError> {
		Ok(FieldRva {
			rva: reader.read_rva()?,
			field: reader.read_field_handle()?,
		})
	}
}
