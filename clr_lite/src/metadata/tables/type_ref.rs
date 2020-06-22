///! ECMA-335 II.22.38
use crate::metadata::tables::*;

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
		TypeRefHandle(x + 1)
	}
}

impl TableRow for TypeRef {
	type Handle = TypeRefHandle;
	const TYPE: TableType = TableType::TypeRef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<TypeRef, TableReaderError> {
		Ok(TypeRef {
			resolution_scope: reader.read_resolution_scope_handle()?,
			name: reader.read_string_handle()?,
			namespace: reader.read_string_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::tables::*;

	#[test]
	fn test_type_ref() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/TypeRefTests/bin/Debug/netcoreapp3.1/TypeRefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		assert!(metadata
			.tables()
			.type_ref
			.rows()
			.iter()
			.any(|r| metadata.strings().get(r.name) == Some("Console")));
	}
}
