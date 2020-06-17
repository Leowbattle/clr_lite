use crate::metadata::*;

#[derive(Debug)]
pub struct TypeRef {
	pub resolution_scope: ResolutionScopeHandle,
	pub name: StringHandle,
	pub namespace: StringHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypeRefHandle(pub(crate) usize);

impl From<TypeRefHandle> for usize {
	fn from(h: TypeRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for TypeRefHandle {
	fn from(x: usize) -> TypeRefHandle {
		TypeRefHandle(x)
	}
}

impl TableRow for TypeRef {
	type Handle = TypeRefHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<TypeRef, TableReaderError> {
		Ok(TypeRef {
			resolution_scope: reader.read_resolution_scope_handle()?,
			name: reader.read_string_handle()?,
			namespace: reader.read_string_handle()?,
		})
	}
}
