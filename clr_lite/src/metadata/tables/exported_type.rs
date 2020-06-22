use crate::metadata::tables::*;

#[derive(Debug)]
pub struct ExportedType {
	pub flags: TypeAttributes,
	pub type_def_id: TypeDefHandle,
	pub name: StringHandle,
	pub namespace: StringHandle,
	pub implementation: ImplementationHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportedTypeHandle(pub(crate) usize);

impl From<ExportedTypeHandle> for usize {
	fn from(h: ExportedTypeHandle) -> usize {
		h.0
	}
}

impl From<usize> for ExportedTypeHandle {
	fn from(x: usize) -> ExportedTypeHandle {
		ExportedTypeHandle(x + 1)
	}
}

impl TableRow for ExportedType {
	type Handle = ExportedTypeHandle;
	const TYPE: TableType = TableType::ExportedType;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ExportedType, TableReaderError> {
		Ok(ExportedType {
			flags: TypeAttributes::from_raw(reader._read::<u32>()?)?,
			type_def_id: TypeDefHandle(reader._read::<u32>()? as usize),
			name: reader.read_string_handle()?,
			namespace: reader.read_string_handle()?,
			implementation: reader.read_implementation_handle()?,
		})
	}
}
