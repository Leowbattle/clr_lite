use crate::metadata::tables::*;

#[derive(Debug)]
pub struct TypeSpec {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypeSpecHandle(pub(crate) usize);

impl From<TypeSpecHandle> for usize {
	fn from(h: TypeSpecHandle) -> usize {
		h.0
	}
}

impl From<usize> for TypeSpecHandle {
	fn from(x: usize) -> TypeSpecHandle {
		TypeSpecHandle(x + 1)
	}
}

impl TableRow for TypeSpec {
	type Handle = TypeSpecHandle;
	const TYPE: TableType = TableType::TypeSpec;

	fn read_row(reader: &mut TableReader<'_>) -> Result<TypeSpec, TableReaderError> {
		unimplemented!()
	}
}
